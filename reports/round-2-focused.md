# round-2: focused tournament (new versions vs old)

Fit: 81027 boards, alpha 20, residual sd 0.284 (u units), win calibration k = 1.85, smoothing scale 25128 points.
Tier agreement: 141 sigils, lift correlation 0.23 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 2880000 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| control-common | C | points | 1 | 3371 | -1.6 [-2.3, -0.9] | -1.0 | -0.6 | 8.8 | 100.0 |  | 2.8 | lift<band | +25 contract points |
| r2-diamonds-beat | U | enabler | 3 | 18087 | 18.4 [17.5, 19.3] | 10.8 | 7.6 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [♦]s beat every other card of their suit |
| r2-four-row-x | U | xmult | 5 | 12309 | 17.5 [16.5, 18.5] | 9.9 | 7.6 | 23.5 | 37.7 | 38.6 | 8.6 |  | ×3 contract multiplier when your team wins four tricks in a row |
| r2-diamond-honor-hold | U | points | 6 | 12342 | 16.7 [15.8, 17.5] | 8.7 | 8.0 | 24.0 | 94.7 | 95.0 | 9.1 |  | +40 contract points for each [J] through [A] of [♦] your team holds |
| uc-twos-beat | U | enabler | 4 | 10111 | 14.1 [13.0, 15.2] | 8.0 | 6.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s beat every other card of their suit |
| r2-queens-beat | U | enabler | 4 | 18560 | 10.8 [9.8, 11.7] | 7.2 | 3.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [Q]s beat every other card of their suit |
| r2-opp-win-points | U | points | 2 | 21316 | 10.3 [9.4, 11.2] | 6.1 | 4.2 | 25.2 | 98.7 | 98.4 | 8.4 |  | +15 contract points when the opponents win a trick |
| r2-diamond-seven-x | U | xmult | 5 | 8982 | 7.0 [5.7, 8.2] | 3.1 | 3.9 | 16.6 | 44.6 | 45.1 | 5.0 |  | ×2.5 contract multiplier if your team holds seven [♦]s |
| r2-diamond-honor-lead | U | mult | 6 | 12310 | 6.2 [5.3, 7.1] | 3.9 | 2.3 | 16.0 | 58.1 | 59.4 | 6.4 |  | +12 contract multiplier when your team leads a [J] through [A] of [♦] |
| r2-diamond-lead-mult | U | mult | 3 | 9134 | 6.1 [4.8, 7.3] | 3.9 | 2.1 | 14.6 | 81.2 | 82.7 | 6.0 |  | +6 contract multiplier when your team leads [♦] |
| ub-low-to-aces | U | enabler | 8 | 9758 | 5.5 [4.4, 6.5] | 2.4 | 3.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three [2]s through [9]s your team holds become [A]s |
| control-uncommon | U | points | 1 | 3359 | 0.6 [-0.2, 1.3] | -0.1 | 0.6 | 11.6 | 100.0 |  | 4.7 |  | +35 contract points |
| r2-low-to-two-aces | U | enabler | 8 | 18691 | -0.3 [-1.2, 0.6] | -0.5 | 0.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two [2]s through [9]s your team holds become [A]s |
| r2-last-trick-grow | U | mult | 5 | 13168 | -0.3 [-1.2, 0.6] | -1.3 | 1.0 | 12.6 | 49.7 | 50.1 | 3.9 | lift<band | This sigil gains +8 contract multiplier every time your team wins the last trick of a round (currently +0) |
| r2-kings-untrumpable | U | enabler | 4 | 18320 | -2.7 [-3.7, -1.7] | -1.8 | -0.9 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team's [K]s can't be trumped |
| r2-last-trump-x | U | xmult | 4 | 13389 | -3.3 [-4.2, -2.5] | -4.9 | 1.5 | 9.4 | 12.2 | 12.2 | 0.9 | lift<band | ×2.5 contract multiplier when your team wins the last trick of a round by trumping |
| r1-middle-free | R | enabler | 4 | 11953 | 14.0 [13.0, 15.1] | 6.8 | 7.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [6]s through [9]s even when it can follow suit |
| r2-twos-beat-rare | R | enabler | 4 | 19154 | 12.5 [11.6, 13.4] | 6.1 | 6.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s beat every other card of their suit |
| r2-high-middle-free | R | enabler | 4 | 22651 | 12.3 [11.4, 13.2] | 4.7 | 7.6 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [7]s through [9]s even when it can follow suit |
| ra-jq-become-aces | R | enabler | 7 | 7992 | 11.6 [10.5, 12.6] | 4.6 | 7.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Every [J] through [Q] your team holds becomes an [A] |
| control-rare | R | points | 1 | 3352 | 1.3 [0.6, 2.0] | 0.1 | 1.2 | 14.0 | 100.0 |  | 4.8 |  | +45 contract points |
| r2-queens-become-aces | R | enabler | 6 | 15532 | -3.3 [-4.3, -2.3] | -3.2 | -0.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [Q] your team holds becomes an [A] |
| control-legendary | L | points | 1 | 3333 | 1.0 [0.2, 1.8] | 0.9 | 0.1 | 16.8 | 100.0 |  | 5.9 |  | +70 contract points |
