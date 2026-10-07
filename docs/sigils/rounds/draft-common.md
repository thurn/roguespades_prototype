# Draft pass: commons

## Brief

**Target.** Keep about 66–72 commons (110–120% of the soft target of 60), from two sources run
side by side to settle [D21](../../game-design.md#d21-candidate-generation):

- **Enumerated:** the grammar enumerator's common payoffs, hand-level screened; the most
  promising half joins the tournament.
- **Designed:** three designers, blind to the enumeration, each writing about 35 candidates
  (1.5 times a third of the target). GDD seeds at common (23, plus the two common enablers) enter
  with the designed source.

**Coverage holes so far.** The pool is empty, so every target is a hole. Priorities:

- **Enablers at common.** Only two seeds exist, and both measured poorly in the pilot
  (`seed-swap` −0.3, `seed-become-two` −11.5 pts against the common control). About a third of
  the pool should mainly change hands or play; commons should carry simple, honest enablers.
- **Bid tension.** At least a third of contract-point sigils and a quarter of multipliers should
  scale with or require the contract size.
- **Minor archetypes** (Low Cards, Rainbow, Streaks, Exact) need about five pieces each across
  rarities, so two or three commons each.
- **Named suits:** [♦] first; [♣], [♥], [♠] versions only where they add something.
- **×multipliers at common** are fine when they stay under the power ceiling.

**Archetypes that need the most help** (pilot readings, seed pool, tier 0):

- **Suits:** every [♦] seed measured at or below the control (−2.6 to −7.2 pts); milestones of
  three or five [♦] wins almost never fire (8% and 0.2% of rounds).
- **Nil and Exact:** `seed-nil-points` −2.9, `seed-exact-points` −4.5; they rarely decide wins.
- **Low Cards:** `seed-low-win` −5.6.
- **Strong today:** `seed-rainbow-first` +10.6, `seed-ace-hold` +8.0, `seed-spade-lead` +7.1,
  `seed-rainbow-mult` +6.7: whole-hand counts and broad conditions measured well.

**The game as played is lopsided.** With the seed pool the median final margin is 58% of the
winner's score and scores stop growing after round 4. Prefer payoffs whose condition creates a
decision (a bid, a lead, a hold) over unconditional stat sticks, and enablers that change the
hand before bids.

**Amounts.** Designers propose starting amounts; the pipeline retunes them every round. The
common control is "+40 contract points" and measures +3 pts of win rate net of its price; a
common should aim for about +2 pts over that control.

## Outcome

**Kept 66 commons** (target 66–72). Dashboard: [draft-common-tournament](../../../reports/draft-common-tournament.md).

### Process

- **Designed:** three designers returned 101 candidates; 8 cross-cluster duplicates were merged
  and 3 same-signature pairs merged on the critic's advice. The critic kept 119 of 121
  (designed plus seeds), fixed one tag set, and dropped `seed-flat-points` as an exact duplicate
  of `control-common`.
- **New hooks** (general mechanisms): Opening changes with a source filter, wins filtered by the
  led card, leading [♠] before broken, leading the first trick, and any-suit windows on the
  first N tricks or once the contract is won.
- **Enumerated:** 201 candidates; the hand-level screen set 9 aside, 51 merged into designed or
  seed candidates with the same signature (credited to both sources), and 16 per-event
  ×multipliers were set aside because a balanced factor (about ×1.04 per event) is not
  printable. A 32,575-board pre-tournament picked the most promising half (62) by projected
  lift after retuning and simplicity.
- **Tournaments:** 179 candidates plus controls. Tournament A, 65,954 boards; tournament B,
  127,226 boards concentrating grants on the promising and uncertain half; 19,083 tier-1
  calibration boards. Combined fit: 173,964 boards; median 90% half-width 1.1–1.6 pts by
  source. Ledger rescoring matched every one of 6.2 million rounds.

### Results by source (D21)

| Source | Measured | Median lift | Kept | Kept signatures also enumerated |
| --- | --- | --- | --- | --- |
| GDD seeds | 28 | −1.6 | 17 | 15 |
| Designed | 89 | −0.6 | 43 | 17 |
| Enumerated (not merged) | 62 | +8.8 | 6 | — |

The enumerator's amounts were set to a budget from the screen, so its candidates started far
stronger than designed ones (median +8.8 pts); lift alone does not separate the sources.
Designers supplied most of the decision-rich shapes (lead milestones, contract scaling, nil
plans, enablers); the enumerator independently produced 32 of the 66 kept signatures, which
says the designed pool covers the grammar's natural common space well.

### Pool shape after the pass

- **Categories:** 30 +mult, 20 points, 8 hybrids, 5 enablers, 3 ×mult.
- **Archetypes:** Bid High 16, Spades 14, Suits 12, Generic 11, Nil 11, Ranks 10, Streaks 9,
  Low Cards 6, Rainbow 5, Exact 3 (sigils carry several tags).
- **Retunes:** 38 amounts moved by the damped Newton step (for example `cc-nil-made-mult`
  +20 → +10, `seed-nil-points` +50 → +100). One sigil, `seed-diamond-lead`, has a flat amount
  slope: its amount is not what limits it.

### Coverage holes and concerns

- **Enablers are the main hole.** Option-style enablers (lead [♠] early, lead first, any suit,
  swaps) measure as roughly a blank in a standalone probe: they rarely change a hand enough to
  beat +40 points. Only `cb-become-spade` (+10.3), `ca-raise-one` (+2.8), and `cb-become-ace`
  (+3.0) beat the control; `seed-swap` and `cc-any-suit-first-two` are kept for their families
  and are round-1 targets.
- **Exact** has three commons; **Rainbow** and **Low Cards** have five or six.
- **The environment is thin.** The shop offered only the controls during this pass, so
  multipliers on a bare base of 10 measure strongly; later passes re-measure everything in a
  fuller pool.
- **Tier agreement is low** (lift correlation 0.24 between tiers on this pool), so later
  tournaments raise the tier-1 share.
