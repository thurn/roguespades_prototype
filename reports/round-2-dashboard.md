# round-2: dashboard

Fit: 118289 boards, alpha 20, residual sd 0.469 (u units), win calibration k = 1.23, smoothing scale 25128 points.
Tier agreement: 130 sigils, lift correlation 0.29 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 4200992 rounds.

**Fun score: 47.6** (90% interval 46.7–48.5)

| Family | Weight | Score | Sub-metrics |
| --- | --- | --- | --- |
| Close and live | 25 | 0.57 | median margin / winner 0.73; trailer after round 5 wins 0.21; rounds 1-4 share 0.35 |
| Commitment works | 15 | 0.50 | online by round 4 0.26; committed win rate 0.53 |
| Archetypes viable | 15 | 0.68 | archetypes with committed win 40-60% 1.00; largest archetype share of winning flexible builds 0.41 |
| Synergy and combos | 15 | 0.03 | same-archetype pair gain over parts 0.00; strong pairs (>=50% over parts) 1; strong cross-archetype pairs 0 |
| Skill and bidding | 15 | 0.79 | stronger tier beats tier 0 (boards) 0.81; late overtricks per made 0.91; set rate 0.34 |
| Simplicity | 15 | 0.23 | mean S 0.23 |

Commitment: {"per_archetype": {"Suits": 0.49875, "Spades": 0.5825, "Ranks": 0.54375, "BidHigh": 0.5075, "Nil": 0.52875, "LowCards": 0.51125, "Rainbow": 0.5625, "Streaks": 0.515, "Exact": 0.51875}, "committed_win": 0.5298611111111111, "online": 0.26375, "online_by_arch": {"Suits": 0.17375, "Spades": 0.52875, "Ranks": 0.16875, "BidHigh": 0.45125, "Nil": 0.31375, "LowCards": 0.23375, "Rainbow": 0.13875, "Streaks": 0.35125, "Exact": 0.01375}, "winning_share": {"Suits": 0.034, "Spades": 0.13, "Ranks": 0.41133333333333333, "BidHigh": 0.13133333333333333, "Nil": 0.02266666666666667, "LowCards": 0.06133333333333333, "Rainbow": 0.05, "Streaks": 0.09266666666666666, "Exact": 0.0}}

Ladder (tier 1 vs tier 0 with the pool, boards): 0.805

