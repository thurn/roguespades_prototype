# Optimization round 3 (final round)

Round 3 is the final round. The fun-score trend had not plateaued (55.8 → 47.6), but the plan's
work was stopped after three rounds (see [plan deviations](../plan-deviations.md)).

## Measurement

[Dashboard](../../../reports/round-3-dashboard.md): 120,937 tier-0 boards (1.5 times the usual
size) with a 30% tier-1 calibration, commitment arms, the flexible field, the ladder,
standalone enabler trials, a build search with ten build-chaser arms, and band re-centering.

**Fun score 51.5** [50.6, 51.8]:

| Family | Round 1 | Round 2 | Round 3 |
| --- | --- | --- | --- |
| Close and live (25) | 0.61 | 0.57 | 0.66 |
| Commitment works (15) | 0.50 | 0.50 | 0.30 |
| Archetypes viable (15) | 0.87 | 0.68 | 1.00 |
| Synergy and combos (15) | 0.28 | 0.03 | 0.00 |
| Skill and bidding (15) | 0.82 | 0.79 | 0.80 |
| Simplicity (15) | 0.22 | 0.23 | 0.23 |
| **Fun score** | **55.8** | **47.6** | **51.5** |

- **Bands:** the median pool lift sits at +0.1 pts against the same-rarity control, so the
  targets were not re-centered.
- **Global scale:** no change (factor 1.0). The round shape is wrong, not the total: mean round
  scores run 663, 1,078, 1,769, 2,716, 2,953, 3,305, 3,132, 1,901 against par 600 to 6,000, so
  late rounds lose heavily to sets.
- **Power ceiling:** the best build-chaser won 51.7% (`ca-king-hold`); no dominant attainable
  build.
- **Tier 2:** [calibration report](../../../reports/round-3-tier2.md). Working estimates and
  tier-2 lifts correlate 0.84 over 24 spot-checked sigils; tier 2 beats tier 0 on 88% of boards.

## Final audit and decisions

An auditor checked the pool against the plan's final-round list. Fixes:

- **Unmeasured amounts.** The round's retune ran after the measurement and changed 71 amounts.
  They were reverted to the measured amounts; each proposal is recorded in its sigil's history.
- **Cuts (12):** `cb-make-x`, `cb-grow-make-mult`, `cc-nil-bid-xmult`, `e-last-xmult`,
  `ca-spade-last-trick`, `seed-ace-hold`, `seed-four-row`, `rc-low-win-x`, `cb-bid7-mult`,
  `cc-nil-contract-trick`, `cb-first-trick-mult`, `ca-diamond-lead-three`: each below its
  control, mostly with a flat amount slope or a redundant twin on the same trigger.
- **`rb-trailing-x`** (measured −6.2 in round 1) was resolved as cut.
- **Records:** every kept sigil gained a round-3 line with its final reading; 27 seeds and
  enumerated sigils gained decision and opponent notes; four icon-family fields were
  normalized.
- **Naming:** the namer named and iconed the pool at once; every name, icon family, and icon
  word is unique.

**Final pool: 117 sigils (49 common, 43 uncommon, 21 rare, 4 legendary).**
