"""Per-sigil purchase rates on a variant's field runs: dead sigils, the most bought, and the winning
build archetypes.

    uv run python -m rsa.poolstats <variant> [--env dev0-t1-wp] [--dead 0.01]
"""

import argparse
import collections
import glob
import json

from . import it
from .common import REPORTS


def main() -> None:
    ap = argparse.ArgumentParser(prog="poolstats")
    ap.add_argument("variant")
    ap.add_argument("--env", default="dev0-t1-wp")
    ap.add_argument("--dead", type=float, default=0.01)
    ap.add_argument("--lifts", default="s1-res2,s1-res,s1-pool")
    a = ap.parse_args()
    b = it.Variant.load(a.variant).materialize()
    p = sorted(
        glob.glob(str(it.OUT / a.variant / b.digest / a.env / "field*.jsonl")),
        key=lambda x: -len(open(x, "rb").read()),
    )[0]
    lifts = {}
    for n in a.lifts.split(","):
        try:
            for r in json.loads((REPORTS / "search-2" / f"sigils-{n}-dev0-t0-wp.json").read_text()):
                lifts.setdefault(r["id"], r["lift"])
        except FileNotFoundError:
            pass
    bought = collections.Counter()
    offered = collections.Counter()
    n = 0
    for line in open(p):
        r = json.loads(line)
        for t in r["teams"]:
            n += 1
            bought.update({s["id"] for s in t["sigils"] if not s["grant"]})
            offered.update({x for _s, o in t.get("offers", []) for x in o})
    fa = it.field_arrays(p, b.defs, b.pool, 5)
    gs = it.field_stats(fa, __import__("numpy").arange(len(fa.board)))
    print("winning share:", {k: round(v, 2) for k, v in gs["winningShare"].items() if v > 0.01})
    rows = sorted(b.pool, key=lambda s: bought[s] / max(1, offered[s]))
    dead = [s for s in rows if bought[s] / n < a.dead]
    print(f"{len(dead)} dead of {len(b.pool)} (bought in < {a.dead:.0%} of team-runs):")
    for s in rows:
        d = b.defs[s]
        tag = "DEAD" if s in dead else "    "
        print(
            f"{tag} {d['rarity'][0]} buy {bought[s] / n:.3f} of-offered {bought[s] / max(1, offered[s]):.2f} "
            f"lift {lifts.get(s, float('nan')):+5.1f} {'/'.join(d['archetypes']):26s} {s}: {d.get('text')}"
        )


if __name__ == "__main__":
    main()
