"""Phase 1 of design search 2: screen the candidate reservoir.

Every sigil in a variant's data that isn't in its pool, plus the new designs in a pool fragment, is
screened against the grammar, GDD §5, and the search plan's rules:

- no Opening that picks cards at random ("N cards your team holds become ...");
- no control-only grammar (a "beyond" rider, a rank-range destination) or catch-up trigger;
- C at most 4 at common and 5 above;
- one candidate per signature and rarity, and none repeating a pool sigil's signature at its rarity.

    uv run python -m rsa.reservoir <variant> [--fragment new] [--out reservoir]

Writes data/search/pool/<out>.json: the screened ids (with the new designs as "add") and the reason
each other sigil was screened out.
"""

import argparse
import json

from . import it
from .compose import POOL

MAX_C = {"common": 4, "uncommon": 5, "rare": 5, "legendary": 5}


def reason(d: dict) -> str | None:
    e = d["effect"]
    on = e.get("on") or {}
    if d["id"].startswith("control-"):
        return "control"
    if d.get("lint"):
        return "lint"
    if e["type"] == "become" and (e.get("count") or 0) > 0:
        return "random Opening"
    if e.get("ranks") or on.get("beyond"):
        return "control-only grammar"
    if on.get("event") == "behind":
        return "catch-up trigger"
    c = d.get("simplicity", {}).get("C", 99)
    if c > MAX_C[d["rarity"]]:
        return f"C {c} above the {d['rarity']} ceiling"
    return None


def main() -> None:
    ap = argparse.ArgumentParser(prog="reservoir")
    ap.add_argument("variant")
    ap.add_argument("--fragment", default="")
    ap.add_argument("--out", default="reservoir")
    a = ap.parse_args()
    v = it.Variant.load(a.variant)
    if a.fragment:
        fr = json.loads((POOL / f"{a.fragment}.json").read_text())
        v.spec = dict(v.spec)
        v.spec["add"] = v.spec.get("add", []) + fr.get("add", [])
    b = v.materialize()
    pool = set(b.pool)
    taken = {(b.defs[s].get("signature"), b.defs[s]["rarity"]) for s in pool}
    out, why = [], {}
    # Kept-then-cut sigils and new designs first, then the older reservoir by id.
    order = sorted(
        (s for s in b.defs if s not in pool),
        key=lambda s: (b.defs[s].get("status") not in ("cut", "candidate", None), s),
    )
    for sid in order:
        d = b.defs[sid]
        r = reason(d)
        key = (d.get("signature"), d["rarity"])
        if r is None and key in taken:
            r = "repeats a signature at its rarity"
        if r:
            why[sid] = r
            continue
        taken.add(key)
        out.append(sid)
    res = {"ids": out, "screened": why}
    if a.fragment:
        res["add"] = fr.get("add", [])
    (POOL / f"{a.out}.json").write_text(json.dumps(res, indent=1, ensure_ascii=False) + "\n")
    by: dict = {}
    for sid in out:
        by[b.defs[sid]["rarity"]] = by.get(b.defs[sid]["rarity"], 0) + 1
    reasons: dict = {}
    for r in why.values():
        k = r.split(" above")[0] if r.startswith("C ") else r
        reasons[k] = reasons.get(k, 0) + 1
    print(f"{len(out)} candidates {by}; screened out {len(why)}: {reasons}")


if __name__ == "__main__":
    main()
