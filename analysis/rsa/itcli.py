"""`uv run python -m rsa.itcli measure|compare ...`: fun-score measurements and paired comparisons
of design variants (see `rsa.it`). Holdout and final seed uses are logged automatically."""

import argparse
import datetime
import json
import subprocess

import numpy as np

from . import it, pairsyn
from .common import REPORTS, ROOT

LOG = ROOT / "docs" / "search" / "holdout-log.md"
OUTDIR = REPORTS / "search"


def commit() -> str:
    return subprocess.run(
        ["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, capture_output=True, text=True
    ).stdout.strip()


def log_use(env: it.Env, what: str, result: str) -> None:
    if env.seeds not in ("holdout", "final"):
        return
    LOG.parent.mkdir(parents=True, exist_ok=True)
    if not LOG.exists():
        LOG.write_text(
            "# Holdout and final seed uses\n\nEvery measurement on holdout or final seeds, appended by "
            "`rsa.itcli`. A holdout block retires after 20 uses.\n\n| When | Seeds | Env | Measurement | Result | Commit |\n"
            "| --- | --- | --- | --- | --- | --- |\n"
        )
    s = json.loads((it.ITER / "seeds.json").read_text())[env.seeds]
    now = datetime.datetime.now().strftime("%Y-%m-%d %H:%M")
    with open(LOG, "a") as f:
        f.write(
            f"| {now} | {env.seeds} block {s.get('block', 0)} | {env.key()} | {what} | {result} | {commit()} |\n"
        )


def sizes_of(a) -> it.Sizes:
    return it.Sizes(field=a.field, commit=a.commit, ladder=a.ladder, ladder_on=not a.no_ladder)


def syn_of(a):
    if not a.syn:
        return None
    rep = tuple(x for x in a.syn_replicate.split(",") if x)
    return pairsyn.estimator(n=a.syn, tier=a.syn_tier, n_same=a.syn_same, n_cross=a.syn_cross, replicate=rep)


def save(name: str, data: dict) -> None:
    OUTDIR.mkdir(parents=True, exist_ok=True)
    (OUTDIR / f"{name}.json").write_text(json.dumps(data, indent=1, default=float) + "\n")


def main() -> None:
    ap = argparse.ArgumentParser(prog="itcli")
    ap.add_argument("cmd", choices=["measure", "compare"])
    ap.add_argument("variants", nargs="+")
    ap.add_argument("--tier", type=int, default=1)
    ap.add_argument("--util", default="wp")
    ap.add_argument("--seeds", default="dev")
    ap.add_argument("--rep", type=int, default=0)
    ap.add_argument("--field", type=int, default=1500)
    ap.add_argument("--commit", type=int, default=400)
    ap.add_argument("--ladder", type=int, default=400)
    ap.add_argument("--no-ladder", action="store_true")
    ap.add_argument("--syn", type=int, default=0, help="pair-arm boards (0 = no synergy)")
    ap.add_argument("--syn-tier", type=int, default=0)
    ap.add_argument("--syn-same", type=int, default=60)
    ap.add_argument("--syn-cross", type=int, default=20)
    ap.add_argument(
        "--syn-replicate", default="", help="seed sets a registered strong pair must also pass on"
    )
    ap.add_argument("--boot", type=int, default=300)
    a = ap.parse_args()
    env = it.Env(a.tier, a.util, a.seeds, a.rep)
    sizes = sizes_of(a)
    ms = [it.measure(v, env, sizes, syn_of(a)) for v in a.variants]
    for seeds in a.syn_replicate.split(","):
        if seeds:
            log_use(
                it.Env(0, a.util, seeds, a.rep),
                f"synergy replicate for {' '.join(a.variants)}",
                f"{a.syn} boards",
            )
    stamp = {"env": env.key(), "commit": commit(), "sizes": vars(sizes), "syn": a.syn}
    if a.cmd == "measure":
        for v, m in zip(a.variants, ms, strict=True):
            r = it.interval(m, a.boot)
            print(it.fmt_measure(v, r), flush=True)
            save(f"{v}-{env.key()}", stamp | {"variant": v, "result": r})
            log_use(env, f"measure {v}", f"v2net {r['score']:.1f} [{r['lo']:.1f}, {r['hi']:.1f}]")
    else:
        base = ms[0]
        print(it.fmt_measure(a.variants[0], base.compute()), flush=True)
        for v, m in zip(a.variants[1:], ms[1:], strict=True):
            p = it.paired(base, m, a.boot)
            print(it.fmt_measure(v, p["b"]))
            print("  " + it.fmt_paired(p), flush=True)
            save(f"cmp-{a.variants[0]}-{v}-{env.key()}", stamp | {"a": a.variants[0], "b": v, "result": p})
            log_use(
                env,
                f"compare {a.variants[0]} vs {v}",
                f"v2net {p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}]",
            )
    _ = np


if __name__ == "__main__":
    main()
