# Rogue Spades 2.0: sigil design orchestration plan

This plan designs the full sigil pool for Rogue Spades 2.0. One
**orchestrator** session coordinates subagents through three stages:

1. **Build** the simulation harness and the viewer.
2. **Draft** a first pool, one rarity at a time.
3. **Optimize** the whole pool over several rounds.

Design is a **continuous optimization**, not a series of pass/fail checks.
Each round measures the pool against the metrics in
[GDD §12](game-design.md#12-metrics-what-fun-means) using strong AI players.
It then changes the pieces that most hold the game back, and keeps the changes
that make it better by the evidence and by aesthetic judgment.

The output is about 145 named, iconed sigils, each with its evidence and the
reasoning for keeping it, browsable in a viewer site.

The orchestrator runs every step in order **without pausing for review**.
Each draft pass and optimization round ends with a checkpoint commit that can
be reviewed asynchronously.

You can run this plan without reading the whole
[game design document](game-design.md) (the GDD). Section references point
into it for detail. Where this plan and the GDD disagree, this plan wins
until Phase 0 brings the GDD in line.

## Outcome

- **A pool of about 145 sigils.** Rarity targets are about 60 common, 60
  uncommon, 20 rare, and 5 legendary, following Balatro's joker pool.
- **Evidence and reasoning for every sigil.** Each sigil, including every cut
  or replaced candidate, has a data file. It holds the sigil's metrics, sweep,
  simplicity costs, report links
  ([GDD §16](game-design.md#sigil-records)), and a short history of why it was
  kept, changed, or cut.
- **A viewer site** at `/sigils` in the prototype app. It shows every sigil's
  icon, name, rarity, and rules text, with filters, and a detail panel with
  its evidence. It is modeled on 1.0's viewer (`~/rsp/src/sigils/`) and is
  read-only.
- **An up-to-date GDD.** §9 and §10 point to the data files and the viewer
  instead of illustrative sigils, and every decision and parameter this
  process touched has its evidence recorded.

## Game in brief

This summary is all a subagent needs to start. Point agents at the GDD
sections for detail.

- **The run.** Two teams of two play partnership Spades for 8 rounds. Before
  each round, each team shops for sigils and cards. The team with more points
  after round 8 wins. In single-player, a human and an AI partner face an AI
  team ([§1](game-design.md#1-the-table-the-run-and-victory)).
- **Spades.** Each partner bids nil or 1–13. The team's contract is the sum of
  its non-nil bids. [♠]s are trump and must be broken. There are no bags and
  no blind nil ([§3](game-design.md#3-spades-rules)).
- **Scoring**
  ([§4](game-design.md#4-scoring)):

  ```
  round score = (10 × contract + contract points + nil score)
              × (10 + Σ +contract multipliers) × Π ×contract multipliers
  ```

  - A **set** loses all contract points, and its base becomes −10 × contract.
    Multipliers still apply, so a strong build that overreaches loses more.
  - Overtricks score nothing.
  - **Par** grows from 600 in round 1 to 6,000 in round 8.
- **Sigils** are team-owned, up to 7 per team. Each has a rarity, a price, and
  usually one effect ([§5](game-design.md#5-sigils)).
  - **Payoffs** score in three categories: +contract points, +contract
    multiplier, and ×contract multiplier.
  - **Enablers** change the hand or how tricks can be played, and should be
    worth buying before any payoff.

  | Rarity | Offer odds | Price |
  | --- | --- | --- |
  | Common | 69% | 50 |
  | Uncommon | 25% | 75 |
  | Rare | 5% | 100 |
  | Legendary | 1% | 150 |

- **Cards and engravings.** A bought card is dealt to its owner every round,
  up to 8 per player. From shop 3, card offers carry engravings, including
  synthetic copies of honors
  ([§6](game-design.md#6-cards), [§7](game-design.md#7-engravings)).
  Cards and engravings are **fixed inputs** in this plan, at the GDD's
  starting values; this process doesn't tune them.
- **Archetypes** are internal plans that are never named to players
  ([§9](game-design.md#9-archetypes)):
  - **Majors:** Suits ([♦] first), Spades, Ranks, Bid High, and Nil.
  - **Minors:** Low Cards, Rainbow, Streaks, and Exact.
  - **Generic** sigils fit any build.

## Design rules

### Hard rules

These are never traded away. A candidate that breaks one is dropped, not
revised.

- **No activated abilities.** A sigil may offer a choice only at a fixed
  moment, such as `Opening:`, or through the ordinary choice of a legal card.
- **Rules text is generated from the sigil's data,** following the templates
  in [§5 rules text](game-design.md#rules-text): benefit first, then the
  condition; "your team"; bracketed ranks and suits; `Opening:` first on
  every before-bidding effect. Nobody hand-writes rules text.
- **Evidence leads, and it is written down.** Every change to the pool is
  measured in simulation before it is kept, and every kept sigil records its
  evidence and the reasoning for keeping it.

### Guidelines

These are strong defaults from the GDD, not rules. A designer may depart from
one when the design is better for it, and says why in the candidate's
rationale. The critic flags departures but doesn't drop them; simulation and
the simplicity rubric show whether they earn their place.

- **One trigger or condition, and one effect,** with no riders. Higher rarity
  usually buys power rather than clauses
  ([§5 simplicity rules](game-design.md#simplicity-rules)). Every extra
  clause still pays its full simplicity cost, so the comparison rule favors
  the simpler version unless the extra clause shows a net gain.
- **Effects the GDD's current pool avoids**
  ([§5 categories](game-design.md#categories-and-families)):
  - shop and economy modifiers;
  - information effects, such as revealing cards;
  - randomness after bidding begins;
  - effects that only rename cards for payoff checks;
  - exceptions to the number of tricks needed to make a bid.

### Soft pool targets

These targets replace the GDD's §10 slot tables. They guide each pass's brief
and each round's choice of changes. They are not quotas, and many sigils count
in more than one category.

| Target | Guidance |
| --- | --- |
| Pool size | About 145; each rarity within about ±25% of 60 / 60 / 20 / 5 |
| Mix | About two-thirds mainly score and about a third mainly change hands or play. A hybrid, such as a payoff whose condition reshapes how you play, counts as either |
| Major archetypes | Payoffs in all three scoring categories, across at least two rarities, plus shared enablers that measurably help them |
| Minor archetypes | About five pieces each |
| Bid tension | At least a third of contract-point sigils and a quarter of multipliers scale with or require the contract size ([§4](game-design.md#keeping-bids-tense)) |
| Named suits | [♦] first; [♣], [♥], and [♠] versions where simulation shows they add something ([D12](game-design.md#d12-suit-structure)) |
| ×Multipliers | Mostly uncommon and above; commons are fine when they stay under the power ceiling |

A shortfall against a target is a **coverage hole**. Holes are one of the
things each optimization round can choose to work on.

## Metrics and judgment

Nothing in this plan is a pass/fail bar. The orchestrator decides what to keep
by weighing three kinds of input.

### Sigil metrics

These describe what each sigil contributes. The GDD's starting target bands
are in [§12](game-design.md#sigil-metrics).

| Metric | What it shows |
| --- | --- |
| Choices matter | Forced-pick win-rate lift over a same-rarity flat-points control; for point sigils, the share of the holder's wins it decides |
| Payoffs are doable | How often a payoff fires at base, and when its team commits to it |
| Standalone value | Enablers only: lift with no other sigils at shops 1, 3, and 5; comparison with a same-price flat-points control; then trigger lift for two payoff families ([§12](game-design.md#standalone-enabler-value)) |
| No invalidation | Sigils that touch opponents: how far they cut each archetype's win rate |
| Power ceiling | Lift, and the win rate of its strongest pairs |
| Simplicity | Itemized complexity cost C from the [§12 rubric](game-design.md#simplicity-rubric) |

A sigil outside a band is a **signal**, not a verdict. Large, confident misses
get attention first. A sigil can be well worth keeping while sitting outside a
band, for example a slightly too-strong rare that creates a run's best
moments.

### Pool metrics

The **fun score** ([§12](game-design.md#fun-score)) describes the whole pool:
Close and live, Commitment works, Archetypes viable, Synergy, Skill and
bidding, and Simplicity. Each round reports it by family, before and after the
round's changes, on paired seeds.

### Elegance

The critic gives a ranked aesthetic review of the pool each round:

- pieces that feel clunky, fiddly, or samey;
- pieces that are dull to own or frustrating to play against;
- places where two sigils would read better as one, or one as two;
- archetypes whose sigils don't hang together as a plan.

Metrics can't see these, so they carry real weight in the keep decision.

### Keeping changes

For each proposed change, the orchestrator keeps it when the evidence and
judgment together say the game got better. Usually that means:

- the fun score rose;
- a sigil moved toward its bands;
- a coverage hole closed; or
- a piece became clearly more elegant at little cost.

It writes one line of reasoning per decision into the sigil's history.

- **Confidence intervals** say how sure the evidence is; they are not
  cutoffs. A change whose effect is too uncertain to call can be measured
  again next round, or kept on elegance alone if it costs nothing measurable.
- **The comparison rule.** Between two versions doing the same job, prefer the
  simpler one, unless the paired 90% confidence interval shows a net
  fun-score gain after its simplicity penalty.
- **Playtest targets,** such as the 40% human choice rate for enablers, are
  recorded as `pending`. Agents never stand in for them.

### Re-centering bands

The GDD's bands are placeholders. After the first full-pool measurement, the
analyst compares the pool's distribution with each band. If most of the pool
sits far to one side of a band, the analyst proposes a re-centered band
within the ranges in the
[parameter register](game-design.md#appendix-b-parameter-register). The
orchestrator adopts it and writes a decision record. Bands are re-centered at
most once more, in the final round.

## AI tiers and simulation budget

The AI tiers follow [§13](game-design.md#ai-tiers). No AI component has
sigil-specific code: search plays through the real rules, so a new sigil
changes the AI's choices the moment it exists.

| Tier | AI | Used for |
| --- | --- | --- |
| 0 | Heuristic rollouts and a one-trick lookahead | Number sweeps, hand-level screens, the whole-pool dashboard each round |
| 1 | Information-set MCTS, about 200 iterations | Measuring new and changed sigils, and the whole pool in the final round |
| 2 | Information-set MCTS, about 1,500 iterations | Final-round spot checks and tier calibration |

The measurement budget is **about 6 hours** of wall-clock time per draft pass
or optimization round on this machine (18 cores):

- **Tier 0 measures everything** cheaply: hand-level trigger rates, number
  sweeps to each rarity's budget ([§10 budgets](game-design.md#budgets)), and
  forced-pick lift.
- **Tier 1 measures what is being decided:** new candidates that tier 0 marks
  as promising or uncertain, and every change under consideration. It uses
  sequential testing: a trial stops once its 90% confidence interval is
  narrow enough to rank the sigil or make the decision at hand, capped at
  2,000 paired runs.
- **Forced picks** run at shop 1, except for enablers, which also run at
  shops 3 and 5.
- **When work exceeds the budget,** the analyst measures what matters most
  for the round's decisions first, and reports the rest as unmeasured.

## Roles

Only the orchestrator decides. Every subagent gets this plan's
[Game in brief](#game-in-brief) and [Design rules](#design-rules), its own
role section, and the files listed for it.

| Role | Writes | Job |
| --- | --- | --- |
| Orchestrator | Briefs, keep decisions, GDD records, commits | Runs the plan and decides what stays |
| Implementer | Code and candidate data files | Builds the harness, then adds the grammar features and rule hooks new designs need |
| Reviewer | Nothing | Checks each Phase 0 build against its done criteria |
| Designer | Nothing; returns text | Writes new candidates, reshapes, and replacements |
| Critic | Nothing; returns text | Reviews candidates before simulation, and gives the elegance review each round |
| Analyst | Reports, and metrics in data files | Runs the harness and returns the dashboard |
| Namer | Nothing; returns text | Names and icons the final pool |
| Auditor | Nothing; returns text | Audits the final pool |

- **Continue within a step, respawn between steps.** Within a draft pass or
  optimization round, follow-up work reuses an agent with SendMessage.
  Each new pass or round spawns fresh agents, so contexts stay small; the
  registry and round notes carry the state.
- **Serialize writers.** The implementer and the analyst never run at the same
  time, because both write to the repository. Designers, the critic, and the
  namer only return text.

## Artifacts

| Path | Owner | Contents |
| --- | --- | --- |
| `docs/sigil-design-plan.md` | Orchestrator | This plan |
| `docs/game-design.md` | Orchestrator | The GDD, updated at each checkpoint |
| `data/sigils/<id>.json` | Implementer creates; analyst adds metrics; orchestrator sets status, history, name, and icon | One file per candidate, including cut and replaced ones |
| `docs/sigils/registry.md` | Generated by `npm run registry` | Compact table of every candidate: id, status, rarity, name, icon family, generated text, simplicity cost C, key metrics, and effect signature. Agents read this, not the data files |
| `docs/sigils/icons.txt` | Copied from `~/rsp/docs/sigils/icons.txt` | 431 curated filled Boxicons, one per distinct image |
| `docs/sigils/rounds/<step>.md` | Orchestrator | Each pass's or round's brief, then its outcome: changes tried, kept, and reverted with reasons, fun score by family, coverage holes, and band changes |
| `reports/<experiment>.md` and `.json` | Analyst, via the harness | Experiment reports ([§16](game-design.md#experiment-reports)) |

### Sigil data file

The harness owns the format. It must hold at least:

- **Identity:** `id`, `rarity`, `price`, `role` (payoff, enabler, or
  hybrid), internal `archetypes`, `source` (`enumerated`, `designed`, or
  `gdd-seed`), and the step that created it.
- **The effect** in the declarative grammar ([§13](game-design.md#declarative-sigils)),
  plus the rules text generated from it.
- **Design notes:** `decision`, the choice it changes for its owner;
  `opponent`, how it looks from the other side of the table; and
  `rationale`, including any guideline departure and why.
- **Status:** `candidate`, `kept`, `cut`, or `replaced` (with the id that
  replaced it).
- **History:** one entry per decision, with the step and a line of
  reasoning, such as "Round 2: amount 15 → 20; lift was far below band".
- **Name and icon,** after naming: `name`, `icon`, `iconFamily`, and
  `iconWord`.
- **Metrics:** simplicity costs (itemized, with C and S), sweep results,
  sigil metrics with confidence intervals and run counts, synergy pairs, and
  report links.

The **effect signature** normalizes the grammar data as trigger | filter |
scaling | effect type. Two candidates are **near-duplicates** when their
signatures differ only in amount, named suit, or named rank. Near-duplicates
are an elegance concern for the critic, not an automatic cut.

## Phase 0: harness and viewer

Three implementer builds run in order. Each build gets one implementer and
then one reviewer. Each is committed and delivered through Tollgate before the
next build starts.

These are engineering checks on the harness, not design decisions, so they
have concrete done criteria.

- **No test suites** (per `AGENTS.md`). Each build proves itself by running
  its done criteria as a script.
- **Repair limit:** a build that misses its criteria gets two repair rounds.
  After that, the orchestrator proceeds, records the shortfall in
  `docs/sigils/rounds/phase-0.md`, and repeats it in every later round's
  notes.

### 0a: kernel, grammar, and viewer scaffold

Build:

- A Vite + React + TypeScript app with `format:check`, `lint`, and `build`
  scripts, so `scripts/ci` runs. Reuse 1.0's tooling and card rendering where
  it fits ([§13 carry-over](game-design.md#what-carries-over-from-10)).
- The rules kernel from [§13](game-design.md#the-rules-kernel):
  - compact state with apply-and-undo moves;
  - hook tables for effective suit and rank, legal plays, the next leader,
    and scoring events;
  - a scoring ledger that can recompute any score without any set of sigils;
  - seeded random streams;
  - no globals, and runnable under Node's type stripping.
- The declarative sigil grammar and interpreter
  ([§13](game-design.md#declarative-sigils)), the rules-text generator, the
  text lint, and the simplicity scorer
  ([§12 rubric](game-design.md#simplicity-rubric)).
- **GDD seeds** as data files, with `source: gdd-seed`:
  - every illustrative sigil in §9;
  - the 7 shared enablers;
  - the 2.0 seeds in [Appendix E](game-design.md#appendix-e-seeds-from-rogue-spades-10).
- `npm run registry`.
- The `/sigils` viewer route. See [Viewer](#viewer).

**Done when:**

- Every §9 seed's generated text matches its GDD text exactly, or the
  difference is listed as a GDD typo.
- Every row of the §12 rubric table scores the C shown there.
- A plain round of Spades plays and scores as in the
  [§4 worked examples](game-design.md#worked-examples).
- The viewer lists every seed.
- `scripts/ci` passes.

### 0b: AI tiers

Build:

- Port 1.0's AI (`~/rsp/src/ai/`) onto the kernel: deal sampling fitted to
  bids, information-set MCTS, tie-breaking among near-equal moves, and
  expected-score bidding and nil decisions.
- Tiers 0–2 at the [§13](game-design.md#ai-tiers) budgets.
- A plain-Spades bench, reusing 1.0's metric definitions
  (`~/rsp/docs/prototype-status.md`).

**Done when** (plain Spades, tier 1):

- The team set rate is at most 18%.
- Nil success is at least 65%, and nil suicides are under 1%.
- Tier 2 beats tier 0 in at least 65% of paired runs.
- Throughput: tier 0 takes about 0.3 s per run, and a 2,000-run tier-1
  experiment takes at most about 15 minutes on 16 workers.

### 0c: shop, runner, and metrics

Build:

- The card shop, the 8-card cap, income, interest, and rerolls.
- Engravings and synthetic cards at their GDD starting values.
- The sigil shop, with its rarity odds and prices.
- The rescoring shop AI with flexible and committed policies
  ([§13 shop AI](game-design.md#shop-ai)), including fresh rollouts for
  enablers.
- The paired-seed experiment runner, with worker processes and sequential
  testing. It runs every trial type in
  [§13 experiments](game-design.md#experiments): hand-level, forced-pick,
  standalone enabler, commitment, counter scan, and pair trials.
- **Pool comparison:** the runner can measure the fun score of two pool
  versions on the same paired seeds, which every round's keep decisions use.
- Sigil-metric and fun-score calculators, the report writer, and the metrics
  writer for the data files.
- **The dashboard:** one command that writes a round's table of every sigil's
  metrics against its bands, plus the fun score by family.
- The common-payoff enumerator: every grammar combination at common, about
  270 candidates ([D21](game-design.md#d21-candidate-generation)).

**Done when:**

- Flexible against flexible wins 50% within its confidence interval.
- The seed pool runs a full 8-round experiment end to end and writes a
  report, metrics, and a dashboard.
- The rescoring shopper beats a random shopper on paired seeds
  ([D17](game-design.md#d17-shop-ai)).

### GDD alignment

At the end of Phase 0, the orchestrator writes the pool-shape decision into
the GDD:

- **D28, "Pool shape":** soft targets of about 145, replacing §10's slot
  tables and the pool sizes in §5's rarity table.
- **P21:** the new targets and their range.
- **D21:** the enumerated-versus-designed comparison this plan runs at common.

## Draft passes

Three passes build a first pool, one rarity at a time: commons, then
uncommons, then rares and legendaries. Each pass keeps somewhat more than its
rarity target, about 110–120%, so the optimization rounds have room to prune.

Each pass runs these steps.

1. **Brief.** The orchestrator writes `docs/sigils/rounds/draft-<rarity>.md`.
   It gives the rarity's targets, the coverage holes so far, and the
   archetypes that need the most help.
2. **Design.** Three designers work in parallel, one per archetype cluster.
   Each writes about 1.5 times its share of the pass's target.
   - **Cluster A:** Suits, Spades, Ranks, and Rainbow.
   - **Cluster B:** Bid High, Streaks, Exact, and Generic.
   - **Cluster C:** Nil, Low Cards, and enablers and rule benders.
3. **Critique.** The critic reviews every candidate and returns keep, fix (with
   the fix), or drop, plus its concerns.
4. **Implement.** The implementer adds any grammar features and rule hooks the
   candidates need, then writes their data files. If a candidate's mechanic
   can't be built cleanly in this pass, it is deferred to an optimization
   round. The implementer delivers through Tollgate.
5. **Measure.** The analyst screens and sweeps every candidate at tier 0,
   measures the promising and uncertain ones at tier 1, and returns the
   dashboard.
6. **Choose.** The orchestrator keeps the most promising candidates toward the
   pass's targets, weighing metrics, elegance, coverage, and the comparison
   rule. The rest are cut, with one line of reasoning each.
7. **Checkpoint.** The orchestrator writes the pass outcome, regenerates the
   registry, commits, and delivers through Tollgate.

### Commons: two sources

Commons come from two sources run side by side, to settle
[D21](game-design.md#d21-candidate-generation):

- **Enumerated.** The analyst runs the enumerator's about 270 candidates
  through the tier-0 screen and sweep before step 3. Only the most promising
  go on to the critic and tier 1.
- **Designed.** The three designers work **blind to the enumeration**, so the
  comparison is fair. GDD seeds at common enter with the designed source.

Where the two sources produce the same signature, they merge into one
candidate that credits both. The pass notes report how each source's
candidates measured and how many each supplied to the draft pool. D21's record
is updated with those numbers.

### Rares and legendaries

Power matters most here. The analyst runs pair trials between each rare or
legendary and the drafted sigils its designer named as likely partners.
Legendaries are about 5 build-arounds; they need not fit any one archetype.

## Optimization rounds

Once the draft pool exists, up to **four rounds** improve it as a whole. Any
round can change any sigil at any rarity.

Each round runs these steps.

1. **Measure.** The analyst runs the dashboard on the whole pool at tier 0,
   plus pair trials for the synergy family: every same-archetype pair, and up
   to 100 cross-archetype pairs that designers nominated. Round 1 also
   proposes re-centered bands (see [Re-centering bands](#re-centering-bands)).
2. **Review.** The critic gives its ranked elegance review of the whole pool.
3. **Choose targets.** The orchestrator writes the round's brief in
   `docs/sigils/rounds/round-<n>.md`. It lists about 15–25 targets, drawn
   from:
   - sigils with large, confident misses against their bands;
   - the critic's top elegance concerns;
   - coverage holes;
   - the fun-score families furthest from their bands;
   - for the pool as a whole, trimming toward about 145.

   Each target names the change wanted: retune, reshape, replace, merge, cut,
   or add.
4. **Redesign.** One designer, given the targets, the dashboard rows, and the
   critic's notes, proposes a change for each. Retunes come straight from the
   analyst's sweep and skip this step.
5. **Critique and implement.** The critic reviews the proposals, and the
   implementer builds what they need.
6. **Measure the changes.** The analyst measures each changed or new sigil at
   tier 1. It then compares the fun score of the pool before and after the
   round's changes on paired seeds.
7. **Keep or revert.** The orchestrator keeps or reverts each change, with
   one line of reasoning in the sigil's history (see
   [Keeping changes](#keeping-changes)).
8. **Checkpoint.** The orchestrator writes the round's outcome, updates the
   GDD's touched records, regenerates the registry, commits, and delivers
   through Tollgate.

### Stopping

Optimization stops after round 4, or earlier when a round **plateaus**: its
kept changes don't measurably move the fun score, and the critic raises no
major elegance concerns.

### Final round

The last round, whether round 4 or an earlier plateau, adds these steps
before its checkpoint.

1. **Full measurement.** The analyst measures the whole pool at tier 1 once,
   so every sigil's final numbers are current.
2. **Tier calibration.** The analyst spot-checks the pool and every sigil near
   a keep decision at tier 2, to confirm tiers 0 and 1 rank sigils the same
   way. Where tier 2 disagrees, the orchestrator revisits that decision with
   the tier-2 numbers.
3. **Naming.** The namer names and icons the whole pool at once, so names stay
   consistent and unique.
4. **Audit.** The auditor checks:
   - pool size and rarity split against the soft targets;
   - archetype coverage;
   - bid tension;
   - rules-text lint;
   - hard-rule compliance, and every guideline departure with its
     justification;
   - a final duplicate sweep;
   - name and icon uniqueness;
   - that every kept sigil has evidence and reasoning in its history.

   The orchestrator fixes findings. Any sigil whose effect changes goes back
   through the analyst.
5. **Final report.** The orchestrator writes `docs/sigils/final-report.md`. It
   covers:
   - pool counts;
   - the fun score by family, and how it moved across rounds;
   - remaining coverage holes and open risks;
   - pending playtest targets;
   - any Phase 0 shortfall.

   It updates the GDD so §9 and §10 point at the data and the viewer.

## Role instructions

### Designer

Each designer receives:

- the GDD's [§5](game-design.md#5-sigils), its archetype sections in
  [§9](game-design.md#9-archetypes), and
  [§9's shared enablers](game-design.md#shared-enabler-candidates);
- the grammar reference from the harness;
- `docs/sigils/registry.md`;
- the pass or round brief.

For each candidate, the designer:

1. Writes it in the grammar where possible. If the grammar can't express it,
   the designer writes the effect in plain terms and lists the hook it needs.
2. Checks the registry and its own batch for duplicates and near-duplicates.
3. Explains the decision it changes. A sigil that changes no decision is a
   **stat stick**, which is fine at common and should be rare above it.
4. Notes which existing sigils it combines with. Pair trials use these notes.

In optimization rounds, the designer works from a target's dashboard row and
the critic's notes. It may propose more than one option per target; the
analyst measures each, and the orchestrator keeps the best.

Enablers should answer this question without naming a future payoff: "Would I
buy this before knowing my build?"

Designers return each candidate in this format. Rules text is generated later,
so the designer's text is only a draft.

```
Cluster:    A
Rarity:     Uncommon
Role:       Payoff (+mult)
Archetypes: Suits; partner Ranks
Effect:     { "type": "mult", "amount": 15, "on": { "event": "win", "card": { "suit": "D" }, "count": 3 } }
Draft text: +15 contract multiplier when your team wins three tricks with [♦]
Hooks:      none
Decision:   Lead [♦] honors early to bank three wins before opponents void.
Opponent:   Revealed on the third [♦] win; opponents can void [♦] and trump.
Partners:   [♦] Hold payoff; any +contract-points sigil
Rationale:  The milestone payoff for a [♦] build; pays only once a plan works.
```

### Critic

The critic has two jobs.

**Candidate review,** in draft passes and for each round's proposals. It
receives the candidates, the registry, and this plan's
[Design rules](#design-rules), and checks:

- **The hard rules,** especially no activated abilities.
- **Guideline departures:** whether the candidate's rationale justifies each
  one, and whether a simpler version would do the same job.
- **Duplicates and near-duplicates** against the registry and the batch.
- **Fun to play:** does it change a bid, a play, or a purchase, and does it
  create a memorable moment?
- **Fun to play against:** can opponents see it and respond through bidding or
  play?
- **AI-evaluability:** can search through the rules value its choices without
  sigil-specific code?

It returns keep, fix (with the exact fix), or drop for each candidate. Drop is
only for hard-rule breaks and exact duplicates; everything else is a concern
the orchestrator weighs. It doesn't judge power; simulation does.

**Elegance review,** at the start of each optimization round. It receives the
registry and returns a ranked list of concerns, as described in
[Elegance](#elegance), each with a suggested direction.

### Implementer

In Phase 0, the implementer builds the harness. In each pass or round, it
receives the candidates and:

- adds the grammar features and rule hooks they need, as general mechanisms,
  never per-sigil code;
- writes each candidate's data file;
- confirms the generated text passes lint;
- runs `scripts/ci`, commits with Conventional Commits, and delivers through
  Tollgate.

It returns, for each candidate: created, deferred (with what blocks it), or
changed (with what changed in the effect and why).

### Analyst

The analyst receives the sigil ids to measure, the bands, and the budget. It
runs the harness as described in each step and returns:

- **the dashboard:** every measured sigil's metrics against their bands, with
  confidence intervals, and the fun score by family;
- **diagnoses** for large misses, such as "fires in 4% of rounds; the
  condition is too narrow" or "lift is flat at every amount; the effect
  doesn't change outcomes";
- **sweep results:** the amount that best centers each sigil in its bands,
  already written to the data file as a suggestion;
- **before-and-after comparisons** for each round's changes;
- anything it couldn't measure within the budget.

It writes reports and metrics but never decides what to keep.

### Namer

The namer runs once, in the final round. It receives the kept pool,
`docs/sigils/icons.txt`, and the registry. It follows 1.0's naming rules
(`~/rsp/docs/rules-text.md`, "Names and icons"):

- **Two or three words,** in title case. One word, the **icon word**, names
  the icon's subject or a close synonym.
- **A little poetic,** and echoing the effect where possible.
- **No rules vocabulary,** such as "nil", "contract", or "trick".
- **Unique pool-wide:** every name, icon family, and icon word appears on only
  one sigil. An icon and its variants (`-alt`, `-circle`, `-square`, `-plus`,
  `-minus`) form one family.
- **No archetype motifs.** Archetypes are never named to players, so icons
  are chosen for the effect, not for a theme per archetype.

For each sigil, the namer returns three name-and-icon pairs in preference
order. The orchestrator claims the first pair that is still free.

### Auditor

The auditor receives the registry, this plan, and the final dashboard. It
returns findings, ranked by severity, against the audit list in the
[final round](#final-round).

## Viewer

The viewer is the `/sigils` route of the prototype app, built in Phase 0a and
filled as the pool grows. It is modeled on 1.0's viewer (`~/rsp/src/sigils/`)
and is read-only. The data files are its only source.

- **Grid.** Each tile shows the icon (a Boxicons filled glyph from
  `@boxicons/core`, colored by rarity), the name, the rarity, and the
  generated rules text. Unnamed candidates show their id and a placeholder
  glyph until the final round.
- **Filters:**
  - rarity;
  - category: points, +mult, ×mult, enabler, or hybrid;
  - archetype (internal, so the viewer is a design tool, not a player view);
  - status, defaulting to kept;
  - source;
  - text search.
- **Detail panel:**
  - the full data and design notes;
  - the simplicity breakdown;
  - each sigil metric shown against its band, with its confidence interval;
  - the sweep and the chosen amount;
  - synergy pairs;
  - the history of decisions and their reasoning;
  - links to the reports.
- **Running it:**

  ```bash
  npm run dev
  ```

  Then open `http://localhost:5173/sigils`.

## Checkpoints

The orchestrator commits and delivers through Tollgate at the end of each
Phase 0 build, each draft pass, and each optimization round. (The
implementer's code commits are delivered as it goes.)

- **Messages** follow Conventional Commits and name the step, such as
  `feat(sigils): draft common pool` or `feat(sigils): apply optimization
  round 2`.
- **Bodies** summarize the step:
  - changes tried, kept, and reverted;
  - the fun score by family, before and after;
  - coverage holes;
  - any band changes.
- **Delivery** follows `AGENTS.md`. On master, run `tg push-master --wait`. In
  a Tollgate worktree, submit with `tg candidate HEAD`, approve, and wait for
  promotion.

## Budget summary

| Step | Subagents | Measurement time |
| --- | --- | --- |
| Phase 0 | 3 implementers, 3 reviewers | Done criteria only |
| Draft: commons | 3 designers, critic, implementer, analyst | About 6 h, plus the enumeration screen |
| Draft: uncommons | Same 6 | About 6 h |
| Draft: rares and legendaries | Same 6 | About 6 h |
| Optimization rounds 1–4 | Analyst, critic, designer, implementer per round | About 6 h per round |
| Final round extras | Namer, auditor | About 8 h, including tier 2 |
| **Total** | **About 42** | **About 2.5–3 days** |
