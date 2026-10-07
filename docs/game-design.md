# Rogue Spades 2.0: game design

Rogue Spades 2.0 is a roguelike built on partnership Spades. Two teams of two
play eight rounds. Before each round, each team shops for **sigils** (up to
seven team-wide scoring and rule-changing pieces) and **cards**. A bought card
is dealt to its owner every round, and later cards carry **engravings**. The
team with more points after round 8 wins.

One rule outranks every other: **evidence leads**. Strong AI players play
thousands of simulated runs, and the design improves over rounds of iteration
toward the fun metrics in [§12](#12-metrics-what-fun-means), weighed with the
designer's sense of elegance. The metrics are targets to optimize, not
pass/fail bars.

This document records the starting design from the design interview of
2026-10-05. [Appendix A](#appendix-a-decision-records) holds a record for each
decision, with the alternatives considered.
[Appendix B](#appendix-b-parameter-register) lists every number to test, and
[Appendix C](#appendix-c-unexamined-assumptions) lists the assumptions made
without a dedicated question. It builds on two predecessors:

- **Rogue Spades 1.0** (`~/rsp`): its
  [rules](../../rsp/docs/game-overview.md),
  [archetypes](../../rsp/docs/archetypes.md),
  [AI plan](../../rsp/docs/automation-plan.md), and TypeScript code, much of
  which carries over ([§13](#what-carries-over-from-10)).
- **Bridge of Rogues** (`~/rbp`): its [design](../../rbp/docs/game-design.md)
  of the shop, owned cards, engravings, payoffs and enablers, the pool
  skeleton, and sigil validation.

## Status: provisional

This is version 0 of the design, written before any simulation has run.
Everything in it is a hypothesis for the first round of testing:

- **Rules and structure,** such as scoring, owned cards, engravings, and the
  shop, are starting choices. Each has a decision record in
  [Appendix A](#appendix-a-decision-records) that lists the alternatives
  considered and the experiment that will choose between them.
- **Numbers,** such as shop sizes, prices, gold, slots, caps, odds, and sigil
  budgets, are starting values. [Appendix B](#appendix-b-parameter-register)
  lists each one with a range to test.
- **Sigils** are illustrative. None has evidence yet, and most will change or
  be cut.
- **Metric target bands and weights** are placeholders until the harness
  runs.

A decision is settled only when its record shows the alternatives and the
numbers that chose between them. Every sigil in the game has a record of the
evidence and reasoning for its inclusion
([§16](#16-evidence-and-design-records)).

## Starting design at a glance

Every row is a starting hypothesis.

| Topic | Starting hypothesis |
| --- | --- |
| Golden Rule | Rules, sigils, and numbers are iterated toward the metrics ([§12](#12-metrics-what-fun-means)); simulation evidence and designer judgment decide what stays |
| Run | 8 rounds; each round is a shop, then a deal of Spades |
| Spades | Partners' bids add up to a team contract; nil; [♠]s must be broken; no blind nil; no bags |
| Scoring | (10 × contract + contract points + nil score) × (10 + Σ +multipliers) × Π ×multipliers; overtricks score nothing |
| Failure | A set team loses its contract points and scores (−10 × contract + nil score) × its multipliers |
| Victory | Most points after round 8; equal totals draw |
| Sigils | Up to 7 per team, shared by both partners; hidden from opponents until they first trigger |
| Cards | A bought card is dealt to its owner every round; at most 8 owned per player; later offers include synthetic [J]s, [Q]s, [K]s, and [A]s that replace lower cards of the same suit |
| Engravings | Four fixed types on card offers from shop 3, one of them Synthetic; never bought alone, and never change a card's suit or rank |
| Shop | 3 sigil and 3 card offers per team; unlimited purchases; purely random offers within rarity |
| Income | 100 base gold + 10 per trick in a made contract + interest |
| Archetypes | Majors: Suits ([♣] [♦] [♥]), Spades, Ranks, Bid High, Nil. Minors: Low Cards, Rainbow, Streaks, Exact. Never named in the game |
| Simulation | Declarative sigils on a native Rust simulator; three AI tiers; duplicate grant tournaments measure every sigil at once ([§13](#experiments)) |
| Prototype | Single-player: you and an AI partner against an AI team |

## Design pillars

1. **Evidence leads, and it is written down.** Design questions are answered
   by experiments first, and by designer judgment for what metrics can't see,
   such as elegance. The design improves over rounds of iteration rather than
   clearing fixed bars. Every decision records the
   alternatives considered and the numbers that chose between them, and every
   sigil records the evidence for its inclusion. This document defines fun as
   measurable metrics, and the game is built so the AI can play, bid, buy, and
   measure a sigil the moment it is written.
2. **Real Spades underneath.** With no sigils and no cards, a round plays and
   scores as partnership Spades without bags, at ten times the point scale.
3. **Direct scoring.** Most sigils say "you get points for doing X" in one of
   three Balatro-style categories, and builds come from stacking them. Every
   payoff comes with **enablers** that make X happen more often.
4. **Simple pieces, emergent combos.** Each sigil does one thing. Archetypes
   are an internal design tool and are never named to players. Sigils share
   plain nouns such as [A]s, [♥]s, and the last trick, so players discover
   combinations themselves.
5. **Commitment without railroading.** Shop offers are purely random within
   rarity, as in Balatro. Commitment works because cards are always for sale,
   and because every payoff fires at a real rate before its enablers arrive.
   Density levers wait until the metrics call for them.
6. **Symmetric teams.** Humans and AIs follow identical rules and see
   identical information.

## 1. The table, the run, and victory

> **Status:** provisional. The run length is parameter P1 in
> [Appendix B](#appendix-b-parameter-register).

- Four seats form two teams, with partners opposite.
- A run is 8 rounds. A round is a shop, a deal, bidding, 13 tricks, scoring,
  and income.
- Scores belong to teams and can go negative. After round 8, the team with
  more points wins. Equal totals are a draw.
- Gold and sigils belong to teams. Each owned card belongs to one player.
- In single-player you sit South. Your AI partner sits North, and an AI team
  sits East–West.
  - You make every shop decision for your team.
  - North never shops, but bids and plays toward your team's sigils.
- Multiplayer is covered in [§11](#11-multiplayer).

## 2. The lifecycle of a round

> **Status:** provisional. The shop and income steps follow
> [D7](#d7-offer-density) and [D9](#d9-income).

1. **Shop.** Both teams shop at once ([§8](#8-shops-and-economy)). The first
   shop opens before round 1, with starting gold.
2. **Deal.** Each player receives their owned cards. The rest are shuffled and
   dealt to fill every hand to 13 ([§6](#6-cards)).
3. **Opening.** Resolve every `Opening:` effect after the deal and before
   any bids. Random card changes finish here, so players bid on the resulting
   hands.
4. **Bid.** Starting left of the dealer, each player bids nil or 1–13. After
   the last bid, Hold payoffs count the resulting hands once.
5. **Play.** The player left of the dealer leads, and 13 tricks follow.
6. **Score.** Each team scores ([§4](#4-scoring)).
7. **Income.** Each team gains interest, then base and contract gold.
8. **Next round.** The deal passes clockwise. After round 8 the run ends
   without a shop.

## 3. Spades rules

> **Status:** provisional. Overtricks are [D1](#d1-which-tricks-score). Blind
> nil, ties, and the other base rules are assumptions in
> [Appendix C](#appendix-c-unexamined-assumptions).

### Bidding

- Each player bids **nil** or a number from 1 to 13.
- A team's **contract** B is the sum of its non-nil bids. The team makes its
  contract if its non-nil bidders win at least B tricks.
- **Nil** promises to win no tricks. Tricks won by a nil bidder never count
  toward the partner's contract. If both partners bid nil, the team has no
  contract.
- **Blind nil** is not part of the rules or the current sigil pool.

### Play

- Players must follow suit if able. A player who can't may play any card.
- [♠]s are trump. The highest [♠] wins; otherwise the highest card of the
  led suit wins.
- [♠]s can't be led until a [♠] has been played on another suit, unless
  the leader holds only [♠]s.
- **Effective cards.** Rule benders can change a card's rank or suit, as in
  "Opening: Four cards your team holds become [K]s". A changed card
  counts at its new identity everywhere: following suit, winning tricks, and sigil
  conditions. Ranks cap at [A].
- **Ties.** [Synthetic cards](#synthetic-cards) and rule benders can put two
  cards of the same suit and rank in one trick. Between equal cards, the first
  one played wins.

## 4. Scoring

> **Status:** provisional. The formula comes from [D1](#d1-which-tricks-score),
> [D2](#d2-nil-scoring), [D3](#d3-scoring-categories), and
> [D8](#d8-bid-tension), the par curve from [D4](#d4-score-growth), and the
> multiplier scale from [D25](#d25-multiplier-scale). Every number is a
> parameter in [Appendix B](#appendix-b-parameter-register).

### The formula

```
round score = ( 10 × B               base contract
              + contract points      added by sigils and engravings this round
              + nil score )          each made nil: +100 plus nil points; each failed nil: −100
            × (10 + Σ +contract multipliers)
            × Π ×contract multipliers
```

The starting contract multiplier is **10**. With no sigils this is Spades
without bags at ten times the point scale: a made contract of 7 scores 700,
and a set scores −700. Nil stays ±100 inside the parentheses, so an
unmodified made or failed nil contributes ±1,000 to the final score.

Contract-point amounts stay on their existing scale; gold, prices, and income
do not change. This deliberately changes displayed scores rather than dividing
them back down. Larger +mult values provide finer tuning without adding
conditions ([D25](#d25-multiplier-scale)).

### Three categories

| Category | Balatro analog | Example (not final) |
| --- | --- | --- |
| +Contract points | Chips | "+20 contract points when your team wins a trick with an [A]" |
| +Contract multiplier | +Mult | "+15 contract multiplier when your team wins the last trick of a round" |
| ×Contract multiplier | ×Mult | "×1.5 contract multiplier if your team makes its contract exactly" |

- **+Contract multipliers add up on top of 10.** One +15 sigil makes ×25;
  two make ×40.
- **×Contract multipliers compound** and apply last, so their order never
  matters. They can appear at any rarity, as in Balatro, and the
  [skeleton](#10-sigil-pool-skeleton) sets how many sit at each one.
- **Contract points accumulate during the round.** They come per event
  ("when your team wins a trick with an [A]") or once ("when your team bids 8 or more").
- **Nil points go to made nils instead.** "+50 nil points" adds 50 to
  each nil your team makes, so it counts even when the contract is set.

### Made contracts, sets, and nils

- **Made:** the team's non-nil bidders win at least B tricks, and the formula
  above applies.
  - **Overtricks score nothing, and there are no bags.**
  - Contract points earned on an overtrick still count.
- **Set:** the team loses all its contract points, and its base becomes
  −10 × B.
  - Multipliers still apply, except those whose condition requires making the
    contract.
  - So multipliers cut both ways: a strong build that overreaches loses more.
- **The nil score** counts whether the contract is made or set.
- Contract points triggered by a trick that a nil bidder wins don't count.
- A team whose partners both bid nil has no contract. Its nil score is still
  multiplied, and any contract points are lost.

### Worked examples

| Situation | Calculation | Score |
| --- | --- | --- |
| Bids 4 + 3, win 9, no sigils | 70 × 10 | 700 |
| Bids 4 + 3, win 6, no sigils | −70 × 10 | −700 |
| Bid 7, win 8; "[A]s: +20" fires twice; one +15 holds | (70 + 40) × 25 | 2,750 |
| Same, but win 6 | −70 × 25 | −1,750 |
| Partner bids 5 and wins 6; you make nil; "+50 nil points"; one +15 | (50 + 150) × 25 | 5,000 |
| Same, but your nil fails | (50 − 100) × 25 | −1,250 |
| Late run: bid 9 and make it; 210 contract points; +35 from sigils; one ×1.5 | (90 + 210) × 45 × 1.5 | 20,250 |
| Same, set, with every multiplier's condition still met | −90 × 45 × 1.5 | −6,075 |

### Keeping bids tense

A set loses every contract point, while each extra bid trick adds only 10
before multipliers. Left alone, large point totals would push late-run teams to
bid safe and drain the tension from the bidding. The pool is built against
this:

- At least a third of contract-point sigils and a quarter of multipliers scale
  with or require the contract size: "+5 contract points for each trick in
  your team's contract" and "+15 contract multiplier when your team bids 8 or more".
- Bid High is a major archetype ([§9](#bid-high)).
- The Skill and bidding family of the fun score watches late-run overtricks and
  set rates ([§12](#fun-score)).

### Par curve

Par is the typical round score of a reasonably built team, averaged over makes
and sets. It grows about ×1.39 per round, ×10 over the run:

| Round | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Par | 600 | 850 | 1,150 | 1,600 | 2,250 | 3,100 | 4,300 | 6,000 |

- Rounds 1–4 make up about a fifth of a run's points. Early play still counts,
  while builds still visibly grow.
- Round 1 is nearly plain Spades at the new point scale.
- These targets are the previous curve multiplied by ten, not a prediction
  that the revised sigils preserve it. Their larger relative rewards must be
  swept against this curve and the early-round share in simulation.
- A 7-sigil build can compound well past these targets, so the budgets
  ([§10](#budgets)) remain provisional.

## 5. Sigils

> **Status:** provisional. Visibility is [D10](#d10-sigil-visibility), the
> effect families [D13](#d13-effect-families), the enabler redesign
> [D26](#d26-enablers-worth-taking-before-payoffs), Opening
> [D27](#d27-opening-and-random-card-changes), and rules text
> [D22](#d22-rules-text). Slots, rarity odds, and prices are
> [parameters](#appendix-b-parameter-register) P3, P14, and P15.

### Slots and ownership

- Sigils belong to the team, **up to 7 per team**. Buying one needs a free
  slot, so a full team sells first.
- Sigil text explicitly says "your team". A trick either partner wins counts
  as a team win, except tricks won by a nil bidder.
- A team is never offered a sigil it already owns. Both teams may own the same
  sigil.
- A sigil sells for half its price, rounded down to a multiple of 5.
- Sigils never have activated abilities. A sigil may offer a choice at a fixed
  moment, such as "Opening: Swap three cards with your partner".

### Terms

These terms extend the standard Spades vocabulary, following the terms table of
1.0's [rules-text guide](../../rsp/docs/rules-text.md#terms).

| Term | Meaning |
| --- | --- |
| **your team** | Both partners. Team wins count tricks either partner wins, except tricks a nil bidder wins |
| **contract points** | Points added to your team's contract, paid only if the contract is made |
| **contract multiplier** | Written `+15` when it adds up with others, or `×1.5` when it compounds. It multiplies your team's round score, made or set |
| **nil points** | Points added to each nil your team makes. A failed nil still costs 100 before multipliers |
| **your team holds** | Cards currently in either partner's hand; Hold payoffs take their snapshot after bidding, once Opening has resolved |
| **Opening:** | Resolve after dealing and before the first bid; the keyword carries the new-term complexity cost |

### Visibility

- Partners share every sigil, the team's gold, and knowledge of each other's
  owned cards.
- An opponent's sigil is hidden until the first time it changes a score, a
  legal play, or a trick's winner. It then stays revealed for the rest of the
  run. Hidden sigils work normally.
- Both teams' scores and gold are public. Owned cards are hidden from opponents
  until they're played.

### Categories and families

The three scoring categories are **payoffs**. The other sigils are **enablers**:
substantial changes to the hand or to how tricks can be played. They must be
worth taking before the player owns any matching payoff. Supporting several
payoffs is a consequence of a useful effect, not its only purpose.

The shared catalog in [§9](#shared-enabler-candidates) covers hand reshaping,
control of the lead, and freedom to play cards. Simple hand changes can be
common; rarity follows power and complexity, not an automatic rule-bender tax.

The current pool excludes:

- Shop and economy modifiers: discounts, offer steering, extra offers,
  reroll or interest bonuses, and sell-value growth. The base shop still sells
  cards, but sigils do not make its small adjustments their whole benefit.
- Effects that only rename cards for payoff checks, and exceptions that alter
  the number of tricks needed to make a bid. Enablers change actual play.
- Information effects such as revealing an opponent's cards.
- Randomness once bidding begins. Random deal and Opening effects are input
  randomness: they finish before players commit their bids. No later random
  changes are allowed.
- Activated abilities. Choices happen only at a specified resolution window
  or during the ordinary choice of a legal card.

### Simplicity rules

- Every sigil has one trigger or condition and one effect, with no riders, at
  every rarity. Higher rarity buys power, not more clauses.
- Commons name one thing and give one benefit: "+20 contract points when your
  team wins a trick with an [A]".
- Count the whole quantity: "for each [♠] your team holds", never "beyond five".
  Balance with the payout, price, or rarity before adding a restriction.
- Use a threshold only when reaching it is the idea, such as winning three
  tricks by trumping. Prefer a named suit to a computed suit and a team result
  to separate conditions for each partner.
- Use familiar Spades language. Say "bids 8 or more", not "bids a high
  contract", and "wins four tricks in a row", not a named counter. New
  vocabulary incurs a complexity cost, doubled at common.
- Rules text is generated from the sigil's data
  ([§13](#declarative-sigils)). The text lint checks these conventions, while
  the [simplicity rubric](#simplicity-rubric) penalizes mechanical complexity
  in the fun score. Simulation sweeps amounts; a candidate that reaches its
  targets only with extra bookkeeping is usually reshaped or replaced. No sigil
  has an activated ability.

### Rules text

Use plain English, one short sentence, **benefit first, then condition**.
`Opening:` is the timing-prefix exception: put it first on every effect that
resolves before bidding. These templates replace 1.0's rules-text guide.

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
| Rank change | Opening: Four cards your team holds become [2]s |
| Rank change | Opening: Four cards your team holds become [K]s |
| Suit change | Opening: Four cards your team holds become [♥]s |
| Rank increase | Opening: Raise every card your team holds by two ranks |
| Engraving | +5 contract multiplier when this card wins a trick |
| Synthetic | Replaces [4♣] |

- **Lead with the amount.** Use "+20 contract points", "+15 contract
  multiplier", "×1.5 contract multiplier", or "+50 nil points", without
  "add" or "gain". Additive multipliers use `+15`, never `+15×`.
- **Always name "your team"** for bids, holdings, leads, wins, and results;
  never use "you" as shorthand for the team. Possessives use "your team's".
  Direct choices may say "your partner", and engravings say "this card".
- **Use `Opening:` consistently.** Every before-bidding effect starts with
  it; omit the redundant words "before bidding". All current hand-change
  enablers use this window, including effects previously placed after bidding.
- **Use "X cards your team holds become Y"** for a fixed-count transformation.
  It means a random selection, not a player choice; the destination rank or
  suit is fixed by Y. Do not add "random" to every card. "Every card" affects
  the whole hand and needs no random selection.
- **Omit obvious scope and timing.** No "each round", "in a round", "after
  bidding" on holdings, "in your shop", or redundant "once". Keep distinctive
  play timing such as "the last trick of a round".
- **Bracket ranks and suits.** Use `[A]`, `[K]`, `[Q]`, `[J]`, `[10]` through
  `[2]`, and `[♣]`, `[♦]`, `[♥]`, `[♠]`. Pluralize outside the brackets, as
  `[2]s` and `[K]s`; write a specific card as `[4♣]`. Counts such as "Four
  cards" and changes such as "two ranks" are not card identities and stay
  unbracketed. Name `[♦]` rather than "your longest suit". Brackets are a
  display convention, not a way to change complexity costs.
- **Use "when" for events and "if" for results.** A trick event pays on
  every matching trick. A milestone pays when its count is first reached;
  three trump wins do not pay again at six. Numeric thresholds mean at least
  that many unless the text says "exactly". Holdings are counted once after
  bidding, after Opening has resolved, and result conditions once at scoring.
  A nil-bid trigger fires for each partner who bids nil. These defaults live
  in the rules, not on every sigil.
- **Round effects reset; growth persists.** Ordinary points, multipliers,
  and event counters reset between rounds. Growth says "This sigil gains …
  every time … (currently …)" and keeps its accumulated benefit for the run.
  In the growth example, ×0 is the stored extra multiplier, added to the
  base ×10: after two gains the sigil contributes +10, for ×20 before
  other sigils. It never multiplies the score by zero; a standalone
  "×1.5 contract multiplier" still compounds.
- **Only engravings say "this card".** Sigils are never engraved. Lead
  triggers fire on the lead whether or not the trick is won.

### Rarity and price

| Rarity | Offer odds | Price | Pool size (soft target, [D30](#d30-pool-shape)) |
| --- | --- | --- | --- |
| Common | 69% | 50 | About 60 |
| Uncommon | 25% | 75 | About 60 |
| Rare | 5% | 100 | About 20 |
| Legendary | 1% | 150 | About 5 |

### Timing

- Opening effects resolve after the deal and before the first bid. All
  current hand changes happen there; Hold payoffs count once after bidding.
- Contract points and multipliers accumulate as events happen and apply at
  scoring. Sums and products make sigil order irrelevant.
- Conflicting rule benders resolve by a fixed priority, defined once in the
  rules kernel.

## 6. Cards

> **Status:** provisional. Ownership is [D5](#d5-owned-cards). The card cap,
> the number of card offers, and every card price are
> [parameters](#appendix-b-parameter-register) P2, P5, and P16.

- The deck is the standard 52 cards. Each real card exists once;
  [synthetic cards](#synthetic-cards) are extra copies that replace other
  cards.
- Buying a card means it is **dealt to its owner every round**.
- Each player owns **at most 8 cards**, so every hand keeps at least 5 random
  cards. Selling a card makes room. The cap is a tuning lever.
- Every card offer is tagged with the player who will own it, and the tag
  can't change.
  - In single-player, your team's cards always go to you. Each AI team has one
    card-owning seat, chosen at random for the run.
  - On a team of two humans, each offer is tagged to one partner at random.
- Real card offers are drawn uniformly from cards nobody owns and nobody else
  is being offered, so two teams never compete for the same offer.
- Owned cards are known to the partner, since the team shops together. They're
  hidden from opponents until played.
- A card sells for half its price, rounded down to a multiple of 5. It returns
  to the unowned pool and loses its engraving.

### Prices

Stage 1 ([§14](#14-staging)) sets card prices from each card's simulated value
per gold. Starting values:

| Rank | [A] | [K] | [Q] | [J] | [10] | [2]–[9] |
| --- | --- | --- | --- | --- | --- | --- |
| [♣] [♦] [♥] | 80 | 60 | 45 | 35 | 25 | 15 |
| [♠] | 120 | 90 | 70 | 50 | 40 | 25 |

An engraved card costs its price plus the engraving's premium.

### Dealing

1. Each player receives all of their owned cards, synthetic ones included.
2. Cards replaced by owned synthetic cards are set aside for the round.
3. All other cards are shuffled and dealt to fill every hand to 13.

## 7. Engravings

> **Status:** provisional. The catalog is [D6](#d6-engraving-catalog) and
> [D23](#d23-identity-engravings). The engraved share of card offers, the
> premiums, and the cards a synthetic copy can replace are
> [parameters](#appendix-b-parameter-register) P17–P19.

An engraving is a fixed modifier printed on a card offer. Engravings can't be
bought alone, take no sigil slot, and stay with the card while it's owned.

- Card offers carry engravings from **shop 3** on: a quarter of card offers in
  shop 3, rising by 10 points per shop to three quarters in shop 8. Card
  offers therefore grow more powerful as the run goes on.
- Each card holds at most one engraving.
- An engraving works for the card's current holder, including after a partner
  swap; ownership and the next round's guaranteed deal do not change.
- Engravings are hidden from opponents until the card is played.

Engravings never change a card's suit or rank. When an offer should be a
different card, the shop offers that card instead, as a synthetic copy. The
catalog has four types in two families:

| Engraving | Family | Rules text | Premium |
| --- | --- | --- | --- |
| Bonus | Scoring | +20 contract points when this card wins a trick | 25 |
| Herald | Scoring | +20 contract points when your team leads this card | 25 |
| Multiplier | Scoring | +5 contract multiplier when this card wins a trick | 50 |
| Synthetic | Identity | Replaces [4♣] | 25 |

- Multiplier is offered half as often as the other types.
- The Synthetic text is shown for a [J♣], [Q♣], [K♣], or [A♣] that replaces [4♣].

### Synthetic cards

A synthetic card is an extra copy of a real card, such as a second [A♥]. It lets a build grow past what the 52-card deck allows, such as a
fifth [A] or a second [A♠].

- It counts as the card it copies everywhere. A synthetic [A♥] is an
  [A] and a [♥] for following suit, winning tricks, and sigils.
- Each synthetic offer names the real card it replaces, an unowned card from
  [2] through [9] of the **same suit**, chosen at random. While the copy is
  owned, the replaced card is removed from the deck, so every hand still gets
  13 cards. Selling the copy puts the replaced card back.
- The copied card is always a **[J], [Q], [K], or [A]**, even one somebody
  owns. Synthetic copies raise rank without changing suit: [A♣] can replace [4♣],
  but [A♦] cannot. If no eligible replacement is available, omit that offer.
- A synthetic card costs the copied card's price plus the premium, counts
  toward the 8-card cap, and holds no other engraving.
- A copy and its original can meet in one trick, and the first one played wins.

Synthetic cards are the late-run enabler, and they combine with payoffs in ways
players discover: a synthetic [A♦] strengthens a [♦] build and feeds
Ranks, and a synthetic [A♠] upgrades a trump. Suit counts in the deck stay the
same; ownership guarantees the stronger card to its buyer.

## 8. Shops and economy

> **Status:** provisional. Purely random offers are [D7](#d7-offer-density),
> and income is [D9](#d9-income). Every count, price, and gold amount in this
> section is a starting value to test: [parameters](#appendix-b-parameter-register) P4–P15.

### Shop

- Each team has one shop per round, shared by both partners, with **3 sigil
  offers and 3 card offers**.
- Purchases are unlimited, bounded by gold, 7 sigil slots, and 8 owned cards
  per player. A bought offer leaves an empty slot until the next reroll.
- A **reroll** refreshes all six offers for 50 gold, plus 10 for each further
  reroll in the same shop. The cost resets at the next shop.
- Sigil offers are **purely random within rarity**, as in Balatro, and never
  include a sigil the team owns. Card offers are uniform over available cards.
  Density levers ([§15](#15-risks-and-tuning-levers)) wait until the
  commitment metrics call for them.
- Sigils and cards can be sold at any shop for half price.

### Gold

- Each team starts with **150 gold**.
- After each round, a team first gains **interest**: 10 per 50 gold held, up
  to 50. It then gains:
  - **100 base income**;
  - **10 gold per trick in its contract**, if it made the contract;
  - 50 gold per made nil.
- Gold from sigils arrives when it triggers.
- A typical round earns about 185 gold, so a run has about 1,450 to spend:
  roughly 7 sigils, 8 cards, and a reroll per shop.
- Contract gold gives early rounds an economic stake while their points are
  small, and it rewards bidding high and making it. The snowball is mild: two
  extra sets cost about one sigil over a run.

### How often a team sees support

With one reroll per shop, a team sees about 48 sigil offers per run: about 33
commons, 12 uncommons, 2.4 rares, and a legendary every other run. Under purely
random offers:

| Target | Sigils in pool | Relevant offers per run |
| --- | --- | --- |
| A major archetype | Its payoffs plus whichever shared enablers help in trials | Remeasure with the smaller pool |
| A minor archetype | About 5 payoffs plus shared support | Remeasure with the smaller pool |
| One side suit | Named-suit payoffs plus hand reshaping and lead control | Depends on the tested suit allocation |
| One specific card | — | seen in about two runs out of three |

- **Cards are the dense enabler.** A team sees most cards at some point,
  including about four [A] offers per run.
- **Sigil payoffs are the thin resource.** The
  [Commitment works](#fun-score) family decides whether density levers are
  needed.

## 9. Archetypes

> **Status:** the sigil pool now lives in the data files `data/sigils/` (one per
> candidate, with evidence and reasoning), the compact [registry](sigils/registry.md),
> and the viewer (`npm --prefix viewer run dev`, then `/sigils`). The
> [final report](sigils/final-report.md) summarizes the pool of 117 sigils and its
> open risks. The illustrative sigils below are the original seeds, kept as history;
> each one's fate is in its data file. The roster is [D11](#d11-archetype-roster) and
> the suit structure [D12](#d12-suit-structure).

An archetype is a plan a team can build a run around. Archetypes are a design
and simulation tool only. The game never names them, tags sigils with them, or
steers offers by them.

| Archetype | Weight | Plan | Natural partners |
| --- | --- | --- | --- |
| Suits [♣] [♦] [♥] | Major family | Own one side suit, its honors or its length; hold, lead, and win with it | Ranks, Low Cards, Streaks |
| Spades | Major | Own long or high [♠]s; win and trump with them | Bid High, Streaks |
| Ranks | Major | Collect one rank, mostly [A]s; hold, lead, and win with it | Suits, Bid High, Rainbow |
| Bid High | Major | Bid 8 or more and make the contract | Spades, Ranks, Streaks |
| Nil | Major | Bid nil and make it | Low Cards, Exact |
| Low Cards | Minor | Win with [2]s through [10]s | Suits, Spades, Nil |
| Rainbow | Minor | Win tricks with all four suits | Ranks, Suits |
| Streaks | Minor | Win tricks in a row | Bid High, Spades |
| Exact | Minor | Make exactly your contract | Nil, Low Cards |

- **Majors** are intended to carry a run; simulation decides how much support
  they need, without a dedicated-enabler quota.
- **Minors** have about five point sigils, and usually join a major.
- **Generic** point sigils support every build. Ideas too narrow for an
  archetype become one-offs
  ([Appendix C](#appendix-d-one-offs-not-archetypes)).

### Ways to score

Each archetype rewards one idea through several triggers, so its sigils stack
instead of competing.

| Trigger | What it checks | Example |
| --- | --- | --- |
| Hold | Both partners' hands after Opening and bidding | +20 contract points for each [A] your team holds |
| Lead | Cards your team leads | +15 contract points when your team leads an [A] |
| Win | The card that wins a trick for your team | +20 contract points when your team wins a trick with an [A] |
| Trump | Winning a trick by trumping | +20 contract points when your team wins a trick by trumping |
| Bid | The contract's size, or a nil bid | +15 contract multiplier when your team bids 8 or more |
| Make | The round's result | +15 contract multiplier if your team makes its contract exactly |

### Payoffs and enablers

**Payoffs** reward a result. **Enablers** make the underlying Spades hand better
or give the team meaningful control, even with no payoff sigils. A useful
question is: "Would I buy this before knowing what my scoring build will be?"
An answer that only names a future combo is insufficient.

Owned cards and same-suit synthetic upgrades remain sources of reliable
strength. Enabler sigils provide larger interventions in play, shared across
archetypes. No archetype is entitled to bespoke enablers, and the pool is not
padded with discounts or tiny offer adjustments to fill slots.

An enabler should show [standalone value](#standalone-enabler-value), then
additional support for at least two payoff families. It need not be
good in every hand: making a nil practical, for example, is already useful in
ordinary Spades. Every payoff must still work without an enabler.

### Shared enabler candidates

These seven candidates replace the previous enabler and utility lists. Values
and rarities are hypotheses, not simulated balance results. The benefit must
be visible in actual cards, legal plays, or who leads; none modifies the shop
or only changes how a payoff labels a card.

| Rarity | Rules text | Why take it before payoffs? | Further uses |
| --- | --- | --- | --- |
| C | Opening: Swap three cards with your partner | Move winners to one hand, offload dangerous cards, or create a void | Nil, suits, trumping, bidding 8 or more |
| C | Opening: Four cards your team holds become [2]s | See a lower hand before deciding whether to bid nil; some winners may be lost | Nil, exact bids, [2]–[10] payoffs |
| U | Opening: Four cards your team holds become [K]s | Gain potential winners before choosing a bid, with a risk of changing [A]s too | Larger bids, rank payoffs, winning in several suits |
| U | Opening: Four cards your team holds become [♥]s | Concentrate [♥]s and sometimes create a void before choosing how to bid | Suits, trumping, Nil; Rainbow only where trials show a benefit |
| U | Choose which partner leads after your team wins a trick | Reach the partner's winners and choose which hand controls the next lead | Consecutive wins, protecting nils, suit play |
| R | Opening: Raise every card your team holds by two ranks | Improve the whole team's hand, including its trumps | Larger bids, ranks, consecutive wins |
| R | Your team can play any suit on the last three tricks | Cash trumps or shed dangerous cards even when the led suit remains in hand | Nil, exact bids, last-trick payoffs |

**Resolution defaults:**

- "X cards your team holds become Y" samples exactly X distinct cards
  uniformly without replacement from both partners' current hands combined,
  then applies the change automatically. Partners do not choose, split the
  count evenly, decline, or reroll. Cards already matching Y remain eligible
  and count toward X. An [A] selected to become a [K] is downgraded.
- A rank destination changes only rank; a suit destination changes only suit.
  The destination is fixed, not random. "Raise every card" changes all cards
  by the stated number of ranks, capped at [A]. These changes affect following
  suit, trick strength, and payoff checks, and expire before the next deal.
- Opening effects resolve in the kernel's fixed priority. Each random effect
  draws from a seeded Opening stream; a later effect can select a card changed
  by an earlier one. Random changes finish before bids, never during play.
- The swap's explicit instruction offers a choice of up to three cards per
  partner, using each seat's existing information, and exchanges equal counts
  simultaneously. It does not use the random "become" template. Cards and
  engravings move together; ownership and the next guaranteed deal do not.
- The lead-choice effect moves the next lead between partners; the actual
  trick winner and trick count do not change. Playing any suit on the last
  three tricks overrides following suit, not the rule about leading unbroken
  [♠]s. Ordinary trump priority and tie-breaking still apply.
- Opening results are visible to the affected card's holder before bidding;
  they do not reveal extra cards from a partner's hand. Hold payoffs count
  once after bidding. No effect is an activated ability.

The archetypes below retain their payoffs. Names such as Bid High, Low Cards,
and Streaks are internal labels, never keywords printed on a sigil.

### Suits

**Plan.** Pick a side suit ([♣], [♦], or [♥]). Buy its honors or its length, add
synthetic copies late in the run, and hold, lead, and win with it. Honors and
length are two lines of one archetype that share its sigils:

- The **honors** line owns the [A], [K], and [Q] and cashes them early.
- The **length** line owns many cards of the suit. Long side suits get trumped
  once opponents run out, so this line leans on Hold and Lead payoffs, suit
  changes, and control of which partner leads.

**Named suits.** Start with [♦] examples rather than an adaptive suit package.
Use [♣], [♥], or [♠] versions only where simulation shows they earn their pool
slots; complete cycles are not required. The Suits budget stays at 18 payoff
slots, with the former six adaptive slots reassigned to named-suit candidates.
Concentrating support on [♦] is an explicit alternative if spreading it across
three side suits makes each build too thin ([D12](#d12-suit-structure)).

| Trigger | Rarity, category | Text |
| --- | --- | --- |
| Win | C, points | +10 contract points when your team wins a trick with [♦] |
| Hold | C, points | +10 contract points for each [♦] your team holds |
| Lead | C, points | +10 contract points when your team leads [♦] |
| Win | U, +mult | +15 contract multiplier when your team wins three tricks with [♦] |
| Make | U, ×mult | ×1.5 contract multiplier if your team wins five tricks with [♦] |
| Win | R, +mult | +2 contract multiplier when your team wins a trick with [♦] |

**Notes:**

- **Buys:** the named suit's honors, or its cheap low cards for length.
- **Later:** synthetic copies of its honors replace low cards of that suit.
- **Threat:** opponents void the suit and trump it.

### Spades

**Plan.** Own long or high [♠]s. [♠]s win in every hand: high [♠]s cash,
and low [♠]s trump. Named-[♠] versions of suit payoffs can score here too.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Trump | [C, points] +20 contract points when your team wins a trick by trumping |
| Lead | [C, points] +10 contract points when your team leads [♠] |
| Hold | [U, points] +5 contract points for each [♠] your team holds |
| Trump | [U, +mult] +15 contract multiplier when your team wins three tricks by trumping |
| Win | [R, +mult] +2 contract multiplier when your team wins a trick with [♠] |
| Win | [R, ×mult] ×2 contract multiplier if your team wins five tricks with [♠] |

**Notes:**

- **Buys:** [♠]s, both high and low.
- **Threat:** opponents strip trumps by leading [♠]s.

### Ranks

**Plan.** Collect one rank, usually [A]s, and hold, lead, and win with it. [A]s
win in every side suit and cost the most. A few sigils pay for [K]s instead.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Win | [C, points] +20 contract points when your team wins a trick with an [A] |
| Hold | [C, points] +20 contract points for each [A] your team holds |
| Lead | [C, points] +15 contract points when your team leads an [A] |
| Win | [C, points] +15 contract points when your team wins a trick with a [K] |
| Hold | [U, +mult] +20 contract multiplier if your team holds four [A]s |
| Win | [U, ×mult] ×1.5 contract multiplier when your team wins three tricks with [A]s |
| Win | [R, +mult] +5 contract multiplier when your team wins a trick with an [A] |

**Notes:**

- **Buys:** [A]s, then synthetic [A]s in later shops.
- **Threat:** [A]s cost the most, and they get trumped once a suit runs out.

### Bid High

**Plan.** Bid 8 or more and make the contract. Bid High pays for
bidding high, so its risk lives in the bidding, and a set multiplies against
you.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Bid | [C, points] +5 contract points for each trick in your team's contract |
| Bid | [C, points] +60 contract points when your team bids 8 or more |
| Bid | [C, +mult] +15 contract multiplier when your team bids 8 or more |
| Bid | [U, +mult] +20 contract multiplier if your team's contract is 10 or more tricks |
| Bid | [U, points] +10 contract points for each trick in your team's contract |
| Make | [R, +mult] This sigil gains ×5 contract multiplier every time your team makes a contract of 8 or more (currently ×0) |
| Make | [R, ×mult] ×2 contract multiplier if your team makes a contract of 8 or more |

**Notes:**

- **Buys:** high cards and [♠]s.
- **Threat:** a set multiplies against you, and opponents defend hard against
  big contracts.

### Nil

**Plan.** Bid nil and make it. The nil score joins the base, so every
multiplier also grows your nils, and a failed nil costs more as the build
grows.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Make | [C, points] +50 nil points |
| Bid | [C, +mult] +15 contract multiplier when your team bids nil |
| Make | [U, ×mult] ×1.5 contract multiplier if your team makes a nil |
| Make | [R, points] This sigil gains +40 nil points every time your team makes a nil (currently +0) |

**Notes:**

- **Buys:** low cards. Nil teams avoid owning [A]s and high [♠]s.
- **Threat:** opponents lead low to force the nil bidder to win a trick.

### Low Cards

**Plan.** Win tricks with [2]s through [10]s, the cheapest cards in the shop. Low
cards win through length, by trumping, and after the honors are gone.

- **Payoffs:**
  - [C, points] +15 contract points when your team wins a trick with a [2] through [10]
  - [C, points] +10 contract points when your team leads a [2] through [10]
  - [U, +mult] +15 contract multiplier when your team wins four tricks with [2]s through [10]s
  - [R, points] +150 contract points when your team wins a trick with a two

### Rainbow

**Plan.** Win tricks with cards of all four suits. That takes a winner in
every suit, so Rainbow buys an [A] or [K] of each and fills gaps with
synthetic copies.

- **Payoffs:**
  - [C, points] +25 contract points when your team wins its first trick with each suit
  - [C, +mult] +15 contract multiplier if your team wins tricks with all four suits
  - [U, ×mult] ×1.5 contract multiplier if your team leads all four suits
  - [R, ×mult] ×2 contract multiplier if your team wins tricks with all four suits

### Streaks

**Plan.** Win tricks in a row. Each hand becomes a sequencing puzzle: give up
your losers at the right moment, then keep the lead.

- **Payoffs:**
  - [C, points] +10 contract points when your team wins consecutive tricks
  - [C, +mult] +15 contract multiplier when your team wins the last trick of a round
  - [U, +mult] +20 contract multiplier when your team wins four tricks in a row
  - [R, ×mult] ×2 contract multiplier if your team wins five tricks in a row

### Exact

**Plan.** Make exactly your team's contract. Overtricks already score
nothing, and Exact pays for avoiding them. Exact pairs naturally with Nil and Low Cards.

- **Payoffs:**
  - [C, points] +50 contract points if your team makes its contract exactly
  - [C, +mult] +15 contract multiplier if your team makes its contract exactly
  - [U, ×mult] ×1.5 contract multiplier if your team makes its contract exactly
  - [R, +mult] This sigil gains ×5 contract multiplier every time your team makes its contract exactly (currently ×0)

### Generic

Generic point sigils fit any build and carry runs while a plan comes together:

- [C, points] +5 contract points when your team wins a trick
- [C, points] +40 contract points
- [U, +mult] +10 contract multiplier
- [R, ×mult] ×1.5 contract multiplier
- [L, ×mult] ×2 contract multiplier

Generic builds use the shared enablers above. There is no separate economy or
shop-modifier sigil pool: a sigil slot should buy a substantial scoring effect
or a new way to play the hand.

## 10. Sigil pool skeleton

> **Status:** superseded by the soft pool targets of [D30](#d30-pool-shape). The
> pool is in `data/sigils/`, the [registry](sigils/registry.md), and the viewer; see
> the [final report](sigils/final-report.md) for counts by rarity, category, and
> archetype. The tables below are kept as history.
>
> **Earlier status:** provisional. The category split follows
> [D3](#d3-scoring-categories), pool sizes are [parameters](#appendix-b-parameter-register) P21, and every
> budget is a starting size for number sweeps.

The working pool has **97 candidates: 90 payoffs and 7 shared enablers**.
The payoff allocation is retained; the former 55 utility slots are retired.
There is no target utility percentage and no requirement to refill those
slots. Cut candidates that don't make the game better, even if the final pool
is smaller.

### By rarity and category

| Rarity | +Points | +Mult | ×Mult | Utility | Total |
| --- | --- | --- | --- | --- | --- |
| Common | 30 | 9 | 3 | 2 | 44 |
| Uncommon | 10 | 14 | 6 | 3 | 33 |
| Rare | 4 | 5 | 6 | 2 | 17 |
| Legendary | — | — | 3 | — | 3 |
| **Total** | **44** | **28** | **18** | **7** | **97** |

At least 15 of the 44 contract-point sigils, and at least 12 of the 46
multipliers, scale with or require the contract size
([keeping bids tense](#keeping-bids-tense)).

### Point sigils by archetype

| Archetype | Common | Uncommon | Rare | Total |
| --- | --- | --- | --- | --- |
| Named suits ([♦] first; [♣] [♥] [♠] variants tested) | 11 | 6 | 1 | 18 |
| Spades | 3 | 3 | 2 | 8 |
| Ranks | 4 | 3 | 2 | 9 |
| Bid High | 4 | 3 | 2 | 9 |
| Nil | 3 | 3 | 2 | 8 |
| Low Cards | 3 | 1 | 1 | 5 |
| Rainbow | 3 | 1 | 1 | 5 |
| Streaks | 3 | 1 | 1 | 5 |
| Exact | 2 | 2 | 1 | 5 |
| Generic and one-offs | 6 | 7 | 2 | 15 |
| **Total** | **42** | **30** | **15** | **87, plus 3 legendary = 90** |

Any [♠] variants in the named-suit allocation also support Spades. Their count
and the split between side suits are measured in stage 3, not assumed.

### Utility by family

| Family | Common | Uncommon | Rare | Legendary | Total |
| --- | --- | --- | --- | --- | --- |
| Hand reshaping | 2 | 2 | 1 | — | 5 |
| Lead control | — | 1 | — | — | 1 |
| Freedom to play cards | — | — | 1 | — | 1 |
| **Total** | **2** | **3** | **2** | **—** | **7** |

These are the shared candidates in §9, not separate enablers for each
archetype. Shop and economy sigils receive no allocation.

### Budgets

These are starting sizes for sigils of the right kind on a made contract. Number
sweeps ([§13](#declarative-sigils)) move each sigil toward the par curve and
its lift band.

| Rarity | +Contract points | +Contract multiplier | ×Contract multiplier |
| --- | --- | --- | --- |
| Common | +15 to +20 per narrow event; +5 per trick in the contract; +40 to +60 flat | +10 to +15 on a condition met in about a third of made contracts | ×1.5 on a narrow condition |
| Uncommon | +30 per event, or a tuned amount per card held | +15 to +20 on a moderate condition, or +10 broadly | ×1.5 broadly, or ×2 narrowly |
| Rare | +50 per event, or a run-long counter | +2 to +5 per repeated event or growth step | ×2 broadly, or ×3 narrowly |
| Legendary | — | — | ×2 to ×3 broadly |

Repeated-event rewards start smaller than one-time rewards: +2 on each [♦]
win, +15 for the last trick, and +5 on a winning engraved card are illustrative
candidates. Tune amounts to trigger frequency and total contribution, not by
multiplying every old +mult value by the same constant. ×Multipliers retain
their existing illustrative factors.

Arithmetic checks against par, using assumed rates rather than simulation:

- **Round 4 (par 1,600).** Two commons provide +40 points each, and a third
  provides +15 mult with probability one third. Assume that probability is the
  same on makes and sets, so expected mult is 10 + 15 / 3 = 15. A made 8
  averages (80 + 40 + 40) × 15 = 2,400. A set averages −80 × 15 = −1,200.
  With one set in five, the round averages 1,680.
- **Round 8 (par 6,000).** Two point sigils provide +85 total; +20 and +15
  mult both hold, giving ×45; a ×1.5 holds half the time on both makes and
  sets, giving an expected compounding factor of 1.25. A made 9 averages
  (90 + 85) × 45 × 1.25 = 9,843.75. A set averages −90 × 45 × 1.25 =
  −5,062.5. With one set in four, the round averages about 6,117.

The harness must measure the actual joint trigger and set rates. These checks
do not establish balance or justify adding restrictions to hit a target.

## 11. Multiplayer

> **Status:** provisional, and untested until stage 7
> ([D19](#d19-simulation-configuration)).

The first prototype is single-player. Every rule is symmetric, so multiplayer
needs only these additions:

- Two humans can partner against an AI team, or four humans can play two
  against two.
- Partners share one shop and one gold pool. Either partner can buy, sell, or
  reroll at any time during the shop, and the first action taken wins. The
  team's shop closes when both partners press Done.
- Card offers are tagged to one partner at random. A team of two humans can
  own 16 cards, so its scores run above the single-player par curve. This is
  accepted, and checked as a secondary configuration
  ([§13](#experiments)).
- Teams shop at the same time, and card offers never overlap.

## 12. Metrics: what fun means

> **Status:** provisional. The structure is [D14](#d14-metric-structure), and
> every target band, tolerance, and weight is a placeholder until the harness
> runs: [parameters](#appendix-b-parameter-register) P25–P28 and P30.

The metrics answer one question: what does it mean for Rogue Spades to be fun?
They are **targets for continuous optimization**, not pass/fail bars. The
design improves over rounds of iteration: each round measures the pool, changes
the pieces that most hold it back, and keeps the changes that make the game
better. The metrics come in two layers:

- **Sigil metrics** describe what each sigil contributes, so "every sigil
  matters" can't be averaged away. A sigil outside its target bands is a prime
  candidate for retuning, reshaping, or replacement, weighed against what it
  adds elsewhere.
- The **fun score** is a weighted sum that describes the whole pool, used to
  compare pool versions and tuning levers.

Neither replaces judgment. Designers weigh the metrics alongside subjective
elegance: whether a piece is clean, surprising, and pleasing to play. Every
kept change records its evidence and its reasoning.

Every band is a placeholder. Real values are set once the harness runs, and
recalibrated at stage 7.

### Sigil metrics

| Metric | Your goal | Measured as | Starting target band |
| --- | --- | --- | --- |
| 1. Choices matter | Every sigil contributes measurably to victory | Win-rate lift over a same-rarity control, from randomized grants ([§13](#experiments)). For point sigils, the decisive share: the holder's wins that become losses or draws when rescored without the sigil | Lift above 0; decisive in at least 5% of wins |
| 2. Payoffs are doable | Payoffs work naturally; enablers are worth buying before payoffs | Base and committed payoff trigger rates; standalone enabler value under the trials below; additional trigger lift for at least two payoff families | Payoff base at least 10% of rounds and committed at least 60%; enablers show standalone value and add at least 15 percentage points to trigger rates in each of two families |
| 3. No invalidation | Opponents can't turn a strategy into a non-game | For each archetype and each sigil that touches opponents, the drop in the archetype's win rate when the opponents hold it | At most 15 points |
| Power ceiling | No dominant best build | Lift; win rate of the strongest pairs and the strongest attainable builds | Lift at most 12 points; no pair above a 65% win rate |

Distance from a band is a signal, not a verdict. A sigil far below its band on
Choices matter probably needs a bigger number or a new shape. One slightly over
the power ceiling may still be worth keeping if it creates a run's best moments
and opponents can answer it. Large, confident misses get attention first.

### Standalone enabler value

Choices matter's "lift above zero" says little about enablers: a tiny shop
improvement might technically help or become useful only beside a narrow
payoff. Every enabler is also measured on these, before combo trials:

- **Material benefit without payoffs.** On paired seeds, give the team the
  candidate while both teams otherwise play without sigils; prohibit further
  sigil purchases in both arms, but retain ordinary card purchases and the
  base economy.
  Compare against the same run with the candidate disabled, charging the
  same gold and reserving the same slot in both arms. The target is a lower
  bound of at least **3 percentage points** on the 90% confidence interval
  for win-rate lift. Measure buys at shops 1, 3, and 5 separately.
- **Worth its slot and price.** In that same no-other-sigils setting, compare
  buying it with buying the same-rarity flat-points control at the same price.
  The target is a lower bound of the paired 90% interval no worse than **−2
  percentage points** at each purchase stage. Beating a blank says little if
  an ordinary payoff makes it a poor purchase.
- **Breadth.** Run support trials for at least two different payoff
  families, against the trigger-lift band in Payoffs are doable. An enabler
  whose value comes from one spectacular combo is a narrow enabler, though it
  may still earn a place as a payoff-like piece.
- **A reason to want it.** In playtests, offer the candidate beside that
  same-price generic payoff before any other sigils are owned. As a starting
  target, at least **40%** of choices should favor the enabler, with players able
  to explain a useful change to their hand or play without invoking a future
  payoff. Report sample size and reasons; simulation lift alone does not prove
  an effect is interesting.

These are provisional targets, not evidence about the seven candidates.
Report results by purchase stage and compare a range of ordinary hands, not a
hand chosen to flatter the effect. If the confidence intervals stay wide,
gather more runs before acting on them. Prefer tuning the amount or reshaping
the effect over rescuing a weak enabler with shop steering, payoff-only
identity changes, or extra restrictions. The power ceiling and opponent
counter scans still matter, especially for freedom from following suit.

### Fun score

| Family | Weight | Your goal | Sub-metrics and starting bands |
| --- | --- | --- | --- |
| 4. Close and live | 25 | Close and meaningful at every stage, with no runaway leader | Median final margin at most 20% of the winner's score; the team trailing after round 5 wins at least 25% of runs; rounds 1–4 hold at least 20% of all points |
| 5. Commitment works | 15 | I can "do the thing" if I commit | Builds committed by shop 2 are online by round 4 in at least 60% of runs; committed teams win at least as often as flexible ones |
| 6. Archetypes viable | 15 | Every archetype is viable, no build is forced, nothing railroads | Each archetype's committed win rate is 40–60% against a flexible field; no archetype appears in more than 25% of winning flexible builds; two-archetype builds win within 5 points of pure ones |
| 7. Synergy and combos | 15 | Pieces multiply each other, and broken combos are there to discover | Same-archetype pairs beat the sum of their parts by at least 25% on average; at least 15 strong pairs (50% over their parts), at least 5 of them crossing archetypes |
| 8. Skill and bidding | 15 | Good play wins, and bidding stays tense | Tier 2 beats tier 0 in at least 65% of paired runs; late-run (rounds 6–8) overtricks average at most 1 per made contract; set rate 10–25% |
| 9. Simplicity | 15 | Each piece is easy to understand and track; depth comes from combinations | Mean per-sigil simplicity score from the rubric below; fewer numeric values, boolean conditions, arithmetic riders, and tracked states score higher |

- **Scoring the families.** In families 4–8, each sub-metric scores 1 inside
  its band and falls linearly to 0 at a tolerance listed with it. A family
  scores the mean of its sub-metrics. Family 9 uses the rubric below. The fun
  score is the weighted sum, from 0 to 100.
- **Using it.** Each optimization round compares the pool before and after
  its changes on paired seeds. A change is kept when the evidence and the
  designer's judgment together say the game got better: usually a fun-score
  gain, a sigil moved toward its bands, or a clearly more elegant piece at
  little cost. Confidence intervals say how sure the evidence is; they are
  not cutoffs. A more complex replacement should show a net improvement after
  its simplicity penalty; if the paired 90% confidence interval for that
  improvement includes zero, prefer the simpler candidate. Complexity never
  excuses an activated ability ([§5](#slots-and-ownership)).
- **Online** means holding two of the archetype's payoffs and one of its
  enablers. Three owned cards that its payoffs reward count as an enabler.
- **Flexible and committed teams.** A flexible team buys by plain value. A
  committed team adds a bonus for its archetype's sigils and for the cards
  they reward ([shop AI](#shop-ai)).

### Simplicity rubric

Score the player-facing rule in the sigil data and its canonical text, not
just sentence length. **Every numeric value and every boolean condition has a
positive cost**, including the payout and the first condition. Nothing becomes
free by spelling a number out, renaming a condition, or hiding it in a keyword.

Add the following **complexity costs** to get C:

| Burden | Cost | Example |
| --- | --- | --- |
| Numeric value or rank literal | +1 per occurrence | Payouts, counts, thresholds, rank literals, range endpoints, ordinals, and displayed totals all count; "Four" costs 1, `[2]s` costs 1, and `[K]s` also costs 1; +20 is one value, not two digits |
| Boolean condition | +1 per atomic check | Whether your team wins, holds a card, makes its contract exactly, or whether a card is [♠]; count every check in an AND, OR, or exception, not just the whole clause |
| Arithmetic rider on the quantity rewarded | +2 per operation | Subtracting five and flooring at zero in "beyond five" is one combined arithmetic rider, in addition to its number and cutoff condition |
| Computed selector or variable destination | +1 per selector | "Longest suit" compares suits; "become low cards" needs a destination-selection rule beyond naming one rank |
| Extra tracked state | +1 per counter or remembered fact | A sigil-specific milestone count, consecutive-win count, a once-only flag, or a growth counter carried between rounds |
| New player-facing term | +2 per term; +4 at common | `Opening:` is an intentional new keyword and pays this cost, as would shorthand such as "high contract", "low cards", or "streak" |

Count rule-level checks, including triggers and filters, regardless of whether
the text says "when", "if", or "for each". A numeric threshold charges both
for its number and for its comparison. The removed "high contract" shorthand
would still charge for its hidden 8 and comparison, plus the new-term penalty.
Write "bids 8 or more" instead. Repeating the same numeral in different roles
charges each occurrence; a rule field and the text generated from that
field are one occurrence, not two. A changing "currently ×0" display charges
one numeric value plus the cost of remembering that state.

Ordinary score addition and multiplication add no arithmetic-rider cost, but
their printed amounts always incur the numeric cost. "Your team" establishes
scope without another check. Do not charge implementation guards or repeated
evaluations of the same check; charge the distinct checks the player must
understand. Reusing an ordinary Spades fact adds no state cost, but a condition
on that fact still costs. Increasing rarity or adding a UI counter waives none
of these costs.

The vocabulary penalty is additional to the mechanics it names. Putting a
term in a glossary or reusing it across the pool does not waive the per-sigil
cost. Familiar card and Spades language (face cards, suits, trump, nil, bid,
contract) and the three core scoring labels are the baseline vocabulary;
new shorthand is not. "High contract" adds 4 complexity at common compared
with spelling out "8 or more"; "streak" likewise adds 4 on top of tracking
consecutive wins. Internal archetype labels carry no cost because players
never see them. The removed lost-trick exception does not return under a new
name.

**Expand hidden complexity before scoring.** Rank names, symbols, and aliases
have the same cost: `[2]` and its spelled-out name are one rank literal, as
is `[K]`. The plural "s" is not another value. The count "Four cards" is
independent of its destination rank and always adds its own point.
For random transformations, charge the team-held filter once; it establishes
the eligible cards, with no invented condition for each sampled card.

"Become low cards" is not a cheaper replacement for "become [2]s". Using the
old meaning [2] through [10], it hides two rank endpoints and a rule for picking
an output rank, in addition to the term cost. Charge that expanded structure
plus every actual boolean check. The effect is incomplete until it defines
which ranks can result and how they are selected. A range and an equivalent
list of ranks normalize to the same structure before scoring; changing the
wording cannot lower the cost. Even familiar classes such as "face cards"
still pay for their underlying rank set and output-selection rule.

`Opening:` does not waive its new-term penalty just because it is now an
approved template. At common, the four-[2] transformation pays 4 for Opening,
1 for Four, 1 for [2], and 1 for the team-held filter, totaling 7.

The per-sigil simplicity score is **S = 1 / (1 + C)**. Family 9 is the mean S
across all sigils in the candidate pool, counting each once, including utility
and rule benders; an empty pool scores 1. Report each candidate's costs and
compare replacements in the same pool slot so unrelated simple filler cannot
mask a rider's cost. The one-effect rule remains mandatory at every rarity.

| Candidate | Cost breakdown | C | S |
| --- | --- | --- | --- |
| +40 contract points | One number | 1 | 1/2 |
| ×1.5 contract multiplier if your team makes its contract exactly | One number, one condition | 2 | 1/3 |
| +20 contract points when your team wins a trick with an [A] | Payout and rank literal, two conditions: team wins and winning rank is [A] | 4 | 1/5 |
| +5 contract points for each [♠] your team holds | One number, two conditions: held by your team and suit is [♠] | 3 | 1/4 |
| +5 contract points for each [♠] your team holds beyond five | Two numbers, three conditions (including count > 5), one arithmetic rider worth 2 | 7 | 1/8 |
| Opening: Four cards your team holds become [2]s (Common) | Opening 4, Four 1, [2] 1, team-held filter 1 | 7 | 1/8 |
| Opening: Four cards your team holds become low cards (Common; rejected shorthand) | Opening 4, Four 1, hidden [2]/[10] endpoints 2, destination selector 1, "low cards" term 4, team-held filter 1; further checks cost extra | ≥13 | ≤1/14 |

Thus "beyond five" strictly lowers the score with everything else held equal,
even if both versions balance equally well. For a useful balance comparison,
sweep each version's payout against the same metrics and par targets on paired
seeds; compare their best-tuned versions, not just identical amounts with
very different power. The old rider may be an experiment control, but remains
excluded from the shipped pool by §5.

These costs and the 15-point family weight are provisional designer choices,
not simulated evidence about human comprehension. Simulations measure whether
complexity buys better play; playtests calibrate the rubric using time to
explain a sigil and errors predicting when it pays. Record both in the decision
report before changing the rubric ([P28](#appendix-b-parameter-register)).

### Diagnostics

These are tracked but not weighted:

- **Input randomness:** random cards per hand, which the card cap keeps at 5
  or more. Opening draws are also input randomness and finish before bidding;
  log their effects on hand strength and nil feasibility separately from the
  number of randomly dealt cards.
- **Economy:** gold unspent at the end of a run; purchases and rerolls per
  shop.
- **Text quality:** the wording lint ([§5](#simplicity-rules)); mechanical
  simplicity is scored in family 9, not left as an unweighted diagnostic.
- **AI play quality:** 1.0's bench checks, such as wasted overtakes, missed
  nil covers, and nil suicides.

### Results

- Sigil-metric and fun-score results go into the sigil records and decision records
  described in [§16](#16-evidence-and-design-records).
- A sigil far from its targets is retuned, usually by a number sweep,
  reshaped, or replaced.
- An archetype whose sigils keep falling short drops to one-offs.

## 13. Simulation

> **Status:** provisional. The architecture follows
> [D15](#d15-sigil-representation)–[D18](#d18-throughput), the measurement
> design [D28](#d28-measurement-design), and the simulator language
> [D29](#d29-simulator-language). The tier budgets and experiment precision are
> [parameters](#appendix-b-parameter-register) P24 and P27. The
> [sigil design plan](sigil-design-plan.md#the-measurement-engine) gives the
> full measurement engine.

### The chicken-and-egg answer

The worry is circular: designing sigils needs strong simulated players, but the
players need to understand the sigils. The answer is that **no AI component has
sigil-specific knowledge**:

- Play and bidding search the real rules and scoring through the rules kernel,
  so a new sigil changes their choices the moment it exists.
- The shop values an offer with a model fitted to randomized simulation,
  checked against playing sampled hands with and without it.
- Hand changes and play-rule enablers need new rollouts with their choices
  enabled; any fitted estimates come from those measured outcomes.

Fidelity therefore grows by stage, through search budgets and fitted values,
and never through per-sigil AI code ([§14](#14-staging)).

### The rules kernel

One kernel runs every simulation and AI search. It is native Rust, for
throughput ([D29](#d29-simulator-language)). The playable game comes later and
must share its rules, through WASM or a port checked by replaying seeds.

- **Compact state with apply-and-undo moves,** with cards as 64-bit masks, so
  search never deep-copies the game. 1.0's engine runs `structuredClone` on
  the whole state for every action.
- **Hook tables.** Sigil data compiles into tables for legal plays, each card's
  effective suit and rank, legal plays, the next leader, and scoring events.
- **A ledger** records every contract point and multiplier each sigil
  contributes, so any score can be recomputed without any set of sigils.
- **No globals.** Randomness comes from explicit seeded streams, and runs are
  deterministic: the same seed and configuration give the same record.

### Declarative sigils

Every sigil and engraving is data: a trigger, a filter, an effect, its numbers,
and its rarity. One interpreter runs them all, and the rules text is generated
from the data. For example:

```json
{
  "id": "aces-win",
  "rarity": "Common",
  "archetypes": ["Ranks"],
  "role": "payoff",
  "effect": { "type": "points", "amount": 20, "on": { "event": "win", "card": { "rank": "A" } } }
}
```

generates "+20 contract points when your team wins a trick with an [A]".
A rule bender names one of a small set of rule hooks written in code:

```json
{
  "id": "four-kings",
  "rarity": "Uncommon",
  "archetypes": ["Bid High", "Ranks", "Rainbow"],
  "role": "enabler",
  "effect": { "type": "setRank", "rank": "K", "count": 4, "on": "opening", "selection": "uniformWithoutReplacement", "scope": "teamHands" }
}
```

The grammar covers:

- **Events:** opening, hold, lead, play, win, trump, bid, make, exact, nil made, first
  and last trick, and consecutive wins.
- **Card filters:** rank, rank range, and named suit (including [♠]).
- **Scaling:** per card held, per trick in the contract, and per run-long
  counter.
- **Thresholds:** simple milestones; no subtraction from a held-card count.
- **Effects:** contract points, +multiplier, ×multiplier, fixed-window hand
  changes, and play-rule hooks. No shop-modifier hooks are needed.

Keeping sigils as data makes three things possible:

- **Number sweeps.** Grant tournaments randomize each sigil's amount, which
  measures its dose-response, and the amount moves toward its target lift
  ([experiments](#experiments)).
- **Enumeration.** Common payoffs are enumerated from the grammar, roughly
  6 triggers × 15 nouns × 3 effects ≈ 270 candidates.
- **Targeting.** The grammar shows which sigils touch opponents, so the No
  invalidation scan runs only on those.

### AI tiers

| Tier | Play | Bidding | Shop | Throughput target (18 cores) | Used for |
| --- | --- | --- | --- | --- | --- |
| 0 | Heuristic: 1.0's rollout policy on the kernel, plus a one-trick lookahead scored with sigils | Trick estimate, then expected value over the scoring model | Fitted value model | At least 100 runs per second | Grant tournaments, sweeps, screening |
| 1 | Information-set MCTS, about 200 iterations over 32 deals that fit the bids | Expected value over 40 rollouts | Fitted value model | At least 5 runs per second | Calibration and confirmation |
| 2 | Information-set MCTS, 1,500 iterations over the best 64 of 512 deals | Expected value over 160 rollouts | Rescoring with 64 hands | At least 0.5 runs per second | Final checks, tier calibration, and the shipped AI |

- The algorithms are 1.0's, ported to Rust: deals sampled to fit the bids,
  UCB search, heuristic rollouts, tie-breaking among near-equal moves, and
  bidding and nil decisions by expected value.
- **Value means win probability** over the rest of the run, not round score.
  A fitted curve converts each outcome's score margin and the rounds left into
  win probability, so trailing teams take risks late, as people do. Growth
  counters and gold are valued with the shop value model, so the AI plays
  toward run-long effects.
- **Opening choices,** such as a swap, are chosen from about 10 heuristic
  candidates by rollouts at tiers 1 and 2; tier 0 uses the heuristic alone.
- Tier-0 estimates are corrected toward tier 1 by a calibration subset on
  shared seeds, and tier 2 spot-checks rankings at the end
  ([experiments](#experiments)).

### Information and fairness

AI seats see exactly what a human in their seat would see:

- their own hand, and their partner's owned cards;
- every card played;
- opposing sigils, but only once revealed.

Searches sample hidden hands that are consistent with voids, bids, and known
cards. No tier ever reads hidden cards. Opponents' synthetic cards, and the
cards they replace, stay unknown until played, so searches treat the unseen
deck as standard.

### Shop AI

- **Value from a fitted model** ([D17](#d17-shop-ai)). Each offer's value is
  its predicted gain in win probability, net of its price at that shop's
  value of gold. It comes from the outcome model fitted to the latest grant
  tournament: the sigil's value by shop, its pair terms with owned sigils, and
  its coherence with the build. Cards use the same form.
- **Enabler values** come from the same randomized grants, so they include
  play under the changed rules, seeded Opening changes, and swap or lead
  choices made with each seat's actual information.
- **Validation by rescoring.** On a subsample of shop visits, the AI samples
  hands built from the team's owned cards plus random fill, and plays them on
  tier 0 with and without each offer. The model's ranking is checked against
  that, and clearly mispriced offers are corrected.
- **New sigils** start from a feature model (rarity, category, trigger type,
  and hand-level trigger rate) until a tournament has measured them.
- **Exploration.** In simulation, about 5% of purchases are random among
  affordable offers, so the model keeps seeing alternatives.
- **Buying.**
  - Buy the best value per gold while it clears a bar that accounts for
    interest.
  - Reroll when nothing clears the bar and gold allows.
  - Sell when a slot is needed and an offer beats an owned item by more than
    the sale loses.
- **Policies.** A flexible team uses plain values. A committed team adds a
  bonus for one archetype's sigils and for the cards they reward. A
  build-chaser buys one target build's pieces when offered.

### Experiments

- **Seeded streams.** Every random draw comes from a named stream:
  - deals, one stream per round;
  - Opening selections, streams keyed by round, team, and sigil identity;
  - offers, one stream per team, shop, and reroll;
  - AI search, one stream per seat.

  The arms of an experiment share deals and offers until their choices
  diverge.
- **Stable dealing.** Each round's deal is a fixed permutation of the deck.
  Owned cards are pulled out, and the random fill takes the next cards in
  order, so arms whose owned cards differ slightly still get nearly the same
  hands.
- **Duplicate boards.** Every seed is played twice with the teams swapping
  seats, as in duplicate bridge, which cancels most deal luck between them. The
  pair of runs, a **board**, is the unit of analysis. Its outcome is the score
  margin smoothed through the fitted margin-to-win curve, which carries more
  information than a win or loss.
- **Grant tournaments** ([D28](#d28-measurement-design)) are the main
  experiment. Both teams shop normally, and at random shops each team is
  forced to buy random sigils at full price. One regression over all boards
  estimates every sigil's value at once. It has terms for amount, acquiring
  shop, coherence with the team's build, and pairs (a factorization machine),
  with empirical-Bayes shrinkage and cluster-bootstrap 90% intervals.
  - **Grants** mix uniform draws with draws coherent with a build, and favor
    uncertain sigils. Candidates enter only through grants.
  - **Randomized amounts and gold** measure each sigil's dose-response and the
    value of gold at each shop.
  - **Clean boards,** about a tenth, have no grants and measure the game as
    played.
- **Precision targets.** Experiments are sized to a target interval width
  (P27) from measured throughput, not to a fixed run count.
- **Calibration.** A tier-1 subset of each tournament, on shared seeds,
  corrects tier-0 estimates and measures each sigil's skill gradient.
- **Hand-level trials** (stage 2). Fixed builds of sigils and owned cards play
  single rounds against random deals, measuring trigger rates, points per
  round, and set rates. They're cheap, and they don't depend on the shop AI.
- **Controls.** Every tournament grants same-rarity plain sigils, "+N
  contract points" with N at the rarity's flat budget, so every lift is
  measured against them in the same runs. Forced-pick trials on a few sigils
  check the tournament's estimates.
- **Standalone enabler trials.** A tournament whose shops sell cards but no
  sigils, granting only enablers and controls, runs the no-payoff comparisons
  in [§12](#standalone-enabler-value) by purchase stage.
- **Commitment trials.** One team commits to an archetype at shop 1. The trial
  measures online rates and win rates against a flexible field.
- **Counter scans.** In commitment trials, opponents receive random grants of
  sigils that touch opponents, measuring each archetype's loss.
- **Pairs and builds.** The strongest predicted pairs are oversampled as joint
  grants and confirmed on fresh seeds. A search over attainable builds feeds
  build-chaser arms, narrowed by successive halving, to find the power
  ceiling.
- **Secondary configuration.** Teams of two humans with 16 owned cards are
  checked at stage 7, but not balanced for.

### What carries over from 1.0

| Keep | Rebuild |
| --- | --- |
| The card model and rendering; the table, hand, trick, shop, and scoreboard components; styles | The engine core: `reduce`, `drain`, and the per-action `structuredClone` become the Rust kernel |
| The Spades rules logic, as the kernel's reference | Sigil handlers and the 28-window `Ctx` API become declarative data and rule hooks |
| The AI algorithms, ported to Rust: deal sampling fitted to bids, MCTS, tie-breaking, expected-score bidding, and nil logic | The scoring probes, which the kernel's exact scoring replaces |
| The bench metric definitions: set rate, overtakes, and nil covers | The shop, which becomes team-level and sells cards with engravings |
| — | Randomness: seeded streams instead of patching `Math.random` |
| — | The bench, which becomes duplicate grant tournaments with per-sigil ledgers and run records |

## 14. Staging

> **Status:** provisional ([D20](#d20-staging),
> [D21](#d21-candidate-generation)).

The simulation and the first draft of the pool grow together, one rarity at a
time; each stage moves on once its work is in place. Optimization doesn't stop
at a stage boundary: later rounds revisit any sigil as the pool around it
changes. Cards come before sigils on purpose: they're the main enabler, so
their prices must be stable before any payoff can be measured.

| Stage | Builds | Moves on when |
| --- | --- | --- |
| 0. Kernel | The Rust rules kernel, AI tiers 0–2, seeded streams, the experiment runner, and the analysis pipeline | Plain 8-round Spades: the AI meets 1.0's play targets (set rate, nil success, bid error); measured throughput per tier sizes every experiment; the pipeline recovers planted effects |
| 1. Cards | The card shop, the 8-card cap, income and interest, and rerolls, with no sigils; 1.0's UI ported onto the kernel | Card prices give roughly equal value per gold; no card dominates; hands keep their variety; the game is playable by hand |
| 2. Commons, hand level | The grammar, text generation, and lint; about 270 enumerated common payoffs; hand-designed common enablers and utility | Hand-level trigger rates and swept numbers for every candidate; about 1.5 promising candidates for each common the pool needs |
| 3. Commons, run level | The rescoring shop AI and its policies | A first common pool chosen on sigil metrics, standalone enabler value, and elegance; the fun score is recorded; density levers are decided here if Commitment works lags its band |
| 4. Engravings | The four-type catalog on card offers, including synthetic cards | Sigil metrics measured by engraving type |
| 5. Uncommons | Hand-designed waves of about 1.5 candidates per slot, seeded from 1.0, including shared enablers | A first uncommon pool, with enablers measured for standalone value and support for two payoff families |
| 6. Rares and legendaries | Up to 20 candidates | A first rare and legendary pool, with the power ceiling watched closely |
| 7. Validation | The full pool at tier 2 | Target bands recalibrated; tiers calibrated; the two-human configuration checked; human playtests |

- **Choosing the pool.** Simulation supplies the numbers and the evidence.
  Designers choose what to keep for strength, variety, clarity, elegance, and
  archetype coverage, using the simplicity rubric and its comparison rule.
  For example, the common payoffs are picked from the most promising
  enumerated and designed candidates.
- **Records at every stage.** A stage isn't done until the decision, parameter,
  and sigil records its experiments touched are written
  ([§16](#16-evidence-and-design-records)), so the documentation grows with
  the pool.
- **Hand play from stage 1.** The ported UI lets the designer play each
  stage's pool by hand. Hand play measures comprehension to calibrate the
  structural simplicity score, checks the feel of railroading, and catches
  AI blind spots early.
- **Mining 1.0.** Higher-rarity waves start from the 1.0 sigils that fit
  2.0's rules ([Appendix D](#appendix-e-seeds-from-rogue-[♠]s-10)).

## 15. Risks and tuning levers

| Risk | Lever |
| --- | --- |
| Purely random offers leave committed builds thin | Run pools (K random archetypes per run, never named), affinity weighting, card lean, a fourth sigil offer, cheaper rerolls |
| Late-run teams bid safe despite the bid-scaled pool | Share of bid-scaled sigils, Bid High budgets, and the set penalty |
| Low-rarity ×multipliers overshoot the ×10 curve | ×Multiplier counts and sizes per rarity |
| Long side suits get trumped, so the Suits length line fails | Hold and Lead payoffs, suit changes, partner lead selection, and synthetic cards |
| Synthetic duplicates confuse card counting | Copies are visibly synthetic when played; synthetic offer share |
| Hidden sigils make AI opponents misread builds | A prior over unrevealed sigils, built from offer odds |
| The 8-card cap is too loose or too tight for input randomness | The cap, between 6 and 10 |
| Two-human teams (16 cards) outscore single-player par | A per-mode cap; otherwise accepted as a secondary configuration |
| Rescoring undervalues enablers | Fresh rollouts through changed rules and choices; standalone trials before synergy credit |
| Tiers 0 and 1 rank sigils differently from tier 2 | Calibration at each stage; borderline sigils promoted to tier 2 trials |
| Partners knowing each other's owned cards distorts Spades play | Accepted; the AI uses the same information |
| Nil is weak in single-player, where the AI partner decides its own nils | The partner bids nil by expected score, including team sigils |
| Experiments run too slowly | Tier budgets, and fewer runs thanks to paired seeds |

## 16. Evidence and design records

The project's lasting output is its design documentation as much as the game.
Every design decision records the alternatives considered and the numbers that
chose between them. Every sigil records the evidence for its inclusion. The
harness writes most of this itself, so the evidence stays current and anyone
can reproduce it.

### Decision records

[Appendix A](#appendix-a-decision-records) holds one record per decision:

| Field | Contents |
| --- | --- |
| Starting choice | What the design does now |
| Alternatives | Every option considered, including the ones rejected in the interview |
| Prior reasoning | Arguments and numbers from before any simulation, labeled as such |
| Decided by | The metrics that will choose between the alternatives, named before any experiment runs |
| Test | The experiment: its arms, stage, AI tier, and run count |
| Evidence | Results for every arm, with 90% confidence intervals and links to experiment reports |
| Status | Hypothesis, supported, revised, or rejected |

- **Settling a decision.** A decision is supported when its choice beats every
  alternative on its deciding metrics, or ties with them and is simpler. If an
  alternative wins, the decision is revised, and the record keeps the old
  choice with its numbers.
- **Kinds of decision.** Design decisions are judged by the sigil metrics, the
  fun score, and designer judgment of elegance. Method decisions, such as the AI architecture, are judged by harness
  measurements like play quality, throughput, and agreement between tiers.
  Where simulation can't measure a question, such as rules-text style, the
  record says it rests on designer judgment and playtests.
- **Assumptions** in [Appendix C](#appendix-c-unexamined-assumptions) get a
  record when an experiment first examines them.

### Parameter records

[Appendix B](#appendix-b-parameter-register) lists every number with its
starting value, a range to test, and the metrics that set it. Once a number is
tested, its row links to the sweep that chose it, which shows the results at
every value tried.

### Sigil records

Every sigil candidate gets a record. The harness writes it into the sigil's
data file and into a generated catalog, so nobody edits evidence by hand.

| Field | Contents |
| --- | --- |
| Identity | Rules text, rarity, archetypes, and role |
| Status | Candidate, included, revised, or cut, and the stage that decided it |
| Sigil metrics | Base and committed trigger rates, win-rate lift over its control, decisive share, standalone enabler lifts against disabled and flat-points controls by purchase stage, support lift in two payoff families, and counter scans, each with confidence intervals and run counts |
| Reasoning | Why it was kept, changed, or cut, weighing the metrics and elegance |
| Standalone appeal | No-payoff playtest choice rate, sample size, and the concrete reasons players give for taking or rejecting an enabler |
| Sweep | Every amount tried, with its lift and points per round, and the amount chosen |
| Synergy | Its strongest pairs, and how far they beat the sum of their parts |
| Simplicity | Itemized complexity costs, C and S, the rubric version, and comparison with a simpler alternative after payout tuning |
| Alternatives | Other amounts, triggers, or wordings tried for the same slot, and why they lost |
| Reports | Links to the experiment reports behind every number |

The sigils in this document are illustrative and have no records yet. The
first records come from stage 2.

### Experiment reports

Every experiment writes a report listing:

- its question and the record it updates;
- its arms, configuration, AI tier, seeds, and run count;
- the code and pool versions it ran against;
- its results, with confidence intervals, the simplicity cost breakdown, and
  the net fun-score change after the complexity penalty;
- the command that reproduces it.

Records cite reports, so empirical claims trace back to runs anyone can
repeat; designer-set rubric costs and weights remain labeled as such.

### Keeping evidence current

Evidence goes stale when the rules or the pool change. Each record names the
code and pool versions behind its numbers. At each stage's exit, the harness
re-runs every record whose evidence predates a change that could affect it,
and flags any conclusion that flips.

## Appendix A: Decision records

Every record follows the format in [§16](#decision-records). No experiment has
run yet, so balance records remain hypotheses with pending evidence;
wording conventions can be chosen by designer direction.

- **Design decisions** (D1–D13 and D23–D27) are judged by the sigil metrics,
  the fun score, and designer judgment.
- **Method decisions** (D14–D21, D28, and D29) are judged by harness measurements or designer
  judgment.
- **Presentation** (D22) rests on designer judgment and playtests.

### D1. Which tricks score

- **Starting choice:** overtricks score nothing, and there are no bags. This was
  first chosen as "the team scores its B best tricks", which became 10 × B once
  [D3](#d3-scoring-categories) dropped per-trick values.
- **Alternatives:**
  - every trick scores, and every 5 bags cost 50 × the multiplier;
  - every trick scores, with no bags;
  - 1.0's rule: every trick scores, and 10 bags cost 100.
- **Prior reasoning:** under 1.0's rule the expected-score AI bid low, taking
  1.24 bags per made contract, and an 8-round run reaches 10 bags about once.
  Without bags, underbidding costs nothing.
- **Decided by:** Skill and bidding, then Close and live.
- **Test:** one arm per alternative, on plain Spades at stage 0 and with
  commons at stage 3.
- **Evidence:** pending. **Status:** hypothesis.

### D2. Nil scoring

- **Starting choice:** the nil score joins the base and is multiplied with it.
- **Alternatives:**
  - a separate nil pipeline with its own multipliers;
  - 1.0's flat ±100, which multipliers never touch.
- **Prior reasoning:** flat nil faded late in 1.0 runs, so Nil couldn't scale
  like the other archetypes.
- **Decided by:** Archetypes viable (Nil's committed win rate), and Synergy
  (pairs involving Nil).
- **Test:** Nil commitment trials under each rule at stages 3 and 5.
- **Evidence:** pending. **Status:** hypothesis.

### D3. Scoring categories

- **Starting choice:** +contract points, +contract multiplier, and ×contract
  multiplier, with ×multipliers at any rarity.
- **Alternatives:**
  - trick value, contract value, and +contract multiplier, with nothing
    compounding (chosen first, then revised);
  - those three plus ×multipliers at rare and legendary only;
  - Bridge of Rogues' five categories, including per-trick multipliers.
- **Prior reasoning:** the categories mirror Balatro's chips, +Mult, and ×Mult,
  and compounding is the main source of "broken" combos.
- **Decided by:** Synergy and combos, Close and live, and the power ceiling.
- **Test:** pool variants with and without ×multipliers at each rarity, at
  stages 3–6.
- **Evidence:** pending. **Status:** hypothesis.

### D4. Score growth

- **Starting choice:** par grows about ×10 from round 1 to round 8, from 600
  to 6,000 (parameter P20), using the starting ×10 multiplier from D25.
- **Alternatives:**
  - about ×5;
  - about ×20, as in Bridge of Rogues;
  - no fixed curve, tuned only to the closeness metrics.
- **Prior reasoning:** with constant growth per round, rounds 1–4 hold 28% of
  points at ×5, 21% at ×10, and 15% at ×20. These shares are unchanged by
  rescaling points, but the revised sigil budgets still need simulation.
- **Decided by:** Close and live.
- **Test:** budget sweeps at stages 3–6 that land the pool on each curve.
- **Evidence:** pending. **Status:** hypothesis.

### D5. Owned cards

- **Starting choice:** each player owns at most 8 cards, and all of them are
  dealt every round.
- **Alternatives:**
  - own any number of cards and receive a random 8;
  - own up to 13, as in Bridge of Rogues.
- **Prior reasoning:** without a cap, a single-player human could own a fixed
  13-card hand by mid-run.
- **Decided by:** input randomness (random cards per hand), Commitment works,
  and Close and live.
- **Test:** card-only runs with caps of 6, 8, 10, and 13 at stage 1, then the
  chosen rule again at stage 3.
- **Evidence:** pending. **Status:** hypothesis.

### D6. Engraving catalog

- **Starting choice:** a small fixed catalog of scoring engravings plus
  Synthetic, as revised by [D23](#d23-identity-engravings).
- **Alternatives:**
  - a large designed pool, like 1.0's engraving sigils;
  - identity engravings only;
  - scoring engravings only.
- **Decided by:** Commitment works and Synergy, with designer judgment on
  simplicity.
- **Test:** catalog variants as arms at stage 4.
- **Evidence:** pending. **Status:** hypothesis.

### D7. Offer density

- **Starting choice:** sigil offers are purely random within rarity.
- **Alternatives:**
  - hidden run pools of K archetypes, plus card offers that lean toward cards
    your sigils reward;
  - affinity weighting toward archetypes you already own;
  - card lean only.
- **Prior reasoning:** with about 48 sigil offers per run, a major archetype
  appears about 4 times and a minor one about twice.
- **Decided by:** Commitment works, and Archetypes viable (the spread of
  winning builds).
- **Test:** each lever as an arm at stage 3, if Commitment works lags its band
  at baseline.
- **Evidence:** pending. **Status:** hypothesis.

### D8. Bid tension

- **Starting choice:** a set loses all contract points, and at least a third of
  contract-point sigils and a quarter of multipliers scale with the contract
  (parameter P23).
- **Alternatives:**
  - event points are banked even through a set;
  - a symmetric stake, where a set loses the full contract value;
  - accept safe bidding.
- **Prior reasoning:** late in a run, bidding 5 instead of 7 lowers a made
  score only from 10,800 to 10,000 in an illustrative +200-point, ×40 build.
  If that sharply cuts set risk, an expected-score bidder can sandbag; the
  larger starting multiplier alone does not solve this incentive.
- **Decided by:** Skill and bidding.
- **Test:** rule arms at stages 3 and 5, plus a sweep of the bid-scaled share
  from none to half.
- **Evidence:** pending. **Status:** hypothesis.

### D9. Income

- **Starting choice:** base gold, plus 10 gold per trick in a made contract,
  plus interest.
- **Alternatives:**
  - 10 gold per trick won;
  - flat income;
  - catch-up gold for the trailing team.
- **Prior reasoning:** contract gold gives early rounds an economic stake and
  rewards ambitious bids, and two extra sets cost about one sigil over a run.
- **Decided by:** Close and live, then Skill and bidding.
- **Test:** income arms at stages 1 and 3.
- **Evidence:** pending. **Status:** hypothesis.

### D10. Sigil visibility

- **Starting choice:** opponents' sigils stay hidden until they first trigger.
- **Alternatives:**
  - always public;
  - hidden all run.
- **Decided by:** No invalidation and Skill and bidding, plus how
  readable playtesters find the table.
- **Test:** visibility arms at stage 3. In each arm the AI reasons only about
  what it can see.
- **Evidence:** pending. **Status:** hypothesis.

### D11. Archetype roster

- **Starting choice:** majors Suits, Spades, Ranks, Bid High, and Nil; minors
  Low Cards, Rainbow, Streaks, and Exact.
- **Alternatives:**
  - honors and length as separate archetypes;
  - seven archetypes, with Exact and Streaks cut to one-offs and Rainbow folded
    into Suits;
  - Nil as a minor.
- **Decided by:** Archetypes viable and Commitment works.
- **Test:** commitment trials for every archetype at stages 3 and 5. An
  archetype whose sigils keep falling short drops to one-offs.
- **Evidence:** pending. **Status:** hypothesis.

### D12. Suit structure

- **Starting choice:** named-suit payoffs, using [♦] as the example, with shared
  hand changes and lead control as enablers;
  18 payoff slots formerly divided between cycles and adaptive suits now
  serve named suits. No required cycles and no adaptive family.
- **Alternatives:** spread the named-suit slots across [♣], [♦], and [♥], with [♠]
  variants where useful; concentrate side-suit support on [♦]; allow one
  adaptive one-off from the generic budget.
- **Prior reasoning:** explicit suits are easier to read and track. The old
  estimate of three relevant offers per suit depended on adaptive sigils and
  no longer applies.
- **Decided by:** Commitment works and Archetypes viable for each supported
  suit, plus designer judgment on simplicity.
- **Test:** paired-seed pool variants at stage 3 with the same total slots;
  measure support offers and commitment success for each supported suit.
- **Evidence:** pending; no simulator exists yet. **Status:** hypothesis.

### D13. Effect families

- **Starting choice:** payoffs plus shared hand reshaping, lead control, and
  freedom to play cards. Every enabler must stand alone before payoffs.
- **Excluded:** shop/economy modifiers, payoff-only identity aliases,
  contract-count exceptions, information effects, randomness from bidding
  onward, and activated abilities. Seeded random Opening changes are allowed.
- **Alternatives:** the previous archetype-specific utility catalog, retained
  only as a historical experiment control; payoff-only pools.
- **Decided by:** standalone enabler value, the sigil metrics, Simplicity, and
  player judgments of whether the effect is worth taking before a combo.
- **Test:** stages 2–6 compare each new candidate alone, beside a generic
  payoff, and as support for two payoff families; no family gets a quota.
- **Evidence:** pending. **Status:** hypothesis.

### D14. Metric structure

- **Starting choice:** per-sigil metrics with target bands plus a weighted fun
  score: Close and live 25, Commitment works 15, Archetypes viable 15,
  Synergy 15, Skill and bidding 15, and Simplicity 15. Both are targets for
  continuous optimization over rounds of iteration, weighed with designer
  judgment of elegance; neither is a pass/fail bar. Mechanical complexity is
  scored from sigil data, with a positive cost for every numeric value and
  boolean condition; more complex replacements need a demonstrated net
  improvement.
- **Alternatives:**
  - per-sigil pass/fail gates plus the fun score (the previous starting
    choice, revised by designer direction on 2026-10-06: design is a
    continuous optimization, and black-and-white bars misjudge pieces that are
    weak on one metric but valuable overall);
  - one weighted score with no per-sigil metrics;
  - the same metrics, with weights that favor synergy;
  - the previous five-family score with simplicity only as a diagnostic.
- **Decided by:** designer judgment for the starting rubric and weights,
  calibrated against comprehension playtests and simulation comparisons of
  simpler and more complex candidates; revisit at stage 7 if the score
  disagrees with playtests.
- **Evidence:** pending. **Status:** hypothesis.

### D15. Sigil representation

- **Starting choice:** declarative data, with generated text and named rule
  hooks.
- **Alternatives:**
  - 1.0's per-sigil handlers;
  - handlers that read their numbers from a parameter table.
- **Decided by:** whether the grammar can express every candidate worth
  testing, and how fast number sweeps run.
- **Evidence:** pending. **Status:** hypothesis.

### D16. AI architecture

- **Starting choice:** one rules kernel shared by the simulation and the AI
  search, in Rust ([D29](#d29-simulator-language)); the later playable game
  must share its rules.
- **Alternatives:**
  - patch 1.0's plain-Spades search;
  - search with perfect information in simulation.
- **Decided by:** AI play quality and throughput at stage 0.
- **Evidence:** pending. **Status:** hypothesis.

### D17. Shop AI

- **Starting choice:** value offers with a model fitted to each round's grant
  tournament, refit every round, with random exploration. Rescoring sampled
  hands with and without an offer checks the model on a subsample.
- **Alternatives:**
  - rescoring in every shop, the previous starting choice. It was revised on
    2026-10-06 by designer direction: about 200 hand simulations per shop
    visit dwarf the run itself and make measuring the whole pool every round
    unaffordable;
  - hand-written heuristics.
- **Decided by:** rank agreement with rescoring on a subsample, and shop skill
  at stage 3: on paired boards, the model shopper should match or beat the
  rescoring shopper.
- **Evidence:** pending. **Status:** hypothesis.

### D18. Throughput

- **Starting choice:** three AI tiers. Bulk tournaments run at tier 0 and are
  corrected toward tier 1 by a calibration subset on shared seeds. Experiments
  are sized to precision targets from measured throughput (parameters P24 and
  P27).
- **Alternatives:**
  - fixed 2,000-run paired experiments at tier 1, the previous starting
    choice;
  - full search only, with experiments run overnight;
  - heuristic bots, plus full-search spot checks.
- **Decided by:** agreement between tiers (whether tiers 0 and 1 rank sigils
  the way tier 2 does) and measured run times.
- **Evidence:** pending. **Status:** hypothesis.

### D19. Simulation configuration

- **Starting choice:** balance for single-player first, with one card-owning
  seat per team.
- **Alternatives:**
  - balance for teams of two humans first;
  - AI teams always own cards in both seats.
- **Decided by:** product priority. The two-human configuration is checked at
  stage 7.
- **Evidence:** pending. **Status:** hypothesis.

### D20. Staging

- **Starting choice:** kernel, cards, commons (hand level, then run level),
  engravings, uncommons, rares, then validation, with the UI port at stage 1.
- **Alternatives:**
  - UI after the commons pass;
  - UI only at the end.
- **Decided by:** designer judgment.
- **Evidence:** not applicable. **Status:** hypothesis.

### D21. Candidate generation

- **Starting choice:** enumerate common payoffs from the grammar, and design
  higher rarities by hand in waves seeded from 1.0. The
  [sigil design plan](sigil-design-plan.md#commons-two-sources) runs both
  sources side by side at common, with designers blind to the enumeration,
  and records how each source's candidates measured and how many each
  supplied to the draft pool.
- **Alternatives:**
  - hand-designed waves at every rarity;
  - enumeration at every rarity.
- **Decided by:** how many candidates from each source end up in the pool,
  and how they score on the sigil metrics.
- **Evidence:** the commons draft pass
  ([outcome](sigils/rounds/draft-common.md#outcome)): of 66 kept commons, 17 are GDD
  seeds, 43 designed, and 6 enumerated-only; the enumerator independently produced 32 of the 66
  kept signatures. Designers supplied most decision-rich shapes and every new rule hook.
- **Status:** supported for commons: both sources are worth running; the enumerator is most
  useful as a coverage check on the designed pool.

### D22. Rules text

- **Starting choice:** benefit first, then condition; explicit "your team";
  no redundant timing or shop scope; bracketed ranks and suits; `+15` for
  additive multipliers; `Opening:` on all before-bidding effects. The full
  templates and timing defaults are in [§5](#rules-text).
- **Alternatives:** 1.0's timing-first "gain"; the previous "add" templates;
  colon shorthand.
- **Decided by:** the designer's 2026-10-06 wording preference, checked for
  comprehension in playtests. Wording alone is not a balance claim.
- **Evidence:** designer direction. **Status:** text convention chosen;
  behavioral simplifications remain hypotheses under D24.

### D23. Identity engravings

- **Starting choice:** synthetic offers copy only [J], [Q], [K], or [A], replacing an
  unowned [2]–[9] of the same suit. The offer shows its resulting identity and
  "Replaces [4♣]"; the engraving never changes that printed identity in play.
- **Alternatives:** the earlier unrestricted synthetic copies; the original
  identity catalog of Converted, Raised, Crowned, and Wild.
- **Prior reasoning:** synthetic cards upgrade ranks without moving cards
  between suits, and the replacement label says only what the player needs.
- **Decided by:** the designer's same-suit and [J]-or-higher constraints, then
  Commitment works, Archetypes viable, and the power ceiling for balance.
- **Test:** stage 4 sweeps premiums, offer share, replacement rank range, and
  [J]/[Q]/[K]/[A] weights within those constraints, including Rainbow and Nil runs.
- **Evidence:** pending; no simulator exists yet. **Status:** hypothesis.

### D24. Simpler sigil conditions

- **Starting choice:** remove "beyond N" arithmetic, adaptive suit tracking,
  separate partner-result checks, and compound trick-sequence restrictions.
  Keep whole-count payouts and simple milestones; tune numbers first.
- **Alternatives:** the previous restricted candidates as experiment controls;
  smaller payouts or fewer candidates if the simple versions dominate.
- **Decided by:** the sigil metrics and the weighted fun score, especially the
  power ceiling, Skill and bidding, Commitment works, and the explicit
  Simplicity cost in family 9.
- **Test:** paired-seed comparisons in stages 2–6, sweeping payout, price, and
  rarity, recording both mechanical complexity and the net fun-score change.
  In particular, compare the former +15 per [♠] beyond five with whole
  holdings payouts starting at +5 per [♠]; also remeasure the revised Spades,
  Bid High, Rainbow, Streaks, and Exact candidates. Cut failures instead of
  restoring fiddly clauses.
- **Evidence:** pending; no simulator exists yet. All revised amounts are
  illustrative, not validated balance. **Status:** hypothesis.

### D25. Multiplier scale

- **Starting choice:** start at ×10, with repeated-event +mult rewards around
  +2 to +5 and one-time rewards around +10 to +20. Leave contract points,
  nil amounts inside the formula, ×mult factors, and the economy on their
  existing scales. Plain Spades scores become ten times larger.
- **Alternatives:** the previous starting ×1 with +1/+2 rewards; starting ×1
  with larger rewards; other starting multipliers paired with tuned rewards.
- **Prior reasoning:** +15 on a starting 10 increases a score by 150%, while
  +15 on a starting 1 increases it by 1,500%. Larger numeric ranges allow
  smaller proportional balance adjustments without extra conditions; +12 and
  +15 incur the same numeric simplicity cost.
- **Decided by:** the sigil metrics and the fun score, particularly the power ceiling,
  Close and live, Skill and bidding, and Simplicity.
- **Test:** paired-seed sweeps at stages 3–6 of the starting multiplier and
  payout amounts, including points and ×mult alternatives. Compare score
  growth relative to each arm's unmodified score scale, first-+mult purchase
  advantage, early-round contribution, and set losses; ensure simple rewards
  can balance without restrictive riders.
- **Evidence:** pending; no simulator exists yet. The arithmetic examples are
  checks of the formula, not empirical balance results. **Status:** hypothesis.

### D26. Enablers worth taking before payoffs

- **Starting choice:** retire the old enabler and utility catalog and its
  55-slot quota. Start with seven shared candidates that change actual hands,
  lead control, or legal plays, leaving the 90-payoff allocation intact.
- **Alternatives:** the old per-archetype utility quota as a control;
  payoffs plus owned cards with no enabler sigils.
- **Prior reasoning:** tiny discounts and offer adjustments may give a positive
  win-rate lift without being appealing purchases. Payoff-only card aliases
  have no value before the matching payoff. Mandatory archetype slots reward
  filler, while strong shared effects can support several builds naturally.
- **Decided by:** standalone win-rate lift, competitiveness with a generic
  payoff, support for two payoff families, the power ceiling, the fun score,
  and no-payoff playtest choices. Invented terms pay an additional complexity
  cost, especially at common.
- **Test:** the staged comparisons in [§12](#standalone-enabler-value), followed
  by full-pool trials against the payoff-only arm. Remeasure offer coverage
  and synergy targets with the smaller pool; do not count a rewritten rule
  as empirically validated until those trials run.
- **Evidence (2026-10-07):** standalone trials each round. Most enablers beat a
  blank by 10–38 pts when bought at shop 1 but lose to a same-price flat-points
  control; narrow play-freedom and card-strength rules beat it. Lead choice,
  late any-suit play, and swaps failed at every rarity
  ([final report](sigils/final-report.md)).
- **Status:** partly supported: hand-changing enablers have standalone value but rarely beat flat points.

### D27. Opening and random card changes

- **Starting choice:** `Opening:` means after dealing and before bidding.
  Fixed-count changes use "X cards your team holds become Y", choosing X
  distinct cards uniformly from both hands without replacement and applying
  the specified rank or suit automatically. All current rank and suit changes
  move to this window. Whole-hand rank increases affect every card.
- **Alternatives:** the previous holder-selected changes, some after bidding;
  excluding cards already matching the destination from the random pool.
- **Prior reasoning:** the same short template exposes the size and result of
  an effect. Random selection is input randomness that resolves before any
  bid; it may hit existing winners or cards already at the destination.
- **Decided by:** standalone enabler value, the sigil metrics and the fun score,
  including input-randomness diagnostics and the full simplicity cost of
  Opening, counts, ranks, filters, and any hidden category definitions.
- **Test:** paired-seed trials with the dedicated Opening streams, sweeping
  counts and rank increases and measuring standalone value, nil feasibility,
  bid choices, and how often changes help, hurt, or leave a card unchanged.
  Compare only after players have seen their own resulting hands.
- **Evidence:** pending; no simulator exists yet. **Status:** hypothesis.

### D28. Measurement design

- **Starting choice:** grant tournaments. These are normal games where both
  teams shop, plus random forced purchases at full price, played as duplicate
  boards with seats swapped. One regression over all boards estimates every
  sigil's value at once, with terms for amount, acquiring shop, coherence, and
  pairs, empirical-Bayes shrinkage, and cluster-bootstrap intervals.
  Commitment, standalone, pair-confirmation, and build-chaser arms cover what
  single-sigil estimates can't. The
  [sigil design plan](sigil-design-plan.md#the-measurement-engine) has the
  detail.
- **Alternatives:**
  - separate paired forced-pick trials for each sigil, the previous design;
  - fully random loadouts with no sigil shopping;
  - estimates from ordinary shop purchases.
- **Prior reasoning:** these are estimates, made before any measurement.
  - A 2,000-run paired trial resolves about ±2 win-rate points. One trial per
    sigil and purchase stage costs more than a round's budget, while each
    tournament board informs about six sigils.
  - Shop-chosen purchases are confounded with being ahead.
  - Fully random loadouts ignore building around a sigil, and undervalue
    payoffs that need support.
- **Decided by:** pipeline validation (planted effects recovered, and the
  same sigil under two ids estimated alike), interval width per run, and
  agreement with forced-pick trials on a sample of sigils.
- **Test:** the pilot in Phase 0 of the plan, plus forced-pick trials on
  about ten sigils compared with their tournament estimates.
- **Evidence:** the [pilot](../reports/phase-0-pilot.md) recovered planted
  effects monotonically, covered 86% of a perturbed synthetic truth with 90%
  intervals, and passed an A/A test. Two design changes were needed: duplicate
  boards swap offer luck as well as seats, and grants hold their slot for the
  run (otherwise the shop sold them and standard errors tripled). Forced-pick
  trials were replaced by tier-2 spot checks (correlation 0.84 with the working
  estimates).
- **Status:** supported, with the changes recorded in
  [plan deviations](sigils/plan-deviations.md).

### D29. Simulator language

- **Starting choice:** a native Rust simulator for the kernel, AI, shop, and
  runner, with Python for analysis and a TypeScript sigil viewer. The playable
  game is deferred. When built, it must share the kernel's rules, through WASM
  or a port checked by replaying seeds.
- **Alternatives:**
  - TypeScript on Node worker threads, as previously planned;
  - a Rust core compiled to WASM for the game from the start;
  - TypeScript first, porting the hot path only if it proves slow.
- **Prior reasoning:** tier 1 at about 3 seconds per run needs roughly 700,000
  card plays per second per core. Measuring the whole pool every round needs
  far more runs than per-sigil trials did. This is an estimate, not a
  measurement.
- **Decided by:** designer direction on 2026-10-06, and measured throughput in
  Phase 0 of the plan.
- **Evidence:** pending. **Status:** hypothesis.

### D30. Pool shape

- **Starting choice:** soft pool targets of about 145 sigils, about 60 common,
  60 uncommon, 20 rare, and 5 legendary (each within about ±25%), following
  Balatro's joker pool. They replace §10's slot tables and the pool sizes in
  §5's rarity table. Mix, archetype coverage, bid tension, named suits, and
  ×multiplier placement are soft targets in the
  [sigil design plan](sigil-design-plan.md#soft-pool-targets); a shortfall is
  a coverage hole that an optimization round may work on, not a quota.
- **Alternatives:**
  - the 97-candidate skeleton of §10 (90 payoffs and 7 enablers) with its slot
    tables;
  - hard per-archetype quotas.
- **Prior reasoning:** fixed slot tables reward filler; a larger, softer pool
  gives runs more variety while each piece still has to earn its place by the
  sigil metrics and elegance.
- **Decided by:** designer direction on 2026-10-06; the fun score's
  Commitment works and Archetypes viable families check the result.
- **Test:** the draft passes and optimization rounds of the
  [sigil design plan](sigil-design-plan.md).
- **Evidence:** the draft passes kept 140 sigils; three optimization rounds
  trimmed and reshaped them to 117 (49 common, 43 uncommon, 21 rare, 4 legendary),
  within P21's range but under the 145 target. Fun score by round: 55.8, 47.6,
  51.5 ([final report](sigils/final-report.md)).
- **Status:** supported as a target shape; the uncommon tier and the total remain
  below target.

## Appendix B: Parameter register

Every number below is a starting value. Each row lists the range to test, the
metrics that set it, and the stage that tests it. Once tested, a row links to
the sweep that chose its value ([§16](#parameter-records)).

| # | Parameter | Starting value | Range to test | Decided by | Stage |
| --- | --- | --- | --- | --- | --- |
| P1 | Rounds per run | 8 | 6–10 | Close and live | 3 |
| P2 | Owned cards per player | 8 | 6–10, and 13 | Input randomness; Commitment works | 1 |
| P3 | Sigil slots per team | 7 | 5–9 | Synergy; Archetypes viable | 3 |
| P4 | Sigil offers per shop | 3 | 2–5 | Commitment works | 3 |
| P5 | Card offers per shop | 3 | 2–5 | Commitment works | 1 |
| P6 | Purchases per shop | Unlimited | 1, 2, or unlimited | Commitment works; economy diagnostics | 3 |
| P7 | Reroll cost | 50, plus 10 per further reroll | 25–100 to start | Commitment works | 3 |
| P8 | Sell value | Half price, rounded down to a multiple of 5 | 25–75% | Economy diagnostics | 3 |
| P9 | Starting gold | 150 | 100–250 | Close and live (early share) | 1 |
| P10 | Base income | 100 per round | 50–150 | Economy diagnostics | 1 |
| P11 | Contract gold | 10 per trick in a made contract | 0–20 | Skill and bidding; Close and live | 1 |
| P12 | Nil gold | 50 per made nil | 0–100 | Archetypes viable (Nil) | 3 |
| P13 | Interest | 10 per 50 held, up to 50 | Cap of 0–100 | Economy diagnostics | 1 |
| P14 | Rarity odds | 69 / 25 / 5 / 1% | Each tier halved or doubled | Commitment works; power ceiling | 3 |
| P15 | Sigil prices | 50 / 75 / 100 / 150 | ±50% | Economy diagnostics; Archetypes viable | 3 |
| P16 | Card prices | The [§6](#prices) table | Set from each card's simulated value | Value per gold roughly equal across cards | 1 |
| P17 | Engraved share of card offers | 25% at shop 3, plus 10 points per shop | 0–100% | Commitment works | 4 |
| P18 | Engraving premiums | 25, or 50 for Multiplier | 0–100 | Economy diagnostics | 4 |
| P19 | Synthetic replacement range and copied ranks | Unowned [2]–[9] of the same suit; copies [J]/[Q]/[K]/[A] | Replacement ranks below [J]; weights across [J]/[Q]/[K]/[A] | Input randomness; Commitment works; Archetypes viable; power ceiling | 4 |
| P20 | Par growth over a run | ×10, from 600 to 6,000 | ×5–×20 | Close and live | 3–6 |
| P21 | Pool sizes and category split | About 145: 60 / 60 / 20 / 5, each within about ±25%; about two-thirds mainly scoring ([D30](#d30-pool-shape)) | 110–180 in total | Commitment works; Archetypes viable | 3–6 |
| P22 | Nil base value before multipliers | ±100 | 50–150 | Archetypes viable (Nil) | 0 |
| P23 | Bid-scaled share of the pool | A third of contract-point sigils, a quarter of multipliers | None to half | Skill and bidding | 3–5 |
| P24 | AI tier budgets | Search budgets as in [§13](#ai-tiers); at least 100, 5, and 0.5 runs per second on 18 cores. Measured in Phase 0 with the shop and seed pool: about 1,000, 115, and 16 runs per second ([pilot](../reports/phase-0-pilot.md)) | Per tier | Agreement between tiers; run time | 0 |
| P25 | Fun score weights | 25 / 15 / 15 / 15 / 15 / 15 (families 4–9) | Any | Designer judgment, checked against playtests | 7 |
| P26 | Sigil-metric target bands and fun score bands | As in [§12](#12-metrics-what-fun-means). Checked after the first full-pool measurement and in the final round: the median pool lift sat within 0.3 pts of the control both times, so the bands were not re-centered ([round 3](sigils/rounds/round-3.md)) | Any | Calibrated once the harness runs, and again at stage 7 | 3, 7 |
| P27 | Experiment precision | Median 90% interval half-width of 1.5 win-rate points per sigil. Pilot: residual sd of the smoothed board outcome 0.25 at tiers 0 and 1; about 167,000 tier-0 boards (6 minutes) reach the target for 170 sigils; tier-0 and tier-1 lifts correlate 0.82 ([pilot](../reports/phase-0-pilot.md)) | 1–3 points | Run time; decision quality | 0 |
| P28 | Simplicity rubric | Number/rank/check 1; arithmetic rider 2; selector/state 1; new term including Opening 2, or 4 at common; S = 1 / (1 + C) | Positive costs; compare 1–3 per burden; twice the term cost at common; expand aliases and rank classes before scoring | Designer judgment, calibrated by comprehension playtests and paired simulation tradeoffs | 2–7 |
| P29 | Starting contract multiplier and +mult scale | Start at 10; +2–5 repeated rewards, +10–20 one-time rewards | Starting values 1, 5, 10, 15; sweep rewards jointly | Power ceiling; Close and live; Skill and bidding; Simplicity | 3–6 |
| P30 | Standalone enabler acceptance | 90% lower bounds: ≥3 percentage-point lift over disabled effect; ≥−2 versus same-price flat payoff; ≥40% no-payoff playtest preference; support two families | Lift 1–5 points; comparison tolerance 0–3 points; preference 30–60% | Material benefit; standalone appeal; breadth; power ceiling | 2–7 |
| P31 | Opening card changes | Four random cards become [2]s, [K]s, or [♥]s; all-card increase of two ranks | Random count 1–6; rank increase 1–3; named destination variants tested separately | Standalone enabler value; bidding; input randomness; power ceiling | 2–7 |

## Appendix C: Unexamined assumptions

These starting assumptions were made without a dedicated question. They are
hypotheses like everything else, and each gets a decision record when an
experiment first examines it.

- A set team scores (−10 × B + nil score) × its multipliers and loses all
  contract points. Multipliers whose condition requires making the contract
  don't count.
- Blind nil is excluded from the rules and current sigil pool.
- 1.0's other base rules stay: individual bids add up to the team contract,
  nil, [♠]s must be broken, the first lead comes from the dealer's left, and
  the deal rotates.
- Partners know each other's owned cards; opponents don't. The AI gets exactly
  the same information.
- Archetypes are never named or tagged in the game.
- ×Multipliers compound and apply last.
- The shop has 3 sigil and 3 card offers and unlimited purchases. A reroll
  refreshes all offers for 50 + 10 per further reroll. Selling is at half
  price, and a team is never offered a sigil it owns.
- A shop opens before round 1 with 150 starting gold, for 8 shops per run.
- The simplicity rules in [§5](#simplicity-rules).
- Every random draw comes from a named, seeded stream. Random transformations
  happen during Opening, before bidding; none occur after bids begin.
- Contract points earned on overtricks count. Contract points from tricks a
  nil bidder wins don't. A double-nil team's contract points are lost.
- Nil points go to made nils rather than to the contract, so they survive a
  set.
- Sigil text spells out bids of 8 or more, ranks [2]–[10], and consecutive wins;
  invented shorthand carries an additional complexity penalty, doubled at
  common. Internal archetype names are not player-facing terms.
- A multiplier from a repeated trigger stacks each time it triggers, unlike
  1.0's once-per-round cap.
- Ties go to the first card played.
- A hidden sigil reveals itself the first time it changes a score, a legal
  play, or a trick's winner.
- Engravings are hidden from opponents until their card is played, and are
  destroyed when the card is sold. A gold engraving was left out.
- Wild left with the other suit- and rank-changing engravings, taking its
  special rules with it. Rainbow now leans on synthetic cards and random
  Opening suit and rank changes that also affect ordinary Spades hands.
- A synthetic offer is [J], [Q], [K], or [A] and replaces a random unowned [2]–[9] of the
  same suit, named on the offer. Selling it puts the replaced card back.
- A run is one 8-round match. Equal totals draw, scores can go negative, and
  meta-progression is out of scope.
- Each AI team's card-owning seat is chosen at random per run. On a team of
  two humans, each offer's tag is random.
- Rarity odds are 69/25/5/1, with legendaries in normal offers; prices are
  50/75/100/150.
- Named suits replace the adaptive suit family; any adaptive one-off needs
  its own explicit definition and evidence.
- Hold sigils count both partners' hands after Opening and bidding.
- Both teams may own the same sigil.
- Income is 100 base gold, plus 10 per contract trick if made, plus 50 per
  made nil, with interest of 10 per 50 held up to 50.
- Lead triggers fire on the lead, whether or not the trick is won.
- In multiplayer, the first partner to act on an offer gets it, and a team's
  shop closes when both partners press Done.
- Conflicting rule benders resolve by a fixed priority defined in the kernel.

## Appendix D: One-offs, not archetypes

The brief judged these ideas weak as archetypes, so they appear only as single
sigils from the generic budget.

| Idea | As a one-off |
| --- | --- |
| Throwing off cards | A generic discard payoff or two |
| While held | At most one or two sigils; no engraving uses it |
| Your partner winning tricks | Mostly moot, since team-scoped sigils treat both partners' tricks alike |
| Bonus chaser | Generic contract-point sigils |
| Contract attacker | A few payoffs ending "if the opponents miss their contract"; the No invalidation metric applies |

## Appendix E: Seeds from Rogue Spades 1.0

These 1.0 payoffs seed the hand-designed waves; the previous enabler and
shop-utility seeds are removed. The historical 1.0 text below is quoted
unchanged; every 2.0 candidate must use [§5](#rules-text), simplify
its conditions under D24, and earn its place in simulation; enablers follow D26. Colors and resonances are dropped.

| 1.0 sigil | 1.0 text | 2.0 seed |
| --- | --- | --- |
| Headsman's Axe (RE-C08) | Whenever you win a trick with a spade, gain +10 contract value. | Named-[♠] Win payoff |
| Schooling Fish (GR-C13) | If you win three or more tricks with diamonds in a round, gain +40 contract value. | Named-[♦] milestone payoff |
| Evening Melody (GR-R03) | If you win three or more tricks with hearts in a round, gain +1× contract multiplier. | Named-[♥] multiplier candidate |
| Laurel Finale (DU-R01) | If your team wins the last trick of a round with a heart, gain +1× contract multiplier. | Streaks or Suits |
| Rising Flame (RE-C10) | Whenever you win a trick after winning the previous one, gain +10 contract value. | Streaks common |
| Early Sprint (RE-C05) | Whenever you win one of the first three tricks, gain +10 contract value. | Streaks common |
| Honest Ruler (BL-C10) | If your team makes its contract exactly, gain +30 contract value. | Exact common |
| True Aim (BL-R04) | If your team makes its contract exactly, gain +1× contract multiplier. | Exact +mult |
| Balanced Yin-Yang (DU-S13) | If you take exactly your own bid, gain +1× contract multiplier. | Exact: "×1.5 contract multiplier if your team makes its contract exactly" |
| Square Meal (GY-R04) | After bidding, if your team's contract is 9 or more tricks, gain +1× contract multiplier. | Bid High: "+15 contract multiplier when your team bids 8 or more" |
| Gambler's Wheel (OR-R01) | If your team's contract is 10 or more tricks, gain +2× contract multiplier. | Bid High uncommon +mult |
| Planner's Whiteboard (GY-C18) | Gain +5 contract value for each trick your partner bids. | Bid High: "+5 contract points for each trick in your team's contract" |
| Vigil Candle (PU-C10) | Gain +30 nil value. | Nil: "+50 nil points" |
| Widening Circle (PU-R04) | Your nil value is doubled. | Nil ×mult |
| Tricolor Triangle (RE-U07) | If you lead three different suits in a round, gain +50 contract value. | Rainbow lead payoff |
| Clean Bullseye (BL-U03) | If you win four or more tricks with cards that aren't spades in a round, gain +1× contract multiplier. | Suits or Rainbow +mult |
| Ticking Bomb (RE-R02) | Each round, once you've trumped three times, gain +1× contract multiplier. | Spades +mult |
| Crown Jewel (RE-C07) | When this card wins a trick, gain +25 contract value. | Bonus engraving |
| Opening Bell (OR-C02) | When you lead with this card, gain +20 contract value. | Herald engraving |
