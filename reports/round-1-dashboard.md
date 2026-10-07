# round-1: dashboard

Fit: 131530 boards, alpha 20, residual sd 0.416 (u units), win calibration k = 1.27, smoothing scale 25128 points.
Tier agreement: 145 sigils, lift correlation 0.36 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 4671264 rounds.

**Fun score: 55.8** (90% interval 54.7–56.7)

| Family | Weight | Score | Sub-metrics |
| --- | --- | --- | --- |
| Close and live | 25 | 0.61 | median margin / winner 0.67; trailer after round 5 wins 0.23; rounds 1-4 share 0.38 |
| Commitment works | 15 | 0.50 | online by round 4 0.23; committed win rate 0.52 |
| Archetypes viable | 15 | 0.87 | archetypes with committed win 40-60% 1.00; largest archetype share of winning flexible builds 0.31 |
| Synergy and combos | 15 | 0.28 | same-archetype pair gain over parts 0.01; strong pairs (>=50% over parts) 9; strong cross-archetype pairs 1 |
| Skill and bidding | 15 | 0.82 | stronger tier beats tier 0 (boards) 0.84; late overtricks per made 0.92; set rate 0.33 |
| Simplicity | 15 | 0.22 | mean S 0.22 |

Commitment: {"per_archetype": {"Suits": 0.505, "Spades": 0.545, "Ranks": 0.50125, "BidHigh": 0.515, "Nil": 0.5375, "LowCards": 0.55375, "Rainbow": 0.50125, "Streaks": 0.49, "Exact": 0.5125}, "committed_win": 0.5179166666666667, "online": 0.22527777777777777, "online_by_arch": {"Suits": 0.13125, "Spades": 0.45, "Ranks": 0.1575, "BidHigh": 0.39375, "Nil": 0.2525, "LowCards": 0.23375, "Rainbow": 0.11375, "Streaks": 0.2775, "Exact": 0.0175}, "winning_share": {"Suits": 0.024, "Spades": 0.17933333333333334, "Ranks": 0.314, "BidHigh": 0.17333333333333334, "Nil": 0.03, "LowCards": 0.06333333333333334, "Rainbow": 0.032, "Streaks": 0.12266666666666666, "Exact": 0.0}}

Ladder (tier 1 vs tier 0 with the pool, boards): 0.845

Global factor 0.84

Build chasers:

