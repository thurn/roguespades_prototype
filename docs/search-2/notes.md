# Design search 2 notes

One entry per phase: what I looked at, what surprised me, and what I'd look at next.

## Phase 0: infrastructure on master

- Ported search 1's measurement tooling (lift-refit shop model, sigil metrics, synergy table,
  composition, rarity scaling, run stats, Phase 4/5 tables, apply), its AI controls and
  calibrations (nil handicap 1,100, refitted margin curve, 10 kernel slots, `rsim play --rules`),
  and its measurement fixes. Rules, pool, and GDD stayed at master. Landed as `f178f6f`.
- Fun score v4 adds Replayability on the field runs. The dev check reproduces the plan's table:
  overlap 0.094 / 0.159, concentration 24% / 33%, pool in play 84% / 88% (plan: 85% / 85%) for
  the base game / search 1. Under v4 search 1 still beats the base game by +9.5 (v3: +12.9);
  Replayability costs it 2.7 points.
- Surprise: search 1 offers each team 71% of its 36 commons in one run (base game 40% of 64): the
  small pool repeats itself.
- Routine comparisons skip the tier-2 ladder: it takes longer than the rest of a variant's
  measurement and Skill and bidding (now weight 5) was saturated throughout search 1.
