# Rogue Spades 2.0: design search plan 2 (a full pool)

A second 12-hour, time-boxed search. It holds the pool at **at least 60 common, 60 uncommon, 20
rare, and 5 legendary sigils** and ends in the agent's recommendation of the best design at that
size.

The [first search](design-search-plan.md) (branch `claude/design-search`, report at
`docs/search/report.md` on that branch) improved the fun score by cutting the pool to 96 sigils
with 36 commons. The fun score rewarded that because it couldn't see repetition: two runs' builds
overlapped 70% more than in the base game, and a player saw about 70% of all commons every run.
This search keeps the pool large and adds a scored **Replayability** family to the fun score.

As before, metrics are a rough approximation of fun. The deliverable is a reasoned opinion, backed
by evidence, not a number that cleared a bar.

## Outcome

- **A recommended design:** one rules file (`data/rules.json`) and one pool at or above the floor,
  with the reasons for each choice, what the metrics said, and what they can't see.
- **One or two runners-up** where the evidence is close, each with its trade-off stated plainly.
- **A playable build** of the recommendation, to compare with master and with search 1.
- **A report**, `docs/search-2/report.md`, plus a ledger of everything tried.

## Hard rules

- **The pool floor.** Every pool the search accepts as an incumbent has at least 60 commons, 60
  uncommons, 20 rares, and 5 legendaries. A tier may run a few over its floor (up to about
  65 / 65 / 22 / 6) when that buys coverage, but never as filler. A cut is a swap: whatever leaves
  a tier at its floor is replaced in the same step. Candidate pools below the floor may be measured
  for diagnosis but can't be accepted.
- **Search only the table.** Every value outside
  [Open for testing](game-design.md#open-for-testing) is fixed, as in search 1. If the evidence
  points at a change outside the table, report it as a question for the designer and don't build
  it.
- **Sigils have no activated abilities,** and every sigil follows GDD §5.
- **Metrics are approximations.** Say "no measured difference", never "costs nothing" or "adds
  nothing". Never pick a design you'd find ugly to explain because it scored a point higher. If the
  metric and your judgment disagree, say so and explain why.
- **Don't edit the fun score to raise it.** Fun score v4 (v3 plus Replayability, below) is fixed
  in Phase 0, before any candidate is measured, and doesn't change after that. Measurement fixes
  are fine; record them in the ledger and re-measure the incumbent afterward.
- **Commits.** Phase 0 lands on master through Tollgate. Everything after it stays on the search
  worktree's branch (`claude/design-search-2`), committed with Conventional Commits after
  `scripts/ci`; the designer decides what to promote.
- Code is disposable; write no tests.

## Starting point

Search 1's branch, `claude/design-search`, is the source of infrastructure and evidence. Its
recommended design is a starting incumbent, not a constraint.

- **Infrastructure to port** (measurement and AI calibration only, not design):
  - the per-pool shop model refit from single-grant lifts (`rsa.liftmodel`, incremental with
    `--vs`), and `rsa.sigilmetrics`, `rsa.syntable`, `rsa.compose`, `rsa.rarityscale`,
    `rsa.runstats`, `rsa.phase4`, `rsa.phase4sum`, `rsa.apply`;
  - the refitted margin-to-win curve (`data/models/search-1.json`) and the AI nil handicap of
    1,100;
  - the measurement fixes: no control sigils in field shops, no forced opponent grants in
    commitment arms, committed Suits shoppers valuing every side suit's honors;
  - the never-nil and bag-blind AI controls, `rsim play --rules`, and 10 kernel sigil slots;
  - effect overrides that name a type replace the effect outright (`it.Variant`).
- **Starting incumbent:** search 1's rules (5 sigil offers, 250 starting gold, card prices ×1.5,
  8 slots, symmetric set) and its 96-sigil pool (36 / 37 / 16 / 7). It is below the floor, so the
  first accepted incumbent is the Phase 2 fill.
- **The reservoir:** about 400 cut sigils in `data/sigils/` (284 commons, 85 uncommons, 29 rares,
  10 legendaries), search 1's 23 cuts including the Rainbow archetype, and new designs within the
  grammar.
