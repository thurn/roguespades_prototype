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

## Pool assembly (Phase 2)

Dev, tier 1, 1,500 field and 375 commitment boards, no ladder or synergy. Each pool carries its
own shop model refit from the Phase 1 lifts; the reference is search 1's rules and pool with a
refit made the same way (`search1-L`: fun 71.8, Replayability 0.64).

| Pool | Rule | Size (c/u/r/l) | Dev vs `search1-L` | Holdout | Decision | Reason |
| --- | --- | --- | --- | --- | --- | --- |
| `p2-coverage` | Coverage first | 66/62/20/6 | −12.64 [−13.46, −11.54] (Arch −7.5, Rep −2.8) | — | reject | ~18 ×mult commons; Suits 71% of winning builds |
| `p2-value` | Value first | 60/60/20/6 | −14.25 [−14.81, −12.82] (Arch −7.5, Close −1.4) | — | reject | Same; Suits 74% |
| `p2-restore` | Restore first (Rainbow back) | 60/60/20/6 | −14.03 [−14.52, −12.47] | — | reject | Same; Suits 78% |
| `p2-additive` | Coverage, no new ×mult commons | 65/60/20/6 | −3.76 [−4.92, −2.50] (Close +1.5, Arch −2.1) | — | reject | Ranks 32% of winning builds; 42 sigils never bought |
| `p2-alive` | Additive coverage; weak amounts raised, strong commons lowered, dead commons and uncommons swapped | 62/60/20/6 | **+4.43 [+3.40, +5.59]** (Rep +3.3, Commit +2.0, Close +1.8, Arch −1.2, Skill −0.7, Simpl −0.8) | t1 +3.88 [+2.79, +5.08]; t2 +6.12 [+4.13, +8.22] | **accept: incumbent** | Every sigil worth buying; additive commons; v3 diff ≈ 0 (−0.05 t1, +1.52 t2), so the gain is Replayability and Commitment against Simplicity and Archetypes |
| `p2-alivex` | Alive with ×mult commons | 64/60/20/6 | −8.39 [−9.28, −7.29] (Arch −7.5) | — | reject | Suits 60% of winning builds |

## Rules on the full pool (Phase 3)

Screen: dev, tier 1, 1,000 field and 250 commitment boards per level, against `p2-alive-L`
(rules: 5 sigil offers, 8 slots, 250 gold, card prices ×1.5, symmetric set, draws with
replacement). No level had empty offers.

| Change | Dev | Overlap / conc / in play | Decision | Reason |
| --- | --- | --- | --- | --- |
| 3 sigil offers | −3.42 [−4.88, −2.05] (Commit −3.4, Close −1.0) | 0.074 / 0.17 / 1.00 | reject | Fewer offers: commitment collapses |
| 4 sigil offers | −0.75 [−2.28, +0.49] (Commit −1.5) | 0.081 / 0.18 / 0.99 | reject | No gain; Commitment lower |
| 7 slots | −0.76 [−2.06, +0.31] (Close −0.7) | 0.084 / 0.19 / 0.99 | reject | No gain |
| 9 slots | −0.08 [−1.36, +1.15] | 0.093 / 0.19 / 0.99 | reject | No measured difference; more to track |
| Per-team draws without replacement | +0.16 [−1.17, +1.37] (Commit +0.8, Rep −0.3) | 0.097 / 0.21 / 0.99 | combine test | No empty offers at 148 sigils; commons seen 57% |
| Shared draws | −0.41 [−1.73, +0.93] | 0.083 / 0.18 / 0.99 | reject | No measured difference |
| 200 starting gold | −0.17 [−1.40, +1.00] | 0.089 / 0.20 / 0.99 | reject | No measured difference |
| 150 starting gold | −1.46 [−2.59, −0.18] (Close −1.1) | 0.088 / 0.20 / 0.97 | reject | Worse |
| Card prices ×1 | −1.44 [−2.55, +0.09] (Close −1.0) | 0.091 / 0.19 / 0.99 | reject | Worse, as in search 1 |
| Richer odds (38/50/10/2) | −2.95 [−4.41, −1.58] (Arch −3.6, Commit +2.0) | 0.104 / 0.22 / 0.99 | reject | Rares and legendaries dominate builds |
| Flat set | +0.49 [−0.79, +1.86] (Skill +0.8: set rate up) | 0.089 / 0.19 / 0.99 | combine test | v3 +1.30 |

