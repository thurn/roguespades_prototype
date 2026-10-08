# Rogue Spades 2.0: design search plan

A 12-hour, time-boxed search over the levers in the GDD's
[Open for testing](game-design.md#open-for-testing) table, ending in the
agent's own recommendation of the best design.

The agent spends the 12 hours at its discretion: simulation, analysis of game
logs, reading its own replays, and judgment. Metrics are a valuable tool and
are reported throughout, but they are a rough approximation of fun. The
deliverable is a reasoned opinion, backed by evidence, not a number that cleared
a bar.

## Outcome

- **A recommended design:** one rules file (`data/rules.json`) and one sigil
  pool, with the reasons for each choice, what the metrics said, and what they
  can't see.
- **One or two runners-up** where the evidence is close, each with its trade-off
  stated plainly.
- **A playable build** of the recommendation, to compare with master.
- **A report**, `docs/search/report.md`, plus a ledger of everything tried.

## Hard rules

- **Search only the table.** Every value outside
  [Open for testing](game-design.md#open-for-testing) is fixed. Don't add
  rule knobs, catch-up mechanics, offer steering, archetype pools, or hybrid
  penalties. If the evidence points at a change outside the table, report it
  as a question for the designer and don't build it.
- **Sigils have no activated abilities,** and every sigil follows GDD §5.
- **Metrics are approximations.** Say "no measured difference", never "costs
  nothing" or "adds nothing". Never pick a design you'd find ugly to explain
  because it scored a point higher. If the metric and your judgment disagree,
  say so and explain why.
- **Don't edit the fun score to raise it.** Measurement fixes (a broken
  estimator, a mis-scored sigil) are fine; record them in the ledger and
  re-measure the incumbent afterward.
- **Commits.** Phase 0 lands on master through Tollgate. Everything after it
  stays on the search worktree's branch, committed with Conventional Commits
  after `scripts/ci`; the designer decides what to promote.
- Code is disposable; write no tests.

## Starting point

The earlier design-iteration run (branch `claude/design-iteration`) is the
source of reusable infrastructure and of evidence, not of design:

- **Infrastructure to port:** the rules file read by `sim/` and the wasm
  client (`f490e08`); the paired fun-score comparison pipeline and the AI bid
  calibration fix (`0254e1c`); public deck composition and the per-sigil bid
  discount (`615e7dc`); the diagnostics, sweep, and pair-arm tooling
  (`26760fa`); bid candidates centered on the discounted estimate (`04ca5dd`).
  Port these, not the rules, levers, or sigils those commits also introduced.
- **Evidence worth knowing** (under that branch's old rules, so a prior, not a
  result):
  - Master's AI overbid: it predicted 79% makes and made 65%. Once fixed, the
    set rate fell to about 12% and the median margin to about 0.47 of the
    winner's score.
  - The team trailing after round 5 wins about 28%; plain Spades is more
    runaway than the game.
  - Commitment is the weak family: 31% of committed builds online by round 4,
    and committed teams win about 42%. Same-archetype pairs gained about
    nothing over their parts in win-rate units.
  - Removing engravings, synthetic cards, interest, and contract and nil gold
    showed no measured gameplay difference. Removing the card shop cost Skill
    about 2 points.

## Phase 0: base game on master (target: 2 hours)

1. Archive the old worktree: commit its uncommitted S1/S2 work to
   `claude/design-iteration` as-is. Don't merge it.
2. On master, port the infrastructure commits listed above, keeping only the
   rules-file keys the Open-for-testing table needs. Drop the keys for
   rejected levers (`overtrickValue`, `set.cpShare`, `set.floor`,
   `set.perUndertrick`, comeback offers, `behindX`, `affinity`,
   `runArchetypes`, interest).
3. Implement the new base rules in the kernel, wasm client, and game UI:
   - bags: overtricks plus nil bidders' tricks; −1,000 per 10, flat, carried
     over between rounds, shown on the scoreboard;
   - the symmetric set (contract points and every fired multiplier apply);
   - the flat-set alternative as a rules-file option;
   - nil gold 100, no interest;
   - no engravings or synthetic cards;
   - public sigils;
   - offer draws: with replacement (base), without replacement per team, and
     shared between teams, as a rules-file option;
   - starting multiplier as a rules-file value.
4. **Simplicity scorer.** Compute every sigil's C mechanically from its
   generated text and effect data, per the [GDD rubric](game-design.md#simplicity-rubric):
   +1 per number, rank, and suit literal, +1 per condition, +2 arithmetic,
   +1 selector or tracked state, +2 jargon (+4 at common). Check it against
   the rubric's worked examples, then replace every hand-entered
   `simplicity.C`. Expect the pool to read more complex than before, because
   suits now cost.
5. **Fun score v3** in `analysis/rsa/funscore.py`: the GDD §10 weights
   (20 / 15 / 15 / 15 / 10 / 25), the plain-Spades margin band, margin-point
   synergy, and the rescaled Simplicity band (full at mean C ≤ 3, zero at
   ≥ 7). Drop the rules-complexity charge.
6. Run `scripts/ci`, commit, and deliver with `tg push-master --wait`. Then
   create the search worktree from master.

## Phase 1: does the AI play the new rules? (target: 1 hour)

Bags and the symmetric set change bidding incentives, and the AI was badly
miscalibrated once before. Before trusting any number:

- **Bid calibration:** predicted against realized make rates, by round, at
  tiers 1 and 2.
- **Bid-offset test:** teams bidding −1 or +1 against their choice, in
  duplicate. If an offset team wins significantly more, fix the bidder first.
- **Bags:** check that the AI's search sees the 1,000-point bag penalty as it
  approaches 10 bags. A bidder blind to bags will show it as a rising bag rate
  late in the run.
- **A/A test** and one **planted effect** per family that drives decisions
  (a planted combo pair for Synergy, a flat boost to one archetype for
  Commitment).
- **Baseline:** measure the base game at tiers 1 and 2 on holdout seeds, and
  plain Spades for reference.

## Phase 2: rules and economy (target: 3–4 hours)

- **Screen** each lever in the table one factor at a time at tier 1 on dev
  seeds, at 2–4 values each. Start with the levers most likely to matter: set
  rule, starting multiplier, offer draws, sigil offers, slots, rounds, card cap,
  starting gold, and base income.
- **Combine** the two or three levers with the clearest effects in a small
  factorial around their best values.
- **Look past the score.** For each candidate worth keeping, read a few game
  logs. Do bids look like Spades? Do the bags matter? Does the shop feel
  starved or flooded (gold unspent, rerolls per shop)?
- **Confirm** the leading rules on holdout seeds at tiers 1 and 2 before
  moving on.

## Phase 3: the sigil pool (target: 4 hours)

On the Phase 2 rules:

- **Re-fit amounts** for the new scoring. The symmetric set makes contract
  points risky in a way they weren't, so the global scale and the per-sigil
  amounts both move. Fit the global scale once, then adjust individual sigils,
  with at most one retune per sigil per pass.
- **Simplify:** for each sigil with C above about 5, try a simpler same-slot
  version tuned to its best amount. Prefer the simpler one unless the complex
  one is clearly better on evidence and on feel.
- **Cut** sigils that are weak, redundant, or far outside the sigil metrics,
  keeping the pool near its soft size targets.
- **Synergy and commitment** are the biggest measured gaps. Design a few
  deliberate combo pairs and payoffs that reward going deep in one archetype.
  New sigils are allowed within the grammar and GDD §5. Register candidate
  pairs before the confirming read.
- **Exact and bags.** Bags give Exact a natural reason to exist; check that
  its payoffs now pull their weight before adding to them.

## Phase 4: recommendation (target: 2 hours, never skipped)

1. Run the recommendation and runners-up on fresh **final** seeds at tiers 1
   and 2, three replicates, against the base game.
2. Leave a playable build: `npm --prefix viewer run dev -- --port 5174` in the
   worktree, with the seeds worth playing listed.
3. Write `docs/search/report.md`:
   - **the recommended design** as a rules table and pool summary, and every
     difference from the base game;
   - **why:** the evidence, your judgment, and where they disagree;
   - **the fun score by family**, with intervals, for the recommendation, the
     runners-up, the base game, and plain Spades;
   - **what the metrics can't see,** and what to look for in playtests;
   - **open questions** outside the table, for the designer.
4. Update the GDD on the branch so it describes the recommendation.

## Statistical protocol

- **Seeds.** Disjoint **dev**, **holdout**, and **final** ranges in
  `data/search/seeds.json`. Holdout confirms; final is used once, in Phase 4.
- **Paired comparisons.** A candidate and the incumbent play the same boards
  in the same run of the pipeline. Report the paired difference with a 90%
  cluster-bootstrap interval for the total and every family. Never compare a
  number with one from an earlier run.
- **Size runs to the decision.** Aim for a paired 90% half-width of about 1
  fun point at tier 1; spend more only when a close call matters.
- **Accepting a change:** a gain whose interval excludes zero on dev, the same
  sign on holdout, and no unexplained drop in a family. A change with no
  measured difference may still be kept, or dropped, on judgment; say which
  and why.
- **Throughput** (18 cores, earlier runs): tier 0 about 1,000 runs/s, tier 1
  about 85, tier 2 about 13.5. A tier-1 arm of 10,000 runs takes about 2
  minutes; synergy reads need about 3,000 boards.

## Reporting during the run

- `docs/search/ledger.md`: every change tried, its dev and holdout estimates,
  the decision, and the reason.
- `docs/search/notes.md`: a short entry per phase, covering what you looked at,
  what surprised you, and what you'd look at next.
- Commit a checkpoint at the end of each phase.

## When time or usage runs out

At 12 hours, stop searching and finish Phase 4 with what you have. If the
usage limit nears first, write `HANDOFF.md` in the worktree with the current
phase, the incumbent rules and pool, open experiments, and the next step.
