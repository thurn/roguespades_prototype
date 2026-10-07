"""Readings from a fitted tournament: sigil metrics with intervals, estimates in data files,
amount retunes, global scale, the shop model export, and the dashboard."""

import json
import math
from dataclasses import dataclass, field

import numpy as np

from . import fit as F
from .common import CARD_CLASSES, CONTROL, MODELS, PAR, PRICE, REPORTS, RUNS, category, history, save_sigil
from .records import boards, load

DELTA = {"common": 2.0, "uncommon": 3.0, "rare": 5.0, "legendary": 8.0}
BANDS_PATH = MODELS / "bands.json"
DEFAULT_BANDS = {
    "lift": [0.0, 12.0],
    "decisive": [5.0, None],
    "fire": [10.0, None],
    "fireCommitted": [60.0, None],
    "standalone": [3.0, None],
    "standaloneVsControl": [-2.0, None],
    "invalidation": [None, 15.0],
    "pairWin": [None, 65.0],
    "delta": DELTA,
}


def bands() -> dict:
    if BANDS_PATH.exists():
        return json.loads(BANDS_PATH.read_text())
    return json.loads(json.dumps(DEFAULT_BANDS))


def save_bands(b: dict) -> None:
    BANDS_PATH.parent.mkdir(parents=True, exist_ok=True)
    BANDS_PATH.write_text(json.dumps(b, indent=2) + "\n")


def is_payoff_coherent(d: dict) -> bool:
    return d.get("role") != "enabler" and any(a != "Generic" for a in d.get("archetypes", []))


@dataclass
class Reading:
    sid: str
    n: int = 0
    lift: float = float("nan")
    lift_lo: float = float("nan")
    lift_hi: float = float("nan")
    lift_u: float = float("nan")
    se_u: float = float("nan")
    slope: float = float("nan")
    slope_lo: float = float("nan")
    slope_hi: float = float("nan")
    slope_u: float = float("nan")
    slope_se_u: float = float("nan")
    decisive: float = float("nan")
    decisive_n: int = 0
    fire: float = float("nan")
    fire_n: int = 0
    fire_c: float = float("nan")
    fire_c_n: int = 0
    fire_g0: float = float("nan")
    skill: float = float("nan")
    skill_lo: float = float("nan")
    skill_hi: float = float("nan")
    working: float = float("nan")
    pairs: list = field(default_factory=list)
    extra: dict = field(default_factory=dict)


@dataclass
class Measurement:
    step: str
    name: str
    des: F.Design
    fit: F.Fit
    k: float
    scale: float
    readings: dict
    loaded: object
    calib: dict = field(default_factory=dict)


def prop_ci(x: int, n: int) -> tuple:
    if n == 0:
        return (float("nan"),) * 3
    p = x / n
    z = 1.645
    den = 1 + z * z / n
    c = (p + z * z / (2 * n)) / den
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / den
    return 100 * p, 100 * (c - h), 100 * (c + h)


def lift_samples(ft: F.Fit, i: int, ctrl: int | None, c: float) -> tuple:
    """Point estimate, shrunk bootstrap samples, and raw bootstrap samples of a lift."""

    def val(coef):
        v = coef[i * F.N_TERMS] + c * coef[i * F.N_TERMS + 3]
        if ctrl is not None:
            v -= coef[ctrl * F.N_TERMS]
        return v

    point = float(val(ft.coef))
    samples = np.array([val(b) for b in ft.boot]) if ft.boot is not None else np.array([point])
    raw = np.array([val(b) for b in ft.boot_raw]) if ft.boot_raw is not None else None
    return point, samples, raw


