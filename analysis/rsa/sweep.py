"""One-factor sweeps and challenger screens: each level is a variant on top of the incumbent,
measured on the incumbent's boards and reported as paired fun-score differences by family.

    uv run python -m rsa.sweep <sweep-file.json> [--tier 1] [--seeds dev] [--field N] ...

A sweep file: {"incumbent": "inc-a", "levels": {"name": {variant spec without "base"}, ...}}.
Each level is written to data/search/variants/<name>.json with "base": incumbent.
"""

import argparse
import json

from . import it, itcli
from .common import REPORTS


def write_level(name: str, spec: dict, incumbent: str) -> None:
    spec = dict(spec)
    spec.setdefault("base", incumbent)
    p = it.VARIANTS / f"{name}.json"
    text = json.dumps(spec, indent=1) + "\n"
    if not p.exists() or p.read_text() != text:
        p.write_text(text)


def row(name: str, p: dict) -> str:
    g = p["b"]["game"]
    c = p["b"]["commit"]
    rp = p["b"]["replay"]
    fams = " | ".join(
        f"{d:+.2f} [{lo:+.1f},{hi:+.1f}]"
        for d, lo, hi in zip(p["fam_diff"], p["fam_lo"], p["fam_hi"], strict=True)
    )
    return (
        f"| {name} | {p['b']['score']:.1f} | {p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}] | {p['v3_diff']:+.2f} | {fams} | "
        f"{g['medianMarginShare']:.3f} | {g['trailerAfter5Wins']:.3f} | {g['earlyShare']:.3f} | {g['setRate']:.3f} | "
        f"{g['bagsPerRun']:.1f} | {c['online']:.3f} | {c['committed_win']:.3f} | {max(g['winningShare'].values()):.3f} | "
        f"{p['b']['ladder']:.3f} | {rp['overlap']:.3f} | {rp['concentration']:.3f} | {rp['inPlay']:.3f} | "
        f"{g['commonsSeen']:.3f} | {g['emptyOffers']:.0f} |"
    )


HEADER = (
    "| Variant | Fun | Paired diff (90%) | v3 diff | Close | Commit | Arch | Syn | Skill | Simpl | Rep | Margin | "
    "Trail5 | Early | Set | Bags | Online | CWin | TopShare | Ladder | Overlap | Conc | InPlay | Commons seen | "
    "Empty |\n" + "| --- " * 25 + "|"
)


def main() -> None:
    ap = argparse.ArgumentParser(prog="sweep")
    ap.add_argument("file")
    ap.add_argument("--tier", type=int, default=1)
    ap.add_argument("--util", default="wp")
    ap.add_argument("--seeds", default="dev")
    ap.add_argument("--rep", type=int, default=0)
    ap.add_argument("--field", type=int, default=1500)
    ap.add_argument("--commit", type=int, default=400)
    ap.add_argument("--ladder", type=int, default=400)
    ap.add_argument("--no-ladder", action="store_true")
    ap.add_argument("--only", default="")
    ap.add_argument("--syn", type=int, default=0)
    ap.add_argument("--syn-tier", type=int, default=0)
    ap.add_argument("--syn-same", type=int, default=60)
    ap.add_argument("--syn-cross", type=int, default=20)
    ap.add_argument("--syn-replicate", default="")
    a = ap.parse_args()
    sw = json.loads(open(a.file).read())
    inc = sw["incumbent"]
    env = it.Env(a.tier, a.util, a.seeds, a.rep)
    sizes = itcli.sizes_of(a)
    syn = itcli.syn_of(a)
    levels = sw["levels"]
    if a.only:
        keep = set(a.only.split(","))
        levels = {k: v for k, v in levels.items() if k in keep}
    for name, spec in levels.items():
        write_level(name, spec, inc)
    base = it.measure(inc, env, sizes, syn)
    for seeds in a.syn_replicate.split(","):
        if seeds:
            itcli.log_use(
                it.Env(0, a.util, seeds, a.rep),
                f"synergy replicate for sweep {sw.get('name')}",
                f"{a.syn} boards",
            )
    print(it.fmt_measure(inc, base.compute()), flush=True)
    out = REPORTS / "search-2" / f"sweep-{sw.get('name', 'sweep')}-{env.key()}.md"
    out.parent.mkdir(parents=True, exist_ok=True)
    lines = [f"# Sweep {sw.get('name', '')} ({env.key()}, incumbent {inc})", "", HEADER]
    for name in levels:
        m = it.measure(name, env, sizes, syn)
        p = it.paired(base, m)
        print(it.fmt_measure(name, p["b"]), "\n  " + it.fmt_paired(p), flush=True)
        lines.append(row(name, p))
        out.write_text("\n".join(lines) + "\n")
        itcli.save(f"cmp-{inc}-{name}-{env.key()}", {"a": inc, "b": name, "result": p})
        itcli.log_use(env, f"compare {inc} vs {name}", f"{p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}]")


if __name__ == "__main__":
    main()
