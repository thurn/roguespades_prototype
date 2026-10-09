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

## Phase 1: the candidate reservoir

- **Screen** (`rsa.reservoir`, `data/search/pool/reservoir.json`): of 496 non-pool sigils in search
  1's data plus 49 new designs, 263 passed: 152 commons, 70 uncommons, 29 rares, 12 legendaries.
  Out: 90 repeated a signature at their rarity, 120 exceeded the C ceiling (C 4 at common, 5
  above), 14 random Openings, 8 controls, 1 catch-up trigger ("if your team is behind").
- 22 of the 49 new designs repeat a signature the enumerated reservoir already had ([♣] and [♥]
  holds and wins, [Q]/[J]/[10] holds and wins, last-trick points); the enumerated versions were
  measured instead. 27 new designs were measured.
- **Lifts** (tier 0, granted at shop 3, 1,000 boards, same boards for every arm; ±2 points 90%):
  incumbent commons average +1.5 (uncommons +5.2, rares +5.8, legendaries +8.2). Candidates are
  wider: the enumerated reservoir carries amounts nobody tuned, so 23 candidates measured above
  +12 (up to +42 for "×1.25 contract multiplier for each [2] through [10] your team holds").
- **Retarget** (`rsa.retarget`): those 23 were scaled toward a per-rarity target lift (common 3.5,
  uncommon 5, rare 7, legendary 10) and re-measured. Most landed at +2 to +7. Per-card ×mults
  that bottom out at ×1.05 ("for each card other than [♠] your team holds", "for each [2] through
  [10]") stay far above the ceiling and are out.
- Surprises: "Opening: Raise every [♣] your team holds by two ranks" measured −3.7 and the [♥]
  version −0.9: raising side-suit cards seems to cost more than it gives (a raised card can land
  on a rank another card already holds and duplicates are worthless; or the AI's bids don't
  adjust). "Your team's [Q]s can't be trumped" at common measured +5.2, and "[♣]s / [♦]s can't
  be trumped" +8.3 / +8.9 at uncommon: cheap enablers that are good alone exist after all.
- **Baseline** (Phase 0 check, same dev block): base game Replayability 0.71, search 1 0.44.

## Phase 2: fill to the floor

- **Measurement bug, fixed before any decision:** a variant derived from a pool variant
  (`p2-value-L` from `p2-value`) silently fell back to the 96 "kept" sigils, because only an
  explicit `pool` list was read and it wasn't inherited. Pools now inherit through `base`. The
  first comparison was stopped and rerun.
- **Reference:** search 1's rules and pool with a shop model refit the same way as every new pool
  (`search1-L`, from the Phase 1 lifts). It scores 71.8 against 76.3 with search 1's own model:
  my refit values sigils about 35% higher on average (0.096 vs 0.070), so the commitment arm's
  fixed archetype bonus weighs less and committed builds come online less (0.38 vs 0.55). Every
  pool is compared with the same refit method, so the comparisons are fair; the level isn't
  comparable with search 1's numbers.
- **Coverage / value / restore** (each at the floor): −12.6, −14.3, −14.0 against `search1-L`.
  Every rule added about 20 ×mult commons from the enumerated reservoir ("×1.25 contract
  multiplier when your team leads [♣]"), each under the lift ceiling alone. Every team bought
  them, Suits (the tag on most of them) took 71–74% of winning builds, and a sample game ran to
  ×349 in round 8.
- **Additive commons** (coverage, no new ×mult commons): −3.76 [−4.92, −2.50]; Close +1.5, but
  Ranks took 32% of winning builds (lead [J]/[Q]/[10] commons at lift +7) and 42 of 151 sigils
  were never bought.
- **Surprise: the shop AI is nearly deterministic.** Per sigil, the share of offers that end in a
  purchase is bimodal: 1–2% (the 5% exploration) below a lift of about +2.5 at common, 85–98%
  above it. Pool in play is therefore a direct readout of how many sigils the model values above
  their price. Dead sigils need stronger amounts or a swap, not more offers.
- **Alive** (`rsa.assemble coverage --alive --additive-commons`, pool `p2-alive`): every sigil
  must be worth buying. 57 weak pool sigils had their amounts raised toward the rarity target
  (`tune2`, capped at ×2.5, ×2 for a non-positive lift) and 14 over-strong commons lowered
  (`tune3`, half-way back where that overshot: `tune4`); then commons and uncommons still below
  the alive line were swapped for live candidates of the same archetype (4 swaps). Result
  against `search1-L`: **+4.43 [+3.40, +5.59]** on dev: Replayability +3.3 (overlap 0.089,
  concentration 0.19, in play 99%), Commitment +2.0 (online 0.46 vs 0.38), Close +1.8;
  Archetypes −1.2 (Bid High 29% of winning builds), Skill −0.7 (set rate 7.2%, below its 10% band),
  Simplicity −0.8 (mean C 3.30).
- The same rule with ×mult commons allowed (`p2-alivex`) scored −8.39: Suits 60% of winning
  builds again. Compounding commons are the problem, not their amounts.
- Measurement note: a lift's raw component carries the opportunity cost of the purchase the grant
  displaces, which grows when the whole pool gets stronger (the common control's raw lift is −0.6
  in search 1's pool, −3.3 after raising every weak sigil). Lift models built from several reports
  now put every row on one scale: lift over control plus the reference report's control offset.
- Game log (`p2-alive-L`, seed 7, tier 1): every shop had two or three plausible buys; commons
  were bought early and sold later for uncommons and rares (the team ended with 2 of its 9
  commons); builds were mixed rather than one plan, as the flexible AI shops by value.
- **Holdout** (block 1): `p2-alive-L` against `search1-L` +3.88 [+2.79, +5.08] at tier 1 and
  +6.12 [+4.13, +8.22] at tier 2 (500 field, 125 commitment boards). It is the Phase 2 incumbent:
  62 / 60 / 20 / 6, mean C 3.30. Under v3 it measures no difference from search 1 (−0.05, +1.52):
  the gain is the scored Replayability and Commitment, paid for partly in Simplicity and
  Archetypes.
- Next: rules on the full pool (Phase 3), then Archetypes (Bid High 29% of winning builds), the
  set rate (7%, below band), and Simplicity in Phase 4.

## Phase 3: rules for a full pool

- Eleven levels on `p2-alive-L` (dev, 1,000 / 250 boards). Nothing beat search 1's rules. The
  pool size didn't move the levers that matter: 3 or 4 offers still cost Commitment (−3.4, −1.5),
  150 gold and card prices ×1 still cost Close. Richer rarity odds raised Commitment (+2.0) but
  rares and legendaries took over winning builds (Archetypes −3.6).
- **Offer draws** now work mechanically: per-team draws without replacement left no empty offers
  at 148 sigils (commons seen per run 57%, against 46% with replacement), but measured no
  difference (+0.16 alone, +0.04 with the flat set). With replacement stays.
- **Flat set** again measured no difference (+0.49 screen, −0.13 at full size) while raising the
  set rate into its band (Skill +0.7) and costing a little Close. Same verdict as search 1:
  runner-up material, not a change.
- Surprise: none. The economy levers measure the same on a 148-sigil pool as on a 96-sigil one.

## Phase 4: pool refinement at the floor

- **Tags (P4a, +0.94):** Bid High was on seven payoffs that reward something else (trick wins
  with [♠] or [A], four in a row, hold seven [♠], bid nil). Removing it fixed the top archetype
  share. This changes only the internal archetype readings, never the game; I accept it as a
  correction and say so here because it moves a scored family.
- **Simplify (P4b, +1.66):** 11 of the 17 C5 sigils got simpler same-slot versions at the same
  rarity and archetype (for example "×1.5 if your team holds three [A]s" → "+8 contract
  multiplier when your team wins a trick with a [K]"; "×2.5 if your team holds seven [♥]s" →
  "+8 contract multiplier when your team wins a trick with [♦]"). The six C5 sigils left
  (four in a row ×3, last trick with [♠], hold seven [♠], [A]-win per contract trick, [2]–[9]
  wins, last-trick growth) are each an archetype's signature piece.
- **Synergy milestone** (800 pair-arm boards, tier 0, dev): `search1-L` 0.05 (gain +0.02, 1 strong
  pair), `p4ab` 0.09 (gain −0.04, 4 strong, 0 cross); paired +0.60 [−2.58, +4.59], no measured
  difference. With synergy included `p4ab` leads `search1-L` by +6.10 [+2.29, +10.16].
- **Registered combos** (14 enabler-plus-payoff and payoff-plus-payoff pairs, read at 800 boards
  before any decision): "[♦]s can't be trumped" + "+75 contract points when your team wins a trick
  with [♦]" read +0.86 of its parts; the Exact points-and-×mult pair +0.41; three [A]s from an
  Opening + [A]-win points +0.45. But "[A]s can't be trumped" with either [A]-win payoff, and the
  [♠] Openings with [♠]-hold payoffs, read negative. At ±6,000 margin points per pair these are
  hints, not results; I didn't tune toward them.
- **Commitment bottleneck:** for committed Suits and Ranks teams, 44–46% of runs hold two or more
  payoffs by shop 4 but no enabler. Enablers are uncommon or rarer, so a committed team rarely
  sees one in four shops. Two extra common enablers (P4c: swap five cards; [K]s can't be trumped)
  moved online by only 0.01–0.03.
- **Common enablers, second try (P4e, +1.03, holdout +1.75 for the package):** milder enablers
  that are good alone but not dominant ("play [♣]s / [♥]s even when you can follow suit",
  "[J]s / [K]s can't be trumped", lifts +3.4 to +6.5) raised online 0.43 → 0.47 without moving
  concentration. Strong ones (P4d, the "can't be trumped" suits at common, +9 to +12; P4g, "[9]s
  can't be trumped" and "[J]s beat every other card of their suit") were bought by everyone.
  The line between the two is roughly a common's lift of +7.
- **Commons per gold (P4f):** after Phase 2's raises, commons measured +0.109 lift per gold against
  uncommons' +0.094. Trimming the ten strongest commons to ×0.7 brought them back in line at no
  measured cost.
- **Rules on the refined pool:** flat set +0.47 (v3 +1.50), per-team draws −0.40. The flat set has
  now measured within noise three times, always raising the set rate into its band.
- **Global scale ×0.8 / ×1.25:** no measured difference either way.
- **Phase 5 false start (01:27):** I launched the final-seed runs, then stopped them two minutes
  later, before any final-seed result was computed or read: Phase 4 had run about two of its
  4.5 hours. Phase 4 continued. (The game data and GDD were already updated to `rec2` = `p4i-L`;
  they will be re-applied if the recommendation changes.)
- **Sigil prices (the clearest Phase 4 find):** commons at 25 gold, everything else unchanged,
  measured +2.28 on dev and +1.89 / +1.75 on holdout at tiers 1 and 2, almost all of it
  Commitment (online 0.49 → 0.58–0.60): a committed team can afford the common pieces of its plan
  alongside the uncommons. The effect is monotone in the common price (75: −2.56; 50: 0; 35:
  +0.61; 25: +2.28). All prices halved also gained (+1.76) but cost Close (−2.2): bigger builds
  make the late rounds swamp the early ones. Commons at 25 with their amounts cut to ×0.6 lost
  −2.49: cheap commons help because they're good, not because there are more of them.
- **Where the metric and the plan disagree:** the plan asks for a common's value per gold close
  to an uncommon's. At 25 gold the recommended commons give about twice an uncommon's lift per
  gold, and every attempt to restore parity (trimming amounts) measured worse. I followed the
  evidence: commons are cheap, simple, additive, and clearly worth buying; uncommons and up are
  where ×multipliers and rule benders live.
- **Registered combos:** registering 18 pairs before a holdout read added one strong pair
  (Synergy 0.09 → 0.11). The pairs that read strong on dev mostly didn't replicate.
- **Rules again, on cheaper commons:** with commons at 25, 200 gold, 4 offers, and 9 slots all
  measured worse, but 7 slots (+0.45, holdout +0.92) and the flat set (+0.85, holdout +1.73)
  both leaned positive, and together measured +1.71 [+0.88, +2.75] on dev, +1.64 on holdout
  tier 1 and +0.75 at tier 2. Most of it is Skill and bidding: under the symmetric set the AI
  bids short (set rate 7%, below the 10% band); a flat set lets a strong build bid boldly. I
  accepted both. This reverses search 1's call on the set rule, which it made with the flat
  set within noise every time; here the combination clears the acceptance bar. The symmetric
  set stays the first runner-up, because the GDD's "a strong build that overreaches loses as
  much as it would have won" is a real design value the metric can't weigh.
- **Phase 5 second false start (02:42):** launched and stopped within a minute (nothing computed
  or read) to give Phase 4 a last economy screen (reroll, card offers, sell value: nothing).
  Phase 4 ran about 3 hours 20 minutes in all, under its 4.5-hour target: the last eight
  changes I tried measured no difference or worse, and the final runs plus the report need the
  remaining time with margin.

## Phase 5: recommendation

- Final block 1, tiers 1 and 2, three replicates: the recommendation (`rec3`), runners-up
  (symmetric set with 8 slots; flat set with 8 slots), the base game, search 1 with its own shop
  model and with this search's, and plain Spades.
- Measurement fix during the run: `rsa.phase5` first shared replicate 0's synergy read across all
  replicates, which would turn synergy's noise (±0.3 of the family on 400 boards) into a fixed
  offset instead of averaging it. Replicates 1 and 2 read synergy on their own boards; replicate
  0 is unchanged. The ladder stays shared (Skill and bidding is saturated).
- **Tier 1** (three replicates): the recommendation scores 85.0 (v3 84.8) against the base
  game's 70.7; paired +14.25, +11.90, +16.68, every interval above zero. Commitment +8.4,
  Replayability +2.7, Simplicity +2.4, Close +0.8.
- Search 1 scores 78.9 with its own shop model and 73.1 with this search's. Under v3 it scores
  84.4 with its own model, level with the recommendation's 84.8: the gain over search 1 is the
  new Replayability family (1.00 vs 0.43) plus cheaper commons, not the old families.
- Surprise: with its own model search 1 reaches online 0.56 on final seeds, the recommendation's
  level; under this search's refit it reaches 0.39. The Commitment gap I measured against
  `search1-L` throughout is partly the shop model.
- The flat-set-with-8-slots runner-up reads 86.5, ahead on synergy (0.32 vs 0.21) and Close.
  Its holdout evidence was mixed (flat set alone +1.73 at tier 1, −0.05 at tier 2, against the
  symmetric set with 8 slots), so final seeds confirm a close call rather than settle it.
- **Tier 2** agrees: 85.3 against the base game's 70.9 (+15.77, +10.50, +17.09) and search 1's
  79.1 (own model). Direct pairs against the recommendation: flat set with 8 slots +1.53 (t1),
  +2.27 (t2), one of six replicate intervals above zero, mostly synergy; symmetric set with 8
  slots −0.86, −1.18 (Skill and bidding); search 1 −6.1, −6.2 (v3: −0.4, −0.3).
- The recommendation stands as declared: final seeds confirm a close call on the slot count
  rather than settle it.
- Close-out: the game data, client, `rsim play` default model, GDD, and registry describe the
  recommendation; `rsim check`'s worked examples follow the rules file's set rule; `scripts/ci`
  passes.
