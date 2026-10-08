# Rogue Spades 2.0: sigil design orchestration plan

> **Historical.** This plan is complete. The GDD was rewritten on 2026-10-08, so its
> section links below may not resolve; the current process is the
> [design search plan](design-search-plan.md).

This plan designs the full sigil pool for Rogue Spades 2.0. One
**orchestrator** session coordinates subagents through three stages:

1. **Build** the simulator, its measurement engine, and the viewer.
2. **Draft** a first pool, one rarity at a time.
3. **Optimize** the whole pool over several rounds.

Design is a **continuous optimization**, not a series of pass/fail checks.
Each round measures the pool against the metrics in
[GDD §12](game-design.md#12-metrics-what-fun-means) using strong AI players.
It then changes the pieces that most hold the game back, and keeps the changes
that make it better by the evidence and by aesthetic judgment.

Finding the best possible pool is intractable, so this plan doesn't try. It
runs a **guided local search**. Designers' judgment proposes a small number of
promising moves. A [measurement engine](#the-measurement-engine), built to
squeeze precise estimates out of noisy simulations, judges them. Most of the
plan's hidden difficulty lives in that engine.

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
  or replaced candidate, has a data file. It holds the sigil's estimates with
  their uncertainty, simplicity costs, report links
  ([GDD §16](game-design.md#sigil-records)), and a short history of why it was
  kept, changed, or cut.
- **A viewer site** at `/sigils`. It shows every sigil's icon, name, rarity,
  and rules text, with filters, and a detail panel with its evidence. It is
  modeled on 1.0's viewer (`~/rsp/src/sigils/`) and is read-only.
- **A reusable measurement engine:** the Rust simulator, the run database, and
  the analysis pipeline, so later tuning reuses the same machinery.
- **An up-to-date GDD.** §9 and §10 point to the data files and the viewer
  instead of illustrative sigils, and every decision and parameter this
  process touched has its evidence recorded.

This plan builds **no playable game**. The simulator is Rust and runs
headless; a playable TypeScript game comes later
([D29](game-design.md#d29-simulator-language)).

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
are in [§12](game-design.md#sigil-metrics). The
[measurement engine](#the-measurement-engine) section explains how each is
estimated.

| Metric | What it shows |
| --- | --- |
| Choices matter | Win-rate lift over a same-rarity flat-points control, in the contexts where the sigil is used; for point sigils, the share of the holder's wins it decides |
| Payoffs are doable | How often a payoff fires at base, and when its team commits to it |
| Standalone value | Enablers only: lift with no other sigils at shops 1, 3, and 5, and against a same-price flat-points control; then trigger lift for two payoff families ([§12](game-design.md#standalone-enabler-value)) |
| No invalidation | Sigils that touch opponents: how far they cut each archetype's win rate |
| Power ceiling | Lift, the strongest pairs, and the strongest attainable builds |
| Skill gradient | How much more the sigil is worth to a stronger AI ([Multi-fidelity estimates](#multi-fidelity-estimates)) |
| Simplicity | Itemized complexity cost C from the [§12 rubric](game-design.md#simplicity-rubric) |

A sigil outside a band is a **signal**, not a verdict. Large, confident misses
get attention first. A sigil can be well worth keeping while sitting outside a
band, for example a slightly too-strong rare that creates a run's best
moments.

### Pool metrics

The **fun score** ([§12](game-design.md#fun-score)) describes the whole pool:
Close and live, Commitment works, Archetypes viable, Synergy, Skill and
bidding, and Simplicity. It is measured once per round with an interval and
read as a **trend across rounds**, not as a test of single changes. One
changed sigil out of 145 moves it by far less than its measurement noise.

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

- the new version's estimates sit closer to its bands than the old one's;
- a coverage hole closed;
- a piece became clearly more elegant at little measured cost; or
- the round's fun-score trend held or improved.

It writes one line of reasoning per decision into the sigil's history.

- **Intervals say how sure the evidence is;** they are not cutoffs. A change
  whose effect is too uncertain to call is oversampled next round, or kept on
  elegance alone if it costs nothing measurable.
- **The comparison rule.** Between two versions doing the same job, prefer the
  simpler one unless the estimates show it is clearly worse.
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

## The measurement engine

This section is the technical heart of the plan. Phase 0 builds it, and every
later step uses it.

### Why this is hard

A run's outcome is dominated by card luck. Under naive design:

- **Noise.** A 2,000-run paired experiment resolves about ±2 win-rate points,
  so testing 145 sigils one at a time is unaffordable.
- **A moving target.** A sigil's value depends on every other sigil: what
  partners combine it with, what opponents build, and what the shop offers.
  Every change moves every estimate.
- **The shop AI is part of the measurement.** A sigil the shop misjudges is
  bought in the wrong builds, and its numbers mislead.
- **Relative versus absolute power.** Win rate is unchanged when every sigil
  gets stronger together, but the par curve and early-round share are not.
- **Sparse interactions.** About 10,000 possible pairs, and broken combos may
  need three or more pieces.
- **Selection bias.** Picking the best or worst of 145 noisy estimates
  systematically picks lucky or unlucky ones: the winner's curse.
- **AI limits.** Search with a one-round horizon can't value run-long growth.
  A cheap AI can rank sigils differently from a strong one, and a sigil can
  look weak only because the AI can't play it.

The engine answers each problem below. Every technique serves one idea: get a
precise estimate of **every** sigil's contribution **every** round, from one
large, well-designed experiment, rather than one small experiment per
question.

### Throughput

The simulator is native Rust
([D29](game-design.md#d29-simulator-language)), multithreaded across all
cores.

- **Cards are 64-bit masks.** State changes use apply and undo, with no
  allocation in hot loops.
- **Sigils compile to dispatch tables** from their JSON data when a pool
  loads.
- **Every budget is sized from measured throughput.** Phase 0 measures runs
  per hour at each tier on this machine (18 cores); no budget in this plan
  assumes a number it hasn't measured.

| Tier | Target | Use |
| --- | --- | --- |
| 0 | At least 100 runs per second on the machine | Bulk tournaments, sweeps, screens |
| 1 | At least 5 runs per second | Calibration and confirmation |
| 2 | At least 0.5 runs per second | Final spot checks and the skill ladder |

If a tier misses its target, the analyst shrinks experiment sizes using the
precision formula in [Budget sizing](#budget-sizing) and reports the cost in
precision.

### Run format

These choices reduce noise before any statistics run.

- **Seeded streams** follow [§13](game-design.md#experiments): deals per
  round, offers per team and shop, Opening draws, and AI search.
- **Stable dealing.** Each round's deal stream is a fixed permutation of the
  52 cards. Owned cards are pulled out, and the random fill takes the next
  cards in permutation order. Two arms whose owned cards differ slightly still
  get nearly the same hands.
- **Duplicate play.** Every seed is played twice, with the teams swapping
  seats, as in duplicate bridge. Each team keeps its own grants, offers, and
  decisions, but faces the other side's cards in the second run. Averaging
  the pair cancels most deal luck between the teams. A **board** is one such
  pair of runs and is the unit of analysis.
- **Smoothed outcome.** Each board yields a continuous outcome rather than
  win or loss:

  ```
  u = mean over the board's two runs of (2·W(margin) − 1)
  ```

  W is a logistic curve from final score margin to win probability, fitted on
  clean runs (below). Margins carry far more information than a win or loss.
  Effects are reported back in win-rate points through the same curve.
- **Run records.** Every run writes one compact record:
  - seed, code version, and pool version;
  - each team's sigils, with how each was acquired (shop or grant), the shop,
    the amount, and the price paid;
  - owned cards and gold by shop;
  - bids, tricks, and round scores;
  - each sigil's ledger contributions and trigger counts;
  - the outcome.

  Records go to `runs/` as Parquet, which git ignores, so any model can be
  refit without rerunning.

### The grant tournament

The **grant tournament** is the engine's main experiment, run every round. It
measures every sigil in the pool, and every new candidate, at once.

- **A normal game.** Both teams shop with the
  [fitted shop policy](#the-shop-value-model).
- **Random grants.** At each of shops 1–6, each team receives, with set
  probability, one **forced purchase** of a sigil drawn from the grant
  distribution. It pays the full price and takes a slot; if the slots are
  full, the shop policy sells its least valuable sigil. About three grants per
  team per run.
- **The grant distribution** is a mixture:
  - about 40% uniform over the sigils being measured, which measures value in
    any build;
  - about 60% **build-coherent**: an archetype is chosen, weighted toward
    what the team already holds, and the grant comes from sigils tagged with
    it. This puts payoffs in the committed contexts they're designed for.
- **Adaptive allocation.** Each sigil's exposure is weighted by its
  uncertainty from the previous fit, with a floor for every sigil. New
  candidates and sigils near a decision get more grants.
- **Candidates are measured but never offered.** A candidate enters runs only
  through grants until it is kept, so testing it doesn't distort the shop.
  Opponents receive grants too, so effects on opponents are measured.
- **Gold perturbations.** A small random gold bonus or penalty at random
  shops measures what a unit of gold is worth at each shop. That is the
  shadow price the shop policy and the value-per-gold readings need.
- **Controls.** The same-rarity flat-points controls from §13 ("+N contract
  points", N at the rarity's flat budget) are in the grant distribution, so
  every lift is measured against them in the same runs.
- **Clean runs.** About 10% of boards have no grants and no perturbations.
  They measure the game as players will see it: Close and live, set rates,
  overtricks, and the par curve.

Because grants are random, comparing teams with and without a grant measures
its **causal** value, including how the shop policy builds around it, which is
what the GDD's forced pick means. Shop-chosen purchases are never used to
estimate value, because a team buys good sigils when it's already ahead.

### The outcome model

The analyst fits one model per tournament to all boards. For board d, between
teams A and B:

```
u_d = S(A) − S(B) + ε_d

S(T) = Σ over grants g of team T:
         β_i + β_i^amt · log(a_g / a_i) + β_i^shop · (t_g − 3.5) + β_i^coh · c_g
     + Σ over pairs of grants g, h of team T:  ⟨v_i, v_h⟩
     + Σ over gold perturbations of team T:    λ_t · Δgold
```

- **β_i** is sigil i's value at its current amount, acquired at a middle
  shop.
- **β_i^amt** is the slope in log amount, from randomized amounts (see
  [Amount tuning](#amount-tuning)).
- **β_i^shop** captures how value changes with the acquiring shop t_g; early
  sigils play more rounds.
- **β_i^coh** captures how value grows with coherence c_g: how many of the
  team's other grants share an archetype tag with i. A payoff that needs
  support shows a large β^coh.
- **⟨v_i, v_h⟩** is a factorization machine. Each sigil gets a short latent
  vector (rank 4–8), and pair synergy is their dot product. It estimates all
  10,000 pairs from far fewer co-occurrences by sharing structure.
- **λ_t** is the value of gold at shop t.

Only grants enter the sums. Shop-chosen sigils are part of the environment the
grant's effect is measured in.

**Fitting.** Ridge-regularized least squares, with the factorization machine
fitted by alternating least squares. Regularization strength comes from
cross-validation over boards.

**Shrinkage.** Each β_i's prior mean is the mean for its rarity and category.
That is empirical Bayes: noisy estimates are pulled toward their group, which
corrects the winner's curse and the multiple-comparison problem in one move.

**Uncertainty.** A cluster bootstrap over boards, with 200 refits, gives every
estimate a 90% interval.

**Readings.** Each sigil metric is read from the fit:

| Metric | Reading |
| --- | --- |
| Choices matter | β_i − β_control at the median shop. Payoffs use committed coherence (c = 2); enablers and generic sigils use c = 0 |
| Decisive share | Ledger rescoring of the holder's wins without the sigil, from run records |
| Payoffs are doable | Trigger counts in run records, at base and in coherent grants |
| Synergy | Pair terms ⟨v_i, v_j⟩, relative to each part's value |
| No invalidation | From commitment arms (see [Builds and power](#builds-and-power)) |

### Budget sizing

Precision follows a simple formula. A sigil with n_i grant exposures has
standard error about:

```
SE_i ≈ σ_u / sqrt(n_i)      n_i ≈ (grants per board · boards) / sigils measured
```

σ_u is the residual spread of the smoothed outcome after duplicate play and
the model, which Phase 0's pilot measures.

**Example,** with assumed values the pilot will replace:

- inputs: σ_u = 0.35 in win-probability units, 6 grants per board, 170
  sigils measured, and 60,000 boards (120,000 runs);
- result: about 2,100 exposures each, an SE of about 0.8 win-rate points, and
  a 90% interval of about ±1.3 points;
- cost: at the tier-0 target, 20 minutes.

The analyst sizes every tournament to a **precision target**: a median 90%
half-width of at most 1.5 win-rate points at tier 0, after
[multi-fidelity correction](#multi-fidelity-estimates). Run counts follow
from that target.

### Multi-fidelity estimates

Bulk tournaments run at tier 0. Tier 0 and tier 1 can disagree, most of all
on sigils that reward skilled play. Each round therefore runs a **calibration
tournament** at tier 1, using about a tenth of the boards with the same seeds
and grants.

- **The correction.** The analyst fits δ_i = β_i(tier 1) − β_i(tier 0) on the
  shared boards. It shrinks δ_i toward a feature model (category, trigger
  type, and whether the sigil adds a choice) and reports β_i(tier 0) + δ_i
  as the working estimate. That is a control-variate estimator: it gets
  near-tier-1 accuracy at near-tier-0 cost while the tiers correlate well.
- **Skill gradient.** δ_i itself is reported as each sigil's skill gradient.
  A large positive gradient means stronger play gets more from the sigil.
  That is good for Skill and bidding, but it also warns that tier 0
  understates the sigil and that humans may too.
- **Low agreement.** If the tiers' estimates correlate below 0.7, the analyst
  raises the tier-1 share of the next tournament and says so.
- **Tier 2** checks rankings in the final round
  ([Final round](#final-round)).

### Amount tuning

Number sweeps are folded into the tournament:

- **Randomized amounts.** Each grant draws its amount log-uniformly from about
  ×0.5 to ×2 of the sigil's current amount a_i, so β_i^amt measures the
  dose-response. Offered copies always use a_i.
- **A damped Newton step.** After each fit, every sigil's amount moves halfway
  toward its target value τ_r (in log space), clamped to ×0.5–×2 per round:

  ```
  a_i ← nice( a_i · exp( 0.5 · (τ_r − β_i) / β_i^amt ) )
  ```

- **Nice numbers.** `nice` rounds to the nearest value a designer would
  print: multiples of 5 for points and +multipliers, and halves for
  ×multipliers. The simplicity rubric charges the same for +15 and +17, but
  elegance doesn't.
- **When amount doesn't matter.** If β_i^amt's interval includes zero, the
  amount isn't what limits the sigil. It isn't retuned, and is flagged as a
  structural problem for the designer.
- **Targets.** τ_r is the rarity's control value plus a target lift Δ_r, set
  in the middle of the Choices matter band and below the power ceiling.
  Starting values are about +2 win-rate points at common, +3 at uncommon, +5
  at rare, and +8 at legendary; the first band re-centering revisits them.
- **Conditions aren't amounts.** Thresholds such as "three tricks" or "bids 8
  or more" change the condition, so designers propose them as variants, and
  each variant is a separate candidate.

Retuning runs automatically every round for every sigil, and is recorded in
each sigil's history.

### Global scale

Win-rate lift can't see uniform power creep, so the absolute score scale has
its own loop.

- **Clean runs measure the scale:** mean round score by round against the par
  curve ([§4](game-design.md#par-curve)), and the share of points scored in
  rounds 1–4.
- **A few global knobs set it:** one amount multiplier per rarity and
  category, and the starting contract multiplier (P29).
- **Order.** Each round adjusts global knobs first, damped by half, and then
  relative amounts. The controls scale with their rarity, so relative targets
  stay meaningful.

### The shop value model

Bulk runs can't afford rollout rescoring at every shop, so they shop from a
fitted model ([D17](game-design.md#d17-shop-ai)).

- **Value of an offer** is its predicted gain in win probability, net of
  price:

  ```
  value = β_i + β_i^shop · (t − 3.5) + β_i^coh · c + Σ owned j ⟨v_i, v_j⟩ − λ_t · price
  ```

  Cards use the same form, with features for rank, suit, and the sigils that
  reward them.
- **Buying.** Follow §13's policy: buy the best value while it beats holding
  the gold, reroll when nothing does, and sell when an offer beats an owned
  item by more than the sale loses.
- **Committed policy:** adds a bonus for one archetype's tags, used in
  commitment arms.
- **Exploration.** About 5% of purchases are random among affordable offers,
  so the policy keeps seeing alternatives.
- **New sigils** get a starting value from a feature model: rarity, category,
  trigger type, and hand-level trigger rate. The tournament replaces it once
  the sigil has been granted.
- **Iteration.** Each round's policy uses the previous round's fit: a
  fictitious-play loop. Coefficients are averaged with the previous model's
  to damp oscillation.
- **Validation.** On a subsample of shop visits, the analyst compares the
  model's ranking of offers with tier-0 rollout rescoring from §13. It reports
  rank correlation, and corrects sigils the model clearly misprices.

### Builds and power

Pair terms and archetype-level trials cover what single-sigil estimates
can't.

- **Commitment arms.** One arm per archetype plays a committed shopper against
  the flexible field, in duplicate. They measure Commitment works ("online"
  by round 4) and Archetypes viable (committed win rates, and each
  archetype's share of winning flexible builds).
  - **No invalidation.** In these arms, opponents receive random grants of the
    sigils that touch opponents. The interaction between those grants and the
    committed archetype estimates each archetype's loss.
- **Standalone enabler arm.** A small tournament where shops sell cards but
  no sigils, and grants are enablers and controls only. It measures
  standalone value by acquiring shop, with all enablers in one experiment.
- **Pair confirmation.** The top 20 predicted pairs, from the factorization
  machine and designer nominations, are oversampled as joint grants in the
  next tournament. The top few also get confirmation runs on fresh seeds,
  because a maximum over 10,000 noisy pairs is always optimistic.
- **The build search finds the strongest attainable builds.**
  1. A beam search over loadouts of sigils and key cards maximizes the fitted
     S(T). It is weighted by **attainability**: the chance of seeing those
     offers by the shop they're needed, under offer odds and one reroll per
     shop.
  2. The top 10 builds become **build-chaser** arms: shoppers that buy their
     build's pieces when offered and otherwise shop flexibly. They play
     against the flexible field at tier 1.
  3. **Successive halving** gives each arm a small budget, keeps the best
     half, and doubles the budget, until three remain with tight intervals.

  The best build-chaser's win rate is the **power ceiling** reading. Broken
  combos that need luck to assemble are fine; dominant strategies are not.

### Fun score measurement

Each round, the fun score comes from parts the round already produced:

| Family | Source |
| --- | --- |
| Close and live | Clean runs |
| Commitment works, Archetypes viable | Commitment arms |
| Synergy | Pair terms and confirmations |
| Skill and bidding | Clean-run diagnostics, plus a tier-1 against tier-0 ladder each round; tier 2 against tier 0 in the final round |
| Simplicity | Computed from sigil data |

A cluster bootstrap gives the score and each family an interval. The round
notes plot every family's trend across rounds.

### Choosing what to change

Between rounds, the orchestrator picks about 15–25 structural targets. It
ranks sigils by a **priority score**:

```
priority_i = Σ over metrics m:  w_m · max(0, distance_m(i) / SE_m(i) − 1)
           + elegance flags + coverage-hole bonus
```

Subtracting one standard error means noise alone never makes a sigil a
target. The score is a ranking aid, not a rule.

- **Retunes don't count against targets.** Amount retuning runs every round
  for every sigil.
- **Stability.** At most about 15% of the pool changes structurally per
  round. A smaller change keeps the environment stable enough that this
  round's estimates still describe next round's pool.

### Statistical hygiene

- **Fresh seeds** every round. Decisions and confirmations never share seeds.
- **Data reuse.** Each fit uses the current round's boards, plus earlier
  rounds' boards at half weight per round of age. A changed sigil gets a new
  id; a retuned one is covered by its amount term.
- **Shrinkage everywhere.** Every reported estimate is shrunk. Raw estimates
  are kept for diagnostics only.
- **Selection.** Any "best" or "worst" claim that drives a decision is
  re-measured on fresh boards before it is acted on.

### Pipeline validation

The engine must prove it measures what it claims before any design work
relies on it. Phase 0's pilot checks:

- **Planted effects.** Controls at several known amounts must give a
  monotone dose-response. A synthetic dataset generated from a known model
  must be recovered within the intervals.
- **An A/A test.** The same sigil under two ids must get matching estimates.
- **Variance reduction.** The pilot measures how much duplicate play and
  smoothing shrink σ_u, and reports the effective sample-size gain.
- **Tier agreement.** The pilot reports the correlation between tier-0 and
  tier-1 estimates.

### Known biases

| Bias | Effect | Mitigation |
| --- | --- | --- |
| AI blind spots | A sigil looks weak because the AI can't use it | Skill gradient, tier-2 checks, later human playtests |
| Shop model error | Misjudged sigils are bought in the wrong builds | Value from grants only; exploration; rollout validation |
| Hidden opponent sigils | The AI searches as if unrevealed sigils don't exist, which favors surprise effects | Recorded as an assumption; opponents' revealed sigils are used |
| Grant economy | Forced buys distort gold and slots | Game-level metrics come only from clean runs |
| Moving environment | Estimates describe last round's pool | Re-estimate every round; damping; at most 15% change per round |
| Overfitting to the AI | The pool fits the AI's play, not people's | Tier 2 in the final round; pending playtest targets |

### AI design requirements

These requirements go to the Phase 0b implementer. They matter because sigils
change what good play is.

- **Win probability as utility.** Search and bidding maximize win probability
  over the rest of the run, not round score. Each outcome's margin is
  converted through W and the rounds left. This makes late, trailing teams
  take risks, as people do.
- **Run-long state has value.** Growth counters and gold are valued at the
  end of a searched round, using the shop value model's coefficients, so the
  AI plays toward growth sigils.
- **Opening choices,** such as "Swap three cards with your partner": generate
  about 10 candidate choices heuristically, then evaluate them by rollouts at
  tier 1 and above. Tier 0 uses the heuristic alone.
- **Determinization** samples hidden hands consistent with voids, bids, the
  partner's owned cards, and synthetic replacements, without slow rejection
  sampling.
- **No sigil-specific code.** Every choice comes from search over the real
  rules and scoring.

## Roles

Only the orchestrator decides. Every subagent gets this plan's
[Game in brief](#game-in-brief) and [Design rules](#design-rules), its own
role section, and the files listed for it.

| Role | Writes | Job |
| --- | --- | --- |
| Orchestrator | Briefs, keep decisions, GDD records, commits | Runs the plan and decides what stays |
| Implementer | Code and candidate data files | Builds the engine, then adds the grammar features and rule hooks new designs need |
| Reviewer | Nothing | Checks each Phase 0 build against its done criteria |
| Designer | Nothing; returns text | Writes new candidates, reshapes, and replacements |
| Critic | Nothing; returns text | Reviews candidates before simulation, and gives the elegance review each round |
| Analyst | Reports, run data, and estimates in data files | Runs the engine and returns the dashboard |
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
| `sim/` | Implementer | The Rust workspace: kernel, sigil grammar and text, AI, shop, runner, and the `rsim` command-line tool |
| `analysis/` | Implementer | The Python analysis pipeline, managed with `uv`: model fitting, bootstrap, amount tuning, build search, and the dashboard |
| `viewer/` | Implementer | The Vite + React + TypeScript viewer |
| `data/sigils/<id>.json` | Implementer creates; analyst adds estimates; orchestrator sets status, history, name, and icon | One file per candidate, including cut and replaced ones |
| `data/models/<round>.json` | Analyst | Each round's fitted model, which the next round's shop policy reads |
| `runs/` | Analyst | Run records as Parquet; ignored by git |
| `docs/sigils/registry.md` | Generated by `rsim registry` | Compact table of every candidate: id, status, rarity, name, icon family, generated text, simplicity cost C, key estimates, and effect signature. Agents read this, not the data files |
| `docs/sigils/icons.txt` | Copied from `~/rsp/docs/sigils/icons.txt` | 431 curated filled Boxicons, one per distinct image |
| `docs/sigils/rounds/<step>.md` | Orchestrator | Each pass's or round's brief, then its outcome: changes tried, kept, and reverted with reasons, fun score by family, coverage holes, and band changes |
| `reports/<experiment>.md` and `.json` | Analyst | Experiment reports ([§16](game-design.md#experiment-reports)) |

`scripts/ci` grows with the code: `cargo fmt --check`, `cargo clippy`, and a
release build for `sim/`; `ruff` for `analysis/`; and format, lint, and build
for `viewer/`.

### Sigil data file

The simulator owns the format. It must hold at least:

- **Identity:** `id`, `rarity`, `price`, `role` (payoff, enabler, or
  hybrid), internal `archetypes` (which drive coherent grants), `source`
  (`enumerated`, `designed`, or `gdd-seed`), and the step that created it.
- **The effect** in the declarative grammar ([§13](game-design.md#declarative-sigils)),
  with its current amounts, plus the rules text generated from it.
- **Design notes:** `decision`, the choice it changes for its owner;
  `opponent`, how it looks from the other side of the table; and
  `rationale`, including any guideline departure and why.
- **Status:** `candidate`, `kept`, `cut`, or `replaced` (with the id that
  replaced it).
- **History:** one entry per decision, with the step and a line of
  reasoning, such as "Round 2: amount 15 → 20 by retune; lift was 2.4 points
  under target".
- **Name and icon,** after naming: `name`, `icon`, `iconFamily`, and
  `iconWord`.
- **Estimates:** simplicity costs (itemized, with C and S), the sigil-metric
  readings with 90% intervals and exposure counts, the amount slope, the
  skill gradient, its strongest pairs, and report links.

The **effect signature** normalizes the grammar data as trigger | filter |
scaling | effect type. Two candidates are **near-duplicates** when their
signatures differ only in amount, named suit, or named rank. Near-duplicates
are an elegance concern for the critic, not an automatic cut.

## Phase 0: engine and viewer

Four implementer builds run in order. Each build gets one implementer and
then one reviewer. Each is committed and delivered through Tollgate before the
next build starts.

These are engineering checks on the engine, not design decisions, so they have
concrete done criteria.

- **No test suites** (per `AGENTS.md`). Each build proves itself by running
  its done criteria as a script.
- **Repair limit:** a build that misses its criteria gets two repair rounds.
  After that, the orchestrator proceeds, records the shortfall in
  `docs/sigils/rounds/phase-0.md`, and repeats it in every later round's
  notes.

### 0a: kernel, grammar, and viewer

Build:

- **The Rust workspace in `sim/`,** with the rules kernel from
  [§13](game-design.md#the-rules-kernel):
  - mask-based cards, with apply-and-undo moves;
  - hook tables for effective suit and rank, legal plays, the next leader,
    and scoring events;
  - a scoring ledger that can recompute any score without any set of sigils;
  - seeded streams with stable dealing.
- **The declarative sigil grammar and interpreter**
  ([§13](game-design.md#declarative-sigils)), the rules-text generator, the
  text lint, and the simplicity scorer
  ([§12 rubric](game-design.md#simplicity-rubric)). `rsim gen-text` writes the
  generated text into each data file.
- **GDD seeds** as data files, with `source: gdd-seed`:
  - every illustrative sigil in §9;
  - the 7 shared enablers;
  - the 2.0 seeds in [Appendix E](game-design.md#appendix-e-seeds-from-rogue-spades-10).
- `rsim registry`, and `scripts/ci` extended as above.
- **The viewer** in `viewer/`. See [Viewer](#viewer).

**Done when:**

- Every §9 seed's generated text matches its GDD text exactly, or the
  difference is listed as a GDD typo.
- Every row of the §12 rubric table scores the C shown there.
- A plain round of Spades plays and scores as in the
  [§4 worked examples](game-design.md#worked-examples).
- A random-play benchmark reports card plays per second per core.
- The viewer lists every seed.
- `scripts/ci` passes.

### 0b: AI tiers

Build:

- **Port 1.0's AI** (`~/rsp/src/ai/`) to Rust: deal sampling fitted to bids,
  information-set MCTS, tie-breaking among near-equal moves, and bidding and
  nil decisions.
- **Tiers 0–2** at the [§13](game-design.md#ai-tiers) budgets.
- **The [AI design requirements](#ai-design-requirements).** Until a fitted
  model exists, W comes from plain-Spades runs.
- **A plain-Spades bench,** reusing 1.0's metric definitions
  (`~/rsp/docs/prototype-status.md`).

**Done when** (plain Spades):

- At tier 1, the team set rate is at most 18%.
- At tier 1, nil success is at least 65%, and nil suicides are under 1%.
- Tier 2 beats tier 0 in at least 65% of boards.
- Measured throughput for each tier is reported against the
  [throughput targets](#throughput).

### 0c: shop, runner, and run records

Build:

- **The economy:** the card shop, the 8-card cap, income, interest, and
  rerolls; engravings and synthetic cards at their GDD starting values; the
  sigil shop, with its rarity odds and prices.
- **The shop policy,** from a model file (a hand-set starting model until the
  first fit), with flexible, committed, and build-chaser variants and
  exploration.
- **Tier-0 rollout rescoring** for shop validation.
- **The runner,** running every experiment in the
  [measurement engine](#the-measurement-engine) from one config file:
  - duplicate boards;
  - grant tournaments with mixture and adaptive grant distributions;
  - randomized amounts and gold perturbations;
  - clean runs;
  - commitment, standalone, pair-confirmation, and build-chaser arms;
  - successive halving.
- **Run records** to Parquet.
- **The common-payoff enumerator:** every grammar combination at common, about
  270 candidates ([D21](game-design.md#d21-candidate-generation)).

**Done when:**

- Flexible against flexible scores 50% within its interval.
- A small grant tournament over the seed pool runs end to end and writes
  records.
- Two runs from the same seed and config produce identical records.

### 0d: analysis pipeline and pilot

Build:

- **The Python pipeline in `analysis/`:**
  - the W curve;
  - the outcome model, with its factorization machine, shrinkage, and
    cluster bootstrap;
  - multi-fidelity correction;
  - amount tuning and global-scale adjustment;
  - the build search;
  - the fun score;
  - writing estimates into data files;
  - the dashboard: one table of every sigil's readings against its bands,
    plus the fun score by family;
  - the shop model export.
- **One command per step,** such as `uv run analysis round --round 2`, that
  runs the experiments, fits, and writes everything.
- **A pilot on the seed pool** to set the budgets.

**Done when:** the [pipeline validation](#pipeline-validation) passes, and the
pilot reports:

- σ_u, and how much duplicate play and smoothing reduce it;
- tier-0 and tier-1 agreement;
- rank correlation between the shop model and rollout rescoring;
- the board counts each tier needs to meet the precision target.

### GDD alignment

At the end of Phase 0, the orchestrator writes the pool-shape decision into
the GDD:

- **D30, "Pool shape":** soft targets of about 145, replacing §10's slot
  tables and the pool sizes in §5's rarity table.
- **P21:** the new targets and their range.
- **D21:** the enumerated-versus-designed comparison this plan runs at common.

It also records the pilot's measured throughput, σ_u, and tier agreement in
P24 and P27.

## Draft passes

Three passes build a first pool, one rarity at a time: commons, then
uncommons, then rares and legendaries. Each pass keeps somewhat more than its
rarity target, about 110–120%, so the optimization rounds have room to prune.

The shop offers whatever has already been kept, plus the controls. Each pass's
candidates enter runs only through grants.

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
5. **Measure.** The analyst:
   1. runs a **hand-level screen**: trigger rates on fixed builds, which sets
      aside candidates that almost never or almost always fire;
   2. runs a **grant tournament** on the rest, with successive halving: a
      first tournament over all of them, then a second that concentrates
      grants on the more promising and the uncertain half;
   3. returns the dashboard.
6. **Choose.** The orchestrator keeps the most promising candidates toward the
   pass's targets, weighing estimates, elegance, coverage, and the comparison
   rule. The rest are cut, with one line of reasoning each.
7. **Checkpoint.** The orchestrator writes the pass outcome, regenerates the
   registry, commits, and delivers through Tollgate.

### Commons: two sources

Commons come from two sources run side by side, to settle
[D21](game-design.md#d21-candidate-generation):

- **Enumerated.** The enumerator's about 270 candidates go through the
  hand-level screen. Its most promising half joins the tournament.
- **Designed.** The three designers work **blind to the enumeration**, so the
  comparison is fair. GDD seeds at common enter with the designed source.

Where the two sources produce the same signature, they merge into one
candidate that credits both. The pass notes report how each source's
candidates measured and how many each supplied to the draft pool. D21's record
is updated with those numbers.

### Rares and legendaries

Power matters most here. Designers nominate likely partners for each rare and
legendary, and those pairs are oversampled as joint grants. The pass ends with
a first [build search](#builds-and-power). Legendaries are about 5
build-arounds; they need not fit any one archetype.

## Optimization rounds

Once the draft pool exists, up to **four rounds** improve it as a whole. Any
round can change any sigil at any rarity.

Each round runs these steps.

1. **Measure.** The analyst runs the round's experiments and returns the
   dashboard:
   - the grant tournament with its tier-1 calibration and clean runs;
   - commitment arms, the standalone enabler arm, and pair confirmations;
   - the build search, every few rounds or when the top builds change.

   It also applies amount retunes and global-scale adjustments. Round 1
   proposes re-centered bands (see [Re-centering bands](#re-centering-bands)).
2. **Review.** The critic gives its ranked elegance review of the whole pool.
3. **Choose targets.** The orchestrator writes the round's brief in
   `docs/sigils/rounds/round-<n>.md`. It lists about 15–25 structural
   targets, using the [priority score](#choosing-what-to-change), the
   critic's top concerns, coverage holes, the weakest fun-score families,
   and trimming toward about 145. Each target names the change wanted:
   reshape, replace, merge, cut, or add.
4. **Redesign.** One designer, given the targets, the dashboard rows, and the
   critic's notes, proposes one or two options per target.
5. **Critique and implement.** The critic reviews the proposals, and the
   implementer builds what they need.
6. **Measure the changes.** The analyst measures the new versions in a
   focused grant tournament that oversamples them, against the current pool.
   It reports old and new estimates side by side.
7. **Keep or revert.** The orchestrator keeps or reverts each change, with one
   line of reasoning in the sigil's history (see
   [Keeping changes](#keeping-changes)).
8. **Checkpoint.** The orchestrator writes the round's outcome, updates the
   GDD's touched records, regenerates the registry, commits, and delivers
   through Tollgate.

### Stopping

Optimization stops after round 4, or earlier when a round **plateaus**. A
round plateaus when its fun-score trend is flat within its interval, the share
of sigils inside their bands stops rising, and the critic raises no major
elegance concerns.

### Final round

The last round, whether round 4 or an earlier plateau, adds these steps
before its checkpoint.

1. **Full measurement.** A larger grant tournament with a larger tier-1 share
   brings every estimate to its tightest interval.
2. **Tier calibration.** Tier 2 spot-checks the top and bottom sigils, every
   sigil near a keep decision, and the best build-chasers, and runs the tier-2
   ladder for Skill and bidding. Where tier 2 disagrees with the working
   estimate, the orchestrator revisits that decision.
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
   - the [known biases](#known-biases) with their measured size;
   - pending playtest targets;
   - any Phase 0 shortfall.

   It updates the GDD so §9 and §10 point at the data and the viewer.

## Role instructions

### Designer

Each designer receives:

- the GDD's [§5](game-design.md#5-sigils), its archetype sections in
  [§9](game-design.md#9-archetypes), and
  [§9's shared enablers](game-design.md#shared-enabler-candidates);
- the grammar reference from the simulator;
- `docs/sigils/registry.md`;
- the pass or round brief.

For each candidate, the designer:

1. Writes it in the grammar where possible. If the grammar can't express it,
   the designer writes the effect in plain terms and lists the hook it needs.
2. Tags its archetypes; coherent grants and the coherence term depend on
   these tags.
3. Checks the registry and its own batch for duplicates and near-duplicates.
4. Explains the decision it changes. A sigil that changes no decision is a
   **stat stick**, which is fine at common and should be rare above it.
5. Names likely partners. Pair oversampling uses these.

Amounts are starting points; retuning moves them. Designers spend their effort
on the condition and the shape.

In optimization rounds, the designer works from a target's dashboard row and
the critic's notes.

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

In Phase 0, the implementer builds the engine. In each pass or round, it
receives the candidates and:

- adds the grammar features and rule hooks they need, as general mechanisms,
  never per-sigil code;
- writes each candidate's data file and generated text;
- confirms the text passes lint;
- runs `scripts/ci`, commits with Conventional Commits, and delivers through
  Tollgate.

It returns, for each candidate: created, deferred (with what blocks it), or
changed (with what changed in the effect and why).

### Analyst

The analyst receives the step's experiment list, the sigil ids to measure,
the bands, and the budget. It runs the engine and returns:

- **the dashboard:** every measured sigil's readings against their bands, with
  90% intervals and exposure counts, and the fun score by family;
- **diagnoses** for large misses, such as "fires in 4% of rounds; the
  condition is too narrow" or "the amount slope is flat; the effect doesn't
  change outcomes";
- **retunes and global-scale changes** it applied, with before and after;
- **side-by-side estimates** for each changed sigil;
- **engine health:** tier agreement, shop-model agreement, σ_u, and anything
  it couldn't measure within the budget.

It writes reports, run data, and estimates. It never decides what to keep.

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

The viewer in `viewer/` is a small Vite + React + TypeScript site, modeled on
1.0's viewer (`~/rsp/src/sigils/`). It is read-only, and the data files are
its only source; it never reimplements rules or text.

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
  - each sigil metric drawn against its band, with its interval;
  - the amount history and slope;
  - the skill gradient;
  - the strongest pairs;
  - the history of decisions and their reasoning;
  - links to the reports.
- **Running it:**

  ```bash
  npm --prefix viewer run dev
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
  - retunes;
  - the fun score by family;
  - coverage holes;
  - engine health;
  - any band changes.
- **Run data stays local.** Only reports, models, and estimates are
  committed.
- **Delivery** follows `AGENTS.md`. On master, run `tg push-master --wait`. In
  a Tollgate worktree, submit with `tg candidate HEAD`, approve, and wait for
  promotion.

## Budget summary

Times assume the [throughput targets](#throughput). The pilot replaces them
with measured numbers, and the analyst sizes each step to the precision
target.

| Step | Subagents | Simulation time |
| --- | --- | --- |
| Phase 0 | 4 implementers, 4 reviewers | Pilot, about 2 h |
| Draft: commons | 3 designers, critic, implementer, analyst | About 3 h |
| Draft: uncommons | Same 6 | About 2 h |
| Draft: rares and legendaries | Same 6 | About 3 h, including the build search |
| Optimization rounds 1–4 | Analyst, critic, designer, implementer per round | About 3–4 h per round |
| Final round extras | Namer, auditor | About 6 h, including tier 2 |
| **Total** | **About 44** | **About 1.5–2 days** |

Each step leaves about 2× headroom under the earlier 6-hour ceiling, in case
measured throughput falls short.
