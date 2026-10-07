"""The fun score (GDD §12): six weighted families, each the mean of banded sub-metrics."""

import numpy as np

from .analyze import game_stats


def band_score(x, lo=None, hi=None, tol_lo=None, tol_hi=None) -> float:
    if x is None or (isinstance(x, float) and np.isnan(x)):
        return float("nan")
    if lo is not None and x < lo:
        return max(0.0, 1 - (lo - x) / (lo - tol_lo)) if tol_lo is not None and lo != tol_lo else 0.0
    if hi is not None and x > hi:
        return max(0.0, 1 - (x - hi) / (tol_hi - hi)) if tol_hi is not None and tol_hi != hi else 0.0
    return 1.0


def _fam(name, weight, subs):
    vals = [s["score"] for s in subs if not np.isnan(s["score"])]
    return {
        "name": name,
        "weight": weight,
        "score": float(np.mean(vals)) if vals else float("nan"),
        "subs": subs,
    }


def _sub(name, value, score, fmt="{:.2f}"):
    v = "n/a" if value is None or (isinstance(value, float) and np.isnan(value)) else fmt.format(value)
    return {"name": name, "value": v, "score": score}


def close_and_live(gs):
    return [
        _sub(
            "median margin / winner",
            gs["medianMarginShare"],
            band_score(gs["medianMarginShare"], hi=0.20, tol_hi=0.40),
        ),
        _sub(
            "trailer after round 5 wins",
            gs["trailerAfter5Wins"],
            band_score(gs["trailerAfter5Wins"], lo=0.25, tol_lo=0.10),
        ),
        _sub("rounds 1-4 share", gs["earlyShare"], band_score(gs["earlyShare"], lo=0.20, tol_lo=0.10)),
    ]


def skill_and_bidding(gs, ladder):
    return [
        _sub("stronger tier beats tier 0 (boards)", ladder, band_score(ladder, lo=0.65, tol_lo=0.50)),
        _sub(
            "late overtricks per made",
            gs["lateOvertricks"],
            band_score(gs["lateOvertricks"], hi=1.0, tol_hi=2.0),
        ),
        _sub("set rate", gs["setRate"], band_score(gs["setRate"], lo=0.10, hi=0.25, tol_lo=0.0, tol_hi=0.40)),
    ]


def compute(
    field_runs, commit: dict, synergy: dict, ladder: float, simplicity: float, boot: int = 200, seed: int = 3
) -> dict:
    """`commit`: {"online": share, "committed_win": mean, "per_archetype": {a: win}, "winning_share": {a: share}}.
    `synergy`: {"same_arch_gain": mean ratio, "strong": n, "strong_cross": n}."""
    gs = game_stats(field_runs)
    fams = []
    fams.append(_fam("Close and live", 25, close_and_live(gs)))
    fams.append(
        _fam(
            "Commitment works",
            15,
            [
                _sub(
                    "online by round 4",
                    commit.get("online"),
                    band_score(commit.get("online"), lo=0.60, tol_lo=0.30),
                ),
                _sub(
                    "committed win rate",
                    commit.get("committed_win"),
                    band_score(commit.get("committed_win"), lo=0.50, tol_lo=0.35),
                ),
            ],
        )
    )
    per = commit.get("per_archetype", {})
    arch_scores = [band_score(w, lo=0.40, hi=0.60, tol_lo=0.25, tol_hi=0.75) for w in per.values()]
    ws = commit.get("winning_share", {})
    top = max(ws.values()) if ws else float("nan")
    fams.append(
        _fam(
            "Archetypes viable",
            15,
            [
                _sub(
                    "archetypes with committed win 40-60%",
                    float(np.mean(arch_scores)) if arch_scores else float("nan"),
                    float(np.mean(arch_scores)) if arch_scores else float("nan"),
                ),
                _sub(
                    "largest archetype share of winning flexible builds",
                    top,
                    band_score(top, hi=0.25, tol_hi=0.50),
                ),
            ],
        )
    )
    fams.append(
        _fam(
            "Synergy and combos",
            15,
            [
                _sub(
                    "same-archetype pair gain over parts",
                    synergy.get("same_arch_gain"),
                    band_score(synergy.get("same_arch_gain"), lo=0.25, tol_lo=0.0),
                ),
                _sub(
                    "strong pairs (>=50% over parts)",
                    synergy.get("strong"),
                    min(1.0, (synergy.get("strong") or 0) / 15),
                    "{:.0f}",
                ),
                _sub(
                    "strong cross-archetype pairs",
                    synergy.get("strong_cross"),
                    min(1.0, (synergy.get("strong_cross") or 0) / 5),
                    "{:.0f}",
                ),
            ],
        )
    )
    fams.append(_fam("Skill and bidding", 15, skill_and_bidding(gs, ladder)))
    fams.append(_fam("Simplicity", 15, [_sub("mean S", simplicity, simplicity)]))

    def total(fs):
        return sum(f["weight"] * (0 if np.isnan(f["score"]) else f["score"]) for f in fs)

    score = total(fams)
    # Cluster bootstrap over field boards for the run-measured families.
    clean = [r for r in field_runs if r.clean]
    by_board: dict = {}
    for r in clean:
        by_board.setdefault(r.board, []).append(r)
    keys = list(by_board)
    rng = np.random.default_rng(seed)
    samples = []
    for _ in range(boot if keys else 0):
        pick = rng.choice(len(keys), len(keys))
        rs = [r for j in pick for r in by_board[keys[j]]]
        g = game_stats(rs)
        f2 = [dict(f) for f in fams]
        f2[0] = _fam("Close and live", 25, close_and_live(g))
        f2[4] = _fam("Skill and bidding", 15, skill_and_bidding(g, ladder))
        samples.append(total(f2))
    lo, hi = np.quantile(samples, [0.05, 0.95]) if samples else (score, score)
    return {"score": score, "lo": float(lo), "hi": float(hi), "families": fams, "game": gs}
