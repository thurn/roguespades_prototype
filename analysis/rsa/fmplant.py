"""Validity test of the v1 synergy estimator (the factorization machine) on a planted combo.

Runs a tier-0 grant tournament over the kept pool plus the planted sigil, with the planted pair
among the joint-grant pairs, fits the outcome model, and reads the synergy sub-metrics and the
planted pair's predicted gain. Compare with the direct pair arms (`rsa.pairsyn`).
"""

import json
import sys

import numpy as np

from . import analyze as A
from . import arms as R
from . import experiments as E
from . import it


def main(boards: int = 60000):
    b = it.Variant.load("plant-syn").materialize()
    step = "iter-fmplant"
    controls = [f"control-{r}" for r in ("common", "uncommon", "rare", "legendary")]
    measured = {s: 1.0 for s in b.pool + controls}
    pairs = [list(p) for p in b.combos]
    p = E.tournament(
        step,
        "tournament",
        offerable=b.pool + controls,
        measured=measured,
        boards=boards,
        tier=0,
        seed=61,
        model=b.model,
        pairs=pairs,
        pair_prob=0.15,
        dir=str(b.dir),
        rules=str(b.rules),
    )
    scale = json.loads(open(b.model).read())["wFinal"]
    m = A.measure(step, "tournament", [p], b.defs, scale, boot=40)
    lam = float(np.mean(json.loads(open(b.model).read())["lambda"][1:7]))
    syn = R.synergy(m, b.defs, lam)
    idx = {s: i for i, s in enumerate(m.des.ids)}
    out = {
        "same_arch_gain": syn["same_arch_gain"],
        "strong": syn["strong"],
        "strong_cross": syn["strong_cross"],
    }
    for x, y in b.combos:
        i, j = idx[x], idx[y]
        parts = R.gross(m.fit, i, b.defs[x], lam) + R.gross(m.fit, j, b.defs[y], lam)
        dot = float(m.fit.V[i] @ m.fit.V[j])
        out[f"{x}+{y}"] = {"dot": dot, "parts": parts, "gain": dot / parts if parts > 0.01 else None}
    out["V_norm_mean"] = float(np.mean(np.linalg.norm(m.fit.V, axis=1)))
    out["top"] = syn["top"][:5]
    print(json.dumps(out, indent=1, default=float))
    (it.ROOT / "reports" / "search").mkdir(parents=True, exist_ok=True)
    (it.ROOT / "reports" / "search" / "fm-plant.json").write_text(json.dumps(out, indent=1, default=float))


if __name__ == "__main__":
    main(int(sys.argv[1]) if len(sys.argv) > 1 else 60000)