Global factor 0.70

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| cb-become-spade | C | enabler | 6 | 4107 | 11.5 [9.8, 13.3] | 4.7 | 6.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [♠]s |
| ca-spade-lead-three | C | hybrid | 6 | 2490 | 10.0 [8.2, 11.7] | 3.1 | 6.8 | 7.2 | 31.0 | 33.1 | 3.6 |  | +30 contract multiplier when your team leads [♠] three times |
| cc-nil-made-mult | C | mult | 2 | 6567 | 8.4 [7.1, 9.8] | 4.0 | 4.4 | 7.0 | 24.6 | 28.0 | 4.5 |  | +20 contract multiplier if your team makes a nil |
| seed-rainbow-mult | C | mult | 5 | 2801 | 7.3 [5.8, 8.9] | 2.8 | 4.5 | 6.5 | 31.0 | 35.0 | 3.0 |  | +25 contract multiplier if your team wins tricks with all four suits |
| e-hold-suit-s-mult | C | mult | 3 | 6433 | 6.8 [5.6, 8.1] | 3.7 | 3.1 | 8.3 | 100.0 | 100.0 | 3.4 |  | +2 contract multiplier for each [♠] your team holds |
| cb-make8-mult | C | mult | 4 | 2212 | 6.4 [5.1, 7.7] | 1.8 | 4.6 | 6.6 | 18.3 | 19.7 | 2.7 |  | +25 contract multiplier if your team makes a contract of 8 or more |
| cc-nil-bid-xmult | C | xmult | 2 | 7676 | 6.3 [4.9, 7.6] | 2.5 | 3.8 | 8.0 | 42.1 | 47.4 | 3.7 |  | ×2 contract multiplier when your team bids nil |
| cb-become-ace | C | enabler | 7 | 15296 | 6.2 [4.9, 7.4] | 2.8 | 3.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two cards your team holds become [A]s |
| e-trump-mult | C | mult | 3 | 2846 | 6.0 [4.6, 7.4] | 1.8 | 4.2 | 7.7 | 82.9 | 84.3 | 2.4 |  | +5 contract multiplier when your team wins a trick by trumping |
| cb-spade-seven | C | mult | 5 | 2411 | 6.0 [4.6, 7.4] | 2.7 | 3.3 | 7.6 | 57.3 | 57.7 | 3.9 |  | +20 contract multiplier if your team holds seven [♠]s |
| seed-last-trick | C | mult | 3 | 2667 | 5.8 [4.4, 7.2] | 2.0 | 3.8 | 6.6 | 48.6 | 49.2 | 0.9 | flat slope | +20 contract multiplier when your team wins the last trick of a round |
| seed-trump-points | C | points | 3 | 1970 | 5.7 [4.1, 7.4] | 3.0 | 2.8 | 6.9 | 84.9 | 86.1 | 2.7 |  | +30 contract points when your team wins a trick by trumping |
| cc-nil-contract-trick | C | hybrid | 2 | 4079 | 5.7 [4.6, 6.7] | -1.1 | 6.8 | 3.2 | 100.0 | 100.0 | 3.0 | lift<band | +15 nil points for each trick in your team's contract |
| cb-opp-set-mult | C | mult | 2 | 1570 | 5.5 [4.0, 7.1] | 1.9 | 3.7 | 7.7 | 44.8 |  | 1.9 | flat slope | +20 contract multiplier if the opponents miss their contract |
| cc-two-hold-mult | C | mult | 4 | 2877 | 5.5 [4.1, 7.0] | 1.3 | 4.2 | 3.7 | 80.0 | 83.1 | 0.7 | flat slope | +5 contract multiplier for each [2] your team holds |
| seed-contract-points | C | points | 2 | 2986 | 5.4 [4.2, 6.7] | 2.4 | 3.0 | 5.0 | 100.0 | 100.0 | 2.0 | rarely decisive | +10 contract points for each trick in your team's contract |
| cc-low-last-mult | C | hybrid | 6 | 3333 | 5.4 [4.1, 6.7] | 1.6 | 3.8 | 7.6 | 24.4 | 23.9 | 1.6 |  | +30 contract multiplier when your team wins the last trick of a round with a [2] through [10] |
| cb-three-row-mult | C | mult | 5 | 2618 | 5.3 [3.8, 6.7] | 1.6 | 3.6 | 6.7 | 61.4 | 65.1 | 2.0 |  | +10 contract multiplier when your team wins three tricks in a row |
| ca-spade-last-trick | C | mult | 4 | 2980 | 5.1 [3.8, 6.3] | 0.9 | 4.1 | 6.7 | 42.2 | 42.2 | 2.0 |  | +20 contract multiplier when your team wins the last trick of a round with [♠] |
| cc-two-win-mult | C | hybrid | 4 | 2980 | 4.9 [3.7, 6.2] | 1.2 | 3.8 | 5.7 | 25.7 | 28.0 | 2.2 |  | +20 contract multiplier when your team wins a trick with a [2] |
| ca-diamond-led-win | C | points | 3 | 2402 | 4.7 [3.2, 6.1] | 2.9 | 1.8 | 6.4 | 81.9 | 82.2 | 3.7 |  | +30 contract points when your team wins a trick led with [♦] |
| seed-exact-mult | C | mult | 2 | 5182 | 4.6 [2.4, 6.7] | 1.1 | 3.4 | 15.6 | 21.7 | 21.6 | 1.6 |  | +30 contract multiplier if your team makes its contract exactly |
| seed-rainbow-first | C | points | 4 | 2510 | 4.4 [2.9, 5.9] | 3.0 | 1.5 | 5.9 | 99.3 | 99.4 | 3.2 |  | +15 contract points when your team wins its first trick with each suit |
| ca-rainbow-lead-mult | C | hybrid | 5 | 6649 | 4.4 [3.3, 5.5] | 1.8 | 2.7 | 5.3 | 44.9 | 47.3 | 2.9 |  | +15 contract multiplier if your team leads all four suits |
| cb-consecutive-mult | C | mult | 3 | 3032 | 4.4 [3.0, 5.9] | 1.6 | 2.9 | 4.8 | 85.5 | 86.7 | 2.9 |  | +2 contract multiplier when your team wins consecutive tricks |
| seed-e-spade-win | C | points | 3 | 4485 | 4.4 [3.3, 5.5] | 2.7 | 1.7 | 5.7 | 95.5 | 95.6 | 2.3 |  | +15 contract points when your team wins a trick with [♠] |
| cc-small-contract-mult | C | hybrid | 3 | 4308 | 4.4 [2.8, 6.0] | 1.5 | 2.9 | 6.8 | 29.4 | 31.9 | 3.7 |  | +20 contract multiplier when your team bids 4 or less |
| r1-low-win-points | C | points | 5 | 9755 | 4.2 [3.2, 5.2] | 2.2 | 2.0 | 5.1 | 62.4 | 64.1 | 2.2 |  | +35 contract points when your team wins a trick with a [2] through [6] |
| cc-any-suit-first-two | C | enabler | 2 | 11470 | 4.0 [2.8, 5.2] | 0.2 | 3.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play any suit on the first two tricks |
| seed-nil-points | C | points | 1 | 5743 | 3.9 [2.3, 5.4] | 1.7 | 2.2 | 5.0 | 100.0 | 100.0 | 1.4 |  | +85 nil points |
| cc-low-hold-points | C | points | 5 | 4653 | 3.9 [2.6, 5.2] | 1.4 | 2.5 | 4.4 | 99.9 | 100.0 | 2.0 | rarely decisive | +5 contract points for each [2] through [6] your team holds |
| cb-grow-make-mult | C | mult | 4 | 1859 | 3.7 [2.3, 5.1] | 1.0 | 2.6 | 5.0 | 56.3 |  | 0.8 | flat slope | This sigil gains +4 contract multiplier every time your team makes its contract (currently +0) |
| e-win-suit-d-mult | C | mult | 3 | 2692 | 3.6 [1.6, 5.6] | 0.8 | 2.8 | 3.1 | 61.4 | 62.6 | 1.5 |  | +4 contract multiplier when your team wins a trick with [♦] |
| seed-ace-hold | C | points | 4 | 3161 | 3.5 [2.2, 4.8] | 2.1 | 1.5 | 5.5 | 97.7 | 98.4 | 2.1 |  | +10 contract points for each [A] your team holds |
| ca-diamond-lead-three | C | hybrid | 6 | 8245 | 3.3 [1.9, 4.7] | -0.6 | 3.8 | 4.2 | 27.8 | 30.1 | 1.3 |  | +15 contract multiplier when your team leads [♦] three times |
| cb-trick-mult | C | mult | 2 | 5057 | 2.9 [1.8, 3.9] | 0.4 | 2.5 | 4.2 | 99.4 | 99.5 | 2.2 |  | +1 contract multiplier when your team wins a trick |
| cb-make-x | C | xmult | 2 | 1574 | 2.8 [1.5, 4.2] | 0.5 | 2.3 | 3.3 | 59.3 |  | 2.1 |  | ×1.25 contract multiplier if your team makes its contract |
| e-last-xmult | C | xmult | 3 | 4310 | 2.8 [1.5, 4.1] | 0.8 | 2.0 | 5.5 | 48.8 | 49.9 | 2.3 |  | ×1.5 contract multiplier when your team wins the last trick of a round |
| ca-three-aces-mult | C | mult | 6 | 4252 | 2.6 [1.5, 3.7] | -0.1 | 2.7 | 4.3 | 64.9 | 74.2 | 2.2 |  | +10 contract multiplier if your team holds three [A]s |
| cc-nil-made-points | C | points | 2 | 6037 | 2.5 [1.1, 4.0] | -0.0 | 2.5 | 4.9 | 21.7 | 25.2 | 3.1 | rarely decisive | +135 contract points if your team makes a nil |
| cb-make-mult | C | mult | 2 | 2586 | 2.4 [1.1, 3.7] | -0.1 | 2.5 | 3.6 | 50.8 | 49.2 | 2.7 |  | +5 contract multiplier if your team makes its contract |
| seed-win-points | C | points | 2 | 3467 | 2.3 [1.2, 3.4] | 0.0 | 2.3 | 3.1 | 99.5 |  | 2.8 | rarely decisive | +5 contract points when your team wins a trick |
| seed-diamond-lead | C | points | 3 | 4670 | 1.9 [0.2, 3.6] | 1.4 | 0.5 | 6.4 | 82.7 | 83.3 | 1.9 |  | +20 contract points when your team leads [♦] |
| ca-ace-win-two-mult | C | mult | 7 | 2660 | 1.9 [0.6, 3.2] | -0.9 | 2.8 | 2.5 | 68.9 | 74.9 | 0.8 | flat slope | +4 contract multiplier when your team wins two tricks with [A]s |
| ca-diamond-hold-eight | C | mult | 5 | 3919 | 1.9 [0.2, 3.5] | 0.2 | 1.6 | 7.6 | 23.9 | 24.2 | 2.3 |  | +30 contract multiplier if your team holds eight [♦]s |
| seed-ace-win | C | points | 4 | 5042 | 1.8 [0.7, 3.0] | 2.0 | -0.2 | 4.6 | 89.9 | 91.7 | 4.2 | rarely decisive | +15 contract points when your team wins a trick with an [A] |
| cb-contract-mult | C | mult | 2 | 3379 | 1.7 [0.5, 2.9] | -1.2 | 3.0 | 3.3 | 100.0 | 100.0 | 1.5 | lift<band | +1 contract multiplier for each trick in your team's contract |
| seed-diamond-hold | C | points | 3 | 3669 | 1.4 [-0.4, 3.1] | 1.2 | 0.2 | 3.1 | 100.0 | 100.0 | 1.4 | rarely decisive | +5 contract points for each [♦] your team holds |
| ca-high-spade-hold | C | points | 6 | 13114 | 1.2 [0.2, 2.2] | 1.1 | 0.2 | 7.0 | 99.1 | 99.2 | 3.8 |  | +15 contract points for each [10] through [A] of [♠] your team holds |
| cb-honor-hold | C | points | 5 | 4081 | 1.2 [0.1, 2.4] | 1.2 | 0.0 | 3.8 | 100.0 | 100.0 | 2.3 | rarely decisive | +4 contract points for each [J] through [A] your team holds |
| cb-first-trick-mult | C | mult | 3 | 7040 | 1.1 [-0.1, 2.3] | -1.8 | 2.8 | 1.5 | 48.3 | 50.2 | 2.2 | lift<band | +4 contract multiplier when your team wins the first trick of a round |
| seed-nil-bid-mult | C | mult | 2 | 3402 | 0.7 [-1.1, 2.4] | -1.2 | 1.8 | 5.2 | 37.1 | 42.1 | 1.7 |  | +10 contract multiplier when your team bids nil |
| ca-face-win | C | points | 5 | 2718 | 0.5 [-0.9, 1.9] | -0.2 | 0.7 | 5.4 | 82.8 | 80.7 | 3.1 |  | +20 contract points when your team wins a trick with a [J] through [K] |
| e-lead-rank-k-mult | C | mult | 4 | 4816 | 0.4 [-0.8, 1.6] | -1.5 | 1.9 | 3.2 | 51.3 | 50.2 | 1.0 | lift<band, flat slope | +4 contract multiplier when your team leads a [K] |
| cb-bid7-mult | C | mult | 3 | 2172 | 0.3 [-1.1, 1.7] | -1.8 | 2.1 | 4.8 | 57.3 | 61.1 | 0.9 | lift<band, flat slope | +10 contract multiplier when your team bids 7 or more |
| ca-diamond-win-two | C | mult | 6 | 4213 | 0.2 [-1.5, 1.9] | -2.5 | 2.8 | 3.2 | 24.7 | 27.2 | 2.9 | lift<band | +10 contract multiplier when your team wins two tricks with [♦] |
| cb-bid8-trick-mult | C | mult | 4 | 2326 | 0.2 [-1.2, 1.5] | -3.1 | 3.3 | 5.0 | 41.3 | 44.7 | 0.6 | lift<band, flat slope | +2 contract multiplier for each trick in your team's contract when your team bids 8 or more |
| ca-king-hold | C | points | 4 | 2210 | -1.2 [-2.5, 0.0] | -0.4 | -0.8 | 4.5 | 94.1 | 93.6 | 2.0 | rarely decisive | +20 contract points for each [K] your team holds |
| seed-diamond-win | C | points | 3 | 3223 | -2.8 [-4.4, -1.1] | -0.2 | -2.5 | 4.3 | 62.1 | 64.4 | 2.6 | rarely decisive | +25 contract points when your team wins a trick with [♦] |
| control-common | C | points | 1 | 2780 | -6.0 [-6.8, -5.1] | -2.7 | -3.3 | 3.0 | 100.0 |  | 2.5 | lift<band, rarely decisive | +35 contract points |
| uc-twos-beat | U | enabler | 4 | 24906 | 18.7 [17.5, 19.8] | 9.9 | 8.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s beat every other card of their suit |
| ub-low-to-aces | U | enabler | 8 | 26039 | 13.1 [12.0, 14.3] | 5.7 | 7.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three [2]s through [9]s your team holds become [A]s |
| uc-trump-freely | U | enabler | 2 | 6266 | 13.1 [11.5, 14.8] | 5.3 | 7.9 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [♠]s even when it can follow suit |
| ub-become-four-spades | U | enabler | 5 | 6665 | 12.4 [11.0, 13.8] | 5.4 | 7.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Four cards other than [♠] your team holds become [♠]s |
| uc-low-discard | U | enabler | 4 | 8252 | 11.5 [10.3, 12.8] | 5.1 | 6.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [2]s through [5]s even when it can follow suit |
| r1-low-hold-mult | U | mult | 5 | 9404 | 10.1 [8.8, 11.3] | 5.7 | 4.4 | 17.1 | 99.8 | 99.9 | 6.3 |  | +3 contract multiplier for each [2] through [6] your team holds |
| ub-first-trick-xmult | U | hybrid | 3 | 6325 | 9.4 [8.1, 10.7] | 2.7 | 6.7 | 12.5 | 52.1 | 53.6 | 5.3 |  | ×2.5 contract multiplier when your team wins the first trick of a round |
| seed-four-row | U | mult | 5 | 4047 | 8.7 [7.4, 10.1] | 4.6 | 4.1 | 13.1 | 38.5 | 42.7 | 4.9 |  | +35 contract multiplier when your team wins four tricks in a row |
| seed-rainbow-lead-x | U | xmult | 5 | 5369 | 8.6 [7.4, 9.9] | 4.6 | 4.0 | 17.3 | 44.3 | 45.5 | 6.0 |  | ×3 contract multiplier if your team leads all four suits |
| ua-become-three-aces | U | enabler | 5 | 15212 | 8.6 [7.3, 9.9] | 4.5 | 4.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [A]s |
| ub-high-honor-hold-mult | U | mult | 5 | 5564 | 8.4 [7.3, 9.5] | 4.8 | 3.6 | 15.0 | 100.0 | 100.0 | 4.9 |  | +3 contract multiplier for each [J] through [A] your team holds |
| seed-nil-made-x | U | xmult | 2 | 6701 | 8.1 [6.7, 9.5] | 5.9 | 2.2 | 16.0 | 25.6 | 29.4 | 5.7 |  | ×2.5 contract multiplier if your team makes a nil |
| uc-spade-lead-mult | U | hybrid | 3 | 6136 | 7.0 [5.8, 8.1] | 3.7 | 3.3 | 14.2 | 79.0 | 79.4 | 5.2 |  | +10 contract multiplier when your team leads [♠] |
| ub-bid5-xmult | U | hybrid | 3 | 6859 | 6.7 [5.4, 8.0] | 2.8 | 3.9 | 14.9 | 47.1 | 49.6 | 4.8 |  | ×2.5 contract multiplier when your team bids 5 or less |
| seed-flat-mult | U | mult | 1 | 3763 | 6.6 [5.2, 8.0] | 3.1 | 3.5 | 12.2 | 100.0 |  | 4.9 |  | +20 contract multiplier |
| ua-three-aces-x | U | xmult | 6 | 6663 | 6.4 [5.3, 7.5] | 2.4 | 4.0 | 13.9 | 72.5 | 81.2 | 4.4 |  | ×2.5 contract multiplier if your team holds three [A]s |
| uc-nil-low-hold | U | points | 5 | 2931 | 6.2 [4.8, 7.7] | 5.1 | 1.2 | 15.1 | 99.9 | 100.0 | 4.9 |  | +30 nil points for each [2] through [6] your team holds |
| ua-diamonds-untrumpable | U | enabler | 3 | 2782 | 5.7 [4.0, 7.5] | 1.6 | 4.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [♦]s can't be trumped |
| ua-ace-hold-mult | U | mult | 4 | 6321 | 5.5 [4.4, 6.7] | 2.4 | 3.2 | 12.1 | 97.8 | 98.1 | 3.2 |  | +5 contract multiplier for each [A] your team holds |
| ua-rainbow-first-mult | U | mult | 4 | 5877 | 5.4 [4.2, 6.7] | 1.6 | 3.9 | 7.6 | 99.4 | 99.3 | 4.1 |  | +4 contract multiplier when your team wins its first trick with each suit |
| ub-make8-trick-mult | U | mult | 5 | 4807 | 5.1 [3.8, 6.5] | 2.3 | 2.8 | 10.8 | 18.6 | 19.6 | 6.5 |  | +5 contract multiplier for each trick in your team's contract if your team makes a contract of 8 or more |
| ua-diamond-hold-mult | U | mult | 3 | 4966 | 5.1 [3.4, 6.9] | 1.8 | 3.3 | 10.6 | 100.0 | 100.0 | 3.6 |  | +3 contract multiplier for each [♦] your team holds |
| ub-consecutive-contract-points | U | hybrid | 4 | 15661 | 5.1 [4.1, 6.1] | 2.4 | 2.7 | 10.6 | 87.6 | 88.3 | 4.7 |  | +3 contract points for each trick in your team's contract when your team wins consecutive tricks |
| ua-spade-raise | U | enabler | 5 | 34620 | 4.5 [3.3, 5.6] | 0.2 | 4.3 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every [♠] your team holds by two ranks |
| ub-four-row-points | U | points | 5 | 4904 | 4.3 [3.0, 5.6] | 1.8 | 2.5 | 8.8 | 40.2 | 44.4 | 4.9 |  | +125 contract points when your team wins four tricks in a row |
| seed-spade-hold | U | points | 3 | 5047 | 3.9 [2.5, 5.3] | 4.5 | -0.6 | 13.9 | 100.0 | 100.0 | 6.6 |  | +15 contract points for each [♠] your team holds |
| uc-low-lead-mult | U | hybrid | 5 | 3246 | 3.7 [2.1, 5.2] | 0.8 | 2.9 | 9.8 | 88.3 | 89.2 | 3.7 |  | +5 contract multiplier when your team leads a [2] through [6] |
| ub-first-trick-contract-points | U | hybrid | 4 | 5733 | 3.6 [2.4, 4.7] | 1.5 | 2.1 | 8.6 | 52.4 | 53.0 | 3.7 |  | +20 contract points for each trick in your team's contract when your team wins the first trick of a round |
| ua-diamond-honor-win | U | points | 6 | 5725 | 3.5 [2.4, 4.7] | 0.8 | 2.7 | 7.7 | 58.5 | 59.7 | 5.0 |  | +70 contract points when your team wins a trick with a [J] through [A] of [♦] |
| ua-side-raise | U | enabler | 5 | 44161 | 3.3 [2.1, 4.5] | 0.1 | 3.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card other than [♠] your team holds by two ranks |
| ua-spade-seven-x | U | xmult | 5 | 4111 | 3.2 [2.0, 4.4] | 1.7 | 1.5 | 12.8 | 56.3 | 56.5 | 3.6 |  | ×2.5 contract multiplier if your team holds seven [♠]s |
| uc-low-win-mult | U | hybrid | 5 | 13459 | 2.8 [1.7, 3.9] | 1.6 | 1.2 | 12.9 | 71.6 | 74.2 | 4.9 |  | +10 contract multiplier when your team wins a trick with a [2] through [9] |
| ua-high-spade-win | U | points | 6 | 15762 | 2.8 [1.7, 3.8] | 2.8 | -0.0 | 12.7 | 89.7 | 90.0 | 5.8 |  | +45 contract points when your team wins a trick with a [J] through [A] of [♠] |
| uc-nil-cover-points | U | hybrid | 2 | 10508 | 2.5 [1.2, 3.8] | 2.1 | 0.4 | 11.5 | 99.1 | 99.2 | 4.6 |  | +35 nil points when your team wins a trick |
| ub-make7-xmult | U | xmult | 4 | 4706 | 2.1 [0.9, 3.4] | 0.7 | 1.5 | 9.7 | 26.3 | 27.5 | 4.7 |  | ×2 contract multiplier if your team makes a contract of 7 or more |
| r1-exact-points | U | points | 2 | 7664 | 0.9 [-1.0, 2.9] | 2.0 | -1.1 | 26.7 | 22.3 | 23.2 | 4.7 |  | +400 contract points if your team makes its contract exactly |
| ub-bid7-points | U | points | 3 | 3928 | -0.0 [-1.4, 1.3] | -0.6 | 0.6 | 8.0 | 64.1 | 67.0 | 2.7 |  | +140 contract points when your team bids 7 or more |
| ua-rainbow-first-contract | U | points | 5 | 5715 | -0.2 [-1.3, 0.9] | 0.2 | -0.3 | 9.4 | 99.3 | 99.4 | 3.3 |  | +4 contract points for each trick in your team's contract when your team wins its first trick with each suit |
| ub-bid8-trick-points | U | points | 4 | 4169 | -0.9 [-2.2, 0.3] | 0.6 | -1.5 | 11.1 | 49.9 | 52.9 | 3.8 |  | +35 contract points for each trick in your team's contract when your team bids 8 or more |
| r1-exact-xmult | U | xmult | 2 | 7495 | -1.5 [-3.5, 0.4] | -3.1 | 1.6 | 17.5 | 21.7 | 22.0 | 1.5 | lift<band | ×2.5 contract multiplier if your team makes its contract exactly |
| control-uncommon | U | points | 1 | 2617 | -3.2 [-4.2, -2.3] | -1.2 | -2.0 | 6.2 | 100.0 |  | 4.1 | lift<band | +50 contract points |
| ua-diamond-two-x | U | xmult | 6 | 3670 | -3.3 [-5.2, -1.5] | -3.3 | -0.0 | 8.0 | 25.5 | 27.9 | 2.2 | lift<band | ×2 contract multiplier when your team wins two tricks with [♦] |
| ra-jq-become-aces | R | enabler | 7 | 5907 | 19.1 [17.6, 20.6] | 10.1 | 9.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Every [J] through [Q] your team holds becomes an [A] |
| r1-middle-free | R | enabler | 4 | 14225 | 18.6 [17.4, 19.9] | 8.9 | 9.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [6]s through [9]s even when it can follow suit |
| ra-spade-lead-x | R | hybrid | 3 | 3402 | 14.7 [13.4, 16.0] | 8.5 | 6.2 | 22.2 | 78.3 | 78.8 | 8.0 |  | ×1.5 contract multiplier when your team leads [♠] |
| rb-honors-free | R | enabler | 4 | 5710 | 12.4 [10.8, 14.0] | 4.3 | 8.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [10]s through [A]s even when it can follow suit |
| seed-raise | R | enabler | 4 | 97498 | 11.9 [10.9, 13.0] | 5.5 | 6.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by two ranks |
| seed-flat-x | R | xmult | 1 | 2137 | 9.4 [7.9, 11.0] | 5.9 | 3.5 | 18.0 | 100.0 |  | 5.6 |  | ×2.5 contract multiplier |
| seed-ace-mult | R | mult | 4 | 4348 | 8.7 [7.4, 9.9] | 4.5 | 4.1 | 15.7 | 88.6 | 91.0 | 5.1 |  | +10 contract multiplier when your team wins a trick with an [A] |
| ra-diamond-hold-x | R | xmult | 3 | 4449 | 8.6 [6.9, 10.3] | 5.5 | 3.1 | 17.7 | 100.0 | 100.0 | 9.6 |  | ×1.15 contract multiplier for each [♦] your team holds |
| rb-streak-contract-mult | R | mult | 4 | 3560 | 7.3 [6.2, 8.5] | 4.3 | 3.0 | 15.0 | 84.2 | 84.7 | 5.1 |  | +1 contract multiplier for each trick in your team's contract when your team wins consecutive tricks |
| ra-diamond-led-win-x | R | hybrid | 3 | 3067 | 6.9 [5.4, 8.4] | 3.0 | 3.9 | 13.8 | 80.7 | 80.9 | 7.0 |  | ×1.5 contract multiplier when your team wins a trick led with [♦] |
| rb-opp-win-mult | R | mult | 2 | 4504 | 6.5 [5.1, 7.8] | 3.1 | 3.4 | 17.6 | 99.6 | 99.5 | 2.8 |  | +4 contract multiplier when the opponents win a trick |
| ra-trump-x | R | xmult | 3 | 3957 | 5.8 [4.6, 7.1] | 4.7 | 1.2 | 21.7 | 81.5 | 82.1 | 6.8 |  | ×1.5 contract multiplier when your team wins a trick by trumping |
| ra-ace-hold-x | R | xmult | 4 | 41846 | 3.7 [2.8, 4.7] | 2.1 | 1.7 | 14.9 | 98.6 | 98.9 | 5.9 |  | ×1.15 contract multiplier for each [A] your team holds |
| seed-rainbow-x | R | xmult | 5 | 4221 | 3.5 [2.0, 4.9] | 2.3 | 1.2 | 14.5 | 27.9 | 31.8 | 4.8 |  | ×3.5 contract multiplier if your team wins tricks with all four suits |
| rc-low-lead-x | R | xmult | 5 | 13594 | 2.3 [1.2, 3.4] | 1.1 | 1.3 | 15.8 | 82.0 | 81.8 | 6.8 |  | ×1.25 contract multiplier when your team leads a [2] through [6] |
| r1-spade-four-x | R | xmult | 6 | 7859 | 1.9 [0.9, 3.0] | 0.4 | 1.6 | 16.2 | 43.4 | 43.8 | 3.9 |  | ×2.5 contract multiplier when your team wins four tricks with [♠] |
| ra-rainbow-first-x | R | xmult | 4 | 4678 | 1.1 [0.0, 2.2] | 0.6 | 0.5 | 11.3 | 99.2 | 99.3 | 4.2 |  | ×1.2 contract multiplier when your team wins its first trick with each suit |
| rb-outbid-x | R | xmult | 2 | 3262 | -0.8 [-2.2, 0.5] | -1.8 | 1.0 | 12.7 | 42.0 | 44.1 | 2.5 | lift<band | ×3 contract multiplier when your team bids more than the opponents |
| control-rare | R | points | 1 | 2645 | -2.6 [-3.5, -1.7] | -1.5 | -1.2 | 9.4 | 100.0 |  | 3.2 | lift<band | +65 contract points |
| rc-low-win-contract-points | R | points | 6 | 23186 | -2.7 [-3.7, -1.6] | 1.6 | -4.3 | 18.6 | 74.5 | 74.5 | 5.6 |  | +10 contract points for each trick in your team's contract when your team wins a trick with a [2] through [9] |
| seed-two-win | R | points | 4 | 1898 | -3.0 [-5.1, -0.9] | 0.5 | -3.5 | 7.9 | 28.9 | 36.4 | 2.5 |  | +130 contract points when your team wins a trick with a [2] |
| ra-side-win-points | R | points | 3 | 4443 | -3.3 [-4.4, -2.2] | 0.2 | -3.6 | 9.8 | 90.3 | 90.7 | 5.5 |  | +25 contract points when your team wins a trick with a card other than [♠] |
| ra-ace-win-x | L | xmult | 4 | 32985 | 17.4 [16.4, 18.3] | 14.8 | 2.6 | 32.0 | 94.1 | 94.2 | 12.5 | lift>ceiling | ×1.5 contract multiplier when your team wins a trick with an [A] |
| r1-free-discards | L | enabler | 2 | 5323 | 9.8 [8.7, 10.8] | 2.1 | 7.6 | 0.0 | 0.0 |  | 0.0 |  | Your team can play cards other than [♠] even when it can follow suit |
| r1-faces-untrumpable | L | enabler | 5 | 14234 | 4.6 [3.5, 5.7] | 1.3 | 3.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [Q]s through [A]s can't be trumped |
| rc-low-win-x | L | xmult | 5 | 33112 | 2.5 [1.5, 3.5] | 0.1 | 2.4 | 17.3 | 59.2 | 59.3 | 6.4 |  | ×1.5 contract multiplier when your team wins a trick with a [2] through [6] |
| rb-contract-trick-x | L | xmult | 2 | 3621 | 1.8 [0.5, 3.2] | 0.1 | 1.7 | 15.2 | 100.0 | 100.0 | 6.5 |  | ×1.15 contract multiplier for each trick in your team's contract |
| control-legendary | L | points | 1 | 2669 | -5.3 [-6.2, -4.4] | -2.1 | -3.2 | 12.7 | 100.0 |  | 4.6 | lift<band | +100 contract points |
