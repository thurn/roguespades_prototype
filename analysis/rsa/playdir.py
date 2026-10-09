"""Plays one autoplayed game of a variant with `rsim play` and prints its shop and score lines.

    uv run python -m rsa.playdir <variant> <seed> [--tier 0]

Writes runs/play/<variant>/ (sigils with the variant's pool as "kept") and the session log
runs/play/<variant>-<seed>.jsonl.
"""

import argparse
import json
import shutil

from . import it
from .common import RUNS, rsim


def main() -> None:
    ap = argparse.ArgumentParser(prog="playdir")
    ap.add_argument("variant")
    ap.add_argument("seed")
    ap.add_argument("--tier", default="0")
    a = ap.parse_args()
    b = it.Variant.load(a.variant).materialize()
    d = RUNS / "play" / a.variant
    if d.exists():
        shutil.rmtree(d)
    d.mkdir(parents=True)
    pool = set(b.pool)
    for sid, x in b.defs.items():
        x = dict(x)
        x["status"] = "kept" if sid in pool else "cut"
        (d / f"{sid}.json").write_text(json.dumps(x, ensure_ascii=False))
    out = RUNS / "play" / f"{a.variant}-{a.seed}.jsonl"
    rsim("play", "--seed", a.seed, "--tier", a.tier, "--dir", str(d), "--model", b.model,
         "--rules", str(b.rules), "--out", str(out), quiet=True)  # fmt: skip
    for line in out.read_text().splitlines():
        e = json.loads(line)
        m = e.get("msg", "")
        if any(k in m for k in ("offer", "buys", "sells", "reroll", "Round", "score", "wins", "final")):
            print(m)


if __name__ == "__main__":
    main()
