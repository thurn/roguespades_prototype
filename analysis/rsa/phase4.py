"""Phase 4: the recommendation and runners-up against the base game on fresh final seeds, at tiers 1
and 2 with three replicates each, every family with a paired 90% interval; plain Spades for
reference.

    uv run python -m rsa.phase4 <tier> <rep> <base> <candidate> [...]
"""

import json
import sys

from . import it, itcli, pairsyn
from .common import REPORTS

SIZES = {
    1: it.Sizes(field=1500, commit=375, ladder=375),
    2: it.Sizes(field=500, commit=125, ladder=375),
}


def main(tier: int, rep: int, names: list) -> None:
    env = it.Env(tier, "wp", "final", rep)
    syn = pairsyn.estimator(n=400, tier=0)
    ms = {n: it.measure(n, env, SIZES[tier], syn) for n in names}
    plain = it.measure("plain-n", env, SIZES[tier])
    out = {"env": env.key(), "measures": {}, "paired": {}}
    for n, m in ms.items():
        r = it.interval(m)
        out["measures"][n] = r
        print(it.fmt_measure(n, r), flush=True)
        itcli.log_use(env, f"phase 4 measure {n}", f"{r['score']:.1f} [{r['lo']:.1f}, {r['hi']:.1f}]")
    r = it.interval(plain)
    out["measures"]["plain"] = r
    print(it.fmt_measure("plain", r), flush=True)
    base = ms[names[0]]
    for n in names[1:]:
        p = it.paired(base, ms[n])
        out["paired"][n] = p
        print(f"  {n} vs {names[0]}: " + it.fmt_paired(p), flush=True)
        itcli.log_use(
            env, f"phase 4 compare {names[0]} vs {n}", f"{p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}]"
        )
    d = REPORTS / "search-2"
    d.mkdir(parents=True, exist_ok=True)
    (d / f"phase4-{env.key()}.json").write_text(json.dumps(out, indent=1, default=float) + "\n")


if __name__ == "__main__":
    main(int(sys.argv[1]), int(sys.argv[2]), sys.argv[3:])
