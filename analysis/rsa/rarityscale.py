"""Writes a pool fragment that multiplies the payoff amounts of one rarity (amounts lever).

    uv run python -m rsa.rarityscale <variant> <rarity|all> <factor> <fragment-name> [types]

+points, +mult, and nil points scale by the factor; ×mult scales its excess over 1. Amounts are
rounded to the pool's usual steps (5s for points, whole +mults, 0.05 for ×mults).
"""

import json
import sys

from . import it
from .compose import POOL


def nice(kind: str, a: float) -> float:
    if kind == "xmult":
        return round(round(a / 0.05) * 0.05, 2)
    if kind == "mult":
        return float(max(1, round(a)))
    return float(max(5, round(a / 5) * 5)) if a >= 10 else float(max(1, round(a)))


def main(
    name: str, rarity: str, factor: float, out: str, types: tuple = ("points", "mult", "xmult", "nilPoints")
) -> None:
    b = it.Variant.load(name).materialize()
    sig = {}
    for sid in b.pool:
        d = b.defs[sid]
        e = d["effect"]
        if rarity not in ("all", d["rarity"]) or e["type"] not in types:
            continue
        a = e["amount"]
        new = 1 + (a - 1) * factor if e["type"] == "xmult" else a * factor
        sig[sid] = {"effect": {"amount": nice(e["type"], new)}}
    (POOL / f"{out}.json").write_text(json.dumps({"sigils": sig}, indent=1) + "\n")
    print(f"{out}: {len(sig)} {rarity} payoffs ×{factor}")


if __name__ == "__main__":
    types = tuple(sys.argv[5].split(",")) if len(sys.argv) > 5 else ("points", "mult", "xmult", "nilPoints")
    main(sys.argv[1], sys.argv[2], float(sys.argv[3]), sys.argv[4], types)
