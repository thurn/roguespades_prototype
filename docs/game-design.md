# Rogue Spades 2.0: game design

Rogue Spades 2.0 is a roguelike built on partnership Spades. Two teams of two
play eight rounds. Before each round, each team shops for **sigils** (up to
seven team-wide scoring and rule-changing pieces) and **cards**. A bought card
is dealt to its owner every round. The team with more points after round 8
wins.

The rules below are the base game. They are deliberately fixed: only the
levers in [Open for testing](#open-for-testing) are searched in simulation,
and the [design search plan](design-search-plan.md) says how. Simulation
metrics guide that search; they approximate fun and don't define it.

## Design pillars

1. **Real Spades underneath.** With no sigils and no cards, a round plays and
   scores as standard partnership Spades, bags included, at ten times the
   point scale.
2. **Direct scoring.** Most sigils say "you get points for doing X" in one of
   three Balatro-style categories, and builds come from stacking them.
3. **Simple pieces, emergent combos.** Each sigil does one thing. Archetypes
   are an internal design tool and are never named to players. Sigils share
   plain nouns such as [A]s, [♥]s, and the last trick, so players discover
   combinations themselves.
4. **Random offers, as in Balatro.** Sigil offers are purely random within
   rarity. Commitment comes from cards that are always for sale and from
   payoffs that fire at a real rate before their enablers arrive.
5. **Symmetric teams.** Humans and AIs follow identical rules and see
   identical information.

## 1. The table, the run, and victory

- Four seats form two teams, with partners opposite.
- A run is 8 rounds. A round is a shop, a deal, bidding, 13 tricks, scoring,
  and income.
- Scores belong to teams and can go negative. After round 8, the team with
  more points wins. Equal totals are a draw.
- Gold and sigils belong to teams. Each owned card belongs to one player.
- In single-player you sit South. Your AI partner sits North, and an AI team
  sits East–West. You make every shop decision for your team; North bids and
  plays toward your team's sigils.

## 2. The lifecycle of a round

1. **Shop.** Both teams shop at once ([§7](#7-shop-and-economy)). The first
   shop opens before round 1, with starting gold.
2. **Deal.** Each player receives their owned cards. The rest are shuffled and
   dealt to fill every hand to 13.
3. **Opening.** Resolve every `Opening:` effect. Random card changes finish
   here, so players bid on the resulting hands.
4. **Bid.** Starting left of the dealer, each player bids nil or 1–13. After
   the last bid, Hold payoffs count the resulting hands once.
5. **Play.** The player left of the dealer leads, and 13 tricks follow.
6. **Score.** Each team scores and takes its bags ([§4](#4-scoring)).
7. **Income.** Each team gains gold ([§7](#gold)).
8. **Next round.** The deal passes clockwise. After round 8 the run ends
   without a shop.

## 3. Spades rules

### Bidding

- Each player bids **nil** or a number from 1 to 13. There is no blind nil.
- A team's **contract** B is the sum of its non-nil bids. The team makes its
  contract if its non-nil bidders win at least B tricks.
- **Nil** promises to win no tricks. Tricks won by a nil bidder never count
  toward the partner's contract. If both partners bid nil, the team has no
  contract.

### Play

- Players must follow suit if able. A player who can't may play any card.
- [♠]s are trump. The highest [♠] wins; otherwise the highest card of the
  led suit wins.
- [♠]s can't be led until a [♠] has been played on another suit, unless
  the leader holds only [♠]s.
- **Effective cards.** Sigils can change a card's rank or suit ("Opening:
  Four cards your team holds become [K]s"). A changed card counts at its new
  identity everywhere: following suit, winning tricks, and sigil conditions.
  Ranks cap at [A].
- **Ties.** If a rank or suit change puts two identical cards in one trick,
  the first one played wins.

## 4. Scoring

### The formula

```
round score = ( ±10 × B                + when made, − when set
              ± contract points        + when made, − when set
              + nil score )            each made nil: +100 plus nil points; each failed nil: −100
            × (10 + Σ +contract multipliers)
            × Π ×contract multipliers

bags        = overtricks + tricks won by nil bidders; every 10 bags cost 1,000
```

The starting contract multiplier is **10**. With no sigils this is Spades at
ten times the point scale: a made contract of 7 scores 700, a set scores
−700, a made nil +1,000, and ten bags −1,000.

### Made contracts and sets

- **Made:** the team's non-nil bidders win at least B tricks and score
  +10 × B plus their contract points, times their multipliers.
- **Set:** the sign flips. The team scores −10 × B minus its contract points,
  times its multipliers. A strong build that overreaches loses as much as it
  would have won.
- A multiplier whose condition requires making the contract ("if your team
  makes its contract exactly") doesn't fire on a set. Every other multiplier
  that fired applies.
- Contract points earned on an overtrick count. Contract points triggered by a
  trick a nil bidder wins don't.
- **The nil score** counts whether the contract is made or set, and it is
  multiplied like the rest of the base.
- A team whose partners both bid nil has no contract. Its nil score is still
  multiplied, and any contract points are lost.

### Bags

- Each trick a team's non-nil bidders win beyond the contract is a **bag**.
  Every trick a nil bidder wins is also a bag.
- A team's bags carry over between rounds. Each time its total reaches 10, the
  team loses **1,000 points**, flat, and the total drops by 10.
- Bags sit outside the round formula: sigils and multipliers never change
  them.

### Three categories

| Category | Balatro analog | Example |
| --- | --- | --- |
| +Contract points | Chips | "+20 contract points when your team wins a trick with an [A]" |
| +Contract multiplier | +Mult | "+15 contract multiplier when your team wins the last trick of a round" |
| ×Contract multiplier | ×Mult | "×1.5 contract multiplier if your team makes its contract exactly" |

- **+Contract multipliers add up on top of 10.** One +15 sigil makes ×25;
  two make ×40.
- **×Contract multipliers compound** and apply last, so their order never
  matters. They can appear at any rarity.
- **Contract points accumulate during the round,** per event ("when your team
  wins a trick with an [A]") or once ("when your team bids 8 or more").
- **Nil points go to made nils.** "+50 nil points" adds 50 to each nil your
  team makes, so it counts even when the contract is set.

### Worked examples

| Situation | Calculation | Score | Bags |
| --- | --- | --- | --- |
| Bids 4 + 3, win 9, no sigils | 70 × 10 | 700 | +2 |
| Bids 4 + 3, win 6, no sigils | −70 × 10 | −700 | 0 |
| Bid 7, win 8; "[A]s: +20" fires twice; one +15 | (70 + 40) × 25 | 2,750 | +1 |
| Bid 7, win 6; "[A]s: +20" fires twice; one +15 | (−70 − 40) × 25 | −2,750 | 0 |
| Partner bids 5 and wins 6; you make nil; "+50 nil points"; one +15 | (50 + 100 + 50) × 25 | 5,000 | +1 |
| Partner bids 5 and wins 6; your nil takes a trick; same sigils | (50 − 100) × 25 | −1,250 | +2 |
| Late run: bid 9 and make it; 210 contract points; +35 from sigils; one ×1.5 | (90 + 210) × 45 × 1.5 | 20,250 | — |

### Keeping bids tense

At least a third of contract-point sigils and a quarter of multipliers scale
with or require the contract size: "+5 contract points for each trick in
your team's contract" and "+15 contract multiplier when your team bids 8 or
more". Bid High is a major archetype ([§8](#8-archetypes)).

## 5. Sigils

### Slots and ownership

- Sigils belong to the team, **up to 7 per team**. Buying one needs a free
  slot, so a full team sells first.
- A trick either partner wins counts as a team win, except tricks won by a nil
  bidder.
- A team is never offered a sigil it already owns. Both teams may own the same
  sigil.
- A sigil sells for half its price, rounded down to a multiple of 5.
- **Sigils never have activated abilities.** A sigil may offer a choice at a
  fixed moment, such as "Opening: Swap three cards with your partner".

### Visibility

- Every sigil is **public**: both teams see each other's sigils at all times.
- Partners share every sigil, the team's gold, and knowledge of each other's
  owned cards. Owned cards are hidden from opponents until they're played.
- Both teams' scores, bags, and gold are public.

### Terms

| Term | Meaning |
| --- | --- |
| **your team** | Both partners. Team wins count tricks either partner wins, except tricks a nil bidder wins |
| **contract points** | Points added to your team's contract: gained when it's made, lost when it's set |
| **contract multiplier** | Written `+15` when it adds up with others, or `×1.5` when it compounds. It multiplies your team's round score, made or set |
| **nil points** | Points added to each nil your team makes |
| **your team holds** | Cards in either partner's hand; Hold payoffs count once after bidding, after Opening |
| **Opening:** | Resolves after dealing and before the first bid |

### Payoffs and enablers

The three scoring categories are **payoffs**. The other sigils are
**enablers**: changes to the hand, to who leads, or to legal plays. An
enabler should be worth buying before the player owns a matching payoff.

The pool excludes:

- shop and economy modifiers (discounts, offer steering, extra offers,
  reroll bonuses, sell-value growth);
- effects that only rename cards for payoff checks, or that change how many
  tricks make a bid;
- information effects, such as revealing an opponent's cards;
- randomness once bidding begins. Opening effects finish before any bid;
- activated abilities.

**Opening resolution.** "X cards your team holds become Y" picks X distinct
cards uniformly from both partners' hands combined and applies the fixed
destination automatically; cards already matching Y can be picked. A rank
change changes only rank, and a suit change only suit; changes expire before
the next deal. "Raise every card" changes every card, capped at [A]. A swap
lets each partner choose up to three cards and exchanges equal counts at the
same time. Opening effects resolve in the kernel's fixed priority, each from
its own seeded stream.

### Simplicity rules

- Every sigil has one trigger or condition and one effect, with no riders, at
  every rarity. Higher rarity buys power, not more clauses.
- Count the whole quantity: "for each [♠] your team holds", never "beyond
  five". Balance with the payout, price, or rarity before adding a
  restriction.
- Use a threshold only when reaching it is the idea, such as winning three
  tricks by trumping. Prefer a named suit to a computed suit.
- Use familiar Spades language: "bids 8 or more", not "bids a high contract";
  "wins four tricks in a row", not a named counter.

### Rules text

Plain English, one short sentence, **benefit first, then condition**.
`Opening:` goes first on every effect that resolves before bidding. Rules
text is generated from the sigil's data.

| Kind | Example |
| --- | --- |
| Always on | +40 contract points |
| Trick event | +20 contract points when your team wins a trick with an [A] |
| Last trick | +15 contract multiplier when your team wins the last trick of a round |
| Result | ×1.5 contract multiplier if your team makes its contract exactly |
| Milestone | +15 contract multiplier when your team wins three tricks by trumping |
| Holdings | +20 contract points for each [A] your team holds |
| Bid | +15 contract multiplier when your team bids nil |
| Growth | This sigil gains ×5 contract multiplier every time your team makes a contract of 8 or more (currently ×0) |
| Card movement | Opening: Swap three cards with your partner |
| Rank change | Opening: Four cards your team holds become [K]s |
| Suit change | Opening: Four cards your team holds become [♥]s |

- **Lead with the amount:** "+20 contract points", "+15 contract multiplier",
  "×1.5 contract multiplier", "+50 nil points". Additive multipliers use
  `+15`, never `+15×`.
- **Always name "your team"** for bids, holdings, leads, wins, and results.
- **Use "when" for events and "if" for results.** A trick event pays on every
  matching trick. A milestone pays when its count is first reached. Numeric
  thresholds mean at least that many unless the text says "exactly". A
  nil-bid trigger fires for each partner who bids nil.
- **Round effects reset; growth persists.** Growth keeps its accumulated
  benefit for the run; "currently ×0" is the stored extra, added to the
  base ×10.
- **Bracket ranks and suits:** `[A]`, `[10]`, `[♣]`; pluralize outside the
  brackets (`[K]s`); write a specific card as `[4♣]`.
- Lead triggers fire on the lead, whether or not the trick is won.

### Rarity and price

| Rarity | Offer odds | Price | Pool size (soft target) |
| --- | --- | --- | --- |
| Common | 69% | 50 | About 60 |
| Uncommon | 25% | 75 | About 60 |
| Rare | 5% | 100 | About 20 |
| Legendary | 1% | 150 | About 5 |

The pool lives in `data/sigils/`, the [registry](sigils/registry.md), and the
viewer (`npm --prefix viewer run dev`, then `/sigils`).

## 6. Cards

- The deck is the standard 52 cards, each existing once.
- Buying a card means it is **dealt to its owner every round**.
- Each player owns **at most 8 cards**, so every hand keeps at least 5 random
  cards. Selling a card makes room.
- Every card offer is tagged with the player who will own it. In
  single-player, your team's cards always go to you; each AI team has one
  card-owning seat, chosen at random for the run.
- Card offers are drawn uniformly from cards nobody owns and nobody else is
  being offered, so two teams never compete for the same offer.
- A card sells for half its price, rounded down to a multiple of 5, and
  returns to the unowned pool.

| Rank | [A] | [K] | [Q] | [J] | [10] | [2]–[9] |
| --- | --- | --- | --- | --- | --- | --- |
| [♣] [♦] [♥] | 80 | 60 | 45 | 35 | 25 | 15 |
| [♠] | 120 | 90 | 70 | 50 | 40 | 25 |

## 7. Shop and economy

### Shop

- Each team has one shop per round, shared by both partners, with **3 sigil
  offers and 3 card offers**.
- Purchases are unlimited, bounded by gold, 7 sigil slots, and 8 owned cards
  per player. A bought offer leaves an empty slot until the next reroll.
- A **reroll** refreshes all six offers for 50 gold, plus 10 for each further
  reroll in the same shop. The cost resets at the next shop.
- Sigil offers are **purely random within rarity**, drawn with replacement,
  and never include a sigil the team owns.
- Sigils and cards can be sold at any shop for half price.

### Gold

- Each team starts with **150 gold**.
- After each round, a team gains:
  - **100 base income**;
  - **10 gold per trick in its contract**, if it made the contract;
  - **100 gold per made nil**.
- There is no interest.
- A typical round earns about 165 gold, so a run has about 1,300 to spend.

## 8. Archetypes

An archetype is a plan a team can build a run around. Archetypes are a design
and simulation tool only: the game never names them, tags sigils with them, or
steers offers by them.

| Archetype | Weight | Plan |
| --- | --- | --- |
| Suits [♣] [♦] [♥] | Major family | Own one side suit, its honors or its length; hold, lead, and win with it |
| Spades | Major | Own long or high [♠]s; win and trump with them |
| Ranks | Major | Collect one rank, mostly [A]s; hold, lead, and win with it |
| Bid High | Major | Bid 8 or more and make the contract |
| Nil | Major | Bid nil and make it |
| Low Cards | Minor | Win with [2]s through [10]s |
| Rainbow | Minor | Win tricks with all four suits |
| Streaks | Minor | Win tricks in a row |
| Exact | Minor | Make exactly your contract, taking no bags |

Each archetype rewards one idea through several triggers, so its sigils stack
instead of competing:

| Trigger | What it checks | Example |
| --- | --- | --- |
| Hold | Both partners' hands after Opening and bidding | +20 contract points for each [A] your team holds |
| Lead | Cards your team leads | +15 contract points when your team leads an [A] |
| Win | The card that wins a trick for your team | +20 contract points when your team wins a trick with an [A] |
| Trump | Winning a trick by trumping | +20 contract points when your team wins a trick by trumping |
| Bid | The contract's size, or a nil bid | +15 contract multiplier when your team bids 8 or more |
| Make | The round's result | +15 contract multiplier if your team makes its contract exactly |

Generic sigils ("+40 contract points", "+10 contract multiplier") support
every build while a plan comes together.

## 9. Multiplayer

The first prototype is single-player. Every rule is symmetric, so multiplayer
needs only these additions:

- Two humans can partner against an AI team, or four humans can play two
  against two.
- Partners share one shop and one gold pool; either can buy, sell, or reroll,
  and the first action wins. The team's shop closes when both press Done.
- Card offers are tagged to one partner at random.

## 10. Metrics

Simulated AI players measure the game. The metrics are a useful but rough
approximation of fun: they can't see elegance, feel, or clarity, and "no
measured difference" never means "costs nothing". They inform the designer's
judgment and never replace it.

### Fun score

Each sub-metric scores 1 inside its band and falls linearly to 0 at its
tolerance (`analysis/rsa/funscore.py`). A family scores the mean of its
sub-metrics, and the fun score is the weighted sum, from 0 to 100.

| Family | Weight | Goal | Sub-metrics and bands |
| --- | --- | --- | --- |
| Close and live | 20 | Close and meaningful at every stage | Median final margin at most 45% of the winner's score; the team trailing after round 5 wins at least 25%; rounds 1–4 hold at least 20% of all points |
| Commitment works | 15 | I can "do the thing" if I commit | Builds committed by shop 2 are online by round 4 in at least 60% of runs; committed teams win at least as often as flexible ones |
| Archetypes viable | 15 | Every archetype is viable, nothing is forced | Each archetype's committed win rate is 40–60%; no archetype appears in more than 25% of winning flexible builds |
| Synergy and combos | 15 | Pieces multiply each other | Same-archetype pairs beat the sum of their parts by at least 25% in final margin; at least 15 strong pairs, 5 of them crossing archetypes |
| Skill and bidding | 10 | Good play wins, and bidding stays tense | Tier 2 beats tier 0 in at least 65% of paired runs; set rate 10–25% |
| Simplicity | 25 | Each piece is easy to read and track | Mean sigil complexity C: full credit at 3 or less, zero at 7 or more |

- **Online** means holding two of the archetype's payoffs and one of its
  enablers; three owned cards its payoffs reward count as an enabler.
- **Flexible** teams buy by plain value; **committed** teams add a bonus for
  one archetype's sigils and the cards they reward.

### Sigil metrics

These guide pool work; a sigil far outside them is a candidate for
retuning, reshaping, or cutting.

| Metric | Measured as | Guide |
| --- | --- | --- |
| Choices matter | Win-rate lift over a same-rarity flat-points control | Above 0 |
| Payoffs are doable | Trigger rate, base and committed | At least 10% of rounds base, 60% committed |
| No invalidation | An archetype's win-rate drop when opponents hold a sigil | At most 15 points |
| Power ceiling | Lift; win rate of the strongest pairs and builds | Lift at most 12 points; no pair above 65% |

### Simplicity rubric

Complexity C is computed mechanically from each sigil's generated rules text
and effect data, never entered by hand. Every token the player must read and
track costs something:

| Burden | Cost |
| --- | --- |
| Each number (payout, count, threshold, ordinal, displayed total) | +1 per occurrence |
| Each rank literal (`[A]`, `[2]`) | +1 per occurrence |
| Each suit literal (`[♠]`, `[♦]`) | +1 per occurrence |
| Each condition clause or atomic check ("when your team wins a trick", "with an [A]", "if … exactly", each part of an AND or OR) | +1 |
| Arithmetic on the rewarded quantity ("beyond five") | +2 |
| Computed selector ("your longest suit") | +1 |
| Tracked state (milestone count, streak, growth counter) | +1 |
| Jargon: any term beyond base vocabulary ("streak", "high contract", "low cards") | +2, or +4 at common |

Base vocabulary is free: trick, lead, follow, trump, bid, nil, contract, your
team, hold, `Opening:`, and the three scoring labels. Hidden structure is expanded before
scoring: "low cards" pays for its rank endpoints and its term.

| Candidate | C |
| --- | --- |
| +40 contract points | 1 |
| ×1.5 contract multiplier if your team makes its contract exactly | 2 |
| +5 contract points for each [♠] your team holds | 3 |
| +20 contract points when your team wins a trick with an [A] | 4 |
| Opening: Four cards your team holds become [2]s | 3 |

### Diagnostics

Tracked, not weighted: late overtricks and bags per run, gold unspent at the
end of a run, purchases and rerolls per shop, and AI play quality (wasted
overtakes, missed nil covers, nil suicides).

## 11. Simulation

- **One rules kernel,** native Rust (`sim/`), runs every simulation and AI
  search; the browser game runs the same kernel compiled to wasm
  (`sim/web`). Runs are deterministic from named seeded streams.
- **Declarative sigils.** Every sigil is data (trigger, filter, effect,
  amounts, rarity) run by one interpreter; rules text is generated from it.
  Rule benders name one of a small set of rule hooks written in code.
- **Rules file.** Every lever in [Open for testing](#open-for-testing) is a
  value in `data/rules.json`, read by both the simulator and the game.
- **No AI component has sigil-specific knowledge.** Play and bidding search
  the real rules through the kernel. The shop values offers with a model
  fitted to randomized simulation.
- **Information.** AI seats see exactly what a human in their seat would see,
  and searches sample hidden hands consistent with voids, bids, and known
  cards.
- **Duplicate boards.** Every seed is played twice with the teams swapping
  seats; the pair is the unit of analysis.

| Tier | Play | Bidding | Used for |
| --- | --- | --- | --- |
| 0 | Heuristic rollout policy with one-trick lookahead | Trick estimate, then expected value | Screening and sweeps |
| 1 | Information-set MCTS, about 200 iterations over 32 deals | Expected value over 40 rollouts | Comparisons |
| 2 | Information-set MCTS, 1,500 iterations over the best 64 of 512 deals | Expected value over 160 rollouts | Confirmation and the shipped AI |

Value means win probability over the rest of the run, through a fitted curve
from score margin and rounds left.

## Open for testing

Everything not in this table is fixed. The [design search
plan](design-search-plan.md) searches these levers; a change outside the
table needs the designer's approval first.

| Lever | Base | Values to test |
| --- | --- | --- |
| Set rule | Symmetric: a set loses its contract points, and multipliers apply | Flat: a set scores −100 × B, untouched by sigils |
| Starting contract multiplier | ×10 | ×20 |
| Sigil offer draws | With replacement, never one the team owns | Without replacement per team per run; shared between teams (never one either team owns) |
| Rounds per run | 8 | 6–10 |
| Owned cards per player | 8 | 4, 6 |
| Sigil slots | 7 | 5–9 |
| Sigil offers per shop | 3 | 2–5 |
| Card offers per shop | 3 | 2–5 |
| Purchases per shop | Unlimited | 1, 2 |
| Reroll cost | 50, +10 per further reroll | Base 25–100; step 0–25 |
| Sell value | Half price | 25–75% |
| Starting gold | 150 | 100, 200, 250 |
| Base income | 100 | 10, 25, 50 |
| Rarity odds | 69 / 25 / 5 / 1% | Each tier halved or doubled |
| Sigil prices | 50 / 75 / 100 / 150 | ±50% |
| Card prices | The [§6](#6-cards) table | Retuned toward equal value per gold |
| Sigil pool | Master's pool | Amounts (+points, +mult, ×mult); simpler same-slot replacements; cuts; new sigils within the grammar and §5 rules; archetype roster and suit coverage through those changes |

**Fixed, not to be searched:** Spades scoring with bags (10 per contract
trick, −1,000 per 10 bags), the nil score of ±100 in the base, contract gold
(10 per trick), nil gold (100), no interest, no engravings or synthetic
cards, public sigils, no blind nil, the three scoring categories and their
order, the bid-tension share of the pool, the pool's soft size targets, no
catch-up or rubber-banding rules, and no activated abilities.