def measure(
    step: str, name: str, paths: list, sigils: dict, scale: float, boot: int = 200, alpha: float | None = None
) -> Measurement:
    loaded = load(paths, sigils, parquet_dir=RUNS / step / f"{name}.parquet")
    bb = boards(loaded.runs)
    des = F.build(bb, scale)
    ft = F.fit(des, sigils, boot=boot, alpha=alpha)
    k = F.win_calibration(des)
    readings = {}
    index = {s: i for i, s in enumerate(des.ids)}
    for sid, i in index.items():
        d = sigils.get(sid, {})
        r = Reading(sid, n=int(des.exposures[i]))
        ctrl = index.get(CONTROL.get(d.get("rarity", ""), ""))
        if sid.startswith("control-"):
            ctrl = None
        c = 2.0 if is_payoff_coherent(d) else 0.0
        point, samples, raw = lift_samples(ft, i, ctrl, c)
        lo, hi = F.interval(samples, point, raw)
        r.lift_u, r.se_u = point, F.sd_eb(samples, raw)
        r.lift, r.lift_lo, r.lift_hi = (float(F.to_points(x, k)) for x in (point, lo, hi))
        sl = ft.coef[i * F.N_TERMS + 1]
        ss = ft.boot[:, i * F.N_TERMS + 1] if ft.boot is not None else np.array([sl])
        sr = ft.boot_raw[:, i * F.N_TERMS + 1] if ft.boot_raw is not None else None
        slo, shi = F.interval(ss, sl, sr)
        r.slope_u, r.slope_se_u = float(sl), F.sd_eb(ss, sr)
        # Win-rate points per doubling of the amount.
        r.slope, r.slope_lo, r.slope_hi = (float(F.to_points(x * math.log(2), k)) for x in (sl, slo, shi))
        cnt = loaded.counters.get(sid)
        if cnt:
            r.decisive_n = cnt.wins_held
            r.decisive = prop_ci(cnt.decisive, cnt.wins_held)[0]
            r.fire_n = cnt.held_rounds
            r.fire = prop_ci(cnt.fired_rounds, cnt.held_rounds)[0]
            r.fire_c_n = cnt.held_rounds_c
            r.fire_c = prop_ci(cnt.fired_rounds_c, cnt.held_rounds_c)[0]
            r.fire_g0 = prop_ci(cnt.fired_rounds_g0, cnt.held_rounds_g0)[0]
        # Strongest pairs.
        if ft.V is not None and np.any(ft.V[i]):
            dots = ft.V @ ft.V[i]
            order = np.argsort(-dots)
            r.pairs = [
                (des.ids[j], float(F.to_points(dots[j], k))) for j in order[:6] if j != i and dots[j] > 0
            ][:5]
        r.working = r.lift
        readings[sid] = r
    return Measurement(step, name, des, ft, k, scale, readings, loaded)


def calibrate(m0: Measurement, m1_paths: list, sigils: dict, step: str) -> dict:
    """Multi-fidelity correction: refit tier 0 and tier 1 on the shared boards, shrink the per-sigil
    differences toward their category means, and add them to the tier-0 estimates."""
    t1 = measure(step, "calib-t1", m1_paths, sigils, m0.scale, boot=60, alpha=m0.fit.alpha)
    shared = set(t1.des.board_ids.tolist())
    sub = {b: rs for b, rs in boards(m0.loaded.runs).items() if b in shared}
    des0 = F.build(sub, m0.scale, measured=m0.des.ids)
    ft0 = F.fit(des0, sigils, boot=60, alpha=m0.fit.alpha)
    k = m0.k
    deltas, ses, cats, sids = [], [], [], []
    l0s, l1s = [], []
    for sid, r1 in t1.readings.items():
        if sid not in m0.readings or r1.n < 20:
            continue
        i0 = des0.ids.index(sid)
        d = sigils.get(sid, {})
        ctrl = des0.ids.index(CONTROL[d["rarity"]]) if CONTROL.get(d.get("rarity")) in des0.ids else None
        if sid.startswith("control-"):
            ctrl = None
        c = 2.0 if is_payoff_coherent(d) else 0.0
        p0, s0, raw0 = lift_samples(ft0, i0, ctrl, c)
        deltas.append(r1.lift_u - p0)
        ses.append(math.sqrt(r1.se_u**2 + F.sd_eb(s0, raw0) ** 2))
        cats.append((d.get("role"), category(d)))
        sids.append(sid)
        l0s.append(p0)
        l1s.append(r1.lift_u)
    deltas, ses = np.array(deltas), np.array(ses)
    out = {"n": len(sids)}
    if len(sids) < 5:
        return out
    # Both tiers play the same boards, deals, and grants, so their errors correlate and this
    # correlation overstates the agreement of the underlying values.
    corr = float(np.corrcoef(l0s, l1s)[0, 1])
    out["corr"] = corr
    groups: dict = {}
    for j, c in enumerate(cats):
        groups.setdefault(c, []).append(j)
    for js in groups.values():
        dm = float(np.average(deltas[js], weights=1 / ses[js] ** 2))
        tau2 = max(0.0, float(np.var(deltas[js])) - float(np.mean(ses[js] ** 2)))
        for j in js:
            shrink = tau2 / (tau2 + ses[j] ** 2) if tau2 > 0 else 0.0
            ds = dm + shrink * (deltas[j] - dm)
            se_s = math.sqrt(shrink) * ses[j] if tau2 > 0 else ses[j] / math.sqrt(len(js))
            r = m0.readings[sids[j]]
            r.skill = float(F.to_points(ds, k))
            r.skill_lo = float(F.to_points(ds - 1.645 * se_s, k))
            r.skill_hi = float(F.to_points(ds + 1.645 * se_s, k))
            r.working = float(F.to_points(r.lift_u + ds, k))
    m0.calib = out
    return out


