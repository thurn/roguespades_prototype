# round-1: focused tournament (new versions vs old)

Fit: 81027 boards, alpha 8, residual sd 0.403 (u units), win calibration k = 1.33, smoothing scale 25128 points.
Tier agreement: 149 sigils, lift correlation 0.40 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 2880000 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| cc-low-narrow-win | C | hybrid | 5 | 6070 | 10.7 [9.5, 11.8] | 4.4 | 6.3 | 7.6 | 73.1 | 75.4 | 3.9 |  | +30 contract points when your team wins a trick with a [2] through [7] |
| r1-low-win-points | C | points | 5 | 11853 | 9.4 [8.2, 10.5] | 5.4 | 3.9 | 12.0 | 67.3 | 69.5 | 4.5 |  | +35 contract points when your team wins a trick with a [2] through [6] |
| cb-exact-trick-mult | C | mult | 3 | 5548 | -0.1 [-1.3, 1.2] | -0.1 | 0.0 | 11.0 | 23.3 | 23.0 | 1.2 |  | +3 contract multiplier for each trick in your team's contract if your team makes its contract exactly |
| r1-exact-points-common | C | points | 2 | 8294 | -0.7 [-2.1, 0.7] | -1.7 | 1.0 | 15.2 | 24.7 | 24.5 | 1.9 | lift<band | +150 contract points if your team makes its contract exactly |
| control-common | C | points | 1 | 2192 | -4.7 [-5.6, -3.8] | -2.3 | -2.4 | 3.9 | 100.0 |  | 3.0 | lift<band, rarely decisive | +35 contract points |
| r1-exact-grow-mult | C | mult | 4 | 8259 | -7.3 [-8.8, -5.9] | -4.0 | -3.3 | 7.1 | 25.2 | 24.6 | 0.2 | lift<band, flat slope | This sigil gains +6 contract multiplier every time your team makes its contract exactly (currently +0) |
| r1-low-hold-mult | U | mult | 5 | 12616 | 17.4 [16.3, 18.6] | 7.8 | 9.6 | 22.8 | 100.0 | 100.0 | 7.5 |  | +3 contract multiplier for each [2] through [6] your team holds |
| r1-middle-free | U | enabler | 4 | 19219 | 17.0 [15.9, 18.2] | 6.5 | 10.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [6]s through [9]s even when it can follow suit |
| uc-low-hold-mult | U | mult | 5 | 6686 | 12.6 [11.5, 13.7] | 5.4 | 7.2 | 16.6 | 99.7 | 99.7 | 7.0 |  | +4 contract multiplier for each [2] through [4] your team holds |
| ub-first-trick-contract-points | U | hybrid | 4 | 6103 | 8.1 [6.9, 9.2] | 1.5 | 6.5 | 10.3 | 51.6 | 52.5 | 4.4 |  | +20 contract points for each trick in your team's contract when your team wins the first trick of a round |
| uc-exact-contract-points | U | points | 3 | 5658 | 6.1 [5.0, 7.3] | 1.7 | 4.4 | 26.4 | 23.6 | 23.5 | 5.6 |  | +65 contract points for each trick in your team's contract if your team makes its contract exactly |
| r1-exact-points | U | points | 2 | 8375 | 5.9 [4.6, 7.3] | 4.5 | 1.5 | 30.4 | 25.0 | 25.0 | 6.5 |  | +400 contract points if your team makes its contract exactly |
| r1-exact-xmult | U | xmult | 2 | 8392 | 4.7 [3.3, 6.1] | -0.2 | 4.9 | 20.2 | 23.1 | 23.6 | 3.5 |  | ×2.5 contract multiplier if your team makes its contract exactly |
| r1-first-trick-points | U | points | 3 | 12450 | 2.0 [0.6, 3.3] | 0.7 | 1.3 | 14.3 | 50.2 | 50.8 | 4.9 |  | +120 contract points when your team wins the first trick of a round |
| control-uncommon | U | points | 1 | 2118 | -1.5 [-2.4, -0.5] | -0.6 | -0.9 | 8.2 | 100.0 |  | 3.3 |  | +50 contract points |
| r1-lead-choice | U | enabler | 2 | 20199 | -5.3 [-6.4, -4.2] | -5.8 | 0.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Choose which partner leads after your team wins a trick |
| r1-any-suit-last-three | U | enabler | 2 | 19456 | -6.4 [-7.5, -5.4] | -6.0 | -0.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit on the last three tricks |
| ra-spade-win-contract | R | mult | 4 | 5451 | 22.1 [20.9, 23.3] | 11.5 | 10.6 | 24.4 | 91.1 | 91.2 | 8.1 |  | +2 contract multiplier for each trick in your team's contract when your team wins a trick with [♠] |
| r1-spade-win-mult | R | mult | 3 | 8219 | 18.9 [17.5, 20.2] | 5.5 | 13.4 | 24.3 | 91.2 | 91.8 | 7.0 |  | +8 contract multiplier when your team wins a trick with [♠] |
| r1-spade-four-x | R | xmult | 6 | 10911 | 9.3 [8.2, 10.5] | 3.8 | 5.5 | 19.4 | 39.1 | 39.7 | 5.6 |  | ×2.5 contract multiplier when your team wins four tricks with [♠] |
| control-rare | R | points | 1 | 2161 | -5.7 [-6.7, -4.7] | -1.3 | -4.4 | 11.3 | 100.0 |  | 4.9 | lift<band | +65 contract points |
| r1-exact-growth | R | xmult | 4 | 8264 | -7.9 [-9.4, -6.5] | -8.5 | 0.5 | 11.3 | 24.6 | 23.5 | -1.6 | lift<band | This sigil gains ×0.5 contract multiplier every time your team makes its contract exactly (currently ×1) |
| r1-low-cards-beat | L | enabler | 5 | 15251 | 68.2 [67.0, 69.3] | 37.8 | 30.4 | 0.0 | 0.0 | 0.0 | 0.0 | lift>ceiling | Your team's [2]s through [6]s beat every other card of their suit |
| r1-low-three-beat | L | enabler | 5 | 15204 | 49.7 [48.5, 50.8] | 26.3 | 23.4 | 0.0 | 0.0 | 0.0 | 0.0 | lift>ceiling | Your team's [2]s through [4]s beat every other card of their suit |
| rc-twos-always-win | L | enabler | 5 | 7504 | 38.4 [37.0, 39.7] | 19.2 | 19.2 | 0.0 | 0.0 | 0.0 | 0.0 | lift>ceiling | Your team's [2]s win every trick they are played to |
| ra-team-untrumpable | L | enabler | 2 | 10634 | 29.5 [28.5, 30.6] | 17.8 | 11.7 | 0.0 | 0.0 | 0.0 | 0.0 | lift>ceiling | Your team's cards can't be trumped |
| seed-flat-x2 | L | xmult | 1 | 4292 | 18.9 [17.8, 20.1] | 10.9 | 8.0 | 25.8 | 100.0 |  | 11.1 |  | ×3 contract multiplier |
| r1-honors-untrumpable | L | enabler | 5 | 20068 | 18.2 [17.1, 19.4] | 10.0 | 8.3 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [10]s through [A]s can't be trumped |
| r1-faces-untrumpable | L | enabler | 5 | 20020 | 10.0 [8.9, 11.2] | 5.1 | 5.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [Q]s through [A]s can't be trumped |
| r1-free-discards | L | enabler | 2 | 8561 | 1.8 [0.8, 2.8] | -0.1 | 1.9 | 0.0 | 0.0 |  | 0.0 |  | Your team can play cards other than [♠] even when it can follow suit |
| r1-trick-growth | L | mult | 4 | 8442 | 0.5 [-0.6, 1.6] | -1.4 | 1.9 | 18.8 | 98.4 |  | 5.6 | lift<band | This sigil gains +1 contract multiplier every time your team wins a trick (currently +0) |
| control-legendary | L | points | 1 | 2086 | -5.1 [-6.0, -4.2] | -2.1 | -3.0 | 15.0 | 100.0 |  | 8.5 | lift<band | +100 contract points |
