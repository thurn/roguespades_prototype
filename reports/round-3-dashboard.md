# round-3: dashboard

Fit: 120937 boards, alpha 20, residual sd 0.315 (u units), win calibration k = 1.70, smoothing scale 25128 points.
Tier agreement: 133 sigils, lift correlation 0.30 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 4295040 rounds.

**Fun score: 51.5** (90% interval 50.6–51.8)

| Family | Weight | Score | Sub-metrics |
| --- | --- | --- | --- |
| Close and live | 25 | 0.66 | median margin / winner 0.72; trailer after round 5 wins 0.25; rounds 1-4 share 0.33 |
| Commitment works | 15 | 0.30 | online by round 4 0.30; committed win rate 0.44 |
| Archetypes viable | 15 | 1.00 | archetypes with committed win 40-60% 1.00; largest archetype share of winning flexible builds 0.22 |
| Synergy and combos | 15 | 0.00 | same-archetype pair gain over parts 0.00; strong pairs (>=50% over parts) 0; strong cross-archetype pairs 0 |
| Skill and bidding | 15 | 0.80 | stronger tier beats tier 0 (boards) 0.82; late overtricks per made 0.95; set rate 0.34 |
| Simplicity | 15 | 0.23 | mean S 0.23 |

Commitment: {"per_archetype": {"Suits": 0.425, "Spades": 0.458125, "Ranks": 0.4475, "BidHigh": 0.49125, "Nil": 0.41625, "LowCards": 0.41875, "Rainbow": 0.43, "Streaks": 0.42875, "Exact": 0.44625}, "committed_win": 0.4402083333333333, "online": 0.29541666666666666, "online_by_arch": {"Suits": 0.2125, "Spades": 0.60375, "Ranks": 0.15875, "BidHigh": 0.45, "Nil": 0.39625, "LowCards": 0.26625, "Rainbow": 0.13375, "Streaks": 0.40125, "Exact": 0.03625}, "winning_share": {"Suits": 0.016666666666666666, "Spades": 0.14533333333333334, "Ranks": 0.050666666666666665, "BidHigh": 0.13133333333333333, "Nil": 0.16533333333333333, "LowCards": 0.12133333333333333, "Rainbow": 0.043333333333333335, "Streaks": 0.216, "Exact": 0.0013333333333333333}}

Ladder (tier 1 vs tier 0 with the pool, boards): 0.823

Global factor 1.00

Build chasers:

