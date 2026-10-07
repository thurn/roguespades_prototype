"""Phase 0d pilot: pipeline validation on the seed pool, and the budgets it sets."""

import json
import math
import shutil

import numpy as np

from . import analyze as A
from . import experiments as E
from . import fit as F
from .common import RUNS, load_sigils
from .records import boards, load

STEP = "phase-0"
DOSES = [10, 20, 40, 80, 160]


def _pilot_dir(sigils):
    d = RUNS / STEP / "sigils"
    if d.exists():
        shutil.rmtree(d)
    d.mkdir(parents=True)
    for sid, s in sigils.items():
        (d / f"{sid}.json").write_text(json.dumps(s, ensure_ascii=False))
    extra = {}
    for a in DOSES:
        extra[f"pilot-dose-{a}"] = {
            "id": f"pilot-dose-{a}",
            "rarity": "common",
            "role": "payoff",
            "archetypes": ["Generic"],
            "source": "control",
            "effect": {"type": "points", "amount": a},
            "status": "cut",
        }
    twin = dict(sigils["seed-ace-win"])
    twin["id"] = "aa-ace-win"
    extra["aa-ace-win"] = twin
    for sid, s in extra.items():
        (d / f"{sid}.json").write_text(json.dumps(s, ensure_ascii=False))
    all_s = dict(sigils)
    all_s.update(extra)
    return d, all_s


def spearman(a, b):
    ra = np.argsort(np.argsort(a))
    rb = np.argsort(np.argsort(b))
    if np.std(ra) == 0 or np.std(rb) == 0:
        return float("nan")
    return float(np.corrcoef(ra, rb)[0, 1])


