"""Phase 5 of design search 2: the recommendation, runners-up, the base game, and search 1's
recommendation on final seeds, every family with a paired 90% interval against the base game.

    uv run python -m rsa.phase5 <tier> <rep> <base> <candidate> [...]

Field, commitment, and synergy pair-arm runs (tier 0) use the replicate's boards, so synergy
noise averages over replicates; synergy is shared between tiers 1 and 2 of a replicate. The
ladder (tier 2 against tier 0) is read once, on replicate 0, and shared by every replicate. Plain Spades (`plain-n`) is measured for
reference.
"""

import json
import sys

from . import it, itcli, pairsyn
from .common import REPORTS

SIZES = {1: (1500, 375), 2: (500, 125)}
LADDER = 375
SYN = 400


def measure(name: str, env: it.Env, syn) -> it.Measured:
    field, commit = SIZES[env.tier]
    b = it.Variant.load(name).materialize()
    rules = json.loads(b.rules.read_text())
    fa = it.field_arrays(it.run_field(b, env, field), b.defs, b.pool, rules["sigilOffers"])
    commits = {a: it.commit_arrays(it.run_commit(b, env, a, commit), a, b.defs) for a in it.archetypes_of(b)}
    e0 = it.Env(env.tier, env.util, env.seeds, 0)
    lad = it.ladder_arrays(it.run_ladder(b, e0, LADDER))
    s = syn(b, it.Env(0, env.util, env.seeds, env.rep)) if (syn and commits) else None
    pool = b.pool if b.exp.get("sigilShop", True) else []
    return it.Measured(name, env, fa, commits, lad, it.simplicity_of(b), s, pool).prepare()


def main(tier: int, rep: int, names: list) -> None:
    env = it.Env(tier, "wp", "final", rep)
    syn = pairsyn.estimator(n=SYN, tier=0)
    ms = {n: measure(n, env, syn) for n in names}
    out = {"env": env.key(), "measures": {}, "paired": {}}
    for n, m in ms.items():
        r = it.interval(m)
        out["measures"][n] = r
        print(it.fmt_measure(n, r), flush=True)
        itcli.log_use(env, f"phase 5 measure {n}", f"{r['score']:.1f} [{r['lo']:.1f}, {r['hi']:.1f}]")
    plain = measure("plain-n", env, None)
    r = it.interval(plain)
    out["measures"]["plain"] = r
    print(it.fmt_measure("plain", r), flush=True)
    base = ms[names[0]]
    for n in names[1:]:
        p = it.paired(base, ms[n])
        out["paired"][n] = p
        print(f"  {n} vs {names[0]}: " + it.fmt_paired(p), flush=True)
        itcli.log_use(
            env, f"phase 5 compare {names[0]} vs {n}", f"{p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}]"
        )
    d = REPORTS / "search-2"
    d.mkdir(parents=True, exist_ok=True)
    (d / f"phase5-{env.key()}.json").write_text(json.dumps(out, indent=1, default=float) + "\n")


if __name__ == "__main__":
    main(int(sys.argv[1]), int(sys.argv[2]), sys.argv[3:])