Combine (dev, tier 1, 1,500 field and 375 commitment boards, against `p2-alive-L`):

| Change | Dev | Decision | Reason |
| --- | --- | --- | --- |
| Flat set | −0.13 [−1.15, +0.73] (Skill +0.7, Close −0.7) | reject | No measured difference at full size; the symmetric set stays (runner-up, as in search 1) |
| Flat set + per-team draws | +0.04 [−0.96, +1.00] (Commit +0.8, Skill +0.8, Arch −0.9, Rep −0.3) | reject | No measured difference |

Phase 3 decision: the rules stay as search 1 left them. Nothing to confirm on holdout.

## Pool refinement (Phase 4)

Dev, tier 1, 1,500 field and 375 commitment boards, paired against the incumbent named.

| Change | Vs | Dev | Holdout | Decision | Reason |
| --- | --- | --- | --- | --- | --- |
| P4a: Bid High tag off 7 payoffs that don't depend on contract size (trick wins with [♠] or [A], four in a row, hold seven [♠], [J]–[A] wins, bids nil) | `p2-alive-L` | +0.94 [+0.32, +1.51] (Arch +1.2) | — | accept | Tags are internal: this corrects the archetype readings, not the game. Bid High's share of winning builds was inflated by sigils any plan buys |
| P4b: simpler same-slot versions for 11 C5 sigils (four/three in a row, hold three or four [A]s, hold seven [♥], [J]–[A] wins, make-8 per trick, win seven tricks, low-led wins, [2]–[6] wins) | `p2-alive-L` | +1.66 [+0.75, +2.50] (Simpl +0.6, Arch +1.2) | — | accept | Mean C 3.30 → 3.18; the replacements are C2–C4 and measured live |
| P4a + P4b together (`p4ab`) | `p2-alive-L` | +1.07 [+0.08, +1.90] (Arch +1.2, Simpl +0.6) | +0.79 [−0.30, +1.64] vs `p2-alive-L` (t1) | accept: incumbent | Both changes; Archetypes was already full with P4b alone |
| P4c: two common enablers (swap five cards with your partner; [K]s can't be trumped) for two dead-ish commons | `p2-alive-L` | +1.02 [+0.03, +1.98], vs `p4ab` ≈ −0.05 (Close −0.8) | — | reject | Online for Suits and Ranks moved 0.33 → 0.36 and 0.37 → 0.38: no measured difference over P4ab |
| Global amount scale ×0.8 | `p4ab` | +0.49 [−0.07, +1.13] | — | reject | No measured difference; ×1 stays |
| Global amount scale ×1.25 | `p4ab` | −0.06 [−0.65, +0.55] | — | reject | No measured difference |
| P4d: the four "can't be trumped" enablers ([♣], [♦], [♥], [A]) move to common (lift +9 to +12 there); two new uncommons fill their slots; four weak commons out | `p4ab` | −1.68 [−2.47, −0.63] (Rep −0.9, Arch −1.1, Commit +0.5, Skill +0.6) | — | reject | Suits online 0.33 → 0.50, but every team buys strong common enablers: concentration 0.19 → 0.23, Suits 29% of winning builds |
| P4e: three modest common enablers ("play [♣]s / [♥]s even when you can follow suit", "[J]s can't be trumped") plus "[K]s can't be trumped" for four weak commons (per-contract-trick +1, made-nil +360, +10 per trick won, bid-8 +25) | `p4ab` | +1.03 [+0.38, +1.83] (Commit +0.8, Skill +0.3) | +1.75 [+0.83, +2.57] vs `p2-alive-L` (t1; Commit −0.03) | accept: incumbent | Online 0.43 → 0.47 with concentration unchanged (0.18); lifts alone +3.4 to +6.5 |
| P4f: ten commons above +7 trimmed to ×0.7 of their amount (re-measured +3.2 to +7.1) | `p4e-L` | +0.31 [−0.60, +0.96] (Rep +0.1, Commit +0.2) | — | accept on judgment | No measured difference; brings commons' lift per gold (0.109) back toward uncommons' (0.094), the plan's aim |
| P4g: two more common enablers ("[9]s can't be trumped", "[J]s beat every other card of their suit", lifts +7.5 and +10.4) for two Streaks point commons | `p4f-L` | −0.39 [−1.46, +0.59] (Close −0.8, Skill +0.4) | — | reject | No measured difference; strong commons again |
| Runner-up check: flat set on `p4f-L` | `p4f-L` | +0.47 [−0.27, +1.39] (v3 +1.50; Skill +1.0) | — | runner-up | Third time within noise; raises the set rate into band |
| Runner-up check: per-team draws without replacement on `p4f-L` | `p4f-L` | −0.40 [−1.26, +0.48] (Close −0.6, Commit +0.3) | — | runner-up | No measured difference; "never the same sigil twice in a run" |
| P4h: six odd amounts rounded (×2.05 → ×2, ×1.85 / ×1.8 → ×1.75, ×1.9 → ×2, +260 → +250, +360 → +350, +24 → +25) | `p4f-L` | −0.30 [−0.75, +0.39] | — | accept on judgment | No measured difference; numbers a player can read at a glance |

Tier-2 holdout of the refined pool (500 field, 125 commitment boards, against `search1-L`):
`p2-alive-L` +6.12 [+4.13, +8.22]; `p4h` **+7.78 [+6.58, +9.84]** (v3 +2.74; Commit +3.0,
Close +1.6, Rep +4.2, Skill −0.8). Phase 4's changes hold their sign at tier 2.
| P4i: two common enablers for LowCards and Streaks ("play [2]s / [10]s even when you can follow suit", +6.2 / +7.1 alone) for two redundant commons (a second [♥]-win payoff, make-7 points) | `p4h` | −0.18 [−0.98, +0.55] (Commit +0.5 [+0.3, +0.7], Simpl +0.1, Close −0.4) | — | accept on judgment | No measured difference overall; the plan asks for a cheap enabler per archetype, and Commitment's gain excludes zero |
| P4j: two Exact enablers ("play any suit once your team has won its contract", "any suit on the last four tricks"; +1.2 and +0.5 alone) for an Exact and a Suits common | `p4i-L` | +0.25 [−0.72, +1.06] (Commit −0.2) | — | reject | Exact online didn't move; both are weak alone |

Synergy milestone 2 (800 pair-arm boards, tier 0, dev; totals include synergy, against `search1-L`):
`p2-alive-L` +8.81 [+3.58, +11.85] (Synergy 0.34: 9 strong pairs, 2 cross, gain +0.01);
`p4i-L` +7.63 [+5.02, +14.71] (Synergy 0.13: 3 strong, 1 cross, gain −0.04). The Phase 4 swaps
changed which pairs are sampled; a holdout replicate follows.
Holdout replicate (800 boards): `p2-alive-L` Synergy 0.11, `p4i-L` 0.09; `p4i-L` against
`p2-alive-L` +1.37 [−3.90, +5.62] with synergy (Synergy −0.33 [−5.82, +4.01]; Commit +0.7,
Simpl +0.7). The dev gap in synergy (0.34 vs 0.13) didn't replicate: noise, as in search 1.
The same strong pairs read in both pools, at lower interaction in `p4i-L` (outbid ×3 with the
streak-scaled contract multiplier: +58,909 vs +33,872 margin points).
| P4k: two Exact common payoffs (+200 if your team makes its contract exactly; +15 when the opponents win a trick) for two [K] Ranks commons | `p4i-L` | +0.04 [−0.90, +1.11] (Commit −0.2) | — | reject | Exact online didn't move (Exact teams lack payoffs and enablers alike; one more common of each isn't enough) |
| Holdout check of P4f + P4h + P4i together (`p4i-L` vs `p4e-L`) | `p4e-L` | — | −0.05 [−0.93, +0.84] (Commit +0.73 [+0.51, +0.93], Arch −0.8, Simpl +0.1, Rep +0.2) | keep | The three judgment calls measure no difference together; the Commitment gain replicates |
| Sigil prices 25 / 40 / 50 / 75 | `p4i-L` | +1.76 [+0.77, +2.76] (Commit +2.1, Skill +1.1, Arch +0.9, Close −2.2) | — | screen | Bigger builds: the late rounds swamp the early ones |
| Commons at 25 gold (25 / 75 / 100 / 150) | `p4i-L` | **+2.28 [+1.22, +3.36]** (Commit +2.2, Skill +0.3, Close −0.4) | t1 +1.89 [+1.05, +2.74] (Commit +1.9); t2 +1.75 [+0.17, +2.99] (Commit +2.2) | **accept: incumbent** | Committed teams reach their pieces; Close barely moves |
| Sigil prices 75 / 110 / 150 / 225 | `p4i-L` | −1.18 [−2.28, −0.25] (Commit −2.2, Close +0.6) | — | reject | |
| Commons at 75 gold | `p4i-L` | −2.56 [−3.59, −1.64] (Commit −2.2, Close −1.0) | — | reject | |
| Register 18 combo pairs (14 designed enabler + payoff or payoff + payoff pairs, plus the pairs that read strong on dev in both `p2-alive-L` and `p4i-L`) before a confirming read | `p4i-L` | — | Synergy 0.09 → 0.11 on holdout (800 boards): one registered pair reads strong | keep registered | Game unchanged; the dev-strong pairs mostly don't replicate, as search 1 found |
| Commons 25, uncommons 50 (25 / 50 / 100 / 150) | `p4i-L` | +2.80 [+1.79, +3.51] (Commit +2.1, Arch +0.9, Skill +0.7, Close −0.7) | — | runner-up material | Within noise of commons-only at 25; cheaper uncommons add little and cost some Close |
| Commons at 35 gold | `p4i-L` | +0.61 [−0.40, +1.52] (Commit +1.1) | — | reject | The gain scales with how cheap commons are |
| Commons at 25 with their amounts ×0.6 (value per gold back toward uncommons') | `r4-prices-c25` | −2.49 [−3.37, −1.49] (Commit −1.9, Close −1.2) | — | reject | Cheap commons help because they're good, not because they're many |

Rules on the commons-at-25 incumbent (dev, 1,500 / 375 boards, against `r4-prices-c25`):

| Change | Dev | Decision | Reason |
| --- | --- | --- | --- |
| 200 starting gold | −1.45 [−2.29, −0.49] (Arch −0.9, Commit −0.6) | reject | 250 stays |
| 4 sigil offers | −1.86 [−2.81, −0.85] (Commit −2.3) | reject | 5 stays |
| 9 slots | −0.94 [−1.93, −0.07] (Arch −1.1, Close +0.9) | reject | |
| 7 slots | +0.45 [−0.50, +1.40] (Skill +0.5, Arch +0.5, Close −0.6) | holdout t1 +0.92 [−0.03, +1.87] | see combination | |
| Flat set | +0.85 [−0.11, +1.81] (v3 +1.74; Skill +0.9) | holdout t1 +1.73 [+0.89, +2.52]; t2 −0.05 [−1.14, +1.50] | see combination | |
| Per-team draws | −1.19 [−2.17, −0.18] (Arch −1.1, Commit +0.4) | reject | No longer a runner-up |
| 7 slots + flat set | **+1.71 [+0.88, +2.75]** (Skill +1.1, Arch +0.5) | t1 +1.64 [+0.77, +2.38]; t2 +0.75 [−0.48, +2.32] | **accept: incumbent** | Gain excludes zero on dev and holdout tier 1, same sign at tier 2. The set rate rises into its band (bolder bids); one fewer slot to track; a set no longer scales with sigils, a simpler rule |
| Shop model refit from scratch in the final design's own context (all 148 sigils granted at shop 3 under the final rules and pool, raw lift) | `rec3` | −18.34 [−19.56, −17.75] (Rep −6.5, Arch −7.6, Commit −5.4) | — | reject (measurement) | In a pool of strong sigils a free grant displaces a good purchase, so raw lifts of most sigils go negative and the AI stops buying a third of the pool. The recommendation keeps the model built from search-1-context lifts on one scale |

Last economy screen on the final design (dev, 1,500 / 375 boards, against `rec3`):

| Change | Dev | Decision | Reason |
| --- | --- | --- | --- |
| Reroll 25 | +0.10 [−0.68, +0.62] | reject | No measured difference; the shop AI rerolls at most once |
| 4 card offers | −0.91 [−1.69, −0.41] (Close −0.6) | reject | |
| Sell value 75% | −0.46 [−1.29, +0.05] | reject | |
| 2 card offers | −0.11 [−0.93, +0.45] | reject | No measured difference; base value kept |
