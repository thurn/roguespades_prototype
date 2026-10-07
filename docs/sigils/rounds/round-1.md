# Optimization round 1

## Brief

Measured pool: 140 kept sigils. Dashboard: [round-1-dashboard](../../../reports/round-1-dashboard.md).

**Fun score 55.8** (90% interval 54.7–56.7): Close and live 0.61 (median final margin 67% of the winner's score; the trailer after round 5 wins 23%), Commitment works 0.50 (online by round 4 in only 23% of committed runs), Archetypes viable 0.87, Synergy 0.28, Skill and bidding 0.82 (set rate 33%), Simplicity 0.22.

**Retunes and scale.** The pool scored above par, so the global scale moved 58 amounts by ×0.84; then 67 relative retunes. The bands were not re-centered: the median pool lift is +0.3 pts against the control, close enough to the targets.

**Power ceiling.** Ten build-chaser arms (full-pool fit): the best won 51.3% [48.5, 54.1]; no dominant attainable build.

## Structural targets

### Cuts (decided from the evidence and the critic's elegance review)

- `rc-opp-aces-fall` (rare, C 6): Opening: Every [A] the opponents hold becomes a [2] — lift -13.1 [-14.3, -11.9], fire 0%, slope +0.0/2x. **Cut:** Frustrating to play against (wipes out bought [A]s before bids with no answer) and measures −13.
- `rc-low-untrumpable` (rare, C 5): Your team's [2]s through [10]s can't be trumped — lift -5.6 [-6.6, -4.6], fire 0%, slope +0.0/2x. **Cut:** An odd middle rung of the untrumpable family that maps to no plan; −5.6 and well below the control in standalone trials.
- `ca-raise-one` (common, C 6): Opening: Raise every card your team holds by one rank — lift -6.1 [-7.9, -4.3], fire 0%, slope +0.0/2x. **Cut:** A weaker copy of seed-raise; the worst of four raise enablers.
- `ca-heart-hold-eight` (common, C 5): +15 contract multiplier if your team holds eight [♥]s — lift -3.0 [-4.9, -1.1], fire 24%, slope +1.7/2x. **Cut:** An orphan [♥] threshold with no [♥] package around it.
- `seed-swap` (common, C 5): Opening: Swap three cards with your partner — lift -2.7 [-4.4, -0.9], fire 0%, slope +0.0/2x. **Cut:** Negative even against a blank in standalone trials (−5.5 pts at shop 1): the swap rarely helps the AI.
- `seed-become-king` (uncommon, C 5): Opening: Four cards your team holds become [K]s — lift -7.5 [-9.4, -5.5], fire 0%, slope +0.0/2x. **Cut:** Random picks downgrade [A]s; −7.5, and the [K] line is thin.
- `uc-junk-to-aces` (uncommon, C 8): Opening: Two [2]s through [6]s your team holds become [A]s — lift +1.2 [-0.6, +3.1], fire 0%, slope +0.0/2x. **Cut:** One of six [A]-making enablers; near-duplicate of ub-low-to-aces.
- `ua-twos-become-aces` (uncommon, C 6): Opening: Every [2] your team holds becomes an [A] — lift -4.4 [-6.6, -2.3], fire 0%, slope +0.0/2x. **Cut:** One of six [A]-making enablers; weaker than ub-low-to-aces.
- `cb-contract-mult-made` (common, C 3): +1 contract multiplier for each trick in your team's contract if your team makes its contract — lift +0.9 [-0.1, +2.0], fire 59%, slope +3.0/2x. **Cut:** Same sentence as cb-contract-mult with a make condition few players will notice.
- `e-make-pts` (common, C 2): +45 contract points if your team makes its contract — lift +2.2 [+0.7, +3.6], fire 69%, slope +2.7/2x. **Cut:** One of five made-contract generics; a flat bonus with an extra word.
- `ub-consecutive-xmult` (uncommon, C 3): ×1.1 contract multiplier when your team wins consecutive tricks — lift +1.7 [+0.1, +3.4], fire 83%, slope +5.0/2x. **Cut:** Opaque ×1.1 compounding on a near-automatic trigger (fires 85%).
- `seed-e-lead-three-suits` (common, C 5): +40 contract points if your team leads three suits — lift +2.9 [+1.4, +4.4], fire 84%, slope +3.0/2x. **Cut:** Leading three suits is near-automatic; the Rainbow lead line keeps ca-rainbow-lead-mult and seed-rainbow-lead-x.
- `ua-spade-last-x` (uncommon, C 4): ×3 contract multiplier when your team wins the last trick of a round with [♠] — lift +1.1 [-0.3, +2.6], fire 41%, slope +3.1/2x. **Cut:** Barely differs from the plain last-trick pair because the last trick usually falls to a [♠].
- `ub-trick-xmult` (uncommon, C 2): ×1.1 contract multiplier when your team wins a trick — lift +8.5 [+6.8, +10.2], fire 99%, slope +6.9/2x. **Cut:** ×1.05 per trick is opaque; cb-trick-mult already pays per trick additively.

### Redesign targets

- **T1 (reshape)** `rc-twos-always-win` (legendary, C 5): Your team's [2]s win every trick they are played to — lift +31.4 [+30.5, +32.4], fire 0%, slope +0.0/2x. Over the power ceiling (the most frustrating piece in the pool): opponents need a way to play around it. Keep it a legendary rule change.
- **T2 (reshape)** `ra-team-untrumpable` (legendary, C 2): Your team's cards can't be trumped — lift +17.6 [+16.6, +18.6], fire 0%, slope +0.0/2x. Over the ceiling and hollows out opposing Spades builds; narrow it (for example to a rank or suit range) so opponents keep a counter. Keep it legendary.
- **T3 (replace)** `seed-flat-x2` (legendary, C 1): ×3 contract multiplier — lift +3.5 [+2.2, +4.8], fire 100%, slope +7.1/2x. Legendaries should change how a run is played, not reuse a flat ×; propose a rule-changing or growth legendary that fits no single archetype.
- **T4 (add)** **Exact**. Exact has no plan: add an unscaled Exact ladder piece at uncommon and one at rare (making the contract exactly is the idea; no contract scaling).
- **T5 (reshape)** `cb-exact-trick-mult` (common, C 3): +3 contract multiplier for each trick in your team's contract if your team makes its contract exactly — lift -3.5 [-4.7, -2.3], fire 23%, slope -0.0/2x. Drop the contract scaling from this Exact payoff (contract-scaling sprawl, and Exact should not push toward big bids).
- **T6 (reshape)** `uc-exact-contract-points` (uncommon, C 3): +65 contract points for each trick in your team's contract if your team makes its contract exactly — lift -2.4 [-3.6, -1.2], fire 23%, slope +3.1/2x. Drop the contract scaling (same reason).
- **T7 (reshape)** `ub-first-trick-contract-points` (uncommon, C 4): +20 contract points for each trick in your team's contract when your team wins the first trick of a round — lift +0.5 [-1.0, +2.1], fire 52%, slope +4.7/2x. Event-times-contract is a two-count rider; turn it into a flat event payoff or a cleaner shape.
- **T8 (reshape)** `ra-spade-win-contract` (rare, C 4): +2 contract multiplier for each trick in your team's contract when your team wins a trick with [♠] — lift -2.6 [-3.6, -1.7], fire 92%, slope +5.3/2x. Event-times-contract rider on a rare; reshape to a cleaner Spades rare.
- **T9 (reshape)** `cc-low-narrow-win` (common, C 5): +30 contract points when your team wins a trick with a [2] through [7] — lift +1.8 [+0.5, +3.1], fire 72%, slope +3.6/2x. Standardize the Low Cards range on [2] through [6].
- **T10 (reshape)** `uc-low-hold-mult` (uncommon, C 5): +4 contract multiplier for each [2] through [4] your team holds — lift -0.8 [-2.3, +0.7], fire 97%, slope +3.6/2x. Standardize the Low Cards range on [2] through [6].
- **T11 (add)** **enablers**. About a sixth of the pool is enablers against a target of about a third. Add two uncommon enablers that change how the hand is played (not more [A]-making or raises): ideas include lead control that the owner can always use, or play freedom on the last tricks.

## Outcome

Focused tournament: 81,027 tier-0 boards with new versions oversampled four to one against the
current pool, plus a tier-1 calibration ([report](../../../reports/round-1-focused.md)).

| Target | Decision | Evidence (lift over the same-rarity control, pts) |
| --- | --- | --- |
| T1 `rc-twos-always-win` | Cut, no replacement | Old +38.4; reshaped "beats" options +68.2 ([2]–[6]) and +49.7 ([2]–[4]): a "beats" card also makes low [♠]s top trumps |
| T2 `ra-team-untrumpable` | Replaced by `r1-faces-untrumpable` | +10.0 [+8.9, +11.2] against +29.5; the [10]–[A] option was +18.2 |
| T3 `seed-flat-x2` | Replaced by `r1-free-discards` | +1.8 against +18.9 (the flat ×3 after retuning); a rule-changing legendary instead of a flat × |
| T4 Exact add | Added `r1-exact-xmult` (uncommon) | +4.7; the rare Exact growth was −7.9 |
| T5 `cb-exact-trick-mult` | Cut | −0.1; the Exact ladder moves to unscaled uncommons |
| T6 `uc-exact-contract-points` | Replaced by `r1-exact-points` | +5.9 against +6.1, simpler (C 2 against 3), no contract rider |
| T7 `ub-first-trick-contract-points` | Kept (revert) | The flat version was clearly worse: +2.0 against +8.1 |
| T8 `ra-spade-win-contract` | Replaced by `r1-spade-four-x` | +9.3 against +22.1 (over the ceiling) |
| T9 `cc-low-narrow-win` | Replaced by `r1-low-win-points` | +9.4 against +10.7; standard [2]–[6] range |
| T10 `uc-low-hold-mult` | Replaced by `r1-low-hold-mult` | +17.4 against +12.6; standard range, retuning scales it down |
| T11 enablers | Added `r1-middle-free`, moved to rare | +17.0 at uncommon; lead choice (−5.3) and any suit on the last three tricks (−6.4) failed again |

**Pool after round 1:** 126 kept (59 common, 41 uncommon, 21 rare, 5 legendary); 14 cuts, 2
further cuts, 6 replacements, 2 additions. Uncommons sit just under the soft range (45–75).

**Fun score, round 1 measurement:** 55.8 [54.7, 56.7]. The next round's measurement shows the
trend.

**Notes and open issues**

- **Amount retuning:** 67 relative retunes after a ×0.84 global scale; 14 sigils have flat
  slopes (the amount isn't what limits them).
- **Over the ceiling after this round:** `r1-low-hold-mult` (retuning scales it), and the
  uncommon `uc-twos-beat` (+14.6), which shares the "beats" mechanism.
- **Critic concerns carried forward:** the "beats" mechanism needs to exclude [♠] (and the text
  renderer must print that range) before any low-card "beats" can return; the [Q]–[A] range is a
  new honor band; `r1-free-discards` lets a nil bidder duck almost every trick.
- **Commitment works** (online by round 4 in 23% of committed runs) is the weakest family and
  likely needs offer-density levers ([D7](../../game-design.md#d7-offer-density)), which are
  rule changes outside this plan.
