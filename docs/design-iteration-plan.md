# Rogue Spades 2.0: design iteration plan to a fun score of 70

This plan iterates on the whole game design, not just the sigil pool, until the fun score
([GDD §12](game-design.md#fun-score)) reaches **70** on evidence that is real: replicated on fresh
seeds, robust to the strength and style of the AI players, and backed by a playable build the
designer can compare against the current version.

It follows the [sigil design plan](sigil-design-plan.md), which ended at **51.5**. That run
showed that more sigil rounds alone will not get there: the biggest gaps sit in the rules, the
shop, and the measurements themselves. This plan fixes the measurements first, then works outward
from rules to economy to sigils.

The orchestrator runs every phase in order **without pausing for review**, in its own worktree.
Each iteration ends with a checkpoint commit on the worktree branch and a playable build.

## Outcome

- **A fun score of at least 70** under the frozen definition below, measured at tier 2 on held-out
  seeds, with every family at or above its baseline.
- **A rules file** (`data/rules.json`) that holds every gameplay lever this plan touches, read by
  both the simulator and the browser game, so a rules change is data, not code.
- **A playable branch build** to play side by side with the baseline on master.
- **An evidence ledger:** every change tried, its predicted and confirmed effect, and why it was
  kept or reverted, so a later reader can tell real gains from luck.
- **An updated GDD**, sigil data, and a final report in `docs/iteration/`.

## Starting point

**Baseline:** master at the commit the worktree branches from (the sigil design plan ended at `b96b3eb`). Fun score **51.5**:

| Family (weight) | Score | Points | Sub-metrics |
| --- | --- | --- | --- |
| Close and live (25) | 0.66 | 16.5 | median margin / winner **0.72** (scores 0); trailer after round 5 wins 0.25 (at its band edge); rounds 1–4 share 0.33 |
| Commitment works (15) | 0.30 | 4.5 | online by round 4 **0.30** (scores 0); committed win rate **0.44** |
| Archetypes viable (15) | 1.00 | 15.0 | all committed archetypes 40–60%; largest winning share 0.22 |
| Synergy and combos (15) | 0.00 | 0.0 | no strong pairs |
| Skill and bidding (15) | 0.80 | 12.0 | ladder 0.82; late overtricks 0.95; set rate **0.34** |
| Simplicity (15) | 0.23 | 3.4 | mean S 0.23; S = 1 / (1 + C), so a realistic ceiling is about 0.3 |

**What earlier diagnostics established:**

- **Seed noise is small.** Three fresh-seed replicates of the round-3 arms gave fun scores of
  52.3, 51.5, and 51.4 (sd about 0.4); the committed win rate's sd was about 0.01. The
  round-to-round moves (55.8 → 47.6 → 51.5) were real, but each round also changed the shop model
  and the global amount scale, so they cannot be attributed to the pool.
- **The published interval covered two families only.** The fun score's bootstrap resampled
  Close and live and Skill and bidding; Commitment, Archetypes, and Synergy had no error bars.
- **The set rate is not a search-strength artifact.** Tier 1 vs tier 1 and tier 2 vs tier 2 both
  set 34% of contracts, rising from 12–15% in round 1 to about 58% in round 8; leading teams are
  set 31%, trailing teams 42%. Scoring rewards the gamble: a made contract scores
  (10 × bid + contract points) × multiplier, a set loses only 10 × bid × multiplier.
- **Bidding is the same weak method at every tier.** `choose_bid_explain`
  (`sim/src/ai/search.rs`) scores three candidates near a heuristic trick estimate with heuristic
  rollouts (10, 40, or 160 by tier); tier search never touches bidding. A shared bidding bias would
  survive the tier test above.
- **Synergy's zero is reproducible** (both halves of round 3 agree), but no one has checked that
  the factorization machine can detect a synergy at all.
- **The shop model is weak:** its rank correlation with rollout rescoring stayed near zero, and
  Commitment works is measured through the committed shop policy.

## Hard rules

- **Sigils have no activated abilities.**
- **Every design decision is validated by simulation** against the target metrics; designer
  judgment proposes, evidence decides, and both are recorded.
- **The fun score definition is frozen** (see [The target](#the-target)). The orchestrator never
  edits a band, weight, or formula to raise the score.
- **The holdout seeds are only for confirmation** (see [Statistical protocol](#statistical-protocol)).
- **Commits stay on the worktree branch.** Commit each checkpoint with Conventional Commits and run
  `scripts/ci`, but do **not** submit to Tollgate or merge to master: master is the baseline for
  playtests. The designer decides when to promote. (This overrides the usual delivery rule for
  this effort only.)
- Treat code as disposable and write no tests, as `AGENTS.md` says.

## The target

**Fun score v1** is the formula in `analysis/rsa/funscore.py` at the baseline commit, with these
measurement-only fixes allowed, because they change how precisely or validly a sub-metric is
estimated, not what it means:

1. **Intervals for every family** (all arms and the synergy fit join the bootstrap).
2. **Ladder at tier 2 vs tier 0**, as the GDD specifies, instead of tier 1 vs tier 0.
3. **Estimator replacement** for a sub-metric that fails its validity test in Phase A (for
   example, synergy measured by direct pair arms if the factorization machine misses a planted
   synergy). The band stays the same.

Anything else (a new band, a new weight, a rescaled Simplicity family) is a **v2 proposal**:
compute it alongside v1 in every report, argue for it in the final report, and let the designer
decide. **The goal stays on v1.**

**Reached** means all of these hold on the final candidate:

- v1 fun score **≥ 70** at tier 2 on fresh holdout seeds, with the 90% lower bound **≥ 68.5**;
- **≥ 68** at tier 1 on the same holdout, so the result does not rest on one AI strength;
- **≥ 67** under the risk-neutral utility (see [AI validity](#ai-validity));
- every family **≥ its baseline score** minus its noise (two replicate sd);
- three independent holdout replicates agree within their intervals.

## Statistical protocol

The aim is that every number that drives a decision is real.

- **Seed sets.** Create three disjoint, recorded seed ranges in `data/iteration/seeds.json`:
  - **dev** for screening and tuning (used freely);
  - **holdout** for confirming accepted changes (each use logged in `docs/iteration/holdout-log.md`);
  - **final** for the stopping check only.
  Retire a holdout block after 20 uses and draw a new one; repeated looks leak.
- **Same environment within a comparison.** Candidate and incumbent run with the same shop
  model, global scale, AI tiers, utility, and seeds. Never compare a score with one measured in an
  earlier iteration; **re-measure the incumbent every iteration** alongside its challengers. (The
  sigil run's 55.8 → 51.5 trend violated this and cannot be read.)
- **Paired comparisons.** Each candidate plays the same boards as the incumbent; report the paired
  difference and its 90% interval from a cluster bootstrap over boards, for the total and every
  family.
- **Sample sizes from the effect, not habit.** The smallest gain worth keeping is **1 fun point**.
  Size runs so the paired 90% half-width is at most 0.5 points at tier 1; measure the replicate sd
  in Phase A and recompute. Longer runs are fine: tier-1 arms run about 85 runs/s, tier 2 about
  13.5, tier 0 about 1,000 (18 cores).
- **Accepting a change** takes all of:
  1. a paired gain whose 90% interval excludes zero on **dev** seeds;
  2. the same sign and at least half the size on **holdout** seeds;
  3. no family dropping by more than two replicate sd (a regression must be traded explicitly in
     the iteration notes, never silently).
- **The winner's curse ledger.** For every accepted change, record the dev estimate and the
  holdout estimate. If the holdout gains average less than half the dev gains over five changes,
  double sample sizes and say so in the next iteration notes.
- **A/A tests.** Every third iteration, submit the incumbent as its own challenger. More than 1 in
  10 A/A "gains" passing the dev gate means the gate is too loose; tighten it.
- **Planted effects (positive controls).** In Phase A and every fifth iteration, plant a known
  change and confirm the pipeline finds it in the right family with roughly the right size:
  - a deliberate combo pair (for example "+N contract multiplier for each [♠] your team holds" with
    "Opening: three cards your team holds become [♠]s") for Synergy;
  - a flat power boost to one archetype's payoffs for Archetypes viable and Commitment works;
  - a harsher set penalty for the set rate.
  A family whose estimator misses its plant is **invalid** until fixed; changes may not be accepted
  on an invalid family's evidence.
- **Damped amounts.** At most one retune per sigil per two iterations, unless its reading is more
  than twice its interval width from the band; apply global scale and relative retunes in the same
  step, never alternating (the sigil run oscillated).
- **Reproducibility.** Every reported number names its config, seeds, code commit, and rules file
  hash.

## AI validity

The fun score is measured through AI players. A change counts only if it improves the game, not
just how the AI copes with it.

### Principles

- **Measure under the best available play.** A metric about what players can achieve (committed
  win rate, power ceiling, combo value) is read with the strongest policy available for that
  question, tuned toward a best response before it is read. A weak policy understates what humans
  will find.
- **Invariance across AI.** A real improvement shows at tier 1 **and** tier 2, and under both the
  win-probability and the risk-neutral utility. Gains that appear for one AI only are recorded as
  AI artifacts and not accepted.
- **Large skill gradients are warnings.** A sigil or rule whose effect differs by more than 3 pts
  between tiers is flagged; its readings need tier-2 confirmation.

### Diagnostics (Phase A, repeated at milestones)

1. **Bid calibration.** For each tier, bucket bids by the bidder's predicted make probability and
   compare with realized make rates; report by round and score state.
2. **Bid-offset test.** Add a per-team bid offset to the run config. Play offset −1, 0, +1 teams
   against the normal AI in duplicate at tier 1 and tier 2. If an offset team wins significantly
   more, the AI's bidding is miscalibrated: fix it before trusting Skill and bidding or Close and
   live (for example, widen the candidate set, use the tier's own search to value bids, or
   calibrate the trick estimator to the realized results).
3. **Risk-neutral control.** Add a utility option that maximizes expected margin instead of win
   probability. Humans are rarely as risk-seeking late as a pure win-probability player, so every
   set-rate and closeness reading is reported under both.
4. **Sigil awareness.** For each kept sigil, compare its lift with and without the AI knowing its
   own sigils (search over the true rules versus the base rules). Sigils that gain nothing from
   awareness while humans plainly would are AI blind spots; list them for playtests.
5. **Shop policy sensitivity.** Sweep the committed shopper's archetype bonus (0, ½×, 1×, 2×) and
   pick the best per archetype before reading Commitment works and Archetypes viable. If Commitment
   works swings by more than 0.2 with the bonus, it is measuring the shopper; fix the shopper first.
6. **Degenerate-strategy hunters.** Fixed exploit policies (always bid high, always nil when
   possible, hoard gold, rush one archetype) play the field each milestone. Any hunter above 55%
   is a rules problem to fix before accepting further gains, because humans will find it.
7. **Baseline sanity.** Measure the fun-score families on plain Spades (no sigils, no shop) with
   equal AIs. If a sub-metric is far outside its band there, note in the v2 proposals that its band
   may be miscalibrated for this game, but keep v1 as the goal.

### Human proxies

The simulator cannot measure everything. These stay human, and the orchestrator does not claim
them:

- P28 simplicity calibration (time to explain, prediction errors);
- P30 enabler preference;
- whether games feel close, whether sets feel fair, whether sigil rules read clearly.

Designer playtests feed the loop as **hypotheses** that simulation then tests (see
[Playtesting](#playtesting-against-the-baseline)).

## Levers

Work outward: rules first (they set the stage every sigil plays on), then the shop, then the
pool. Each lever lists the families it should move; a lever that moves nothing in its sweep is
dropped.

### Rules (Close and live, Skill and bidding)

- **Set penalty.** Variants: the multiplier applies to a set at a reduced rate; a set also
  forfeits a share of the round's +multipliers; a set costs 10 × contract plus a share of the
  contract points it would have earned; a flat floor per set. Goal: set rate into 10–25% at tier 2
  without killing bid tension.
- **Multiplier schedule** (P29): starting multiplier 1, 5, 10, 15, and the +mult reward scale with
  it.
- **Par growth** (P20) and **rounds per run** (P1).
- **Contract gold** (P11) and **nil gold** (P12): rewards for making contracts rather than bidding
  them.
- **Comeback mechanics** if closeness stays short: income or offer bonuses for the trailing team.
  Prefer economy levers over score rubber-banding.

### Shop and economy (Commitment works, Archetypes viable)

- **Offer density (D7):** offers per shop (P4), rarity odds (P14), reroll cost (P7),
  **affinity weighting** (offers lean toward archetypes the team already holds), and **run pools**
  (each run draws a subset of archetypes).
- **Slots** (P3) and **purchases per shop** (P6).
- **Prices** (P15) and **sell value** (P8).

### Sigil pool (Synergy, Commitment works, Skill and bidding)

Only after the rules and shop are settled:

- **Bid-scaled pieces** to reach P23's targets (a third of contract-point sigils, a quarter of
  multipliers), designed as clean Bid High payoffs rather than riders.
- **Exact** pieces beyond "makes its contract exactly".
- **Designed combos:** pairs and triples created on purpose, each with a measured joint gain.
- **Legendary build-arounds** that change how a run is played.
- **Re-tune or cut** sigils left far from their bands by the new rules.

Follow the sigil plan's design rules, roles, and data format for every sigil change.

### Measurement fixes (not game changes)

AI bidding, shop policy, and estimator fixes from Phase A are made as needed. They change the
measurement, so after any of them **re-baseline**: re-measure the incumbent and restart the
winner's curse ledger.

## Phases

### Phase A: instruments and baseline (no design changes)

1. **Rules file.** Move every lever above into `data/rules.json`, read by `sim/` and the wasm
   client (`sim/web`), with defaults equal to today's constants. Check that a run with the default
   file reproduces the baseline records exactly. The game client imports a fixed shop model
   (`MODEL_STEP` in `viewer/src/game/engine.ts`); point it at the branch's latest model whenever a
   new one is exported, and have the client import the rules file the same way.
2. **Measurement upgrades:** intervals for every family, the tier-2 ladder, seed sets, a paired
   comparison command that takes two rules or pool variants and returns the paired fun-score
   difference by family.
3. **AI diagnostics 1–7** above, with fixes where they fail.
4. **Validity tests:** A/A and planted effects for every family.
5. **Like-for-like pool test:** measure the round-1 and round-3 pools (from git history) under
   identical model, scale, and seeds, to learn whether the sigil rounds helped or hurt.
6. **Re-baseline** the incumbent at tiers 1 and 2, under both utilities, on holdout seeds.

**Exit:** every family has a valid estimator and an interval; the replicate sd is measured and
sample sizes set; the incumbent's v1 score is known at both tiers. Write
`docs/iteration/phase-a.md`.

### Phase B: rules sweep

1. **One-factor sweeps** of each rules lever on dev seeds at tier 1 (4–6 levels each).
2. **Response surface** on the 2–3 strongest levers: a small factorial around their best levels.
3. **Confirm** the best combination on holdout seeds at tiers 1 and 2 and under both utilities.
4. **Re-run the degenerate-strategy hunters** on the new rules.

### Phase C: shop and economy sweep

The same procedure for the shop and economy levers, on top of the Phase B rules. Re-tune the
committed shopper (diagnostic 5) before reading Commitment works.

### Phase D: sigil iterations

Optimization rounds in the sigil plan's style, with this plan's protocol: each iteration proposes
a handful of changes aimed at the families furthest from their bands, screens them on dev seeds,
and confirms them on holdout seeds. The global amount scale is re-fitted once at the start of the
phase for the new rules, then damped.

### Phase E: confirmation

Run the stopping check on **final** seeds: tier 2, tier 1, and risk-neutral, three replicates.
Then write the final report, update the GDD (decisions and parameters with their evidence), and
leave a playtest build.

### Iterations and order

Phases B–D run as **iterations** of about 1–3 hours of compute each. Each iteration:

1. reads `docs/iteration/playtests.md` for new designer notes;
2. re-measures the incumbent;
3. proposes changes (at most 6), aimed at the largest weighted gap to target;
4. screens on dev seeds, confirms on holdout seeds, applies the acceptance rules;
5. writes `docs/iteration/iter-NN.md`: changes tried, kept, and reverted with reasons; the family
   table with intervals at both tiers; the ledger; any v2 proposals;
6. commits a checkpoint on the branch.

Phases may be revisited: if sigil work in Phase D moves the set rate or closeness, a short rules
re-sweep is allowed.

### Stopping

- **Success:** the target is reached on **final** seeds as defined in [The target](#the-target).
- **Plateau:** four consecutive iterations with less than 1 point of confirmed gain. Before
  stopping, try one broader move (a lever family not yet swept, or a structural rules change);
  if that also fails, stop and report the gap, what limits each family, and the v2 proposals.
- **Budget:** stop after 40 iterations or about 5 days of compute, whichever comes first.

Never declare success on a dev or holdout score.

## Playtesting against the baseline

- **Prerequisite.** The browser game client (`sim/src/session.rs`, `sim/web/`,
  `viewer/src/game/`) must be on master before the worktree branches. If the worktree lacks it,
  stop and ask the designer to land it first; do not rebuild it on the branch.
- **Two builds.** In the master checkout: `npm --prefix viewer run dev` (port 5173). In the
  worktree: `npm --prefix viewer run dev -- --port 5174`. Each serves the game at `/` and its own
  pool at `/sigils`; Vite rebuilds the wasm engine, so the branch build always reflects the
  branch's rules file and pool.
- **Same deals.** If the game client cannot start from a given seed, add a `?seed=` parameter in
  Phase A so the designer can play the same deals on both builds.
- **Blind option.** On request, the orchestrator labels the two builds A and B in
  `docs/iteration/playtests.md` with the key recorded separately.
- **Notes.** The designer writes notes in `docs/iteration/playtests.md` (date, build, seed, felt
  closeness 1–5, did sets feel fair, sigils that confused, anything that felt broken). Each
  iteration reads new notes, turns them into hypotheses, and tests them; a note that conflicts with
  the simulation is reported back, not overruled silently.
- **Milestone builds.** At the end of each phase, record in `docs/iteration/` what changed for a
  player since the baseline, so a playtest knows what to look for.

## Reporting

- `docs/iteration/phase-a.md`, `iter-NN.md` per iteration, and `final-report.md`.
- `docs/iteration/ledger.md`: every change, its dev and holdout estimates, the decision, and the
  tier and utility checks.
- `docs/iteration/holdout-log.md`: every use of holdout and final seeds.
- Dashboards in `reports/iteration/`.
- Every report shows **v1** (the goal) and any **v2** proposals side by side.

## Budget

Rough costs on 18 cores, from the sigil run:

| Measurement | Runs | Time |
| --- | --- | --- |
| Arms and field at tier 1 (one replicate) | about 10,000 | about 2 minutes |
| The same at tier 2 | about 10,000 | about 13 minutes |
| Synergy tournament at tier 0 plus fit | about 240,000 | about 5–10 minutes |
| One-factor sweep, 6 levels × 3 replicates, tier 1 | about 180,000 | about 40 minutes |

Spend compute on precision where decisions are close; never shorten a confirmation to save time.

## When the usage limit nears

Write `HANDOFF.md` in the worktree with the current phase and iteration, the incumbent's rules and
pool commit, open experiments, and the next step.
