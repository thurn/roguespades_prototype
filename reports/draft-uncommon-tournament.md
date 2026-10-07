# Draft pass: draft-uncommon — tournament dashboard

Fit: 105255 boards, alpha 8, residual sd 0.281 (u units), win calibration k = 1.85, smoothing scale 25128 points.
Tier agreement: 109 sigils, lift correlation 0.25 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 3735488 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| control-common | C | points | 1 | 3158 | 6.5 [5.7, 7.3] | 2.8 | 3.7 | 6.1 | 100.0 |  | 6.5 |  | +40 contract points |
| ub-consecutive-contract-points | U | hybrid | 4 | 7547 | 16.7 [15.7, 17.7] | 12.8 | 3.9 | 25.2 | 88.1 | 88.4 | 13.0 |  | +5 contract points for each trick in your team's contract when your team wins consecutive tricks |
| uc-low-discard | U | enabler | 4 | 17079 | 13.1 [12.2, 14.1] | 4.1 | 9.0 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team can play [2]s through [5]s even when it can follow suit |
| uc-twos-beat | U | enabler | 4 | 12873 | 11.4 [10.3, 12.5] | 5.5 | 5.9 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s beat every other card of their suit |
| uc-nil-low-hold | U | points | 5 | 10460 | 10.1 [9.1, 11.1] | 4.1 | 6.0 | 23.0 | 100.0 | 100.0 | 9.8 |  | +20 nil points for each [2] through [6] your team holds |
| ua-rainbow-first-contract | U | points | 5 | 7573 | 9.5 [8.5, 10.5] | 6.5 | 3.0 | 21.7 | 99.6 | 99.6 | 10.7 |  | +5 contract points for each trick in your team's contract when your team wins its first trick with each suit |
| control-uncommon | U | points | 1 | 3167 | 7.7 [6.9, 8.4] | 5.0 | 2.7 | 7.5 | 100.0 |  | 7.4 |  | +60 contract points |
| ub-low-to-aces | U | enabler | 8 | 12101 | 7.6 [6.5, 8.7] | 4.3 | 3.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three [2]s through [9]s your team holds become [A]s |
| ua-become-three-aces | U | enabler | 5 | 9794 | 7.3 [6.2, 8.4] | 1.8 | 5.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [A]s |
| ua-high-spade-win | U | points | 6 | 9010 | 4.5 [3.4, 5.5] | 5.8 | -1.3 | 22.1 | 88.8 | 89.2 | 10.2 |  | +40 contract points when your team wins a trick with a [J] through [A] of [♠] |
| ua-diamond-honor-win | U | points | 6 | 9207 | 2.8 [1.8, 3.7] | -2.0 | 4.8 | 12.9 | 61.4 | 62.2 | 6.8 | lift<band | +40 contract points when your team wins a trick with a [J] through [A] of [♦] |
| ub-high-honor-hold-mult | U | mult | 5 | 9979 | 1.0 [-0.0, 2.0] | -1.6 | 2.6 | 15.4 | 100.0 | 100.0 | 6.9 | lift<band | +2 contract multiplier for each [J] through [A] your team holds |
| uc-junk-to-aces | U | enabler | 8 | 12545 | 0.9 [-0.1, 1.9] | -1.0 | 1.9 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two [2]s through [6]s your team holds become [A]s |
| ua-twos-become-aces | U | enabler | 6 | 9874 | -0.2 [-1.3, 0.8] | -1.3 | 1.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [2] your team holds becomes an [A] |
| ub-consecutive-xmult | U | xmult | 3 | 6073 | -0.7 [-2.1, 0.6] | -1.4 | 0.6 | 15.6 | 84.9 | 85.2 | 7.3 |  | ×1.1 contract multiplier when your team wins consecutive tricks |
| ua-diamonds-untrumpable | U | enabler | 3 | 9367 | -0.9 [-1.9, 0.2] | -2.1 | 1.3 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team's [♦]s can't be trumped |
| uc-low-lead-mult | U | hybrid | 5 | 10453 | -1.2 [-2.2, -0.2] | -2.0 | 0.8 | 13.9 | 90.3 | 90.1 | 6.6 | lift<band | +5 contract multiplier when your team leads a [2] through [6] |
| ub-bid8-trick-points | U | points | 4 | 5201 | -1.3 [-2.4, -0.2] | -2.6 | 1.3 | 15.7 | 38.0 | 41.0 | 6.1 | lift<band | +20 contract points for each trick in your team's contract when your team bids 8 or more |
| ub-bid7-points | U | points | 3 | 5350 | -1.7 [-2.7, -0.7] | -3.1 | 1.5 | 15.2 | 55.2 | 58.0 | 5.8 | lift<band | +100 contract points when your team bids 7 or more |
| seed-become-king | U | enabler | 5 | 9760 | -1.9 [-3.1, -0.8] | -3.5 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [K]s |
| ua-spade-last-x | U | hybrid | 4 | 8968 | -2.5 [-3.5, -1.5] | -3.0 | 0.5 | 17.4 | 41.6 | 41.9 | 4.7 | lift<band | ×2 contract multiplier when your team wins the last trick of a round with [♠] |
| ub-become-four-spades | U | enabler | 5 | 13427 | -2.7 [-3.7, -1.7] | -3.1 | 0.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards other than [♠] your team holds become [♠]s |
| ub-four-row-points | U | points | 5 | 6073 | -2.7 [-4.1, -1.3] | -0.7 | -2.0 | 15.6 | 39.2 | 40.7 | 6.7 |  | +100 contract points when your team wins four tricks in a row |
| uc-low-hold-mult | U | mult | 5 | 10491 | -3.0 [-4.0, -1.9] | -3.0 | 0.0 | 12.7 | 100.0 | 100.0 | 6.0 | lift<band | +2 contract multiplier for each [2] through [4] your team holds |
| ub-make8-trick-mult | U | mult | 5 | 5174 | -3.1 [-4.2, -2.0] | -4.4 | 1.3 | 11.5 | 15.5 | 16.8 | 4.1 | lift<band | +3 contract multiplier for each trick in your team's contract if your team makes a contract of 8 or more |
| ua-rainbow-first-mult | U | mult | 4 | 5938 | -4.2 [-5.5, -2.8] | -3.2 | -0.9 | 12.2 | 99.5 | 99.7 | 5.3 | lift<band | +4 contract multiplier when your team wins its first trick with each suit |
| ua-diamond-hold-mult | U | mult | 3 | 6877 | -4.2 [-5.7, -2.7] | -5.0 | 0.8 | 12.2 | 100.0 | 100.0 | 5.4 | lift<band | +2 contract multiplier for each [♦] your team holds |
| uc-nil-cover-points | U | hybrid | 2 | 9535 | -4.6 [-5.8, -3.4] | -2.3 | -2.3 | 17.2 | 99.5 | 99.4 | 6.6 | lift<band | +25 nil points when your team wins a trick |
| ub-first-trick-contract-points | U | hybrid | 4 | 7751 | -4.6 [-5.6, -3.7] | -0.2 | -4.4 | 15.6 | 52.4 | 52.7 | 7.7 |  | +15 contract points for each trick in your team's contract when your team wins the first trick of a round |
| uc-trump-freely | U | enabler | 2 | 14185 | -4.7 [-5.8, -3.7] | -4.2 | -0.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play [♠]s even when it can follow suit |
| ua-ace-hold-mult | U | mult | 4 | 8137 | -5.0 [-6.1, -4.0] | -4.0 | -1.1 | 10.8 | 96.6 | 97.0 | 4.9 | lift<band | +4 contract multiplier for each [A] your team holds |
| ua-spade-led-win-mult | U | hybrid | 3 | 8352 | -5.1 [-6.1, -4.1] | -6.0 | 0.9 | 10.9 | 80.4 | 80.7 | 4.2 | lift<band | +4 contract multiplier when your team wins a trick led with [♠] |
| seed-spade-hold | U | points | 3 | 6707 | -5.5 [-6.8, -4.1] | -3.1 | -2.4 | 13.4 | 100.0 | 100.0 | 5.1 | lift<band | +5 contract points for each [♠] your team holds |
| ub-honor-win-mult | U | mult | 5 | 10023 | -5.6 [-6.6, -4.6] | -4.7 | -0.8 | 10.9 | 98.2 | 98.3 | 4.1 | lift<band | +2 contract multiplier when your team wins a trick with a [J] through [A] |
| uc-two-grow-points | U | points | 6 | 7232 | -5.8 [-7.1, -4.4] | -5.6 | -0.1 | 6.5 | 24.8 | 34.3 | 2.5 | lift<band | This sigil gains +20 contract points every time your team wins a trick with a [2] (currently +0) |
| seed-nil-made-x | U | xmult | 2 | 6798 | -5.9 [-7.2, -4.7] | -4.5 | -1.4 | 11.2 | 25.1 | 27.2 | 3.5 | lift<band | ×1.5 contract multiplier if your team makes a nil |
| seed-rainbow-lead-x | U | xmult | 5 | 5886 | -6.7 [-8.1, -5.4] | -5.5 | -1.3 | 9.9 | 42.8 | 45.8 | 2.8 | lift<band | ×1.5 contract multiplier if your team leads all four suits |
| ub-bid5-xmult | U | hybrid | 3 | 14055 | -6.9 [-7.9, -5.9] | -5.4 | -1.5 | 9.1 | 48.5 | 49.5 | 3.9 | lift<band | ×1.5 contract multiplier when your team bids 5 or less |
| ub-make7-xmult | U | xmult | 4 | 7976 | -6.9 [-7.9, -5.8] | -5.3 | -1.6 | 10.9 | 25.1 | 26.3 | 4.6 | lift<band | ×1.5 contract multiplier if your team makes a contract of 7 or more |
| seed-flat-mult | U | mult | 1 | 6365 | -7.1 [-8.0, -6.2] | -4.8 | -2.3 | 10.1 | 100.0 |  | 4.6 | lift<band | +10 contract multiplier |
| ub-trick-xmult | U | xmult | 2 | 6334 | -7.1 [-8.1, -6.1] | -3.0 | -4.1 | 13.9 | 99.5 |  | 6.3 | lift<band | ×1.05 contract multiplier when your team wins a trick |
| uc-spade-lead-mult | U | hybrid | 3 | 8992 | -7.3 [-8.3, -6.3] | -5.6 | -1.7 | 10.4 | 79.5 | 79.9 | 4.0 | lift<band | +4 contract multiplier when your team leads [♠] |
| uc-low-win-mult | U | hybrid | 5 | 3420 | -7.6 [-8.9, -6.3] | -5.2 | -2.4 | 11.1 | 79.7 | 81.0 | 4.0 | lift<band | +4 contract multiplier when your team wins a trick with a [2] through [9] |
| uc-clubs-to-spades | U | enabler | 5 | 11473 | -7.9 [-8.9, -6.9] | -5.6 | -2.3 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four [♣]s your team holds become [♠]s |
| ua-ace-win-contract | U | mult | 5 | 7494 | -8.2 [-9.2, -7.2] | -4.3 | -4.0 | 14.0 | 87.0 | 87.3 | 3.8 | lift<band | +1 contract multiplier for each trick in your team's contract when your team wins a trick with an [A] |
| ua-lead-three-suits-x | U | hybrid | 5 | 6091 | -8.5 [-9.9, -7.1] | -4.4 | -4.1 | 12.7 | 81.9 | 83.9 | 5.5 | lift<band | ×1.5 contract multiplier if your team leads three suits |
| ub-first-trick-xmult | U | hybrid | 3 | 8780 | -8.7 [-9.9, -7.4] | -5.2 | -3.4 | 9.7 | 50.1 | 50.7 | 3.2 | lift<band | ×1.5 contract multiplier when your team wins the first trick of a round |
| ua-side-win-mult | U | mult | 3 | 11479 | -9.0 [-9.9, -8.1] | -4.3 | -4.7 | 11.1 | 93.3 | 93.6 | 5.3 | lift<band | +3 contract multiplier when your team wins a trick with a card other than [♠] |
| seed-ace-three | U | xmult | 7 | 2049 | -9.1 [-10.8, -7.3] | -7.9 | -1.1 | 6.7 | 29.0 | 34.6 | 2.1 | lift<band | ×1.5 contract multiplier when your team wins three tricks with [A]s |
| seed-four-row | U | mult | 5 | 5955 | -9.1 [-10.7, -7.6] | -4.9 | -4.3 | 12.7 | 36.3 | 36.9 | 3.9 | lift<band | +20 contract multiplier when your team wins four tricks in a row |
| seed-e-side-four | U | mult | 6 | 3042 | -9.4 [-10.8, -7.9] | -6.1 | -3.3 | 8.6 | 32.1 | 34.1 | 2.8 | lift<band | +15 contract multiplier when your team wins four tricks with cards other than [♠] |
| ub-seven-tricks-xmult | U | xmult | 5 | 8128 | -9.6 [-10.7, -8.5] | -5.1 | -4.4 | 11.2 | 42.3 | 44.0 | 4.5 | lift<band | ×1.5 contract multiplier when your team wins seven tricks |
| ua-three-aces-x | U | xmult | 6 | 5894 | -9.6 [-10.9, -8.3] | -7.6 | -2.0 | 9.5 | 51.8 | 59.8 | 2.3 | lift<band | ×1.5 contract multiplier if your team holds three [A]s |
| ua-kings-become-aces | U | enabler | 6 | 3515 | -9.6 [-10.9, -8.3] | -6.4 | -3.2 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [K] your team holds becomes an [A] |
| uc-low-spade-win | U | hybrid | 6 | 3335 | -9.6 [-11.0, -8.3] | -6.6 | -3.0 | 12.5 | 73.0 | 74.0 | 4.8 | lift<band | +6 contract multiplier when your team wins a trick with a [2] through [9] of [♠] |
| uc-low-lead-nil | U | hybrid | 5 | 3299 | -10.1 [-11.4, -8.8] | -6.6 | -3.5 | 11.9 | 90.7 | 90.3 | 4.3 | lift<band | +50 nil points when your team leads a [2] through [6] |
| ub-four-row-xmult | U | xmult | 5 | 2030 | -10.2 [-12.3, -8.2] | -7.3 | -2.9 | 9.7 | 35.7 | 36.3 | 2.9 | lift<band | ×1.5 contract multiplier when your team wins four tricks in a row |
| ua-trump-two-x | U | hybrid | 6 | 2212 | -10.3 [-12.4, -8.2] | -8.0 | -2.3 | 10.8 | 52.7 | 54.8 | 1.8 | lift<band | ×1.5 contract multiplier when your team wins two tricks by trumping |
| ua-king-two-x | U | xmult | 7 | 5821 | -10.3 [-11.6, -9.1] | -8.4 | -1.9 | 7.9 | 28.7 | 28.4 | 2.6 | lift<band | ×1.5 contract multiplier when your team wins two tricks with [K]s |
| ua-diamond-led-win-mult | U | hybrid | 3 | 3179 | -10.3 [-11.7, -8.9] | -7.3 | -3.0 | 8.9 | 81.7 | 82.1 | 2.7 | lift<band | +4 contract multiplier when your team wins a trick led with [♦] |
| ua-aces-untrumpable | U | enabler | 4 | 13189 | -10.4 [-11.5, -9.3] | -6.7 | -3.6 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team's [A]s can't be trumped |
| ub-three-row-contract-mult | U | hybrid | 6 | 7692 | -10.8 [-11.7, -9.8] | -5.5 | -5.2 | 11.3 | 58.7 | 58.9 | 4.6 | lift<band | +2 contract multiplier for each trick in your team's contract when your team wins three tricks in a row |
| ua-diamond-two-x | U | xmult | 6 | 2088 | -10.8 [-12.9, -8.8] | -7.6 | -3.3 | 6.8 | 25.3 | 26.4 | 1.1 | lift<band, flat slope | ×1.5 contract multiplier when your team wins two tricks with [♦] |
| ua-spade-seven-x | U | xmult | 5 | 2754 | -10.9 [-12.2, -9.6] | -7.3 | -3.6 | 9.3 | 59.1 | 60.4 | 3.4 | lift<band | ×1.5 contract multiplier if your team holds seven [♠]s |
| ub-exact-trick-xmult | U | xmult | 3 | 3592 | -11.0 [-12.2, -9.8] | -7.8 | -3.2 | 14.9 | 24.4 | 24.3 | 3.7 | lift<band | ×1.1 contract multiplier for each trick in your team's contract if your team makes its contract exactly |
| uc-led-low-win | U | hybrid | 5 | 4137 | -11.4 [-12.5, -10.3] | -7.7 | -3.7 | 7.4 | 88.0 | 87.8 | 2.0 | lift<band | +3 contract multiplier when your team wins a trick led with a [2] through [6] |
| uc-low-last-x | U | xmult | 6 | 3036 | -11.4 [-12.7, -10.1] | -9.5 | -2.0 | 9.4 | 27.0 | 27.3 | 1.9 | lift<band | ×1.5 contract multiplier when your team wins the last trick of a round with a [2] through [10] |
| uc-three-twos-x | U | xmult | 6 | 3186 | -11.6 [-12.9, -10.3] | -8.8 | -2.8 | 5.8 | 28.6 | 30.1 | 2.2 | lift<band | ×1.5 contract multiplier if your team holds three [2]s |
| ub-last-trick-contract-mult | U | hybrid | 4 | 2556 | -11.7 [-13.1, -10.4] | -7.1 | -4.7 | 10.5 | 47.3 | 47.6 | 3.1 | lift<band | +2 contract multiplier for each trick in your team's contract when your team wins the last trick of a round |
| ub-outbid-mult | U | hybrid | 2 | 1634 | -11.8 [-13.6, -10.1] | -6.7 | -5.1 | 10.4 | 44.3 | 46.1 | 3.0 | lift<band | +20 contract multiplier when your team bids more than the opponents |
| ub-contract-trick-xmult | U | hybrid | 2 | 2701 | -11.9 [-13.3, -10.5] | -8.2 | -3.7 | 11.0 | 100.0 | 100.0 | 2.5 | lift<band | ×1.05 contract multiplier for each trick in your team's contract |
| uc-low-lead-three-x | U | hybrid | 8 | 5188 | -12.0 [-13.0, -10.9] | -8.1 | -3.8 | 9.5 | 43.1 | 42.0 | 3.3 | lift<band | ×1.5 contract multiplier when your team leads a [2] through [6] three times |
| seed-four-aces | U | mult | 6 | 1996 | -12.0 [-14.1, -10.0] | -7.6 | -4.4 | 5.8 | 24.7 | 34.9 | 1.9 | lift<band | +20 contract multiplier if your team holds four [A]s |
| ub-opp-set-xmult | U | hybrid | 2 | 3966 | -12.0 [-13.9, -10.1] | -8.6 | -3.5 | 6.0 | 35.9 | 33.6 | 1.3 | lift<band | ×1.5 contract multiplier if the opponents miss their contract |
| ub-make8-grow-x | U | xmult | 6 | 1796 | -12.1 [-13.5, -10.7] | -9.7 | -2.4 | 5.8 | 14.2 | 15.4 | -2.6 | lift<band | This sigil gains ×0.5 contract multiplier every time your team makes a contract of 8 or more (currently ×1) |
| ub-exact-nil-points | U | points | 2 | 4019 | -12.3 [-13.8, -10.9] | -8.9 | -3.4 | 7.4 | 24.5 | 24.7 | 1.6 | lift<band | +200 nil points if your team makes its contract exactly |
| ua-heart-hold-mult | U | mult | 3 | 6924 | -12.3 [-13.7, -11.0] | -3.5 | -8.8 | 12.7 | 100.0 | 100.0 | 6.8 | lift<band | +2 contract multiplier for each [♥] your team holds |
| ua-ace-first-x | U | hybrid | 5 | 2882 | -12.4 [-13.5, -11.2] | -8.1 | -4.3 | 7.5 | 34.0 | 34.4 | 2.6 | lift<band | ×1.5 contract multiplier when your team wins the first trick of a round with an [A] |
| uc-small-bid-x | U | hybrid | 3 | 4041 | -12.4 [-13.8, -11.0] | -8.2 | -4.2 | 6.0 | 32.2 | 32.9 | 1.2 | lift<band | ×1.5 contract multiplier when your team bids 4 or less |
| ua-queen-two-x | U | hybrid | 7 | 1988 | -12.6 [-14.4, -10.7] | -10.2 | -2.3 | 5.1 | 18.0 | 16.9 | 0.2 | lift<band, flat slope | ×1.5 contract multiplier when your team wins two tricks with [Q]s |
| ua-four-kings-x | U | xmult | 6 | 2956 | -12.6 [-14.1, -11.2] | -9.9 | -2.7 | 4.8 | 11.0 | 12.1 | 1.5 | lift<band | ×2 contract multiplier if your team holds four [K]s |
| ub-exact-grow-x | U | xmult | 4 | 3027 | -12.8 [-14.8, -10.7] | -10.1 | -2.6 | 11.0 | 26.3 | 25.6 | -2.2 | lift<band | This sigil gains ×0.5 contract multiplier every time your team makes its contract exactly (currently ×1) |
| ub-opp-set-contract-mult | U | hybrid | 3 | 2711 | -12.8 [-14.3, -11.3] | -8.6 | -4.2 | 6.3 | 36.6 | 39.8 | 3.1 | lift<band | +2 contract multiplier for each trick in your team's contract if the opponents miss their contract |
| ua-diamond-lead-three-x | U | hybrid | 6 | 3083 | -12.9 [-14.2, -11.6] | -9.6 | -3.3 | 7.1 | 25.0 | 25.0 | 2.1 | lift<band | ×1.5 contract multiplier when your team leads [♦] three times |
| uc-low-three-x | U | hybrid | 8 | 4265 | -13.1 [-14.2, -12.0] | -8.8 | -4.3 | 8.2 | 26.0 | 26.9 | 1.6 | lift<band | ×1.5 contract multiplier when your team wins three tricks with [2]s through [9]s |
| seed-lead-choice | U | enabler | 2 | 3997 | -13.7 [-14.9, -12.5] | -9.7 | -4.0 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Choose which partner leads after your team wins a trick |
| uc-exact-contract-points | U | points | 3 | 3602 | -13.9 [-15.2, -12.6] | -8.3 | -5.6 | 17.8 | 25.1 | 25.4 | 3.4 | lift<band | +20 contract points for each trick in your team's contract if your team makes its contract exactly |
| ua-trump-contract-mult | U | mult | 4 | 2666 | -13.9 [-15.2, -12.7] | -6.9 | -7.1 | 12.7 | 79.9 | 80.0 | 2.5 | lift<band | +1 contract multiplier for each trick in your team's contract when your team wins a trick by trumping |
| uc-nil-last-cover | U | hybrid | 3 | 3086 | -14.7 [-15.9, -13.6] | -9.4 | -5.3 | 8.5 | 46.0 | 46.1 | 1.5 | lift<band | +120 nil points when your team wins the last trick of a round |
| seed-trump-three | U | mult | 6 | 2191 | -15.1 [-17.3, -12.9] | -9.3 | -5.8 | 6.6 | 26.1 | 28.4 | 1.8 | lift<band | +15 contract multiplier when your team wins three tricks by trumping |
| uc-nil-contract-mult | U | hybrid | 3 | 2757 | -15.4 [-16.6, -14.2] | -9.3 | -6.2 | 7.2 | 40.0 | 40.4 | 2.0 | lift<band | +2 contract multiplier for each trick in your team's contract when your team bids nil |
| seed-exact-x | U | xmult | 2 | 3042 | -15.5 [-17.6, -13.3] | -11.7 | -3.8 | 9.3 | 24.4 | 25.0 | 0.2 | lift<band, flat slope | ×1.5 contract multiplier if your team makes its contract exactly |
| uc-exact-grow-points | U | points | 4 | 3009 | -15.6 [-17.6, -13.5] | -9.4 | -6.2 | 9.7 | 25.7 | 24.8 | 0.7 | lift<band, flat slope | This sigil gains +30 contract points every time your team makes its contract exactly (currently +0) |
| ua-side-raise | U | enabler | 5 | 4535 | -16.5 [-18.0, -15.1] | -11.6 | -4.9 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every card other than [♠] your team holds by two ranks |
| seed-diamond-three | U | mult | 6 | 2097 | -16.9 [-18.7, -15.1] | -10.9 | -6.0 | 2.2 | 5.5 | 7.0 | 1.1 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins three tricks with [♦] |
| ua-rainbow-contract | U | mult | 6 | 2560 | -17.2 [-18.4, -16.0] | -9.7 | -7.4 | 7.9 | 29.0 | 29.1 | 1.1 | lift<band, flat slope | +2 contract multiplier for each trick in your team's contract if your team wins tricks with all four suits |
| ub-bid8-xmult | U | hybrid | 3 | 1718 | -17.3 [-19.0, -15.5] | -10.2 | -7.1 | 11.8 | 31.7 | 34.5 | 4.4 | lift<band | ×2 contract multiplier when your team bids 8 or more |
| uc-low-to-twos | U | enabler | 7 | 5336 | -17.6 [-18.7, -16.5] | -11.1 | -6.6 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [3] through [6] your team holds becomes a [2] |
| seed-e-last-heart | U | mult | 4 | 3086 | -17.7 [-18.9, -16.5] | -10.8 | -6.9 | 1.7 | 3.4 | 3.3 | 0.0 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins the last trick of a round with [♥] |
| uc-opp-twos | U | enabler | 5 | 2785 | -17.9 [-19.2, -16.6] | -11.2 | -6.7 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Three cards the opponents hold become [2]s |
| ua-diamond-eight-contract | U | mult | 6 | 2773 | -18.3 [-19.5, -17.2] | -9.4 | -9.0 | 4.4 | 23.5 | 22.7 | 1.8 | lift<band | +2 contract multiplier for each trick in your team's contract if your team holds eight [♦]s |
| ua-low-become-spades | U | enabler | 6 | 3573 | -18.8 [-20.2, -17.4] | -13.0 | -5.8 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [2] through [4] your team holds becomes a [♠] |
| uc-nil-grow-mult | U | mult | 4 | 2198 | -18.9 [-20.7, -17.0] | -10.3 | -8.5 | 3.2 | 24.2 | 26.7 | 0.1 | lift<band, flat slope | This sigil gains +4 contract multiplier every time your team makes a nil (currently +0) |
| seed-e-heart-three | U | mult | 6 | 2244 | -19.7 [-21.6, -17.8] | -12.4 | -7.3 | 2.1 | 5.2 | 5.4 | 0.6 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins three tricks with [♥] |
| ua-spade-raise | U | enabler | 5 | 3568 | -20.9 [-22.4, -19.5] | -12.4 | -8.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every [♠] your team holds by two ranks |
| seed-low-four | U | mult | 8 | 2322 | -21.1 [-23.0, -19.1] | -10.9 | -10.2 | 5.9 | 17.9 | 21.2 | 1.4 | lift<band | +15 contract multiplier when your team wins four tricks with [2]s through [10]s |
| seed-become-heart | U | enabler | 4 | 4201 | -36.5 [-37.8, -35.1] | -19.6 | -16.8 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [♥]s |
| control-rare | R | points | 1 | 3093 | 15.4 [14.6, 16.1] | 7.7 | 7.7 | 13.6 | 100.0 |  | 11.7 |  | +80 contract points |
| control-legendary | L | points | 1 | 3041 | 22.7 [21.9, 23.5] | 10.7 | 12.0 | 20.1 | 100.0 |  | 13.6 |  | +120 contract points |
