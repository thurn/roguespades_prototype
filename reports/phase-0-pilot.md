# Phase 0 pilot: pipeline validation and budgets

Seed pool grant tournament: 8000 boards at tier 0 and 1600 shared boards at tier 1. Reproduce with `uv run analysis pilot` from `analysis/`.

## Smoothed outcome

Final-margin sd on clean runs: 25128 points. Smoothing scale chosen by planted-effect t-statistics:

| Scale multiple | Scale | Mean dose t | Residual sd |
| --- | --- | --- | --- |
| 0.25 | 6282 | 4.69 | 0.466 |
| 0.5 | 12564 | 4.86 | 0.363 |
| 1.0 | 25128 | 4.94 | 0.242 |
| 2.0 | 50257 | 4.88 | 0.141 |

Chosen scale 25128; win calibration k = 1.94; sigma_u = 0.242.

## Pipeline validation

| Check | Result | Pass |
| --- | --- | --- |
| Planted dose-response (+10 to +160 points; raw beta, pts) | 10: -2.8, 20: -0.4, 40: +0.5, 80: +10.2, 160: +19.9 | yes |
| Synthetic data from the fitted model: 90% interval coverage | 86% | yes |
| A/A: seed-ace-win vs its twin (lift +5.8 vs +9.9) | diff -4.1 [-11.2, +2.9] | yes |
| Variance reduction: dose SE single-run binary / duplicate binary / duplicate smoothed (pts) | 2.51 / 1.66 / 1.36 | effective sample-size gain per run x1.14 from duplicate play, x1.69 with smoothing |
| Tier agreement (lifts, tier 0 vs tier 1, shared boards) | r = 0.79 over 74 sigils (shared boards overstate it) | yes |
| Shop model vs tier-0 rollout rescoring (mean Spearman per visit) | -0.00 over 4460 visits | — |

## Budgets

Median 90% half-width: 4.45 pts at tier 0 (8000 boards), 6.57 pts at tier 1 (1600 boards).
Residual sd on the shared boards: tier 0 0.242, tier 1 0.231.
Boards for a median half-width of 1.5 pts with 170 measured sigils: about 162105 at tier 0 and 70531 at tier 1.

## Game as played (clean boards)

Mean round score by round: 1005, 1816, 2672, 3081, 3484, 3563, 3819, 3766 (par 600, 850, 1150, 1600, 2250, 3100, 4300, 6000).
Set rate 21.9%, late overtricks 1.29 per made contract, rounds 1–4 share 36.8%, trailer after round 5 wins 21.4%, median margin share 58.0%.
