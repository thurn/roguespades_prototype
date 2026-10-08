"""Phase A batch: diagnostics and validity tests on dev seeds at tier 1 (see docs/iteration/phase-a.md)."""

import json
import sys

from . import diag, it
from .common import REPORTS


def save(name, data):
    d = REPORTS / "search"
    d.mkdir(parents=True, exist_ok=True)
    (d / f"{name}.json").write_text(json.dumps(data, indent=1, default=float) + "\n")


def cmp(a, b, env, sizes=None):
    ma = it.measure(a, env, sizes)
    mb = it.measure(b, env, sizes)
    p = it.paired(ma, mb)
    print(it.fmt_measure(b, p["b"]), "\n  " + it.fmt_paired(p), flush=True)
    save(f"cmp-{a}-{b}-{env.key()}", {"a": a, "b": b, "result": p})
    return p


def main(what):
    env = it.Env(1, "wp", "dev", 0)
    if "aifix" in what:
        cmp("baseline0", "base", env)
    if "hunters" in what:
        h = diag.hunters("base", env, 600)
        for k, (w, ci) in h.items():
            print(f"hunter {k}: {w:.3f} ± {ci:.3f}", flush=True)
        save("hunters-base-" + env.key(), h)
    if "shop" in what:
        s = diag.shop_sensitivity("base", env, 400)
        for k, row in s.items():
            import numpy as np

            print(
                f"bonus {k}: committed win {np.mean([v[0] for v in row.values()]):.3f} online {np.mean([v[1] for v in row.values()]):.3f} | "
                + " ".join(f"{a} {v[0]:.2f}/{v[1]:.2f}" for a, v in row.items()),
                flush=True,
            )
        save("shop-sensitivity-base-" + env.key(), s)
    if "aa" in what:
        for i in (1, 2, 3):
            cmp("base", f"aa{i}", env)
    if "plants" in what:
        cmp("base", "plant-set", env)
        cmp("base", "plant-arch", env)
    if "plain" in what:
        print(diag.field_report("plain", env, 1500), flush=True)
    if "pools" in what:
        for p in ("pool-r1", "pool-r2", "pool-r3"):
            cmp("pool-final", p, env)


if __name__ == "__main__":
    main(sys.argv[1:])
