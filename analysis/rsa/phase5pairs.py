"""Phase 5 paired differences between the recommendation and every other design, per replicate,
from the cached final-seed runs (run `rsa.phase5` first).

    uv run python -m rsa.phase5pairs <tier> <rec> <other> [...]
"""

import json
import sys

from . import it, pairsyn, phase5
from .common import REPORTS


def main(tier: int, rec: str, others: list) -> None:
    syn = pairsyn.estimator(n=phase5.SYN, tier=0)
    out = {}
    for rep in range(3):
        env = it.Env(tier, "wp", "final", rep)
        a = phase5.measure(rec, env, syn)
        for o in others:
            p = it.paired(a, phase5.measure(o, env, syn))
            out.setdefault(o, []).append(p)
            print(f"t{tier} rep {rep} {o} vs {rec}: " + it.fmt_paired(p), flush=True)
    (REPORTS / "search-2" / f"phase5-pairs-t{tier}.json").write_text(
        json.dumps(out, indent=1, default=float) + "\n"
    )


if __name__ == "__main__":
    main(int(sys.argv[1]), sys.argv[2], sys.argv[3:])
