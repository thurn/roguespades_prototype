# Final round: tier-2 calibration

Tier-2 spot checks of the top, bottom, and near-decision sigils of the final pool, plus a tier-2 against tier-0 ladder with the pool. Reproduce with `uv run python -m rsa.final round-3` from `analysis/`.

- **Spot checks:** 24 sigils, 2,400 tier-2 boards. Correlation between the working estimate and the tier-2 lift: 0.84; the working estimate falls inside the tier-2 90% interval for 54% of them.
- **Ladder:** tier 2 beats tier 0 on 88% of 300 boards with the final pool (Skill and bidding band: at least 65%).

| Group | Sigil | Working estimate | Tier 2 (90%) | Agree |
| --- | --- | --- | --- | --- |
| bottom | `ca-ace-win-two-mult` | -7.3 | -4.8 [-10.7, +1.1] | yes |
| bottom | `ca-diamond-win-two` | -7.7 | -3.0 [-8.3, +2.2] | yes |
| bottom | `cb-bid7-mult` | -7.2 | -4.3 [-9.8, +1.1] | yes |
| bottom | `cb-bid8-trick-mult` | -8.1 | -4.9 [-9.2, -0.7] | yes |
| bottom | `cb-make-x` | -7.3 | -1.6 [-5.2, +2.0] | no |
| near | `cb-opp-set-mult` | -0.1 | +2.2 [-1.7, +6.1] | yes |
| bottom | `cc-nil-bid-xmult` | -6.0 | -1.6 [-6.0, +2.8] | no |
| bottom | `e-last-xmult` | -8.7 | -0.1 [-5.1, +5.0] | no |
| bottom | `e-lead-rank-k-mult` | -6.0 | -1.2 [-6.6, +4.2] | yes |
| near | `r1-low-win-points` | -0.0 | +5.4 [+1.9, +8.9] | no |
| top | `r2-diamonds-beat` | +12.5 | +14.1 [+10.9, +17.3] | yes |
| top | `r2-four-row-x` | +15.0 | +4.1 [-0.3, +8.5] | no |
| top | `r2-high-middle-free` | +11.6 | +12.9 [+9.8, +16.0] | yes |
| top | `r2-queens-beat` | +12.9 | +12.3 [+9.1, +15.5] | yes |
| top | `r2-twos-beat-rare` | +11.5 | +16.2 [+13.0, +19.5] | no |
| near | `ra-spade-lead-x` | +0.1 | +4.9 [+1.0, +8.7] | no |
| top | `rb-streak-contract-mult` | +12.9 | +10.4 [+5.6, +15.3] | yes |
| near | `seed-ace-mult` | -0.0 | -0.3 [-5.1, +4.6] | yes |
| top | `seed-flat-x` | +11.4 | +2.8 [-0.5, +6.1] | no |
| near | `seed-nil-made-x` | -0.3 | +1.6 [-2.4, +5.6] | yes |
| near | `seed-rainbow-first` | -0.3 | -1.9 [-6.4, +2.5] | yes |
| near | `ua-three-aces-x` | -0.2 | +7.5 [+1.5, +13.6] | no |
| top | `uc-low-discard` | +12.6 | +16.1 [+12.7, +19.4] | no |
| near | `uc-spade-lead-mult` | -0.3 | +6.6 [+2.6, +10.6] | no |

Disagreements worth revisiting: `r2-four-row-x` and `seed-flat-x` measure far weaker at tier 2 (+4.1 and +2.8 against +15.0 and +11.4); several near-zero sigils (`ua-three-aces-x`, `uc-spade-lead-mult`, `r1-low-win-points`, `ra-spade-lead-x`) measure 5 to 8 points stronger at tier 2, so tier 0 understates sigils that reward stronger play.
