"""Optimization rounds: the full measurement, retunes, global scale, builds, and focused checks."""

import json
import math

import numpy as np

from . import analyze as A
from . import arms as R
from . import experiments as E
from . import steps as S
from .common import RUNS, load_sigils
from .records import load


def adaptive_weights(ids: list, model_path: str | None, floor: float = 0.5) -> dict:
    """Grant exposure weighted by each sigil's uncertainty in the previous fit, with a floor."""
    if not model_path:
        return {s: 1.0 for s in ids}
    prev = json.loads(open(model_path).read()).get("sigils", {})
    ses = [prev[s]["se"] for s in ids if s in prev and prev[s].get("se")]
    med = float(np.median(ses)) if ses else 1.0
    out = {}
    for s in ids:
        se = prev.get(s, {}).get("se")
        out[s] = max(floor, min(3.0, (se / med) ** 2)) if se else 2.0
    return out


def chasers(step: str, builds: list, pool: list, model: str, start: int = 60) -> list:
    """Build-chaser arms against the flexible field at tier 1, narrowed by successive halving."""
    sigils = load_sigils()
    alive = list(range(len(builds)))
    results = {i: [] for i in alive}
    budget, offset, rnd = start, 0, 0
    while True:
        for i in alive:
            p = E.chaser(
                step,
                f"chaser-{i}-r{rnd}",
                builds[i]["build"],
                offerable=pool + S.controls(),
                boards=budget,
                model=model,
                board_offset=offset,
                seed=41 + i,
            )
            results[i].extend(r.win for r in load([p], sigils).runs)
        offset += budget
        rnd += 1
        if len(alive) <= 3:
            break
        alive.sort(key=lambda i: -np.mean(results[i]))
        alive = alive[: max(3, len(alive) // 2)]
        budget *= 2
    out = []
    for i in range(len(builds)):
        w = results[i]
        p = float(np.mean(w))
        se = float(np.std(w) / math.sqrt(len(w)))
        out.append(
            {
                **builds[i],
                "win": p,
                "lo": p - 1.645 * se,
                "hi": p + 1.645 * se,
                "runs": len(w),
                "finalist": i in alive,
            }
        )
    return sorted(out, key=lambda x: -x["win"])


def recenter_bands(m: A.Measurement, pool: list, sigils: dict) -> dict:
    """Proposes re-centered bands when most of the pool sits far to one side of a band."""
    b = A.bands()
    lifts = [m.readings[s].working for s in pool if s in m.readings]
    fires = [m.readings[s].fire for s in pool if s in m.readings and sigils[s].get("role") != "enabler"]
    dec = [m.readings[s].decisive for s in pool if s in m.readings and sigils[s].get("category") == "points"]
    prop = {"before": json.loads(json.dumps(b)), "notes": []}
    med_lift = float(np.nanmedian(lifts)) if lifts else float("nan")
    # Target lifts: keep the rarity ordering, centered on the pool's own spread when it is far off.
    if lifts and abs(med_lift - np.mean(list(b["delta"].values()))) > 3:
        prop["notes"].append(
            f"Median pool lift {med_lift:+.1f} pts is far from the targets; targets kept, amounts move."
        )
    med_fire = float(np.nanmedian(fires)) if fires else float("nan")
    if fires and med_fire < b["fire"][0]:
        b["fire"] = [max(5.0, round(med_fire / 2)), None]
        prop["notes"].append(f"Median fire rate {med_fire:.0f}%: base fire band lowered to {b['fire'][0]}%.")
    med_dec = float(np.nanmedian(dec)) if dec else float("nan")
    if dec and med_dec > 3 * b["decisive"][0]:
        b["decisive"] = [round(med_dec / 2), None]
        prop["notes"].append(f"Median decisive share {med_dec:.0f}%: band raised to {b['decisive'][0]}%.")
    prop["after"] = b
    prop["medians"] = {"lift": med_lift, "fire": med_fire, "decisive": med_dec}
    return prop


def side_by_side(m: A.Measurement, pairs: list) -> list:
    rows = []
    for old, new in pairs:
        ro, rn = m.readings.get(old), m.readings.get(new)
        rows.append(
            {
                "old": old,
                "new": new,
                "old_lift": None if ro is None else ro.working,
                "old_lo": None if ro is None else ro.lift_lo + ro.working - ro.lift,
                "old_hi": None if ro is None else ro.lift_hi + ro.working - ro.lift,
                "new_lift": None if rn is None else rn.working,
                "new_lo": None if rn is None else rn.lift_lo + rn.working - rn.lift,
                "new_hi": None if rn is None else rn.lift_hi + rn.working - rn.lift,
                "old_fire": None if ro is None else ro.fire,
                "new_fire": None if rn is None else rn.fire,
            }
        )
    return rows


def runs_of(step: str, name: str) -> list:
    return load([RUNS / step / f"{name}.jsonl"], load_sigils()).runs


def top_pairs(m: A.Measurement, pool: list, n: int = 20) -> list:
    """The strongest predicted pairs among pool sigils, from the factorization machine."""
    idx = {s: i for i, s in enumerate(m.des.ids)}
    ids = [s for s in pool if s in idx]
    out = []
    for a in range(len(ids)):
        for b in range(a + 1, len(ids)):
            out.append((float(m.fit.V[idx[ids[a]]] @ m.fit.V[idx[ids[b]]]), ids[a], ids[b]))
    out.sort(reverse=True)
    return [[a, b] for _, a, b in out[:n]]


def measure_round(
    step: str,
    pool: list,
    extra: list | None = None,
    prev_m: A.Measurement | None = None,
    calib_share: float = 0.2,
    builds: bool = False,
    boards: int | None = None,
) -> dict:
    """The full measurement of an optimization round. Returns everything the round needs."""
    import pickle

    from .common import RUNS

    sigils = load_sigils()
    model = S.latest_model()
    measured = pool + list(extra or [])
    weights = adaptive_weights(measured, model)
    pairs = top_pairs(prev_m, pool) if prev_m is not None else None
    nb = boards or S.boards_for(len(measured) + 8, half_width=1.5)
    offer = pool + S.controls()
    m = S.measure_tournaments(
        step,
        measured,
        offer,
        model,
        nb,
        weights=weights,
        name="tournament",
        seed=61,
        calib_share=calib_share,
        pairs=pairs,
    )
    arms = S.pool_arms(step, pool, model)
    enablers = [s for s in pool if sigils[s].get("role") == "enabler"]
    stand = S.standalone_readings(step, enablers, model)
    fun = S.fun(step, m, arms, pool)
    out = {
        "m": m,
        "arms": {k: v for k, v in arms.items() if k != "arm_runs" and k != "field"},
        "standalone": stand,
        "fun": fun,
        "pairs": pairs,
        "boards": nb,
    }
    if builds:
        bs = R.build_search(m, sigils, pool)
        out["builds"] = chasers(step, bs, pool, model)
    pickle.dump(m, open(RUNS / step / "measurement.pkl", "wb"))
    json.dump(
        {k: v for k, v in out.items() if k != "m"},
        open(RUNS / step / "round.json", "w"),
        indent=1,
        default=lambda o: float(o) if hasattr(o, "__float__") else str(o),
    )
    out["field"] = arms["field"]
    return out
