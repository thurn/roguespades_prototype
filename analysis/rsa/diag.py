"""Phase A diagnostics: bid calibration, bid offsets, hunters, plain Spades, shop sensitivity."""

import sys

import numpy as np

from . import it


def calibration(p) -> dict:
    rows = []
    for r in it._lines(p):
        a, c = r["teams"]
        for ti, t in enumerate((a, c)):
            o = c if ti == 0 else a
            cum = ocum = 0.0
            for rn, rd in enumerate(t["rounds"], 1):
                if rd.get("pm") is not None and rd["contract"] > 0:
                    rows.append((rd["pm"], rd["made"], rn, cum < ocum))
                cum += rd["score"]
                ocum += o["rounds"][rn - 1]["score"]
    R = np.array(rows, dtype=float)
    out = {
        "n": len(R),
        "pred": float(R[:, 0].mean()),
        "made": float(R[:, 1].mean()),
        "buckets": [],
        "rounds": [],
    }
    for lo, hi in [(0, 0.5), (0.5, 0.7), (0.7, 0.85), (0.85, 0.95), (0.95, 1.01)]:
        s = (R[:, 0] >= lo) & (R[:, 0] < hi)
        if s.any():
            out["buckets"].append((lo, hi, int(s.sum()), float(R[s, 0].mean()), float(R[s, 1].mean())))
    for rn in sorted(set(R[:, 2].astype(int))):
        s = R[:, 2] == rn
        b = s & (R[:, 3] == 1)
        a = s & (R[:, 3] == 0)
        out["rounds"].append(
            (
                rn,
                float(R[s, 0].mean()),
                float(R[s, 1].mean()),
                float(R[b, 1].mean()) if b.any() else float("nan"),
                float(R[a, 1].mean()) if a.any() else float("nan"),
            )
        )
    return out


def fmt_cal(c: dict) -> str:
    lines = [f"calibration: n {c['n']} predicted {c['pred']:.3f} made {c['made']:.3f}"]
    lines += [f"  pm [{lo:.2f},{hi:.2f}) n={n} pred {p:.3f} made {m:.3f}" for lo, hi, n, p, m in c["buckets"]]
    lines += [
        f"  round {rn}: pred {p:.3f} made {m:.3f} (behind {b:.3f}, ahead {a:.3f})"
        for rn, p, m, b, a in c["rounds"]
    ]
    return "\n".join(lines)


def field_report(name: str, env: it.Env, n: int, extra: dict | None = None, tag: str = "field") -> str:
    b = it.Variant.load(name).materialize()
    p = it.run_field(b, env, n, extra, tag)
    f = it.field_arrays(p, b.defs)
    gs = it.field_stats(f, np.arange(len(f.board)))
    rm = f.round_mean.mean(0)
    return (
        f"{name} {env.key()} {tag}: margin {gs['medianMarginShare']:.3f} trail5 {gs['trailerAfter5Wins']:.3f} "
        f"early {gs['earlyShare']:.3f} set {gs['setRate']:.3f} ot {gs['lateOvertricks']:.2f}\n"
        f"  set by round {np.round(f.sets_by_round.sum(0) / np.maximum(1, f.contracts_by_round.sum(0)), 3).tolist()}\n"
        f"  mean round score {np.round(rm).tolist()}\n" + fmt_cal(calibration(p))
    )


if __name__ == "__main__":
    name = sys.argv[1]
    tier = int(sys.argv[2]) if len(sys.argv) > 2 else 1
    n = int(sys.argv[3]) if len(sys.argv) > 3 else 600
    print(field_report(name, it.Env(tier, "wp", "dev", 0), n))


def offset_test(name: str, env: it.Env, n: int, offsets=(-1, 1), extra: dict | None = None) -> str:
    """Team A bids `offset` away from its choice against a normal team B, in duplicate."""
    b = it.Variant.load(name).materialize()
    out = []
    for off in offsets:
        x = (extra or {}) | {"bidOffset": [off, 0]}
        tag = f"offset{off:+d}" + "".join(f"-{k}{v}" for k, v in (extra or {}).items())
        p = it.run_field(b, env, n, x, tag)
        by: dict = {}
        sets = made = 0
        for r in it._lines(p):
            by.setdefault(r["board"], []).append(r["margin"])
            for rd in r["teams"][0]["rounds"]:
                if rd["contract"] > 0:
                    made += rd["made"]
                    sets += not rd["made"]
        w = np.array([1.0 if sum(v) > 0 else 0.0 if sum(v) < 0 else 0.5 for v in by.values()])
        se = w.std() / np.sqrt(len(w))
        out.append(
            f"offset {off:+d}: team A board wins {w.mean():.3f} ± {1.645 * se:.3f}, set rate {sets / (sets + made):.3f}"
        )
    return f"{name} {env.key()}\n  " + "\n  ".join(out)


def board_wins(p) -> tuple:
    by: dict = {}
    for r in it._lines(p):
        by.setdefault(r["board"], []).append(r["margin"])
    w = np.array([1.0 if sum(v) > 0 else 0.0 if sum(v) < 0 else 0.5 for v in by.values()])
    return float(w.mean()), float(1.645 * w.std() / np.sqrt(len(w)))


def hunters(name: str, env: it.Env, n: int) -> dict:
    """Fixed exploit policies as team A against the normal field (board win rates)."""
    b = it.Variant.load(name).materialize()
    out = {}
    specs = {
        "bid +1": {"bidOffset": [1, 0]},
        "bid +2": {"bidOffset": [2, 0]},
        "always nil": {"alwaysNil": [True, False]},
        "hoard gold": {"arm": {"type": "hunter", "archetype": "hoard"}},
    }
    for a in it.archetypes_of(b):
        specs[f"rush {a}"] = {"arm": {"type": "hunter", "archetype": a}, "commitBonus": [10.0, 1.0]}
    for k, x in specs.items():
        p = it.run_field(b, env, n, x, "hunter-" + k.replace(" ", "").replace("+", "p"))
        out[k] = board_wins(p)
    return out


def shop_sensitivity(name: str, env: it.Env, n: int, bonuses=(0.0, 0.5, 1.0, 2.0)) -> dict:
    """Committed win rate and online share by archetype as the committed bonus scales."""
    b = it.Variant.load(name).materialize()
    out = {}
    for k in bonuses:
        row = {}
        for a in it.archetypes_of(b):
            p = it.run_commit(b, env, a, n, {"commitBonus": [k, 1.0]})
            ca = it.commit_arrays(p, a, b.defs)
            row[a] = (float(ca.win.mean()), float(ca.online.mean()))
        out[k] = row
    return out
