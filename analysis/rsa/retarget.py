"""Rescales payoff amounts toward a per-rarity target lift (a first guess, re-measured after).

    uv run python -m rsa.retarget <variant> <lifts-report> <fragment> [--ids a,b] [--over 12]

Payoffs whose measured lift is above `--over` (or every listed id) get amount × target / lift for
points, +mult, nil points, and growth steps, and their excess over 1 scaled for ×mult; amounts are
rounded to the pool's usual steps. Writes data/search/pool/<fragment>.json.
"""

import argparse
import json

from . import it
from .common import REPORTS
from .compose import POOL
from .rarityscale import nice

TARGET = {"common": 3.5, "uncommon": 5.0, "rare": 7.0, "legendary": 10.0}
# Below this lift the shop model values a sigil under its price and the AI never buys it.
ALIVE = {"common": 2.5, "uncommon": 3.5, "rare": 4.5, "legendary": 6.0}


def scaled(e: dict, f: float) -> dict | None:
    t = e["type"]
    if t == "grow":
        k = e.get("kind")
        return {"step": nice(k, 1 + (e["step"] - 1) * f if k == "xmult" else e["step"] * f)}
    if t not in ("points", "mult", "xmult", "nilPoints"):
        return None
    a = e["amount"]
    new = nice(t, 1 + (a - 1) * f if t == "xmult" else a * f)
    if t == "xmult":
        new = max(1.05, new)
    return {"amount": new}


def main() -> None:
    ap = argparse.ArgumentParser(prog="retarget")
    ap.add_argument("variant")
    ap.add_argument("report")
    ap.add_argument("fragment")
    ap.add_argument("--ids", default="")
    ap.add_argument("--over", type=float, default=12.0)
    ap.add_argument("--under", action="store_true", help="raise payoffs below the rarity's alive floor")
    ap.add_argument("--max-factor", type=float, default=2.5)
    a = ap.parse_args()
    b = it.Variant.load(a.variant).materialize()
    got: dict = {}
    for rep in a.report.split(","):
        for r in json.loads((REPORTS / "search-2" / f"sigils-{rep}-dev0-t0-wp.json").read_text()):
            got.setdefault(r["id"], r)
    rows = list(got.values())
    ids = {x for x in a.ids.split(",") if x}
    out = {}
    for r in rows:
        if r["id"] not in b.defs or (ids and r["id"] not in ids):
            continue
        d = b.defs[r["id"]]
        if a.under:
            if r["lift"] >= ALIVE[d["rarity"]]:
                continue
            f = 2.0 if r["lift"] <= 0 else min(a.max_factor, TARGET[d["rarity"]] / r["lift"])
        else:
            if (not ids and r["lift"] <= a.over) or r["lift"] <= 0:
                continue
            f = TARGET[d["rarity"]] / r["lift"]
        s = scaled(d["effect"], f)
        if s and s != {k: d["effect"].get(k) for k in s}:
            out[r["id"]] = {"effect": s}
            print(
                f"{r['id']}: lift {r['lift']:+.1f} {json.dumps(d['effect'].get('amount') or d['effect'].get('step'))} -> {s}"
            )
    (POOL / f"{a.fragment}.json").write_text(json.dumps({"sigils": out}, indent=1) + "\n")
    print(f"{len(out)} retargeted")


if __name__ == "__main__":
    main()
