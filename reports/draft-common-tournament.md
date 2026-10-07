# Draft pass: commons — tournament dashboard

Fit: 173964 boards, alpha 8, residual sd 0.237 (u units), win calibration k = 1.95, smoothing scale 25128 points.
Tier agreement: 183 sigils, lift correlation 0.24 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 6181760 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| e-win-suit-h-xmult | C | xmult | 3 | 5730 | 26.7 [25.2, 28.1] | 12.8 | 13.8 | 17.2 | 68.7 | 68.1 | 11.8 |  | ×1.5 contract multiplier when your team wins a trick with [♥] |
| e-win-suit-d-xmult | C | xmult | 3 | 5867 | 25.1 [23.7, 26.6] | 12.0 | 13.1 | 16.5 | 68.8 | 68.3 | 10.9 |  | ×1.5 contract multiplier when your team wins a trick with [♦] |
| e-lead-rank-k-xmult | C | xmult | 4 | 5700 | 23.4 [21.8, 24.9] | 13.5 | 9.9 | 21.4 | 72.9 | 72.7 | 14.1 |  | ×1.5 contract multiplier when your team leads a [K] |
| e-oppset-xmult | C | xmult | 2 | 8119 | 23.1 [22.3, 23.9] | 13.6 | 9.5 | 19.6 | 36.3 |  | 10.1 | lift>ceiling | ×3 contract multiplier if the opponents miss their contract |
| e-lead-suit-d-xmult | C | xmult | 3 | 5885 | 21.2 [19.8, 22.6] | 11.7 | 9.5 | 16.6 | 85.0 | 84.3 | 12.5 |  | ×1.25 contract multiplier when your team leads [♦] |
| e-lead-rank-j-xmult | C | xmult | 4 | 5601 | 21.1 [19.7, 22.6] | 10.8 | 10.4 | 19.4 | 66.1 | 64.2 | 10.8 |  | ×1.5 contract multiplier when your team leads a [J] |
| e-lead-suit-c-xmult | C | xmult | 3 | 5821 | 20.8 [19.5, 22.2] | 11.0 | 9.8 | 16.8 | 84.9 | 82.6 | 12.4 |  | ×1.25 contract multiplier when your team leads [♣] |
| e-lead-suit-h-xmult | C | xmult | 3 | 5910 | 20.7 [19.3, 22.0] | 10.3 | 10.4 | 17.5 | 85.2 | 84.5 | 12.2 |  | ×1.25 contract multiplier when your team leads [♥] |
| cc-nil-made-mult | C | mult | 2 | 9476 | 20.6 [19.5, 21.8] | 14.3 | 6.4 | 23.5 | 38.9 | 41.3 | 11.2 | lift>ceiling | +20 contract multiplier if your team makes a nil |
| e-lead-rank-2-xmult | C | xmult | 4 | 9326 | 19.9 [18.5, 21.3] | 10.9 | 9.0 | 17.0 | 61.8 | 61.7 | 10.0 |  | ×1.5 contract multiplier when your team leads a [2] |
| e-lead-rank-k-mult | C | mult | 4 | 5704 | 18.8 [17.3, 20.3] | 13.1 | 5.6 | 20.9 | 73.9 | 72.6 | 11.2 |  | +10 contract multiplier when your team leads a [K] |
| e-hold-rank-10-xmult | C | xmult | 4 | 5649 | 17.9 [16.6, 19.3] | 10.7 | 7.2 | 16.1 | 94.3 | 94.3 | 9.7 |  | ×1.25 contract multiplier for each [10] your team holds |
| e-lead-rank-q-xmult | C | xmult | 4 | 5732 | 17.2 [15.8, 18.6] | 10.5 | 6.7 | 20.3 | 67.7 | 66.9 | 12.9 |  | ×1.5 contract multiplier when your team leads a [Q] |
| e-win-rank-2-xmult | C | xmult | 4 | 9192 | 16.4 [15.2, 17.7] | 9.2 | 7.2 | 16.4 | 26.0 | 26.8 | 7.0 |  | ×3 contract multiplier when your team wins a trick with a [2] |
| e-last-xmult | C | xmult | 3 | 8073 | 16.2 [14.8, 17.7] | 8.2 | 8.1 | 17.3 | 51.2 | 50.7 | 7.1 |  | ×2 contract multiplier when your team wins the last trick of a round |
| e-win-rank-10-xmult | C | xmult | 4 | 5639 | 16.2 [14.8, 17.7] | 10.3 | 5.9 | 19.0 | 46.9 | 43.0 | 10.2 |  | ×2 contract multiplier when your team wins a trick with a [10] |
| cb-contract-mult | C | mult | 2 | 11456 | 15.3 [14.2, 16.3] | 9.2 | 6.0 | 18.9 | 100.0 | 100.0 | 9.8 |  | +2 contract multiplier for each trick in your team's contract |
| e-hold-rank-10-mult | C | mult | 4 | 5618 | 14.4 [13.0, 15.8] | 7.7 | 6.7 | 14.4 | 94.3 | 94.0 | 7.2 |  | +4 contract multiplier for each [10] your team holds |
| e-lead-rank-a-mult | C | mult | 4 | 5651 | 13.8 [12.3, 15.3] | 8.6 | 5.2 | 15.2 | 80.3 | 80.8 | 9.7 |  | +5 contract multiplier when your team leads an [A] |
| e-win-rank-j-xmult | C | xmult | 4 | 5586 | 13.5 [12.1, 14.9] | 6.2 | 7.3 | 14.4 | 56.0 | 53.3 | 8.7 |  | ×1.5 contract multiplier when your team wins a trick with a [J] |
| e-lead-rank-j-mult | C | mult | 4 | 5679 | 13.0 [11.6, 14.3] | 7.8 | 5.2 | 20.5 | 66.5 | 64.5 | 10.3 |  | +10 contract multiplier when your team leads a [J] |
| seed-nil-bid-mult | C | mult | 2 | 9360 | 12.9 [11.8, 14.0] | 9.3 | 3.6 | 19.2 | 60.6 | 65.6 | 9.9 |  | +15 contract multiplier when your team bids nil |
| cb-consecutive-mult | C | mult | 3 | 8143 | 12.2 [11.0, 13.5] | 7.6 | 4.6 | 14.3 | 89.5 | 89.4 | 8.4 |  | +2 contract multiplier when your team wins consecutive tricks |
| ca-three-aces-mult | C | mult | 6 | 5560 | 12.0 [10.5, 13.5] | 5.9 | 6.1 | 13.7 | 44.1 | 48.1 | 5.9 |  | +15 contract multiplier if your team holds three [A]s |
| e-lead-low-mult | C | mult | 5 | 9251 | 11.7 [10.4, 12.9] | 6.0 | 5.7 | 15.5 | 98.4 | 98.3 | 8.8 |  | +2 contract multiplier when your team leads a [2] through [10] |
| e-hold-rank-j-mult | C | mult | 4 | 5659 | 11.4 [10.0, 12.9] | 8.0 | 3.4 | 15.3 | 95.2 | 94.4 | 8.5 |  | +4 contract multiplier for each [J] your team holds |
| e-lead-suit-h-mult | C | mult | 3 | 5900 | 11.0 [9.6, 12.4] | 6.6 | 4.4 | 14.0 | 84.9 | 83.9 | 8.2 |  | +4 contract multiplier when your team leads [♥] |
| cc-two-hold-mult | C | mult | 4 | 14913 | 10.9 [10.0, 11.9] | 9.4 | 1.5 | 17.7 | 92.9 | 93.4 | 9.2 |  | +5 contract multiplier for each [2] your team holds |
| e-trump-xmult | C | xmult | 3 | 7412 | 10.8 [9.5, 12.2] | 7.4 | 3.5 | 16.1 | 81.0 | 81.9 | 8.5 |  | ×1.25 contract multiplier when your team wins a trick by trumping |
| ca-ace-win-two-mult | C | mult | 7 | 5638 | 10.7 [9.5, 12.0] | 4.8 | 5.9 | 13.4 | 62.0 | 62.9 | 6.6 |  | +10 contract multiplier when your team wins two tricks with [A]s |
| e-win-rank-10-mult | C | mult | 4 | 5554 | 10.7 [9.4, 12.0] | 7.5 | 3.2 | 17.5 | 46.8 | 43.8 | 9.5 |  | +15 contract multiplier when your team wins a trick with a [10] |
| e-hold-suit-s-mult | C | mult | 3 | 7261 | 10.7 [9.4, 11.9] | 4.4 | 6.3 | 13.2 | 100.0 | 100.0 | 7.5 |  | +1 contract multiplier for each [♠] your team holds |
| cb-become-spade | C | enabler | 6 | 15305 | 10.3 [9.6, 11.1] | 5.5 | 4.8 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [♠]s |
| e-hold-rank-a-mult | C | mult | 4 | 5845 | 10.1 [8.8, 11.4] | 5.1 | 5.0 | 13.6 | 96.5 | 96.9 | 7.1 |  | +3 contract multiplier for each [A] your team holds |
| e-win-rank-k-xmult | C | xmult | 4 | 5681 | 9.8 [8.3, 11.3] | 5.1 | 4.7 | 12.1 | 76.1 | 74.0 | 6.7 |  | ×1.25 contract multiplier when your team wins a trick with a [K] |
| seed-last-trick | C | mult | 3 | 8154 | 9.7 [8.4, 11.1] | 5.5 | 4.2 | 17.0 | 51.3 | 51.3 | 6.7 |  | +15 contract multiplier when your team wins the last trick of a round |
| e-lead-suit-s-mult | C | mult | 3 | 7250 | 9.7 [8.5, 11.0] | 8.4 | 1.3 | 18.8 | 84.4 | 85.1 | 8.6 |  | +4 contract multiplier when your team leads [♠] |
| e-lead-suit-c-mult | C | mult | 3 | 5833 | 9.7 [8.4, 11.0] | 5.4 | 4.3 | 15.2 | 84.6 | 81.1 | 7.7 |  | +4 contract multiplier when your team leads [♣] |
| e-hold-suit-d-mult | C | mult | 3 | 5918 | 9.1 [7.8, 10.3] | 5.1 | 3.9 | 12.5 | 100.0 | 100.0 | 7.5 |  | +1 contract multiplier for each [♦] your team holds |
| ca-rainbow-first-mult | C | mult | 4 | 8645 | 9.0 [7.8, 10.2] | 4.7 | 4.3 | 13.0 | 99.8 | 99.7 | 6.6 |  | +2 contract multiplier when your team wins its first trick with each suit |
| e-win-side-mult | C | mult | 3 | 10807 | 8.9 [8.0, 9.8] | 5.7 | 3.2 | 12.4 | 96.4 | 96.8 | 7.5 |  | +2 contract multiplier when your team wins a trick with a card other than [♠] |
| e-exact-xmult | C | xmult | 2 | 18893 | 8.6 [7.3, 10.0] | 4.0 | 4.7 | 31.2 | 26.9 | 25.3 | 7.7 |  | ×3 contract multiplier if your team makes its contract exactly |
| e-lead-rank-j-pts | C | points | 4 | 5795 | 8.6 [7.1, 10.0] | 5.0 | 3.5 | 13.7 | 67.3 | 64.2 | 7.5 |  | +70 contract points when your team leads a [J] |
| e-hold-suit-c-mult | C | mult | 3 | 5815 | 8.5 [7.2, 9.8] | 4.9 | 3.6 | 12.2 | 99.2 | 97.7 | 5.9 |  | +1 contract multiplier for each [♣] your team holds |
| cb-make-mult | C | mult | 2 | 11570 | 8.4 [7.5, 9.4] | 4.0 | 4.4 | 13.8 | 64.4 | 60.1 | 6.1 |  | +6 contract multiplier if your team makes its contract |
| cb-first-trick-mult | C | mult | 3 | 12284 | 8.4 [7.3, 9.6] | 3.8 | 4.6 | 12.8 | 57.0 | 55.9 | 6.7 |  | +10 contract multiplier when your team wins the first trick of a round |
| e-lead-suit-d-mult | C | mult | 3 | 5806 | 8.2 [7.0, 9.5] | 6.4 | 1.8 | 15.0 | 84.7 | 85.2 | 8.4 |  | +4 contract multiplier when your team leads [♦] |
| cb-trick-mult | C | mult | 2 | 11413 | 8.2 [7.2, 9.1] | 6.3 | 1.8 | 13.9 | 99.7 | 99.7 | 7.6 |  | +1 contract multiplier when your team wins a trick |
| cb-spade-seven | C | mult | 5 | 10840 | 8.1 [7.3, 9.0] | 4.4 | 3.8 | 12.7 | 52.0 | 55.4 | 6.0 |  | +12 contract multiplier if your team holds seven [♠]s |
| seed-rainbow-first | C | points | 4 | 8669 | 8.0 [6.7, 9.3] | 5.7 | 2.3 | 13.8 | 99.7 | 99.7 | 6.7 |  | +25 contract points when your team wins its first trick with each suit |
| e-hold-suit-h-mult | C | mult | 3 | 5751 | 7.6 [6.4, 8.8] | 4.3 | 3.4 | 12.0 | 100.0 | 100.0 | 6.6 |  | +1 contract multiplier for each [♥] your team holds |
| e-win-rank-k-mult | C | mult | 4 | 5672 | 7.5 [6.1, 8.8] | 3.0 | 4.5 | 13.8 | 76.7 | 75.4 | 5.9 |  | +5 contract multiplier when your team wins a trick with a [K] |
| seed-diamond-hold | C | points | 3 | 5833 | 7.4 [6.2, 8.6] | 5.6 | 1.8 | 11.8 | 100.0 | 100.0 | 4.6 |  | +10 contract points for each [♦] your team holds |
| cb-contract-mult-made | C | mult | 3 | 11377 | 6.9 [6.0, 7.9] | 3.3 | 3.6 | 13.8 | 61.6 | 59.1 | 5.7 |  | +1 contract multiplier for each trick in your team's contract if your team makes its contract |
| cb-three-row-mult | C | mult | 5 | 8122 | 6.4 [5.0, 7.7] | 4.7 | 1.7 | 11.9 | 62.5 | 61.9 | 6.2 |  | +8 contract multiplier when your team wins three tricks in a row |
| e-win-suit-s-mult | C | mult | 3 | 7171 | 6.2 [5.0, 7.4] | 5.9 | 0.3 | 14.4 | 96.1 | 95.7 | 7.4 |  | +2 contract multiplier when your team wins a trick with [♠] |
| cb-opp-set-mult | C | mult | 2 | 7939 | 5.9 [5.1, 6.6] | 4.4 | 1.5 | 11.2 | 36.2 |  | 6.4 |  | +15 contract multiplier if the opponents miss their contract |
| e-hold-rank-k-mult | C | mult | 4 | 5666 | 5.9 [4.5, 7.2] | 3.6 | 2.3 | 12.8 | 95.3 | 95.6 | 6.5 |  | +3 contract multiplier for each [K] your team holds |
| cc-nil-bid-xmult | C | xmult | 2 | 9388 | 5.6 [4.5, 6.7] | 5.0 | 0.6 | 10.9 | 52.1 | 60.8 | 5.6 |  | ×1.5 contract multiplier when your team bids nil |
| ca-ace-first-trick | C | mult | 5 | 5701 | 5.5 [4.3, 6.8] | 2.0 | 3.5 | 10.8 | 43.9 | 43.3 | 5.4 |  | +10 contract multiplier when your team wins the first trick of a round with an [A] |
| control-common | C | points | 1 | 3875 | 5.3 [4.7, 6.0] | 2.9 | 2.4 | 5.1 | 100.0 |  | 2.7 |  | +40 contract points |
| ca-spade-last-trick | C | mult | 4 | 11815 | 5.3 [4.4, 6.2] | 0.7 | 4.6 | 13.0 | 47.1 | 46.4 | 5.4 |  | +10 contract multiplier when your team wins the last trick of a round with [♠] |
| e-trump-mult | C | mult | 3 | 7266 | 5.2 [4.0, 6.4] | 3.6 | 1.6 | 13.7 | 81.3 | 81.5 | 6.0 |  | +4 contract multiplier when your team wins a trick by trumping |
| ca-rainbow-lead-mult | C | hybrid | 5 | 8671 | 5.1 [3.9, 6.4] | 3.6 | 1.5 | 12.7 | 47.9 | 50.1 | 6.2 |  | +10 contract multiplier if your team leads all four suits |
| e-win-suit-d-mult | C | mult | 3 | 5678 | 5.1 [3.8, 6.4] | 3.4 | 1.7 | 11.4 | 67.9 | 67.7 | 6.5 |  | +5 contract multiplier when your team wins a trick with [♦] |
| cb-make-x | C | xmult | 2 | 7889 | 5.0 [4.2, 5.8] | 2.8 | 2.2 | 10.5 | 63.6 |  | 5.6 |  | ×1.3 contract multiplier if your team makes its contract |
| e-lead-suit-h-pts | C | points | 3 | 5702 | 4.7 [3.4, 6.0] | 2.3 | 2.4 | 9.2 | 84.0 | 84.3 | 4.5 |  | +25 contract points when your team leads [♥] |
| e-win-suit-h-mult | C | mult | 3 | 5857 | 4.7 [3.4, 6.0] | 2.8 | 1.9 | 11.9 | 68.3 | 68.8 | 6.2 |  | +5 contract multiplier when your team wins a trick with [♥] |
| cc-low-lead-mult | C | hybrid | 8 | 15178 | 4.4 [3.6, 5.2] | 5.2 | -0.8 | 17.8 | 54.7 | 51.6 | 8.3 |  | +15 contract multiplier when your team leads a [2] through [6] three times |
| seed-rainbow-mult | C | mult | 5 | 8596 | 4.4 [3.2, 5.5] | 3.6 | 0.7 | 12.0 | 31.2 | 31.2 | 5.0 |  | +15 contract multiplier if your team wins tricks with all four suits |
| cb-grow-make-mult | C | mult | 4 | 7692 | 4.3 [3.5, 5.2] | 0.7 | 3.6 | 8.6 | 65.6 |  | 4.6 |  | This sigil gains +2 contract multiplier every time your team makes its contract (currently +0) |
| e-win-suit-c-mult | C | mult | 3 | 5963 | 4.2 [3.0, 5.5] | 2.0 | 2.2 | 10.9 | 67.4 | 65.0 | 5.7 |  | +5 contract multiplier when your team wins a trick with [♣] |
| cc-small-contract-mult | C | hybrid | 3 | 6377 | 3.6 [2.6, 4.5] | 1.3 | 2.3 | 9.6 | 32.3 | 32.7 | 3.9 |  | +10 contract multiplier when your team bids 4 or less |
| e-first-pts | C | points | 3 | 8083 | 3.4 [2.2, 4.5] | 2.7 | 0.7 | 10.0 | 55.5 | 55.8 | 6.4 |  | +90 contract points when your team wins the first trick of a round |
| ca-spade-lead-three | C | hybrid | 6 | 7297 | 3.0 [1.8, 4.2] | 3.3 | -0.2 | 15.9 | 39.8 | 42.8 | 6.8 |  | +15 contract multiplier when your team leads [♠] three times |
| cb-become-ace | C | enabler | 7 | 14466 | 3.0 [2.2, 3.7] | 3.7 | -0.7 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two cards your team holds become [A]s |
| e-win-rank-10-pts | C | points | 4 | 5721 | 3.0 [1.5, 4.4] | 1.3 | 1.7 | 11.2 | 46.7 | 44.2 | 4.9 |  | +95 contract points when your team wins a trick with a [10] |
| e-lead-rank-10-pts | C | points | 4 | 5607 | 2.8 [1.6, 4.1] | 2.3 | 0.5 | 13.5 | 63.7 | 61.3 | 5.9 |  | +70 contract points when your team leads a [10] |
| ca-raise-one | C | enabler | 6 | 4567 | 2.8 [1.7, 3.8] | 0.3 | 2.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by one rank |
| cb-make8-mult | C | mult | 4 | 2168 | 2.7 [1.1, 4.4] | 0.3 | 2.4 | 14.6 | 20.2 | 22.7 | 3.6 |  | +15 contract multiplier if your team makes a contract of 8 or more |
| cc-nil-contract-trick | C | hybrid | 2 | 12791 | 2.6 [1.8, 3.4] | 3.3 | -0.7 | 12.9 | 100.0 | 100.0 | 5.9 |  | +20 nil points for each trick in your team's contract |
| seed-exact-mult | C | mult | 2 | 4977 | 2.2 [0.3, 4.2] | -0.7 | 2.9 | 20.1 | 27.1 | 22.9 | 3.4 |  | +15 contract multiplier if your team makes its contract exactly |
| ca-spade-hold-eight | C | mult | 5 | 10706 | 2.2 [1.3, 3.1] | 1.9 | 0.3 | 9.7 | 27.8 | 32.5 | 4.5 |  | +15 contract multiplier if your team holds eight [♠]s |
| e-lead-suit-c-pts | C | points | 3 | 1944 | 2.0 [0.4, 3.7] | 1.4 | 0.7 | 9.3 | 83.3 | 82.3 | 4.8 |  | +25 contract points when your team leads [♣] |
| e-lead-side-mult | C | mult | 3 | 10768 | 2.0 [1.1, 2.9] | 1.8 | 0.2 | 10.0 | 99.7 | 99.6 | 6.2 |  | +1 contract multiplier when your team leads a card other than [♠] |
| e-win-suit-c-pts | C | points | 3 | 1868 | 1.8 [0.1, 3.6] | 1.6 | 0.2 | 8.8 | 65.4 | 61.4 | 3.9 |  | +45 contract points when your team wins a trick with [♣] |
| e-lead-side-pts | C | points | 3 | 3696 | 1.8 [0.7, 2.9] | 1.4 | 0.4 | 9.9 | 99.7 | 99.7 | 4.9 |  | +10 contract points when your team leads a card other than [♠] |
| cb-bid8-trick-mult | C | mult | 4 | 2135 | 1.6 [-0.1, 3.4] | -1.5 | 3.1 | 12.8 | 39.1 | 41.9 | 1.6 |  | +2 contract multiplier for each trick in your team's contract when your team bids 8 or more |
| seed-win-points | C | points | 2 | 2665 | 1.6 [0.6, 2.6] | -0.1 | 1.7 | 6.3 | 99.8 |  | 3.9 |  | +5 contract points when your team wins a trick |
| ca-diamond-win-two | C | mult | 6 | 1978 | 1.6 [-0.3, 3.6] | -1.5 | 3.1 | 8.7 | 27.1 | 27.9 | 3.6 |  | +10 contract multiplier when your team wins two tricks with [♦] |
| cc-low-last-mult | C | hybrid | 6 | 13595 | 1.4 [0.6, 2.3] | 0.2 | 1.2 | 15.1 | 33.0 | 32.2 | 4.7 |  | +15 contract multiplier when your team wins the last trick of a round with a [2] through [10] |
| seed-ace-hold | C | points | 4 | 1890 | 1.3 [-0.6, 3.2] | 0.8 | 0.5 | 9.3 | 96.4 | 97.2 | 3.7 |  | +20 contract points for each [A] your team holds |
| cc-nil-bid-points | C | hybrid | 2 | 3783 | 1.1 [0.1, 2.0] | 0.7 | 0.4 | 9.6 | 49.9 | 51.7 | 5.1 |  | +80 contract points when your team bids nil |
| seed-e-lead-three-suits | C | points | 5 | 2933 | 0.9 [-0.7, 2.5] | 0.2 | 0.8 | 9.1 | 85.1 | 86.8 | 3.3 |  | +50 contract points if your team leads three suits |
| cb-honor-hold | C | points | 5 | 3714 | 0.8 [-0.6, 2.1] | 0.0 | 0.7 | 9.1 | 100.0 | 100.0 | 2.5 |  | +5 contract points for each [J] through [A] your team holds |
| cb-bid7-mult | C | mult | 3 | 7085 | 0.7 [-0.4, 1.8] | 0.4 | 0.3 | 10.7 | 57.4 | 59.4 | 4.2 |  | +10 contract multiplier when your team bids 7 or more |
| seed-ace-win | C | points | 4 | 5656 | 0.7 [-0.7, 2.0] | 1.3 | -0.6 | 7.6 | 90.0 | 90.0 | 4.0 |  | +20 contract points when your team wins a trick with an [A] |
| e-hold-rank-j-pts | C | points | 4 | 1982 | 0.5 [-1.4, 2.4] | 0.6 | -0.1 | 7.7 | 94.9 | 94.4 | 4.2 |  | +20 contract points for each [J] your team holds |
| cb-five-row-mult | C | mult | 5 | 11618 | 0.4 [-0.4, 1.3] | 0.4 | 0.1 | 11.1 | 22.2 | 21.5 | 3.6 |  | +15 contract multiplier when your team wins five tricks in a row |
| e-make-pts | C | points | 2 | 2661 | 0.4 [-0.6, 1.4] | 0.8 | -0.4 | 9.8 | 63.9 |  | 4.1 |  | +50 contract points if your team makes its contract |
| e-suits-win-pts | C | points | 5 | 8545 | 0.3 [-0.9, 1.4] | 1.8 | -1.6 | 9.8 | 30.8 | 33.0 | 4.2 |  | +130 contract points if your team wins tricks with all four suits |
| e-hold-low-pts | C | points | 5 | 2747 | 0.3 [-1.6, 2.1] | 0.8 | -0.6 | 10.3 | 100.0 | 100.0 | 4.6 |  | +3 contract points for each [2] through [10] your team holds |
| ca-ace-lead-three | C | hybrid | 7 | 1956 | 0.0 [-1.6, 1.7] | 0.4 | -0.4 | 8.3 | 16.5 | 16.6 | 2.0 |  | +15 contract multiplier when your team leads an [A] three times |
| ca-face-win | C | points | 5 | 1905 | -0.0 [-1.9, 1.9] | -0.3 | 0.3 | 8.0 | 94.3 | 93.9 | 1.7 |  | +15 contract points when your team wins a trick with a [J] through [K] |
| e-win-rank-q-pts | C | points | 4 | 1962 | -0.1 [-2.0, 1.7] | -0.1 | -0.0 | 11.0 | 65.0 | 64.8 | 4.3 |  | +50 contract points when your team wins a trick with a [Q] |
| ca-diamond-hold-eight | C | mult | 5 | 5902 | -0.1 [-1.3, 1.0] | 0.3 | -0.4 | 8.7 | 26.6 | 28.1 | 3.9 |  | +15 contract multiplier if your team holds eight [♦]s |
| e-win-rank-j-pts | C | points | 4 | 1912 | -0.2 [-2.1, 1.7] | -0.4 | 0.3 | 10.7 | 56.0 | 51.3 | 3.5 |  | +65 contract points when your team wins a trick with a [J] |
| ca-heart-hold-eight | C | mult | 5 | 5743 | -0.2 [-1.5, 1.1] | -0.0 | -0.2 | 8.7 | 25.8 | 26.5 | 4.0 |  | +15 contract multiplier if your team holds eight [♥]s |
| ca-high-spade-hold | C | points | 6 | 3023 | -0.3 [-1.4, 0.8] | -0.5 | 0.2 | 9.0 | 98.6 | 98.7 | 3.5 |  | +15 contract points for each [10] through [A] of [♠] your team holds |
| ca-diamond-lead-three | C | hybrid | 6 | 5939 | -0.4 [-1.7, 0.9] | 0.9 | -1.3 | 11.8 | 29.7 | 29.6 | 4.7 |  | +15 contract multiplier when your team leads [♦] three times |
| cc-two-win-mult | C | hybrid | 4 | 3800 | -0.4 [-1.6, 0.8] | 0.5 | -0.9 | 8.9 | 26.2 | 28.3 | 3.0 |  | +10 contract multiplier when your team wins a trick with a [2] |
| cc-low-narrow-win | C | hybrid | 5 | 3904 | -0.5 [-1.6, 0.6] | -0.5 | -0.1 | 7.5 | 71.0 | 71.6 | 3.0 |  | +25 contract points when your team wins a trick with a [2] through [7] |
| cc-low-hold-points | C | points | 5 | 4250 | -0.6 [-1.8, 0.5] | -0.5 | -0.1 | 9.2 | 100.0 | 100.0 | 3.9 |  | +5 contract points for each [2] through [6] your team holds |
| e-hold-rank-2-pts | C | points | 4 | 2763 | -0.7 [-2.2, 0.9] | -0.1 | -0.6 | 8.9 | 92.6 | 93.2 | 3.5 |  | +20 contract points for each [2] your team holds |
| ca-king-hold | C | points | 4 | 1912 | -0.9 [-2.8, 0.9] | -0.9 | 0.0 | 9.1 | 95.2 | 95.4 | 4.4 |  | +20 contract points for each [K] your team holds |
| seed-e-spade-win | C | points | 3 | 3033 | -1.0 [-2.2, 0.1] | -0.9 | -0.2 | 7.3 | 96.2 | 96.5 | 3.5 |  | +10 contract points when your team wins a trick with [♠] |
| seed-low-lead | C | points | 5 | 2658 | -1.1 [-3.1, 0.8] | -1.0 | -0.1 | 8.7 | 98.2 | 97.8 | 2.3 |  | +10 contract points when your team leads a [2] through [10] |
| cc-nil-hold-twos | C | points | 4 | 4410 | -1.2 [-2.3, -0.0] | -0.4 | -0.8 | 6.7 | 92.7 | 93.7 | 3.1 |  | +30 nil points for each [2] your team holds |
| cc-low-spade-win | C | hybrid | 6 | 3978 | -1.2 [-2.2, -0.1] | -0.5 | -0.7 | 8.1 | 80.8 | 81.4 | 3.7 |  | +20 contract points when your team wins a trick with a [2] through [10] of [♠] |
| seed-consecutive | C | points | 3 | 2749 | -1.2 [-2.9, 0.4] | -0.7 | -0.5 | 6.0 | 89.2 | 88.8 | 3.7 |  | +10 contract points when your team wins consecutive tricks |
| ca-ace-win-contract | C | points | 5 | 3003 | -1.2 [-2.4, -0.1] | -1.3 | 0.1 | 7.7 | 89.9 | 89.8 | 3.1 | lift<band | +3 contract points for each trick in your team's contract when your team wins a trick with an [A] |
| e-hold-side-pts | C | points | 3 | 3647 | -1.4 [-2.6, -0.3] | 0.0 | -1.5 | 8.4 | 100.0 | 100.0 | 3.2 |  | +2 contract points for each card other than [♠] your team holds |
| cc-nil-made-points | C | points | 2 | 2851 | -1.5 [-2.9, 0.0] | -1.5 | 0.1 | 5.4 | 31.6 | 37.5 | 1.6 | lift<band | +80 contract points if your team makes a nil |
| ca-become-spades | C | enabler | 6 | 15750 | -1.5 [-2.3, -0.8] | 2.6 | -4.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Two cards your team holds become [♠]s |
| cb-exact-trick-mult | C | mult | 3 | 5920 | -1.5 [-2.5, -0.6] | -3.9 | 2.4 | 18.5 | 26.6 | 25.3 | 2.9 | lift<band | +2 contract multiplier for each trick in your team's contract if your team makes its contract exactly |
| e-make8-pts | C | points | 4 | 2191 | -1.6 [-3.2, 0.0] | -1.1 | -0.5 | 9.6 | 18.8 | 19.7 | 4.1 |  | +150 contract points if your team makes a contract of 8 or more |
| seed-low-win | C | points | 5 | 2782 | -1.6 [-3.5, 0.2] | -2.0 | 0.3 | 7.8 | 88.0 | 87.2 | 2.1 | lift<band | +15 contract points when your team wins a trick with a [2] through [10] |
| seed-contract-points | C | points | 2 | 2148 | -1.6 [-3.2, -0.1] | -2.2 | 0.6 | 6.8 | 100.0 | 100.0 | 1.8 | lift<band | +5 contract points for each trick in your team's contract |
| ca-diamond-lead-contract | C | points | 4 | 2891 | -1.7 [-2.7, -0.7] | -1.6 | -0.1 | 6.8 | 82.7 | 82.2 | 0.7 | lift<band, flat slope | +3 contract points for each trick in your team's contract when your team leads [♦] |
| seed-diamond-lead | C | points | 3 | 1943 | -1.7 [-3.4, -0.1] | -1.0 | -0.7 | 4.0 | 81.9 | 81.3 | 0.8 | rarely decisive, flat slope | +10 contract points when your team leads [♦] |
| ca-four-aces-x | C | xmult | 6 | 1933 | -1.8 [-3.6, -0.1] | -2.6 | 0.8 | 3.7 | 16.5 | 23.5 | 1.5 | lift<band | ×1.5 contract multiplier if your team holds four [A]s |
| e-hold-suit-s-pts | C | points | 3 | 2354 | -1.9 [-3.3, -0.5] | -2.0 | 0.2 | 6.3 | 100.0 | 100.0 | 2.7 | lift<band | +5 contract points for each [♠] your team holds |
| ca-trump-contract | C | points | 4 | 3422 | -1.9 [-3.1, -0.8] | -1.3 | -0.6 | 9.4 | 82.0 | 81.6 | 3.3 | lift<band | +4 contract points for each trick in your team's contract when your team wins a trick by trumping |
| seed-bid8-mult | C | mult | 3 | 2194 | -2.1 [-3.8, -0.4] | -2.3 | 0.3 | 9.8 | 39.4 | 41.1 | 1.6 | lift<band | +15 contract multiplier when your team bids 8 or more |
| seed-nil-points | C | points | 1 | 2830 | -2.1 [-3.6, -0.6] | -1.1 | -1.0 | 5.5 | 100.0 | 100.0 | 2.1 |  | +50 nil points |
| ca-diamond-led-win | C | points | 3 | 3176 | -2.1 [-3.2, -1.0] | -0.6 | -1.6 | 6.1 | 83.7 | 83.0 | 2.7 |  | +20 contract points when your team wins a trick led with [♦] |
| cb-grow-four-row | C | mult | 7 | 8158 | -2.2 [-3.4, -0.9] | -0.8 | -1.4 | 7.4 | 38.3 | 38.4 | 3.9 |  | This sigil gains +3 contract multiplier every time your team wins four tricks in a row (currently +0) |
| cc-twos-to-spades | C | enabler | 7 | 5490 | -2.3 [-3.3, -1.4] | -1.4 | -1.0 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [2] your team holds becomes a [♠] |
| ca-king-lead | C | hybrid | 4 | 1897 | -2.5 [-4.4, -0.7] | -1.8 | -0.7 | 5.0 | 66.8 | 66.8 | 1.4 |  | +20 contract points when your team leads a [K] |
| cb-opp-set-points | C | points | 2 | 2558 | -2.7 [-3.5, -1.8] | -2.8 | 0.1 | 5.0 | 35.5 |  | 0.6 | lift<band, flat slope | +60 contract points if the opponents miss their contract |
| seed-diamond-win | C | points | 3 | 1952 | -2.8 [-4.5, -1.1] | -2.3 | -0.5 | 2.1 | 67.0 | 66.5 | 1.6 | lift<band, rarely decisive | +10 contract points when your team wins a trick with [♦] |
| cc-lead-a-two | C | hybrid | 4 | 4303 | -2.8 [-3.8, -1.8] | -3.7 | 0.9 | 3.5 | 52.2 | 54.0 | 2.2 | lift<band | +20 contract points when your team leads a [2] |
| ca-diamond-hold-nine-x | C | xmult | 5 | 1891 | -2.9 [-4.8, -1.0] | -6.0 | 3.1 | 1.7 | 10.8 | 13.1 | -1.0 | lift<band, flat slope | ×1.5 contract multiplier if your team holds nine [♦]s |
| ca-ace-diamond-win | C | points | 5 | 2695 | -3.0 [-4.1, -1.8] | -3.1 | 0.1 | 4.3 | 41.7 | 41.8 | 1.8 | lift<band, rarely decisive | +40 contract points when your team wins a trick with the [A♦] |
| ca-ace-become | C | enabler | 7 | 4568 | -3.1 [-4.1, -2.0] | -0.5 | -2.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: One card your team holds becomes an [A] |
| cc-three-queens | C | enabler | 7 | 4565 | -3.1 [-4.1, -2.1] | -0.3 | -2.7 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Three cards your team holds become [Q]s |
| seed-ace-lead | C | points | 4 | 1907 | -3.4 [-5.3, -1.4] | -3.4 | -0.0 | 4.5 | 77.1 | 76.6 | 1.4 | lift<band, rarely decisive | +15 contract points when your team leads an [A] |
| seed-trump-points | C | points | 3 | 2298 | -3.4 [-4.9, -2.0] | -2.6 | -0.8 | 6.6 | 81.3 | 81.7 | 3.7 | lift<band | +20 contract points when your team wins a trick by trumping |
| cb-three-row-points | C | points | 5 | 2790 | -3.8 [-5.4, -2.3] | -2.3 | -1.6 | 5.8 | 61.9 | 63.2 | 2.3 | lift<band | +40 contract points when your team wins three tricks in a row |
| cc-low-three-win | C | points | 8 | 2698 | -4.0 [-5.8, -2.2] | -3.3 | -0.7 | 4.5 | 37.7 | 39.3 | 2.2 | lift<band, rarely decisive | +50 contract points when your team wins three tricks with [2]s through [10]s |
| seed-spade-lead | C | points | 3 | 2321 | -4.1 [-5.5, -2.6] | -1.7 | -2.4 | 5.9 | 82.5 | 84.2 | 1.5 | lift<band | +10 contract points when your team leads [♠] |
| seed-king-win | C | points | 4 | 1877 | -4.1 [-6.1, -2.1] | -3.3 | -0.8 | 3.4 | 73.3 | 71.1 | 1.9 | lift<band, rarely decisive | +15 contract points when your team wins a trick with a [K] |
| ca-rainbow-contract | C | points | 6 | 3825 | -4.3 [-5.4, -3.1] | -2.8 | -1.4 | 5.3 | 29.7 | 28.1 | 1.8 | lift<band | +10 contract points for each trick in your team's contract if your team wins tricks with all four suits |
| cb-bid9-xmult | C | xmult | 3 | 2166 | -4.9 [-6.3, -3.5] | -5.1 | 0.3 | 3.7 | 18.8 | 23.8 | 0.8 | lift<band, flat slope | ×1.5 contract multiplier when your team bids 9 or more |
| cb-grow-make-points | C | points | 4 | 2599 | -4.9 [-5.8, -4.1] | -2.4 | -2.5 | 6.6 | 63.9 |  | 3.2 | lift<band | This sigil gains +15 contract points every time your team makes its contract (currently +0) |
| e-hold-suit-h-pts | C | points | 3 | 1913 | -5.0 [-6.6, -3.5] | -3.2 | -1.8 | 6.7 | 100.0 | 100.0 | 3.9 | lift<band | +5 contract points for each [♥] your team holds |
| cb-last-trick-points | C | points | 3 | 6478 | -5.5 [-6.7, -4.3] | -3.6 | -1.9 | 5.4 | 51.0 | 49.8 | 1.9 | lift<band | +40 contract points when your team wins the last trick of a round |
| cb-bid8-trick-points | C | points | 4 | 2243 | -5.5 [-7.1, -3.9] | -4.1 | -1.4 | 5.3 | 36.2 | 40.8 | 2.4 | lift<band | +10 contract points for each trick in your team's contract when your team bids 8 or more |
| ca-side-win | C | points | 3 | 3590 | -5.8 [-6.9, -4.6] | -2.7 | -3.1 | 5.5 | 96.2 | 96.5 | 3.2 | lift<band | +8 contract points when your team wins a trick with a card other than [♠] |
| cb-opp-set-grow | C | mult | 4 | 2551 | -6.1 [-7.0, -5.2] | -2.7 | -3.4 | 5.1 | 35.5 |  | 1.1 | lift<band, flat slope | This sigil gains +3 contract multiplier every time the opponents miss their contract (currently +0) |
| ca-diamond-first-trick | C | points | 4 | 1907 | -6.1 [-7.8, -4.4] | -4.2 | -1.9 | 2.0 | 19.8 | 19.8 | 2.2 | lift<band, rarely decisive | +40 contract points when your team wins the first trick of a round with [♦] |
| ca-two-spade-win | C | points | 5 | 3997 | -6.4 [-7.4, -5.4] | -3.8 | -2.6 | 3.9 | 25.3 | 26.5 | 1.6 | lift<band, rarely decisive | +60 contract points when your team wins a trick with the [2♠] |
| cc-swap-five | C | enabler | 5 | 5765 | -6.5 [-7.6, -5.4] | -0.6 | -5.9 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Swap five cards with your partner |
| cb-grow-make8-points | C | points | 6 | 2234 | -6.5 [-8.0, -5.0] | -4.1 | -2.5 | 2.2 | 15.7 | 17.2 | 0.4 | lift<band, rarely decisive, flat slope | This sigil gains +25 contract points every time your team makes a contract of 8 or more (currently +0) |
| seed-bid8-points | C | points | 3 | 2188 | -6.7 [-8.3, -5.0] | -5.6 | -1.0 | 3.7 | 35.2 | 38.5 | 0.7 | lift<band, rarely decisive, flat slope | +60 contract points when your team bids 8 or more |
| cb-exact-trick-points | C | points | 3 | 6038 | -6.7 [-7.6, -5.7] | -5.4 | -1.3 | 9.9 | 27.4 | 25.8 | -0.2 | lift<band, flat slope | +15 contract points for each trick in your team's contract if your team makes its contract exactly |
| cc-low-diamond-win | C | points | 6 | 3577 | -7.4 [-8.4, -6.3] | -5.2 | -2.1 | 0.8 | 14.8 | 14.2 | 0.4 | lift<band, rarely decisive, flat slope | +20 contract points when your team wins a trick with a [2] through [10] of [♦] |
| seed-e-diamond-three | C | points | 6 | 1990 | -7.5 [-9.2, -5.8] | -7.0 | -0.5 | 0.5 | 4.7 | 4.2 | 1.2 | lift<band, rarely fires, rarely decisive | +40 contract points when your team wins three tricks with [♦] |
| seed-exact-points | C | points | 2 | 4818 | -7.6 [-9.2, -6.0] | -6.1 | -1.6 | 5.7 | 27.8 | 25.5 | 1.0 | lift<band | +50 contract points if your team makes its contract exactly |
| cc-any-suit-last-four | C | enabler | 2 | 9157 | -8.2 [-9.1, -7.4] | -3.2 | -5.0 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit on the last four tricks |
| cb-grow-exact-points | C | points | 4 | 4878 | -8.8 [-10.3, -7.4] | -8.1 | -0.8 | 3.7 | 28.0 | 25.3 | 0.3 | lift<band, rarely decisive, flat slope | This sigil gains +20 contract points every time your team makes its contract exactly (currently +0) |
| cb-free-after-made | C | enabler | 2 | 7973 | -8.9 [-9.7, -8.1] | -3.8 | -5.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit once it has won the tricks in its contract |
| seed-swap | C | enabler | 5 | 5782 | -9.0 [-10.0, -8.0] | -2.3 | -6.8 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Swap three cards with your partner |
| cb-first-lead | C | enabler | 1 | 5183 | -9.7 [-10.6, -8.8] | -5.2 | -4.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team leads the first trick of a round |
| cc-any-suit-first-two | C | enabler | 2 | 6255 | -9.9 [-10.9, -8.9] | -5.0 | -4.9 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit on the first two tricks |
| cc-swap-one | C | enabler | 5 | 4651 | -12.8 [-13.8, -11.7] | -4.8 | -8.0 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Swap one card with your partner |
| ca-lead-spades | C | enabler | 2 | 5108 | -13.7 [-14.7, -12.8] | -6.3 | -7.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can lead [♠] before [♠]s are broken |
| ca-diamond-become-from-clubs | C | enabler | 6 | 4668 | -14.5 [-15.6, -13.3] | -6.5 | -7.9 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [♣] your team holds becomes a [♦] |
| cc-two-twos | C | enabler | 7 | 8019 | -20.2 [-21.0, -19.3] | -9.3 | -10.9 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Two cards your team holds become [2]s |
| seed-become-two | C | enabler | 7 | 7975 | -23.9 [-24.7, -23.1] | -11.6 | -12.3 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [2]s |
| control-uncommon | U | points | 1 | 3989 | -0.1 [-0.6, 0.5] | 0.3 | -0.3 | 13.0 | 100.0 |  | 5.5 |  | +60 contract points |
| control-rare | R | points | 1 | 3826 | 3.6 [2.9, 4.2] | 1.6 | 2.0 | 19.0 | 100.0 |  | 7.3 |  | +80 contract points |
| control-legendary | L | points | 1 | 3864 | 12.7 [12.1, 13.3] | 6.8 | 5.9 | 17.4 | 100.0 |  | 11.0 |  | +120 contract points |
