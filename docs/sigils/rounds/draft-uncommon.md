# Draft pass: uncommons

## Brief

**Target.** Keep about 66–72 uncommons (110–120% of the soft target of 60). The shop now offers
the 66 kept commons and the controls; uncommon candidates enter runs only through grants. GDD
seeds at uncommon (20) enter with the designed source; there is no enumerated source at this
rarity ([D21](../../game-design.md#d21-candidate-generation)).

**Coverage holes after the commons pass** ([outcome](draft-common.md#outcome)):

- **Enablers, above all.** Only three commons beat their control, and every one changes the
  hand at Opening (`cb-become-spade` three cards become [♠]s, `ca-raise-one`, `cb-become-ace`).
  Pure options (lead [♠] early, lead first, any suit, swaps) measure close to a blank in
  standalone play. Uncommon enablers should give a large, visible change to the hand or to who
  wins tricks: more cards changed, better destinations, or stronger play freedom. The pool needs
  about a third of its pieces to mainly change hands or play; at uncommon, aim for at least a
  third enablers or hybrids.
- **Exact** (three commons), **Rainbow** and **Low Cards** (five to six each) need uncommon
  payoffs; Streaks has nine commons but no uncommon yet besides `seed-four-row`.
- **×Multipliers belong mostly at uncommon and above.** Commons have only three. Each major
  archetype needs payoffs in all three scoring categories across at least two rarities.
- **Bid tension.** At least a quarter of multipliers should scale with or require the contract
  size.
- **Named suits:** [♦] first; [♥] has one common payoff for `seed-become-heart`; add [♣]/[♥]/[♠]
  versions only where they add something.

**Pilot readings for the uncommon seeds** (seed pool, tier 0; lift over the uncommon control):
`seed-flat-mult` +9.3, `seed-four-row` +4.2, `seed-four-aces` +3.0, `seed-ace-three` +2.6;
`seed-diamond-five` −11.9 (fires 0.2% of rounds), `seed-bid10-mult` −9.1 (fires 4%),
`seed-become-heart` −15.1, `seed-lead-choice` −6.6. Five-trick milestones in one suit almost
never fire; whole-hand counts and broad conditions measure well.

**The game is lopsided.** With the commons pool, clean games still end with a median margin of
53% of the winner's score, and a third of contracts are set. Prefer pieces that reward a
decision over pieces that snowball a lead.

**Amounts.** The uncommon control is "+60 contract points"; an uncommon should aim for about
+3 pts of win rate over it. Amounts are retuned every round.

## Outcome

**Kept 44 uncommons** (soft target 60; range 45–75), plus two Opening raise enablers held for
re-measurement. Dashboard: [draft-uncommon-tournament](../../../reports/draft-uncommon-tournament.md).

### Process

- **Designed:** three designers returned 94 candidates; 6 cross-cluster duplicates merged. The
  critic kept 104 of 108 (designed plus seeds), fixed three low-card ranges to [2] through [6]
  and one honor range to [J] through [A], and dropped `seed-contract-points-u` as a duplicate of
  a kept common.
- **New hooks** (general): filtered "can't be trumped", filtered Opening raises, filtered cards
  playable even when following suit, filtered cards that outrank the [A], Opening changes to the
  opponents' cards, compounding contract-scaled ×multipliers, and a bid trigger for outbidding
  the opponents.
- **Screen:** set aside `seed-bid10-mult` (0.3% fire) and `seed-diamond-five` (0%).
- **Tournaments:** 106 candidates. A first measurement was discarded: with the kept commons now
  for sale, teams filled their slots and the shop sold low-valued grants, tripling standard
  errors. Grants now hold their slot for the run. Re-run: 39,854 + 76,880 tier-0 boards and
  19,220 tier-1 calibration boards; median standard error 0.75 pts.

### Results

- Holding a grant for the whole run makes a weak sigil pay for a dead slot, so lifts sit lower
  than in the commons pass. Against the uncommon control (+60 contract points, itself worth
  +7.7 pts over nothing), the median candidate measured −9.6 pts; 11 kept uncommons beat it.
- **Strong shapes:** play-freedom and trick-strength rules for low cards (`uc-low-discard`
  +13.1, `uc-twos-beat` +11.4), source-filtered conversions to [A]s, contract-scaled points on
  frequent events, and nil points per low card held.
- **Weak shapes:** conditional ×1.5 multipliers that fire in a quarter of rounds, all-suit and
  single-suit milestones, and random Opening suit changes (`seed-become-heart` −36.5, its
  random picks wreck the hand).
- **A hook bug:** the filtered Opening raise was compiled but never applied, so
  `ua-spade-raise` and `ua-side-raise` measured as blanks. Fixed; both are held as candidates
  and measured again with the rares.
- **Kept:** 10 enablers, 9 hybrids, 9 points, 8 +mult, 8 ×mult; 38 designed and 6 seeds.
  28 amounts retuned. `ua-diamond-two-x` has a flat amount slope (structural).
- **Tier agreement** rose to 0.25 once grants held their slot.

### Coverage holes

- **Suits** (4 uncommons) and **Exact** (3) are thin; Exact's uncommon payoffs all measured far
  below the control.
- **×multipliers at uncommon** are kept mostly for coverage and depend on retuning upward.
- **Enablers** are still mostly weaker than a flat +60 points: the AI turns stronger hands into
  overtricks more than into higher bids, and overtricks score nothing.
