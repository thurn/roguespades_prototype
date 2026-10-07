"""Measurement steps for draft passes and optimization rounds."""

import json
import math

import numpy as np

from . import analyze as A
from . import arms as R
from . import experiments as E
from . import funscore
from .common import CONTROL, MODELS, PRICE, load_sigils, rsim
from .experiments import BLANK
from .records import load

ARCHES = R.ARCH


def controls() -> list:
    return list(CONTROL.values())


def kept_ids(sigils: dict) -> list:
    return sorted(s for s, d in sigils.items() if d.get("status") == "kept")


def latest_model() -> str:
    order = [
        "pilot",
        "draft-common",
        "draft-uncommon",
        "draft-rare",
        "round-1",
        "round-2",
        "round-3",
        "round-4",
    ]
    best = None
    for name in order:
        if (MODELS / f"{name}.json").exists():
            best = name
    return str(MODELS / f"{best}.json") if best else None


def scale_of(model_path: str) -> float:
    return json.loads(open(model_path).read())["wFinal"]


def gen_text() -> None:
    rsim("gen-text", "--dir", "../data/sigils", quiet=True)


def screen(step: str, ids: list) -> dict:
    stats = E.screen(step, ids, rounds=300)
    return {s["id"]: s for s in stats}


def boards_for(n_measured: int, half_width: float = 1.5, base: float = 166551, ref: int = 170) -> int:
    """Tier-0 boards for a median half-width target, from the pilot's measured precision."""
    return int(base * (n_measured / ref) * (1.5 / half_width) ** 2)


def measure_tournaments(
    step: str,
    measured_ids: list,
    offerable: list,
    model: str,
    boards: int,
    weights: dict | None = None,
    seed: int = 1,
    calib_share: float = 0.1,
    extra_paths: list | None = None,
    pairs=None,
    name: str = "tournament",
    tier: int = 0,
    calib_tier: int = 1,
    board_offset: int = 0,
) -> A.Measurement:
    sigils = load_sigils()
    measured = {s: (weights or {}).get(s, 1.0) for s in measured_ids}
    for c in controls():
        measured.setdefault(c, 1.0)
    p = E.tournament(
        step,
        name,
        offerable=offerable,
        measured=measured,
        boards=boards,
        tier=tier,
        seed=seed,
        model=model,
        pairs=pairs,
        pair_prob=0.15 if pairs else 0.0,
        validate=0.01,
        board_offset=board_offset,
    )
    paths = [p] + list(extra_paths or [])
    m = A.measure(step, name, paths, sigils, scale_of(model))
    if calib_share > 0:
        nb = max(200, int(boards * calib_share))
        pc = E.tournament(
            step,
            f"{name}-calib",
            offerable=offerable,
            measured=measured,
            boards=nb,
            tier=calib_tier,
            seed=seed,
            model=model,
            pairs=pairs,
            pair_prob=0.15 if pairs else 0.0,
            board_offset=board_offset,
        )
        A.calibrate(m, [pc], sigils, step)
    return m


