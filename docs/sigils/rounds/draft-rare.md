# Draft pass: rares and legendaries

## Brief

**Target.** Keep about 22–24 rares and 6 legendaries (110–120% of the soft targets of 20 and 5).
The shop now offers the kept commons and uncommons and the controls; candidates enter runs only
through grants. GDD seeds at rare (14) and legendary (1) enter with the designed source.

**Power matters most here.** Every designer names likely partners for each candidate; those
pairs are oversampled as joint grants, and the pass ends with a first build search whose
strongest builds play as build-chaser arms. The power ceiling is a lift of at most 12 pts and no
pair above a 65% win rate; a broken combo that needs luck to assemble is fine, a dominant
strategy is not.

**Legendaries** are about five build-arounds. They need not fit any one archetype; each should
change how a run is played and be memorable when it appears (about once every two runs).

**What earlier passes found:**

- Opening hand changes that upgrade cards (raise, become [♠], become [A], source-filtered
  changes that only touch low cards) are the enablers that measure well; pure options (lead
  [♠] early, lead first, swaps) measure close to a blank.
- Whole-hand counts and broad conditions measure well; five-trick milestones in one suit almost
  never fire.
- The game is lopsided (median final margin about half the winner's score): favor pieces that
  reward decisions, defense, or comebacks over pieces that snowball a lead.
- Rare seeds from the pilot: `seed-raise` +12.4, `seed-ace-mult` +9.9, `seed-spade-mult` +5.7
  over the rare control; growth seeds (`seed-grow-make8`, `seed-grow-nil`, `seed-grow-exact`) and
  `seed-any-suit` measured −7 to −11.

**Amounts.** The rare control is "+80 contract points" and the legendary control "+120 contract
points"; targets are about +5 and +8 pts of win rate over them. Amounts are retuned every round.

## Outcome

**Kept 22 rares and 6 legendaries.** The two Opening raise enablers held from the uncommon pass
were re-measured and kept (46 uncommons). The draft pool now has **140 sigils: 66 common,
46 uncommon, 22 rare, 6 legendary.** Dashboard:
[draft-rare-tournament](../../../reports/draft-rare-tournament.md).

### Process

- **Designed:** three designers returned 39 candidates (30 rare, 9 legendary) with partner
  nominations; two were deferred because their hooks were not built (a payoff scaled by the
  opponents' contract, a bid-gated hand change). New hooks: "beats" that also beats trumps,
  Opening changes to the opponents' cards with a source filter, a run-state trigger (`behind`),
  and wins counted for the opponents.
- **Critic:** kept 50 of 52, fixed `seed-diamond-mult` into a [♦] contract-scaled payoff, and
  dropped `rb-side-untrumpable` as functionally identical to `ra-team-untrumpable`.
- **Tournaments:** 52 candidates with 117 nominated partner pairs oversampled as joint grants;
  21,161 + 40,821 tier-0 boards and a tier-1 calibration. The residual spread rose to 0.42 (rare
  and legendary grants swing games far more).
- **The screen** wrongly set aside `rb-trailing-x`: a one-round screen has no run score, so
  "behind" never holds there. It is restored and measured in round 1.

### Results

- **Over the ceiling:** `rc-low-beats` ([2]–[4] beat every other card of their suit) measured
  +52.9 pts; as a rule change it has no amount to retune, so it was cut (`uc-twos-beat` keeps
  the shape at a safe size). `rc-twos-always-win` (+19.6) and `ra-team-untrumpable` (+13.3) are
  over the +12 ceiling but kept as legendary build-arounds, flagged for round 1.
- **Strong rares:** `ra-jq-become-aces` +14.2, `ra-side-win-points` +11.3,
  `rb-streak-contract-mult` +8.9, `ra-diamond-hold-x` +8.1, `seed-two-win` +6.8, `seed-raise`
  +5.5 over the rare control.
- **Cut:** every rare growth sigil (−10 to −14), whole-suit conversions
  (`ra-clubs-to-diamonds`, `ra-all-diamonds`), broad low-card raises (−17 to −20, they push
  cards out of the low ranges payoffs reward and the AI overbids the raised hands), and
  `seed-any-suit`.
- **Tier agreement** on this batch was about zero: the tier-1 calibration was small and the
  rare effects are high-variance.

### First build search

A beam search over the fitted values found its strongest attainable builds among the uncommon
partners granted in this tournament. Ten build-chaser arms at tier 1, narrowed by successive
halving to three finalists with 840 runs each, all won 49.9–50.4% against the flexible field.
Duplicate boards mirror exactly when the chaser never sees its pieces, so this first reading
only says that no build is reliably assemblable from random offers; round 1 repeats the search
on a full-pool fit.

### Coverage holes going into round 1

- **Exact** has 3 commons, 1 uncommon, and no rare; **Suits** is thin above common.
- **Enablers** that beat flat points are few, and the strongest ones are low-card rule changes.
- **The power ceiling:** two legendaries are over it.
