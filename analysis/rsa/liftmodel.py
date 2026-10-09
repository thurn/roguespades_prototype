"""A shop model refitted from single-grant lifts under the current rules and pool.

Each pool sigil's value is twice its board win-rate lift when granted free at shop 3 (smoothed
outcome units, the scale of the fitted 1.0 values), shrunk toward its rarity-and-category mean by
empirical Bayes. Shop slopes and coherence are kept from the source model where it knows the sigil;
pair vectors are dropped. Everything else (gold, cards, the margin curve) is the source model's.

    uv run python -m rsa.liftmodel <variant> [--n 1000]

Writes data/search/models/<variant>.json and the variant <variant>-L that uses it.
"""

import argparse
import json

import numpy as np

from . import it, sigilmetrics
from .common import ROOT, category

MODELS = ROOT / "data" / "search" / "models"


def changed(name: str, vs: str) -> list:
    """Pool sigils of `name` that are new or differ (effect or rarity) from variant `vs`."""
    a = it.Variant.load(name).materialize()
    b = it.Variant.load(vs).materialize()
    key = lambda d: json.dumps([d["effect"], d["rarity"]], sort_keys=True)  # noqa: E731
    return [s for s in a.pool if s not in b.defs or key(a.defs[s]) != key(b.defs[s])]


def build(name: str, n: int, seeds: str = "dev", vs: str | None = None, src_model: str | None = None) -> str:
    """With `vs` and `src_model`, measures only sigils changed from `vs` and keeps the rest of
    `src_model`'s values."""
    env = it.Env(0, "wp", seeds, 0)
    b = it.Variant.load(name).materialize()
    ids = changed(name, vs) if vs else None
    rows = sigilmetrics.measure(name, env, n, 0, 3, ids) if ids is None or ids else []
    src = json.loads(open(src_model or b.model).read())
    u = {r["id"]: 2 * r["raw"] / 100 for r in rows}
    se = {r["id"]: 2 * r["lift_ci"] / 1.645 / 100 for r in rows}
    if vs:
        print(f"re-measured {len(rows)} changed sigils: {', '.join(r['id'] for r in rows)}")
        for sid in b.pool:
            if sid not in u and sid in src["sigils"]:
                u[sid] = src["sigils"][sid]["beta"]
                se[sid] = 0.0

    def group(sid):
        d = b.defs[sid]
        c = category(d)
        return f"{d['rarity']}/{'enabler' if c == 'hybrid' else c}"

    groups: dict = {}
    for sid in u:
        groups.setdefault(group(sid), []).append(sid)
    grand = float(np.mean(list(u.values())))
    resid = [u[s] - np.mean([u[x] for x in groups[group(s)]]) for s in u]
    tau2 = max(1e-4, float(np.var(resid)) - float(np.mean([v * v for v in se.values()])))
    if vs:
        tau2 = 0.05**2  # the full-pool fit's spread
    sig = {}
    for sid in u:
        if se[sid] == 0.0:
            sig[sid] = dict(src["sigils"][sid])
            continue
        g = groups[group(sid)]
        # A group mean, itself shrunk toward the grand mean when the group is small.
        gm = (np.mean([u[x] for x in g]) * len(g) + grand * 2) / (len(g) + 2)
        w = tau2 / (tau2 + se[sid] ** 2)
        old = src["sigils"].get(sid, {})
        sig[sid] = {
            "beta": float(gm + w * (u[sid] - gm)),
            "shop": old.get("shop", -0.002),
            "coh": old.get("coh", 0.0),
            "v": [],
            "se": se[sid],
        }
    prior = {g: float(np.mean([sig[s]["beta"] for s in ids])) for g, ids in groups.items()}
    m = dict(src)
    m["step"] = f"lift-{name}"
    m["sigils"] = sig
    m["prior"] = {**src["prior"], **prior}
    MODELS.mkdir(parents=True, exist_ok=True)
    out = MODELS / f"{name}.json"
    out.write_text(json.dumps(m, indent=1) + "\n")
    rel = str(out.relative_to(ROOT))
    (it.VARIANTS / f"{name}-L.json").write_text(json.dumps({"base": name, "model": rel}, indent=1) + "\n")
    print(
        f"tau {np.sqrt(tau2):.3f}, mean se {np.mean(list(se.values())):.3f}; wrote {rel} and variant {name}-L"
    )
    return rel


def main() -> None:
    ap = argparse.ArgumentParser(prog="liftmodel")
    ap.add_argument("variant")
    ap.add_argument("--n", type=int, default=1000)
    ap.add_argument("--vs", default=None, help="re-measure only sigils changed from this variant")
    ap.add_argument("--src", default=None, help="the model to keep values from (with --vs)")
    a = ap.parse_args()
    build(a.variant, a.n, vs=a.vs, src_model=a.src)


if __name__ == "__main__":
    main()
