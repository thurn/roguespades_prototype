"""Phase 1 of the design search: does the AI play the new rules?

`uv run python -m rsa.phase1 calib|offsets|bags|aa|plants|baseline [tier] [n]`
"""

import sys

import numpy as np

from . import diag, it, pairsyn


def bag_profile(p, team: int | None = None) -> dict:
    """Overtricks per made contract by bags carried into the round, and penalties per team-run."""
    buckets = {"0-4": [], "5-7": [], "8-9": []}
    pens = []
    for r in it._lines(p):
        for ti, t in enumerate(r["teams"]):
            if team is not None and ti != team:
                continue
            carried = 0
            n = 0
            for rd in t["rounds"]:
                if rd["contract"] > 0 and rd["made"]:
                    k = "0-4" if carried <= 4 else "5-7" if carried <= 7 else "8-9"
                    buckets[k].append(rd["tricks"] - rd["contract"])
                carried = (carried + rd.get("bags", 0)) % 10
                n += rd.get("bag_pen", 0) < 0
            pens.append(n)
    return {
        "overtricks": {k: (float(np.mean(v)) if v else float("nan"), len(v)) for k, v in buckets.items()},
        "penalties": float(np.mean(pens)),
    }


def fmt_bags(b: dict) -> str:
    o = " ".join(f"{k}: {m:.2f} (n {n})" for k, (m, n) in b["overtricks"].items())
    return f"overtricks per made contract by bags carried: {o}; penalties per team-run {b['penalties']:.2f}"


def calib(tier: int, n: int) -> None:
    env = it.Env(tier, "wp", "dev", 0)
    print(diag.field_report("base", env, n), flush=True)
    b = it.Variant.load("base").materialize()
    print(fmt_bags(bag_profile(it.run_field(b, env, n))), flush=True)


def offsets(tier: int, n: int) -> None:
    print(diag.offset_test("base", it.Env(tier, "wp", "dev", 0), n), flush=True)


def bags(tier: int, n: int) -> None:
    """Team A searches bag-blind against a normal team B, in duplicate."""
    env = it.Env(tier, "wp", "dev", 0)
    b = it.Variant.load("base").materialize()
    p = it.run_field(b, env, n, {"bagBlind": [True, False]}, "bagblind")
    print("blind team A:", fmt_bags(bag_profile(p, 0)), flush=True)
    print("aware team B:", fmt_bags(bag_profile(p, 1)), flush=True)
    w, ci = diag.board_wins(p)
    print(f"blind team A board wins {w:.3f} ± {ci:.3f}", flush=True)


def cmp(a: str, bname: str, env, sizes=None, syn=None) -> None:
    ma = it.measure(a, env, sizes, syn)
    mb = it.measure(bname, env, sizes, syn)
    p = it.paired(ma, mb)
    print(it.fmt_measure(a, p["a"]), flush=True)
    print(it.fmt_measure(bname, p["b"]), "\n  " + it.fmt_paired(p), flush=True)


def aa(tier: int, n: int) -> None:
    env = it.Env(tier, "wp", "dev", 0)
    cmp("base", "aa1", env, it.Sizes(field=n, commit=n // 4, ladder=n // 4))


def plants(tier: int, n: int) -> None:
    env = it.Env(tier, "wp", "dev", 0)
    cmp("base", "plant-arch", env, it.Sizes(field=n, commit=n // 4, ladder=n // 4))
    syn = pairsyn.estimator(n=300, tier=0, n_same=20, n_cross=6)
    cmp("base", "plant-syn", env, it.Sizes(field=200, commit=50, ladder=50, ladder_on=False), syn)


def baseline(tier: int, n: int) -> None:
    env = it.Env(tier, "wp", "holdout", 0)
    for v in ("base", "plain"):
        m = it.measure(v, env, it.Sizes(field=n, commit=n // 4, ladder=n // 4))
        r = it.interval(m)
        print(it.fmt_measure(v, r), flush=True)


if __name__ == "__main__":
    cmd = sys.argv[1]
    tier = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    n = int(sys.argv[3]) if len(sys.argv) > 3 else 1000
    globals()[cmd](tier, n)
