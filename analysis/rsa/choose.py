"""Keep decisions for a draft pass: statuses, one line of reasoning each, estimates, retunes."""

import json
import pickle

from . import analyze as A
from .common import RUNS, load_sigils, save_sigil


def choose(
    step: str, keep: list, rarities: list, hold: list | None = None, reports: list | None = None
) -> dict:
    hold = hold or []
    m = pickle.load(open(RUNS / step / "measurement.pkl", "rb"))
    sig = load_sigils()
    crit_p = RUNS / step / "critic.json"
    crit = {c["id"]: c for c in json.load(open(crit_p))} if crit_p.exists() else {}
    rows = {r["id"]: r for r in json.load(open(RUNS / step / "table.json"))}
    cands = [
        s
        for s, d in sig.items()
        if d["status"] == "candidate" and d["rarity"] in rarities and d["source"] != "control" and s in rows
    ]
    missing = [k for k in keep if k not in cands]
    assert not missing, missing
    kept_all = [s for s, d in sig.items() if d["status"] == "kept"] + keep
    fam_kept: dict = {}
    for k in kept_all:
        fam_kept.setdefault(sig[k].get("family"), []).append(k)

    def fmt(r):
        return f"lift {r['lift']:+.1f} [{r['lo']:+.1f}, {r['hi']:+.1f}] pts (n {r['n']}), fire {r['fire']:.0f}%, C {r['C']}"

    reps = (reports or []) + [f"reports/{step}-tournament.md", f"docs/sigils/rounds/{step}.md"]
    A.write_estimates(m, sig, [s for s in cands + list(A.CONTROL.values()) if s in m.readings], reps)
    sig = load_sigils()
    for s in cands:
        d = sig[s]
        r = rows[s]
        if s in hold:
            A.history(
                d,
                step,
                f"{step}: held as a candidate: {fmt(r)}; its hook was fixed after this measurement, so it is re-measured next step.",
            )
        elif s in keep:
            d["status"] = "kept"
            if r["lift"] > 0:
                why = "beats the same-rarity control"
            elif (r.get("slope") or 0) > 2:
                why = (
                    f"below the control but with a real amount slope ({r['slope']:+.1f} pts per doubling), so retuning can lift it; kept for "
                    + ", ".join(d["archetypes"])
                )
            else:
                why = (
                    "kept for coverage of "
                    + ", ".join(d["archetypes"])
                    + "; a target for the optimization rounds"
                )
            A.history(d, step, f"{step}: kept: {fmt(r)}; {why}.")
        else:
            d["status"] = "cut"
            same = [k for k in fam_kept.get(d.get("family"), []) if k != s]
            c = crit.get(s, {})
            if same:
                why = f"near-duplicate of kept {same[0]}"
            elif r["hi"] < -8:
                why = "far below the control, and one retune step can't close the gap"
            elif c.get("elegance", 3) <= 2:
                why = "critic: " + c.get("concerns", "")
            else:
                why = "the kept pool covers its archetypes with stronger or simpler pieces"
            A.history(d, step, f"{step}: cut: {fmt(r)}; {why}.")
        save_sigil(d)
    sig = load_sigils()
    changes = A.retune(m, sig, keep, step)
    flat = A.structural_flags(m, sig, keep)
    out = {"keep": keep, "retunes": [(c[0], c[1], c[2]) for c in changes], "flat": flat}
    json.dump(out, open(RUNS / step / "choice.json", "w"), indent=1)
    gs = A.game_stats(m.loaded.runs)
    prev = json.load(
        open(sorted((RUNS.parent / "data" / "models").glob("*.json"), key=lambda p: p.stat().st_mtime)[-1])
    )
    A.export_model(step, m, sig, gs, prev)
    return out