- 0.513 [0.485, 0.541] (840 runs): seed-four-row
- 0.512 [0.484, 0.540] (840 runs): ca-high-spade-hold
- 0.500 [0.472, 0.528] (840 runs): uc-twos-beat
- 0.500 [0.457, 0.543] (360 runs): rc-twos-always-win
- 0.500 [0.425, 0.575] (120 runs): ca-face-win
- 0.500 [0.425, 0.575] (120 runs): ra-team-untrumpable
- 0.497 [0.454, 0.541] (360 runs): ub-trick-xmult
- 0.492 [0.417, 0.567] (120 runs): uc-spade-lead-mult
- 0.475 [0.400, 0.550] (120 runs): cb-become-spade
- 0.467 [0.392, 0.542] (120 runs): seed-e-spade-win
| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| seed-ace-hold | C | points | 4 | 3269 | 6.8 [5.5, 8.1] | 5.5 | 1.3 | 7.4 | 95.4 | 96.6 | 4.2 |  | +20 contract points for each [A] your team holds |
| cb-become-spade | C | enabler | 6 | 12880 | 6.4 [5.2, 7.5] | 2.8 | 3.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [♠]s |
| seed-rainbow-first | C | points | 4 | 7030 | 3.6 [2.4, 4.8] | 2.2 | 1.4 | 9.9 | 99.0 | 99.3 | 4.1 |  | +20 contract points when your team wins its first trick with each suit |
| seed-contract-points | C | points | 2 | 2005 | 3.4 [2.0, 4.8] | 2.0 | 1.4 | 8.6 | 100.0 | 100.0 | 4.4 |  | +10 contract points for each trick in your team's contract |
| ca-high-spade-hold | C | points | 6 | 6951 | 3.2 [2.1, 4.2] | 1.8 | 1.4 | 8.5 | 98.5 | 98.6 | 3.0 |  | +20 contract points for each [10] through [A] of [♠] your team holds |
| seed-e-lead-three-suits | C | points | 5 | 2587 | 2.9 [1.4, 4.4] | 1.2 | 1.7 | 4.9 | 84.2 | 86.4 | 3.0 | rarely decisive | +55 contract points if your team leads three suits |
| seed-ace-win | C | points | 4 | 1682 | 2.8 [1.1, 4.5] | 1.8 | 1.0 | 4.7 | 85.8 | 88.6 | 4.0 | rarely decisive | +20 contract points when your team wins a trick with an [A] |
| ca-diamond-led-win | C | points | 3 | 10517 | 2.4 [1.3, 3.4] | 1.4 | 0.9 | 9.0 | 81.3 | 81.4 | 4.5 |  | +35 contract points when your team wins a trick led with [♦] |
| e-make-pts | C | points | 2 | 1228 | 2.2 [0.7, 3.6] | 0.9 | 1.2 | 5.2 | 69.1 |  | 2.7 |  | +55 contract points if your team makes its contract |
| seed-e-spade-win | C | points | 3 | 1819 | 2.1 [0.7, 3.5] | 1.1 | 1.0 | 5.2 | 93.6 | 93.6 | 2.6 |  | +15 contract points when your team wins a trick with [♠] |
| seed-trump-points | C | points | 3 | 7185 | 2.0 [0.9, 3.1] | 1.6 | 0.4 | 8.9 | 78.2 | 79.3 | 4.9 |  | +35 contract points when your team wins a trick by trumping |
| ca-king-hold | C | points | 4 | 7451 | 1.9 [0.8, 3.0] | 1.3 | 0.6 | 7.7 | 94.0 | 93.3 | 3.9 |  | +25 contract points for each [K] your team holds |
| cc-low-narrow-win | C | hybrid | 5 | 1954 | 1.8 [0.5, 3.1] | 1.2 | 0.6 | 5.4 | 72.1 | 73.5 | 3.6 |  | +35 contract points when your team wins a trick with a [2] through [7] |
| ca-face-win | C | points | 5 | 7155 | 1.8 [0.6, 3.0] | 0.3 | 1.5 | 8.5 | 84.7 | 83.1 | 4.3 |  | +25 contract points when your team wins a trick with a [J] through [K] |
| ca-diamond-lead-three | C | hybrid | 6 | 1668 | 1.6 [-0.8, 4.0] | 1.1 | 0.5 | 4.4 | 30.2 | 32.6 | 2.2 |  | +20 contract multiplier when your team leads [♦] three times |
| cb-honor-hold | C | points | 5 | 2310 | 1.6 [0.2, 2.9] | 0.2 | 1.4 | 4.9 | 100.0 | 100.0 | 3.8 | rarely decisive | +5 contract points for each [J] through [A] your team holds |
| control-common | C | points | 1 | 2283 | 1.3 [0.4, 2.3] | 0.6 | 0.7 | 4.6 | 100.0 |  | 1.5 | rarely decisive | +40 contract points |
| cb-contract-mult-made | C | mult | 3 | 9535 | 0.9 [-0.1, 2.0] | -0.4 | 1.3 | 5.2 | 59.0 | 58.4 | 3.0 |  | +1 contract multiplier for each trick in your team's contract if your team makes its contract |
| seed-nil-points | C | points | 1 | 2541 | 0.7 [-1.3, 2.6] | -0.1 | 0.8 | 6.7 | 100.0 | 100.0 | 0.7 | flat slope | +100 nil points |
| e-win-suit-d-mult | C | mult | 3 | 7519 | 0.6 [-0.8, 1.9] | -0.5 | 1.1 | 3.8 | 65.4 | 68.6 | 2.4 |  | +4 contract multiplier when your team wins a trick with [♦] |
| seed-win-points | C | points | 2 | 1110 | 0.1 [-1.4, 1.6] | -0.5 | 0.6 | 3.0 | 99.3 |  | 2.2 | rarely decisive | +5 contract points when your team wins a trick |
| cc-nil-contract-trick | C | hybrid | 2 | 1896 | -0.1 [-1.5, 1.3] | -0.2 | 0.1 | 3.3 | 100.0 | 100.0 | 1.5 | flat slope | +20 nil points for each trick in your team's contract |
| cc-two-win-mult | C | hybrid | 4 | 12061 | -0.2 [-1.2, 0.9] | -0.9 | 0.8 | 7.2 | 32.6 | 36.4 | 2.7 |  | +15 contract multiplier when your team wins a trick with a [2] |
| cb-make-x | C | xmult | 2 | 7158 | -0.2 [-1.2, 0.8] | -1.7 | 1.5 | 4.2 | 62.1 |  | 1.9 | lift<band | ×1.25 contract multiplier if your team makes its contract |
| cc-nil-made-points | C | points | 2 | 2410 | -0.3 [-2.3, 1.7] | -0.3 | 0.0 | 4.3 | 22.1 | 25.1 | 1.3 | rarely decisive, flat slope | +160 contract points if your team makes a nil |
| e-last-xmult | C | xmult | 3 | 2069 | -0.4 [-2.0, 1.1] | -2.6 | 2.2 | 6.1 | 48.2 | 48.4 | 2.5 | lift<band | ×1.5 contract multiplier when your team wins the last trick of a round |
| cb-trick-mult | C | mult | 2 | 1620 | -0.6 [-2.1, 0.9] | -1.6 | 1.0 | 4.8 | 99.2 | 99.2 | 2.8 | lift<band | +1 contract multiplier when your team wins a trick |
| cc-low-hold-points | C | points | 5 | 2232 | -0.6 [-1.9, 0.7] | -1.4 | 0.9 | 4.7 | 100.0 | 100.0 | 2.9 | lift<band, rarely decisive | +5 contract points for each [2] through [6] your team holds |
| seed-rainbow-mult | C | mult | 5 | 7506 | -0.7 [-2.0, 0.5] | -1.3 | 0.5 | 4.9 | 33.1 | 37.7 | 2.2 | lift<band | +15 contract multiplier if your team wins tricks with all four suits |
| cb-make8-mult | C | mult | 4 | 3296 | -1.4 [-2.6, -0.2] | -2.3 | 0.9 | 5.1 | 16.0 | 17.7 | 2.4 | lift<band | +15 contract multiplier if your team makes a contract of 8 or more |
| cc-any-suit-first-two | C | enabler | 2 | 3294 | -1.4 [-3.2, 0.3] | -1.4 | -0.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play any suit on the first two tricks |
| ca-rainbow-lead-mult | C | hybrid | 5 | 1580 | -1.5 [-3.5, 0.4] | -2.0 | 0.5 | 3.8 | 44.3 | 48.3 | 2.4 | lift<band | +10 contract multiplier if your team leads all four suits |
| cc-nil-made-mult | C | mult | 2 | 2407 | -1.5 [-3.7, 0.6] | -1.0 | -0.5 | 3.8 | 21.5 | 24.6 | 1.9 |  | +10 contract multiplier if your team makes a nil |
| seed-diamond-hold | C | points | 3 | 3737 | -1.8 [-3.4, -0.3] | -2.0 | 0.2 | 3.4 | 100.0 | 100.0 | 2.3 | lift<band, rarely decisive | +5 contract points for each [♦] your team holds |
| cb-become-ace | C | enabler | 7 | 13117 | -1.9 [-3.1, -0.6] | 1.5 | -3.3 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two cards your team holds become [A]s |
| cc-low-last-mult | C | hybrid | 6 | 9488 | -2.1 [-3.2, -1.1] | -2.2 | 0.1 | 4.9 | 28.2 | 27.9 | 2.0 | lift<band | +15 contract multiplier when your team wins the last trick of a round with a [2] through [10] |
| seed-diamond-lead | C | points | 3 | 2773 | -2.2 [-4.0, -0.5] | -2.9 | 0.7 | 3.5 | 82.2 | 83.1 | 1.4 | lift<band, rarely decisive | +10 contract points when your team leads [♦] |
| cb-contract-mult | C | mult | 2 | 3324 | -2.2 [-3.5, -1.0] | -2.3 | 0.1 | 4.4 | 100.0 | 100.0 | 0.2 | lift<band, flat slope | +1 contract multiplier for each trick in your team's contract |
| ca-three-aces-mult | C | mult | 6 | 2168 | -2.3 [-3.8, -0.7] | -0.9 | -1.4 | 4.7 | 62.7 | 74.8 | 0.6 | flat slope | +10 contract multiplier if your team holds three [A]s |
| cc-two-hold-mult | C | mult | 4 | 7763 | -2.3 [-3.3, -1.3] | -2.6 | 0.3 | 3.9 | 78.0 | 81.1 | 2.6 | lift<band | +4 contract multiplier for each [2] your team holds |
| ca-spade-last-trick | C | mult | 4 | 10076 | -2.4 [-3.5, -1.3] | -2.3 | -0.2 | 4.4 | 40.3 | 40.1 | 1.6 | lift<band | +10 contract multiplier when your team wins the last trick of a round with [♠] |
| seed-diamond-win | C | points | 3 | 2891 | -2.5 [-4.0, -1.0] | -2.6 | 0.1 | 5.0 | 65.4 | 68.6 | 2.9 | lift<band, rarely decisive | +20 contract points when your team wins a trick with [♦] |
| cc-small-contract-mult | C | hybrid | 3 | 10288 | -2.7 [-3.9, -1.4] | -3.1 | 0.4 | 3.4 | 32.6 | 33.5 | 1.9 | lift<band | +10 contract multiplier when your team bids 4 or less |
| seed-swap | C | enabler | 5 | 2890 | -2.7 [-4.4, -0.9] | -2.0 | -0.7 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Swap three cards with your partner |
| cc-nil-bid-xmult | C | xmult | 2 | 1453 | -2.7 [-5.4, -0.0] | -2.9 | 0.2 | 3.4 | 39.2 | 43.0 | 2.4 | lift<band | ×1.5 contract multiplier when your team bids nil |
| e-lead-rank-k-mult | C | mult | 4 | 2144 | -2.8 [-4.4, -1.2] | -1.8 | -1.1 | 4.2 | 53.0 | 52.3 | 0.3 | lift<band, flat slope | +5 contract multiplier when your team leads a [K] |
| ca-heart-hold-eight | C | mult | 5 | 1727 | -3.0 [-4.9, -1.1] | -2.1 | -0.9 | 2.4 | 23.7 | 23.6 | 1.7 | lift<band, flat slope | +20 contract multiplier if your team holds eight [♥]s |
| seed-last-trick | C | mult | 3 | 7773 | -3.1 [-4.3, -1.9] | -2.0 | -1.1 | 4.8 | 47.9 | 48.3 | 1.5 | lift<band | +10 contract multiplier when your team wins the last trick of a round |
| ca-spade-lead-three | C | hybrid | 6 | 7090 | -3.1 [-4.3, -1.9] | -3.3 | 0.2 | 4.5 | 29.0 | 29.4 | 1.8 | lift<band | +15 contract multiplier when your team leads [♠] three times |
| cb-exact-trick-mult | C | mult | 3 | 3015 | -3.5 [-4.7, -2.3] | -3.4 | -0.1 | 5.6 | 23.5 | 23.6 | -0.0 | lift<band, flat slope | +3 contract multiplier for each trick in your team's contract if your team makes its contract exactly |
| ca-diamond-win-two | C | mult | 6 | 3918 | -3.5 [-5.2, -1.9] | -2.8 | -0.7 | 2.6 | 27.1 | 32.8 | 1.1 | lift<band, flat slope | +10 contract multiplier when your team wins two tricks with [♦] |
| cb-bid8-trick-mult | C | mult | 4 | 3895 | -3.6 [-4.8, -2.4] | -3.6 | -0.0 | 3.6 | 32.5 | 34.5 | 1.0 | lift<band, flat slope | +2 contract multiplier for each trick in your team's contract when your team bids 8 or more |
| e-hold-suit-s-mult | C | mult | 3 | 1439 | -3.7 [-5.8, -1.7] | -3.0 | -0.7 | 5.0 | 100.0 | 100.0 | 2.6 | lift<band | +1 contract multiplier for each [♠] your team holds |
| cb-make-mult | C | mult | 2 | 6192 | -3.7 [-4.8, -2.7] | -2.1 | -1.6 | 3.4 | 60.0 | 59.0 | 1.8 | lift<band | +4 contract multiplier if your team makes its contract |
| seed-nil-bid-mult | C | mult | 2 | 4112 | -3.8 [-5.3, -2.2] | -2.0 | -1.7 | 2.8 | 38.8 | 42.1 | 1.0 | lift<band, flat slope | +10 contract multiplier when your team bids nil |
| cb-consecutive-mult | C | mult | 3 | 3145 | -4.8 [-6.1, -3.5] | -2.0 | -2.8 | 3.4 | 84.0 | 84.5 | 2.1 | lift<band | +1 contract multiplier when your team wins consecutive tricks |
| cb-spade-seven | C | mult | 5 | 9731 | -4.8 [-5.9, -3.8] | -2.5 | -2.3 | 3.9 | 57.4 | 57.7 | 2.1 | lift<band | +10 contract multiplier if your team holds seven [♠]s |
| ca-ace-win-two-mult | C | mult | 7 | 5934 | -4.9 [-6.1, -3.7] | -3.5 | -1.4 | 2.5 | 58.4 | 66.9 | 0.5 | lift<band, flat slope | +5 contract multiplier when your team wins two tricks with [A]s |
| ca-diamond-hold-eight | C | mult | 5 | 4561 | -5.0 [-6.6, -3.4] | -4.1 | -0.9 | 3.5 | 23.8 | 23.8 | 1.2 | lift<band | +20 contract multiplier if your team holds eight [♦]s |
| cb-first-trick-mult | C | mult | 3 | 1741 | -5.6 [-7.3, -3.9] | -2.8 | -2.8 | 2.3 | 47.9 | 48.5 | 1.3 | lift<band, flat slope | +5 contract multiplier when your team wins the first trick of a round |
| cb-bid7-mult | C | mult | 3 | 6780 | -5.8 [-7.0, -4.6] | -2.9 | -2.9 | 5.1 | 49.5 | 52.5 | 0.6 | lift<band, flat slope | +10 contract multiplier when your team bids 7 or more |
| cb-grow-make-mult | C | mult | 4 | 7250 | -5.8 [-6.9, -4.8] | -3.3 | -2.5 | 1.7 | 66.9 |  | 1.1 | lift<band | This sigil gains +2 contract multiplier every time your team makes its contract (currently +0) |
| cb-three-row-mult | C | mult | 5 | 6549 | -5.9 [-7.1, -4.7] | -2.3 | -3.6 | 2.8 | 58.6 | 59.8 | 1.6 | lift<band | +5 contract multiplier when your team wins three tricks in a row |
| cb-opp-set-mult | C | mult | 2 | 6483 | -5.9 [-6.9, -4.9] | -2.7 | -3.3 | 2.9 | 37.5 |  | 2.2 | lift<band | +10 contract multiplier if the opponents miss their contract |
| e-trump-mult | C | mult | 3 | 3496 | -6.0 [-7.3, -4.6] | -3.0 | -2.9 | 3.0 | 77.4 | 78.7 | 1.7 | lift<band | +3 contract multiplier when your team wins a trick by trumping |
| ca-raise-one | C | enabler | 6 | 2226 | -6.1 [-7.9, -4.3] | -2.5 | -3.6 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every card your team holds by one rank |
| seed-exact-mult | C | mult | 2 | 10879 | -9.2 [-10.8, -7.7] | -5.5 | -3.7 | 8.6 | 22.6 | 22.7 | 2.1 | lift<band | +15 contract multiplier if your team makes its contract exactly |
| uc-twos-beat | U | enabler | 4 | 27324 | 14.6 [13.4, 15.7] | 7.8 | 6.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s beat every other card of their suit |
| uc-low-discard | U | enabler | 4 | 4017 | 9.0 [7.5, 10.4] | 2.3 | 6.7 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [2]s through [5]s even when it can follow suit |
| ub-trick-xmult | U | xmult | 2 | 1202 | 8.5 [6.8, 10.2] | 5.1 | 3.4 | 15.9 | 98.8 |  | 6.9 |  | ×1.1 contract multiplier when your team wins a trick |
| uc-trump-freely | U | enabler | 2 | 17726 | 8.0 [6.8, 9.2] | 1.7 | 6.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [♠]s even when it can follow suit |
| ub-low-to-aces | U | enabler | 8 | 2945 | 5.9 [3.9, 7.9] | 2.2 | 3.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three [2]s through [9]s your team holds become [A]s |
| ua-become-three-aces | U | enabler | 5 | 13294 | 5.4 [4.2, 6.7] | 1.3 | 4.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [A]s |
| ub-become-four-spades | U | enabler | 5 | 15879 | 4.9 [3.6, 6.3] | 0.8 | 4.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Four cards other than [♠] your team holds become [♠]s |
| seed-four-row | U | mult | 5 | 2650 | 3.6 [2.1, 5.1] | 2.7 | 0.9 | 14.4 | 38.6 | 39.0 | 3.6 |  | +40 contract multiplier when your team wins four tricks in a row |
| uc-spade-lead-mult | U | hybrid | 3 | 1725 | 2.8 [1.3, 4.4] | 2.0 | 0.8 | 15.3 | 77.0 | 77.0 | 5.3 |  | +10 contract multiplier when your team leads [♠] |
| control-uncommon | U | points | 1 | 2342 | 2.7 [1.7, 3.7] | 2.2 | 0.5 | 8.1 | 100.0 |  | 4.2 |  | +60 contract points |
| uc-low-win-mult | U | hybrid | 5 | 17048 | 2.6 [1.5, 3.7] | 1.9 | 0.7 | 15.8 | 76.8 | 78.1 | 5.7 |  | +10 contract multiplier when your team wins a trick with a [2] through [9] |
| ua-diamond-hold-mult | U | mult | 3 | 2327 | 2.5 [0.7, 4.3] | 1.5 | 1.0 | 9.8 | 100.0 | 100.0 | 5.0 |  | +3 contract multiplier for each [♦] your team holds |
| ub-make7-xmult | U | xmult | 4 | 1612 | 2.2 [0.7, 3.7] | -0.3 | 2.5 | 12.1 | 26.0 | 27.3 | 4.0 |  | ×2 contract multiplier if your team makes a contract of 7 or more |
| ua-rainbow-first-mult | U | mult | 4 | 2005 | 2.1 [0.3, 3.9] | 1.2 | 1.0 | 8.5 | 99.1 | 99.3 | 3.1 |  | +5 contract multiplier when your team wins its first trick with each suit |
| ub-four-row-points | U | points | 5 | 2059 | 1.9 [0.3, 3.6] | 0.1 | 1.8 | 10.2 | 41.2 | 42.3 | 4.1 |  | +135 contract points when your team wins four tricks in a row |
| ub-consecutive-xmult | U | xmult | 3 | 2109 | 1.7 [0.1, 3.4] | -1.2 | 3.0 | 7.6 | 83.2 | 83.6 | 5.0 |  | ×1.1 contract multiplier when your team wins consecutive tricks |
| ua-diamonds-untrumpable | U | enabler | 3 | 11152 | 1.6 [0.5, 2.6] | -1.5 | 3.0 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team's [♦]s can't be trumped |
| ua-high-spade-win | U | points | 6 | 1737 | 1.4 [-0.1, 2.9] | 0.2 | 1.2 | 10.6 | 84.8 | 85.1 | 2.6 |  | +40 contract points when your team wins a trick with a [J] through [A] of [♠] |
| ub-make8-trick-mult | U | mult | 5 | 1165 | 1.3 [-0.4, 3.1] | -0.0 | 1.3 | 9.9 | 17.8 | 18.9 | 4.4 |  | +5 contract multiplier for each trick in your team's contract if your team makes a contract of 8 or more |
| seed-flat-mult | U | mult | 1 | 1196 | 1.2 [-0.3, 2.8] | 0.8 | 0.4 | 9.3 | 100.0 |  | 3.3 |  | +20 contract multiplier |
| uc-junk-to-aces | U | enabler | 8 | 2790 | 1.2 [-0.6, 3.1] | 0.1 | 1.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two [2]s through [6]s your team holds become [A]s |
| ub-consecutive-contract-points | U | hybrid | 4 | 1775 | 1.2 [-0.3, 2.7] | 0.8 | 0.5 | 11.2 | 85.6 | 86.0 | 4.3 |  | +3 contract points for each trick in your team's contract when your team wins consecutive tricks |
| ua-spade-last-x | U | hybrid | 4 | 1813 | 1.1 [-0.3, 2.6] | 0.6 | 0.5 | 10.6 | 41.0 | 40.9 | 3.1 |  | ×2.5 contract multiplier when your team wins the last trick of a round with [♠] |
| ub-high-honor-hold-mult | U | mult | 5 | 2181 | 1.0 [-0.3, 2.4] | 0.3 | 0.7 | 10.1 | 100.0 | 100.0 | 2.9 |  | +2 contract multiplier for each [J] through [A] your team holds |
| ub-bid7-points | U | points | 3 | 1107 | 0.9 [-0.7, 2.5] | -1.1 | 2.0 | 7.4 | 52.9 | 56.7 | 3.2 |  | +130 contract points when your team bids 7 or more |
| ub-first-trick-contract-points | U | hybrid | 4 | 1684 | 0.5 [-1.0, 2.1] | -0.4 | 0.9 | 11.4 | 52.0 | 52.5 | 4.7 |  | +20 contract points for each trick in your team's contract when your team wins the first trick of a round |
| uc-nil-cover-points | U | hybrid | 2 | 2239 | 0.3 [-1.9, 2.6] | -0.4 | 0.7 | 9.8 | 98.8 | 98.7 | 5.5 |  | +35 nil points when your team wins a trick |
| ua-ace-hold-mult | U | mult | 4 | 1871 | 0.3 [-1.2, 1.8] | 1.0 | -0.7 | 11.7 | 95.9 | 96.3 | 3.3 |  | +5 contract multiplier for each [A] your team holds |
| ua-spade-seven-x | U | xmult | 5 | 2316 | 0.3 [-1.1, 1.6] | -1.1 | 1.3 | 10.5 | 57.3 | 57.5 | 1.8 |  | ×2 contract multiplier if your team holds seven [♠]s |
| ub-bid5-xmult | U | hybrid | 3 | 3117 | 0.0 [-1.6, 1.6] | -0.5 | 0.6 | 12.3 | 53.6 | 54.9 | 2.8 |  | ×2 contract multiplier when your team bids 5 or less |
| seed-spade-hold | U | points | 3 | 1818 | -0.4 [-2.2, 1.4] | -1.3 | 0.9 | 13.5 | 100.0 | 100.0 | 4.0 |  | +10 contract points for each [♠] your team holds |
| ua-rainbow-first-contract | U | points | 5 | 1708 | -0.5 [-2.0, 1.0] | -0.6 | 0.1 | 10.7 | 99.1 | 99.2 | 5.5 |  | +4 contract points for each trick in your team's contract when your team wins its first trick with each suit |
| uc-low-hold-mult | U | mult | 5 | 2200 | -0.8 [-2.3, 0.7] | -1.5 | 0.7 | 9.9 | 97.0 | 97.9 | 3.6 | lift<band | +3 contract multiplier for each [2] through [4] your team holds |
| ub-first-trick-xmult | U | hybrid | 3 | 2612 | -1.0 [-2.6, 0.7] | -1.1 | 0.2 | 10.3 | 50.3 | 50.1 | 3.8 |  | ×2 contract multiplier when your team wins the first trick of a round |
| ub-bid8-trick-points | U | points | 4 | 1073 | -1.0 [-2.6, 0.7] | -1.2 | 0.2 | 7.9 | 41.7 | 44.2 | 2.5 |  | +25 contract points for each trick in your team's contract when your team bids 8 or more |
| ua-diamond-honor-win | U | points | 6 | 1832 | -1.2 [-2.7, 0.4] | -2.3 | 1.1 | 5.9 | 58.6 | 60.1 | 2.1 | lift<band | +40 contract points when your team wins a trick with a [J] through [A] of [♦] |
| ua-spade-raise | U | enabler | 5 | 17756 | -1.3 [-2.6, -0.0] | -2.5 | 1.2 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every [♠] your team holds by two ranks |
| seed-nil-made-x | U | xmult | 2 | 1912 | -1.4 [-3.5, 0.8] | -2.0 | 0.7 | 11.7 | 23.6 | 25.9 | 4.5 |  | ×2 contract multiplier if your team makes a nil |
| uc-low-lead-mult | U | hybrid | 5 | 13266 | -1.6 [-2.7, -0.5] | -2.1 | 0.5 | 10.4 | 88.4 | 89.0 | 3.8 | lift<band | +5 contract multiplier when your team leads a [2] through [6] |
| ua-three-aces-x | U | xmult | 6 | 1873 | -1.8 [-3.7, 0.2] | -3.5 | 1.7 | 11.1 | 61.2 | 73.8 | 4.8 | lift<band | ×2 contract multiplier if your team holds three [A]s |
| uc-exact-contract-points | U | points | 3 | 4123 | -2.4 [-3.6, -1.2] | -3.1 | 0.7 | 21.0 | 22.8 | 22.7 | 3.1 | lift<band | +40 contract points for each trick in your team's contract if your team makes its contract exactly |
| seed-rainbow-lead-x | U | xmult | 5 | 1855 | -2.7 [-4.4, -1.0] | -2.6 | -0.1 | 12.7 | 43.6 | 46.7 | 2.9 | lift<band | ×2 contract multiplier if your team leads all four suits |
| ua-side-raise | U | enabler | 5 | 29840 | -2.9 [-4.1, -1.8] | -4.1 | 1.2 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every card other than [♠] your team holds by two ranks |
| uc-nil-low-hold | U | points | 5 | 13029 | -4.3 [-5.3, -3.3] | -4.4 | 0.1 | 8.6 | 99.9 | 100.0 | 4.1 | lift<band | +15 nil points for each [2] through [6] your team holds |
| ua-twos-become-aces | U | enabler | 6 | 2181 | -4.4 [-6.6, -2.3] | -3.5 | -1.0 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [2] your team holds becomes an [A] |
| ua-diamond-two-x | U | xmult | 6 | 4639 | -6.0 [-7.6, -4.5] | -6.1 | 0.1 | 5.9 | 28.8 | 35.4 | 1.1 | lift<band | ×1.5 contract multiplier when your team wins two tricks with [♦] |
| seed-become-king | U | enabler | 5 | 2376 | -7.5 [-9.4, -5.5] | -6.4 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [K]s |
| ra-ace-hold-x | R | xmult | 4 | 15205 | 9.3 [8.3, 10.3] | 5.6 | 3.7 | 21.6 | 97.1 | 97.4 | 8.2 |  | ×1.25 contract multiplier for each [A] your team holds |
| ra-jq-become-aces | R | enabler | 7 | 7322 | 9.1 [7.8, 10.5] | 2.7 | 6.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Every [J] through [Q] your team holds becomes an [A] |
| seed-ace-mult | R | mult | 4 | 2958 | 6.7 [5.1, 8.2] | 3.6 | 3.1 | 15.1 | 84.7 | 87.7 | 4.5 |  | +10 contract multiplier when your team wins a trick with an [A] |
| ra-trump-x | R | xmult | 3 | 3394 | 5.7 [4.4, 7.0] | 2.9 | 2.9 | 20.5 | 72.8 | 73.1 | 6.9 |  | ×1.5 contract multiplier when your team wins a trick by trumping |
| seed-raise | R | enabler | 4 | 58929 | 5.6 [4.5, 6.7] | 1.2 | 4.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by two ranks |
| ra-spade-lead-x | R | hybrid | 3 | 3921 | 5.5 [4.4, 6.7] | 4.8 | 0.7 | 23.7 | 76.1 | 76.3 | 9.2 |  | ×1.5 contract multiplier when your team leads [♠] |
| ra-diamond-led-win-x | R | hybrid | 3 | 3748 | 4.6 [3.4, 5.8] | 3.2 | 1.4 | 17.8 | 79.2 | 79.3 | 8.6 |  | ×1.5 contract multiplier when your team wins a trick led with [♦] |
| rb-streak-contract-mult | R | mult | 4 | 3155 | 4.4 [3.2, 5.6] | 2.5 | 1.9 | 15.1 | 82.7 | 83.0 | 4.7 |  | +1 contract multiplier for each trick in your team's contract when your team wins consecutive tricks |
| ra-rainbow-first-x | R | xmult | 4 | 4061 | 4.0 [2.8, 5.2] | 1.6 | 2.4 | 15.7 | 98.8 | 98.8 | 6.8 |  | ×1.25 contract multiplier when your team wins its first trick with each suit |
| control-rare | R | points | 1 | 2439 | 3.5 [2.6, 4.4] | 2.5 | 1.0 | 12.5 | 100.0 |  | 3.9 |  | +80 contract points |
| rc-low-win-contract-points | R | points | 6 | 37854 | 3.1 [2.2, 4.1] | 1.9 | 1.3 | 20.3 | 81.3 | 81.3 | 6.7 |  | +10 contract points for each trick in your team's contract when your team wins a trick with a [2] through [9] |
| rb-honors-free | R | enabler | 4 | 13358 | 3.0 [1.8, 4.2] | -1.5 | 4.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play [10]s through [A]s even when it can follow suit |
| seed-two-win | R | points | 4 | 37174 | 3.0 [2.0, 3.9] | 2.7 | 0.3 | 16.8 | 45.9 | 51.7 | 5.2 |  | +135 contract points when your team wins a trick with a [2] |
| rb-outbid-x | R | xmult | 2 | 3253 | 1.7 [0.5, 2.9] | -2.3 | 3.9 | 16.4 | 41.0 | 42.9 | 2.9 | lift<band | ×3 contract multiplier when your team bids more than the opponents |
| ra-side-win-points | R | points | 3 | 4047 | 1.6 [0.5, 2.7] | 1.0 | 0.6 | 12.2 | 91.9 | 92.3 | 4.4 |  | +25 contract points when your team wins a trick with a card other than [♠] |
| seed-flat-x | R | xmult | 1 | 4221 | 1.0 [-0.1, 2.2] | -0.1 | 1.2 | 16.5 | 100.0 |  | 5.0 |  | ×2 contract multiplier |
| seed-rainbow-x | R | xmult | 5 | 3788 | 0.8 [-0.7, 2.4] | -0.6 | 1.4 | 15.1 | 31.2 | 36.3 | 2.6 |  | ×3 contract multiplier if your team wins tricks with all four suits |
| rb-opp-win-mult | R | mult | 2 | 7938 | 0.7 [-0.4, 1.7] | -1.0 | 1.6 | 17.7 | 99.4 | 99.4 | 5.7 |  | +4 contract multiplier when the opponents win a trick |
| ra-diamond-hold-x | R | xmult | 3 | 3163 | 0.7 [-1.0, 2.4] | 0.2 | 0.5 | 12.6 | 100.0 | 100.0 | 5.2 |  | ×1.1 contract multiplier for each [♦] your team holds |
| ra-spade-win-contract | R | mult | 4 | 25447 | -2.6 [-3.6, -1.7] | -3.0 | 0.4 | 15.8 | 91.6 | 91.7 | 5.3 | lift<band | +1 contract multiplier for each trick in your team's contract when your team wins a trick with [♠] |
| rc-low-lead-x | R | xmult | 5 | 27105 | -2.8 [-3.7, -1.9] | -2.2 | -0.6 | 17.5 | 88.2 | 88.2 | 8.7 | lift<band | ×1.25 contract multiplier when your team leads a [2] through [6] |
| rc-low-untrumpable | R | enabler | 5 | 29250 | -5.6 [-6.6, -4.6] | -3.2 | -2.3 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team's [2]s through [10]s can't be trumped |
| rb-trailing-x | R | xmult | 2 | 3873 | -6.2 [-7.2, -5.1] | -4.9 | -1.3 | 13.9 | 50.5 |  | 1.7 | lift<band | ×2 contract multiplier if your team is behind when the round begins |
| rc-opp-aces-fall | R | enabler | 6 | 8603 | -13.1 [-14.3, -11.9] | -8.5 | -4.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [A] the opponents hold becomes a [2] |
| rc-twos-always-win | L | enabler | 5 | 68519 | 31.4 [30.5, 32.4] | 15.9 | 15.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift>ceiling | Your team's [2]s win every trick they are played to |
| ra-team-untrumpable | L | enabler | 2 | 29893 | 17.6 [16.6, 18.6] | 11.4 | 6.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's cards can't be trumped |
| ra-ace-win-x | L | xmult | 4 | 15543 | 9.5 [8.5, 10.5] | 8.8 | 0.7 | 32.1 | 91.0 | 91.2 | 12.5 |  | ×1.5 contract multiplier when your team wins a trick with an [A] |
| rb-contract-trick-x | L | xmult | 2 | 3341 | 4.5 [3.3, 5.8] | 1.3 | 3.2 | 21.4 | 100.0 | 100.0 | 8.4 |  | ×1.2 contract multiplier for each trick in your team's contract |
| control-legendary | L | points | 1 | 2411 | 4.0 [3.2, 4.8] | 2.8 | 1.2 | 15.2 | 100.0 |  | 7.2 |  | +120 contract points |
| seed-flat-x2 | L | xmult | 1 | 2516 | 3.5 [2.2, 4.8] | 1.7 | 1.8 | 18.3 | 100.0 |  | 7.1 |  | ×2.5 contract multiplier |
| rc-low-win-x | L | xmult | 5 | 84734 | 1.7 [0.8, 2.5] | -0.1 | 1.7 | 20.8 | 64.1 | 64.4 | 8.1 |  | ×1.5 contract multiplier when your team wins a trick with a [2] through [6] |
