# Phase 0 pilot: pipeline validation and budgets

Seed pool grant tournament: 8000 boards at tier 0 and 1600 shared boards at tier 1. Reproduce with `uv run analysis pilot` from `analysis/`.

## Smoothed outcome

Final-margin sd on clean runs: 24615 points. Smoothing scale chosen by planted-effect t-statistics:

| Scale multiple | Scale | Mean dose t | Residual sd |
| --- | --- | --- | --- |
| 0.25 | 6154 | 4.93 | 0.476 |
| 0.5 | 12307 | 5.20 | 0.373 |
| 1.0 | 24615 | 5.29 | 0.250 |
| 2.0 | 49230 | 5.22 | 0.146 |

Chosen scale 24615; win calibration k = 1.92; sigma_u = 0.250.

## Pipeline validation

| Check | Result | Pass |
| --- | --- | --- |
| Planted dose-response (+10 to +160 points; raw beta, pts) | 10: -5.1, 20: -3.8, 40: +0.9, 80: +8.5, 160: +20.0 | yes |
| Synthetic data from the fitted model: 90% interval coverage | 95% | yes |
| A/A: seed-ace-win vs its twin (lift +1.4 vs +0.1) | diff -2.1 [-5.8, +1.7] | yes |
| Variance reduction: dose SE single-run binary / duplicate binary / duplicate smoothed (pts) | 2.44 / 1.70 / 1.37 | effective sample-size gain per run x1.04 from duplicate play, x1.58 with smoothing |
| Tier agreement (lifts, tier 0 vs tier 1, shared boards) | r = 0.82, disattenuated 1.00 over 74 sigils | yes |
| Shop model vs tier-0 rollout rescoring (mean Spearman per visit) | 0.03 over 4540 visits | — |

## Budgets

Median 90% half-width: 4.52 pts at tier 0 (8000 boards), 6.86 pts at tier 1 (1600 boards).
Residual sd on the shared boards: tier 0 0.252, tier 1 0.244.
Boards for a median half-width of 1.5 pts with 170 measured sigils: about 166551 at tier 0 and 76917 at tier 1.

## Game as played (clean boards)

Mean round score by round: 1028, 1850, 2606, 3172, 3473, 3618, 3913, 3564 (par 600, 850, 1150, 1600, 2250, 3100, 4300, 6000).
Set rate 21.8%, late overtricks 1.29 per made contract, rounds 1–4 share 37.2%, trailer after round 5 wins 20.9%, median margin share 57.7%.
