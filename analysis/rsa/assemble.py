"""Phase 2 of design search 2: assemble pools at the floor from the incumbent plus screened
candidates, each by an explicit rule.

    uv run python -m rsa.assemble <rule> <name> [--base s1-res] [--lifts s1-res,s1-pool]

Rules:
- coverage: each major archetype gets about 10 commons and 9 uncommons, each minor about 5 and 5
  (Rainbow stays out, as in search 1); every archetype has an enabler good alone at common or
  uncommon and at least two payoffs with card filters where the reservoir has them.
- value: the highest-lift candidates per tier, no archetype holding more than a sixth of a tier.
- restore: search 1's screened cuts (Rainbow included) back in, then the best candidates.

Every rule keeps the incumbent pool, respects the power ceiling (lift at most 12) and the trigger
guide (payoffs fire in at least 10% of held rounds), adds at most one sigil per signature, and
never adds a sigil whose signature the pool already has at another rarity. It fills each tier to
its floor (60 / 60 / 20 / 5) and drops the weakest legendary above 6.
"""

import argparse
import json

from . import it
from .common import REPORTS
from .compose import POOL
from .retarget import ALIVE

FLOOR = {"common": 60, "uncommon": 60, "rare": 20, "legendary": 5}
MAJOR = ["Suits", "Spades", "Ranks", "BidHigh", "Nil"]
MINOR = ["LowCards", "Streaks", "Exact"]
QUOTA = {"common": (10, 5), "uncommon": (9, 5)}
S1_CUTS = [
    "ca-ace-win-two-mult", "ca-diamond-hold-eight", "ca-diamond-win-two", "ca-high-spade-hold",
    "ca-rainbow-lead-mult", "ca-spade-lead-three", "ca-three-aces-mult", "cb-honor-hold", "cb-spade-seven",
    "cb-three-row-mult", "cc-low-last-mult", "r2-diamond-honor-hold", "ra-rainbow-first-x", "seed-e-spade-win",
    "seed-rainbow-first", "seed-rainbow-lead-x", "seed-rainbow-mult", "seed-rainbow-x", "ua-diamond-honor-win",
    "ua-rainbow-first-contract", "ua-rainbow-first-mult", "ua-side-raise", "ub-make8-trick-mult",
]  # fmt: skip


def primary(d: dict) -> str:
    tags = [a for a in d["archetypes"] if a != "Generic"]
    return tags[0] if tags else "Generic"


def load_lifts(names: list) -> dict:
    out = {}
    for n in names:
        for r in json.loads((REPORTS / "search-2" / f"sigils-{n}-dev0-t0-wp.json").read_text()):
            out.setdefault(r["id"], r)
    return out


def ok(r: dict, d: dict) -> bool:
    if r["lift"] > 12:
        return False
    payoff = d["effect"]["type"] in ("points", "mult", "xmult", "nilPoints", "grow")
    return not payoff or (r["fire"] == r["fire"] and r["fire"] >= 0.10)


