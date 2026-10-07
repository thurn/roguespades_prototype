# Draft pass: draft-rare — tournament dashboard

Fit: 55998 boards, alpha 20, residual sd 0.418 (u units), win calibration k = 1.29, smoothing scale 25128 points.
Tier agreement: 101 sigils, lift correlation -0.01 on shared boards (overstated by shared deals and grants).
Ledger rescoring check: 0 mismatches in 1983424 rounds.

| Id | Rarity | Cat | C | n | Lift (pts, 90%) | T0 lift | Skill | Decisive % | Fire % | Committed % | Slope/2x | Flags | Text |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| control-common | C | points | 1 | 2729 | -2.3 [-3.0, -1.6] | 0.1 | -2.4 | 6.9 | 100.0 |  | 3.2 |  | +40 contract points |
| ua-side-raise | U | enabler | 5 | 11710 | 1.3 [0.0, 2.5] | -1.3 | 2.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every card other than [♠] your team holds by two ranks |
| control-uncommon | U | points | 1 | 2623 | 0.9 [0.1, 1.8] | 1.6 | -0.7 | 11.5 | 100.0 |  | 4.8 |  | +60 contract points |
| ua-spade-raise | U | enabler | 5 | 10310 | -0.1 [-1.4, 1.2] | -1.3 | 1.2 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every [♠] your team holds by two ranks |
| rc-low-beats | R | enabler | 5 | 11557 | 52.9 [51.6, 54.1] | 25.1 | 27.8 | 0.0 | 0.0 | 0.0 | 0.0 | lift>ceiling | Your team's [2]s through [4]s beat every other card of their suit |
| ra-jq-become-aces | R | enabler | 7 | 11112 | 14.2 [12.9, 15.6] | 5.9 | 8.3 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Every [J] through [Q] your team holds becomes an [A] |
| ra-side-win-points | R | points | 3 | 11833 | 11.3 [10.3, 12.3] | 4.9 | 6.4 | 21.6 | 93.0 | 93.5 | 8.1 |  | +30 contract points when your team wins a trick with a card other than [♠] |
| rb-streak-contract-mult | R | mult | 4 | 10129 | 8.9 [7.9, 9.9] | 4.2 | 4.7 | 21.9 | 82.3 | 82.6 | 5.5 |  | +1 contract multiplier for each trick in your team's contract when your team wins consecutive tricks |
| ra-diamond-hold-x | R | xmult | 3 | 10178 | 8.1 [6.9, 9.3] | 4.8 | 3.3 | 23.2 | 100.0 | 100.0 | 10.8 |  | ×1.1 contract multiplier for each [♦] your team holds |
| seed-two-win | R | points | 4 | 6230 | 6.8 [5.6, 8.0] | 2.5 | 4.3 | 16.7 | 35.9 | 48.5 | 5.8 |  | +150 contract points when your team wins a trick with a [2] |
| control-rare | R | points | 1 | 2676 | 6.0 [5.1, 6.9] | 3.4 | 2.6 | 16.4 | 100.0 |  | 5.3 |  | +80 contract points |
| rc-low-win-contract-points | R | points | 6 | 11042 | 5.6 [4.6, 6.7] | 3.8 | 1.8 | 23.8 | 80.0 | 80.1 | 7.6 |  | +10 contract points for each trick in your team's contract when your team wins a trick with a [2] through [9] |
| seed-raise | R | enabler | 4 | 11791 | 5.5 [4.2, 6.8] | 1.2 | 4.3 | 0.0 | 0.0 | 0.0 | 0.0 |  | Opening: Raise every card your team holds by two ranks |
| ra-ace-hold-x | R | xmult | 4 | 9854 | 5.2 [4.1, 6.4] | 2.9 | 2.3 | 21.4 | 95.2 | 95.7 | 8.8 |  | ×1.25 contract multiplier for each [A] your team holds |
| rb-honors-free | R | enabler | 4 | 10347 | 2.9 [1.5, 4.3] | -2.0 | 4.9 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play [10]s through [A]s even when it can follow suit |
| ra-spade-win-contract | R | mult | 4 | 9829 | 1.7 [0.6, 2.7] | 0.6 | 1.1 | 20.0 | 92.1 | 92.2 | 5.0 |  | +1 contract multiplier for each trick in your team's contract when your team wins a trick with [♠] |
| rc-honor-free | R | enabler | 4 | 6699 | 0.3 [-1.1, 1.8] | -4.2 | 4.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play [J]s through [A]s even when it can follow suit |
| rc-low-lead-x | R | xmult | 5 | 13492 | -1.0 [-2.0, 0.0] | -1.0 | -0.0 | 17.4 | 87.4 | 87.8 | 8.1 |  | ×1.2 contract multiplier when your team leads a [2] through [6] |
| ra-rainbow-first-x | R | xmult | 4 | 13189 | -1.7 [-2.7, -0.7] | -1.1 | -0.6 | 16.5 | 98.9 | 98.9 | 7.2 | lift<band | ×1.2 contract multiplier when your team wins its first trick with each suit |
| ra-diamond-led-win-x | R | hybrid | 3 | 11006 | -1.8 [-2.9, -0.7] | -0.3 | -1.5 | 18.7 | 83.5 | 83.9 | 7.3 |  | ×1.3 contract multiplier when your team wins a trick led with [♦] |
| rc-opp-aces-fall | R | enabler | 6 | 4716 | -2.4 [-3.9, -0.9] | -3.8 | 1.4 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [A] the opponents hold becomes a [2] |
| seed-ace-mult | R | mult | 4 | 6674 | -2.7 [-3.9, -1.6] | -1.8 | -1.0 | 13.3 | 83.7 | 86.8 | 4.1 | lift<band | +5 contract multiplier when your team wins a trick with an [A] |
| ra-spade-lead-x | R | hybrid | 3 | 9556 | -2.8 [-3.9, -1.7] | -2.7 | -0.1 | 17.8 | 75.7 | 75.8 | 6.5 | lift<band | ×1.25 contract multiplier when your team leads [♠] |
| rc-low-untrumpable | R | enabler | 5 | 10505 | -3.5 [-4.8, -2.3] | -2.0 | -1.5 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team's [2]s through [10]s can't be trumped |
| rb-last-contract-x | R | xmult | 4 | 5617 | -5.6 [-6.7, -4.4] | -4.4 | -1.1 | 16.9 | 46.5 | 46.8 | 3.7 | lift<band | ×1.1 contract multiplier for each trick in your team's contract when your team wins the last trick of a round |
| rb-opp-win-mult | R | mult | 2 | 9851 | -6.0 [-7.1, -4.9] | -4.4 | -1.6 | 13.9 | 99.4 | 99.3 | 3.9 | lift<band | +2 contract multiplier when the opponents win a trick |
| rb-outbid-x | R | xmult | 2 | 7276 | -6.2 [-7.2, -5.1] | -4.2 | -2.0 | 13.3 | 44.0 | 45.8 | 3.1 | lift<band | ×2 contract multiplier when your team bids more than the opponents |
| seed-flat-x | R | xmult | 1 | 1699 | -7.1 [-8.5, -5.7] | -4.9 | -2.2 | 13.0 | 100.0 |  | 4.3 | lift<band | ×1.5 contract multiplier |
| ra-trump-x | R | xmult | 3 | 10602 | -7.2 [-8.3, -6.2] | -3.0 | -4.3 | 17.7 | 77.7 | 78.4 | 6.0 | lift<band | ×1.3 contract multiplier when your team wins a trick by trumping |
| seed-spade-mult | R | mult | 3 | 1765 | -7.4 [-8.9, -5.9] | -6.2 | -1.1 | 10.8 | 92.2 | 92.9 | 3.3 | lift<band | +2 contract multiplier when your team wins a trick with [♠] |
| seed-rainbow-x | R | xmult | 5 | 4879 | -7.4 [-8.7, -6.1] | -4.1 | -3.3 | 10.6 | 30.2 | 35.8 | 3.1 | lift<band | ×2 contract multiplier if your team wins tricks with all four suits |
| rb-streak-grow | R | mult | 5 | 5083 | -8.2 [-9.4, -6.9] | -4.8 | -3.4 | 8.1 | 84.3 | 84.6 | 2.8 | lift<band | This sigil gains +1 contract multiplier every time your team wins consecutive tricks (currently +0) |
| seed-five-row-x | R | xmult | 5 | 1802 | -8.5 [-10.2, -6.8] | -4.4 | -4.1 | 11.7 | 23.9 | 23.4 | 2.1 | lift<band | ×2 contract multiplier when your team wins five tricks in a row |
| rc-two-hold-x | R | xmult | 4 | 4817 | -8.7 [-9.9, -7.4] | -3.6 | -5.1 | 13.6 | 75.9 | 76.5 | 5.7 | lift<band | ×1.25 contract multiplier for each [2] your team holds |
| seed-make8-x | R | xmult | 4 | 1515 | -8.9 [-10.3, -7.5] | -5.4 | -3.5 | 12.2 | 15.8 | 17.1 | 2.4 | lift<band | ×2 contract multiplier if your team makes a contract of 8 or more |
| seed-spade-five | R | xmult | 6 | 1766 | -8.9 [-10.7, -7.1] | -6.8 | -2.1 | 9.8 | 22.6 | 24.4 | 2.9 | lift<band | ×2 contract multiplier when your team wins five tricks with [♠] |
| seed-diamond-mult | R | mult | 4 | 2347 | -9.8 [-11.0, -8.6] | -5.7 | -4.1 | 11.3 | 69.6 | 70.5 | 4.8 | lift<band | +1 contract multiplier for each trick in your team's contract when your team wins a trick with [♦] |
| seed-grow-nil | R | points | 4 | 2440 | -10.6 [-12.2, -8.9] | -8.7 | -1.8 | 3.0 | 19.6 | 24.1 | -0.4 | lift<band, rarely decisive, flat slope | This sigil gains +40 nil points every time your team makes a nil (currently +0) |
| rc-nil-contract-mult | R | mult | 3 | 5300 | -11.6 [-12.8, -10.4] | -7.1 | -4.5 | 10.1 | 24.5 | 24.4 | 2.3 | lift<band | +3 contract multiplier for each trick in your team's contract if your team makes a nil |
| ra-clubs-to-diamonds | R | enabler | 4 | 8567 | -12.9 [-14.4, -11.3] | -8.7 | -4.2 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every [♣] your team holds becomes a [♦] |
| rb-make-grow-x | R | xmult | 4 | 5538 | -13.4 [-14.7, -12.1] | -8.9 | -4.5 | 11.5 | 60.7 | 60.6 | -14.0 | lift<band | This sigil gains ×0.15 contract multiplier every time your team makes its contract (currently ×1) |
| rc-nil-grow-x | R | xmult | 4 | 4851 | -13.6 [-14.9, -12.3] | -6.5 | -7.1 | 10.0 | 25.8 | 28.1 | -2.9 | lift<band | This sigil gains ×0.5 contract multiplier every time your team makes a nil (currently ×1) |
| seed-grow-make8 | R | mult | 6 | 1555 | -13.7 [-15.1, -12.4] | -8.5 | -5.2 | 3.8 | 15.3 | 16.2 | 1.3 | lift<band, flat slope | This sigil gains +5 contract multiplier every time your team makes a contract of 8 or more (currently +0) |
| seed-grow-exact | R | mult | 4 | 3891 | -13.8 [-15.4, -12.1] | -10.2 | -3.6 | 6.5 | 24.8 | 24.6 | 1.1 | lift<band, flat slope | This sigil gains +5 contract multiplier every time your team makes its contract exactly (currently +0) |
| seed-any-suit | R | enabler | 2 | 5712 | -14.3 [-15.6, -13.0] | -9.1 | -5.2 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Your team can play any suit on the last three tricks |
| rc-junk-raise | R | enabler | 7 | 7002 | -17.2 [-18.7, -15.8] | -8.5 | -8.7 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every [2] through [9] your team holds by three ranks |
| rb-low-raise | R | enabler | 7 | 8590 | -20.2 [-21.7, -18.8] | -10.5 | -9.7 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Raise every [2] through [8] your team holds by three ranks |
| rc-twos-always-win | L | enabler | 5 | 12329 | 19.6 [18.3, 20.8] | 8.2 | 11.4 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's [2]s win every trick they are played to |
| ra-team-untrumpable | L | enabler | 2 | 13150 | 13.3 [12.0, 14.5] | 7.6 | 5.7 | 0.0 | 0.0 | 0.0 | 0.0 |  | Your team's cards can't be trumped |
| control-legendary | L | points | 1 | 2566 | 8.6 [7.7, 9.5] | 5.8 | 2.8 | 18.2 | 100.0 |  | 9.0 |  | +120 contract points |
| rc-low-win-x | L | xmult | 5 | 11643 | 2.5 [1.5, 3.5] | -1.2 | 3.7 | 21.2 | 62.5 | 63.2 | 7.5 | lift<band | ×1.5 contract multiplier when your team wins a trick with a [2] through [6] |
| rb-contract-trick-x | L | xmult | 2 | 10210 | -0.9 [-2.0, 0.2] | -2.1 | 1.3 | 23.5 | 100.0 | 100.0 | 8.1 | lift<band | ×1.15 contract multiplier for each trick in your team's contract |
| ra-ace-win-x | L | xmult | 4 | 10437 | -0.9 [-1.9, 0.1] | 0.1 | -1.0 | 24.7 | 82.8 | 83.2 | 8.7 |  | ×1.4 contract multiplier when your team wins a trick with an [A] |
| seed-flat-x2 | L | xmult | 1 | 5618 | -2.8 [-4.0, -1.7] | -3.3 | 0.4 | 19.9 | 100.0 |  | 7.5 | lift<band | ×2 contract multiplier |
| rb-exact-triple | L | xmult | 2 | 7038 | -13.1 [-14.7, -11.6] | -9.3 | -3.8 | 23.0 | 23.0 | 23.1 | 4.2 | lift<band | ×3 contract multiplier if your team makes its contract exactly |
| ra-all-diamonds | L | enabler | 4 | 5799 | -13.8 [-15.4, -12.1] | -11.3 | -2.4 | 0.0 | 0.0 | 0.0 | 0.0 | lift<band | Opening: Every card other than [♠] your team holds becomes a [♦] |
