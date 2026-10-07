"""A focused grant tournament that oversamples new versions against the current pool."""

import json
import pickle
import sys

from . import analyze as A
from . import steps as S
from .common import RUNS, load_sigils


def run(step: str, boards: int) -> None:
    sig = load_sigils()
    pool = S.kept_ids(sig)
    new = sorted(s for s, d in sig.items() if d["status"] == "candidate" and d.get("createdIn") == step)
    old = sorted({r for s in new for r in sig[s]["design"].get("replaces", []) if r in sig})
    measured = pool + [s for s in new if s not in pool]
    w = {s: 0.5 for s in pool}
    for s in old:
        w[s] = 2.0
    for s in new:
        w[s] = 4.0
    m = S.measure_tournaments(
        step,
        measured,
        pool + S.controls(),
        S.latest_model(),
        boards,
        weights=w,
        name="focused",
        seed=71,
        calib_share=0.15,
    )
    pickle.dump(m, open(RUNS / step / "focused.pkl", "wb"))
    rows = []
    for s in new + old:
        r = m.readings.get(s)
        if r is None:
            continue
        rows.append(
            {
                "id": s,
                "new": s in new,
                "replaces": sig[s]["design"].get("replaces", []),
                "target": sig[s]["design"].get("target"),
                "lift": r.working,
                "lo": r.lift_lo + r.working - r.lift,
                "hi": r.lift_hi + r.working - r.lift,
                "fire": r.fire,
                "slope": r.slope,
                "C": sig[s]["simplicity"]["C"],
                "text": sig[s]["text"],
                "n": r.n,
            }
        )
    json.dump(rows, open(RUNS / step / "focused.json", "w"), indent=1, default=float)
    md = A.dashboard(m, sig, new + old + S.controls(), f"{step}: focused tournament (new versions vs old)")
    A.write_report(f"{step}-focused", md)
    print("done", flush=True)


if __name__ == "__main__":
    run(sys.argv[1], int(sys.argv[2]))
