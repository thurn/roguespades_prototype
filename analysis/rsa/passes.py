"""Draft passes: screen, tournaments with successive halving, calibration, and the dashboard."""

import json

from . import analyze as A
from . import draft as D
from . import steps as S
from .common import RUNS, load_sigils, rsim


def enumerate_commons(step: str) -> dict:
    """Enumerates common payoffs, screens them at hand level, sets their amounts, and sets aside
    the ones that almost never (or, for boolean conditions, almost always) fire."""
    rsim("enumerate", "--dir", "../data/sigils", "--step", step)
    S.gen_text()
    sig = load_sigils()
    ids = [s for s, d in sig.items() if d.get("source") == "enumerated" and d.get("status") == "candidate"]
    st = S.screen(step, ids)
    kept, aside = [], []
    for sid in ids:
        d = sig[sid]
        why = D.set_aside(st[sid], d)
        if why:
            D.cut(d, step, f"{step}: set aside by the hand-level screen: {why}.")
            aside.append(sid)
            continue
        a = D.screen_amount(d, st[sid])
        if a is not None:
            d["effect"]["amount"] = a
        A.history(
            d,
            step,
            f"{step}: screen fires in {100 * st[sid]['fire']:.0f}% of rounds "
            f"({st[sid]['fires']:.2f} per round); amount set to {a:g}.",
        )
        A.save_sigil(d)
        kept.append(sid)
    S.gen_text()
    return {"screened": len(ids), "passed": kept, "aside": aside, "screen": st}


def pre_tournament(step: str, ids: list, model: str, boards: int) -> A.Measurement:
    """A cheap tournament over the enumerated candidates; its most promising half goes on."""
    return S.measure_tournaments(
        step, ids, S.controls(), model, boards, name="enum-screen", seed=21, calib_share=0.0
    )


def measure_pass(
    step: str, ids: list, offerable: list, model: str, boards_a: int, boards_b: int
) -> A.Measurement:
    """Tournament A over all candidates, then tournament B concentrating grants on the more
    promising and the uncertain half; one combined fit with tier-1 calibration."""
    ma = S.measure_tournaments(
        step, ids + offerable, offerable, model, boards_a, name="tournament-a", seed=31, calib_share=0.0
    )
    w = S.promising_weights(ma, ids)
    for s in offerable:
        w.setdefault(s, 1.0)
    pa = RUNS / step / "tournament-a.jsonl"
    m = S.measure_tournaments(
        step,
        ids + offerable,
        offerable,
        model,
        boards_b,
        weights=w,
        name="tournament-b",
        seed=32,
        extra_paths=[pa],
        calib_share=0.15,
    )
    return m


def save_table(step: str, m: A.Measurement, ids: list) -> None:
    rows = []
    sig = load_sigils()
    for s in ids:
        r = m.readings.get(s)
        if r is None:
            continue
        d = sig[s]
        rows.append(
            {
                "id": s,
                "rarity": d["rarity"],
                "role": d.get("role"),
                "category": d.get("category"),
                "source": d.get("source"),
                "archetypes": d.get("archetypes"),
                "C": d.get("simplicity", {}).get("C"),
                "family": d.get("family"),
                "text": d.get("text"),
                "n": r.n,
                "lift": r.working,
                "lo": r.lift_lo + r.working - r.lift,
                "hi": r.lift_hi + r.working - r.lift,
                "t0": r.lift,
                "skill": r.skill,
                "fire": r.fire,
                "fire_c": r.fire_c,
                "decisive": r.decisive,
                "slope": r.slope,
                "slope_lo": r.slope_lo,
                "slope_hi": r.slope_hi,
            }
        )
    (RUNS / step / "table.json").write_text(json.dumps(rows, indent=1, default=float))
