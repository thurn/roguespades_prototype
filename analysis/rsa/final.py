"""Final-round extras: tier-2 spot checks of chosen sigils and the tier-2 ladder."""

import json
import pickle
import sys

import numpy as np

from . import analyze as A
from . import experiments as E
from . import steps as S
from .common import RUNS, load_sigils
from .records import load


def spot_checks(step: str, m: A.Measurement, n_each: int = 8, boards: int = 2400) -> dict:
    sig = load_sigils()
    pool = S.kept_ids(sig)
    rs = sorted((m.readings[s].working, s) for s in pool if s in m.readings)
    top = [s for _, s in rs[-n_each:]]
    bottom = [s for _, s in rs[:n_each]]
    near = sorted(pool, key=lambda s: abs(m.readings[s].working) if s in m.readings else 99)[:n_each]
    chosen = sorted(set(top + bottom + near))
    measured = {s: 1.0 for s in chosen}
    for c in S.controls():
        measured[c] = 1.0
    p = E.tournament(
        step,
        "tier2-spot",
        offerable=pool + S.controls(),
        measured=measured,
        boards=boards,
        tier=2,
        seed=91,
        model=S.latest_model(),
        clean_share=0.0,
    )
    m2 = A.measure(step, "tier2-spot", [p], sig, m.scale, boot=100)
    rows = []
    for s in chosen:
        if s in m2.readings:
            r2 = m2.readings[s]
            r = m.readings[s]
            rows.append(
                {
                    "id": s,
                    "working": r.working,
                    "tier2": r2.lift,
                    "t2_lo": r2.lift_lo,
                    "t2_hi": r2.lift_hi,
                    "agree": bool(r2.lift_lo <= r.working <= r2.lift_hi),
                    "group": "top" if s in top else "bottom" if s in bottom else "near",
                }
            )
    w = [x["working"] for x in rows]
    t = [x["tier2"] for x in rows]
    out = {
        "rows": rows,
        "corr": float(np.corrcoef(w, t)[0, 1]) if len(rows) > 2 else float("nan"),
        "agree_share": float(np.mean([x["agree"] for x in rows])) if rows else float("nan"),
    }
    return out


def ladder2(step: str, boards: int = 300) -> float:
    sig = load_sigils()
    pool = S.kept_ids(sig)
    p = E.run(
        step,
        "ladder-t2",
        {
            "seed": 93,
            "boards": boards,
            "tier": 2,
            "tierB": 0,
            "model": S.latest_model(),
            "offerable": sorted(pool + S.controls()),
            "measured": {},
            "grantProb": 0.0,
            "cleanShare": 1.0,
            "perturb": 0.0,
            "arm": {"type": "tournament"},
        },
    )
    bw: dict = {}
    for r in load([p], sig).runs:
        bw.setdefault(r.board, []).append(r.margin)
    return float(np.mean([1.0 if sum(v) > 0 else 0.0 if sum(v) < 0 else 0.5 for v in bw.values()]))


if __name__ == "__main__":
    step = sys.argv[1]
    m = pickle.load(open(RUNS / step / "measurement.pkl", "rb"))
    sc = spot_checks(step, m)
    lad = ladder2(step)
    json.dump(
        {"spot": sc, "ladder_t2_vs_t0": lad}, open(RUNS / step / "tier2.json", "w"), indent=1, default=float
    )
    print("spot corr", sc["corr"], "agree", sc["agree_share"], "ladder", lad, flush=True)
    print("done", flush=True)
