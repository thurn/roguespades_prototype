"""Keep proposals for draft passes: rank candidates by projected lift after retuning, simplicity,
coverage, and near-duplicates. The orchestrator reviews the proposal and decides."""

import math

from .analyze import bands

MINORS = {"LowCards", "Rainbow", "Streaks", "Exact"}
MAJORS = {"Suits", "Spades", "Ranks", "BidHigh", "Nil"}


def projected(row: dict, target: float) -> float:
    """Lift after up to one retune step (amount x0.5 to x2) when the amount slope is real."""
    lift, slope, lo = row["lift"], row.get("slope"), row.get("slope_lo")
    if slope is None or (isinstance(slope, float) and math.isnan(slope)) or lo is None or lo <= 0:
        return lift
    gap = target - lift
    return lift + max(-slope, min(slope, gap))


def bid_tense(d: dict) -> bool:
    e = d["effect"]
    on = e.get("on") or {}
    return e.get("per") == "contractTrick" or (
        on.get("event") in ("bid", "make") and ("min" in on or "max" in on)
    )


def propose(
    rows: list, sigils: dict, rarity: str, n_keep: int, already: list, critic: dict | None = None
) -> list:
    b = bands()
    target = b["delta"][rarity]
    critic = critic or {}
    scored = []
    for r in rows:
        d = sigils[r["id"]]
        if d.get("source") == "control" or critic.get(r["id"], {}).get("verdict") == "drop":
            continue
        p = projected(r, target)
        el = critic.get(r["id"], {}).get("elegance", 3)
        score = min(p, target + 2) - 0.25 * (r["C"] or 0) + 0.5 * (el - 3)
        if r["hi"] < 0:
            score -= 2
        scored.append((score, p, r))
    scored.sort(key=lambda x: -x[0])
    keep = list(already)
    fam = {sigils[s].get("family") for s in keep}
    arch = {}
    enablers = sum(1 for s in keep if sigils[s].get("role") != "payoff")
    tense = sum(1 for s in keep if bid_tense(sigils[s]))
    out = []
    for score, p, r in scored:
        if len(keep) >= n_keep:
            break
        d = sigils[r["id"]]
        tags = set(d["archetypes"]) - {"Generic"}
        bonus = 0.0
        if d.get("family") in fam:
            bonus -= 2.0
        if d.get("role") != "payoff" and enablers < n_keep / 3:
            bonus += 1.0
        if bid_tense(d) and tense < n_keep / 4:
            bonus += 0.5
        for t in tags:
            cap = 3 if t in MINORS else 9
            if arch.get(t, 0) >= cap:
                bonus -= 1.0
        if score + bonus < -3:
            continue
        keep.append(r["id"])
        fam.add(d.get("family"))
        for t in tags:
            arch[t] = arch.get(t, 0) + 1
        enablers += d.get("role") != "payoff"
        tense += bid_tense(d)
        out.append((r["id"], round(score + bonus, 2), round(p, 1)))
    return out
