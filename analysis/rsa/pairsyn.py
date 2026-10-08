"""Synergy by direct pair arms: the estimator replacement for the factorization machine.

Team A is granted a fixed set of sigils for free at shop 1 (nothing, one sigil, or a pair) and then
shops as usual against a flexible team B. Every arm plays the same boards, so on each board the
pair's interaction is u(ab) - u(a) - u(b) + u(none) and its parts are (u(a) - u(none)) +
(u(b) - u(none)), in smoothed-outcome units u = 2 w(margin / W) - 1.

- Same-archetype gain: the pooled ratio of interaction to parts over a fixed random sample of
  same-archetype pairs.
- Strong pairs: tested pairs whose interaction is at least half their parts and positive with 90%
  one-sided confidence (parts above 0.01). Untested pairs never count, so the counts are lower
  bounds.
"""

import hashlib
import json
from dataclasses import dataclass

import numpy as np

from . import it


def tags(d: dict) -> set:
    return {a for a in d.get("archetypes", []) if a != "Generic"}


def _u(key: str) -> float:
    return int(hashlib.sha1(key.encode()).hexdigest()[:8], 16) / 0xFFFFFFFF


def choose_pairs(b: "it.Built", n_same: int = 60, n_cross: int = 20, extra: list | None = None) -> list:
    """(a, b, same, source) for hash-sampled same- and cross-archetype pairs, plus `extra`.

    Each pair is included independently by a hash of its ids, at a rate set so the incumbent pool
    (117 sigils) yields about `n_same` and `n_cross` pairs. Pairs shared by two pools are sampled
    identically, so variants that add or remove sigils compare on the same pairs."""
    pool = sorted(b.pool)
    out = []
    for i in range(len(pool)):
        for j in range(i + 1, len(pool)):
            x, y = pool[i], pool[j]
            same = bool(tags(b.defs[x]) & tags(b.defs[y]))
            rate = n_same / 2100 if same else n_cross / 4700
            if _u(f"{x}+{y}") < rate:
                out.append((x, y, same, "sample"))
    seen = {(x, y) for x, y, *_ in out}
    for x, y in extra or []:
        x, y = sorted((x, y))
        if (x, y) not in seen and x in pool and y in pool:
            out.append((x, y, bool(tags(b.defs[x]) & tags(b.defs[y])), "extra"))
            seen.add((x, y))
    return out


def run_arm(b: "it.Built", env: "it.Env", build: list, n: int, tier: int, shop: int = 3) -> "it.Path":
    cfg = it.base_cfg(b, env) | {
        "seed": env.seed(it.COMPONENT["pairs"]),
        "boards": n,
        "tier": tier,
        "cleanShare": 0.0,
        "arm": {"type": "grant", "build": sorted(build), "shop": shop},
    }
    tag = f"s{shop}-" + ("none" if not build else "+".join(sorted(build)))
    if len(tag) > 120:
        tag = hashlib.sha1(tag.encode()).hexdigest()[:16]
    e = it.Env(tier, env.util, env.seeds, env.rep)
    return it._run(cfg, it.OUT / b.name / b.digest / e.key() / "pairs" / f"{tag}.jsonl")


def board_u(p, W: float) -> tuple:
    """Board keys, mean smoothed outcome u, and mean final margin in points."""
    by: dict = {}
    for r in it._lines(p):
        by.setdefault(r["board"], []).append(r["margin"])
    keys = sorted(by)
    u = np.array([np.mean([2 / (1 + np.exp(-np.clip(m / W, -50, 50))) - 1 for m in by[k]]) for k in keys])
    m = np.array([np.mean(by[k]) for k in keys])
    return np.array(keys), u, m


