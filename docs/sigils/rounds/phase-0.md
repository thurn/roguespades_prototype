# Phase 0: engine and viewer

Outcome of the four Phase 0 builds. Reports: [pilot](../../../reports/phase-0-pilot.md),
[pilot dashboard](../../../reports/phase-0-pilot-dashboard.md). Plan deviations are listed in
[plan-deviations.md](../plan-deviations.md).

## 0a: kernel, grammar, and viewer

Checked by `rsim check` (run in `scripts/ci`):

- **Seed texts:** 51 of 58 §9 seeds generate their GDD text exactly; 7 differences are recorded
  as GDD typos in each data file's `gddTypo` (growth written `×5` for an additive multiplier,
  milestones written with "if", "a two" without brackets, and "contract is 10 or more tricks").
- **Simplicity rubric:** all 7 rows of the §12 table score the C shown (1, 2, 4, 3, 7, 7, 13).
- **§4 worked examples:** all 8 score as shown.
- **Random-play benchmark:** about 47 million card plays per second on one core.
- **Viewer:** lists every data file at `http://localhost:5173/sigils`.
- **CI:** `scripts/ci` runs `cargo fmt --check`, `cargo clippy -D warnings`, a release build,
  `rsim check`, `ruff`, and the viewer's format, lint, and build.

## 0b: AI tiers

Plain Spades, tier 1, 500 runs (8,000 team-rounds):

| Criterion | Target | Measured |
| --- | --- | --- |
| Team set rate | ≤ 18% | 10.8% |
| Nil success | ≥ 65% | 65.2% (887/1,360); 65.1% and 64.6% on fresh seeds, so borderline |
| Nil suicides | < 1% | 0.7% |
| Tier 2 beats tier 0 (boards) | ≥ 65% | 92.0% (300 boards) |

Other ladders: tier 1 beats tier 0 on 83% of boards; tier 2 beats tier 1 on 70%.

Throughput on 18 cores with the shop and the seed pool, against the targets:

| Tier | Target | Measured |
| --- | --- | --- |
| 0 | ≥ 100 runs/s | about 1,000 runs/s |
| 1 | ≥ 5 runs/s | about 115 runs/s |
| 2 | ≥ 0.5 runs/s | about 16 runs/s |

The nil margin in bidding rollouts was tuned to 550 points: 300 gave 61% nil success, 700 gave
74% with fewer nils and fewer points per round.

## 0c: shop, runner, and run records

- Flexible against flexible, 6,000 boards: team A wins 49.4% ± 0.9 (90%).
- A 300-board grant tournament over the seed pool runs end to end and writes records.
- Two runs from the same seed and config produce byte-identical records.
- The enumerator writes 213 common candidates (70 conditions × 3 categories, plus contract and
  nil scalers).

## 0d: analysis pipeline and pilot

Seed pool, 8,000 tier-0 boards and 1,600 shared tier-1 boards:

| Validation | Result |
| --- | --- |
| Planted effects (+10 to +160 points) | Monotone: −2.8, −0.4, +0.5, +10.2, +19.9 pts (net of price) |
| Synthetic data from a perturbed known model | 86% of 90% intervals cover the truth |
| A/A test (seed-ace-win and a twin, reported lifts) | Difference −4.1 [−11.2, +2.9] pts |
| Variance reduction | Effective sample size per run ×1.14 from duplicate play alone, ×1.69 with smoothing |
| Tier agreement | Lifts correlate 0.79 between tiers 0 and 1 (74 sigils; shared boards overstate it) |
| Shop model vs rollout rescoring | Spearman −0.00 per visit with the hand-set starting model |

These are the numbers after the Phase 0 review's fixes (stable dealing, fixed grant schedules,
lift-based A/A test, perturbed synthetic truth); the first pilot is superseded.

- **σ_u** is 0.24 at tier 0 and 0.23 at tier 1 on shared boards, so tier 0 stays the bulk tier.
- **Budget:** about 162,000 tier-0 boards (about 6 minutes without shop validation) reach a
  median half-width of 1.5 pts for 170 measured sigils.

## Shortfalls carried forward

- **Shop-model validation is weak.** One-round tier-0 rescoring over 12 deals is too noisy to
  rank offers, and the starting model was hand-set; the correlation is re-checked each round
  with fitted models.
- **Nil success is borderline** at the 65% bar across seeds.
- **The game as played is lopsided with the seed pool:** median final margin 58% of the
  winner's score (band ≤ 20%), and rounds 1–4 hold 37% of points (band ≥ 20%, but scores
  plateau after round 4 instead of growing toward par 6,000). These are design targets for the
  draft passes and rounds, not engine defects.