def promising_weights(m: A.Measurement, ids: list) -> dict:
    """Successive halving: concentrate grants on the more promising and the uncertain half."""
    ucb = {
        s: m.readings[s].working + 1.645 * (m.readings[s].lift_hi - m.readings[s].lift_lo) / 3.29
        for s in ids
        if s in m.readings
    }
    order = sorted(ucb, key=lambda s: -ucb[s])
    top = set(order[: max(1, len(order) // 2)])
    return {s: (3.0 if s in top else 0.4) for s in ids}


def shop_rank_corr(m: A.Measurement) -> float:
    from .pilot import spearman

    rh = [spearman(c["model"], c["rescored"]) for r in m.loaded.runs for c in r.checks if len(c["ids"]) >= 3]
    rh = [x for x in rh if not math.isnan(x)]
    return float(np.mean(rh)) if rh else float("nan")


def simplicity_mean(ids: list, sigils: dict) -> float:
    vals = [sigils[s].get("simplicity", {}).get("S", 0.0) for s in ids]
    return float(np.mean(vals)) if vals else 1.0


def pool_arms(
    step: str,
    pool: list,
    model: str,
    boards_commit: int = 400,
    boards_field: int = 1500,
    ladder_boards: int = 400,
    ladder_tiers=(1, 0),
) -> dict:
    """Commitment arms (with counter grants), the flexible field, and a tier ladder with the pool."""
    sigils = load_sigils()
    offer = pool + controls()
    counter = {s: 1.0 for s in pool if sigils[s].get("touchesOpponents")}
    arm_runs = {}
    for a in ARCHES:
        if not any(a in sigils[s]["archetypes"] for s in pool):
            continue
        p = E.commitment(
            step, a, offerable=offer, boards=boards_commit, tier=1, model=model, counter=counter or None
        )
        arm_runs[a] = load([p], sigils).runs
    pf = E.flexible(step, offerable=offer, boards=boards_field, tier=1, model=model)
    field = load([pf], sigils).runs
    pl = E.run(
        step,
        "ladder",
        {
            "seed": 31,
            "boards": ladder_boards,
            "tier": ladder_tiers[0],
            "tierB": ladder_tiers[1],
            "model": model,
            "offerable": sorted(offer),
            "measured": {},
            "grantProb": 0.0,
            "cleanShare": 1.0,
            "perturb": 0.0,
            "arm": {"type": "tournament"},
        },
    )
    lad = load([pl], sigils).runs
    bw = {}
    for r in lad:
        bw.setdefault(r.board, []).append(r.margin)
    ladder = float(np.mean([1.0 if sum(v) > 0 else 0.0 if sum(v) < 0 else 0.5 for v in bw.values()]))
    commit = R.commitment_summary(arm_runs, sigils)
    commit["winning_share"] = R.winning_shares(field, sigils)
    inval = R.invalidation(arm_runs, set(counter), 1.0) if counter else {}
    return {"commit": commit, "field": field, "ladder": ladder, "invalidation": inval, "arm_runs": arm_runs}


def standalone_readings(step: str, enablers: list, model: str, boards: int = 3000) -> dict:
    """Enablers alone (no sigil shop): lift over a blank and over the flat control, by acquiring shop."""
    sigils = load_sigils()
    if not enablers:
        return {}
    p = E.standalone(step, enablers=enablers, boards=boards, model=model, tier=1)
    m = A.measure(step, "standalone", [p], sigils, scale_of(model), boot=150)
    out = {}
    from . import fit as F

    for e in enablers:
        if e not in m.readings:
            continue
        i = m.des.ids.index(e)
        rar = sigils[e]["rarity"]
        res = {}
        for label, ref in (("standalone", BLANK[rar]), ("standaloneVsControl", CONTROL[rar])):
            if ref not in m.des.ids:
                continue
            j = m.des.ids.index(ref)
            for shop in (1, 3, 5):

                def val(c, shop=shop, j=j, i=i):
                    return (
                        c[i * F.N_TERMS]
                        + c[i * F.N_TERMS + 2] * (shop - 3.5)
                        - c[j * F.N_TERMS]
                        - c[j * F.N_TERMS + 2] * (shop - 3.5)
                    )

                pt = val(m.fit.coef)
                lo, hi = F.interval(
                    np.array([val(b) for b in m.fit.boot]), pt, np.array([val(b) for b in m.fit.boot_raw])
                )
                res[f"{label}{shop}"] = tuple(float(F.to_points(x, m.k)) for x in (pt, lo, hi))
        out[e] = res
    return out


def fun(step: str, m: A.Measurement, arms: dict, pool: list) -> dict:
    sigils = load_sigils()
    lam = float(np.mean(json.loads(open(latest_model()).read())["lambda"][1:7]))
    syn = R.synergy(m, sigils, lam)
    f = funscore.compute(arms["field"], arms["commit"], syn, arms["ladder"], simplicity_mean(pool, sigils))
    f["synergy_top"] = syn["top"]
    return f


def price_of(sid: str, sigils: dict) -> int:
    return PRICE[sigils[sid]["rarity"]]
