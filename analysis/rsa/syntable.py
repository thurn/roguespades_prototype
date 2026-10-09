"""Pair-synergy table for one variant: every sampled pair's interaction and parts in margin points.

uv run python -m rsa.syntable <variant> [--n 300] [--same 60] [--cross 20]
"""

import argparse
import json

from . import it, pairsyn
from .common import REPORTS


def main() -> None:
    ap = argparse.ArgumentParser(prog="syntable")
    ap.add_argument("variant")
    ap.add_argument("--n", type=int, default=300)
    ap.add_argument("--same", type=int, default=60)
    ap.add_argument("--cross", type=int, default=20)
    ap.add_argument("--seeds", default="dev")
    a = ap.parse_args()
    env = it.Env(0, "wp", a.seeds, 0)
    b = it.Variant.load(a.variant).materialize()
    ps = pairsyn.measure(b, env, a.n, 0, a.same, a.cross, b.combos)
    v = ps.value()
    print({k: (round(x, 3) if isinstance(x, float) else x) for k, x in v.items()})
    rows = ps.table()
    for r in sorted(rows, key=lambda r: -r["IM"]):
        g = r["IM"] / r["PM"] if r["PM"] > 0 else float("nan")
        print(
            f"{'S' if r['same'] else 'X'} I {r['IM']:+8.0f} parts {r['PM']:+8.0f} ratio {g:+6.2f}  {r['a']} + {r['b']}"
        )
    out = REPORTS / "search-2" / f"syn-{a.variant}-{env.key()}.json"
    out.write_text(json.dumps({"value": v, "pairs": rows}, indent=1, default=float) + "\n")


if __name__ == "__main__":
    main()