# ---------------------------------------------------------------------------------------------
# Writing estimates


def _metric(key, label, value, lo=None, hi=None, band=(None, None), n=None):
    def clean(x):
        return None if x is None or (isinstance(x, float) and math.isnan(x)) else round(float(x), 2)

    return {
        "key": key,
        "label": label,
        "value": clean(value),
        "lo": clean(lo),
        "hi": clean(hi),
        "bandLo": band[0],
        "bandHi": band[1],
        "n": n,
    }


def write_estimates(
    m: Measurement, sigils: dict, ids: list, reports: list, extra: dict | None = None
) -> None:
    b = bands()
    for sid in ids:
        r = m.readings.get(sid)
        d = sigils[sid]
        if r is None:
            continue
        payoff = d.get("role") != "enabler"
        metrics = [
            _metric(
                "lift",
                "Choices matter: win-rate lift over control (pts)",
                r.working,
                r.lift_lo + (r.working - r.lift),
                r.lift_hi + (r.working - r.lift),
                tuple(b["lift"]),
                r.n,
            ),
            _metric(
                "liftT0",
                "Tier-0 lift before multi-fidelity correction (pts)",
                r.lift,
                r.lift_lo,
                r.lift_hi,
                n=r.n,
            ),
        ]
        if payoff and category(d) in ("points", "hybrid"):
            v, lo, hi = prop_ci(round(r.decisive * r.decisive_n / 100) if r.decisive_n else 0, r.decisive_n)
            metrics.append(
                _metric(
                    "decisive",
                    "Decisive share of holder's wins (%)",
                    v,
                    lo,
                    hi,
                    tuple(b["decisive"]),
                    r.decisive_n,
                )
            )
        if payoff:
            metrics.append(
                _metric("fire", "Payoff fires (% of held rounds)", r.fire, band=tuple(b["fire"]), n=r.fire_n)
            )
            metrics.append(
                _metric(
                    "fireCommitted",
                    "Fires when committed (% of held rounds)",
                    r.fire_c,
                    band=tuple(b["fireCommitted"]),
                    n=r.fire_c_n,
                )
            )
        metrics.append(_metric("slope", "Amount slope (pts per doubling)", r.slope, r.slope_lo, r.slope_hi))
        if not math.isnan(r.skill):
            metrics.append(
                _metric("skill", "Skill gradient: tier 1 minus tier 0 (pts)", r.skill, r.skill_lo, r.skill_hi)
            )
        for key, val in (extra or {}).get(sid, {}).items():
            metrics.append(val if isinstance(val, dict) else _metric(key, key, val))
        est = d.get("estimates") or {}
        amount_hist = est.get("amountHistory", [])
        tun = d["effect"].get("amount", d["effect"].get("step"))
        if tun is not None and (not amount_hist or amount_hist[-1]["amount"] != tun):
            amount_hist.append({"step": m.step, "amount": tun})
        d["estimates"] = {
            "step": m.step,
            "exposures": r.n,
            "metrics": metrics,
            "amountHistory": amount_hist,
            "amountSlope": {
                "value": round(r.slope, 2),
                "lo": round(r.slope_lo, 2),
                "hi": round(r.slope_hi, 2),
            },
            "skillGradient": None
            if math.isnan(r.skill)
            else {"value": round(r.skill, 2), "lo": round(r.skill_lo, 2), "hi": round(r.skill_hi, 2)},
            "pairs": [{"with": j, "synergy": round(v, 2)} for j, v in r.pairs],
            "raw": {
                "liftU": r.lift_u,
                "seU": r.se_u,
                "slopeU": r.slope_u,
                "slopeSeU": r.slope_se_u,
                "k": m.k,
            },
        }
        reps = d.setdefault("reports", [])
        for rp in reports:
            if rp not in reps:
                reps.append(rp)
        save_sigil(d)


# ---------------------------------------------------------------------------------------------
# Amount tuning


