# Rogue Spades 2.0: design search 2 report (a full pool)

The second 12-hour search ([plan](../design-search-plan-2.md)), on branch
`claude/design-search-2`. It holds the pool at or above 60 commons, 60 uncommons, 20 rares, and
5 legendaries, and scores designs with fun score v4, which adds **Replayability**. The
[ledger](ledger.md) lists every change tried; the [notes](notes.md) record each phase as it
happened. The GDD on this branch describes the recommendation.

## The recommendation

**Rules.** Search 1's rules with three changes: **commons cost 25 gold** (from 50), **7 sigil
slots** (search 1: 8; the base game's 7), and the **flat set** (a set scores −100 × B, untouched
by sigils). Unchanged from search 1: 5 sigil offers, 250 starting gold, card prices ×1.5, draws
with replacement. Every other lever keeps its base value.

**Pool.** **148 sigils: 62 common, 60 uncommon, 20 rare, 6 legendary**, mean complexity
C 3.16 (search 1: 96 sigils, C 3.14; base game: 117, C 3.64). It keeps 84 of search 1's 96,
adds 64, and changes the amount or tags of 41. Three rules shape it:

- **Commons add, never compound.** Every new common is `+` contract points or a `+` contract
  multiplier, or a mild enabler; ×multipliers start at uncommon.
- **Every sigil is worth buying.** No sigil measures below about +2.5 points of win-rate lift
  at common (+3.5 above); weak sigils got stronger amounts or were swapped.
