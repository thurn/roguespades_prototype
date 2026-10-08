"""The fun score v3 (GDD §10): six weighted families, each the mean of banded sub-metrics.

Each sub-metric scores 1 inside its band and falls linearly to 0 at its tolerance. A family scores
the mean of its sub-metrics, and the fun score is the weighted sum, from 0 to 100.
"""

import numpy as np

FAMILIES = [
    "Close and live",
    "Commitment works",
    "Archetypes viable",
    "Synergy and combos",
    "Skill and bidding",
    "Simplicity",
]
WEIGHTS = [20, 15, 15, 15, 10, 25]


def band_score(x, lo=None, hi=None, tol_lo=None, tol_hi=None) -> float:
    if x is None or (isinstance(x, float) and np.isnan(x)):
        return float("nan")
    if lo is not None and x < lo:
        return max(0.0, 1 - (lo - x) / (lo - tol_lo)) if tol_lo is not None and lo != tol_lo else 0.0
    if hi is not None and x > hi:
        return max(0.0, 1 - (x - hi) / (tol_hi - hi)) if tol_hi is not None and tol_hi != hi else 0.0
    return 1.0


def _mean(v: list) -> float:
    v = [x for x in v if not np.isnan(x)]
    return float(np.mean(v)) if v else float("nan")


def families(gs: dict, commit: dict, syn: dict | None, ladder: float, mean_c: float) -> list:
    """Family scores (0-1) in FAMILIES order.

    `gs`: medianMarginShare, trailerAfter5Wins, earlyShare, setRate, winningShare.
    `commit`: online, committed_win, per_archetype.
    `syn`: margin-point synergy: same_arch_gain_pts, strong_pts, strong_pts_cross.
    `mean_c`: the pool's mean sigil complexity C.
    """
    close = [
        # Plain Spades runs about 0.45 of the winner's score (design iteration evidence).
        band_score(gs["medianMarginShare"], hi=0.45, tol_hi=0.75),
        band_score(gs["trailerAfter5Wins"], lo=0.25, tol_lo=0.10),
        band_score(gs["earlyShare"], lo=0.20, tol_lo=0.10),
    ]
    com = [
        band_score(commit["online"], lo=0.60, tol_lo=0.30),
        band_score(commit["committed_win"], lo=0.50, tol_lo=0.35),
    ]
    per = commit["per_archetype"]
    arch = [band_score(w, lo=0.40, hi=0.60, tol_lo=0.25, tol_hi=0.75) for w in per.values()]
    top = max(gs["winningShare"].values())
    av = [_mean(arch), band_score(top, hi=0.25, tol_hi=0.50)]
    if syn is None:
        sy = [float("nan")]
    else:
        sy = [
            band_score(syn["same_arch_gain_pts"], lo=0.25, tol_lo=0.0),
            min(1.0, (syn["strong_pts"] or 0) / 15),
            min(1.0, (syn["strong_pts_cross"] or 0) / 5),
        ]
    sk = [
        band_score(ladder, lo=0.65, tol_lo=0.50),
        band_score(gs["setRate"], lo=0.10, hi=0.25, tol_lo=0.0, tol_hi=0.40),
    ]
    simp = band_score(mean_c, hi=3.0, tol_hi=7.0)
    return [_mean(close), _mean(com), _mean(av), _mean(sy), _mean(sk), simp]


def total(fams: list) -> float:
    return float(sum(w * (0 if np.isnan(f) else f) for w, f in zip(WEIGHTS, fams, strict=True)))
