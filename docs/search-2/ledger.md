# Design search 2 ledger

Every change tried. Paired differences are candidate minus incumbent in fun score v4 points with
90% cluster-bootstrap intervals, on the boards of one pipeline run. Dev = `dev` block 1, holdout =
`holdout` block 1 (`data/search/seeds.json`). Tier 1 unless noted.

## Measurement changes

| Change | Why | Re-measured |
| --- | --- | --- |
| Fun score v4: Replayability (build overlap, concentration, pool in play), weights 20/15/15/15/5/20/10 | Search plan 2 | Phase 0 dev check below |
| Run records list every sigil offer a shop drew (rerolls included) | Exact commons-seen and empty-offer diagnostics | — |
| Build overlap pairs builds from different boards only | A board's two seatings play the same deals, so their builds aren't independent runs | — |
| AI nil handicap 1,100 in `data/models/search-1.json` (the default measurement model) | Search 1's calibration, ported | — |

Phase 0 dev check (tier 1, 1,500 field and 375 commitment boards, ladder at tier 2 on 375): base
game overlap 0.094, concentration 0.239, in play 0.838 (Replayability 0.71); search 1 0.159, 0.330,
0.875 (0.44). Search 1 against the base game: +9.54 [+7.91, +9.80] in v4 (v3 +12.88), with
Replayability −2.70. The bootstrap's percentile intervals for Replayability sit slightly off the
point estimate: resampled boards repeat, which nudges concentration and in-play (threshold
statistics); paired differences are affected far less.
