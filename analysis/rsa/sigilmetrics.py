"""Per-sigil metrics on one variant: lift over a same-rarity flat-points control and trigger rate.

Team A is granted one sigil free at shop `shop` and otherwise shops as usual against a flexible
team B; every arm plays the same boards (`pairsyn.run_arm`). Lift is the board win-rate difference
against the none arm, minus the same-rarity control's; the fire rate is the share of held rounds in
which the sigil's payoff fired.

    uv run python -m rsa.sigilmetrics <variant> [--n 400] [--tier 0] [--shop 3] [--ids a,b]
"""

import argparse
import json

import numpy as np

from . import it, pairsyn
from .common import REPORTS

CONTROL = {
    "common": "control-common",
    "uncommon": "control-uncommon",
    "rare": "control-rare",
    "legendary": "control-legendary",
}


def board_wins(p) -> tuple:
    by: dict = {}
    for r in it._lines(p):
        by.setdefault(r["board"], []).append(r["win"])
    keys = sorted(by)
    return np.array(keys), np.array([np.mean(by[k]) for k in keys])


def fire_rate(p, sid: str) -> float:
    held = fired = 0
    for r in it._lines(p):
        t = r["teams"][0]
        idx = [i for i, s in enumerate(t["sigils"]) if s["id"] == sid]
        if not idx:
            continue
        shop = t["sigils"][idx[0]]["shop"]
        for rn, rd in enumerate(t["rounds"], 1):
            if rn < shop:
                continue
            held += 1
            fired += any(x["i"] in idx and x["f"] > 0 for x in rd["sig"])
    return fired / held if held else float("nan")


def measure(name: str, env: "it.Env", n: int, tier: int, shop: int, ids: list | None = None) -> list:
    b = it.Variant.load(name).materialize()
    ids = ids or b.pool
    k0, w0 = board_wins(pairsyn.run_arm(b, env, [], n, tier, shop))
    ctrl = {}
    for rar, cid in CONTROL.items():
        k, w = board_wins(pairsyn.run_arm(b, env, [cid], n, tier, shop))
        assert np.array_equal(k, k0)
        ctrl[rar] = w - w0
    rows = []
    for sid in ids:
        d = b.defs[sid]
        p = pairsyn.run_arm(b, env, [sid], n, tier, shop)
        k, w = board_wins(p)
        assert np.array_equal(k, k0)
        diff = (w - w0) - ctrl[d["rarity"]]
        rows.append(
            {
                "id": sid,
                "rarity": d["rarity"],
                "C": d.get("simplicity", {}).get("C"),
                "arch": [a for a in d["archetypes"] if a != "Generic"],
                "role": d.get("role"),
                "lift": float(100 * diff.mean()),
                "lift_ci": float(100 * 1.645 * diff.std() / np.sqrt(len(diff))),
                "raw": float(100 * (w - w0).mean()),
                "fire": fire_rate(p, sid),
                "text": d.get("text"),
            }
        )
    return rows


def main() -> None:
    ap = argparse.ArgumentParser(prog="sigilmetrics")
    ap.add_argument("variant")
    ap.add_argument("--n", type=int, default=400)
    ap.add_argument("--tier", type=int, default=0)
    ap.add_argument("--shop", type=int, default=3)
    ap.add_argument("--seeds", default="dev")
    ap.add_argument("--ids", default="")
    a = ap.parse_args()
    env = it.Env(a.tier, "wp", a.seeds, 0)
    rows = measure(a.variant, env, a.n, a.tier, a.shop, [x for x in a.ids.split(",") if x] or None)
    rows.sort(key=lambda r: r["lift"])
    out = REPORTS / "search-2" / f"sigils-{a.variant}-{env.key()}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(rows, indent=1) + "\n")
    for r in rows:
        print(
            f"{r['lift']:+6.1f} ±{r['lift_ci']:4.1f} raw {r['raw']:+5.1f} fire {r['fire']:.2f} C{r['C']} "
            f"{r['rarity'][:1]} {'/'.join(r['arch']) or '-':18s} {r['id']}: {r['text']}"
        )


if __name__ == "__main__":
    main()
