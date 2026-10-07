"""Archetype-level readings: commitment arms, winning-build shares, invalidation, synergy, builds."""

import math

import numpy as np

from . import fit as F
from .common import ODDS, PRICE, category

ARCH = ["Suits", "Spades", "Ranks", "BidHigh", "Nil", "LowCards", "Rainbow", "Streaks", "Exact"]
RANKS = {"A": 14, "K": 13, "Q": 12, "J": 11}


def card_matches(f: dict, ident: int) -> bool:
    suit, rank = ident // 13, ident % 13 + 2
    letters = "CDHS"
    if "suit" in f and letters.index(f["suit"]) != suit:
        return False
    if "notSuit" in f and letters.index(f["notSuit"]) == suit:
        return False
    if "rank" in f and RANKS.get(f["rank"], int(f["rank"]) if f["rank"].isdigit() else 0) != rank:
        return False
    if "ranks" in f:
        lo, hi = (RANKS.get(x, int(x) if x.isdigit() else 0) for x in f["ranks"])
        if not lo <= rank <= hi:
            return False
    return True


def payoff_filter(d: dict):
    on = d["effect"].get("on") or {}
    return on.get("card")


def online(team, arch: str, sigils: dict, by_shop: int = 4) -> bool:
    held = [sid for sid, shop, sold, _g in team.held if shop <= by_shop and sold > by_shop]
    pays = [
        s
        for s in held
        if s in sigils and arch in sigils[s]["archetypes"] and sigils[s].get("role") != "enabler"
    ]
    ens = [
        s
        for s in held
        if s in sigils and arch in sigils[s]["archetypes"] and sigils[s].get("role") != "payoff"
    ]
    if len(pays) < 2:
        return False
    if ens:
        return True
    filters = [payoff_filter(sigils[s]) for s in pays]
    filters = [f for f in filters if f]
    cards = [c for c in team.cards if c[1] <= by_shop]
    rewarded = sum(1 for c in cards if any(card_matches(f, c[3]) for f in filters))
    return rewarded >= 3


def commitment_summary(arm_runs: dict, sigils: dict) -> dict:
    """`arm_runs`: archetype -> list of runs where team A (index 0) commits."""
    per, onl = {}, []
    for arch, runs in arm_runs.items():
        if not runs:
            continue
        per[arch] = float(np.mean([r.win for r in runs]))
        onl.extend(online(r.teams[0], arch, sigils) for r in runs)
    return {
        "per_archetype": per,
        "committed_win": float(np.mean(list(per.values()))) if per else float("nan"),
        "online": float(np.mean(onl)) if onl else float("nan"),
        "online_by_arch": {
            a: float(np.mean([online(r.teams[0], a, sigils) for r in rs])) for a, rs in arm_runs.items() if rs
        },
    }


def build_archetype(team, sigils: dict):
    counts = {}
    for sid, _shop, sold, _g in team.held:
        if sold < 9 or sid not in sigils or sigils[sid].get("role") == "enabler":
            continue
        for a in sigils[sid]["archetypes"]:
            if a in ARCH:
                counts[a] = counts.get(a, 0) + 1
    if not counts:
        return None
    a, n = max(counts.items(), key=lambda x: x[1])
    return a if n >= 2 else None


def winning_shares(field_runs: list, sigils: dict) -> dict:
    wins = []
    for r in field_runs:
        if r.win == 0.5:
            continue
        t = r.teams[0] if r.win == 1.0 else r.teams[1]
        wins.append(build_archetype(t, sigils))
    n = max(1, len(wins))
    return {a: sum(1 for w in wins if w == a) / n for a in ARCH}


def invalidation(arm_runs: dict, counter_ids: set, k: float) -> dict:
    """Per archetype: win-rate drop when the opponents were granted any opponent-touching sigil."""
    out = {}
    for arch, runs in arm_runs.items():
        hit = [r.win for r in runs if any(g[0] in counter_ids for g in r.teams[1].grants)]
        clear = [r.win for r in runs if not any(g[0] in counter_ids for g in r.teams[1].grants)]
        if len(hit) >= 20 and len(clear) >= 20:
            out[arch] = 100 * (np.mean(clear) - np.mean(hit))
    return out


