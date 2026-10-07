# Rogue Spades 2.0: sigil design orchestration plan

This plan designs the full sigil pool for Rogue Spades 2.0. One
**orchestrator** session coordinates subagents through a harness build and
four design waves. Strong AI players simulate every candidate, and only
sigils that pass the simulated gates ship. The output is about 145 named,
iconed sigils, each with its evidence, browsable in a viewer site.

The orchestrator runs every phase and wave in order **without pausing for
review**. Each wave ends with a checkpoint commit that can be reviewed
asynchronously.

You can run this plan without reading the whole
[game design document](game-design.md) (the GDD). Section references point
into it for detail. Where this plan and the GDD disagree, this plan wins
until Phase 0 brings the GDD in line.

## Outcome

- **A pool of about 145 sigils.** Rarity targets are about 60 common, 60
  uncommon, 20 rare, and 5 legendary, following Balatro's joker pool.
- **Evidence for every sigil.** Each sigil, including every cut candidate,
  has a data file with its gate results, sweep, simplicity costs, and links
  to the experiment reports behind them
  ([GDD §16](game-design.md#sigil-records)).
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
  one effect ([§5](game-design.md#5-sigils)).
  - **Payoffs** score in three categories: +contract points, +contract
    multiplier, and ×contract multiplier.
  - **Enablers** change the hand or how tricks can be played, and must be worth
    buying before any payoff.

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

- **One trigger or condition, and one effect,** at every rarity, with no
  riders. Higher rarity buys power, not clauses
  ([§5 simplicity rules](game-design.md#simplicity-rules)).
- **No activated abilities.** A sigil may offer a choice only at a fixed
  moment, such as `Opening:`, or through the ordinary choice of a legal card.
- **Excluded effects**
  ([§5 categories](game-design.md#categories-and-families)):
  - shop and economy modifiers;
  - information effects, such as revealing cards;
  - randomness after bidding begins;
  - effects that only rename cards for payoff checks;
  - exceptions to the number of tricks needed to make a bid.
- **Rules text is generated from the sigil's data,** following the templates
  in [§5 rules text](game-design.md#rules-text): benefit first, then the
  condition; "your team"; bracketed ranks and suits; `Opening:` first on
  every before-bidding effect. Nobody hand-writes rules text.
- **Simulation decides.** A sigil ships only if it passes the simulated gates
  below. Designers choose among passing candidates; they never overrule a
  failed gate.

### Soft pool targets

These targets replace the GDD's §10 slot tables. They guide each wave's brief
and the audit. They are not quotas, and many sigils count in more than one
category.

| Target | Guidance |
| --- | --- |
| Pool size | About 145; each rarity within about ±25% of 60 / 60 / 20 / 5 |
| Mix | About two-thirds mainly score and about a third mainly change hands or play. A hybrid, such as a payoff whose condition reshapes how you play, counts as either |
| Major archetypes | Payoffs in all three scoring categories, across at least two rarities, plus shared enablers that measurably help them |
| Minor archetypes | About five pieces each |
| Bid tension | At least a third of contract-point sigils and a quarter of multipliers scale with or require the contract size ([§4](game-design.md#keeping-bids-tense)) |
| Named suits | [♦] first; [♣], [♥], and [♠] versions only where simulation shows they earn their place ([D12](game-design.md#d12-suit-structure)) |
| ×Multipliers | Mostly uncommon and above; commons allowed when they pass the power ceiling |

The audit flags **coverage gaps**, not empty slots. Each gap gets one fresh
design attempt in the next sim loop. If that fails, the gap is reported and
left open.

## Acceptance gates

A sigil is **accepted** when it passes the simulated parts of these gates at
the thresholds in [§12](game-design.md#12-metrics-what-fun-means). It must
also not lower the fun score of the pool it joins.

| Gate | Short version |
| --- | --- |
| 1. Choices matter | Forced-pick win-rate lift over a same-rarity flat-points control is above zero; point sigils are decisive in at least 5% of the holder's wins |
| 2. Payoffs are doable | The payoff fires in at least 10% of rounds at base, and in at least 60% when its team is committed to it |
| Standalone enabler value | Enablers only: at least +3 points of win-rate lift with no other sigils, at shops 1, 3, and 5; no worse than −2 points against the flat-points control; then +15 points of trigger lift in two payoff families ([§12](game-design.md#standalone-enabler-value)) |
| 3. No invalidation | Sigils that touch opponents cut no archetype's win rate by more than 15 points |
| Power ceiling | Lift of at most 12 points; no pair wins more than 65% |

- **Playtest bars,** such as the 40% human choice rate for enablers, are
  recorded as `pending` in each sigil record. Agents never stand in for them.
- **The fun score** ([§12](game-design.md#fun-score)) is computed at each sim
  loop and recorded in the wave summary. It includes the simplicity family
  from the [§12 rubric](game-design.md#simplicity-rubric), which the harness
  computes from sigil data.
- **The comparison rule.** Between two passing candidates for the same job,
  the simpler one wins unless the paired 90% confidence interval shows a net
  fun-score gain after its simplicity penalty.

### Threshold recalibration

The GDD's thresholds are placeholders. If a wave's pass rate falls outside
**25–90%**, the analyst proposes new thresholds:

- within the ranges in the
  [parameter register](game-design.md#appendix-b-parameter-register);
- justified by the distribution of baseline results.

The orchestrator adopts them **once per wave** and writes a decision record.
The balance wave re-applies the final thresholds to every accepted sigil.

## AI tiers and simulation budget

The AI tiers follow [§13](game-design.md#ai-tiers). No AI component has
sigil-specific code: search plays through the real rules, so a new sigil
changes the AI's choices the moment it exists.

| Tier | AI | Used for |
| --- | --- | --- |
| 0 | Heuristic rollouts and a one-trick lookahead | Number sweeps, hand-level screens, cheap re-checks |
| 1 | Information-set MCTS, about 200 iterations | Gate decisions |
| 2 | Information-set MCTS, about 1,500 iterations | Balance-wave spot checks and tier calibration |

Each sim loop has a budget of **about 6 hours** of wall-clock time on this
machine (18 cores):

- **Tier 0 screens every candidate** with full trials: hand-level trigger
  rates and a number sweep to its rarity's budget
  ([§10 budgets](game-design.md#budgets)).
- **Tier 1 gates only the survivors,** using sequential testing. A trial
  stops as soon as its 90% confidence interval clears or misses the bar, and
  is capped at 2,000 paired runs.
- **Forced picks run at shop 1 only,** except for enablers. The standalone
  gate needs shops 1, 3, and 5.
- **Over budget:** if a loop's work would exceed about 6 hours, the analyst
  gates the candidates that matter most for coverage first. It leaves the
  rest `inconclusive`, and they carry into the next loop.

## Roles

Only the orchestrator decides. Every subagent gets this plan's
[Game in brief](#game-in-brief) and [Design rules](#design-rules), its own
role section, and the files listed for it.

| Role | Count | Writes | Job |
| --- | --- | --- | --- |
| Orchestrator | 1 session | Wave briefs, curation, GDD records, commits | Runs the plan, accepts sigils, keeps the pool on target |
| Implementer | 1 per phase or wave | Code, candidate data files | Builds the harness; adds the grammar features and rule hooks each wave needs |
| Reviewer | 1 per Phase 0 build | Nothing | Checks each Phase 0 build against its exit gate |
| Designer | 3 per rarity wave | Nothing; returns text | Writes candidates for one archetype cluster |
| Critic | 1 per rarity wave | Nothing; returns text | Reviews candidates before they reach simulation |
| Analyst | 1 per rarity wave | Reports and evidence fields in data files | Runs the harness and returns verdicts |
| Namer | 1 per rarity wave | Nothing; returns text | Names and icons the accepted sigils |
| Auditor | 1, balance wave | Nothing; returns text | Audits the whole pool |

- **Continue, don't respawn.** Within a wave, the second sim loop reuses each
  agent with SendMessage, so designers revise with their own context intact.
- **Serialize writers.** The implementer and the analyst never run at the same
  time, because both write to the repository. Designers, the critic, and the
  namer only return text.
- **Budget.** About 32 subagents in all: 6 in Phase 0, 7 in each rarity wave,
  and 5 in the balance wave.

## Artifacts

| Path | Owner | Contents |
| --- | --- | --- |
| `docs/sigil-design-plan.md` | Orchestrator | This plan |
| `docs/game-design.md` | Orchestrator | The GDD, updated at each checkpoint |
| `data/sigils/<id>.json` | Implementer creates; analyst adds evidence; orchestrator sets status, name, and icon | One file per candidate, including cut ones |
| `docs/sigils/registry.md` | Generated by `npm run registry` | Compact table of every candidate: id, status, rarity, name, icon family, generated text, simplicity cost C, and effect signature. Agents read this, not the data files |
| `docs/sigils/icons.txt` | Copied from `~/rsp/docs/sigils/icons.txt` | 431 curated filled Boxicons, one per distinct image |
| `docs/sigils/waves/wave-<n>.md` | Orchestrator | The wave's brief, then its outcome: accepted, cut, gaps, fun score, and recalibrations |
| `reports/<experiment>.md` and `.json` | Analyst, via the harness | Experiment reports ([§16](game-design.md#experiment-reports)) |

### Sigil data file

The harness owns the format. It must hold at least:

- **Identity:** `id`, `rarity`, `price`, `role` (payoff, enabler, or
  hybrid), internal `archetypes`, `source` (`enumerated`, `designed`, or
  `gdd-seed`), and `wave`.
- **The effect** in the declarative grammar ([§13](game-design.md#declarative-sigils)),
  plus the rules text generated from it.
- **Design notes:** `decision`, the choice it changes for its owner;
  `opponent`, how it looks from the other side of the table; and
  `rationale`.
- **Status:** `candidate`, `accepted`, `cut`, or `inconclusive`, with the wave
  and loop that decided it and a one-line reason.
- **Name and icon,** once accepted: `name`, `icon`, `iconFamily`, and
  `iconWord`.
- **Evidence:** simplicity costs (itemized, with C and S), sweep results,
  gate results with confidence intervals and run counts, synergy pairs, and
  report links.

The **effect signature** normalizes the grammar data as trigger | filter |
scaling | effect type. Two candidates are **near-duplicates** when their
signatures differ only in amount, named suit, or named rank. The later or
weaker one is revised or cut.

## Phase 0: harness and viewer

Three implementer builds run in order. Each build gets one implementer and
then one reviewer. Each is committed and delivered through Tollgate before the
next build starts.

- **No test suites** (per `AGENTS.md`). Each build proves itself with its exit
  gate, run as a script.
- **Repair limit:** a build that misses its exit gate gets two repair rounds.
  After that, the orchestrator proceeds. It records the shortfall in
  `docs/sigils/waves/wave-0.md` and repeats it in every later wave summary.

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

**Exit gate:**

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

**Exit gate** (plain Spades, tier 1):

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
- Gate and fun-score calculators, the report writer, and the evidence writer
  for the data files.
- The common-payoff enumerator: every grammar combination at common, about
  270 candidates ([D21](game-design.md#d21-candidate-generation)).

**Exit gate:**

- Flexible against flexible wins 50% within its confidence interval.
- The seed pool runs a full 8-round experiment end to end and writes a
  report and evidence.
- The rescoring shopper beats a random shopper on paired seeds
  ([D17](game-design.md#d17-shop-ai)).

### GDD alignment

At the end of Phase 0, the orchestrator writes the pool-shape decision into
the GDD:

- **D28, "Pool shape":** soft targets of about 145, replacing §10's slot
  tables and the pool sizes in §5's rarity table.
- **P21:** the new targets and their range.
- **D21:** the enumerated-versus-designed comparison this plan runs at common.

## Rarity waves

Wave 1 designs commons, wave 2 uncommons, and wave 3 rares and legendaries.
Each later wave builds on an accepted pool whose evidence is current.

Every wave runs the same steps.

1. **Brief.** The orchestrator writes `docs/sigils/waves/wave-<n>.md`. It
   gives the wave's rarity targets, the coverage gaps from earlier waves, and
   priorities: more value for a lagging archetype, tighter conditions for a
   leading one.
2. **Design.** Three designers work in parallel, one per archetype cluster.
   Each writes about 1.5 times its share of the wave's target.
   - **Cluster A:** Suits, Spades, Ranks, and Rainbow.
   - **Cluster B:** Bid High, Streaks, Exact, and Generic.
   - **Cluster C:** Nil, Low Cards, and enablers and rule benders.
3. **Critique.** One critic reviews every candidate and returns pass, fix (with
   the fix), or drop.
4. **Implement.** The implementer adds any grammar features and rule hooks the
   passing candidates need, then writes their data files. If a candidate's
   mechanic can't be built cleanly in this pass, it is deferred to loop 2 or
   cut. The implementer delivers through Tollgate.
5. **Sim loop 1.** The analyst screens, sweeps, and gates every candidate, and
   returns verdicts.
6. **Revise.** Designers, continued, revise candidates with `revise`
   verdicts. They also make one fresh attempt at each coverage gap. The
   critic and the implementer then process the changes.
7. **Sim loop 2.** The analyst gates the revised and retuned candidates. It
   also re-checks the accepted pool from earlier waves (see
   [Keeping evidence current](#keeping-evidence-current)) and computes the
   fun score.
8. **Curate.** The orchestrator accepts passing candidates toward the soft
   targets, applying the comparison rule. Surplus passing candidates are cut
   with the reason `surplus`, and their evidence is kept.
9. **Name.** The namer names and icons every newly accepted sigil.
10. **Checkpoint.** The orchestrator updates the GDD records, writes the wave
    outcome, regenerates the registry, commits, and delivers through Tollgate.

There are only two sim loops per wave. A candidate still failing after loop 2
is cut, and one still `inconclusive` carries into the next wave's loop 1.

### Wave 1: commons, from two sources

Commons come from two sources run side by side, to settle
[D21](game-design.md#d21-candidate-generation):

- **Enumerated.** The analyst runs the enumerator's about 270 candidates
  through the tier-0 screen and sweep before loop 1. Only survivors go on to
  the critic and to the tier-1 gates.
- **Designed.** The three designers work **blind to the enumeration**, so the
  comparison is fair. GDD seeds at common enter with the designed source.

Where the two sources produce the same signature, they merge into one
candidate that credits both. The wave summary reports each source's pass rate
and how many accepted commons it supplied. D21's record is updated with those
numbers.

### Wave 3: rares and legendaries

The power ceiling matters most here. The analyst runs pair trials between each
rare or legendary and the accepted sigils its designer named as likely
partners. Legendaries are about 5 build-arounds; they need not fit any one
archetype.

## Balance wave

Wave 4 checks the whole pool.

1. **Re-gate.** The analyst re-gates every accepted sigil at tier 1 with the
   final thresholds.
2. **Synergy.** The analyst runs pair trials for the synergy family: every
   same-archetype pair at tier 0, plus up to 100 cross-archetype pairs that
   designers nominated.
3. **Calibration.** The analyst spot-checks the pool and every borderline
   sigil at tier 2, to confirm tiers 0 and 1 rank sigils the same way. Any
   verdict that flips at tier 2 uses the tier-2 result.
4. **Retune.** Failures that a number fixes are retuned by sweep. Structural
   failures get one revision by the original cluster's designer, newly
   spawned and given the sigil's record. Anything still failing is cut.
5. **Audit.** The auditor checks:
   - pool size and rarity split against the soft targets;
   - archetype coverage;
   - bid tension;
   - rules-text lint;
   - hard-rule compliance;
   - a final duplicate sweep;
   - name and icon uniqueness.

   The orchestrator fixes findings. Any sigil whose effect changes goes back
   through the analyst.
6. **Final checkpoint.** The orchestrator writes `docs/sigils/final-report.md`
   (pool counts, fun score with its families, coverage gaps, open risks,
   pending playtest bars, and any Phase 0 shortfall). It updates the GDD so §9
   and §10 point at the data and the viewer, then commits and delivers.

## Keeping evidence current

A new rarity changes the pool that partners and opponents draw from, so
earlier evidence goes stale ([§16](game-design.md#keeping-evidence-current)).

- **Every wave, cheaply:** in sim loop 2, the analyst re-runs a tier-0
  forced-pick screen on every accepted sigil. Only sigils whose verdict flips
  get a tier-1 re-gate. A sigil that then fails returns to `candidate` and
  joins the next wave's revisions.
- **At the end, fully:** the balance wave re-gates everything.

## Role instructions

### Designer

Each designer receives:

- the GDD's [§5](game-design.md#5-sigils), its cluster's archetype sections in
  [§9](game-design.md#9-archetypes), and
  [§9's shared enablers](game-design.md#shared-enabler-candidates);
- the grammar reference from the harness;
- `docs/sigils/registry.md`;
- the wave brief.

For each candidate, the designer:

1. Writes it in the grammar where possible. If the grammar can't express it,
   the designer writes the effect in plain terms and lists the hook it needs.
2. Checks the registry and its own batch for duplicates and near-duplicates.
3. Explains the decision it changes. A sigil that changes no decision is a
   **stat stick**, which is allowed but should be rare above common.
4. Notes which accepted sigils it combines with. Pair trials use these notes.

Cluster C owns enablers. An enabler must answer this question without naming a
future payoff: "Would I buy this before knowing my build?"

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

The critic receives the candidates, the registry, and this plan's
[Design rules](#design-rules). For each candidate, it checks:

- **The hard rules,** especially one effect, no activated abilities, and the
  excluded effects.
- **Duplicates** against the registry and the whole batch, across clusters.
- **Fun to play:** does it change a bid, a play, or a purchase, and does it
  create a memorable moment?
- **Fun to play against:** can opponents see it and respond through bidding or
  play?
- **AI-evaluability:** can search through the rules value its choices without
  sigil-specific code?

It returns pass, fix (with the exact fix), or drop, with a one-line reason.
It doesn't judge power; simulation does.

### Implementer

In Phase 0, the implementer builds the harness. In each wave, it receives the
passing candidates and:

- adds the grammar features and rule hooks they need, as general mechanisms,
  never per-sigil code;
- writes each candidate's data file;
- confirms the generated text passes lint;
- runs `scripts/ci`, commits with Conventional Commits, and delivers through
  Tollgate.

It returns, for each candidate: created, deferred (with what blocks it), or
changed (with what changed in the effect and why).

### Analyst

The analyst receives the list of candidate ids, the gates, the budget, and
the previous wave's thresholds. It runs:

1. **Screen.** At tier 0: simplicity cost, hand-level trigger rates, and a
   number sweep to the rarity's budget.
2. **Gates.** At tier 1: every gate that applies, with sequential testing.
3. **Fun score** on the pool as it would stand if every passing candidate
   were accepted.

It writes the reports and evidence, and returns one verdict per candidate:

| Verdict | Meaning |
| --- | --- |
| `pass` | Every gate passes at the chosen amount |
| `retune` | Passes at a different amount, which the analyst has already set |
| `revise` | Fails structurally; includes a diagnosis such as "fires in 4% of rounds; condition too narrow" |
| `cut` | Fails with no plausible fix, or breaks the power ceiling at every amount |
| `inconclusive` | Ran out of budget or confidence; carries forward |

It also returns the wave's pass rate. When that rate falls outside 25–90%, it
returns a threshold proposal.

### Namer

The namer receives the accepted sigils, `docs/sigils/icons.txt`, and the
registry's claimed names and icons. It follows 1.0's naming rules
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

The auditor receives the registry, this plan, and the analyst's final
evidence summary. It returns findings, ranked by severity, against the audit
list in the [balance wave](#balance-wave).

## Viewer

The viewer is the `/sigils` route of the prototype app, built in Phase 0a and
filled by every wave. It is modeled on 1.0's viewer (`~/rsp/src/sigils/`) and
is read-only. The data files are its only source.

- **Grid.** Each tile shows the icon (a Boxicons filled glyph from
  `@boxicons/core`, colored by rarity), the name, the rarity, and the
  generated rules text.
- **Filters:**
  - rarity;
  - category: points, +mult, ×mult, enabler, or hybrid;
  - archetype (internal, so the viewer is a design tool, not a player view);
  - status, defaulting to accepted;
  - source;
  - text search.
- **Detail panel:**
  - the full data and design notes;
  - the simplicity breakdown;
  - gate results with their confidence intervals;
  - the sweep and the chosen amount;
  - synergy pairs;
  - links to the reports;
  - for a cut candidate, why it failed.
- **Unnamed candidates** show their id and a placeholder glyph.
- **Running it:**

  ```bash
  npm run dev
  ```

  Then open `http://localhost:5173/sigils`.

## Checkpoints

The orchestrator commits and delivers through Tollgate at the end of each
Phase 0 build and each wave. (The implementer's code commits are delivered as
it goes.)

- **Messages** follow Conventional Commits and name the wave, such as
  `feat(sigils): accept wave 1 commons` or `docs(gdd): record pool shape
  decision`.
- **Bodies** summarize the wave:
  - candidates by source;
  - accepted, cut, and inconclusive counts;
  - coverage gaps;
  - fun score;
  - any threshold recalibration.
- **Delivery** follows `AGENTS.md`. On master, run `tg push-master --wait`. In
  a Tollgate worktree, submit with `tg candidate HEAD`, approve, and wait for
  promotion.

## Budget summary

| Phase | Subagents | Sim time |
| --- | --- | --- |
| 0a–0c | 3 implementers, 3 reviewers | Exit gates only |
| Wave 1, commons | 3 designers, critic, implementer, analyst, namer | 2 loops of about 6 h, plus the enumeration screen |
| Wave 2, uncommons | Same 7 | 2 loops of about 6 h |
| Wave 3, rares and legendaries | Same 7 | 2 loops of about 6 h |
| Wave 4, balance | Analyst, auditor, implementer, about 2 respawned designers | About 12 h, including tier 2 |
| **Total** | **About 32** | **About 2.5 days** |
