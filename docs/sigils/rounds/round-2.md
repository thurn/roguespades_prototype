# Optimization round 2

## Brief

### Coverage additions (drafted while the round's measurement ran)

The pool has 126 sigils; uncommons (41) sit just under the soft range of 45–75. Add about ten
**uncommon** candidates aimed at the thinnest areas:

- **Suits** has only four uncommons ([♦] honors win, [♦] hold +mult, two-[♦]-wins ×, [♦]
  untrumpable). Add [♦] pieces that reward leading or holding [♦] without contract scaling.
- **Streaks** uncommons rely on "wins consecutive tricks" (fires 85%); add pieces built on
  "wins four tricks in a row" or the last trick.
- **Exact** now has `r1-exact-points` and `r1-exact-xmult`; one more decision-rich Exact or
  Nil–Exact piece is welcome.
- **Enablers** that measure well are play-freedom and card-strength rules on narrow card sets
  (`uc-low-discard`, `uc-trump-freely`, `r1-middle-free`) and source-filtered Opening
  conversions (`ub-low-to-aces`, `cb-become-spade`). Pure options (lead choice, any suit late)
  keep failing. Add two or three enablers in the strong families that serve Suits or Streaks.
- Lessons: "beats every other card of their suit" on low ranks is far over the ceiling (it makes
  low [♠]s top trumps); contract scaling stacked on another condition is sprawl the critic asked
  to remove; use the standard ranges [2] through [6] (low) and [J] through [A] (honors).

### Structural targets

Round measurement ([dashboard](../../../reports/round-2-dashboard.md)): 118,289 tier-0 boards
and a tier-1 calibration over the 126-sigil pool.

**Fun score 47.6** [46.7, 48.5], down from 55.8: Close and live 0.57 (median margin 73% of the
winner's score), Commitment works 0.50, Archetypes viable 0.68 (Ranks is in 41% of winning
flexible builds), Synergy 0.03 (the pair model finds one strong pair), Skill and bidding 0.79
(set rate 34%), Simplicity 0.23. Mean round scores peak in round 5 (7,264) and fall to 5,181
in round 8, so late rounds lose more to sets than they gain; the global scale moved 109 amounts
by ×0.70, then 60 retunes. The share of sigils inside their lift band rose from 77 of 140 to
111 of 126.

Targets:

- **Ranks dominance:** narrow `ra-jq-become-aces` and `ub-low-to-aces`; cut `ua-become-three-aces`.
- **Over the ceiling with no amount:** `uc-twos-beat` (move to rare) and `r1-middle-free`
  (narrow to [7]–[9]).
- **Cut:** `ra-side-win-points` (a generic per-trick rare), `rc-low-win-contract-points`
  (contract-scaling sprawl), `ua-diamond-two-x` (flat slope).
- **Add:** the coverage additions above.

## Outcome

Focused tournament: 81,027 tier-0 boards ([report](../../../reports/round-2-focused.md)).

| Change | Decision | Evidence (pts over the same-rarity control) |
| --- | --- | --- |
| `uc-twos-beat` to rare | Replaced by `r2-twos-beat-rare` | +12.5 at rare against +14.1 at uncommon |
| `r1-middle-free` narrowed | Replaced by `r2-high-middle-free` ([7]–[9]) | +12.3 against +14.0 |
| `ra-jq-become-aces` narrowed to [Q]s | Reverted | −3.3 against +11.6 |
| `ub-low-to-aces` to two cards | Reverted | −0.3 against +5.5 |
| Additions | Kept 6 uncommons and 1 rare | `r2-four-row-x` +17.5, `r2-diamond-honor-hold` +16.7 (both retunable), `r2-queens-beat` +10.8, `r2-opp-win-points` +10.3, `r2-diamond-seven-x` +7.0, `r2-diamond-lead-mult` +6.1; `r2-diamonds-beat` +18.4 moves to rare |
| Additions not kept | 4 | `r2-diamond-honor-lead` (equal to the simpler lead payoff), `r2-last-trick-grow` −0.3, `r2-kings-untrumpable` −2.7, `r2-last-trump-x` −3.3 |
| Cuts | 3 plus `ua-become-three-aces` | See targets |

**Pool after round 2:** 129 kept (59 common, 44 uncommon, 21 rare, 5 legendary).

**The fun score trend is down,** and the drop is mostly in families the pool changes only
indirectly: Synergy depends on a factorization machine whose pair terms are heavily
regularized, and Close and live tracks a late-run collapse in scores driven by sets under
large multipliers. Both are carried to the final round as open risks.
