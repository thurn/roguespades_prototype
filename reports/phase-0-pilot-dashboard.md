# Phase 0 pilot dashboard (seed pool, tier 0)

Fit: 7168 boards, alpha 8, residual sd 0.242 (u units), win calibration k = 1.94, smoothing scale 25128 points.
Tier agreement: 74 sigils, lift correlation 0.79 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 256000 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| pilot-dose-160 | C | points |  | 439 | 21.0 [17.7, 24.2] | 21.0 | -0.1 | 25.4 | 100.0 |  | 10.2 | lift>ceiling |  |
| pilot-dose-80 | C | points |  | 445 | 11.2 [7.9, 14.5] | 11.3 | -0.1 | 17.6 | 100.0 |  | 7.9 |  |  |
| aa-ace-win | C | points | 4 | 475 | 9.8 [4.6, 15.1] | 9.9 | -0.1 | 8.8 | 89.8 | 92.5 | 6.8 |  | +20 contract points when your team wins a trick with an [A] |
| seed-rainbow-first | C | points | 4 | 578 | 9.6 [4.5, 14.6] | 9.6 | -0.1 | 18.5 | 99.7 | 99.8 | 8.2 |  | +25 contract points when your team wins its first trick with each suit |
| seed-rainbow-mult | C | mult | 5 | 625 | 8.9 [3.5, 14.3] | 10.7 | -1.8 | 14.3 | 34.0 | 34.9 | 5.4 |  | +15 contract multiplier if your team wins tricks with all four suits |
| seed-ace-hold | C | points | 4 | 527 | 8.9 [3.3, 14.5] | 9.0 | -0.1 | 14.4 | 95.3 | 95.9 | 5.0 |  | +20 contract points for each [A] your team holds |
| seed-ace-win | C | points | 4 | 469 | 5.7 [0.4, 11.0] | 5.8 | -0.1 | 10.4 | 88.1 | 90.0 | 3.2 |  | +20 contract points when your team wins a trick with an [A] |
| seed-diamond-hold | C | points | 3 | 414 | 5.1 [0.4, 9.8] | 5.2 | -0.1 | 17.1 | 100.0 | 100.0 | 5.5 |  | +10 contract points for each [♦] your team holds |
| seed-e-lead-three-suits | C | points | 5 | 596 | 4.9 [-0.6, 10.4] | 5.0 | -0.1 | 11.9 | 86.3 | 86.5 | 6.1 |  | +50 contract points if your team leads three suits |
| seed-consecutive | C | points | 3 | 518 | 3.9 [-1.1, 8.9] | 4.0 | -0.1 | 11.3 | 89.2 | 90.9 | 5.5 |  | +10 contract points when your team wins consecutive tricks |
| seed-last-trick | C | mult | 3 | 574 | 3.0 [-2.1, 8.2] | 4.8 | -1.8 | 18.7 | 52.8 | 52.1 | 6.3 |  | +15 contract multiplier when your team wins the last trick of a round |
| seed-win-points | C | points | 2 | 423 | 3.0 [-0.0, 6.0] | 3.1 | -0.1 | 8.9 | 99.8 |  | 2.7 | flat slope | +5 contract points when your team wins a trick |
| seed-diamond-lead | C | points | 3 | 436 | 2.8 [-1.9, 7.4] | 2.8 | -0.1 | 8.3 | 86.4 | 85.9 | 2.7 | flat slope | +10 contract points when your team leads [♦] |
| seed-flat-points | C | points | 1 | 418 | 2.6 [-0.6, 5.8] | 2.7 | -0.1 | 12.2 | 100.0 |  | 3.3 | flat slope | +40 contract points |
| seed-e-spade-win | C | points | 3 | 741 | 2.5 [-1.5, 6.4] | 2.5 | -0.1 | 10.7 | 96.5 | 96.5 | 4.0 |  | +10 contract points when your team wins a trick with [♠] |
| seed-spade-lead | C | points | 3 | 533 | 1.8 [-3.7, 7.3] | 1.9 | -0.1 | 7.1 | 80.5 | 80.0 | 2.4 | flat slope | +10 contract points when your team leads [♠] |
| seed-nil-points | C | points | 1 | 489 | 1.6 [-3.0, 6.2] | 1.6 | -0.1 | 3.2 | 100.0 | 100.0 | 0.6 | rarely decisive, flat slope | +50 nil points |
| pilot-dose-40 | C | points |  | 394 | 1.5 [-1.6, 4.6] | 1.6 | -0.1 | 11.1 | 100.0 |  | 0.1 | flat slope |  |
| pilot-dose-20 | C | points |  | 430 | 0.7 [-2.3, 3.6] | 0.7 | -0.1 | 6.7 | 100.0 |  | 1.8 | flat slope |  |
| seed-trump-points | C | points | 3 | 526 | 0.4 [-4.5, 5.3] | 0.5 | -0.1 | 11.8 | 85.6 | 87.4 | 1.0 | flat slope | +20 contract points when your team wins a trick by trumping |
| seed-bid8-mult | C | mult | 3 | 465 | 0.4 [-4.7, 5.5] | 2.2 | -1.8 | 9.9 | 24.2 | 34.6 | 3.4 |  | +15 contract multiplier when your team bids 8 or more |
| seed-swap | C | enabler | 5 | 1224 | 0.3 [-3.0, 3.5] | 1.4 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Swap three cards with your partner |
| seed-contract-points | C | points | 2 | 539 | -0.4 [-5.0, 4.1] | -0.3 | -0.1 | 7.5 | 100.0 | 100.0 | 0.2 | flat slope | +5 contract points for each trick in your team's contract |
| control-common | C | points | 1 | 397 | -1.2 [-3.5, 1.1] | -1.1 | -0.1 | 11.3 | 100.0 |  | 2.0 | flat slope | +40 contract points |
| pilot-dose-10 | C | points |  | 423 | -1.7 [-4.7, 1.2] | -1.7 | -0.1 | 1.9 | 100.0 |  | 0.2 | rarely decisive, flat slope |  |
| seed-ace-lead | C | points | 4 | 463 | -2.3 [-7.3, 2.7] | -2.2 | -0.1 | 6.4 | 72.5 | 77.6 | 2.3 | flat slope | +15 contract points when your team leads an [A] |
| seed-bid8-points | C | points | 3 | 493 | -2.5 [-7.4, 2.4] | -2.4 | -0.1 | 4.5 | 23.7 | 33.6 | 0.6 | rarely decisive, flat slope | +60 contract points when your team bids 8 or more |
| seed-exact-mult | C | mult | 2 | 623 | -2.7 [-7.4, 2.0] | -0.9 | -1.8 | 16.8 | 27.9 | 29.6 | 3.5 |  | +15 contract multiplier if your team makes its contract exactly |
| seed-low-lead | C | points | 5 | 665 | -3.1 [-9.0, 2.8] | -3.0 | -0.1 | 15.1 | 98.6 | 98.9 | 4.7 |  | +10 contract points when your team leads a [2] through [10] |
| seed-nil-bid-mult | C | mult | 2 | 558 | -3.3 [-7.6, 1.0] | -1.5 | -1.8 | 16.5 | 45.1 | 53.1 | 4.7 |  | +15 contract multiplier when your team bids nil |
| seed-e-diamond-three | C | points | 6 | 417 | -3.6 [-7.8, 0.7] | -3.5 | -0.1 | 1.5 | 7.3 | 7.2 | -0.0 | rarely fires, rarely decisive, flat slope | +40 contract points when your team wins three tricks with [♦] |
| seed-diamond-win | C | points | 3 | 414 | -3.8 [-8.3, 0.6] | -3.8 | -0.1 | 4.7 | 68.9 | 68.1 | -0.4 | rarely decisive, flat slope | +10 contract points when your team wins a trick with [♦] |
| seed-exact-points | C | points | 2 | 668 | -3.9 [-8.6, 0.8] | -3.8 | -0.1 | 6.7 | 26.6 | 28.6 | 0.9 | flat slope | +50 contract points if your team makes its contract exactly |
| seed-low-win | C | points | 5 | 694 | -4.5 [-9.5, 0.5] | -4.4 | -0.1 | 12.4 | 88.7 | 90.2 | 5.3 |  | +15 contract points when your team wins a trick with a [2] through [10] |
| seed-king-win | C | points | 4 | 473 | -4.8 [-10.3, 0.7] | -4.7 | -0.1 | 7.6 | 77.0 | 78.9 | 0.6 | flat slope | +15 contract points when your team wins a trick with a [K] |
| seed-become-two | C | enabler | 7 | 1365 | -11.4 [-14.2, -8.7] | -10.3 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [2]s |
| seed-flat-mult | U | mult | 1 | 425 | 7.8 [4.9, 10.7] | 9.6 | -1.8 | 22.5 | 100.0 |  | 9.1 |  | +10 contract multiplier |
| seed-ace-three | U | xmult | 7 | 500 | 4.7 [0.2, 9.2] | 3.4 | 1.3 | 8.8 | 27.1 | 36.3 | 4.3 |  | ×1.5 contract multiplier when your team wins three tricks with [A]s |
| seed-trump-three | U | mult | 6 | 536 | 2.6 [-2.9, 8.2] | 4.4 | -1.8 | 16.1 | 31.8 | 36.4 | 5.8 |  | +15 contract multiplier when your team wins three tricks by trumping |
| seed-four-aces | U | mult | 6 | 479 | 1.1 [-4.1, 6.3] | 2.9 | -1.8 | 9.0 | 15.4 | 26.3 | 3.6 |  | +20 contract multiplier if your team holds four [A]s |
| seed-become-king | U | enabler | 5 | 1170 | 0.9 [-1.9, 3.7] | 2.0 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Four cards your team holds become [K]s |
| seed-rainbow-lead-x | U | xmult | 5 | 573 | 0.5 [-4.4, 5.4] | -0.8 | 1.3 | 15.1 | 47.8 | 50.0 | 3.2 |  | ×1.5 contract multiplier if your team leads all four suits |
| control-uncommon | U | points | 1 | 404 | 0.0 [-2.1, 2.1] | 0.1 | -0.1 | 18.6 | 100.0 |  | 7.0 |  | +60 contract points |
| seed-four-row | U | mult | 5 | 591 | -0.8 [-5.1, 3.5] | 1.0 | -1.8 | 23.5 | 41.5 | 43.3 | 7.3 |  | +20 contract multiplier when your team wins four tricks in a row |
| seed-low-four | U | mult | 8 | 704 | -1.1 [-6.3, 4.0] | 0.7 | -1.8 | 16.5 | 22.0 | 23.7 | 4.1 |  | +15 contract multiplier when your team wins four tricks with [2]s through [10]s |
| seed-nil-made-x | U | xmult | 2 | 507 | -1.3 [-5.4, 2.8] | -2.6 | 1.3 | 8.3 | 19.5 | 25.1 | 0.9 | flat slope | ×1.5 contract multiplier if your team makes a nil |
| seed-spade-hold | U | points | 3 | 521 | -2.4 [-7.9, 3.1] | -2.3 | -0.1 | 11.7 | 100.0 | 100.0 | 3.7 |  | +5 contract points for each [♠] your team holds |
| seed-exact-x | U | xmult | 2 | 649 | -2.5 [-6.6, 1.7] | -3.8 | 1.3 | 11.9 | 26.5 | 27.7 | 1.7 | flat slope | ×1.5 contract multiplier if your team makes its contract exactly |
| seed-lead-choice | U | enabler | 2 | 1074 | -4.7 [-7.3, -2.1] | -3.6 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Choose which partner leads after your team wins a trick |
| seed-contract-points-u | U | points | 2 | 511 | -4.9 [-9.4, -0.4] | -4.8 | -0.1 | 17.4 | 100.0 | 100.0 | 6.2 | lift<band | +10 contract points for each trick in your team's contract |
| seed-e-side-four | U | mult | 6 | 807 | -5.7 [-8.6, -2.7] | -3.9 | -1.8 | 19.6 | 38.3 | 39.6 | 5.9 | lift<band | +15 contract multiplier when your team wins four tricks with cards other than [♠] |
| seed-diamond-three | U | mult | 6 | 428 | -6.9 [-11.3, -2.5] | -5.1 | -1.8 | 5.8 | 7.8 | 7.5 | -0.3 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins three tricks with [♦] |
| seed-bid10-mult | U | mult | 3 | 503 | -7.4 [-11.7, -3.2] | -5.6 | -1.8 | 4.2 | 4.2 | 7.3 | 2.8 | lift<band, rarely fires, flat slope | +20 contract multiplier when your team bids 10 or more |
| seed-e-heart-three | U | mult | 6 | 452 | -8.0 [-12.1, -3.9] | -6.2 | -1.8 | 7.3 | 9.8 | 10.3 | 1.8 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins three tricks with [♥] |
| seed-e-last-heart | U | mult | 4 | 756 | -9.4 [-12.3, -6.6] | -7.6 | -1.8 | 5.0 | 5.6 | 5.4 | 0.9 | lift<band, rarely fires, flat slope | +15 contract multiplier when your team wins the last trick of a round with [♥] |
| seed-diamond-five | U | xmult | 6 | 435 | -9.5 [-13.3, -5.7] | -10.8 | 1.3 | 0.2 | 0.3 | 0.3 | 1.4 | lift<band, rarely fires, flat slope | ×1.5 contract multiplier when your team wins five tricks with [♦] |
| seed-become-heart | U | enabler | 4 | 983 | -12.9 [-15.6, -10.1] | -11.7 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Four cards your team holds become [♥]s |
| seed-raise | R | enabler | 4 | 1100 | 11.0 [8.6, 13.4] | 12.1 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by two ranks |
| seed-ace-mult | R | mult | 4 | 472 | 5.7 [0.6, 10.7] | 7.5 | -1.8 | 21.3 | 88.0 | 90.3 | 10.2 |  | +5 contract multiplier when your team wins a trick with an [A] |
| seed-spade-five | R | xmult | 6 | 501 | 5.3 [0.2, 10.4] | 4.0 | 1.3 | 17.3 | 29.3 | 31.2 | 3.1 | flat slope | ×2 contract multiplier when your team wins five tricks with [♠] |
| control-rare | R | points | 1 | 438 | 4.7 [2.9, 6.5] | 4.7 | -0.1 | 20.6 | 100.0 |  | 8.2 |  | +80 contract points |
| seed-spade-mult | R | mult | 3 | 522 | 3.3 [-1.6, 8.2] | 5.1 | -1.8 | 18.4 | 96.0 | 96.4 | 8.3 |  | +2 contract multiplier when your team wins a trick with [♠] |
| seed-five-row-x | R | xmult | 5 | 531 | 3.1 [-1.2, 7.4] | 1.8 | 1.3 | 17.7 | 25.2 | 27.8 | 4.6 |  | ×2 contract multiplier when your team wins five tricks in a row |
| seed-rainbow-x | R | xmult | 5 | 591 | 1.2 [-3.5, 5.9] | -0.0 | 1.3 | 16.9 | 34.2 | 37.3 | 5.0 |  | ×2 contract multiplier if your team wins tricks with all four suits |
| seed-flat-x | R | xmult | 1 | 421 | 0.9 [-1.8, 3.6] | -0.4 | 1.3 | 18.5 | 100.0 |  | 8.0 |  | ×1.5 contract multiplier |
| seed-make8-x | R | xmult | 4 | 505 | -3.4 [-7.8, 1.1] | -4.7 | 1.3 | 14.5 | 17.1 | 22.0 | 3.4 | lift<band | ×2 contract multiplier if your team makes a contract of 8 or more |
| seed-two-win | R | points | 4 | 731 | -4.5 [-9.7, 0.8] | -4.4 | -0.1 | 16.4 | 28.5 | 39.3 | 1.7 | flat slope | +150 contract points when your team wins a trick with a [2] |
| seed-grow-nil | R | points | 4 | 522 | -9.9 [-13.5, -6.3] | -9.8 | -0.1 | 2.1 | 16.9 | 23.1 | 1.7 | lift<band, rarely decisive, flat slope | This sigil gains +40 nil points every time your team makes a nil (currently +0) |
| seed-grow-make8 | R | mult | 6 | 509 | -10.2 [-14.8, -5.5] | -8.4 | -1.8 | 8.6 | 17.4 | 23.6 | 1.8 | lift<band, flat slope | This sigil gains +5 contract multiplier every time your team makes a contract of 8 or more (currently +0) |
| seed-any-suit | R | enabler | 2 | 1196 | -10.9 [-13.3, -8.5] | -9.8 | -1.1 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit on the last three tricks |
| seed-diamond-mult | R | mult | 3 | 426 | -10.9 [-14.5, -7.3] | -9.1 | -1.8 | 8.5 | 68.5 | 66.3 | 2.0 | lift<band, flat slope | +2 contract multiplier when your team wins a trick with [♦] |
| seed-grow-exact | R | mult | 4 | 631 | -15.0 [-19.4, -10.5] | -13.2 | -1.8 | 16.3 | 27.9 | 30.2 | 2.4 | lift<band, flat slope | This sigil gains +5 contract multiplier every time your team makes its contract exactly (currently +0) |
| control-legendary | L | points | 1 | 422 | 9.1 [7.0, 11.1] | 9.2 | -0.1 | 22.7 | 100.0 |  | 9.1 |  | +120 contract points |
| seed-flat-x2 | L | xmult | 1 | 425 | 3.3 [0.3, 6.3] | 2.0 | 1.3 | 22.2 | 100.0 |  | 7.9 |  | ×2 contract multiplier |