- **Evidence worth knowing** (from search 1; a prior, not a result):
  - Commons are near dead weight: granted free at shop 3, a common moved win rate by +0.35 points
    (uncommon +4.2, rare +5.4, legendary +13). Making every common stronger hurt Commitment
    (×2: −2.94) because off-plan commons crowd out archetype pieces; ×1.5 let the Streaks commons
    take over. A full common tier has to matter without being generic.
  - Opening effects that pick cards at random ("Three cards your team holds become [♦]s",
    "[2]s", "[K]s") measure negative alone: they turn trumps into side cards and high cards into
    low ones. Avoid them as cheap enablers.
  - The free-discard family ("can play [2]s through [5]s even when it can follow suit") is too
    strong for common; every team buys it.
  - Per-team offer draws without replacement emptied late shops with 96 sigils (0–2 sigil offers
    in the last shops). With about 150 they may work; check the empty-offer rate before trusting
    the score.
  - Synergy is the noisiest family: at 300–400 pair-arm boards a pair's interaction carries about
    ±6,000 margin points and the family swings ±4–6 fun points. Strong pairs are compounding
    ×multipliers on events that happen together. Most sigils carry two to four archetype tags, so
    few pairs are truly cross-archetype.
  - The shop AI rerolls at most once per shop and ends runs with about 300 unspent gold; it can't
    see the reroll step.
  - Nil is bid in about 46% of team-rounds even after the handicap fix; that's the fixed nil rules,
    not the bidder.

## Fun score v4: Replayability

Fun score v4 adds a seventh family, **Replayability**: "two runs feel different." Each sub-metric
scores 1 inside its band and falls linearly to 0 at its tolerance, like every other family, and is
read on the field runs (both teams, every run).

| Sub-metric | Measured as | Full credit | Zero at | Master's base game | Search 1 |
| --- | --- | --- | --- | --- | --- |
| Build overlap | Mean Jaccard overlap of the sigils two random runs bought | ≤ 0.08 | ≥ 0.20 | 0.094 | 0.159 |
| Concentration | Share of all purchases going to the 10 most-bought sigils | ≤ 20% | ≥ 40% | 24% | 33% |
| Pool in play | Share of pool sigils bought in at least 1% of runs | ≥ 95% | ≤ 75% | 85% | 85% |

The family's weight comes from the two families that told search 1 the least: Skill and bidding was
saturated in every variant, and Simplicity was the score that rewarded cutting the pool.

| Family | v3 weight | v4 weight |
| --- | --- | --- |
| Close and live | 20 | 20 |
| Commitment works | 15 | 15 |
| Archetypes viable | 15 | 15 |
| Synergy and combos | 15 | 15 |
| Skill and bidding | 10 | 5 |
| Simplicity | 25 | 20 |
| **Replayability** | — | **10** |

The weights and bands are a proposal for the designer; adjust them before Phase 0 ends, not after.
Two more diagnostics are reported beside the score, not scored:

| Diagnostic | Measured as | Guardrail |
| --- | --- | --- |
| Commons seen | Share of the common tier offered to a team in one run | Report; about 55% or less expected at the floor |
| Empty offers | Sigil offer slots a shop couldn't fill | 0; a variant with empty offers is rejected |

## Phase 0: infrastructure on master (target: 1 hour)

1. Port the infrastructure above from `claude/design-search` onto master, without its rules,
   pool, or GDD changes. Keep `data/rules.json` and `data/sigils/` at master.
2. Implement fun score v4 in `analysis/rsa/funscore.py` and `rsa.it` (field arrays, families,
   `fmt_measure`, the sweep and Phase 5 tables), and update GDD §10 to match. Record offers shown
   per shop in the run records so the commons-seen and empty-offer diagnostics are exact. Re-read
   master's base game and search 1's recommendation under v4 on dev to check the bands produce the
   numbers in the table above.
3. Add fresh seed blocks in `data/search/seeds.json`: dev block 1, holdout block 1, final block 1.
   Search 1 used block 0 of each.
4. Run `scripts/ci`, commit, deliver with `tg push-master --wait`, then create the search
   worktree from master.

## Phase 1: the candidate reservoir (target: 1.5 hours)

1. **Screen out** reservoir sigils that fail the grammar or GDD §5, pick cards at random in an
   Opening, repeat another candidate's signature at the same rarity, or exceed C 4 at common or
   C 5 above common.
2. **Draft** new candidates where the reservoir is thin. Priorities: commons that are worth buying
   alone and belong to one archetype; nouns the pool barely uses ([♣], [♥] holdings, [Q], [J],
   [10], the last trick); a cheap enabler for each archetype that is good alone; Exact and Rainbow
   payoffs if those archetypes stay.
3. **Measure** every candidate's single-grant lift and fire rate at tier 0 under the starting
   rules (about 1,000 boards each; about 250 candidates is roughly an hour of compute), against the
   same-rarity flat control.
4. **Read the replayability baseline:** v4 for master's base game and search 1's recommendation
   on these seeds.

## Phase 2: fill to the floor (target: 2.5 hours)

1. **Assemble two or three pools at the floor** from the incumbent plus screened candidates, each
   by an explicit rule, for example:
   - *Coverage first:* each major archetype gets about 10 commons and 9 uncommons, each minor about
     5 and 5; every archetype gets at least one enabler good alone at common or uncommon and at
     least two payoffs with card filters.
   - *Value first:* the highest-lift candidates per tier, subject to the power ceiling and to no
     archetype holding more than a sixth of a tier.
   - *Restore first:* search 1's cuts (including Rainbow) back in, then the best candidates.