def nice(x: float, cat: str, kind: str) -> float:
    if cat == "xmult" or kind == "xmult":
        if x < 1.25:
            return max(1.05, round(x * 20) / 20)
        if x < 1.5:
            return round(x * 4) / 4
        return round(x * 2) / 2
    if x < 5:
        return max(1, round(x))
    return max(5, 5 * round(x / 5))


def retune(m: Measurement, sigils: dict, ids: list, step: str) -> list:
    """Damped Newton step toward the target lift, for every sigil with a measurable amount slope."""
    b = bands()
    changes = []
    for sid in ids:
        r = m.readings.get(sid)
        d = sigils[sid]
        e = d["effect"]
        if (
            r is None
            or d.get("source") == "control"
            or e["type"] not in ("points", "mult", "xmult", "nilPoints", "grow")
        ):
            continue
        key = "step" if e["type"] == "grow" else "amount"
        a = e[key]
        kind = e.get("kind", e["type"])
        target_u = 2 * b["delta"][d["rarity"]] / 100 / max(m.k, 1e-6)
        cur_u = r.lift_u + (0 if math.isnan(r.skill) else 2 * r.skill / 100 / max(m.k, 1e-6))
        lo = r.slope_u - 1.645 * r.slope_se_u
        if r.slope_u <= 0 or lo <= 0:
            continue
        step_log = 0.5 * (target_u - cur_u) / r.slope_u
        step_log = max(-math.log(2), min(math.log(2), step_log))
        if kind == "xmult":
            new = 1 + (a - 1) * math.exp(step_log)
        else:
            new = a * math.exp(step_log)
        new = nice(new, category(d), kind)
        if new != a:
            e[key] = new
            note = (
                f"{step}: amount {fmt(a)} -> {fmt(new)} by retune; lift {r.working:+.1f} pts against target "
                f"{b['delta'][d['rarity']]:+.0f}, slope {r.slope:+.1f} pts per doubling."
            )
            history(d, step, note)
            changes.append((sid, a, new, note))
            save_sigil(d)
    return changes


def structural_flags(m: Measurement, sigils: dict, ids: list) -> list:
    """Sigils whose amount slope interval includes zero: the amount isn't what limits them."""
    out = []
    for sid in ids:
        r = m.readings.get(sid)
        d = sigils[sid]
        if (
            r is None
            or d.get("source") == "control"
            or d["effect"]["type"] not in ("points", "mult", "xmult", "nilPoints", "grow")
        ):
            continue
        if r.slope_lo <= 0 <= r.slope_hi:
            out.append(sid)
    return out


def fmt(x):
    return f"{x:g}"


# ---------------------------------------------------------------------------------------------
# Game-level readings from clean runs


def game_stats(runs: list) -> dict:
    clean = [r for r in runs if r.clean]
    if not clean:
        return {}
    by_round = np.zeros(9)
    n_round = np.zeros(9)
    margins, trail5, early, total = [], 0, 0.0, 0.0
    trail_n = 0
    sets = made = ot = made_late = 0
    sd_left = {r: [] for r in range(9)}
    for r in clean:
        a, b = r.teams
        cum_a = np.cumsum([0] + a.round_scores)
        cum_b = np.cumsum([0] + b.round_scores)
        for rn, (x, y) in enumerate(zip(a.round_scores, b.round_scores, strict=False), start=1):
            by_round[rn] += x + y
            n_round[rn] += 2
        winner = max(a.final, b.final)
        if winner > 0:
            margins.append(abs(a.final - b.final) / winner)
        m5 = cum_a[5] - cum_b[5]
        if m5 != 0:
            trail_n += 1
            trailer_won = (m5 < 0 and a.final > b.final) or (m5 > 0 and b.final > a.final)
            trail5 += trailer_won
        for t in (a, b):
            early += sum(max(0, x) for x in t.round_scores[:4])
            total += sum(max(0, x) for x in t.round_scores)
            sets += t.sets
            made += t.made
            ot += t.overtricks_late
            made_late += t.made_late
        final_m = cum_a[8] - cum_b[8]
        for left in range(1, 9):
            sd_left[left].append(final_m - (cum_a[8 - left] - cum_b[8 - left]))
    mean_round = (by_round / np.maximum(n_round, 1)).tolist()
    return {
        "runs": len(clean),
        "meanRoundScore": mean_round,
        "par": PAR,
        "medianMarginShare": float(np.median(margins)) if margins else float("nan"),
        "trailerAfter5Wins": trail5 / max(trail_n, 1),
        "earlyShare": early / max(total, 1),
        "setRate": sets / max(sets + made, 1),
        "lateOvertricks": ot / max(made_late, 1),
        "wScale": [50.0] + [0.588 * float(np.std(sd_left[k])) for k in range(1, 9)],
        "finalMarginSd": float(np.std([r.margin for r in clean])),
    }


