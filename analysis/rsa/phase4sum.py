"""Markdown tables from the Phase 5 replicate reports (reports/search-2/phase4-final<rep>-t<tier>-wp.json).

uv run python -m rsa.phase4sum
"""

import json

import numpy as np

from .common import REPORTS
from .funscore import FAMILIES, WEIGHTS

NAMES = {
    "base-game": "Base game",
    "rec3": "**Recommendation**",
    "ru3-sym": "Runner-up: symmetric set, 8 slots",
    "ru3-slots8": "Runner-up: flat set, 8 slots",
    "search1": "Search 1 (its own shop model)",
    "search1-L": "Search 1 (this search's shop model)",
    "plain": "Plain Spades",
}


def load(tier: int) -> list:
    out = []
    for rep in range(5):
        p = REPORTS / "search-2" / f"phase5-final{rep}-t{tier}-wp.json"
        if p.exists():
            out.append(json.loads(p.read_text()))
    return out


def fam(x) -> str:
    return "—" if x is None or (isinstance(x, float) and np.isnan(x)) else f"{x:.2f}"


def scores(tier: int) -> str:
    reps = load(tier)
    if not reps:
        return ""
    lines = [
        f"Tier {tier}, {len(reps)} replicate(s) on final seeds: mean fun score, the range of the replicates' "
        "90% intervals, and family scores (0–1, mean of replicates).",
        "",
        "| Design | Fun (v4) | Replicate intervals | v3 | "
        + " | ".join(f"{f} ({w})" for f, w in zip(FAMILIES, WEIGHTS, strict=True))
        + " |",
        "| --- | --- | --- | --- | " + " | ".join("---" for _ in FAMILIES) + " |",
    ]
    for key, label in NAMES.items():
        rs = [r["measures"][key] for r in reps if key in r["measures"]]
        if not rs:
            continue
        sc = np.mean([r["score"] for r in rs])
        lo = min(r["lo"] for r in rs)
        hi = max(r["hi"] for r in rs)
        fams = np.nanmean(np.array([r["families"] for r in rs], dtype=float), axis=0)
        lines.append(
            f"| {label} | {sc:.1f} | {lo:.1f}–{hi:.1f} | {np.mean([r['v3'] for r in rs]):.1f} | "
            + " | ".join(fam(float(x)) for x in fams)
            + " |"
        )
    return "\n".join(lines)


def paired(tier: int) -> str:
    reps = load(tier)
    if not reps:
        return ""
    lines = [
        f"Tier {tier}: paired difference against the base game on the same boards, per replicate, with 90% "
        "cluster-bootstrap intervals; then each family's difference in fun points (mean of replicates).",
        "",
        "| Design | "
        + " | ".join(f"Rep {i}" for i in range(len(reps)))
        + " | Mean | "
        + " | ".join(FAMILIES)
        + " |",
        "| --- | "
        + " | ".join("---" for _ in reps)
        + " | --- | "
        + " | ".join("---" for _ in FAMILIES)
        + " |",
    ]
    for key, label in NAMES.items():
        ps = [r["paired"].get(key) for r in reps]
        if not all(ps):
            continue
        cells = [f"{p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}]" for p in ps]
        fd = np.mean([p["fam_diff"] for p in ps], axis=0)
        lines.append(
            f"| {label} | "
            + " | ".join(cells)
            + f" | {np.mean([p['diff'] for p in ps]):+.2f} | "
            + " | ".join("—" if np.isnan(x) else f"{x:+.2f}" for x in fd)
            + " |"
        )
    return "\n".join(lines)


def rep_s(rs: list) -> str:
    rp = [r.get("replay") for r in rs]
    if not all(rp):
        return "— | — | — | — | — |"
    g = [r["game"] for r in rs]
    return (
        f"{np.mean([x['overlap'] for x in rp]):.3f} | {np.mean([x['concentration'] for x in rp]):.3f} | "
        f"{np.mean([x['inPlay'] for x in rp]):.3f} | {np.mean([x['commonsSeen'] for x in g]):.3f} | "
        f"{np.mean([x['emptyOffers'] for x in g]):.0f} |"
    )


def raw(tier: int) -> str:
    reps = load(tier)
    if not reps:
        return ""
    lines = [
        f"Tier {tier} sub-metrics (mean of replicates):",
        "",
        "| Design | Margin / winner | Trailer after 5 wins | Rounds 1–4 share | Set rate | Bags per team-run | "
        "Online by round 4 | Committed win | Top archetype share | Tier 2 beats tier 0 | Mean C | Synergy gain / strong / cross | "
        "Build overlap | Concentration | Pool in play | Commons seen | Empty offers |",
        "| --- " * 17 + "|",
    ]
    for key, label in NAMES.items():
        rs = [r["measures"][key] for r in reps if key in r["measures"]]
        if not rs:
            continue

        def g(k, rs=rs):
            return np.mean([r["game"][k] for r in rs])

        def c(k, rs=rs):
            return np.mean([r["commit"][k] for r in rs])

        top = np.mean([max(r["game"]["winningShare"].values()) for r in rs])
        syn = [r.get("synergy") for r in rs]
        syn_s = (
            f"{np.mean([s['same_arch_gain_pts'] for s in syn]):.2f} / {np.mean([s['strong_pts'] for s in syn]):.1f} / "
            f"{np.mean([s['strong_pts_cross'] for s in syn]):.1f}"
            if all(syn)
            else "—"
        )
        lines.append(
            f"| {label} | {g('medianMarginShare'):.3f} | {g('trailerAfter5Wins'):.3f} | {g('earlyShare'):.3f} | "
            f"{g('setRate'):.3f} | {g('bagsPerRun'):.1f} | {c('online'):.3f} | {c('committed_win'):.3f} | {top:.3f} | "
            f"{np.mean([r['ladder'] for r in rs]):.3f} | {np.mean([r['meanC'] for r in rs]):.2f} | {syn_s} | "
            + rep_s(rs)
        )
    return "\n".join(lines)


if __name__ == "__main__":
    for t in (1, 2):
        for f in (scores, paired, raw):
            s = f(t)
            if s:
                print(s + "\n")
