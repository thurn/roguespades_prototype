"""Writes a variant from a base variant plus pool fragments (data/search/pool/<name>.json).

    uv run python -m rsa.compose <name> <base> [fragment ...] [--rules '{"k": v}']

A fragment holds "sigils" overrides, "add" (new sigil defs), and "combos" (registered pairs).
Later fragments override earlier ones.
"""

import argparse
import json

from . import it
from .common import ROOT

POOL = ROOT / "data" / "search" / "pool"


def main() -> None:
    ap = argparse.ArgumentParser(prog="compose")
    ap.add_argument("name")
    ap.add_argument("base")
    ap.add_argument("fragments", nargs="*")
    ap.add_argument("--rules", default="")
    a = ap.parse_args()
    spec: dict = {"base": a.base, "sigils": {}, "add": [], "combos": []}
    for f in a.fragments:
        fr = json.loads((POOL / f"{f}.json").read_text())
        for sid, over in fr.get("sigils", {}).items():
            spec["sigils"][sid] = it.deep_merge(spec["sigils"].get(sid, {}), over)
            if "type" in over.get("effect", {}):
                spec["sigils"][sid]["effect"] = over["effect"]
        added = {d["id"] for d in fr.get("add", [])}
        spec["add"] = [d for d in spec["add"] if d["id"] not in added] + fr.get("add", [])
        spec["combos"] += [c for c in fr.get("combos", []) if c not in spec["combos"]]
    if a.rules:
        spec["rules"] = json.loads(a.rules)
    (it.VARIANTS / f"{a.name}.json").write_text(json.dumps(spec, indent=1, ensure_ascii=False) + "\n")
    print(
        f"wrote {a.name}: {len(spec['sigils'])} overrides, {len(spec['add'])} new, {len(spec['combos'])} combos"
    )


if __name__ == "__main__":
    main()