def gross(ft: F.Fit, i: int, d: dict, lam: float) -> float:
    return float(ft.coef[i * F.N_TERMS]) + lam * PRICE[d["rarity"]]


def synergy(m, sigils: dict, lam: float, min_n: int = 60) -> dict:
    ft, des = m.fit, m.des
    idx = [
        i
        for i, s in enumerate(des.ids)
        if s in sigils and sigils[s].get("source") != "control" and des.exposures[i] >= min_n
    ]
    same, strong, strong_cross, top = [], 0, 0, []
    for a in range(len(idx)):
        for b in range(a + 1, len(idx)):
            i, j = idx[a], idx[b]
            di, dj = sigils[des.ids[i]], sigils[des.ids[j]]
            parts = gross(ft, i, di, lam) + gross(ft, j, dj, lam)
            if parts <= 0.01:
                continue
            dot = float(ft.V[i] @ ft.V[j])
            gain = dot / parts
            shared = set(di["archetypes"]) & set(dj["archetypes"]) - {"Generic"}
            if shared:
                same.append(gain)
            if gain >= 0.5:
                strong += 1
                if not shared:
                    strong_cross += 1
            top.append((dot, des.ids[i], des.ids[j], gain))
    top.sort(reverse=True)
    return {
        "same_arch_gain": float(np.mean(same)) if same else float("nan"),
        "strong": strong,
        "strong_cross": strong_cross,
        "top": [(a, b, float(F.to_points(d, m.k)), g) for d, a, b, g in top[:20]],
    }


# ---------------------------------------------------------------------------------------------
# Build search


def attain(d: dict, pool_counts: dict, by_shop: int = 5, offers_per_shop: float = 4.5) -> float:
    q = ODDS[d["rarity"]] / max(1, pool_counts[d["rarity"]])
    return 1 - (1 - q) ** (offers_per_shop * by_shop)


def build_search(m, sigils: dict, offerable: list, width: int = 60, max_size: int = 5, top: int = 10) -> list:
    """Beam search over loadouts maximizing the fitted team value weighted by attainability."""
    ft, des = m.fit, m.des
    index = {s: i for i, s in enumerate(des.ids)}
    cands = [s for s in offerable if s in index and sigils[s].get("source") != "control"]
    counts = {}
    for s in offerable:
        counts[sigils[s]["rarity"]] = counts.get(sigils[s]["rarity"], 0) + 1

    def value(build):
        v = 0.0
        for s in build:
            i = index[s]
            mine = set(sigils[s]["archetypes"]) - {"Generic"}
            coh = sum(1 for o in build if o != s and mine & set(sigils[o]["archetypes"]))
            v += ft.coef[i * F.N_TERMS] + ft.coef[i * F.N_TERMS + 3] * min(coh, 2)
        for a in range(len(build)):
            for b in range(a + 1, len(build)):
                v += float(ft.V[index[build[a]]] @ ft.V[index[build[b]]])
        p = math.prod(attain(sigils[s], counts) for s in build)
        return v * p, v, p

    beam = sorted(([s] for s in cands), key=lambda b: -value(b)[0])[:width]
    best = list(beam)
    for _ in range(max_size - 1):
        nxt = {}
        for b in beam:
            for s in cands:
                if s in b:
                    continue
                nb = tuple(sorted(b + [s]))
                if nb not in nxt:
                    nxt[nb] = value(list(nb))[0]
        beam = [list(b) for b, _ in sorted(nxt.items(), key=lambda x: -x[1])[:width]]
        best.extend(beam)
    scored = sorted({tuple(sorted(b)): value(list(b)) for b in best}.items(), key=lambda x: -x[1][0])
    return [
        {"build": list(b), "score": v[0], "value_pts": float(F.to_points(v[1], m.k)), "attain": v[2]}
        for b, v in scored[:top]
    ]


def category_counts(ids: list, sigils: dict) -> dict:
    out = {}
    for s in ids:
        c = category(sigils[s])
        out[c] = out.get(c, 0) + 1
    return out