- **Each archetype has a cheap enabler.** Suits, Ranks, Low Cards, and Streaks each have one or
  more common enablers that are good alone but not dominant ("Your team can play [♣]s even when
  it can follow suit", "Your team's [J]s can't be trumped", "Your team can play [2]s even when
  it can follow suit").

**On fresh final seeds the recommendation scores 85.0 at tier 1 and 85.3 at tier 2 (fun score
v4), against the base game's 70.7 and 70.9 and search 1's 78.9 and 79.1** (search 1 with its own
shop model). Paired on the same boards it gains +14.3 and +14.5 over the base game (replicates
+14.25, +11.90, +16.68 at tier 1; +15.77, +10.50, +17.09 at tier 2; every 90% interval above
zero) and +6.1 and +6.2 over search 1 (four of six replicate intervals above zero). Against
search 1 nearly all of the gain is the new Replayability family (+5.7 and +6.1 points); under
fun score v3 the two measure level (−0.4 and −0.3).

**The closest alternative is the flat set with 8 slots** (one more slot), which read 1.5–2.3
points higher on final seeds, mostly on synergy, the noisiest family; on holdout the slot count
measured no difference. I kept 7 slots: one fewer sigil per team to read and track. See
[Runners-up](#runners-up).

## The rules

| Lever | Recommended | Search 1 | Base game | Evidence (paired fun-score difference, 90% interval) |
| --- | --- | --- | --- | --- |
| Sigil prices | **25** / 75 / 100 / 150 | 50 / 75 / 100 / 150 | Same as search 1 | Commons 25: dev +2.28 [+1.22, +3.36], holdout +1.89 (t1), +1.75 (t2). Commons 35: +0.61; 75: −2.56. All halved: +1.76 but Close −2.2 |
| Set rule | **Flat** | Symmetric | Symmetric | With 7 slots, on commons-25: dev +1.71 [+0.88, +2.75], holdout +1.64 (t1), +0.75 (t2) |
| Sigil slots | **7** | 8 | 7 | Same combination; 7 alone +0.45 (dev), +0.92 (holdout); 9: −0.94 |
| Sigil offers | 5 | 5 | 3 | 4 offers −1.86, 3 offers −3.42 |
| Starting gold | 250 | 250 | 150 | 200: −1.45; 150: −1.46 |
| Card prices | ×1.5 | ×1.5 | ×1 | ×1: −1.44 |
| Offer draws | With replacement | Same | Same | Per team without replacement: no empty offers at 148 sigils, but −1.19 on the final rules |
| Rarity odds | 69 / 25 / 5 / 1% | Same | Same | 38 / 50 / 10 / 2%: −2.95 (Archetypes −3.6) |
| Everything else | Base | Base | Base | Reroll 25, card offers 2 or 4, sell value 75%, global amount scale ×0.8 or ×1.25: no gain |

## The pool

By tier and primary archetype tag (most sigils carry one to three tags; tags are internal):

| Archetype (primary tag) | Common | Uncommon | Rare | Legendary | Mean C |
| --- | --- | --- | --- | --- | --- |
| Suits ([♣] [♦] [♥]) | 11 | 9 | 2 | 1 | 3.22 |
| Spades | 6 | 7 | 3 | 0 | 3.62 |
| Ranks | 14 | 8 | 4 | 1 | 3.33 |
| Bid High | 6 | 7 | 1 | 1 | 3.07 |
| Nil | 6 | 7 | 5 | 0 | 2.78 |
| Low Cards | 7 | 5 | 3 | 1 | 3.62 |
| Streaks | 5 | 9 | 0 | 1 | 3.47 |
| Exact | 4 | 6 | 1 | 0 | 2.36 |
| Generic | 3 | 2 | 1 | 1 | 1.86 |
| **Total** | **62** | **60** | **20** | **6** | **3.16** |

Complexity: 3 sigils at C 1, 25 at C 2, 72 at C 3, 42 at C 4, 6 at C 5. Commons are 28 `+`
multipliers, 23 `+` point payoffs, 2 nil-point payoffs, 8 enablers, and one ×multiplier
(search 1's "×2 contract multiplier if your team makes its contract exactly").

**Against search 1's pool** (every cut, addition, and retune is listed in [pool-diff.txt](pool-diff.txt);
the data is in `data/sigils/` with a history entry per change):

- **Cut (12).** Four random-pick or weak commons ("Opening: Two cards your team holds become
  [A]s", "+1 contract multiplier for each trick in your team's contract", "any suit on the first
  two tricks", "+6 contract points when your team wins a trick"), the made-nil +180 common, the
  [2]–[6] win common, three C5 uncommons replaced by simpler same-slot versions ("×2.5 if your
  team holds seven [♥]s", "×2 if your team holds three [A]s", "+75 when your team wins four
  tricks in a row"), two uncommons the shop never bought ("Opening: Raise every [♠] your team
  holds by two ranks", "Your team can play [♠]s even when it can follow suit"), and the
  legendary "[K]s through [A]s can't be trumped" (the legendary tier ran one over).
- **Added (64).** 32 commons, 28 uncommons, 4 rares from the reservoir and new designs: [Q],
  [J], [10] payoffs (lead, hold, win), [♣] and [♥] holds and leads, first- and last-trick and
  consecutive-trick points, Exact and Nil commons, the cheap enablers above, and C3–C4
  uncommons for the dropped C5s ("+5 contract multiplier for each [K] your team holds",
  "+8 contract multiplier when your team wins a trick with [♦]", "×2 contract multiplier when
  your team wins the last trick of a round").
- **Retuned (41 of the kept 84).** Mostly raised: search 1 kept commons that the shop AI never
  bought ("+80 nil points" → +160, "+25 contract points when your team wins a trick with an
  [A]" → +45, "+145 when your team bids 7 or more" → +350); a few lowered after Phase 2 raised
  them too far. Odd amounts were rounded (×2.05 → ×2, +360 → +350).
- **Tags.** Bid High came off seven payoffs that don't depend on contract size.
- **Not in the pool:** Rainbow (search 1's cut stands: its shapes are few and C 4–5), compounding
  commons, and Openings that change the opponents' cards.

## Why

### How it got there

Each step paired against the one before it on the same boards (dev block 1, tier 1, 1,500 field
and 375 commitment boards, no synergy or ladder), then confirmed on holdout block 1. Every pool
carries a shop model refit the same way; the reference is search 1's rules and pool under that
refit (`search1-L`).

| Step | Dev | Holdout |
| --- | --- | --- |
| Fill to the floor: additive commons, every sigil worth buying (`p2-alive`), vs `search1-L` | +4.43 [+3.40, +5.59] | +3.88 (t1), +6.12 (t2) |
| Bid High tags corrected; simpler versions of 11 C5 sigils | +1.07 [+0.08, +1.90] | +0.79 (t1) |
| Mild common enablers for Suits and Ranks | +1.03 [+0.38, +1.83] | +1.75 (t1, the package so far) |
| Strong commons trimmed; odd amounts rounded; enablers for Low Cards and Streaks | +0.31, −0.30, −0.18 (each no measured difference; kept on judgment) | −0.05 together (Commitment +0.73) |
| Commons at 25 gold | +2.28 [+1.22, +3.36] | +1.89 (t1), +1.75 (t2) |
| 7 slots and the flat set | +1.71 [+0.88, +2.75] | +1.64 (t1), +0.75 (t2) |

What didn't work, briefly (all in the [ledger](ledger.md)): filling the floor by lift alone
pulled in about 20 compounding ×multiplier commons that every team bought (−12 to −14);
strong common enablers (the "can't be trumped" suits at common: −1.68); weaker, cheaper commons
(−2.49); rarity odds richer in rares (−2.95); fewer offers, more slots, less gold.

### What moved

- **Replayability** is the new family, and search 1's pool scored 0.38–0.43 on it on final
  seeds. A 148-sigil pool where every sigil is worth buying scores 1.00: two runs' builds
  overlap 0.081 (search 1: 0.159), the 10 most-bought sigils take 15% of purchases (33%), and
  98–99% of the pool is bought in at least 1% of runs (84–86%). A team sees 48% of the commons
  in a run (search 1: 71%). No shop ever had an empty offer.
- **Commitment works**: online by round 4 is 0.57–0.58 on final seeds against search 1's 0.39
  under the same shop-model method (0.56–0.57 with search 1's own model), mostly from cheap
  commons and common enablers.
- **Simplicity** costs about 0.3 points against search 1 (mean C 3.16 vs 3.14) despite 52 more
  sigils: the additions are C 2–3 and the C5s got simpler versions.
- **Skill and bidding**: the flat set lifts the set rate from 7–8% to 10–11%, into its band.
  Tier 2 beats tier 0 on 81% of boards (base game 93%): richer builds leave more to luck, still
  well above the 65% band.

### Where my judgment and the metric disagree

- **Commons' value per gold.** The plan asked for a common's value per gold close to an
  uncommon's. At 25 gold the recommended commons give about twice that, and every attempt to
  restore parity measured worse (−2.49 with amounts ×0.6). I followed the evidence: commons are
  cheap, simple, and clearly worth buying, which is what lets a committed team afford its plan.
  A player may find commons too obviously correct; watch for "always buy the commons".
- **The flat set.** Search 1 kept the symmetric set as the GDD's base, with the flat set within
  noise. Here the flat set plus 7 slots clears the acceptance bar, mostly through Skill and
  bidding (bolder bids). The GDD's case for the symmetric set ("a strong build that overreaches
  loses as much as it would have won") is a design value the metric can't weigh; the symmetric
  set is the first runner-up.
- **Judgment calls with no measured difference** (kept): trimming the ten strongest commons,
  rounding odd amounts (×2.05 → ×2), and cheap enablers for Low Cards and Streaks. Together they
  measured −0.05 on holdout and raised Commitment by +0.73.
- **Synergy** stays the noisiest family. Two pools that differ by a few swaps read 0.34 and 0.13
  on dev and 0.11 and 0.09 on holdout; registered combo pairs that read strong on dev mostly
  didn't replicate. I made no decision on synergy.
- **The shop model matters as much as the design.** Search 1's pool scores 76 with its own
  shop model and 72 with this search's refit method on the same boards; refitting this
  search's pool in its own context made the AI stop buying a third of it (−18). Every pool here
  was compared under one method, and the final tables show search 1 both ways.

### Runners-up

- **Flat set, 8 slots** (`ru3-slots8`). On final seeds +1.53 (tier 1) and +2.27 (tier 2) over
  the recommendation, replicates from −3.96 to +5.99, one of six intervals above zero; about
  +1.7 of it is synergy, which swings ±3 points between replicates, and +0.3 to +1.0 is Close.
  On holdout, 8 against 7 slots under the flat set measured no difference. Trade-off: one more
  sigil per team (16 on the table instead of 14) for a possibly better game. **Playtest the slot
  count**; I wouldn't be surprised if 8 plays better.
- **Symmetric set, 8 slots** (`ru3-sym`, search 1's set rule and slots with this pool and
  commons at 25). −0.86 and −1.18 against the recommendation, mostly Skill and bidding: the set
  rate falls to 7–8%, below its band. Trade-off: a set scales with the build, so a strong
  build's overbid costs what it would have won (the GDD's original intent), at the price of
  shorter, safer bids.

## Fun score by family

Final block 1, three replicates per tier. Tier 1: 1,500 field boards and 375 commitment boards
per archetype; tier 2: 500 and 125. Synergy is read at tier 0 from 400 pair-arm boards per arm on
each replicate's own boards (shared by the two tiers); the ladder (tier 2 against tier 0, 375
boards) is read once and shared. The base game is master's rules and pool with its own refit
shop model and the nil fix. Search 1 appears twice: with its own shop model, and with a model
refit the way every pool in this search was. Plain Spades has no sigils, so only its Close
family is a reference. v3 is the same measurements under fun score v3's weights.

Tier 1, 3 replicate(s) on final seeds: mean fun score, the range of the replicates' 90% intervals, and family scores (0–1, mean of replicates).

| Design | Fun (v4) | Replicate intervals | v3 | Close and live (20) | Commitment works (15) | Archetypes viable (15) | Synergy and combos (15) | Skill and bidding (5) | Simplicity (20) | Replayability (10) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base game | 70.7 | 68.7–77.7 | 72.6 | 0.89 | 0.39 | 1.00 | 0.20 | 1.00 | 0.84 | 0.73 |
| **Recommendation** | 85.0 | 81.9–92.5 | 84.8 | 0.92 | 0.95 | 1.00 | 0.21 | 1.00 | 0.96 | 1.00 |
| Runner-up: symmetric set, 8 slots | 84.1 | 81.4–94.5 | 83.5 | 0.93 | 0.96 | 0.99 | 0.20 | 0.88 | 0.96 | 0.98 |
| Runner-up: flat set, 8 slots | 86.5 | 83.3–95.6 | 86.3 | 0.94 | 0.96 | 0.98 | 0.32 | 0.96 | 0.96 | 0.98 |
| Search 1 (its own shop model) | 78.9 | 76.0–84.1 | 84.4 | 0.95 | 0.94 | 1.00 | 0.15 | 1.00 | 0.97 | 0.43 |
| Search 1 (this search's shop model) | 73.1 | 72.0–76.0 | 75.9 | 0.87 | 0.65 | 1.00 | 0.01 | 0.92 | 0.97 | 0.67 |
| Plain Spades | 56.5 | 55.1–58.0 | 65.7 | 0.61 | 0.50 | 1.00 | — | 1.00 | 0.84 | — |

Tier 1: paired difference against the base game on the same boards, per replicate, with 90% cluster-bootstrap intervals; then each family's difference in fun points (mean of replicates).

| Design | Rep 0 | Rep 1 | Rep 2 | Mean | Close and live | Commitment works | Archetypes viable | Synergy and combos | Skill and bidding | Simplicity | Replayability |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **Recommendation** | +14.25 [+9.82, +20.71] | +11.90 [+5.18, +16.42] | +16.68 [+10.52, +21.26] | +14.28 | +0.75 | +8.36 | +0.01 | +0.06 | +0.00 | +2.43 | +2.67 |
| Runner-up: symmetric set, 8 slots | +16.36 [+11.79, +23.00] | +12.46 [+7.27, +18.22] | +11.41 [+7.95, +15.78] | +13.41 | +0.79 | +8.50 | -0.17 | -0.06 | -0.58 | +2.43 | +2.51 |
| Runner-up: flat set, 8 slots | +17.57 [+11.95, +24.18] | +16.96 [+9.90, +21.97] | +12.88 [+10.44, +20.42] | +15.80 | +1.02 | +8.58 | -0.28 | +1.74 | -0.18 | +2.43 | +2.50 |
| Search 1 (its own shop model) | +6.97 [+2.61, +9.57] | +9.75 [+2.95, +13.19] | +7.74 [+3.32, +11.48] | +8.15 | +1.20 | +8.18 | +0.01 | -0.71 | -0.01 | +2.53 | -3.05 |
| Search 1 (this search's shop model) | +2.40 [-2.22, +4.19] | +2.10 [-4.80, +5.07] | +2.76 [-2.00, +5.33] | +2.42 | -0.22 | +3.89 | +0.01 | -2.79 | -0.39 | +2.53 | -0.61 |

Tier 1 sub-metrics (mean of replicates):

| Design | Margin / winner | Trailer after 5 wins | Rounds 1–4 share | Set rate | Bags per team-run | Online by round 4 | Committed win | Top archetype share | Tier 2 beats tier 0 | Mean C | Synergy gain / strong / cross | Build overlap | Concentration | Pool in play | Commons seen | Empty offers |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base game | 0.553 | 0.272 | 0.246 | 0.109 | 16.1 | 0.291 | 0.468 | 0.221 | 0.931 | 3.64 | 0.09 / 3.7 / 0.0 | 0.094 | 0.238 | 0.849 | 0.400 | 0 |
| **Recommendation** | 0.519 | 0.270 | 0.364 | 0.106 | 17.8 | 0.570 | 0.508 | 0.235 | 0.805 | 3.16 | 0.02 / 4.7 / 1.0 | 0.081 | 0.153 | 0.989 | 0.482 | 0 |
| Runner-up: symmetric set, 8 slots | 0.517 | 0.286 | 0.355 | 0.077 | 20.2 | 0.575 | 0.512 | 0.255 | 0.827 | 3.16 | -0.01 / 4.3 / 0.7 | 0.087 | 0.155 | 0.991 | 0.467 | 0 |
| Runner-up: flat set, 8 slots | 0.507 | 0.287 | 0.349 | 0.093 | 18.7 | 0.578 | 0.512 | 0.260 | 0.779 | 3.16 | 0.06 / 4.7 / 2.0 | 0.087 | 0.154 | 0.991 | 0.469 | 0 |
| Search 1 (its own shop model) | 0.499 | 0.267 | 0.266 | 0.100 | 18.2 | 0.563 | 0.506 | 0.151 | 0.896 | 3.14 | 0.08 / 2.0 / 0.0 | 0.158 | 0.326 | 0.861 | 0.705 | 0 |
| Search 1 (this search's shop model) | 0.563 | 0.300 | 0.279 | 0.085 | 18.8 | 0.391 | 0.501 | 0.217 | 0.843 | 3.14 | -0.05 / 0.7 / 0.0 | 0.150 | 0.277 | 0.951 | 0.672 | 0 |
| Plain Spades | 0.591 | 0.145 | 0.578 | 0.146 | 10.4 | 0.000 | 0.500 | 0.000 | 0.919 | 3.64 | — | — | — | — | — | — |

Tier 2, 3 replicate(s) on final seeds: mean fun score, the range of the replicates' 90% intervals, and family scores (0–1, mean of replicates).

| Design | Fun (v4) | Replicate intervals | v3 | Close and live (20) | Commitment works (15) | Archetypes viable (15) | Synergy and combos (15) | Skill and bidding (5) | Simplicity (20) | Replayability (10) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base game | 70.9 | 66.9–79.4 | 72.9 | 0.90 | 0.39 | 1.00 | 0.20 | 0.99 | 0.84 | 0.72 |
| **Recommendation** | 85.3 | 82.2–93.5 | 85.1 | 0.93 | 0.97 | 1.00 | 0.21 | 0.99 | 0.96 | 1.00 |
| Runner-up: symmetric set, 8 slots | 84.1 | 81.4–94.2 | 83.5 | 0.93 | 0.98 | 0.97 | 0.20 | 0.87 | 0.96 | 0.98 |
| Runner-up: flat set, 8 slots | 87.6 | 84.0–96.2 | 87.3 | 0.98 | 0.98 | 0.99 | 0.32 | 0.93 | 0.96 | 0.98 |
| Search 1 (its own shop model) | 79.1 | 76.1–84.0 | 84.8 | 0.99 | 0.94 | 1.00 | 0.15 | 0.94 | 0.97 | 0.38 |
| Search 1 (this search's shop model) | 73.0 | 70.6–75.9 | 75.8 | 0.92 | 0.63 | 1.00 | 0.01 | 0.87 | 0.97 | 0.63 |
| Plain Spades | 58.7 | 56.3–61.6 | 67.9 | 0.72 | 0.50 | 1.00 | — | 1.00 | 0.84 | — |

Tier 2: paired difference against the base game on the same boards, per replicate, with 90% cluster-bootstrap intervals; then each family's difference in fun points (mean of replicates).

| Design | Rep 0 | Rep 1 | Rep 2 | Mean | Close and live | Commitment works | Archetypes viable | Synergy and combos | Skill and bidding | Simplicity | Replayability |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **Recommendation** | +15.77 [+11.22, +22.22] | +10.50 [+4.37, +15.73] | +17.09 [+10.32, +22.03] | +14.45 | +0.53 | +8.68 | +0.00 | +0.06 | -0.03 | +2.43 | +2.78 |
| Runner-up: symmetric set, 8 slots | +17.97 [+13.20, +24.60] | +11.11 [+6.03, +17.64] | +10.75 [+5.97, +14.72] | +13.28 | +0.54 | +8.85 | -0.50 | -0.06 | -0.61 | +2.43 | +2.63 |
| Runner-up: flat set, 8 slots | +20.56 [+13.96, +26.57] | +16.49 [+9.27, +22.56] | +13.13 [+9.58, +20.37] | +16.72 | +1.54 | +8.80 | -0.11 | +1.74 | -0.30 | +2.43 | +2.63 |
| Search 1 (its own shop model) | +9.06 [+4.32, +12.07] | +7.95 [-0.25, +11.81] | +7.82 [+2.04, +11.00] | +8.28 | +1.80 | +8.27 | +0.00 | -0.71 | -0.27 | +2.53 | -3.34 |
| Search 1 (this search's shop model) | +4.89 [-0.31, +7.17] | -0.62 [-7.50, +2.89] | +2.00 [-3.45, +5.03] | +2.09 | +0.35 | +3.48 | +0.00 | -2.79 | -0.62 | +2.53 | -0.85 |

Tier 2 sub-metrics (mean of replicates):

| Design | Margin / winner | Trailer after 5 wins | Rounds 1–4 share | Set rate | Bags per team-run | Online by round 4 | Committed win | Top archetype share | Tier 2 beats tier 0 | Mean C | Synergy gain / strong / cross | Build overlap | Concentration | Pool in play | Commons seen | Empty offers |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Base game | 0.528 | 0.254 | 0.251 | 0.099 | 15.9 | 0.297 | 0.468 | 0.229 | 0.931 | 3.64 | 0.09 / 3.7 / 0.0 | 0.095 | 0.238 | 0.843 | 0.405 | 0 |
| **Recommendation** | 0.516 | 0.281 | 0.352 | 0.098 | 17.5 | 0.583 | 0.513 | 0.239 | 0.805 | 3.16 | 0.02 / 4.7 / 1.0 | 0.081 | 0.154 | 0.984 | 0.487 | 0 |
| Runner-up: symmetric set, 8 slots | 0.516 | 0.311 | 0.355 | 0.074 | 20.0 | 0.593 | 0.512 | 0.263 | 0.827 | 3.16 | -0.01 / 4.3 / 0.7 | 0.087 | 0.153 | 0.991 | 0.472 | 0 |
| Runner-up: flat set, 8 slots | 0.471 | 0.271 | 0.344 | 0.087 | 18.4 | 0.593 | 0.516 | 0.243 | 0.779 | 3.16 | 0.06 / 4.7 / 2.0 | 0.087 | 0.152 | 0.991 | 0.474 | 0 |
| Search 1 (its own shop model) | 0.459 | 0.265 | 0.268 | 0.088 | 18.0 | 0.569 | 0.510 | 0.161 | 0.896 | 3.14 | 0.08 / 2.0 / 0.0 | 0.159 | 0.328 | 0.840 | 0.710 | 0 |
| Search 1 (this search's shop model) | 0.525 | 0.292 | 0.280 | 0.074 | 18.5 | 0.395 | 0.490 | 0.205 | 0.843 | 3.14 | -0.05 / 0.7 / 0.0 | 0.151 | 0.277 | 0.938 | 0.677 | 0 |
| Plain Spades | 0.507 | 0.153 | 0.590 | 0.133 | 10.3 | 0.000 | 0.500 | 0.000 | 0.919 | 3.64 | — | — | — | — | — | — |

Each design minus the recommendation on the same final boards, per replicate, with 90% cluster-bootstrap intervals (v4), the v3 mean, and each family's mean difference in fun points.

**Tier 1**

| Design | Rep 0 | Rep 1 | Rep 2 | Mean | v3 mean | Close and live | Commitment works | Archetypes viable | Synergy and combos | Skill and bidding | Simplicity | Replayability |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Runner-up: symmetric set, 8 slots | +2.11 [-3.22, +9.29] | +0.56 [-2.47, +6.63] | -5.27 [-9.26, +0.68] | -0.86 | -1.28 | +0.04 | +0.14 | -0.19 | -0.13 | -0.58 | +0.00 | -0.16 |
| Runner-up: flat set, 8 slots | +3.32 [-3.52, +10.02] | +5.06 [-0.32, +9.30] | -3.80 [-6.74, +4.79] | +1.53 | +1.50 | +0.27 | +0.22 | -0.29 | +1.67 | -0.18 | +0.00 | -0.16 |
| Search 1 (its own shop model) | -7.28 [-13.25, -4.11] | -2.15 [-8.19, +1.13] | -8.93 [-14.87, -3.42] | -6.12 | -0.39 | +0.45 | -0.17 | +0.00 | -0.78 | -0.01 | +0.10 | -5.72 |
| Search 1 (this search's shop model) | -11.86 [-18.23, -9.95] | -9.80 [-14.34, -7.14] | -13.92 [-19.50, -9.44] | -11.86 | -8.95 | -0.97 | -4.47 | +0.00 | -2.86 | -0.39 | +0.10 | -3.27 |

**Tier 2**

| Design | Rep 0 | Rep 1 | Rep 2 | Mean | v3 mean | Close and live | Commitment works | Archetypes viable | Synergy and combos | Skill and bidding | Simplicity | Replayability |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Runner-up: symmetric set, 8 slots | +2.20 [-3.21, +9.37] | +0.61 [-2.46, +6.08] | -6.34 [-9.81, -0.29] | -1.18 | -1.61 | +0.02 | +0.17 | -0.50 | -0.13 | -0.58 | +0.00 | -0.15 |
| Runner-up: flat set, 8 slots | +4.78 [-2.68, +11.28] | +5.99 [+0.66, +10.34] | -3.96 [-6.44, +5.91] | +2.27 | +2.15 | +1.01 | +0.12 | -0.11 | +1.67 | -0.27 | +0.00 | -0.15 |
| Search 1 (its own shop model) | -6.71 [-13.11, -3.04] | -2.55 [-8.68, +1.19] | -9.27 [-14.68, -3.42] | -6.18 | -0.28 | +1.28 | -0.42 | +0.00 | -0.78 | -0.24 | +0.10 | -6.12 |
| Search 1 (this search's shop model) | -10.88 [-17.77, -8.16] | -11.12 [-15.59, -7.94] | -15.09 [-19.42, -9.99] | -12.36 | -9.29 | -0.18 | -5.20 | +0.00 | -2.86 | -0.59 | +0.10 | -3.63 |


## What the metrics can't see

- **Table load.** 148 sigils, 5 sigil offers and 3 card offers per shop, 7 slots per team, and
  14 public sigils on the table. The simplicity score prices each sigil, not the table or the
  catalog. A player meets each common in fewer than half their runs (commons seen per run about
  48%), so the pool will feel large; whether that reads as discovery or noise is a playtest
  question.
- **"Commons are obviously correct."** At 25 gold every common is worth its price, and the AI
  buys and later sells them as filler. A human may find the common row a chore, or a pleasure.
- **The flat set** makes a big build's bid safe: a set costs −100 × B however strong the build.
  The AI bids bolder (set rate about 11% against 7%). Does a late-run set still sting?
- **The shop AI churns.** It buys by value each shop and sells its weakest to make room, so
  flexible builds are mixed rather than planned. Committed builds come online in 58% of runs,
  but a human plans differently.
- **Synergy is underpowered as measured** (400–800 pair-arm boards, ±6,000 margin points per
  pair). The strongest pairs are compounding ×multipliers on correlated events ("×3 when your
  team bids more than the opponents" with the streak-scaled contract multiplier).
- **Nil and bags** are as search 1 left them: nil is bid in nearly half of team-rounds, and the
  1,000-point bag penalty is small once multipliers scale scores.

**Playtest focus:** whether the common row feels like choice or chores; whether 148 sigils feel
like variety or noise; whether the flat set keeps late bids tense; whether the cheap enablers
("play [♣]s even when you can follow suit") read as a plan; whether Ranks' [Q] / [J] / [10]
payoffs give the archetype a second and third rank worth collecting.

## Open questions for the designer

1. **Set rule.** The flat set wins here, mostly on Skill and bidding, against the GDD's
   "a strong build that overreaches loses as much as it would have won". Which value matters
   more? The symmetric-set runner-up is the same design otherwise.
2. **Common pricing as a principle.** Commons at 25 measured best of every price tried, and
   commons stronger per gold than uncommons. Should the soft rule be "a common is worth about
   two-thirds of an uncommon" or "commons are cheap filler that's always worth it"?
3. **Pool size.** The floor (60 / 60 / 20 / 5) works once every sigil is worth buying; most of
   Replayability's gain comes from that, not from size alone. Is 148 the right size for a
   human, or would ~110 with the same rules read better?
4. **Rainbow.** The grammar has only two Rainbow shapes ("first trick with each suit", "all
   four suits"), both C 4–5, so a minor Rainbow archetype can't reach 5 commons and 5 uncommons
   within the simplicity ceilings. Drop it for good, or add a simpler shape to the grammar?
5. **Nil scoring, nil gold, and bags** (fixed), as in search 1.
6. **The shop model.** The fun score of a design moves by 4–18 points with how the AI's shop
   values are fitted. A shop AI that plans a build (and rerolls) would make the economy levers
   and the Commitment family more trustworthy.
7. **Synergy target.** Fifteen strong pairs on 400–800 boards per arm is beyond what the
   measurement can resolve; consider judging synergy on designed, registered combos only.

## Playable build

In this worktree (`.claude/worktrees/design-search-2`, branch `claude/design-search-2`) the game
data is the recommendation: `data/rules.json`, the 148-sigil pool in `data/sigils/` (every sigil
named), and the shop model `data/models/search-final.json`, which the client and `rsim play`
read. The 64 sigils new to the pool got names and icons (`data/search/pool/rec2-names.tsv`).

```bash
npm --prefix viewer run dev -- --port 5174
```

Then open `http://localhost:5174/?seed=7`. Seeds worth playing (tier-0 autoplay finishes):
7 (52,350 to 57,425) and 42 (125,180 to 165,898) were close; 11 (37,255 to 2,770) stayed
low-scoring; 99 (56,547 to 404,756) ran away. Add `&tier=2` to face the shipped AI. Master's
client is the base game; search 1's is in `.claude/worktrees/design-search`. The runners-up are
variants `ru3-sym` and `ru3-slots8` in `data/search/variants/` (to play one, set
`flatSet` / `slots` in `data/rules.json`).
