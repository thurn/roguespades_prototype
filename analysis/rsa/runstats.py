"""Shop and bidding diagnostics from run records: nil rates, contracts, purchases, rerolls, gold.

uv run python -m rsa.runstats <records.jsonl> [...]
"""

import sys

import numpy as np

from . import it


def stats(p) -> str:
    nb = nm = tr = 0
    contracts, gold_end, sigils, cards, rerolls = [], [], [], [], []
    for r in it._lines(p):
        for t in r["teams"]:
            for rd in t["rounds"]:
                tr += 1
                nb += rd["nil_bids"]
                nm += rd["nil_made"]
                if rd["contract"] > 0:
                    contracts.append(rd["contract"])
            gold_end.append(t["gold"][-1] if t["gold"] else 0)
            sigils.append(sum(1 for s in t["sigils"] if not s["grant"]))
            cards.append(sum(1 for c in t["cards"] if not c[2]))
            rerolls.append(t["rerolls"])
    return (
        f"nil bids per team-round {nb / tr:.3f} (made {nm / max(nb, 1):.3f}); mean contract {np.mean(contracts):.2f}; "
        f"per run: sigils bought {np.mean(sigils):.1f}, cards bought {np.mean(cards):.1f}, rerolls {np.mean(rerolls):.2f}, "
        f"gold at the last shop {np.mean(gold_end):.0f}"
    )


if __name__ == "__main__":
    for p in sys.argv[1:]:
        print(p.split("/runs/")[-1], "\n  ", stats(p))