def main() -> None:
    ap = argparse.ArgumentParser(prog="assemble")
    ap.add_argument("rule", choices=["coverage", "value", "restore"])
    ap.add_argument("name")
    ap.add_argument("--base", default="s1-res")
    ap.add_argument("--incumbent", default="search1")
    ap.add_argument("--lifts", default="s1-res,s1-pool")
    ap.add_argument("--additive-commons", action="store_true", help="no new ×mult commons")
    ap.add_argument("--alive", action="store_true", help="add only live candidates; swap dead pool sigils")
    a = ap.parse_args()
    b = it.Variant.load(a.base).materialize()
    inc = it.Variant.load(a.incumbent).materialize()
    defs = b.defs
    lifts = load_lifts(a.lifts.split(","))
    res = json.loads((POOL / "reservoir.json").read_text())["ids"]
    pool = list(inc.pool)
    sigs = {defs[s].get("signature") for s in pool}
    cands = [s for s in res if s in lifts and ok(lifts[s], defs[s]) and defs[s].get("signature") not in sigs]
    # Openings that change the opponents' cards feel like attacks; they stay out.
    cands = [s for s in cands if not (defs[s].get("touchesOpponents") and defs[s].get("role") == "enabler")]
    if a.alive:
        cands = [s for s in cands if lifts[s]["lift"] >= min(3.5, ALIVE[defs[s]["rarity"]])]
    if a.additive_commons:
        cands = [
            s for s in cands if not (defs[s]["rarity"] == "common" and defs[s]["effect"]["type"] == "xmult")
        ]
    if a.rule != "restore":
        cands = [s for s in cands if "Rainbow" not in defs[s]["archetypes"]]
    cands.sort(key=lambda s: -lifts[s]["lift"])

    def count(tier, arch=None):
        return sum(
            1 for s in pool if defs[s]["rarity"] == tier and (arch is None or primary(defs[s]) == arch)
        )

    def take(s):
        pool.append(s)
        sigs.add(defs[s].get("signature"))

    def free(s):
        return s not in pool and defs[s].get("signature") not in sigs

    if a.rule == "restore":
        for s in S1_CUTS:
            if s in res and free(s):
                take(s)
    if a.rule == "coverage":
        for tier, (qmaj, qmin) in QUOTA.items():
            for arch in MAJOR + MINOR:
                q = qmaj if arch in MAJOR else qmin
                # An enabler good alone first, then payoffs with card filters, then the best.
                ens = [s for s in cands if free(s) and defs[s]["rarity"] == tier and primary(defs[s]) == arch]
                has_en = any(
                    arch in defs[s]["archetypes"]
                    and defs[s].get("role") != "payoff"
                    and lifts.get(s, {"lift": 0})["lift"] > 0
                    and defs[s]["rarity"] in ("common", "uncommon")
                    for s in pool
                )
                if not has_en:
                    for s in ens:
                        if defs[s].get("role") == "enabler" and lifts[s]["lift"] > 0 and free(s):
                            take(s)
                            break
                for s in ens:
                    if count(tier, arch) >= q:
                        break
                    if free(s):
                        take(s)
    for tier, n in FLOOR.items():
        cap = max(1, (n + 5) // 6)
        for s in cands:
            if count(tier) >= n:
                break
            d = defs[s]
            if d["rarity"] != tier or not free(s):
                continue
            if a.rule == "value" and count(tier, primary(d)) >= cap:
                continue
            take(s)
    if a.alive:
        # Commons and uncommons only: they hold almost all the dead sigils, and the reservoir has
        # live replacements for them.
        dead = [
            s
            for s in pool
            if defs[s]["rarity"] in ("common", "uncommon")
            and lifts.get(s, {"lift": 99})["lift"] < ALIVE[defs[s]["rarity"]]
        ]
        for s in dead:
            tier, arch = defs[s]["rarity"], primary(defs[s])
            alt = [c for c in cands if free(c) and defs[c]["rarity"] == tier]
            same = [c for c in alt if primary(defs[c]) == arch]
            if not alt:
                continue
            pool.remove(s)
            sigs.discard(defs[s].get("signature"))
            take((same or alt)[0])
            print(f"  swap {s} ({lifts[s]['lift']:+.1f}) -> {(same or alt)[0]}")
    legs = sorted(
        (s for s in pool if defs[s]["rarity"] == "legendary"), key=lambda s: lifts.get(s, {"lift": 0})["lift"]
    )
    while len(legs) > 6:
        pool.remove(legs.pop(0))
    spec = {"base": a.base, "pool": sorted(pool)}
    (it.VARIANTS / f"{a.name}.json").write_text(json.dumps(spec, indent=1) + "\n")
    by = {t: count(t) for t in FLOOR}
    arch = {}
    for s in pool:
        k = (defs[s]["rarity"][0], primary(defs[s]))
        arch[k] = arch.get(k, 0) + 1
    print(f"{a.name}: {len(pool)} sigils {by}; added {len(pool) - len(inc.pool)}")
    print("  " + " ".join(f"{t}/{x}:{n}" for (t, x), n in sorted(arch.items())))


if __name__ == "__main__":
    main()
