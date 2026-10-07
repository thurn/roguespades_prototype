"""Command-line drivers for a draft pass's measurement step."""

import json
import pickle
import sys

from . import analyze as A
from . import draft as D
from . import passes
from . import steps as S
from .common import RUNS, load_sigils


def measure_draft(
    step: str, rarities: list, calib_share: float = 0.25, half_a: float = 2.5, half_b: float = 1.8, pairs=None
) -> None:
    sig = load_sigils()
    ids = sorted(
        s
        for s, d in sig.items()
        if d["status"] == "candidate" and d["rarity"] in rarities and d["source"] != "control"
    )
    # Hand-level screen: set aside candidates that almost never, or (boolean) almost always, fire.
    st = S.screen(step, ids)
    for s in list(ids):
        why = D.set_aside(st[s], sig[s])
        if why:
            D.cut(sig[s], step, f"{step}: set aside by the hand-level screen: {why}.")
            ids.remove(s)
    kept = S.kept_ids(sig)
    offer = kept + S.controls()
    model = S.latest_model()
    na = S.boards_for(len(ids) + 8, half_width=half_a)
    nb = S.boards_for(len(ids) + 8, half_width=half_b)
    print(len(ids), "candidates; boards", na, nb, flush=True)
    ma = S.measure_tournaments(
        step, ids, offer, model, na, name="tournament-a", seed=51, calib_share=0.0, pairs=pairs
    )
    w = S.promising_weights(ma, ids)
    m = S.measure_tournaments(
        step,
        ids,
        offer,
        model,
        nb,
        weights=w,
        name="tournament-b",
        seed=52,
        extra_paths=[RUNS / step / "tournament-a.jsonl"],
        calib_share=calib_share,
        pairs=pairs,
    )
    passes.save_table(step, m, ids + S.controls())
    pickle.dump(m, open(RUNS / step / "measurement.pkl", "wb"))
    sig = load_sigils()
    md = A.dashboard(m, sig, ids + S.controls(), f"Draft pass: {step} — tournament dashboard")
    A.write_report(f"{step}-tournament", md)
    json.dump({s: st[s] for s in st}, open(RUNS / step / "screen-stats.json", "w"))
    print("done", flush=True)


if __name__ == "__main__":
    measure_draft(sys.argv[1], sys.argv[2].split(","))