@dataclass
class PairSyn:
    pairs: list
    I: np.ndarray  # pairs x boards
    P: np.ndarray  # pairs x boards
    singles: dict  # sid -> lift (u) over none, mean over boards
    IM: np.ndarray = None  # interaction in margin points, pairs x boards
    PM: np.ndarray = None  # parts in margin points
    reg: np.ndarray = None  # registered in the variant spec (combos) before the read
    confirmed: set | None = None  # registered pairs strong on the replicate seed set(s)
    W: float = 1.0  # the model's final-margin scale (parts below 1% of W are too small to judge)

    def resample(self, rng):
        n = self.I.shape[1]
        return rng.integers(0, n, n)

    def value(self, pick=None) -> dict:
        I = self.I if pick is None else self.I[:, pick]
        P = self.P if pick is None else self.P[:, pick]
        n = I.shape[1]
        Im, Pm = I.mean(1), P.mean(1)
        Ise = I.std(1) / np.sqrt(n)
        same = np.array([p[2] for p in self.pairs])
        sample = np.array([p[3] == "sample" for p in self.pairs])
        sel = same & sample & (Pm > 0)
        gain = float(Im[sel].sum() / Pm[sel].sum()) if sel.any() and Pm[sel].sum() > 0 else float("nan")
        strong = (Pm > 0.01) & (Im > 1.645 * Ise) & (Im >= 0.5 * Pm)
        IM = self.IM if pick is None else self.IM[:, pick]
        PM = self.PM if pick is None else self.PM[:, pick]
        IMm, PMm = IM.mean(1), PM.mean(1)
        selm = same & sample & (PMm > 0)
        gain_pts = float(IMm[selm].sum() / PMm[selm].sum()) if selm.any() else float("nan")
        IMse = IM.std(1) / np.sqrt(n)
        strong_pts = (PMm > 0.01 * self.W) & (IMm > 1.645 * IMse) & (IMm >= 0.5 * PMm)
        reg = strong & self.reg
        if self.confirmed is not None:
            ok = np.array([(a, b) in self.confirmed for a, b, *_ in self.pairs])
            reg = reg & ok
        return {
            "same_arch_gain": gain,
            "strong": int(strong.sum()),
            "strong_cross": int((strong & ~same).sum()),
            "same_arch_gain_pts": gain_pts,
            "strong_pts": int(strong_pts.sum()),
            "strong_pts_cross": int((strong_pts & ~same).sum()),
            "strong_reg": int(reg.sum()),
            "strong_reg_cross": int((reg & ~same).sum()),
        }

    def strong_set(self) -> set:
        v = self.I.mean(1), self.P.mean(1), self.I.std(1) / np.sqrt(self.I.shape[1])
        Im, Pm, Ise = v
        strong = (Pm > 0.01) & (Im > 1.645 * Ise) & (Im >= 0.5 * Pm) & self.reg
        return {(a, b) for (a, b, *_), s in zip(self.pairs, strong, strict=True) if s}

    def table(self) -> list:
        n = self.I.shape[1]
        rows = []
        for k, (a, b, same, src) in enumerate(self.pairs):
            rows.append(
                {
                    "a": a,
                    "b": b,
                    "same": same,
                    "src": src,
                    "I": float(self.I[k].mean()),
                    "I_se": float(self.I[k].std() / np.sqrt(n)),
                    "parts": float(self.P[k].mean()),
                    "gain": float(self.I[k].mean() / self.P[k].mean()) if self.P[k].mean() > 0.01 else None,
                    "IM": float(self.IM[k].mean()),
                    "PM": float(self.PM[k].mean()),
                    "reg": bool(self.reg[k]),
                }
            )
        return sorted(rows, key=lambda r: -r["I"])


def measure(
    b: "it.Built",
    env: "it.Env",
    n: int = 300,
    tier: int = 0,
    n_same: int = 60,
    n_cross: int = 20,
    extra: list | None = None,
    shop: int = 3,
    only_registered: bool = False,
) -> PairSyn:
    pairs = choose_pairs(b, n_same, n_cross, extra)
    if only_registered:
        pairs = [p for p in pairs if tuple(sorted(p[:2])) in _regset(extra)]
    W = json.loads(open(b.model).read())["wFinal"]
    keys0, u0, m0 = board_u(run_arm(b, env, [], n, tier, shop), W)
    singles, msingles = {}, {}
    for s in sorted({x for p in pairs for x in p[:2]}):
        k, u, m = board_u(run_arm(b, env, [s], n, tier, shop), W)
        assert np.array_equal(k, keys0)
        singles[s] = u
        msingles[s] = m
    I, P, IM, PM = [], [], [], []
    for a, c, *_ in pairs:
        k, u, m = board_u(run_arm(b, env, [a, c], n, tier, shop), W)
        assert np.array_equal(k, keys0)
        I.append(u - singles[a] - singles[c] + u0)
        P.append(singles[a] + singles[c] - 2 * u0)
        IM.append(m - msingles[a] - msingles[c] + m0)
        PM.append(msingles[a] + msingles[c] - 2 * m0)
    reg = np.array([tuple(sorted(p[:2])) in _regset(extra) for p in pairs], dtype=bool)
    shape = (0, len(keys0))
    return PairSyn(
        pairs,
        np.array(I).reshape(-1, len(keys0)) if I else np.zeros(shape),
        np.array(P).reshape(-1, len(keys0)) if P else np.zeros(shape),
        {s: float((u - u0).mean()) for s, u in singles.items()},
        np.array(IM).reshape(-1, len(keys0)) if IM else np.zeros(shape),
        np.array(PM).reshape(-1, len(keys0)) if PM else np.zeros(shape),
        reg,
        W=W,
    )


def _regset(extra) -> set:
    return {tuple(sorted(p)) for p in extra or []}


def estimator(n: int = 300, tier: int = 0, n_same: int = 60, n_cross: int = 20, extra=None, replicate=()):
    """A `synergy` hook for `it.measure`. With `replicate` seed sets (for example ("dev",) for a
    holdout read), a registered pair counts as strong only if it is also strong on each of them
    (read on the registered pairs only, with the same board count)."""

    def f(b, env):
        reg = (extra or []) + b.combos
        ps = measure(b, env, n, tier, n_same, n_cross, reg)
        if replicate:
            conf = None
            for seeds in replicate:
                e2 = it.Env(env.tier, env.util, seeds, env.rep)
                s2 = measure(b, e2, n, tier, n_same, n_cross, reg, only_registered=True).strong_set()
                conf = s2 if conf is None else conf & s2
            ps.confirmed = conf
        return ps

    return f
