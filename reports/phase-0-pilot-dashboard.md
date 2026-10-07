# Phase 0 pilot dashboard (seed pool, tier 0)

Fit: 7168 boards, alpha 20, residual sd 0.250 (u units), win calibration k = 1.92, smoothing scale 24615 points.
Tier agreement: 74 sigils, correlation 0.82 (disattenuated 1.00).
Ledger rescoring check: 0 mismatches in 256000 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| pilot-dose-160 | C | points |  | 439 | 19.7 [16.2, 23.1] | 18.3 | 1.3 | 25.6 | 100.0 |  | 10.3 | lift>ceiling |  |
| seed-rainbow-first | C | points | 4 | 541 | 10.6 [4.8, 16.4] | 9.3 | 1.3 | 19.9 | 99.8 | 99.8 | 4.2 |  | +25 contract points when your team wins its first trick with each suit |
| pilot-dose-80 | C | points |  | 445 | 8.2 [5.1, 11.2] | 6.8 | 1.3 | 18.6 | 100.0 |  | 2.3 | flat slope |  |
| seed-ace-hold | C | points | 4 | 500 | 8.0 [2.8, 13.1] | 6.6 | 1.3 | 14.6 | 95.3 | 95.9 | 4.6 |  | +20 contract points for each [A] your team holds |
| seed-spade-lead | C | points | 3 | 496 | 7.1 [1.8, 12.5] | 5.8 | 1.3 | 6.8 | 80.6 | 79.3 | 1.6 | flat slope | +10 contract points when your team leads [♠] |
| seed-rainbow-mult | C | mult | 5 | 588 | 6.7 [1.6, 11.9] | 3.9 | 2.9 | 14.4 | 33.5 | 35.2 | 5.8 |  | +15 contract multiplier if your team wins tricks with all four suits |
| seed-last-trick | C | mult | 3 | 535 | 5.2 [-1.2, 11.6] | 2.3 | 2.9 | 19.7 | 53.2 | 54.3 | 4.4 |  | +15 contract multiplier when your team wins the last trick of a round |
| seed-diamond-hold | C | points | 3 | 385 | 3.5 [-1.0, 8.0] | 2.1 | 1.3 | 17.8 | 100.0 | 100.0 | 6.8 |  | +10 contract points for each [♦] your team holds |
| seed-nil-bid-mult | C | mult | 2 | 519 | 3.2 [-1.3, 7.8] | 0.3 | 2.9 | 16.6 | 44.7 | 50.9 | 2.3 | flat slope | +15 contract multiplier when your team bids nil |
| seed-e-spade-win | C | points | 3 | 695 | 3.2 [-0.2, 6.6] | 1.9 | 1.3 | 11.9 | 96.6 | 96.3 | 3.4 |  | +10 contract points when your team wins a trick with [♠] |
| control-common | C | points | 1 | 375 | 3.0 [0.9, 5.2] | 1.7 | 1.3 | 12.4 | 100.0 |  | 3.2 |  | +40 contract points |
| seed-ace-win | C | points | 4 | 445 | 2.8 [-3.3, 8.8] | 1.4 | 1.3 | 11.2 | 87.7 | 89.6 | 3.9 |  | +20 contract points when your team wins a trick with an [A] |
| seed-exact-mult | C | mult | 2 | 596 | 2.6 [-2.6, 7.7] | -0.3 | 2.9 | 15.9 | 28.2 | 29.0 | 1.6 | flat slope | +15 contract multiplier if your team makes its contract exactly |
| seed-flat-points | C | points | 1 | 399 | 2.4 [-1.0, 5.9] | 1.1 | 1.3 | 11.6 | 100.0 |  | 1.5 | flat slope | +40 contract points |
| seed-trump-points | C | points | 3 | 499 | 1.7 [-3.2, 6.6] | 0.4 | 1.3 | 11.5 | 85.5 | 87.7 | 0.9 | flat slope | +20 contract points when your team wins a trick by trumping |
| seed-bid8-mult | C | mult | 3 | 439 | 1.6 [-4.4, 7.6] | -1.3 | 2.9 | 9.4 | 24.4 | 34.1 | -1.4 | flat slope | +15 contract multiplier when your team bids 8 or more |
| aa-ace-win | C | points | 4 | 475 | 1.4 [-4.0, 6.8] | 0.1 | 1.3 | 13.0 | 88.6 | 92.0 | 2.9 |  | +20 contract points when your team wins a trick with an [A] |
| seed-ace-lead | C | points | 4 | 428 | 1.2 [-4.1, 6.4] | -0.2 | 1.3 | 6.2 | 72.1 | 77.6 | 0.5 | flat slope | +15 contract points when your team leads an [A] |
| pilot-dose-40 | C | points |  | 394 | 0.5 [-2.6, 3.5] | -0.8 | 1.3 | 10.9 | 100.0 |  | 2.0 | flat slope |  |
| seed-win-points | C | points | 2 | 398 | 0.2 [-2.9, 3.4] | -1.1 | 1.3 | 11.3 | 99.8 |  | 0.9 | flat slope | +5 contract points when your team wins a trick |
| seed-e-lead-three-suits | C | points | 5 | 563 | 0.2 [-5.5, 5.8] | -1.1 | 1.3 | 12.1 | 86.3 | 87.9 | 5.5 |  | +50 contract points if your team leads three suits |
| seed-contract-points | C | points | 2 | 502 | 0.0 [-5.1, 5.2] | -1.3 | 1.3 | 9.2 | 100.0 | 100.0 | 2.4 | flat slope | +5 contract points for each trick in your team's contract |
| seed-consecutive | C | points | 3 | 491 | -0.0 [-5.0, 5.0] | -1.3 | 1.3 | 9.9 | 89.3 | 90.8 | 4.8 |  | +10 contract points when your team wins consecutive tricks |
| seed-low-lead | C | points | 5 | 618 | -0.0 [-5.7, 5.6] | -1.4 | 1.3 | 14.0 | 98.6 | 98.3 | 2.7 |  | +10 contract points when your team leads a [2] through [10] |
| seed-swap | C | enabler | 5 | 1137 | -0.3 [-3.3, 2.7] | -1.8 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Swap three cards with your partner |
| seed-bid8-points | C | points | 3 | 467 | -1.8 [-7.2, 3.6] | -3.1 | 1.3 | 4.9 | 23.3 | 33.9 | 2.9 | rarely decisive | +60 contract points when your team bids 8 or more |
| seed-diamond-lead | C | points | 3 | 414 | -2.6 [-7.1, 1.9] | -4.0 | 1.3 | 6.8 | 86.8 | 86.4 | 2.6 | flat slope | +10 contract points when your team leads [♦] |
| seed-nil-points | C | points | 1 | 457 | -2.9 [-7.3, 1.4] | -4.3 | 1.3 | 2.4 | 100.0 | 100.0 | 1.7 | rarely decisive, flat slope | +50 nil points |
| seed-diamond-win | C | points | 3 | 386 | -3.8 [-8.3, 0.8] | -5.1 | 1.3 | 4.8 | 69.4 | 69.0 | 1.5 | lift<band, rarely decisive, flat slope | +10 contract points when your team wins a trick with [♦] |
| pilot-dose-20 | C | points |  | 430 | -4.1 [-7.1, -1.1] | -5.5 | 1.3 | 8.0 | 100.0 |  | -0.1 | lift<band, flat slope |  |
| seed-exact-points | C | points | 2 | 630 | -4.5 [-9.4, 0.4] | -5.8 | 1.3 | 6.1 | 27.3 | 29.2 | 1.1 | lift<band, flat slope | +50 contract points if your team makes its contract exactly |
| pilot-dose-10 | C | points |  | 423 | -5.4 [-8.3, -2.6] | -6.8 | 1.3 | 3.5 | 100.0 |  | 1.9 | lift<band, rarely decisive, flat slope |  |
| seed-low-win | C | points | 5 | 649 | -5.6 [-11.3, 0.1] | -6.9 | 1.3 | 12.6 | 89.0 | 89.8 | 3.9 | lift<band | +15 contract points when your team wins a trick with a [2] through [10] |
| seed-king-win | C | points | 4 | 449 | -6.6 [-11.8, -1.3] | -7.9 | 1.3 | 7.2 | 76.8 | 79.0 | 3.0 | lift<band | +15 contract points when your team wins a trick with a [K] |
| seed-e-diamond-three | C | points | 6 | 397 | -7.2 [-11.4, -2.9] | -8.5 | 1.3 | 1.3 | 7.6 | 7.4 | 1.6 | lift<band, rarely fires, rarely decisive, flat slope | +40 contract points when your team wins three tricks with [♦] |
| seed-become-two | C | enabler | 7 | 1267 | -11.5 [-14.3, -8.7] | -13.0 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [2]s |
| seed-flat-mult | U | mult | 1 | 391 | 9.3 [6.4, 12.3] | 6.5 | 2.9 | 23.1 | 100.0 |  | 7.0 |  | +10 contract multiplier |
| control-uncommon | U | points | 1 | 379 | 4.6 [2.3, 6.9] | 3.3 | 1.3 | 18.5 | 100.0 |  | 3.6 |  | +60 contract points |
| seed-four-row | U | mult | 5 | 565 | 4.2 [-0.8, 9.1] | 1.3 | 2.9 | 24.1 | 41.9 | 44.8 | 7.0 |  | +20 contract multiplier when your team wins four tricks in a row |
| seed-four-aces | U | mult | 6 | 463 | 3.0 [-2.2, 8.2] | 0.1 | 2.9 | 8.9 | 15.3 | 28.6 | 4.0 |  | +20 contract multiplier if your team holds four [A]s |
| seed-ace-three | U | xmult | 7 | 474 | 2.6 [-1.8, 7.0] | -0.4 | 3.0 | 8.9 | 27.0 | 36.7 | 4.1 |  | ×1.5 contract multiplier when your team wins three tricks with [A]s |
| seed-trump-three | U | mult | 6 | 510 | 0.8 [-4.3, 5.8] | -2.1 | 2.9 | 17.2 | 31.7 | 36.1 | 4.3 |  | +15 contract multiplier when your team wins three tricks by trumping |
| seed-contract-points-u | U | points | 2 | 494 | -0.1 [-4.8, 4.6] | -1.4 | 1.3 | 17.8 | 100.0 | 100.0 | 4.2 |  | +10 contract points for each trick in your team's contract |
| seed-rainbow-lead-x | U | xmult | 5 | 540 | -0.3 [-5.3, 4.8] | -3.3 | 3.0 | 14.7 | 47.7 | 49.7 | 4.2 |  | ×1.5 contract multiplier if your team leads all four suits |
| seed-spade-hold | U | points | 3 | 496 | -0.4 [-5.2, 4.5] | -1.7 | 1.3 | 11.6 | 100.0 | 100.0 | 5.1 |  | +5 contract points for each [♠] your team holds |
| seed-become-king | U | enabler | 5 | 1110 | -1.2 [-4.3, 1.8] | -2.7 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Four cards your team holds become [K]s |
| seed-low-four | U | mult | 8 | 676 | -2.6 [-7.9, 2.7] | -5.5 | 2.9 | 16.5 | 21.2 | 23.0 | 2.6 | lift<band | +15 contract multiplier when your team wins four tricks with [2]s through [10]s |
| seed-e-side-four | U | mult | 6 | 770 | -2.8 [-5.9, 0.2] | -5.7 | 2.9 | 21.9 | 38.4 | 39.4 | 5.7 | lift<band | +15 contract multiplier when your team wins four tricks with cards other than [♠] |
| seed-nil-made-x | U | xmult | 2 | 478 | -3.9 [-7.9, 0.1] | -6.9 | 3.0 | 9.4 | 19.7 | 25.9 | 0.8 | lift<band, flat slope | ×1.5 contract multiplier if your team makes a nil |
| seed-e-heart-three | U | mult | 6 | 432 | -4.3 [-8.2, -0.3] | -7.1 | 2.9 | 7.7 | 9.4 | 9.7 | 2.1 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins three tricks with [♥] |
| seed-diamond-three | U | mult | 6 | 406 | -4.6 [-9.2, 0.0] | -7.5 | 2.9 | 5.8 | 7.8 | 7.8 | 1.7 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins three tricks with [♦] |
| seed-exact-x | U | xmult | 2 | 619 | -5.6 [-10.0, -1.1] | -8.6 | 3.0 | 12.3 | 26.7 | 27.0 | 1.0 | lift<band, flat slope | ×1.5 contract multiplier if your team makes its contract exactly |
| seed-e-last-heart | U | mult | 4 | 722 | -6.4 [-9.7, -3.1] | -9.3 | 2.9 | 4.9 | 5.9 | 6.0 | 2.7 | lift<band, rarely fires | +15 contract multiplier when your team wins the last trick of a round with [♥] |
| seed-lead-choice | U | enabler | 2 | 1031 | -6.6 [-9.6, -3.5] | -8.1 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Choose which partner leads after your team wins a trick |
| seed-bid10-mult | U | mult | 3 | 475 | -9.1 [-14.5, -3.7] | -12.0 | 2.9 | 3.3 | 4.1 | 6.2 | 2.4 | lift<band, rarely fires, flat slope | +20 contract multiplier when your team bids 10 or more |
| seed-diamond-five | U | xmult | 6 | 414 | -11.9 [-15.7, -8.1] | -14.9 | 3.0 | 0.1 | 0.2 | 0.3 | 0.0 | lift<band, rarely fires, flat slope | ×1.5 contract multiplier when your team wins five tricks with [♦] |
| seed-become-heart | U | enabler | 4 | 941 | -15.1 [-18.0, -12.2] | -16.6 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [♥]s |
| seed-raise | R | enabler | 4 | 1089 | 12.4 [9.5, 15.4] | 10.9 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by two ranks |
| seed-ace-mult | R | mult | 4 | 470 | 9.9 [5.5, 14.3] | 7.0 | 2.9 | 22.4 | 88.5 | 90.2 | 5.0 |  | +5 contract multiplier when your team wins a trick with an [A] |
| control-rare | R | points | 1 | 434 | 7.3 [5.2, 9.4] | 6.0 | 1.3 | 21.2 | 100.0 |  | 7.0 |  | +80 contract points |
| seed-spade-mult | R | mult | 3 | 515 | 5.7 [1.2, 10.2] | 2.8 | 2.9 | 18.8 | 96.0 | 96.7 | 4.8 |  | +2 contract multiplier when your team wins a trick with [♠] |
| seed-rainbow-x | R | xmult | 5 | 581 | 3.1 [-2.2, 8.4] | 0.1 | 3.0 | 17.4 | 33.1 | 34.6 | 5.7 |  | ×2 contract multiplier if your team wins tricks with all four suits |
| seed-flat-x | R | xmult | 1 | 417 | -0.0 [-3.3, 3.3] | -3.0 | 3.0 | 19.5 | 100.0 |  | 4.1 |  | ×1.5 contract multiplier |
| seed-make8-x | R | xmult | 4 | 500 | -1.0 [-5.6, 3.6] | -4.0 | 3.0 | 14.7 | 16.4 | 22.6 | 4.5 |  | ×2 contract multiplier if your team makes a contract of 8 or more |
| seed-spade-five | R | xmult | 6 | 497 | -2.1 [-7.2, 3.1] | -5.1 | 3.0 | 17.4 | 28.6 | 30.4 | 2.7 |  | ×2 contract multiplier when your team wins five tricks with [♠] |
| seed-five-row-x | R | xmult | 5 | 523 | -2.8 [-7.1, 1.5] | -5.8 | 3.0 | 15.7 | 25.0 | 25.4 | 4.3 | lift<band | ×2 contract multiplier when your team wins five tricks in a row |
| seed-two-win | R | points | 4 | 718 | -4.2 [-9.7, 1.2] | -5.6 | 1.3 | 17.5 | 29.2 | 36.2 | 3.7 | lift<band | +150 contract points when your team wins a trick with a [2] |
| seed-grow-exact | R | mult | 4 | 621 | -7.3 [-12.1, -2.6] | -10.2 | 2.9 | 16.9 | 28.1 | 31.0 | 1.5 | lift<band, flat slope | This sigil gains +5 contract multiplier every time your team makes its contract exactly (currently +0) |
| seed-any-suit | R | enabler | 2 | 1181 | -8.1 [-10.7, -5.5] | -9.7 | 1.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit on the last three tricks |
| seed-diamond-mult | R | mult | 3 | 419 | -9.6 [-13.6, -5.6] | -12.5 | 2.9 | 9.4 | 68.3 | 66.9 | 1.0 | lift<band, flat slope | +2 contract multiplier when your team wins a trick with [♦] |
| seed-grow-nil | R | points | 4 | 519 | -10.5 [-14.7, -6.3] | -11.9 | 1.3 | 3.0 | 16.5 | 23.2 | 1.1 | lift<band, rarely decisive, flat slope | This sigil gains +40 nil points every time your team makes a nil (currently +0) |
| seed-grow-make8 | R | mult | 6 | 495 | -11.0 [-15.6, -6.4] | -13.9 | 2.9 | 9.1 | 16.4 | 22.0 | 1.3 | lift<band, flat slope | This sigil gains +5 contract multiplier every time your team makes a contract of 8 or more (currently +0) |
| control-legendary | L | points | 1 | 416 | 10.7 [8.6, 12.9] | 9.4 | 1.3 | 23.2 | 100.0 |  | 8.8 |  | +120 contract points |
| seed-flat-x2 | L | xmult | 1 | 417 | 7.0 [3.9, 10.0] | 3.9 | 3.0 | 22.1 | 100.0 |  | 9.4 |  | ×2 contract multiplier |