def run(boards_t0: int = 8000, calib_share: float = 0.2):
    sigils = load_sigils()
    pdir, allsig = _pilot_dir(sigils)
    seeds = [s for s in sigils if s.startswith("seed-")]
    controls = [s for s in sigils if s.startswith("control-") and "blank" not in s]
    measured = {s: 1.0 for s in seeds + controls + [f"pilot-dose-{a}" for a in DOSES] + ["aa-ace-win"]}
    offer = seeds + controls
    rel = str(pdir.relative_to(RUNS.parent))
    common = dict(offerable=offer, measured=measured, seed=101, validate=0.02, dir="../" + rel)
    p0 = E.tournament(STEP, "pilot-t0", boards=boards_t0, tier=0, **common)
    nb1 = int(boards_t0 * calib_share)
    p1 = E.tournament(STEP, "pilot-t1", boards=nb1, tier=1, **common)
    loaded = load([p0], allsig)
    clean = [r for r in loaded.runs if r.clean]
    sd = float(np.std([r.margin for r in clean]))
    bb = boards(loaded.runs)

    # Smoothing scale: maximize the mean |t| of the planted dose effects.
    dose_ids = [f"pilot-dose-{a}" for a in DOSES]
    scale_rows = []
    best = None
    for mult in (0.25, 0.5, 1.0, 2.0):
        des = F.build(bb, mult * sd, measured=sorted(measured))
        ft = F.fit(des, allsig, boot=60)
        ts = []
        for s in dose_ids:
            i = des.ids.index(s)
            ts.append(
                abs(ft.coef[i * F.N_TERMS])
                / max(F.sd_eb(ft.boot[:, i * F.N_TERMS], ft.boot_raw[:, i * F.N_TERMS]), 1e-9)
            )
        t = float(np.mean(ts))
        scale_rows.append((mult, mult * sd, t, ft.resid_sd))
        if best is None or t > best[0]:
            best = (t, mult * sd)
    scale = best[1]

    m = A.measure(STEP, "pilot-t0", [p0], allsig, scale)
    A.calibrate(m, [p1], allsig, STEP)
    k = m.k

    # 1. Planted dose-response.
    dose = [(a, m.readings[f"pilot-dose-{a}"]) for a in DOSES]
    dose_beta = [
        (a, float(F.to_points(m.fit.coef[m.des.ids.index(f"pilot-dose-{a}") * F.N_TERMS], k))) for a in DOSES
    ]
    monotone = all(dose_beta[j][1] < dose_beta[j + 1][1] for j in range(len(dose_beta) - 1))

    # 2. Synthetic recovery.
    rng = np.random.default_rng(5)
    truth = m.fit.coef.copy()
    syn = F.Design(**{**m.des.__dict__})
    syn.u = m.des.X @ truth + m.fit.fm_pred + rng.normal(0, m.fit.resid_sd, len(m.des.u))
    sft = F.fit(syn, allsig, boot=100, alpha=m.fit.alpha)
    cover = []
    for i, _s in enumerate(m.des.ids):
        if m.des.exposures[i] < 50:
            continue
        lo, hi = F.interval(
            sft.boot[:, i * F.N_TERMS], sft.coef[i * F.N_TERMS], sft.boot_raw[:, i * F.N_TERMS]
        )
        cover.append(lo <= truth[i * F.N_TERMS] <= hi)
    coverage = float(np.mean(cover))

    # 3. A/A test.
    a1, a2 = m.readings["seed-ace-win"], m.readings["aa-ace-win"]
    i1, i2 = m.des.ids.index("seed-ace-win"), m.des.ids.index("aa-ace-win")
    diff_s = m.fit.boot[:, i1 * F.N_TERMS] - m.fit.boot[:, i2 * F.N_TERMS]
    dpt = float(F.to_points(m.fit.coef[i1 * F.N_TERMS] - m.fit.coef[i2 * F.N_TERMS], k))
    diff_r = m.fit.boot_raw[:, i1 * F.N_TERMS] - m.fit.boot_raw[:, i2 * F.N_TERMS]
    dlo, dhi = (
        float(F.to_points(x, k))
        for x in F.interval(diff_s, m.fit.coef[i1 * F.N_TERMS] - m.fit.coef[i2 * F.N_TERMS], diff_r)
    )
    aa_ok = dlo <= 0 <= dhi

    # 4. Variance reduction: SE of the planted effects under three outcome designs.
    def mean_se(des):
        ft = F.fit(des, allsig, boot=60, alpha=m.fit.alpha)
        k2 = F.win_calibration(des)
        return float(
            np.mean(
                [
                    F.to_points(
                        F.sd_eb(
                            ft.boot[:, des.ids.index(s) * F.N_TERMS],
                            ft.boot_raw[:, des.ids.index(s) * F.N_TERMS],
                        ),
                        k2,
                    )
                    for s in dose_ids
                ]
            )
        )

    se_single = mean_se(F.build(bb, 1e-6, measured=sorted(measured), orients=(0,)))
    se_dup = mean_se(F.build(bb, 1e-6, measured=sorted(measured)))
    se_smooth = mean_se(F.build(bb, scale, measured=sorted(measured)))
    gain_dup = (se_single / se_dup) ** 2 / 2
    gain_all = (se_single / se_smooth) ** 2 / 2

    # 5. Shop model against rollout rescoring.
    rhos = [spearman(c["model"], c["rescored"]) for r in loaded.runs for c in r.checks if len(c["ids"]) >= 3]
    rhos = [x for x in rhos if not math.isnan(x)]

    # 6. Budgets: boards needed for a median 90% half-width of 1.5 points.
    half = np.median([(r.lift_hi - r.lift_lo) / 2 for r in m.readings.values() if r.n > 50])
    n_measured = len(measured)
    need0 = int(boards_t0 * (half / 1.5) ** 2 * (170 / n_measured))
    m1 = A.measure(STEP, "pilot-t1-only", [p1], allsig, scale, boot=60)
    sub0 = {b: rs for b, rs in bb.items() if b < nb1}
    ft0s = F.fit(F.build(sub0, scale, measured=sorted(measured)), allsig, boot=0)
    half1 = np.median([(r.lift_hi - r.lift_lo) / 2 for r in m1.readings.values() if r.n > 30])
    need1 = int(nb1 * (half1 / 1.5) ** 2 * (170 / n_measured))

    gs = A.game_stats(loaded.runs)
    model = A.export_model("pilot", m, allsig, gs, prev=None)
    out = {
        "boards_t0": boards_t0,
        "boards_t1": nb1,
        "final_margin_sd": sd,
        "scale_search": scale_rows,
        "scale": scale,
        "k": k,
        "sigma_u": m.fit.resid_sd,
        "sigma_u_t0_shared": ft0s.resid_sd,
        "sigma_u_t1_shared": m1.fit.resid_sd,
        "dose": dose_beta,
        "dose_monotone": monotone,
        "synthetic_coverage": coverage,
        "aa": {"diff": dpt, "lo": dlo, "hi": dhi, "ok": aa_ok, "a": a1.lift, "b": a2.lift},
        "se_single_binary": se_single,
        "se_duplicate_binary": se_dup,
        "se_duplicate_smoothed": se_smooth,
        "ess_gain_duplicate": gain_dup,
        "ess_gain_total": gain_all,
        "tier_agreement": m.calib,
        "shop_rank_corr": float(np.mean(rhos)) if rhos else float("nan"),
        "shop_checks": len(rhos),
        "median_half_width_t0": float(half),
        "median_half_width_t1": float(half1),
        "boards_needed_t0_for_170": need0,
        "boards_needed_t1_for_170": need1,
        "game": gs,
        "model_lambda": model["lambda"],
    }
    md = report_md(out, m, dose)
    A.write_report("phase-0-pilot", md, out)
    md2 = A.dashboard(m, allsig, sorted(measured), "Phase 0 pilot dashboard (seed pool, tier 0)")
    A.write_report("phase-0-pilot-dashboard", md2)
    # The seed pool's estimates go into their data files.
    ids = [s for s in sigils]
    A.write_estimates(
        m,
        sigils,
        [s for s in ids if s in m.readings],
        ["reports/phase-0-pilot.md", "reports/phase-0-pilot-dashboard.md"],
    )
    print(md)
    return out