- 0.517 [0.488, 0.545] (840 runs): ca-king-hold
- 0.514 [0.486, 0.543] (840 runs): r2-four-row-x
- 0.506 [0.478, 0.534] (840 runs): r2-queens-beat
- 0.500 [0.457, 0.543] (360 runs): uc-low-discard
- 0.500 [0.457, 0.543] (360 runs): cb-become-spade
- 0.492 [0.417, 0.567] (120 runs): r2-diamond-honor-hold
- 0.492 [0.417, 0.567] (120 runs): ua-high-spade-win
- 0.475 [0.400, 0.550] (120 runs): seed-spade-hold
- 0.467 [0.392, 0.542] (120 runs): ca-face-win
- 0.467 [0.392, 0.542] (120 runs): r2-opp-win-points
| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| cb-become-spade | C | enabler | 6 | 10291 | 7.1 [6.0, 8.2] | 2.9 | 4.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [♠]s |
| ca-king-hold | C | points | 4 | 3070 | 3.5 [2.4, 4.7] | 2.5 | 1.1 | 10.3 | 93.5 | 92.4 | 4.1 |  | +25 contract points for each [K] your team holds |
| cb-become-ace | C | enabler | 7 | 5511 | 2.6 [1.4, 3.8] | 0.2 | 2.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two cards your team holds become [A]s |
| cc-any-suit-first-two | C | enabler | 2 | 6281 | 1.7 [0.4, 2.9] | -0.4 | 2.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play any suit on the first two tricks |
| seed-diamond-lead | C | points | 3 | 4331 | 1.2 [-0.2, 2.5] | 0.2 | 1.0 | 8.8 | 82.3 | 83.8 | 4.2 |  | +15 contract points when your team leads [♦] |
| seed-diamond-win | C | points | 3 | 4164 | 1.2 [-0.2, 2.5] | 0.4 | 0.8 | 10.3 | 59.7 | 65.7 | 4.1 |  | +30 contract points when your team wins a trick with [♦] |
| ca-high-spade-hold | C | points | 6 | 13471 | 1.1 [0.3, 1.9] | 0.4 | 0.7 | 7.3 | 99.2 | 99.2 | 3.3 |  | +10 contract points for each [10] through [A] of [♠] your team holds |
| seed-last-trick | C | mult | 3 | 3637 | 0.8 [-0.4, 2.0] | 1.1 | -0.3 | 8.1 | 48.6 | 48.4 | 3.1 |  | +15 contract multiplier when your team wins the last trick of a round |
| ca-face-win | C | points | 5 | 3722 | 0.5 [-0.5, 1.5] | 1.3 | -0.8 | 8.1 | 88.8 | 86.8 | 4.5 |  | +20 contract points when your team wins a trick with a [J] through [K] |
| cb-honor-hold | C | points | 5 | 3220 | 0.5 [-0.7, 1.7] | 0.1 | 0.4 | 5.8 | 100.0 | 100.0 | 3.8 |  | +3 contract points for each [J] through [A] your team holds |
| r1-low-win-points | C | points | 5 | 13824 | -0.0 [-0.8, 0.8] | -0.6 | 0.6 | 6.7 | 61.3 | 62.3 | 2.9 |  | +20 contract points when your team wins a trick with a [2] through [6] |
| cb-opp-set-mult | C | mult | 2 | 4151 | -0.1 [-1.1, 0.8] | 0.0 | -0.2 | 5.7 | 41.1 |  | 0.8 | flat slope | +15 contract multiplier if the opponents miss their contract |
| control-common | C | points | 1 | 2981 | -0.2 [-1.0, 0.5] | -0.3 | 0.0 | 8.0 | 100.0 |  | 2.6 |  | +25 contract points |
| seed-rainbow-first | C | points | 4 | 4060 | -0.3 [-1.6, 1.0] | -0.2 | -0.2 | 6.5 | 99.1 | 99.2 | 2.8 |  | +10 contract points when your team wins its first trick with each suit |
| seed-ace-win | C | points | 4 | 2668 | -0.5 [-1.7, 0.7] | -0.2 | -0.3 | 5.1 | 88.3 | 92.4 | 3.3 |  | +10 contract points when your team wins a trick with an [A] |
| e-hold-suit-s-mult | C | mult | 3 | 2743 | -0.9 [-2.0, 0.3] | -0.7 | -0.2 | 5.9 | 100.0 | 100.0 | 0.8 | flat slope | +1 contract multiplier for each [♠] your team holds |
| ca-diamond-led-win | C | points | 3 | 5107 | -0.9 [-1.8, -0.0] | -0.7 | -0.2 | 5.0 | 80.5 | 80.6 | 4.2 |  | +15 contract points when your team wins a trick led with [♦] |
| ca-spade-lead-three | C | hybrid | 6 | 5342 | -1.0 [-2.1, -0.0] | -1.2 | 0.2 | 5.4 | 33.9 | 35.0 | 1.3 | lift<band | +10 contract multiplier when your team leads [♠] three times |
| cb-consecutive-mult | C | mult | 3 | 3964 | -1.5 [-2.7, -0.3] | -1.0 | -0.5 | 3.3 | 83.8 | 84.4 | 1.7 |  | +1 contract multiplier when your team wins consecutive tricks |
| cb-trick-mult | C | mult | 2 | 2700 | -1.5 [-2.8, -0.3] | 0.1 | -1.6 | 5.3 | 99.2 | 99.3 | 3.3 |  | +1 contract multiplier when your team wins a trick |
| seed-win-points | C | points | 2 | 2237 | -1.7 [-2.9, -0.5] | -1.2 | -0.4 | 2.9 | 99.3 |  | 3.1 | lift<band, rarely decisive | +3 contract points when your team wins a trick |
| seed-nil-points | C | points | 1 | 3944 | -1.8 [-3.2, -0.4] | -0.3 | -1.5 | 3.5 | 100.0 | 100.0 | 2.6 | rarely decisive | +40 nil points |
| seed-trump-points | C | points | 3 | 4927 | -2.0 [-3.1, -1.0] | -0.8 | -1.3 | 4.2 | 82.5 | 83.7 | 1.7 | rarely decisive | +10 contract points when your team wins a trick by trumping |
| cc-small-contract-mult | C | hybrid | 3 | 17155 | -2.0 [-3.0, -1.1] | -2.0 | -0.1 | 5.6 | 32.8 | 35.0 | 2.3 | lift<band | +10 contract multiplier when your team bids 4 or less |
| seed-e-spade-win | C | points | 3 | 3007 | -2.2 [-3.2, -1.1] | -0.8 | -1.3 | 3.1 | 95.4 | 95.5 | 1.4 | rarely decisive | +5 contract points when your team wins a trick with [♠] |
| cc-nil-made-points | C | points | 2 | 3463 | -2.4 [-3.7, -1.1] | -1.1 | -1.3 | 5.2 | 25.2 | 29.1 | 1.7 |  | +90 contract points if your team makes a nil |
| cb-spade-seven | C | mult | 5 | 5003 | -2.5 [-3.5, -1.5] | -1.1 | -1.4 | 5.0 | 60.7 | 61.1 | 1.8 | lift<band | +10 contract multiplier if your team holds seven [♠]s |
| ca-spade-last-trick | C | mult | 4 | 4068 | -2.6 [-3.7, -1.5] | -1.6 | -1.0 | 5.4 | 41.9 | 41.9 | 1.2 | lift<band, flat slope | +10 contract multiplier when your team wins the last trick of a round with [♠] |
| cc-nil-made-mult | C | mult | 2 | 3011 | -2.7 [-4.3, -1.1] | -1.4 | -1.3 | 6.9 | 26.2 | 30.3 | 3.5 |  | +10 contract multiplier if your team makes a nil |
| cb-make8-mult | C | mult | 4 | 2762 | -2.9 [-4.0, -1.7] | -1.0 | -1.9 | 4.3 | 16.9 | 19.4 | 1.6 |  | +10 contract multiplier if your team makes a contract of 8 or more |
| cc-low-last-mult | C | hybrid | 6 | 4869 | -2.9 [-3.9, -2.0] | -2.7 | -0.2 | 4.5 | 28.0 | 27.9 | 2.0 | lift<band | +10 contract multiplier when your team wins the last trick of a round with a [2] through [10] |
| seed-rainbow-mult | C | mult | 5 | 4524 | -3.4 [-4.5, -2.2] | -2.0 | -1.4 | 3.5 | 27.4 | 33.3 | 2.7 | lift<band | +10 contract multiplier if your team wins tricks with all four suits |
| cb-contract-mult | C | mult | 2 | 3266 | -3.4 [-4.6, -2.2] | -1.8 | -1.5 | 3.9 | 100.0 | 100.0 | 2.4 | lift<band | +1 contract multiplier for each trick in your team's contract |
| e-trump-mult | C | mult | 3 | 3476 | -3.4 [-4.6, -2.2] | -1.7 | -1.7 | 3.0 | 82.1 | 83.5 | 2.5 | lift<band | +2 contract multiplier when your team wins a trick by trumping |
| cb-grow-make-mult | C | mult | 4 | 3457 | -3.5 [-4.6, -2.4] | -0.8 | -2.7 | 6.0 | 59.5 |  | 0.8 | flat slope | This sigil gains +3 contract multiplier every time your team makes its contract (currently +0) |
| seed-nil-bid-mult | C | mult | 2 | 5176 | -3.7 [-5.0, -2.3] | -2.3 | -1.4 | 3.9 | 41.2 | 45.6 | 1.1 | lift<band | +5 contract multiplier when your team bids nil |
| cc-two-hold-mult | C | mult | 4 | 5749 | -3.8 [-4.8, -2.9] | -2.2 | -1.7 | 3.9 | 82.2 | 84.6 | 1.9 | lift<band | +3 contract multiplier for each [2] your team holds |
| e-win-suit-d-mult | C | mult | 3 | 6095 | -3.9 [-5.0, -2.7] | -1.4 | -2.5 | 2.8 | 60.2 | 65.8 | 1.6 | lift<band | +2 contract multiplier when your team wins a trick with [♦] |
| cc-low-hold-points | C | points | 5 | 4731 | -4.0 [-5.0, -3.0] | -2.7 | -1.3 | 3.3 | 100.0 | 100.0 | 0.5 | lift<band, rarely decisive, flat slope | +2 contract points for each [2] through [6] your team holds |
| ca-diamond-hold-eight | C | mult | 5 | 4166 | -4.0 [-5.3, -2.8] | -2.2 | -1.8 | 6.5 | 22.1 | 23.0 | 2.5 | lift<band | +20 contract multiplier if your team holds eight [♦]s |
| seed-exact-mult | C | mult | 2 | 9472 | -4.1 [-5.3, -3.0] | -3.8 | -0.3 | 8.4 | 23.3 | 23.8 | 1.2 | lift<band | +10 contract multiplier if your team makes its contract exactly |
| cc-two-win-mult | C | hybrid | 4 | 14932 | -4.1 [-4.9, -3.3] | -2.2 | -2.0 | 4.1 | 22.0 | 23.0 | 1.7 | lift<band | +10 contract multiplier when your team wins a trick with a [2] |
| ca-diamond-lead-three | C | hybrid | 6 | 2996 | -4.2 [-5.6, -2.9] | -3.2 | -1.0 | 1.3 | 24.0 | 27.5 | 0.9 | lift<band, flat slope | +5 contract multiplier when your team leads [♦] three times |
| seed-diamond-hold | C | points | 3 | 4794 | -4.6 [-5.7, -3.4] | -2.4 | -2.2 | 7.1 | 100.0 | 100.0 | 3.1 | lift<band | +4 contract points for each [♦] your team holds |
| seed-contract-points | C | points | 2 | 2537 | -4.7 [-5.9, -3.4] | -2.1 | -2.5 | 3.5 | 100.0 | 100.0 | 2.2 | lift<band, rarely decisive | +3 contract points for each trick in your team's contract |
| cb-first-trick-mult | C | mult | 3 | 3753 | -4.9 [-6.1, -3.7] | -3.1 | -1.8 | 1.8 | 47.2 | 48.2 | -0.0 | lift<band, flat slope | +3 contract multiplier when your team wins the first trick of a round |
| seed-ace-hold | C | points | 4 | 3165 | -4.9 [-6.1, -3.8] | -2.4 | -2.5 | 2.4 | 97.2 | 98.2 | 0.4 | lift<band, rarely decisive, flat slope | +4 contract points for each [A] your team holds |
| cb-three-row-mult | C | mult | 5 | 3800 | -5.5 [-6.6, -4.4] | -3.1 | -2.4 | 2.5 | 58.5 | 59.5 | 1.9 | lift<band | +3 contract multiplier when your team wins three tricks in a row |
| cc-nil-contract-trick | C | hybrid | 2 | 2659 | -5.5 [-6.6, -4.4] | -3.3 | -2.3 | 2.2 | 100.0 | 100.0 | 1.2 | lift<band, flat slope | +5 nil points for each trick in your team's contract |
| ca-three-aces-mult | C | mult | 6 | 2543 | -5.6 [-6.8, -4.3] | -3.2 | -2.4 | 2.4 | 63.2 | 75.6 | 2.6 | lift<band | +5 contract multiplier if your team holds three [A]s |
| ca-rainbow-lead-mult | C | hybrid | 5 | 2284 | -5.6 [-7.3, -4.0] | -3.7 | -1.9 | 2.4 | 42.4 | 47.2 | 0.4 | lift<band, flat slope | +5 contract multiplier if your team leads all four suits |
| cb-make-mult | C | mult | 2 | 4067 | -5.9 [-7.0, -4.9] | -3.0 | -3.0 | 3.6 | 54.9 | 53.4 | 0.6 | lift<band, flat slope | +3 contract multiplier if your team makes its contract |
| e-lead-rank-k-mult | C | mult | 4 | 2992 | -6.0 [-7.1, -4.8] | -3.4 | -2.6 | 2.6 | 51.0 | 50.6 | 1.9 | lift<band | +3 contract multiplier when your team leads a [K] |
| cc-nil-bid-xmult | C | xmult | 2 | 3176 | -6.0 [-7.6, -4.5] | -1.9 | -4.1 | 2.5 | 40.3 | 45.1 | 0.5 | lift<band, flat slope | ×1.25 contract multiplier when your team bids nil |
| cb-bid7-mult | C | mult | 3 | 3204 | -7.2 [-8.5, -6.0] | -4.4 | -2.9 | 3.3 | 51.2 | 57.0 | 0.6 | lift<band, flat slope | +5 contract multiplier when your team bids 7 or more |
| cb-make-x | C | xmult | 2 | 3145 | -7.3 [-8.3, -6.2] | -1.9 | -5.4 | 3.0 | 54.2 |  | 0.9 | lift<band, flat slope | ×1.15 contract multiplier if your team makes its contract |
| ca-ace-win-two-mult | C | mult | 7 | 3406 | -7.3 [-8.4, -6.2] | -3.7 | -3.6 | 2.6 | 59.3 | 72.1 | 2.0 | lift<band | +3 contract multiplier when your team wins two tricks with [A]s |
| ca-diamond-win-two | C | mult | 6 | 4506 | -7.7 [-9.0, -6.4] | -4.6 | -3.1 | 2.7 | 25.2 | 32.3 | 1.2 | lift<band | +5 contract multiplier when your team wins two tricks with [♦] |
| cb-bid8-trick-mult | C | mult | 4 | 2874 | -8.1 [-9.4, -6.9] | -5.3 | -2.9 | 3.8 | 35.5 | 40.3 | 1.7 | lift<band | +1 contract multiplier for each trick in your team's contract when your team bids 8 or more |
| e-last-xmult | C | xmult | 3 | 3193 | -8.7 [-9.9, -7.6] | -2.7 | -6.1 | 3.0 | 48.2 | 48.4 | 2.4 | lift<band | ×1.2 contract multiplier when your team wins the last trick of a round |
| r2-four-row-x | U | xmult | 5 | 8577 | 15.0 [14.0, 16.0] | 8.5 | 6.5 | 21.6 | 39.1 | 40.2 | 7.2 |  | ×3 contract multiplier when your team wins four tricks in a row |
| r2-queens-beat | U | enabler | 4 | 14783 | 12.9 [11.8, 14.0] | 6.6 | 6.3 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [Q]s beat every other card of their suit |
| uc-low-discard | U | enabler | 4 | 16517 | 12.6 [11.5, 13.6] | 7.1 | 5.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [2]s through [5]s even when it can follow suit |
| r2-diamond-honor-hold | U | points | 6 | 8553 | 10.1 [9.1, 11.1] | 8.6 | 1.5 | 21.2 | 95.2 | 96.0 | 7.8 |  | +40 contract points for each [J] through [A] of [♦] your team holds |
| ub-bid8-trick-points | U | points | 4 | 2518 | 9.3 [7.9, 10.7] | 7.7 | 1.6 | 19.7 | 48.2 | 53.8 | 5.8 |  | +35 contract points for each trick in your team's contract when your team bids 8 or more |
| uc-trump-freely | U | enabler | 2 | 22393 | 8.9 [7.8, 9.9] | 2.5 | 6.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [♠]s even when it can follow suit |
| ub-become-four-spades | U | enabler | 5 | 18217 | 8.2 [7.0, 9.3] | 1.8 | 6.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Four cards other than [♠] your team holds become [♠]s |
| r2-opp-win-points | U | points | 2 | 13066 | 7.2 [6.2, 8.1] | 7.2 | -0.0 | 23.1 | 99.4 | 99.4 | 7.6 |  | +15 contract points when the opponents win a trick |
| ub-low-to-aces | U | enabler | 8 | 5623 | 7.0 [5.7, 8.3] | 4.1 | 3.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three [2]s through [9]s your team holds become [A]s |
| ub-bid7-points | U | points | 3 | 2841 | 6.9 [5.6, 8.2] | 4.7 | 2.2 | 14.8 | 62.2 | 67.5 | 5.1 |  | +145 contract points when your team bids 7 or more |
| ua-spade-seven-x | U | xmult | 5 | 3645 | 6.8 [5.6, 8.0] | 2.0 | 4.8 | 11.4 | 60.7 | 61.2 | 5.4 |  | ×2 contract multiplier if your team holds seven [♠]s |
| ua-rainbow-first-contract | U | points | 5 | 3114 | 6.5 [5.2, 7.7] | 5.9 | 0.5 | 12.6 | 99.2 | 99.3 | 5.7 |  | +4 contract points for each trick in your team's contract when your team wins its first trick with each suit |
| r2-diamond-lead-mult | U | mult | 3 | 5212 | 6.2 [4.8, 7.7] | 5.1 | 1.1 | 13.6 | 81.4 | 83.9 | 4.3 |  | +6 contract multiplier when your team leads [♦] |
| seed-spade-hold | U | points | 3 | 14096 | 6.2 [5.2, 7.1] | 6.1 | 0.0 | 18.0 | 100.0 | 100.0 | 7.2 |  | +10 contract points for each [♠] your team holds |
| r1-exact-points | U | points | 2 | 8024 | 6.0 [4.6, 7.3] | 5.0 | 0.9 | 34.3 | 23.5 | 23.4 | 8.2 |  | +325 contract points if your team makes its contract exactly |
| ua-high-spade-win | U | points | 6 | 13673 | 5.4 [4.4, 6.3] | 5.6 | -0.3 | 13.7 | 90.0 | 90.3 | 7.3 |  | +30 contract points when your team wins a trick with a [J] through [A] of [♠] |
| ua-spade-raise | U | enabler | 5 | 4536 | 4.6 [3.1, 6.0] | 2.7 | 1.9 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every [♠] your team holds by two ranks |
| uc-nil-low-hold | U | points | 5 | 6022 | 4.3 [3.2, 5.4] | 4.2 | 0.1 | 15.2 | 100.0 | 100.0 | 5.4 |  | +15 nil points for each [2] through [6] your team holds |
| ub-bid5-xmult | U | hybrid | 3 | 15227 | 4.0 [2.9, 5.1] | 3.4 | 0.6 | 12.4 | 52.6 | 54.8 | 5.2 |  | ×2 contract multiplier when your team bids 5 or less |
| ua-diamond-hold-mult | U | mult | 3 | 4678 | 3.1 [1.5, 4.7] | 4.1 | -1.0 | 10.9 | 100.0 | 100.0 | 4.3 |  | +2 contract multiplier for each [♦] your team holds |
| ub-consecutive-contract-points | U | hybrid | 4 | 2590 | 2.9 [1.6, 4.1] | 3.5 | -0.6 | 11.4 | 85.3 | 86.1 | 5.6 |  | +2 contract points for each trick in your team's contract when your team wins consecutive tricks |
| r1-exact-xmult | U | xmult | 2 | 7483 | 2.4 [0.8, 3.9] | 0.3 | 2.1 | 23.1 | 22.2 | 22.6 | 4.5 |  | ×3 contract multiplier if your team makes its contract exactly |
| ua-diamonds-untrumpable | U | enabler | 3 | 7811 | 2.3 [1.2, 3.5] | 1.8 | 0.6 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [♦]s can't be trumped |
| ub-first-trick-contract-points | U | hybrid | 4 | 3563 | 2.0 [0.9, 3.1] | 3.0 | -0.9 | 10.6 | 52.5 | 53.2 | 4.8 |  | +15 contract points for each trick in your team's contract when your team wins the first trick of a round |
| ua-diamond-honor-win | U | points | 6 | 3653 | 2.0 [0.8, 3.1] | 2.4 | -0.4 | 8.7 | 57.6 | 60.0 | 5.3 |  | +50 contract points when your team wins a trick with a [J] through [A] of [♦] |
| uc-nil-cover-points | U | hybrid | 2 | 4213 | 1.7 [0.4, 3.0] | 2.9 | -1.2 | 13.9 | 98.8 | 98.9 | 4.1 |  | +25 nil points when your team wins a trick |
| r2-diamond-seven-x | U | xmult | 5 | 5181 | 1.7 [0.3, 3.2] | 2.1 | -0.4 | 14.3 | 44.4 | 45.8 | 4.9 |  | ×2.5 contract multiplier if your team holds seven [♦]s |
| ub-four-row-points | U | points | 5 | 3010 | 1.5 [0.1, 2.9] | 1.6 | -0.1 | 8.1 | 41.7 | 43.2 | 3.6 |  | +75 contract points when your team wins four tricks in a row |
| uc-low-win-mult | U | hybrid | 5 | 3442 | 1.3 [0.2, 2.4] | 2.4 | -1.1 | 11.3 | 78.4 | 79.4 | 3.9 |  | +5 contract multiplier when your team wins a trick with a [2] through [9] |
| seed-four-row | U | mult | 5 | 3397 | 0.7 [-0.5, 2.0] | 1.5 | -0.7 | 9.6 | 39.0 | 40.8 | 4.1 |  | +15 contract multiplier when your team wins four tricks in a row |
| seed-rainbow-lead-x | U | xmult | 5 | 2960 | 0.7 [-1.0, 2.3] | 1.7 | -1.0 | 11.2 | 44.1 | 48.8 | 4.2 |  | ×2 contract multiplier if your team leads all four suits |
| ub-make8-trick-mult | U | mult | 5 | 2765 | 0.5 [-0.9, 2.0] | 1.7 | -1.2 | 9.9 | 20.1 | 21.5 | 3.7 |  | +3 contract multiplier for each trick in your team's contract if your team makes a contract of 8 or more |
| ua-three-aces-x | U | xmult | 6 | 2332 | -0.2 [-1.5, 1.1] | 1.4 | -1.6 | 9.3 | 58.8 | 75.6 | 2.9 |  | ×2 contract multiplier if your team holds three [A]s |
| seed-nil-made-x | U | xmult | 2 | 3345 | -0.3 [-1.8, 1.3] | -0.3 | 0.0 | 7.1 | 27.6 | 30.6 | 3.6 |  | ×1.5 contract multiplier if your team makes a nil |
| uc-spade-lead-mult | U | hybrid | 3 | 3776 | -0.3 [-1.3, 0.8] | 1.3 | -1.5 | 9.4 | 78.9 | 79.2 | 3.6 |  | +4 contract multiplier when your team leads [♠] |
| ua-side-raise | U | enabler | 5 | 35508 | -0.4 [-1.5, 0.7] | -0.5 | 0.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card other than [♠] your team holds by two ranks |
| control-uncommon | U | points | 1 | 3008 | -0.7 [-1.6, 0.1] | -0.6 | -0.1 | 10.2 | 100.0 |  | 2.5 |  | +35 contract points |
| ub-high-honor-hold-mult | U | mult | 5 | 4533 | -1.4 [-2.5, -0.2] | -0.1 | -1.3 | 8.6 | 100.0 | 100.0 | 2.8 |  | +1 contract multiplier for each [J] through [A] your team holds |
| r1-low-hold-mult | U | mult | 5 | 14539 | -1.9 [-2.8, -0.9] | 0.6 | -2.4 | 9.0 | 99.9 | 100.0 | 4.3 |  | +1 contract multiplier for each [2] through [6] your team holds |
| seed-flat-mult | U | mult | 1 | 3242 | -2.1 [-3.1, -1.0] | -0.1 | -1.9 | 9.4 | 100.0 |  | 2.4 |  | +10 contract multiplier |
| ub-first-trick-xmult | U | hybrid | 3 | 4526 | -2.3 [-3.4, -1.1] | -0.7 | -1.5 | 6.5 | 49.6 | 51.0 | 3.0 |  | ×1.5 contract multiplier when your team wins the first trick of a round |
| ua-rainbow-first-mult | U | mult | 4 | 2786 | -2.9 [-4.4, -1.4] | -0.7 | -2.2 | 5.9 | 99.2 | 99.3 | 2.5 |  | +2 contract multiplier when your team wins its first trick with each suit |
| uc-low-lead-mult | U | hybrid | 5 | 6761 | -2.9 [-3.9, -2.0] | -0.2 | -2.7 | 7.9 | 88.1 | 88.2 | 4.0 |  | +3 contract multiplier when your team leads a [2] through [6] |
| ub-make7-xmult | U | xmult | 4 | 3805 | -3.1 [-4.1, -2.0] | -1.2 | -1.8 | 7.6 | 26.0 | 28.2 | 4.0 | lift<band | ×1.5 contract multiplier if your team makes a contract of 7 or more |
| ua-ace-hold-mult | U | mult | 4 | 4140 | -4.0 [-5.3, -2.8] | -1.4 | -2.6 | 6.3 | 97.2 | 97.7 | 2.4 | lift<band | +2 contract multiplier for each [A] your team holds |
| rb-streak-contract-mult | R | mult | 4 | 3614 | 12.9 [11.6, 14.2] | 9.1 | 3.8 | 18.2 | 82.5 | 82.8 | 8.5 |  | +1 contract multiplier for each trick in your team's contract when your team wins consecutive tricks |
| r2-diamonds-beat | R | enabler | 3 | 8351 | 12.5 [11.4, 13.5] | 7.5 | 4.9 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [♦]s beat every other card of their suit |
| r2-high-middle-free | R | enabler | 4 | 13745 | 11.6 [10.5, 12.8] | 5.1 | 6.6 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [7]s through [9]s even when it can follow suit |
| r2-twos-beat-rare | R | enabler | 4 | 12410 | 11.5 [10.3, 12.8] | 7.5 | 4.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s beat every other card of their suit |
| seed-flat-x | R | xmult | 1 | 4049 | 11.4 [10.3, 12.6] | 5.1 | 6.4 | 14.9 | 100.0 |  | 6.5 |  | ×2 contract multiplier |
| ra-jq-become-aces | R | enabler | 7 | 8240 | 11.3 [10.2, 12.5] | 5.3 | 6.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Every [J] through [Q] your team holds becomes an [A] |
| rb-outbid-x | R | xmult | 2 | 4444 | 10.2 [9.0, 11.5] | 3.5 | 6.7 | 21.6 | 42.2 | 45.6 | 5.1 |  | ×4 contract multiplier when your team bids more than the opponents |
| ra-diamond-hold-x | R | xmult | 3 | 4674 | 9.3 [7.9, 10.7] | 3.9 | 5.4 | 14.4 | 100.0 | 100.0 | 7.7 |  | ×1.1 contract multiplier for each [♦] your team holds |
| rb-honors-free | R | enabler | 4 | 10919 | 9.0 [7.9, 10.1] | 3.0 | 6.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [10]s through [A]s even when it can follow suit |
| r1-spade-four-x | R | xmult | 6 | 2631 | 8.7 [7.4, 10.0] | 3.1 | 5.6 | 17.9 | 44.0 | 44.3 | 4.5 |  | ×2.5 contract multiplier when your team wins four tricks with [♠] |
| seed-raise | R | enabler | 4 | 61188 | 8.6 [7.6, 9.6] | 3.5 | 5.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by two ranks |
| ra-rainbow-first-x | R | xmult | 4 | 4030 | 4.4 [3.3, 5.5] | 2.5 | 1.8 | 13.0 | 98.9 | 99.0 | 7.2 |  | ×1.2 contract multiplier when your team wins its first trick with each suit |
| ra-trump-x | R | xmult | 3 | 14801 | 3.8 [2.8, 4.7] | 0.9 | 2.8 | 14.4 | 78.6 | 79.2 | 5.4 |  | ×1.25 contract multiplier when your team wins a trick by trumping |
| seed-two-win | R | points | 4 | 9123 | 3.7 [2.6, 4.7] | 3.1 | 0.6 | 16.6 | 25.1 | 28.2 | 5.0 |  | +180 contract points when your team wins a trick with a [2] |
| rc-low-lead-x | R | xmult | 5 | 4332 | 2.6 [1.5, 3.6] | -0.3 | 2.9 | 9.7 | 85.7 | 85.7 | 5.1 |  | ×1.15 contract multiplier when your team leads a [2] through [6] |
| seed-rainbow-x | R | xmult | 5 | 3893 | 1.9 [0.5, 3.2] | 2.4 | -0.6 | 11.6 | 27.6 | 34.0 | 3.3 |  | ×2.5 contract multiplier if your team wins tricks with all four suits |
| ra-ace-hold-x | R | xmult | 4 | 13280 | 1.5 [0.5, 2.4] | 0.2 | 1.3 | 10.3 | 98.6 | 99.0 | 5.4 |  | ×1.1 contract multiplier for each [A] your team holds |
| ra-diamond-led-win-x | R | hybrid | 3 | 5267 | 0.8 [-0.3, 1.9] | 0.2 | 0.6 | 9.9 | 78.4 | 78.6 | 4.9 |  | ×1.25 contract multiplier when your team wins a trick led with [♦] |
| ra-spade-lead-x | R | hybrid | 3 | 4514 | 0.1 [-1.0, 1.1] | -0.4 | 0.4 | 8.9 | 77.5 | 77.8 | 5.3 |  | ×1.15 contract multiplier when your team leads [♠] |
| seed-ace-mult | R | mult | 4 | 3110 | -0.0 [-1.4, 1.4] | 2.1 | -2.1 | 9.0 | 84.5 | 89.7 | 3.4 |  | +4 contract multiplier when your team wins a trick with an [A] |
| rb-opp-win-mult | R | mult | 2 | 17087 | -0.8 [-1.7, 0.2] | -0.3 | -0.5 | 13.0 | 99.5 | 99.5 | 4.9 |  | +2 contract multiplier when the opponents win a trick |
| control-rare | R | points | 1 | 2926 | -1.6 [-2.4, -0.8] | -0.7 | -0.9 | 12.2 | 100.0 |  | 3.3 |  | +45 contract points |
| r1-free-discards | L | enabler | 2 | 2054 | 6.7 [5.5, 8.0] | 2.2 | 4.6 | 0.0 | 0.0 |  | 0.0 |  | Your team can play cards other than [♠] even when it can follow suit |
| r1-faces-untrumpable | L | enabler | 5 | 35232 | 1.8 [0.6, 3.0] | 3.0 | -1.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [Q]s through [A]s can't be trumped |
| rb-contract-trick-x | L | xmult | 2 | 4474 | 1.7 [0.3, 3.0] | -0.4 | 2.1 | 17.0 | 100.0 | 100.0 | 7.9 |  | ×1.15 contract multiplier for each trick in your team's contract |
| ra-ace-win-x | L | xmult | 4 | 33871 | 1.6 [0.6, 2.6] | -1.0 | 2.6 | 16.9 | 90.9 | 91.5 | 9.4 | lift<band | ×1.2 contract multiplier when your team wins a trick with an [A] |
| control-legendary | L | points | 1 | 2950 | 1.3 [0.3, 2.3] | 0.8 | 0.5 | 14.5 | 100.0 |  | 7.5 |  | +70 contract points |
| rc-low-win-x | L | xmult | 5 | 99984 | -4.5 [-5.5, -3.5] | -5.0 | 0.4 | 11.1 | 58.4 | 59.0 | 4.2 | lift<band | ×1.25 contract multiplier when your team wins a trick with a [2] through [6] |