2. Refit each pool's shop model and measure each on dev against search 1's recommendation.
3. **Look past the score:** read a few game logs per pool. Do shops offer real choices? Do commons
   get bought and kept? Does any archetype show up every run?
4. Confirm the best assembly on holdout at tiers 1 and 2. It becomes the incumbent.

## Phase 3: rules for a full pool (target: 1.5 hours)

On the Phase 2 incumbent. A bigger pool changes how often a plan's pieces appear, so re-screen
the levers search 1 moved and the ones that interact with pool size: sigil offers (3–5), slots
(7–9), offer draws (all three, watching empty offers), rarity odds, starting gold, card prices, and
the set rule. Combine the two or three clearest, then confirm on holdout at tiers 1 and 2.

## Phase 4: pool refinement at the floor (target: at least 4.5 hours)

This is where most of the time goes, plus any time the other phases leave over. Every step keeps
the floor: a cut is a swap.

- **Amounts.** Re-fit the global scale once, then individual sigils, at most one retune per sigil
  per pass. Aim for commons that matter: a common's value per gold close to an uncommon's, without
  one archetype's commons taking over (watch the top archetype share and concentration).
- **Simplify.** For each sigil above about C 4, try a simpler same-slot version. Prefer the simpler
  one unless the complex one is clearly better on evidence and on feel.
- **Swap** sigils that are weak, redundant, or far outside the sigil metrics for better reservoir
  candidates or new designs.
- **Synergy and commitment.** Design deliberate combo pairs and payoffs that reward going deep in
  one archetype, register candidate pairs before the confirming read, and read synergy at 800 or
  more pair-arm boards at milestones only.
- **Replayability.** Push dead sigils and concentration down: a sigil nobody buys is either too
  weak or redundant; a sigil everybody buys is too generic or too strong.

## Phase 5: recommendation (target: 1 hour, never skipped)

1. Run the recommendation and runners-up on final block 1 at tiers 1 and 2, three replicates,
   against master's base game and search 1's recommendation.
2. Leave a playable build: apply the recommendation to `data/` in the worktree, point the client
   at its shop model, and list seeds worth playing for
   `npm --prefix viewer run dev -- --port 5174`.
3. Write `docs/search-2/report.md`:
   - **the recommended design** as a rules table and pool summary (counts by tier and archetype,
     mean C), and every difference from the base game and from search 1;
   - **why:** the evidence, your judgment, and where they disagree;
   - **the fun score by family (v4, with v3 beside it) and the diagnostics**, with intervals, for the
     recommendation, the runners-up, the base game, search 1's recommendation, and plain Spades;
   - **what the metrics can't see,** and what to look for in playtests;
   - **open questions** outside the table, for the designer.
4. Update the GDD on the branch so it describes the recommendation.

## Statistical protocol

- **Seeds.** Block 1 of dev, holdout, and final in `data/search/seeds.json`. Holdout confirms;
  final is used once, in Phase 5. Don't start the final runs until the recommendation is fixed.
- **Paired comparisons.** A candidate and the incumbent play the same boards in the same run of
  the pipeline, each with its own refitted shop model. Report the paired difference with a 90%
  cluster-bootstrap interval for the total and every family. Never compare a number with one from
  an earlier run.
- **Size runs to the decision.** Tier 1 at 1,500 field and 375 commitment boards gives a paired
  half-width of about 1 fun point without synergy. Leave synergy out of routine comparisons and
  read it at 800 boards at milestones.
- **Accepting a change:** a gain whose interval excludes zero on dev, the same sign on holdout,
  no unexplained drop in a family, and no empty offers. A change with no measured
  difference may still be kept, or dropped, on judgment; say which and why.
- **Throughput** (18 cores): a tier-1 comparison of two variants without synergy takes about 6
  minutes; a synergy read at 800 boards about 8 minutes per variant; a full lift-model refit about
  8 minutes, an incremental one about a minute.

## Reporting during the run

- `docs/search-2/ledger.md`: every change tried, its dev and holdout estimates, the decision, and
  the reason.
- `docs/search-2/notes.md`: a short entry per phase, covering what you looked at, what surprised
  you, and what you'd look at next.
- Commit a checkpoint at the end of each phase.

## When time or usage runs out

Phases 0–3 and 5 are time-boxed; if one finishes early, the spare time goes to Phase 4. At 12
hours, stop searching and finish Phase 5 with what you have. If the usage limit nears first, write
`HANDOFF.md` in the worktree with the current phase, the incumbent rules and pool, open
experiments, and the next step.