def report_md(o: dict, m, dose) -> str:
    L = ["# Phase 0 pilot: pipeline validation and budgets", ""]
    L.append(
        f"Seed pool grant tournament: {o['boards_t0']} boards at tier 0 and {o['boards_t1']} shared boards at tier 1. "
        "Reproduce with `uv run analysis pilot` from `analysis/`."
    )
    L += ["", "## Smoothed outcome", ""]
    L.append(
        f"Final-margin sd on clean runs: {o['final_margin_sd']:.0f} points. Smoothing scale chosen by planted-effect t-statistics:"
    )
    L += ["", "| Scale multiple | Scale | Mean dose t | Residual sd |", "| --- | --- | --- | --- |"]
    for mult, sc, t, rs in o["scale_search"]:
        L.append(f"| {mult} | {sc:.0f} | {t:.2f} | {rs:.3f} |")
    L.append(
        f"\nChosen scale {o['scale']:.0f}; win calibration k = {o['k']:.2f}; sigma_u = {o['sigma_u']:.3f}."
    )
    L += ["", "## Pipeline validation", ""]
    L.append("| Check | Result | Pass |\n| --- | --- | --- |")
    L.append(
        f"| Planted dose-response (+10 to +160 points; raw beta, pts) | {', '.join(f'{a}: {b:+.1f}' for a, b in o['dose'])} | {'yes' if o['dose_monotone'] else 'no'} |"
    )
    L.append(
        f"| Synthetic data from the fitted model: 90% interval coverage | {100 * o['synthetic_coverage']:.0f}% | {'yes' if o['synthetic_coverage'] >= 0.8 else 'no'} |"
    )
    aa = o["aa"]
    L.append(
        f"| A/A: seed-ace-win vs its twin (lift {aa['a']:+.1f} vs {aa['b']:+.1f}) | diff {aa['diff']:+.1f} [{aa['lo']:+.1f}, {aa['hi']:+.1f}] | {'yes' if aa['ok'] else 'no'} |"
    )
    L.append(
        f"| Variance reduction: dose SE single-run binary / duplicate binary / duplicate smoothed (pts) | {o['se_single_binary']:.2f} / {o['se_duplicate_binary']:.2f} / {o['se_duplicate_smoothed']:.2f} | effective sample-size gain per run x{o['ess_gain_duplicate']:.2f} from duplicate play, x{o['ess_gain_total']:.2f} with smoothing |"
    )
    ta = o["tier_agreement"]
    L.append(
        f"| Tier agreement (lifts, tier 0 vs tier 1, shared boards) | r = {ta.get('corr', float('nan')):.2f}, disattenuated {ta.get('corr_disattenuated', float('nan')):.2f} over {ta.get('n')} sigils | {'yes' if ta.get('corr_disattenuated', 0) >= 0.7 else 'low'} |"
    )
    L.append(
        f"| Shop model vs tier-0 rollout rescoring (mean Spearman per visit) | {o['shop_rank_corr']:.2f} over {o['shop_checks']} visits | — |"
    )
    L += ["", "## Budgets", ""]
    L.append(
        f"Median 90% half-width: {o['median_half_width_t0']:.2f} pts at tier 0 ({o['boards_t0']} boards), "
        f"{o['median_half_width_t1']:.2f} pts at tier 1 ({o['boards_t1']} boards)."
    )
    L.append(
        f"Residual sd on the shared boards: tier 0 {o['sigma_u_t0_shared']:.3f}, tier 1 {o['sigma_u_t1_shared']:.3f}."
    )
    L.append(
        f"Boards for a median half-width of 1.5 pts with 170 measured sigils: about {o['boards_needed_t0_for_170']} at tier 0 "
        f"and {o['boards_needed_t1_for_170']} at tier 1."
    )
    g = o["game"]
    L += ["", "## Game as played (clean boards)", ""]
    L.append(
        f"Mean round score by round: {', '.join(f'{x:.0f}' for x in g['meanRoundScore'][1:])} (par {', '.join(str(x) for x in g['par'][1:])})."
    )
    L.append(
        f"Set rate {100 * g['setRate']:.1f}%, late overtricks {g['lateOvertricks']:.2f} per made contract, rounds 1–4 share {100 * g['earlyShare']:.1f}%, "
        f"trailer after round 5 wins {100 * g['trailerAfter5Wins']:.1f}%, median margin share {100 * g['medianMarginShare']:.1f}%."
    )
    return "\n".join(L) + "\n"