def global_factor(gs: dict) -> float:
    """Damped global amount factor that moves mean round scores toward par (in log space, by half)."""
    obs = sum(gs["meanRoundScore"][1:9])
    par = sum(PAR[1:9])
    if obs <= 0:
        return 1.0
    f = math.exp(0.5 * math.log(par / obs))
    return f if abs(math.log(f)) > math.log(1.1) else 1.0


def apply_global(f: float, sigils: dict, ids: list, step: str) -> int:
    n = 0
    for sid in ids:
        d = sigils[sid]
        e = d["effect"]
        if e["type"] not in ("points", "mult", "xmult", "nilPoints", "grow"):
            continue
        key = "step" if e["type"] == "grow" else "amount"
        a = e[key]
        kind = e.get("kind", e["type"])
        new = 1 + (a - 1) * f if kind == "xmult" else a * f
        new = nice(new, category(d), kind)
        if new != a:
            e[key] = new
            history(
                d,
                step,
                f"{step}: amount {fmt(a)} -> {fmt(new)} by global scale x{f:.2f} toward the par curve.",
            )
            save_sigil(d)
            n += 1
    return n


# ---------------------------------------------------------------------------------------------
# Shop model export


def export_model(
    step: str, m: Measurement, sigils: dict, gs: dict, prev: dict | None, damp: float = 0.5
) -> dict:
    ft, des = m.fit, m.des
    n = len(des.ids)
    card_off = n * F.N_TERMS
    gold_off = card_off + len(CARD_CLASSES)
    lam_raw = ft.coef[gold_off : gold_off + 8]
    lam_prev = (prev or {}).get("lambda", [0.0006] * 9)
    lam = [0.0]
    for t in range(8):
        v = float(np.clip(lam_raw[t], 0.0002, 0.003))
        lam.append(damp * lam_prev[t + 1] + (1 - damp) * v if prev else v)
    lam_mid = float(np.mean(lam[1:7]))
    sig = {}
    prev_s = (prev or {}).get("sigils", {})
    for i, sid in enumerate(des.ids):
        d = sigils.get(sid)
        if d is None:
            continue
        r = m.readings.get(sid)
        beta = float(ft.coef[i * F.N_TERMS]) + lam_mid * PRICE[d["rarity"]]
        if r is not None and not math.isnan(r.skill):
            beta += 2 * r.skill / 100 / max(m.k, 1e-6)
        entry = {
            "beta": beta,
            "shop": float(ft.coef[i * F.N_TERMS + 2]),
            "coh": float(ft.coef[i * F.N_TERMS + 3]),
            "v": [float(x) for x in ft.V[i]],
            "se": float(r.se_u) if r else 0.05,
        }
        if sid in prev_s and prev:
            p = prev_s[sid]
            for k in ("beta", "shop", "coh"):
                entry[k] = damp * p.get(k, entry[k]) + (1 - damp) * entry[k]
        sig[sid] = entry
    for sid, p in prev_s.items():
        sig.setdefault(sid, p)
    prior: dict = {}
    for sid, e in sig.items():
        d = sigils.get(sid)
        if not d or d.get("source") == "control":
            continue
        prior.setdefault(f"{d['rarity']}/{category(d) if category(d) != 'hybrid' else 'enabler'}", []).append(
            e["beta"]
        )
    prior = {k: float(np.mean(v)) for k, v in prior.items()}
    base_prior = (prev or {}).get("prior", {})
    for k, v in base_prior.items():
        prior.setdefault(k, v)
    cards = dict((prev or {}).get("cards", {}))
    avg_price = {"A": 80, "K": 60, "Q": 45, "J": 35, "10": 25, "low": 15}
    for j, cls in enumerate(CARD_CLASSES):
        s, rk = cls.split("-")
        price = avg_price[rk] * (1.5 if s == "S" else 1.0)
        v = float(ft.coef[card_off + j]) + lam_mid * price
        old = cards.get(cls, v)
        cards[cls] = damp * old + (1 - damp) * v if prev else v
    model = {
        "step": step,
        "lambda": lam,
        "sigils": sig,
        "prior": prior,
        "cards": cards,
        "cardShop": (prev or {}).get("cardShop", 0.0),
        "wScale": gs.get("wScale", (prev or {}).get("wScale")),
        "wFinal": m.scale,
        "nilHandicap": (prev or {}).get("nilHandicap", 550.0),
        "roundScale": [0.0] + [max(300.0, x) for x in gs.get("meanRoundScore", PAR)[1:9]],
    }
    MODELS.mkdir(parents=True, exist_ok=True)
    (MODELS / f"{step}.json").write_text(json.dumps(model, indent=1) + "\n")
    return model


