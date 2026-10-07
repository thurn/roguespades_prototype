# Rogue Spades 2.0: sigil design final report

This report closes the run of the [sigil design plan](../sigil-design-plan.md). The pool lives
in `data/sigils/` (one file per candidate, including every cut and replaced one), the compact
table in [registry.md](registry.md), and the browsable site in `viewer/`
(`npm --prefix viewer run dev`, then `http://localhost:5173/sigils`). The process notes are in
[rounds/](rounds/), and every deliberate change to the plan is in
[plan-deviations.md](plan-deviations.md).

## Pool counts

**117 kept sigils** against a soft target of about 145 (P21's range is 110–180):

| Rarity | Kept | Soft target (±25%) |
| --- | --- | --- |
| Common | 49 | 60 (45–75) |
| Uncommon | 43 | 60 (45–75), just under |
| Rare | 21 | 20 (15–25) |
| Legendary | 4 | 5 (4–6) |

- **Categories:** 34 +mult, 30 points, 19 ×mult, 15 hybrids, 19 enablers. About 29% of the pool
  mainly changes hands or play (enablers plus hybrids), against a target of about a third.
- **Sources:** 90 designed, 23 GDD seeds, 4 enumerated-only. 541 data files exist in all, including controls and every cut or replaced candidate.
- **Archetype tags:** Bid High 35, Spades 26, Ranks 24, Suits 22, Rainbow 21, Nil 21, Streaks 20,
  Low Cards 17, Generic 16, Exact 10. Every major has payoffs in all three scoring categories
  across at least two rarities, plus tagged enablers, except Spades, whose +multiplier payoffs
  are common-only (its uncommon and rare multipliers are hybrids or ×multipliers).
- **Mean simplicity S:** 0.23.

## Fun score

| Family | Round 1 | Round 2 | Round 3 |
| --- | --- | --- | --- |
| Close and live (25) | 0.61 | 0.57 | 0.66 |
| Commitment works (15) | 0.50 | 0.50 | 0.30 |
| Archetypes viable (15) | 0.87 | 0.68 | 1.00 |
| Synergy and combos (15) | 0.28 | 0.03 | 0.00 |
| Skill and bidding (15) | 0.82 | 0.79 | 0.80 |
| Simplicity (15) | 0.22 | 0.23 | 0.23 |
| **Fun score (90% interval)** | **55.8 [54.7, 56.7]** | **47.6 [46.7, 48.5]** | **51.5 [50.6, 51.8]** |

The trend did not plateau. Archetypes viable improved once Ranks stopped dominating winning
builds (41% of winning flexible builds in round 2, 22% in round 3). Synergy collapsed because
the factorization machine's pair terms are heavily regularized and found almost no strong pairs;
that family measures the model more than the pool.

## Remaining coverage holes and open risks

- **Late-round collapse (Close and live).** Mean round scores fall after round 7 (1,901 in round
  8 against par 6,000), the final margin is typically 72% of the winner's score, and a third of
  contracts are set. Win-probability play makes trailing teams overbid late, and large
  multipliers apply to sets. Levers: set penalties, the starting multiplier (P29), and bid-tension
  pieces that reward making big contracts rather than bidding them.
- **Commitment works** (online by round 4 in 30% of committed runs) needs the offer-density
  levers of [D7](../game-design.md#d7-offer-density): run pools or affinity weighting.
- **Bid tension is under target:** 4 of 30 point sigils and 8 of 53 multipliers scale with or
  require the contract size (targets: a third and a quarter). The critic's push against
  contract-scaling riders and the target pull in opposite directions; the next pass should add
  clean, contract-scaled Bid High pieces rather than riders on other triggers.
- **Exact** has three dedicated pieces, all on "makes its contract exactly", and is almost never
  online in commitment arms.
- **Enablers vs flat points.** Most enablers beat a blank by 10–38 pts when bought early but lose
  to a flat-points control; only narrow play-freedom and card-strength rules beat it. Pure options
  (lead choice, any suit late, swaps) failed at every rarity.
- **Legendaries** sit near their control (+1.6 to +6.7 against a +8 target); none is a run-defining
  build-around yet.
- **Unstable amounts.** Lifts of the same sigils correlate only 0.31 between rounds 2 and 3, and
  some amounts oscillated under alternating global and relative retunes. A further round should
  damp retunes and pool rounds.
- **Below-control commons.** In the final fit most commons sit below the common control; they are
  kept for coverage with retune proposals recorded in their histories.

## Known biases and their measured size

| Bias | Measured size |
| --- | --- |
| AI blind spots | Skill gradient (tier 1 − tier 0) sd 2.7 pts; 24 kept sigils above ±3 pts. Tier 2 rates several near-zero sigils 5–8 pts higher |
| Shop model error | Rank correlation with tier-0 rollout rescoring 0.07 per visit (the rescoring itself is noisy); values come from grants only |
| Hidden opponent sigils | Not measured; recorded as an assumption |
| Grant economy | Grants hold their slot for the run (a dead slot costs a weak sigil); game-level metrics use clean boards only |
| Moving environment | Same-sigil lifts correlate 0.31 between rounds 2 and 3 |
| Overfitting to the AI | Working estimates against tier 2: correlation 0.84, 54% inside the tier-2 interval |

## Pending playtest targets

- The 40% human choice rate for enablers offered beside a same-price payoff (P30): pending.
- Simplicity rubric calibration against time-to-explain and prediction errors (P28): pending.
- Readability of the [Q]–[A] and [7]–[9] ranges, untrumpable and "beats" rules, and the
  free-discard legendary: pending.

## Phase 0 shortfalls

- **Shop-model validation** stayed weak all run (rank correlation near zero), because one-round
  rescoring over 12 deals is too noisy.
- **Nil success** sits at the 0b bar (65–66% across seeds).
- **Tier agreement** between tiers 0 and 1 stayed between 0.23 and 0.40 on full pools, and the
  shared boards overstate it; tier 2 agreed better (0.84 on spot checks).
