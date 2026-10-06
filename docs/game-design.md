# Rogue Spades 2.0: game design

Rogue Spades 2.0 is a roguelike built on partnership Spades. Two teams of two
play eight rounds. Before each round, each team shops for **sigils** (up to
seven team-wide scoring and rule-changing pieces) and **cards**. A bought card
is dealt to its owner every round, and later cards carry **engravings**. The
team with more points after round 8 wins.

One rule outranks every other: **simulation decides**. Strong AI players play
thousands of simulated runs. A rule, sigil, or number stays in the game only if
those runs show it meets the weighted fun metrics in
[§12](#12-metrics-what-fun-means).

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
- **Metric thresholds and weights** are placeholders until the harness runs.

A decision is settled only when its record shows the alternatives and the
numbers that chose between them. A sigil enters the game only when its record
shows the evidence for its inclusion
([§16](#16-evidence-and-design-records)).

## Starting design at a glance

Every row is a starting hypothesis.

| Topic | Starting hypothesis |
| --- | --- |
| Golden Rule | A rule, sigil, or number is kept only if simulation shows it meets the metrics ([§12](#12-metrics-what-fun-means)) |
| Run | 8 rounds; each round is a shop, then a deal of Spades |
| Spades | Partners' bids add up to a team contract; nil; spades must be broken; no blind nil; no bags |
| Scoring | (10 × contract + contract points + nil score) × (1 + Σ +multipliers) × Π ×multipliers; overtricks score nothing |
| Failure | A set team loses its contract points and scores (−10 × contract + nil score) × its multipliers |
| Victory | Most points after round 8; equal totals draw |
| Sigils | Up to 7 per team, shared by both partners; hidden from opponents until they first trigger |
| Cards | A bought card is dealt to its owner every round; at most 8 owned per player; later offers include synthetic Jacks, Queens, Kings, and Aces that replace lower cards of the same suit |
| Engravings | Four fixed types on card offers from shop 3, one of them Synthetic; never bought alone, and never change a card's suit or rank |
| Shop | 3 sigil and 3 card offers per team; unlimited purchases; purely random offers within rarity |
| Income | 100 base gold + 10 per trick in a made contract + interest |
| Archetypes | Majors: Suits (♣ ♦ ♥), Spades, Ranks, Bid High, Nil. Minors: Low Cards, Rainbow, Streaks, Exact. Never named in the game |
| Simulation | Declarative sigils on one rules kernel shared by the game and the AI; three AI tiers; 2,000-run experiments in about 10 minutes |
| Prototype | Single-player: you and an AI partner against an AI team |

## Design pillars

1. **Simulation decides, and the evidence is written down.** Design questions
   are answered by experiments, not intuition. Every decision records the
   alternatives considered and the numbers that chose between them, and every
   sigil records the evidence for its inclusion. This document defines fun as
   measurable metrics, and the game is built so the AI can play, bid, buy, and
   measure a sigil the moment it is written.
2. **Real Spades underneath.** With no sigils and no cards, a round plays and
   scores as partnership Spades without bags.
3. **Direct scoring.** Most sigils say "you get points for doing X" in one of
   three Balatro-style categories, and builds come from stacking them. Every
   payoff comes with **enablers** that make X happen more often.
4. **Simple pieces, emergent combos.** Each sigil does one thing. Archetypes
   are an internal design tool and are never named to players. Sigils share
   plain nouns such as aces, hearts, and the last trick, so players discover
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
3. **Before bidding.** Pre-bid card movement sigils resolve, such as passing a
   card to your partner.
4. **Bid.** Starting left of the dealer, each player bids nil or 1–13.
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
- **Blind nil** is not part of the base rules. A one-off sigil can grant it
  ([Appendix C](#appendix-d-one-offs-not-archetypes)).

### Play

- Players must follow suit if able. A player who can't may play any card.
- Spades are trump. The highest spade wins; otherwise the highest card of the
  led suit wins.
- Spades can't be led until a spade has been played on another suit, unless
  the leader holds only spades.
- **Effective cards.** Rule benders can change a card's rank or suit, as in
  "Your team's nil bidders' Aces become twos after bidding". A changed card counts at
  its new identity everywhere: following suit, winning tricks, and sigil
  conditions. Ranks cap at ace.
- **Ties.** [Synthetic cards](#synthetic-cards) and rule benders can put two
  cards of the same suit and rank in one trick. Between equal cards, the first
  one played wins.

## 4. Scoring

> **Status:** provisional. The formula comes from [D1](#d1-which-tricks-score),
> [D2](#d2-nil-scoring), [D3](#d3-scoring-categories), and
> [D8](#d8-bid-tension), and the par curve from [D4](#d4-score-growth). Every
> number is a parameter in [Appendix B](#appendix-b-parameter-register).

### The formula

```
round score = ( 10 × B               base contract
              + contract points      added by sigils and engravings this round
              + nil score )          each made nil: +100 plus nil points; each failed nil: −100
            × (1 + Σ +contract multipliers)
            × Π ×contract multipliers
```

With no sigils this is Spades without bags: a made contract of 7 scores 70, and
a set scores −70.

### Three categories

| Category | Balatro analog | Example (not final) |
| --- | --- | --- |
| +Contract points | Chips | "+20 contract points when your team wins a trick with an Ace" |
| +Contract multiplier | +Mult | "+1 contract multiplier when your team wins the last trick of a round" |
| ×Contract multiplier | ×Mult | "×1.5 contract multiplier if your team makes its contract exactly" |

- **+Contract multipliers add up.** Two +1 sigils make ×3.
- **×Contract multipliers compound** and apply last, so their order never
  matters. They can appear at any rarity, as in Balatro, and the
  [skeleton](#10-sigil-pool-skeleton) sets how many sit at each one.
- **Contract points accumulate during the round.** They come per event
  ("when your team wins a trick with an Ace") or once ("when your team bids a
  high contract").
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
| Bids 4 + 3, win 9, no sigils | 70 | 70 |
| Bids 4 + 3, win 6 | −70 | −70 |
| Bid 7, win 8; "aces: +20" fires twice; one +1 holds | (70 + 40) × 2 | 220 |
| Same, but win 6 | −70 × 2 | −140 |
| Partner bids 5 and wins 6; you make nil; "+50 nil points"; one +1 | (50 + 150) × 2 | 400 |
| Same, but your nil fails | (50 − 100) × 2 | −100 |
| Late run: bid 9 and make it; 210 contract points; +3 from sigils; one ×1.5 | (90 + 210) × 4 × 1.5 | 1,800 |
| Same, set, with every multiplier's condition still met | −90 × 4 × 1.5 | −540 |

### Keeping bids tense

A set loses every contract point, while each extra bid trick adds only 10
before multipliers. Left alone, large point totals would push late-run teams to
bid safe and drain the tension from the bidding. The pool is built against
this:

- At least a third of contract-point sigils and a quarter of multipliers scale
  with or require the contract size: "+5 contract points for each trick in
  your team's contract" and "+1 contract multiplier when your team bids a
  high contract".
- Bid High is a major archetype ([§9](#bid-high)).
- The Skill and bidding family of the fun score watches late-run overtricks and
  set rates ([§12](#fun-score)).

### Par curve

Par is the typical round score of a reasonably built team, averaged over makes
and sets. It grows about ×1.39 per round, ×10 over the run:

| Round | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Par | 60 | 85 | 115 | 160 | 225 | 310 | 430 | 600 |

- Rounds 1–4 make up about a fifth of a run's points. Early play still counts,
  while builds still visibly grow.
- Round 1 is nearly plain Spades.
- A 7-sigil build would naturally reach ×20 or more, so sigil budgets
  ([§10](#budgets)) are swept toward this curve in simulation.

## 5. Sigils

> **Status:** provisional. Visibility is [D10](#d10-sigil-visibility), the
> effect families [D13](#d13-effect-families), and rules text
> [D22](#d22-rules-text). Slots, rarity odds, and prices are
> [parameters](#appendix-b-parameter-register) P3, P14, and P15.

### Slots and ownership

- Sigils belong to the team, **up to 7 per team**. Buying one needs a free
  slot, so a full team sells first. Utility sigils can add slots.
- Sigil text explicitly says "your team". A trick either partner wins counts
  as a team win, except tricks won by a nil bidder.
- A team is never offered a sigil it already owns. Both teams may own the same
  sigil.
- A sigil sells for half its price, rounded down to a multiple of 5.
- Sigils never have activated abilities. A sigil may offer a choice at a fixed
  moment, such as "Pass a card to your partner before bidding".

### Terms

These terms extend the standard Spades vocabulary, following the terms table of
1.0's [rules-text guide](../../rsp/docs/rules-text.md#terms).

| Term | Meaning |
| --- | --- |
| **your team** | Both partners. Team wins count tricks either partner wins, except tricks a nil bidder wins |
| **contract points** | Points added to your team's contract, paid only if the contract is made |
| **contract multiplier** | Written `+1` when it adds up with others, or `×1.5` when it compounds. It multiplies your team's round score, made or set |
| **nil points** | Points added to each nil your team makes. A failed nil still costs 100 |
| **your team holds** | Both partners' hands after bidding |
| **high contract** | A team contract of 8 or more |
| **low card** | A two through a ten |
| **streak** | Tricks your team wins in a row |

### Visibility

- Partners share every sigil, the team's gold, and knowledge of each other's
  owned cards.
- An opponent's sigil is hidden until the first time it changes a score, a
  legal play, or a trick's winner. It then stays revealed for the rest of the
  run. Hidden sigils work normally.
- Both teams' scores and gold are public. Owned cards are hidden from opponents
  until they're played.

### Categories and families

The three point categories are **payoffs**. Utility comes in three families:

- **Economy and shop:** gold, interest, cheaper or free rerolls, extra offers,
  and better rarity odds.
- **Enablers:** effects aimed at one archetype that make its payoffs trigger
  more often ([§9](#payoffs-and-enablers)).
- **Rule benders and deal control:**
  - Rule benders change how cards play, one rule each, at uncommon and above:
    "Your team's Aces can't be trumped" or "Your team can lead ♠ at any time".
  - Deal control is pre-bid card movement between partners.

Two families are excluded:

- **Information** effects, such as revealing an opponent's cards. Their
  contribution is hard to prove, and they weaken the AI's deal sampling.
- **Randomness after the deal**, such as random ranks during play. That is
  output randomness. Random effects at the deal are input randomness and are
  allowed.

### Simplicity rules

- Every sigil has one trigger or condition and one effect, with no riders, at
  every rarity. Higher rarity buys power, not more clauses.
- Commons name one thing and give one benefit: "+20 contract points when your
  team wins a trick with an Ace".
- Count the whole quantity: "for each ♠ your team holds", never "beyond five".
  Balance with the payout, price, or rarity before adding a restriction.
- Use a threshold only when reaching it is the idea, such as winning three
  tricks by trumping. Prefer a named suit to a computed suit and a team result
  to separate conditions for each partner.
- Rules text is generated from the sigil's data
  ([§13](#declarative-sigils)). The text lint checks these conventions, while
  the [simplicity rubric](#simplicity-rubric) penalizes mechanical complexity
  in the fun score. Simulation sweeps amounts and cuts candidates that cannot
  meet the gates without extra bookkeeping. No sigil has an activated ability.

### Rules text

Use plain English, one short sentence, **benefit first, then condition**.
These templates replace 1.0's timing-first rules-text guide for 2.0.

| Kind | Example |
| --- | --- |
| Always on | +40 contract points |
| Trick event | +20 contract points when your team wins a trick with an Ace |
| Last trick | +1 contract multiplier when your team wins the last trick of a round |
| Result | ×1.5 contract multiplier if your team makes its contract exactly |
| Milestone | +1 contract multiplier when your team wins three tricks by trumping |
| Holdings | +20 contract points for each Ace your team holds |
| Bid | +1 contract multiplier when your team bids nil |
| Growth | This sigil gains ×1 contract multiplier every time your team makes a high contract (currently ×0) |
| Card movement | Swap a card with your partner before bidding |
| Shop | Sigils cost you 10 gold less |
| Engraving | +1 contract multiplier when this card wins a trick |
| Synthetic | Replaces 4♣ |

- **Lead with the amount.** Use "+20 contract points", "+1 contract
  multiplier", "×1.5 contract multiplier", or "+50 nil points", without
  "add" or "gain". Additive multipliers use `+1`, never `+1×`.
- **Always name "your team"** for bids, holdings, leads, wins, and results;
  never use "you" as shorthand for the team. Possessives use "your team's".
  Direct choices may say "your partner", engravings say "this card", and
  discounts may say "cost you".
- **Omit obvious scope and timing.** No "each round", "in a round", "after
  bidding" on holdings, "in your shop", or redundant "once". Keep timing
  that changes a choice or a rule, such as "before bidding" on a swap or
  "after bidding" on a rank change. "The last trick of a round" identifies
  the trick and is fine.
- **Use explicit cards.** Write suits as ♣, ♦, ♥, and ♠; write ranks as Ace,
  King, Queen, and Jack, and specific cards as A♦ or 4♣. Name ♦ rather than
  "your longest suit". Adaptive effects are at most a one-off, not a family.
- **Use "when" for events and "if" for results.** A trick event pays on
  every matching trick. A milestone pays when its count is first reached;
  three trump wins do not pay again at six. Numeric thresholds mean at least
  that many unless the text says "exactly". Holdings are counted once after
  bidding, and result conditions once at scoring. A nil-bid trigger fires
  for each partner who bids nil. These defaults live in the rules, not on
  every sigil.
- **Round effects reset; growth persists.** Ordinary points, multipliers,
  and event counters reset between rounds. Growth says "This sigil gains …
  every time … (currently …)" and keeps its accumulated benefit for the run.
  In the growth example, ×0 is the stored extra multiplier, added to the
  base ×1: after two gains the sigil contributes +2. It never multiplies the
  score by zero; a standalone "×1.5 contract multiplier" still compounds.
- **Only engravings say "this card".** Sigils are never engraved. Lead
  triggers fire on the lead whether or not the trick is won.

### Rarity and price

| Rarity | Offer odds | Price | Pool size |
| --- | --- | --- | --- |
| Common | 69% | 50 | 60 |
| Uncommon | 25% | 75 | 60 |
| Rare | 5% | 100 | 20 |
| Legendary | 1% | 150 | 5 |

### Timing

- Gold effects resolve when they trigger.
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

| Rank | A | K | Q | J | 10 | 2–9 |
| --- | --- | --- | --- | --- | --- | --- |
| ♣ ♦ ♥ | 80 | 60 | 45 | 35 | 25 | 15 |
| ♠ | 120 | 90 | 70 | 50 | 40 | 25 |

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
- An engraving works for the card's holder, who is always its owner.
- Engravings are hidden from opponents until the card is played.

Engravings never change a card's suit or rank. When an offer should be a
different card, the shop offers that card instead, as a synthetic copy. The
catalog has four types in two families:

| Engraving | Family | Rules text | Premium |
| --- | --- | --- | --- |
| Bonus | Scoring | +20 contract points when this card wins a trick | 25 |
| Herald | Scoring | +20 contract points when your team leads this card | 25 |
| Multiplier | Scoring | +1 contract multiplier when this card wins a trick | 50 |
| Synthetic | Identity | Replaces 4♣ | 25 |

- Multiplier is offered half as often as the other types.
- The Synthetic text is shown for a J♣, Q♣, K♣, or A♣ that replaces 4♣.

### Synthetic cards

A synthetic card is an extra copy of a real card, such as a second ace of
hearts. It lets a build grow past what the 52-card deck allows, such as a
fifth ace or a second ace of spades.

- It counts as the card it copies everywhere. A synthetic ace of hearts is an
  ace and a heart for following suit, winning tricks, and sigils.
- Each synthetic offer names the real card it replaces, an unowned card from
  two through nine of the **same suit**, chosen at random. While the copy is
  owned, the replaced card is removed from the deck, so every hand still gets
  13 cards. Selling the copy puts the replaced card back.
- The copied card is always a **Jack, Queen, King, or Ace**, even one somebody
  owns. Synthetic copies raise rank without changing suit: A♣ can replace 4♣,
  but A♦ cannot. If no eligible replacement is available, omit that offer.
- A synthetic card costs the copied card's price plus the premium, counts
  toward the 8-card cap, and holds no other engraving.
- A copy and its original can meet in one trick, and the first one played wins.

Synthetic cards are the late-run enabler, and they combine with payoffs in ways
players discover: a synthetic A♦ strengthens a diamonds build and feeds
Ranks, and a synthetic A♠ upgrades a trump. Suit counts in the deck stay the
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
| A major archetype | about 13 payoffs and enablers | about 4 |
| A minor archetype | about 7 | about 2 |
| One side suit: named-suit payoffs and enablers | Depends on the tested suit allocation | Remeasure after the pool revision |
| One specific card | — | seen in about two runs out of three |

- **Cards are the dense enabler.** A team sees most cards at some point,
  including about four ace offers per run.
- **Sigil payoffs are the thin resource.** The
  [Commitment works](#fun-score) family decides whether density levers are
  needed.

## 9. Archetypes

> **Status:** provisional. The roster is [D11](#d11-archetype-roster) and the
> suit structure [D12](#d12-suit-structure). Every sigil in this section is an
> illustrative candidate for the first round of testing, with no evidence yet.

An archetype is a plan a team can build a run around. Archetypes are a design
and simulation tool only. The game never names them, tags sigils with them, or
steers offers by them.

| Archetype | Weight | Plan | Natural partners |
| --- | --- | --- | --- |
| Suits ♣ ♦ ♥ | Major family | Own one side suit, its honors or its length; hold, lead, and win with it | Ranks, Low Cards, Streaks |
| Spades | Major | Own long or high spades; win and trump with them | Bid High, Streaks |
| Ranks | Major | Collect one rank, mostly aces; hold, lead, and win with it | Suits, Bid High, Rainbow |
| Bid High | Major | Bid high contracts (8 or more) and make them | Spades, Ranks, Streaks |
| Nil | Major | Bid nil and make it | Low Cards, Exact |
| Low Cards | Minor | Win with 2s through 10s | Suits, Spades, Nil |
| Rainbow | Minor | Win tricks with all four suits | Ranks, Suits |
| Streaks | Minor | Win tricks in a row | Bid High, Spades |
| Exact | Minor | Make exactly your contract | Nil, Low Cards |

- **Majors** have sigils at every rarity and can carry a run.
- **Minors** have about five point sigils, and usually join a major.
- **Generic** point sigils support every build. Ideas too narrow for an
  archetype become one-offs
  ([Appendix C](#appendix-d-one-offs-not-archetypes)).

### Ways to score

Each archetype rewards one idea through several triggers, so its sigils stack
instead of competing.

| Trigger | What it checks | Example |
| --- | --- | --- |
| Hold | Both partners' hands after bidding | +20 contract points for each Ace your team holds |
| Lead | Cards your team leads | +15 contract points when your team leads an Ace |
| Win | The card that wins a trick for your team | +20 contract points when your team wins a trick with an Ace |
| Trump | Winning a trick by trumping | +20 contract points when your team wins a trick by trumping |
| Bid | The contract's size, or a nil bid | +1 contract multiplier when your team bids a high contract |
| Make | The round's result | +1 contract multiplier if your team makes its contract exactly |

### Payoffs and enablers

Point sigils are **payoffs**: they score when something happens. **Enablers**
make that thing happen more often. Enablers come from three places:

- **Cards:** buying the suit, rank, or length a payoff counts. Card offers are
  dense, so this is the main enabler.
- **Synthetic cards:** late-run copies that grow a collection past the deck's
  limits, such as a fifth ace or a second ace of spades.
- **Enabler sigils:** shop steering ("One card offer is ♦"), discounts, and rule benders.

Rules for the pool:

- Every archetype has at least one common and one uncommon enabler sigil, and
  each major has at least three.
- Rule-bending enablers are uncommon or above.
- An enabler must read clearly on its own: a player who buys it should see what
  it makes happen.
- Every payoff fires at a real base rate before its enablers arrive. Gate 2
  measures this ([§12](#gates)).

Each archetype below lists its enablers before its payoffs. Examples are tagged
by rarity (C, U, R, L) and category.

### Suits

**Plan.** Pick a side suit (♣, ♦, or ♥). Buy its honors or its length, add
synthetic copies late in the run, and hold, lead, and win with it. Honors and
length are two lines of one archetype that share its sigils:

- The **honors** line owns the A, K, and Q and cashes them early.
- The **length** line owns many cards of the suit. Long side suits get trumped
  once opponents run out, so this line leans on Hold and Lead payoffs and on
  "can't be trumped" enablers.

**Named suits.** Start with ♦ examples rather than an adaptive suit package.
Use ♣, ♥, or ♠ versions only where simulation shows they earn their pool
slots; complete cycles are not required. The Suits budget stays at 18 payoff
slots, with the former six adaptive slots reassigned to named-suit candidates.
Concentrating support on ♦ is an explicit alternative if spreading it across
three side suits makes each build too thin ([D12](#d12-suit-structure)).

| Trigger | Rarity, category | Text |
| --- | --- | --- |
| Win | C, points | +10 contract points when your team wins a trick with ♦ |
| Hold | C, points | +10 contract points for each ♦ your team holds |
| Lead | C, points | +10 contract points when your team leads ♦ |
| Win | U, +mult | +1 contract multiplier when your team wins three tricks with ♦ |
| Make | U, ×mult | ×1.5 contract multiplier if your team wins five tricks with ♦ |
| Win | R, +mult | +1 contract multiplier when your team wins a trick with ♦ |

**Enablers:**

- [C] One card offer is ♦
- [U] ♦ cost you 10 gold less
- [R, rule] Your team's ♦ can't be trumped

**Notes:**

- **Buys:** the named suit's honors, or its cheap low cards for length.
- **Later:** synthetic copies of its honors replace low cards of that suit.
- **Threat:** opponents void the suit and trump it.

### Spades

**Plan.** Own long or high spades. Spades win in every hand: high spades cash,
and low spades trump. Named-♠ versions of suit payoffs can score here too.

**Enablers:**

- [C] ♠ cost you 15 gold less
- [U, rule] Your team can lead ♠ at any time
- [R, rule] Your team wins the first trick it trumps

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Trump | [C, points] +20 contract points when your team wins a trick by trumping |
| Lead | [C, points] +10 contract points when your team leads ♠ |
| Hold | [U, points] +5 contract points for each ♠ your team holds |
| Trump | [U, +mult] +1 contract multiplier when your team wins three tricks by trumping |
| Win | [R, +mult] +1 contract multiplier when your team wins a trick with ♠ |
| Win | [R, ×mult] ×2 contract multiplier if your team wins five tricks with ♠ |

**Notes:**

- **Buys:** spades, both high and low.
- **Threat:** opponents strip trumps by leading spades.

### Ranks

**Plan.** Collect one rank, usually aces, and hold, lead, and win with it. Aces
win in every side suit and cost the most. A few sigils pay for kings instead.

**Enablers:**

- [C] Aces cost you 25 gold less
- [U, rule] Your team's Kings count as Aces for sigils
- [R, rule] Your team's Aces can't be trumped

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Win | [C, points] +20 contract points when your team wins a trick with an Ace |
| Hold | [C, points] +20 contract points for each Ace your team holds |
| Lead | [C, points] +15 contract points when your team leads an Ace |
| Win | [C, points] +15 contract points when your team wins a trick with a King |
| Hold | [U, +mult] +2 contract multiplier if your team holds four Aces |
| Win | [U, ×mult] ×1.5 contract multiplier when your team wins three tricks with Aces |
| Win | [R, +mult] +1 contract multiplier when your team wins a trick with an Ace |

**Notes:**

- **Buys:** aces, then synthetic aces in later shops.
- **Threat:** aces cost the most, and they get trumped once a suit runs out.

### Bid High

**Plan.** Bid high contracts, 8 or more, and make them. Bid High pays for
bidding high, so its risk lives in the bidding, and a set multiplies against
you.

**Enablers:**

- [C] Tens, face cards, and Aces cost you 10 gold less
- [U, rule] Your team needs one trick fewer to make a high contract.
- [R, rule] Tricks your team wins with an Ace count as two toward your team's contract

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Bid | [C, points] +5 contract points for each trick in your team's contract |
| Bid | [C, points] +60 contract points when your team bids a high contract |
| Bid | [C, +mult] +1 contract multiplier when your team bids a high contract |
| Bid | [U, +mult] +2 contract multiplier if your team's contract is 10 or more tricks |
| Bid | [U, points] +10 contract points for each trick in your team's contract |
| Make | [R, +mult] This sigil gains ×1 contract multiplier every time your team makes a high contract (currently ×0) |
| Make | [R, ×mult] ×2 contract multiplier if your team makes a high contract |

**Notes:**

- **Buys:** high cards and spades.
- **Threat:** a set multiplies against you, and opponents defend hard against
  big contracts.

### Nil

**Plan.** Bid nil and make it. The nil score joins the base, so every
multiplier also grows your nils, and a failed nil costs more as the build
grows.

**Enablers:**

- [C] Twos through sixes cost you 5 gold less
- [U, movement] Swap a card with your partner before bidding
- [U, rule] Your team's nil bidders' Aces become twos after bidding

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Make | [C, points] +50 nil points |
| Bid | [C, +mult] +1 contract multiplier when your team bids nil |
| Make | [U, ×mult] ×1.5 contract multiplier if your team makes a nil |
| Make | [R, points] This sigil gains +40 nil points every time your team makes a nil (currently +0) |

**Notes:**

- **Buys:** low cards. Nil teams avoid owning aces and high spades.
- **Threat:** opponents lead low to force the nil bidder to win a trick.

### Low Cards

**Plan.** Win tricks with 2s through 10s, the cheapest cards in the shop. Low
cards win through length, by trumping, and after the honors are gone.

- **Enablers:**
  - [C] Low cards cost you 5 gold less
  - [U, rule] Your team's Jacks count as low cards for sigils
- **Payoffs:**
  - [C, points] +15 contract points when your team wins a trick with a low card
  - [C, points] +10 contract points when your team leads a low card
  - [U, +mult] +1 contract multiplier when your team wins four tricks with low cards
  - [R, points] +150 contract points when your team wins a trick with a two

### Rainbow

**Plan.** Win tricks with cards of all four suits. That takes a winner in
every suit, so Rainbow buys an ace or king of each and fills gaps with
synthetic copies.

- **Enablers:**
  - [C] One card offer is from the suit your team owns the fewest cards of
  - [U, rule] Tricks your team wins by trumping count as won with the suit led for sigils
- **Payoffs:**
  - [C, points] +25 contract points when your team wins its first trick with each suit
  - [C, +mult] +1 contract multiplier if your team wins tricks with all four suits
  - [U, ×mult] ×1.5 contract multiplier if your team leads all four suits
  - [R, ×mult] ×2 contract multiplier if your team wins tricks with all four suits

### Streaks

**Plan.** Win tricks in a row. Each hand becomes a sequencing puzzle: give up
your losers at the right moment, then keep the lead.

- **Enablers:**
  - [C] Your team's streak survives its first lost trick
  - [U, rule] Choose which partner on your team leads the first trick after bidding
- **Payoffs:**
  - [C, points] +10 contract points when your team wins consecutive tricks
  - [C, +mult] +1 contract multiplier when your team wins the last trick of a round
  - [U, +mult] +2 contract multiplier when your team wins four tricks in a row
  - [R, ×mult] ×2 contract multiplier if your team wins five tricks in a row

### Exact

**Plan.** Make exactly your team's contract. Overtricks already score
nothing, and Exact pays for avoiding them. Exact pairs naturally with Nil and Low Cards.

- **Enablers:**
  - [C] One card offer is a two through five
  - [U, rule] Your team's Aces and Kings become twos when your team makes its contract
- **Payoffs:**
  - [C, points] +50 contract points if your team makes its contract exactly
  - [C, +mult] +1 contract multiplier if your team makes its contract exactly
  - [U, ×mult] ×1.5 contract multiplier if your team makes its contract exactly
  - [R, +mult] This sigil gains ×1 contract multiplier every time your team makes its contract exactly (currently ×0)

### Generic

Generic point sigils fit any build and carry runs while a plan comes together:

- [C, points] +5 contract points when your team wins a trick
- [C, points] +40 contract points
- [U, +mult] +1 contract multiplier
- [R, ×mult] ×1.5 contract multiplier
- [L, ×mult] ×2 contract multiplier

Generic enablers steer offers toward whatever you're already building. They
are also the [card-lean lever](#15-risks-and-tuning-levers), offered as
sigils:

- [U] Card offers favor cards your team's sigils mention
- [U] +1 sigil offer

Utility examples:

- **Economy and shop:** "The first reroll is free", "+50 maximum interest",
  "Sigils cost you 10 gold less", and "This sigil gains 15 gold in sell value
  after scoring".
- **Legendary utility:** "+1 sigil slot".

## 10. Sigil pool skeleton

> **Status:** provisional. The category split follows
> [D3](#d3-scoring-categories), pool sizes are [parameters](#appendix-b-parameter-register) P21, and every
> budget is a starting size for number sweeps.

The pool has 145 sigils. Following Bridge of Rogues, about 70% of commons, 50%
of uncommons, 75% of rares, and 60% of legendaries score points.

### By rarity and category

| Rarity | +Points | +Mult | ×Mult | Utility | Total |
| --- | --- | --- | --- | --- | --- |
| Common | 30 | 9 | 3 | 18 | 60 |
| Uncommon | 10 | 14 | 6 | 30 | 60 |
| Rare | 4 | 5 | 6 | 5 | 20 |
| Legendary | — | — | 3 | 2 | 5 |
| **Total** | **44** | **28** | **18** | **55** | **145** |

At least 15 of the 44 contract-point sigils, and at least 12 of the 46
multipliers, scale with or require the contract size
([keeping bids tense](#keeping-bids-tense)).

### Point sigils by archetype

| Archetype | Common | Uncommon | Rare | Total |
| --- | --- | --- | --- | --- |
| Named suits (♦ first; ♣ ♥ ♠ variants tested) | 11 | 6 | 1 | 18 |
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

Any ♠ variants in the named-suit allocation also support Spades. Their count
and the split between side suits are measured in stage 3, not assumed.

### Utility by family

| Family | Common | Uncommon | Rare | Legendary | Total |
| --- | --- | --- | --- | --- | --- |
| Economy and shop | 6 | 9 | 1 | — | 16 |
| Enablers | 9 | 12 | 3 | 1 | 25 |
| Rule benders and deal control | 3 | 9 | 1 | 1 | 14 |
| **Total** | **18** | **30** | **5** | **2** | **55** |

- The 25 enablers are three for each major, two for each minor, and two
  generic offer steerers.
- The three common rule-bender and deal-control slots are pre-bid card
  movement, since rule benders themselves start at uncommon.

### Budgets

These are starting sizes for sigils of the right kind on a made contract. Number
sweeps ([§13](#declarative-sigils)) move each sigil toward the par curve and
its lift band.

| Rarity | +Contract points | +Contract multiplier | ×Contract multiplier |
| --- | --- | --- | --- |
| Common | +15 to +20 per narrow event; +5 per trick in the contract; +40 to +60 flat | +1 on a condition met in about a third of made contracts | ×1.5 on a narrow condition |
| Uncommon | +30 per event, or a tuned amount per card held | +2 on a moderate condition, or +1 broadly | ×1.5 broadly, or ×2 narrowly |
| Rare | +50 per event, or a run-long counter | +1 per counter step | ×2 broadly, or ×3 narrowly |
| Legendary | — | — | ×2 to ×3 broadly |

Checks against par:

- **Round 4 (par 160).** A team holds three commons: "aces: +20" (firing
  twice), "+5 per trick in your team's contract", and a +1 that holds in a third of
  made contracts. A made 8 averages (80 + 40 + 40) × 1.33 ≈ 215. With one set
  in five at −80 × 1.33 ≈ −105, it averages about 150 per round.
- **Round 8 (par 600).** A team holds two contract-point sigils worth +85 on a
  made 9, an uncommon +2 and a common +1 that both hold, and a ×1.5 that
  holds half the time. A made contract scores (90 + 85) × 4 × 1.25 ≈ 875. With
  one set in four at −90 × 4 × 1.25 = −450, it averages about 540.

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
> every threshold, band, and weight is a placeholder until the harness runs:
> [parameters](#appendix-b-parameter-register) P25–P28.

The metrics answer one question: what does it mean for Rogue Spades to be fun?
They come in two layers:

- **Gates** are bars that every sigil must pass to ship, so "every sigil
  matters" can't be averaged away.
- The **fun score** is a weighted sum used to compare pool versions and
  tuning levers.

Every threshold is a placeholder. Real values are set once the harness runs,
and recalibrated at stage 7.

### Gates

| Gate | Your goal | Measured as | Starting bar |
| --- | --- | --- | --- |
| 1. Choices matter | Every sigil contributes measurably to victory | Forced-pick win-rate lift over a same-rarity control. For point sigils, the decisive share: the holder's wins that become losses or draws when rescored without the sigil | Lift above 0; decisive in at least 5% of wins |
| 2. Payoffs are doable | If a sigil needs X, X is consistently possible; if a sigil enables X, X matters | A payoff's trigger rate per round in random hands (base) and with typical support (committed). An enabler's lift on its payoffs' trigger rates | Base at least 10% of rounds; committed at least 60%; enabler lift at least 15 points, plus a win-rate lift above 0 |
| 3. No invalidation | Opponents can't turn a strategy into a non-game | For each archetype and each sigil that touches opponents, the drop in the archetype's win rate when the opponents hold it | At most 15 points |
| Power ceiling | No dominant best build | Forced-pick lift; win rate of the strongest pairs | Lift at most 12 points; no pair above a 65% win rate |

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
- **Using it.** A change to the pool or a lever is kept when every gate still
  passes and the fun score does not fall. A more complex replacement must
  demonstrate a net improvement after its simplicity penalty; if the paired
  90% confidence interval for that improvement includes zero, prefer the
  simpler candidate. Complexity never excuses a failed gate or a forbidden
  design pattern in [§5](#simplicity-rules).
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
| Numeric value | +1 per occurrence | Payouts, multipliers, thresholds, card numbers, range endpoints, explicit quantities, ordinals, and displayed growth totals all count; +20 is one value, not two digits |
| Boolean condition | +1 per atomic check | Whether your team wins, holds a card, makes its contract exactly, or whether a card is ♠; count every check in an AND, OR, or exception, not just the whole clause |
| Arithmetic rider on the quantity rewarded | +2 per operation | Subtracting five and flooring at zero in "beyond five" is one combined arithmetic rider, in addition to its number and cutoff condition |
| Computed card selector | +1 per selector | "Longest suit" requires comparing suits instead of naming ♦ |
| Extra tracked state | +1 per counter or remembered fact | A sigil-specific milestone count, a streak, a once-only flag, or a growth counter carried between rounds |

Count rule-level checks, including triggers and filters, regardless of whether
the text says "when", "if", or "for each". A numeric threshold charges both
for its number and for its comparison. "High contract" still charges for the
hidden 8 and the check against it. Repeating the same numeral in different
roles charges each occurrence; a rule field and the text generated from that
field are one occurrence, not two. A changing "currently ×0" display charges
one numeric value plus the cost of remembering that state.

Ordinary score addition and multiplication add no arithmetic-rider cost, but
their printed amounts always incur the numeric cost. "Your team" establishes
scope without another check. Do not charge implementation guards or repeated
evaluations of the same check; charge the distinct checks the player must
understand. Reusing an ordinary Spades fact adds no state cost, but a condition
on that fact still costs. Increasing rarity or adding a UI counter waives none
of these costs.

The per-sigil simplicity score is **S = 1 / (1 + C)**. Family 9 is the mean S
across all sigils in the candidate pool, counting each once, including utility
and rule benders; an empty pool scores 1. Report each candidate's costs and
compare replacements in the same pool slot so unrelated simple filler cannot
mask a rider's cost. The one-effect rule remains mandatory at every rarity.

| Candidate | Cost breakdown | C | S |
| --- | --- | --- | --- |
| +40 contract points | One number | 1 | 1/2 |
| ×1.5 contract multiplier if your team makes its contract exactly | One number, one condition | 2 | 1/3 |
| +20 contract points when your team wins a trick with an Ace | One number, two conditions: team wins and winning rank is Ace | 3 | 1/4 |
| +5 contract points for each ♠ your team holds | One number, two conditions: held by your team and suit is ♠ | 3 | 1/4 |
| +5 contract points for each ♠ your team holds beyond five | Two numbers, three conditions (including count > 5), one arithmetic rider worth 2 | 7 | 1/8 |

Thus "beyond five" strictly lowers the score with everything else held equal,
even if both versions pass the balance gates. For a useful balance comparison,
sweep each version's payout against the same gates and par targets on paired
seeds; compare their best passing versions, not just identical amounts with
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
  or more. There's no randomness after the deal, by rule.
- **Economy:** gold unspent at the end of a run; purchases and rerolls per
  shop.
- **Text quality:** the wording lint ([§5](#simplicity-rules)); mechanical
  simplicity is scored in family 9, not left as an unweighted diagnostic.
- **AI play quality:** 1.0's bench checks, such as wasted overtakes, missed
  nil covers, and nil suicides.

### Results

- Gate and fun-score results go into the sigil records and decision records
  described in [§16](#16-evidence-and-design-records).
- A sigil that fails is retuned, usually by a number sweep, or cut.
- An archetype whose sigils keep failing drops to one-offs.

## 13. Simulation

> **Status:** provisional. The architecture follows
> [D15](#d15-sigil-representation)–[D18](#d18-throughput), and the tier budgets
> are [parameters](#appendix-b-parameter-register) P24.

### The chicken-and-egg answer

The worry is circular: designing sigils needs strong simulated players, but the
players need to understand the sigils. The answer is that **no AI component has
sigil-specific knowledge**:

- Play and bidding search the real rules and scoring through the rules kernel,
  so a new sigil changes their choices the moment it exists.
- The shop values an offer by playing sampled hands with and without it.
- Only rule benders and economy sigils, which sampled hands value poorly, use
  values fitted from earlier batches.

Fidelity therefore grows by stage, through search budgets and fitted values,
and never through per-sigil AI code ([§14](#14-staging)).

### The rules kernel

One kernel runs the game, the UI, and every AI search:

- **Compact state with apply-and-undo moves,** so search never deep-copies the
  game. 1.0's engine runs `structuredClone` on the whole state for every
  action.
- **Hook tables.** Sigil data compiles into tables for legal plays, each card's
  effective suit and rank, the trick winner, scoring events, and shop rules.
- **A ledger** records every contract point and multiplier each sigil
  contributes, so any score can be recomputed without any set of sigils.
- **Plain TypeScript with no globals.** Randomness comes from explicit seeded
  streams. The harness runs under Node 24's built-in type stripping, without
  Vite.

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

generates "+20 contract points when your team wins a trick with an Ace".
A rule bender names one of a small set of rule hooks written in code:

```json
{
  "id": "aces-untrumpable",
  "rarity": "Rare",
  "archetypes": ["Ranks"],
  "role": "enabler",
  "effect": { "type": "rule", "rule": "cantBeTrumped", "card": { "rank": "A" } }
}
```

The grammar covers:

- **Events:** hold, lead, play, win, trump, bid, make, exact, nil made, first
  and last trick, and streaks.
- **Card filters:** rank, rank range, and named suit (including ♠).
- **Scaling:** per card held, per trick in the contract, and per run-long
  counter.
- **Thresholds:** simple milestones; no subtraction from a held-card count.
- **Effects:** contract points, +multiplier, ×multiplier, gold, a rule hook, a
  shop rule, and pre-bid card movement.

Keeping sigils as data makes three things possible:

- **Number sweeps.** The harness searches for the amount that puts a sigil
  inside its budget and lift band.
- **Enumeration.** Common payoffs are enumerated from the grammar, roughly
  6 triggers × 15 nouns × 3 effects ≈ 270 candidates.
- **Targeting.** The grammar shows which sigils touch opponents, so the gate 3
  scan runs only on those.

### AI tiers

| Tier | Play | Bidding | Shop | Target per run | Used for |
| --- | --- | --- | --- | --- | --- |
| 0 | Heuristic: 1.0's rollout policy on the kernel, plus a one-trick lookahead scored with sigils | Trick estimate, then expected score over the scoring model | Rescoring with 16 sampled hands | About 0.3 s | Sweeps, enumeration, screening |
| 1 | Information-set MCTS, about 200 iterations over 32 deals that fit the bids | Expected score over 40 rollouts | Rescoring with 32 hands | About 3 s | Gate decisions and the fun score |
| 2 | Information-set MCTS, 1,500 iterations over the best 64 of 512 deals | Expected score over 160 rollouts | Rescoring with 64 hands | About 20 s | Final checks, tier calibration, and the shipped AI |

- The algorithms are 1.0's: deals sampled to fit the bids, UCB search,
  heuristic rollouts, tie-breaking among near-equal moves, and expected-score
  bidding and nil decisions.
- Tier 0 and 1 results stand in for tier 2 only while they rank sigils the
  same way. Spot checks at each stage, and stage 7, confirm it.

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

- **Value by rescoring.** For each offer, the AI samples hands built from the
  team's owned cards plus random fill, and plays them on tier 0 with and
  without the offer. That gives the change in points per round. A fitted curve
  from score margin by round to win rate turns this into win probability over
  the remaining rounds, which is then divided by price.
- **Fitted values** cover rule benders and economy sigils, which rescoring
  values poorly. They come from regressing outcomes on holdings in earlier
  batches.
- **Buying.**
  - Buy the best value per gold while it clears a bar that accounts for
    interest.
  - Reroll when nothing clears the bar and gold allows.
  - Sell when a slot is needed and an offer beats an owned item by more than
    the sale loses.
- **Policies.** A flexible team uses plain values. A committed team adds a
  bonus for one archetype's sigils and for the cards they reward.

### Experiments

- **Seeded streams.** Every random draw comes from a named stream:
  - deals, one stream per round;
  - offers, one stream per team, shop, and reroll;
  - AI search, one stream per seat.

  The arms of an experiment share deals and offers until their choices
  diverge.
- **Paired runs.** Arms run on the same seeds. A standard experiment is 2,000
  paired runs, which takes about 10 minutes at tier 1 on 16 worker processes.
  Results report 90% confidence intervals.
- **Hand-level trials** (stage 2). Fixed builds of sigils and owned cards play
  single rounds against random deals, measuring trigger rates, points per
  round, and set rates. They're cheap, and they don't depend on the shop AI.
- **Forced-pick trials.** A candidate is forced into one team at shop 1, 3, or
  5, and that team builds around it.
  - The control arm forces a same-rarity plain sigil instead: "+N contract
    points", with N at its rarity's flat budget.
  - The opponents are a flexible field.
- **Commitment trials.** One team commits to an archetype at shop 1. The trial
  measures online rates and win rates against a flexible field.
- **Counter scans.** A committed archetype plays against opponents forced to
  hold a sigil that touches opponents.
- **Pair trials.** Forced pairs run against each sigil alone, for the synergy
  family.
- **Secondary configuration.** Teams of two humans with 16 owned cards are
  checked at stage 7, but not balanced for.

### What carries over from 1.0

| Keep | Rebuild |
| --- | --- |
| The card model and rendering; the table, hand, trick, shop, and scoreboard components; styles | The engine core: `reduce`, `drain`, and the per-action `structuredClone` become the kernel |
| The Spades rules logic, as the kernel's reference | Sigil handlers and the 28-window `Ctx` API become declarative data and rule hooks |
| The AI algorithms: deal sampling fitted to bids, MCTS, tie-breaking, expected-score bidding, and nil logic | The scoring probes, which the kernel's exact scoring replaces |
| The bench metric definitions: set rate, overtakes, and nil covers | The shop, which becomes team-level and sells cards with engravings |
| — | Randomness: seeded streams instead of patching `Math.random` |
| — | The bench, which becomes parallel paired runs with win rates and per-sigil ledgers |

## 14. Staging

> **Status:** provisional ([D20](#d20-staging),
> [D21](#d21-candidate-generation)).

The simulation and the pool grow together, one rarity at a time. Each stage's
exit gate unlocks the next. Cards come before sigils on purpose: they're the
main enabler, so their prices must be stable before any payoff can be
measured.

| Stage | Builds | Exit gate |
| --- | --- | --- |
| 0. Kernel | The rules kernel, AI tiers 0–2, seeded streams, and the parallel experiment runner | Plain 8-round Spades: the AI meets 1.0's play targets (set rate, nil success, bid error); a 2,000-run experiment takes about 10 minutes |
| 1. Cards | The card shop, the 8-card cap, income and interest, and rerolls, with no sigils; 1.0's UI ported onto the kernel | Card prices give roughly equal value per gold; no card dominates; hands keep their variety; the game is playable by hand |
| 2. Commons, hand level | The grammar, text generation, and lint; about 270 enumerated common payoffs; hand-designed common enablers and utility | Gate 2 from hand-level trials; numbers swept to the common budget; at least 1.5 surviving candidates per common slot |
| 3. Commons, run level | The rescoring shop AI and its policies | 60 commons pass gates 1–3; the fun score is recorded; density levers are decided here if Commitment works fails |
| 4. Engravings | The four-type catalog on card offers, including synthetic cards | The same gates, by engraving type |
| 5. Uncommons | Hand-designed waves of about 1.5 candidates per slot, seeded from 1.0, including most enablers and rule benders | The same gates, plus enabler lift |
| 6. Rares and legendaries | The last 25 sigils | The same gates, plus the power ceiling |
| 7. Validation | The full pool at tier 2 | Thresholds recalibrated; tiers calibrated; the two-human configuration checked; human playtests |

- **Choosing the pool.** Simulation decides pass or fail and the numbers.
  Designers choose among passing candidates for variety, clarity, and
  archetype coverage, using the simplicity rubric and its comparison rule.
  For example, the 42 common payoffs are picked from the enumerated survivors.
- **Records at every exit.** A stage isn't done until the decision, parameter,
  and sigil records its experiments touched are written
  ([§16](#16-evidence-and-design-records)), so the documentation grows with
  the pool.
- **Hand play from stage 1.** The ported UI lets the designer play each
  stage's pool by hand. Hand play measures comprehension to calibrate the
  structural simplicity score, checks the feel of railroading, and catches
  AI blind spots early.
- **Mining 1.0.** Higher-rarity waves start from the 1.0 sigils that fit
  2.0's rules ([Appendix D](#appendix-e-seeds-from-rogue-spades-10)).

## 15. Risks and tuning levers

| Risk | Lever |
| --- | --- |
| Purely random offers leave committed builds thin | Run pools (K random archetypes per run, never named), affinity weighting, card lean, a fourth sigil offer, cheaper rerolls |
| Late-run teams bid safe despite the bid-scaled pool | Share of bid-scaled sigils, Bid High budgets, and the set penalty |
| Low-rarity ×multipliers overshoot the ×10 curve | ×Multiplier counts and sizes per rarity |
| Long side suits get trumped, so the Suits length line fails | Hold and Lead payoffs, "can't be trumped" enablers, and synthetic cards |
| Synthetic duplicates confuse card counting | Copies are visibly synthetic when played; synthetic offer share |
| Hidden sigils make AI opponents misread builds | A prior over unrevealed sigils, built from offer odds |
| The 8-card cap is too loose or too tight for input randomness | The cap, between 6 and 10 |
| Two-human teams (16 cards) outscore single-player par | A per-mode cap; otherwise accepted as a secondary configuration |
| Rescoring undervalues enablers and rule benders | Fitted values and committed-policy bonuses |
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
- **Kinds of decision.** Design decisions are judged by the gates and the fun
  score. Method decisions, such as the AI architecture, are judged by harness
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
| Gates | Base and committed trigger rates, win-rate lift over its control, decisive share, enabler lift, and the counter scan where it applies, each with a confidence interval and run count |
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

- **Design decisions** (D1–D13 and D23–D24) are judged by the gates and the fun
  score.
- **Method decisions** (D14–D21) are judged by harness measurements or designer
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

- **Starting choice:** par grows about ×10 from round 1 to round 8, from 60 to
  600 (parameter P20).
- **Alternatives:**
  - about ×5;
  - about ×20, as in Bridge of Rogues;
  - no fixed curve, tuned only to the closeness metrics.
- **Prior reasoning:** with constant growth per round, rounds 1–4 hold 28% of
  points at ×5, 21% at ×10, and 15% at ×20. A 7-sigil build would naturally
  reach ×20 or more.
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
- **Test:** each lever as an arm at stage 3, if Commitment works fails at
  baseline.
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
  score only from 1,080 to 1,000 while sharply cutting the risk of a set, so
  an expected-score bidder sandbags.
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
- **Decided by:** No invalidation (gate 3) and Skill and bidding, plus how
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
  archetype whose sigils keep failing drops to one-offs.
- **Evidence:** pending. **Status:** hypothesis.

### D12. Suit structure

- **Starting choice:** named-suit payoffs and enablers, using ♦ as the example;
  18 payoff slots formerly divided between cycles and adaptive suits now
  serve named suits. No required cycles and no adaptive family.
- **Alternatives:** spread the named-suit slots across ♣, ♦, and ♥, with ♠
  variants where useful; concentrate side-suit support on ♦; allow one
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

- **Starting choice:** rule benders and pre-bid card movement are allowed;
  information effects and randomness after the deal are excluded.
- **Alternatives:**
  - allow information effects;
  - allow randomness after the deal.
- **Decided by:** gates 1 and 3 for rule benders. The exclusions rest on
  designer judgment, from the brief's preference for input randomness over
  output randomness.
- **Test:** rule benders go through the normal gates at stages 5 and 6.
- **Evidence:** pending. **Status:** hypothesis.

### D14. Metric structure

- **Starting choice:** per-sigil gates plus a weighted fun score: Close and
  live 25, Commitment works 15, Archetypes viable 15, Synergy 15, Skill and
  bidding 15, and Simplicity 15. Mechanical complexity is scored from sigil
  data, with a positive cost for every numeric value and boolean condition;
  more complex replacements need a demonstrated net improvement.
- **Alternatives:**
  - one weighted score with no gates;
  - the same gates, with weights that favor synergy;
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

- **Starting choice:** one rules kernel shared by the game and the AI search.
- **Alternatives:**
  - patch 1.0's plain-Spades search;
  - search with perfect information in simulation.
- **Decided by:** AI play quality and throughput at stage 0.
- **Evidence:** pending. **Status:** hypothesis.

### D17. Shop AI

- **Starting choice:** value offers by playing sampled hands with and without
  them, with fitted values for rule benders and economy sigils.
- **Alternatives:**
  - fitted values only;
  - hand-written heuristics.
- **Decided by:** shop skill at stage 3: on paired seeds, the rescoring shopper
  should beat the simpler ones.
- **Evidence:** pending. **Status:** hypothesis.

### D18. Throughput

- **Starting choice:** three AI tiers, with a standard experiment taking about
  10 minutes (parameters P24 and P27).
- **Alternatives:**
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
  higher rarities by hand in waves seeded from 1.0.
- **Alternatives:**
  - hand-designed waves at every rarity;
  - enumeration at every rarity.
- **Decided by:** how many candidates from each source pass their gates.
- **Evidence:** pending. **Status:** hypothesis.

### D22. Rules text

- **Starting choice:** benefit first, then condition; explicit "your team";
  no redundant timing or shop scope; named suits; `+1` for additive
  multipliers. The full templates and timing defaults are in [§5](#rules-text).
- **Alternatives:** 1.0's timing-first "gain"; the previous "add" templates;
  colon shorthand.
- **Decided by:** the designer's 2026-10-06 wording preference, checked for
  comprehension in playtests. Wording alone is not a balance claim.
- **Evidence:** designer direction. **Status:** text convention chosen;
  behavioral simplifications remain hypotheses under D24.

### D23. Identity engravings

- **Starting choice:** synthetic offers copy only J, Q, K, or A, replacing an
  unowned 2–9 of the same suit. The offer shows its resulting identity and
  "Replaces 4♣"; the engraving never changes that printed identity in play.
- **Alternatives:** the earlier unrestricted synthetic copies; the original
  identity catalog of Converted, Raised, Crowned, and Wild.
- **Prior reasoning:** synthetic cards upgrade ranks without moving cards
  between suits, and the replacement label says only what the player needs.
- **Decided by:** the designer's same-suit and J-or-higher constraints, then
  Commitment works, Archetypes viable, and the power ceiling for balance.
- **Test:** stage 4 sweeps premiums, offer share, replacement rank range, and
  J/Q/K/A weights within those constraints, including Rainbow and Nil runs.
- **Evidence:** pending; no simulator exists yet. **Status:** hypothesis.

### D24. Simpler sigil conditions

- **Starting choice:** remove "beyond N" arithmetic, adaptive suit tracking,
  separate partner-result checks, and compound trick-sequence restrictions.
  Keep whole-count payouts and simple milestones; tune numbers first.
- **Alternatives:** the previous restricted candidates as experiment controls;
  smaller payouts or fewer candidates if the simple versions dominate.
- **Decided by:** all sigil gates and the weighted fun score, especially the
  power ceiling, Skill and bidding, Commitment works, and the explicit
  Simplicity cost in family 9.
- **Test:** paired-seed comparisons in stages 2–6, sweeping payout, price, and
  rarity, recording both mechanical complexity and the net fun-score change.
  In particular, compare the former +15 per ♠ beyond five with whole
  holdings payouts starting at +5 per ♠; also remeasure the revised Spades,
  Bid High, Rainbow, Streaks, and Exact candidates. Cut failures instead of
  restoring fiddly clauses.
- **Evidence:** pending; no simulator exists yet. All revised amounts are
  illustrative, not validated balance. **Status:** hypothesis.

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
| P19 | Synthetic replacement range and copied ranks | Unowned 2–9 of the same suit; copies J/Q/K/A | Replacement ranks below J; weights across J/Q/K/A | Input randomness; Commitment works; Archetypes viable; power ceiling | 4 |
| P20 | Par growth over a run | ×10, from 60 to 600 | ×5–×20 | Close and live | 3–6 |
| P21 | Pool sizes and category split | 60 / 60 / 20 / 5, split as in [§10](#10-sigil-pool-skeleton) | ±25% per rarity | Commitment works; Archetypes viable | 3–6 |
| P22 | Nil base value | ±100 | 50–150 | Archetypes viable (Nil) | 0 |
| P23 | Bid-scaled share of the pool | A third of contract-point sigils, a quarter of multipliers | None to half | Skill and bidding | 3–5 |
| P24 | AI tier budgets | About 0.3 s, 3 s, and 20 s per run | Per tier | Agreement between tiers; run time | 0 |
| P25 | Fun score weights | 25 / 15 / 15 / 15 / 15 / 15 (families 4–9) | Any | Designer judgment, checked against playtests | 7 |
| P26 | Gate thresholds and fun score bands | As in [§12](#12-metrics-what-fun-means) | Any | Calibrated once the harness runs, and again at stage 7 | 3, 7 |
| P27 | Experiment size | 2,000 paired runs | 1,000–10,000 | Width of confidence intervals | 0 |
| P28 | Simplicity rubric | Each numeric value and boolean check 1; arithmetic rider 2; computed selector or tracked state 1; S = 1 / (1 + C) | Positive costs; compare 1–3 per burden; no free numbers or conditions | Designer judgment, calibrated by comprehension playtests and paired simulation tradeoffs | 2–7 |

## Appendix C: Unexamined assumptions

These starting assumptions were made without a dedicated question. They are
hypotheses like everything else, and each gets a decision record when an
experiment first examines it.

- A set team scores (−10 × B + nil score) × its multipliers and loses all
  contract points. Multipliers whose condition requires making the contract
  don't count.
- Blind nil leaves the base rules and exists only as a one-off sigil.
- 1.0's other base rules stay: individual bids add up to the team contract,
  nil, spades must be broken, the first lead comes from the dealer's left, and
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
- Every random draw comes from a named, seeded stream.
- Contract points earned on overtricks count. Contract points from tricks a
  nil bidder wins don't. A double-nil team's contract points are lost.
- Nil points go to made nils rather than to the contract, so they survive a
  set.
- "High contract" (8 or more), "low card" (two through ten), and "streak" are
  terms, so commons can name them as one noun.
- A multiplier from a repeated trigger stacks each time it triggers, unlike
  1.0's once-per-round cap.
- Ties go to the first card played.
- A hidden sigil reveals itself the first time it changes a score, a legal
  play, or a trick's winner.
- Engravings are hidden from opponents until their card is played, and are
  destroyed when the card is sold. A gold engraving was left out.
- Wild left with the other suit- and rank-changing engravings, taking its
  special rules with it. Rainbow now leans on synthetic cards and a card-offer
  enabler.
- A synthetic offer is J, Q, K, or A and replaces a random unowned 2–9 of the
  same suit, named on the offer. Selling it puts the replaced card back.
- A run is one 8-round match. Equal totals draw, scores can go negative, and
  meta-progression is out of scope.
- Each AI team's card-owning seat is chosen at random per run. On a team of
  two humans, each offer's tag is random.
- Rarity odds are 69/25/5/1, with legendaries in normal offers; prices are
  50/75/100/150.
- Named suits replace the adaptive suit family; any adaptive one-off needs
  its own explicit definition and evidence.
- Hold sigils count both partners' hands after bidding.
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
| Bidding blind | A sigil that grants blind nil, worth ±200 |
| Throwing off cards | A generic discard payoff or two |
| While held | At most one or two sigils; no engraving uses it |
| Swap meet | Pre-bid card movement sigils, mostly serving as Nil enablers |
| Your partner winning tricks | Mostly moot, since team-scoped sigils treat both partners' tricks alike |
| Bonus chaser | Generic contract-point sigils |
| Contract attacker | A few payoffs ending "if the opponents miss their contract"; gate 3 applies |
| Nil guard | Pre-bid passing serves it as a Nil enabler |
| Gold miner | Economy utility only; no sigil scores from gold held |

## Appendix E: Seeds from Rogue Spades 1.0

These 1.0 sigils seed the hand-designed waves. The historical 1.0 text below
is quoted unchanged; every 2.0 candidate must use [§5](#rules-text), simplify
its conditions under D24, and pass simulation. Colors and resonances are dropped.

| 1.0 sigil | 1.0 text | 2.0 seed |
| --- | --- | --- |
| Unclouded Sun (DU-S01) | Your aces can't be trumped. | Ranks rule-bender enabler |
| Regal Summit (RE-C02) | Your kings become aces. | Ranks: "Your team's Kings count as Aces for sigils" |
| Rosy Spectacles (BL-U11) | Your team's hearts can't be trumped. | Suits rule bender, with ♣ ♦ ♥ versions as an option |
| Arena's Law (RE-U01) | You can lead spades at any time. | Spades rule-bender enabler |
| Mauling Bear (GR-U03) | The first time each round you trump, you win that trick. | Spades: "Your team wins the first trick it trumps" |
| Headsman's Axe (RE-C08) | Whenever you win a trick with a spade, gain +10 contract value. | Named-♠ Win payoff |
| Schooling Fish (GR-C13) | If you win three or more tricks with diamonds in a round, gain +40 contract value. | Named-♦ milestone payoff |
| Evening Melody (GR-R03) | If you win three or more tricks with hearts in a round, gain +1× contract multiplier. | Named-♥ multiplier candidate |
| Laurel Finale (DU-R01) | If your team wins the last trick of a round with a heart, gain +1× contract multiplier. | Streaks or Suits |
| Rising Flame (RE-C10) | Whenever you win a trick after winning the previous one, gain +10 contract value. | Streaks common |
| Early Sprint (RE-C05) | Whenever you win one of the first three tricks, gain +10 contract value. | Streaks common |
| Honest Ruler (BL-C10) | If your team makes its contract exactly, gain +30 contract value. | Exact common |
| True Aim (BL-R04) | If your team makes its contract exactly, gain +1× contract multiplier. | Exact +mult |
| Balanced Yin-Yang (DU-S13) | If you take exactly your own bid, gain +1× contract multiplier. | Exact: "×1.5 contract multiplier if your team makes its contract exactly" |
| Square Meal (GY-R04) | After bidding, if your team's contract is 9 or more tricks, gain +1× contract multiplier. | Bid High: "+1 contract multiplier when your team bids a high contract" |
| Gambler's Wheel (OR-R01) | If your team's contract is 10 or more tricks, gain +2× contract multiplier. | Bid High uncommon +mult |
| Planner's Whiteboard (GY-C18) | Gain +5 contract value for each trick your partner bids. | Bid High: "+5 contract points for each trick in your team's contract" |
| Vigil Candle (PU-C10) | Gain +30 nil value. | Nil: "+50 nil points" |
| Widening Circle (PU-R04) | Your nil value is doubled. | Nil ×mult |
| Sealed Letter (TE-C02) | Before bidding, pass a card to your partner. | Nil enabler (pre-bid movement) |
| Tricolor Triangle (RE-U07) | If you lead three different suits in a round, gain +50 contract value. | Rainbow lead payoff |
| Clean Bullseye (BL-U03) | If you win four or more tricks with cards that aren't spades in a round, gain +1× contract multiplier. | Suits or Rainbow +mult |
| Ticking Bomb (RE-R02) | Each round, once you've trumped three times, gain +1× contract multiplier. | Spades +mult |
| Crown Jewel (RE-C07) | When this card wins a trick, gain +25 contract value. | Bonus engraving |
| Opening Bell (OR-C02) | When you lead with this card, gain +20 contract value. | Herald engraving |
| Loaded Dice (GY-C01) | Shop rerolls cost 20 gold less. | Economy common |
| Collector's Album (GY-C05) | At least one of your shop offers is always uncommon or rare. | Economy common |
| Grand Treasury (OR-U05) | You can earn up to 100 gold of interest each round instead of 50. | Economy uncommon |