# ---------------------------------------------------------------------------------------------
# Dashboard


def _f(x, d=1):
    return "" if x is None or (isinstance(x, float) and math.isnan(x)) else f"{x:.{d}f}"


def flags(r: Reading, d: dict, b: dict) -> list:
    out = []
    lo, hi = b["lift"]
    if r.lift_hi < lo:
        out.append("lift<band")
    elif hi is not None and r.lift_lo > hi:
        out.append("lift>ceiling")
    if d.get("role") != "enabler":
        if not math.isnan(r.fire) and r.fire < b["fire"][0]:
            out.append("rarely fires")
        if (
            category(d) == "points"
            and not math.isnan(r.decisive)
            and r.decisive_n >= 30
            and r.decisive < b["decisive"][0]
        ):
            out.append("rarely decisive")
    if (
        not math.isnan(r.slope)
        and r.slope_lo <= 0 <= r.slope_hi
        and d["effect"]["type"] in ("points", "mult", "xmult", "nilPoints", "grow")
    ):
        out.append("flat slope")
    return out


def dashboard(
    m: Measurement, sigils: dict, ids: list, title: str, extra_md: str = "", fun: dict | None = None
) -> str:
    b = bands()
    lines = [f"# {title}", ""]
    lines.append(
        f"Fit: {len(m.des.u)} boards, alpha {m.fit.alpha:g}, residual sd {m.fit.resid_sd:.3f} (u units), "
        f"win calibration k = {m.k:.2f}, smoothing scale {m.scale:.0f} points."
    )
    if m.calib:
        lines.append(
            f"Tier agreement: {m.calib.get('n')} sigils, lift correlation {m.calib.get('corr', float('nan')):.2f} "
            "on shared boards (overstated by shared deals and grants)."
        )
    lines.append(
        f"Ledger rescoring check: {m.loaded.rescore_mismatch} mismatches in {m.loaded.rounds_checked} rounds."
    )
    lines.append("")
    if fun:
        lines.append(fun_md(fun))
    if extra_md:
        lines.append(extra_md)
    lines.append(
        "| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |"
    )
    lines.append("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
    rar = {"common": 0, "uncommon": 1, "rare": 2, "legendary": 3}

    def key(s):
        return (rar.get(sigils[s]["rarity"], 9), -(m.readings[s].working if s in m.readings else -99))

    for sid in sorted([s for s in ids if s in m.readings], key=key):
        r = m.readings[sid]
        d = sigils[sid]
        lines.append(
            f"| {sid} | {d['rarity'][0].upper()} | {category(d)} | {d.get('simplicity', {}).get('C', '')} | {r.n} | "
            f"{_f(r.working)} [{_f(r.lift_lo + r.working - r.lift)}, {_f(r.lift_hi + r.working - r.lift)}] | {_f(r.lift)} | "
            f"{_f(r.skill)} | {_f(r.decisive)} | {_f(r.fire)} | {_f(r.fire_c)} | {_f(r.slope)} | "
            f"{', '.join(flags(r, d, b))} | {d.get('text', '').replace('|', '/')} |"
        )
    return "\n".join(lines) + "\n"


def fun_md(fun: dict) -> str:
    lines = [
        f"**Fun score: {fun['score']:.1f}** (90% interval {fun['lo']:.1f}–{fun['hi']:.1f})",
        "",
        "| Family | Weight | Score | Sub-metrics |",
        "| --- | --- | --- | --- |",
    ]
    for fam in fun["families"]:
        subs = "; ".join(f"{s['name']} {s['value']}" for s in fam["subs"])
        lines.append(f"| {fam['name']} | {fam['weight']} | {fam['score']:.2f} | {subs} |")
    return "\n".join(lines) + "\n"


def write_report(name: str, md: str, data: dict | None = None) -> str:
    REPORTS.mkdir(parents=True, exist_ok=True)
    (REPORTS / f"{name}.md").write_text(md)
    if data is not None:
        (REPORTS / f"{name}.json").write_text(json.dumps(data, indent=1, default=float) + "\n")
    return f"reports/{name}.md"
