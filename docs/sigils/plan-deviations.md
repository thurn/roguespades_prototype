# Deviations from the sigil design plan

The orchestrator ran [the plan](../sigil-design-plan.md) without pausing for review and relaxed
or changed these points. Each entry says what changed and why.

## Engine

- **One Rust crate, not a workspace.** `sim/` is a single `rsim` crate with modules for the
  kernel, grammar, text, simplicity, AI, game, and experiments.
- **New rule hooks from the designers** are general mechanisms: Opening changes with a source
  filter ("Every [♣] your team holds becomes a [♦]"), wins filtered by the led card, leading [♠]
  before they are broken, leading the first trick, and playing any suit on the first N tricks or
  once the contract is won. The uncommon pass added: a filtered set of your team's cards that
  can't be trumped, filtered Opening raises, filtered cards your team may play even when it can
  follow suit, filtered cards that outrank the [A] of their suit, Opening changes to the
  opponents' cards, contract-scaled ×multipliers that compound once per contract trick, and a
  bid trigger for outbidding the opponents.
- **Passes overlap.** The rare and legendary designers worked while the uncommons were being
  measured, so they saw uncommon candidates rather than the final uncommon pool.
- **Two rare designs were deferred:** a payoff scaled by the opponents' contract size and a
  bid-gated hand change for nil bidders, because their hooks were not built in this pass.
- **The critic and the measurement run in parallel** in the uncommon and later passes; critic
  fixes that change an effect are re-measured in the next round instead of delaying the pass.
- **Copy instead of undo.** Search copies the small, allocation-free `Play` struct per
  iteration rather than undoing moves; there is still no allocation in the hot loop.
- **Tier 0 is flat Monte Carlo.** Six heuristic rollouts per legal move over common deals, a
  superset of "1.0's rollout policy plus a one-trick lookahead scored with sigils".
- **Opening swaps.** Tier 0 uses the heuristic; tiers 1 and 2 evaluate about six candidate
  give-sets per seat by rollouts from that seat's information, modeling the partner's give with
  the heuristic.
- **Nil margin.** Bidding rollouts charge a nil bid 550 margin points, tuned to meet the 0b nil
  targets.
- **Kernel rulings the GDD leaves open:** growth gains apply from the next round; lead triggers
  count leads by either partner, including a nil bidder; a payoff that fires during tricks
  reveals itself at once, other payoffs at the end of the round in which they first fire.
- **Global-scale knob.** The starting contract multiplier (P29) stays a kernel constant; the
  global scale uses one amount multiplier for all payoff categories.

## Measurement

- **Duplicate play swaps all luck, not just seats.** The pilot showed no variance reduction when
  each team kept its own offers. In the second run of a board each team now gets the other's
  setup, offer, exploration, and Opening streams; grants and gold perturbations stay with their
  team. Grant schedules depend only on the board seed and the team's earlier grants (coherent
  grants weight toward earlier grants, not shop purchases), so both runs apply the same
  treatments.
- **Grants hold their slot.** Once the shop offered the kept commons, teams filled their seven
  slots and the shop policy sold low-valued granted sigils, so many grants were held only briefly
  and their estimates lost most of their precision (controls' standard errors tripled). Granted
  sigils are now never sold to buy something else; only a later grant can displace one. The
  first uncommon measurement was discarded and re-run.
- **The W curve.** A logistic from final margin to win is degenerate on final margins, so W's
  scale is chosen in the pilot by maximizing the t-statistics of planted effects. Effects are
  converted to win-rate points with a fitted slope k of (2·win − 1) on u.
- **Intervals.** A cluster bootstrap of a ridge estimator under-covers (65% in a first pilot).
  Intervals use the geometric mean of the shrunk and nearly unpenalized bootstrap spreads (the
  empirical-Bayes posterior width); synthetic coverage is then 95%.
- **The factorization machine is held fixed in the bootstrap.**
- **Cards in the outcome model.** Card grants are measured by 12 rank-and-suit classes; the bonus
  for cards that owned payoffs reward is a single hand-set coefficient.
- **Run records** are JSONL in `runs/`; the pipeline writes Parquet tables of runs, held sigils,
  and rounds next to them. The per-sigil ledger stays in the JSONL.
- **Shop validation** rescoring plays one round on 12 deals at tier 0 per offer; it is too noisy
  to rank offers well and is reported as a known weakness.
- **Phase 0 review.** One reviewer checked all four builds together, after they were delivered.
  It found that owned cards broke stable dealing, that a grant could be skipped in one run of a
  board when the shop had already bought that sigil, that the tier-agreement disattenuation was
  invalid, that the A/A test compared raw betas instead of lifts, and that partner knowledge was
  wrong after a swap. All were fixed and the pilot re-run: dealing now gives each permutation
  position a fixed seat and passes only the overflow; each team's grants are planned at the start
  of the run and never offered to it; tier agreement is reported as the raw correlation on shared
  boards, which overstates it.
- **Nil success sits at the 0b bar.** Tier-1 nil success is 65–66% across seeds for nil margins
  of 550–650 points; it is recorded as met but borderline.

## Optimization rounds

- **Round 1 changed more than 15% of the pool.** Fourteen cuts (weak, samey, or frustrating
  pieces flagged by both the dashboard and the critic) plus eleven redesign targets. Cuts don't
  add new mechanics to measure, so the stability rule was relaxed for them.
- **The critic reviewed the round's proposals while the focused tournament ran**, so its
  verdicts were applied at the keep-or-revert step rather than before measurement.
- **Data from earlier rounds is not reused at half weight.** Tournaments are cheap enough that
  each round's fit uses only that round's fresh boards.
- **Density levers were not tested.** Commitment works lags its band (online by round 4 in about
  a quarter of committed runs), which [D7](../game-design.md#d7-offer-density) says should
  trigger lever arms; they are rule changes outside sigil design and are left as an open risk.
