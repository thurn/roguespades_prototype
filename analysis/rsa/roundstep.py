"""One optimization round's measurement step, end to end: experiments, fit, readings, retunes,
global scale, bands, model, estimates, and the dashboard."""

import json
import math
import pickle
import sys

from . import analyze as A
from . import rounds as RO
from . import steps as S
from .common import RUNS, load_sigils


def run(
    step: str,
    extra: list,
    builds: bool,
    recenter: bool,
    prev_pickle: str | None,
    calib: float = 0.2,
    boards_mult: float = 1.0,
) -> None:
    sig = load_sigils()
    pool = S.kept_ids(sig)
    prev_m = pickle.load(open(prev_pickle, "rb")) if prev_pickle else None
    nb = int(S.boards_for(len(pool) + len(extra) + 8, half_width=1.5) * boards_mult)
    out = RO.measure_round(
        step, pool, extra=extra, prev_m=prev_m, calib_share=calib, builds=builds, boards=nb
    )
    m = out["m"]
    sig = load_sigils()
    notes = {}
    # Bands: round 1 proposes re-centered bands.
    if recenter:
        prop = RO.recenter_bands(m, pool, sig)
        A.save_bands(prop["after"])
        notes["bands"] = prop
    # Global scale first (damped), then relative amounts.
    gs = A.game_stats(out["field"])
    f = A.global_factor(gs)
    notes["global_factor"] = f
    notes["global_changed"] = A.apply_global(f, sig, pool + S.controls(), step) if f != 1.0 else 0
    sig = load_sigils()
    stand = out["standalone"]
    extra_metrics = {}
    b = A.bands()
    for e, res in stand.items():
        mets = {}
        for key, (v, lo, hi) in res.items():
            base = key.rstrip("135")
            band = tuple(b["standalone"] if base == "standalone" else b["standaloneVsControl"])
            label = (
                "Standalone: lift over a blank, bought at shop "
                if base == "standalone"
                else "Standalone: lift over the flat control, bought at shop "
            ) + key[-1]
            mets[key] = A._metric(key, label, v, lo, hi, band)
        extra_metrics[e] = mets
    for a, v in out["arms"].get("invalidation", {}).items():
        notes.setdefault("invalidation", {})[a] = v
    A.write_estimates(
        m,
        sig,
        [s for s in pool + extra if s in m.readings],
        [f"reports/{step}-dashboard.md", f"docs/sigils/rounds/{step}.md"],
        extra_metrics,
    )
    sig = load_sigils()
    changes = A.retune(m, sig, pool, step)
    notes["retunes"] = [(c[0], c[1], c[2]) for c in changes]
    notes["flat"] = A.structural_flags(m, sig, pool)
    prev = json.load(open(S.latest_model()))
    A.export_model(step, m, sig, gs, prev)
    fun = out["fun"]
    md_extra = f"Commitment: {json.dumps(out['arms']['commit'], default=float)}\n\nLadder (tier 1 vs tier 0 with the pool, boards): {out['arms']['ladder']:.3f}\n\nGlobal factor {f:.2f}\n"
    if builds:
        md_extra += "\nBuild chasers:\n\n" + "\n".join(
            f"- {r['win']:.3f} [{r['lo']:.3f}, {r['hi']:.3f}] ({r['runs']} runs): {', '.join(r['build'])}"
            for r in out["builds"]
        )
    md = A.dashboard(m, sig, pool + extra + S.controls(), f"{step}: dashboard", md_extra, fun)
    A.write_report(
        f"{step}-dashboard",
        md,
        {
            "fun": fun,
            "notes": notes,
            "standalone": stand,
            "commit": out["arms"]["commit"],
            "ladder": out["arms"]["ladder"],
            "builds": out.get("builds"),
            "game": gs,
        },
    )
    pickle.dump(m, open(RUNS / step / "measurement.pkl", "wb"))
    json.dump(notes, open(RUNS / step / "notes.json", "w"), indent=1, default=float)
    print(
        "fun",
        round(fun["score"], 1),
        [(x["name"], round(x["score"], 2)) for x in fun["families"]],
        flush=True,
    )
    print("done", flush=True)


if __name__ == "__main__":
    step = sys.argv[1]
    extra = [x for x in sys.argv[2].split(",") if x]
    run(
        step,
        extra,
        builds=sys.argv[3] == "1",
        recenter=sys.argv[4] == "1",
        prev_pickle=sys.argv[5] if len(sys.argv) > 5 else None,
    )
    _ = math
