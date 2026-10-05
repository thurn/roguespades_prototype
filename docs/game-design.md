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

This document records the design agreed in the design interview of 2026-10-05.
[Appendix A](#appendix-a-decision-log) lists each decision, and
[Appendix B](#appendix-b-calls-made-without-a-dedicated-question) lists the
calls made without a dedicated question. It builds on two predecessors:

- **Rogue Spades 1.0** (`~/rsp`): its
  [rules](../../rsp/docs/game-overview.md),
  [archetypes](../../rsp/docs/archetypes.md),
  [AI plan](../../rsp/docs/automation-plan.md), and TypeScript code, much of
  which carries over ([§13](#what-carries-over-from-10)).
- **Bridge of Rogues** (`~/rbp`): its [design](../../rbp/docs/game-design.md)
  of the shop, owned cards, engravings, payoffs and enablers, the pool
  skeleton, and sigil validation.

> **Every sigil and number in this document is an example or a starting
> value.** Each one must pass simulation before it enters the game.

## At a glance

| Topic | Rule |
| --- | --- |
| Golden Rule | A rule, sigil, or number is kept only if simulation shows it meets the metrics ([§12](#12-metrics-what-fun-means)) |
| Run | 8 rounds; each round is a shop, then a deal of Spades |
| Spades | Partners' bids add up to a team contract; nil; spades must be broken; no blind nil; no bags |
| Scoring | (10 × contract + contract points + nil points) × (1 + Σ +multipliers) × Π ×multipliers; overtricks score nothing |
| Failure | A set team loses its contract points and scores (−10 × contract + nil points) × its multipliers |
| Victory | Most points after round 8; equal totals draw |
| Sigils | Up to 7 per team, shared by both partners; hidden from opponents until they first trigger |
| Cards | Each card exists once; a bought card is dealt to its owner every round; at most 8 owned per player |
| Engravings | Seven fixed types on card offers from shop 3; never bought alone |
| Shop | 3 sigil and 3 card offers per team; unlimited purchases; purely random offers within rarity |
| Income | 100 base gold + 10 per trick in a made contract + interest |
| Archetypes | Majors: Suits (♣ ♦ ♥), Spades, Ranks, Bid High, Nil. Minors: Low Cards, Rainbow, Streaks, Exact. Never named in the game |
| Simulation | Declarative sigils on one rules kernel shared by the game and the AI; three AI tiers; 2,000-run experiments in about 10 minutes |
| Prototype | Single-player: you and an AI partner against an AI team |

## Design pillars

1. **Simulation decides.** Design questions are answered by experiments, not
   intuition. This document defines fun as measurable metrics. The game is
   built so the AI can play, bid, buy, and measure a sigil the moment it is
   written.
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

### Bidding

- Each player bids **nil** or a number from 1 to 13.
- A team's **contract** B is the sum of its non-nil bids. The team makes its
  contract if its non-nil bidders win at least B tricks.
- **Nil** promises to win no tricks. Tricks won by a nil bidder never count
  toward the partner's contract. If both partners bid nil, the team has no
  contract.
- **Blind nil** is not part of the base rules. A one-off sigil can grant it
  ([Appendix C](#appendix-c-one-offs-not-archetypes)).

### Play

- Players must follow suit if able. A player who can't may play any card.
- Spades are trump. The highest spade wins; otherwise the highest card of the
  led suit wins.
- Spades can't be led until a spade has been played on another suit, unless
  the leader holds only spades.
- **Effective cards.** Engravings and rule benders can change a card's suit or
  rank. A changed card counts at its new identity everywhere: following suit,
  winning tricks, and sigil conditions. Ranks cap at ace.
- **Ties.** Two cards can share an effective suit and rank. Between equal
  cards, the first one played wins.

### Wild cards

The Wild engraving ([§7](#7-engravings)) makes a card count as every suit:

- It is always a legal play, and it counts as the suit led.
- When its holder leads it, they name its suit. They may name spades only if
  spades are broken, or if they hold nothing but spades and wild cards.
- It never forces its holder to follow suit. A player holding a wild may still
  trump or discard.
- It wins only as the led suit. It is never a trump unless spades were led.
- For sigils, a trick won with a wild counts as won with every suit. Longest
  suits and voids are judged on non-wild cards only.

## 4. Scoring

### The formula

```
round score = ( 10 × B               base contract
              + contract points      earned from sigils and engravings this round
              + nil points )         +100 plus nil bonuses per made nil, −100 per failed nil
            × (1 + Σ +contract multipliers)
            × Π ×contract multipliers
```

With no sigils this is Spades without bags: a made contract of 7 scores 70, and
a set scores −70.

### Three categories

| Category | Balatro analog | Example (not final) |
| --- | --- | --- |
| +Contract points | Chips | "Tricks you win with an ace: +20 contract points." "Your high contracts: +60 contract points." |
| +Contract multiplier | +Mult | "+1× contract multiplier if your team wins the last trick." |
| ×Contract multiplier | ×Mult | "×1.5 contract multiplier if your team makes exactly its contract." |

- **+Contract multipliers add up.** Two +1× sigils make ×3.
- **×Contract multipliers compound** and apply last, so their order never
  matters. They can appear at any rarity, as in Balatro, and the
  [skeleton](#10-sigil-pool-skeleton) sets how many sit at each one.
- **Contract points accumulate during the round.** They come per event ("each
  trick you win with an ace") or flat ("for your high contracts").
- **Nil bonuses raise nil points instead.** A sigil such as "Your made nils are
  worth +50 more" adds to nil points, so it counts even when the contract is
  set.

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
- **Nil points** count whether the contract is made or set.
- Contract points triggered by a trick that a nil bidder wins don't count.
- A team whose partners both bid nil has no contract. Its nil points are still
  multiplied, and any contract points are lost.

### Worked examples

| Situation | Calculation | Score |
| --- | --- | --- |
| Bids 4 + 3, win 9, no sigils | 70 | 70 |
| Bids 4 + 3, win 6 | −70 | −70 |
| Bid 7, win 8; "aces: +20" fires twice; one +1× holds | (70 + 40) × 2 | 220 |
| Same, but win 6 | −70 × 2 | −140 |
| Partner bids 5 and wins 6; you make nil; "made nils +50"; one +1× | (50 + 150) × 2 | 400 |
| Same, but your nil fails | (50 − 100) × 2 | −100 |
| Late run: bid 9 and make it; 210 contract points; +3× from sigils; one ×1.5 | (90 + 210) × 4 × 1.5 | 1,800 |
| Same, set, with every multiplier's condition still met | −90 × 4 × 1.5 | −540 |

### Keeping bids tense

A set loses every contract point, while each extra bid trick adds only 10
before multipliers. Left alone, large point totals would push late-run teams to
bid safe and drain the tension from the bidding. The pool is built against
this:

- At least a third of contract-point sigils and a quarter of multipliers scale
  with or require the contract size. Examples: "+5 contract points for each
  trick in your contract", "+1× contract multiplier for your high
  contracts".
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

### Slots and ownership

- Sigils belong to the team, **up to 7 per team**. Buying one needs a free
  slot, so a full team sells first. Utility sigils can add slots.
- "You" and "your" in sigil text mean your team. A trick either partner wins is
  a trick you win.
- A team is never offered a sigil it already owns. Both teams may own the same
  sigil.
- A sigil sells for half its price, rounded down to a multiple of 5.
- Sigils never have activated abilities. A sigil may offer a choice at a fixed
  moment, such as "Before bidding, you may pass a card to your partner."

### Scope words

| Phrase | Meaning |
| --- | --- |
| Tricks you win | Tricks either partner wins, except tricks a nil bidder wins |
| Your team holds | Both partners' hands at the start of bidding |
| Your longest suit | For each player, the suit they hold the most cards of at the start of bidding. Every tied suit counts, and spades count |
| Your contract | The team's contract, B |
| A high contract | A contract of 8 or more |
| A low card | A card ranked 2 through 10 |
| Tricks you lead with | Pays when the card is led, whether or not the trick is won |

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
    "Your aces can't be trumped." "You may lead spades at any time."
  - Deal control is pre-bid card movement between partners.

Two families are excluded:

- **Information** effects, such as revealing an opponent's cards. Their
  contribution is hard to prove, and they weaken the AI's deal sampling.
- **Randomness after the deal**, such as random ranks during play. That is
  output randomness. Random effects at the deal are input randomness and are
  allowed.

### Simplicity rules

- Every sigil has one trigger or condition and one effect, with no riders, at
  every rarity.
- Commons have one noun and one effect, in about 12 words: "Tricks you win with
  an ace: +20 contract points."
- Uncommons and above may add one threshold ("10 or more") or one run-long
  counter ("for each … this run"). Counting a noun ("for each ♥ your team
  holds") is fine at any rarity.
- Rules text is generated from the sigil's data
  ([§13](#declarative-sigils)), so it always matches behavior. A lint enforces
  these limits.

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

- The deck is the standard 52 cards, and each card exists once.
- Buying a card means it is **dealt to its owner every round**.
- Each player owns **at most 8 cards**, so every hand keeps at least 5 random
  cards. Selling a card makes room. The cap is a tuning lever.
- Every card offer is tagged with the player who will own it, and the tag
  can't change.
  - In single-player, your team's cards always go to you. Each AI team has one
    card-owning seat, chosen at random for the run.
  - On a team of two humans, each offer is tagged to one partner at random.
- Card offers are drawn uniformly from cards nobody owns and nobody else is
  being offered, so two teams never compete for the same offer.
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

1. Each player receives all of their owned cards.
2. All other cards are shuffled and dealt to fill every hand to 13.

## 7. Engravings

An engraving is a fixed modifier printed on a card offer. Engravings can't be
bought alone, take no sigil slot, and stay with the card while it's owned.

- Card offers carry engravings from **shop 3** on: a quarter of card offers in
  shop 3, rising by 10 points per shop to three quarters in shop 8.
- Each card holds at most one engraving.
- An engraving works for the card's holder, who is always its owner.
- Engravings are hidden from opponents until the card is played.

The catalog has seven types in two families:

| Engraving | Family | Effect | Premium |
| --- | --- | --- | --- |
| Bonus | Scoring | +20 contract points when this card wins a trick | 25 |
| Herald | Scoring | +20 contract points when you lead this card | 25 |
| Multiplier | Scoring | +1× contract multiplier when this card wins a trick; offered half as often | 50 |
| Wild | Identity | Counts as every suit ([wild cards](#wild-cards)) | 50 |
| Converted ♣ / ♦ / ♥ / ♠ | Identity | This card is the named suit; four versions sharing one offer type | 25 |
| Crowned | Identity | Counts as an ace for your sigils, but plays at its own rank | 25 |
| Raised | Identity | +1 rank, up to ace | 25 |

Identity engravings are an enabler channel, and they combine with payoffs in
ways players discover. A Converted card lengthens a suit, a Crowned card adds
an ace for Ranks, and a Wild card completes a Rainbow.

## 8. Shops and economy

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
| One side suit: its cycle members and the adaptive suit sigils | about 9 | about 3 |
| One specific card | — | seen in about two runs out of three |

- **Cards are the dense enabler.** A team sees most cards at some point,
  including about four ace offers per run.
- **Sigil payoffs are the thin resource.** The
  [Commitment works](#fun-score) family decides whether density levers are
  needed.

## 9. Archetypes

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
  ([Appendix C](#appendix-c-one-offs-not-archetypes)).

### Ways to score

Each archetype rewards one idea through several triggers, so its sigils stack
instead of competing.

| Trigger | What it checks | Example |
| --- | --- | --- |
| Hold | Both partners' hands at the start of bidding | +20 contract points for each ace your team holds. |
| Lead | Cards your team leads | Tricks you lead with an ace: +15 contract points. |
| Win | The card that wins a trick for your team | Tricks you win with an ace: +20 contract points. |
| Trump | Winning a trick by trumping | Tricks you win by trumping: +20 contract points. |
| Bid | The contract's size, or a nil bid | +1× contract multiplier for your high contracts. |
| Make | The round's result | +1× contract multiplier if your team makes exactly its contract. |

### Payoffs and enablers

Point sigils are **payoffs**: they score when something happens. **Enablers**
make that thing happen more often. Enablers come from three places:

- **Cards:** buying the suit, rank, or length a payoff counts. Card offers are
  dense, so this is the main enabler.
- **Engravings:** Converted makes length, Crowned and Raised make aces, and
  Wild makes rainbows.
- **Enabler sigils:** shop steering ("one card offer in each of your shops is
  from the suit you own the most cards of"), discounts, and rule benders.

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

**Plan.** Pick a side suit (♣, ♦, or ♥). Buy its honors or its length,
Convert cards into it, and hold, lead, and win with it. Honors and length are
two lines of one archetype that share its sigils:

- The **honors** line owns the A, K, and Q and cashes them early.
- The **length** line owns many cards of the suit. Long side suits get trumped
  once opponents run out, so this line leans on Hold and Lead payoffs and on
  "can't be trumped" enablers.

**Cycles.** Three templates have identical members in ♣, ♦, ♥, and ♠, so
learning one member teaches them all. They are shown for ♥:

| Template | Rarity, category | Text |
| --- | --- | --- |
| Win | C, points | Tricks you win with a ♥: +15 contract points. |
| Hold | C, points | +10 contract points for each ♥ your team holds. |
| Honors | U, +mult | +2× contract multiplier if your team holds the A, K, and Q of ♥. |

The ♠ members also feed the Spades major.

**Adaptive suit sigils.** These work for whichever suit you build. Every hand
has a longest suit, so they fire from round 1, and buying hearts makes hearts
your longest suit reliably.

| Rarity, category | Text |
| --- | --- |
| C, points | Tricks you win with your longest suit: +10 contract points. |
| C, points | Tricks you lead with your longest suit: +10 contract points. |
| C, points | +5 contract points for each card in your longest suit. |
| U, +mult | +1× contract multiplier if a player on your team holds seven or more cards of one suit. |
| U, ×mult | ×1.5 contract multiplier if your team wins five or more tricks with one suit. |
| R, +mult | +1× contract multiplier for each card beyond six in your longest suit. |

**Enablers:**

- [C] One card offer in each of your shops is from the suit you own the most
  cards of.
- [C] Cards of the suit you own the most of cost you 10 less.
- [R, rule] Cards of your longest suit can't be trumped.

**Notes:**

- **Buys:** the suit's honors, or its cheap low cards for length.
- **Engraves:** Converted into the suit; Raised on its honors.
- **Threat:** opponents void the suit and trump it.

### Spades

**Plan.** Own long or high spades. Spades win in every hand: high spades cash,
and low spades trump. The ♠ members of the suit cycles score here too.

**Enablers:**

- [C] Spades cost you 15 less.
- [U, rule] You may lead spades at any time.
- [R, rule] The first trick you trump each round can't be overtrumped.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Trump | [C, points] Tricks you win by trumping: +20 contract points. |
| Lead | [C, points] Tricks you lead with a spade: +10 contract points. |
| Hold | [U, points] +15 contract points for each spade your team holds beyond five. |
| Trump | [U, +mult] +1× contract multiplier if your team wins three or more tricks by trumping. |
| Hold | [R, +mult] +1× contract multiplier for each spade your team holds beyond seven. |
| Win | [R, ×mult] ×2 contract multiplier if your team wins the last four tricks with spades. |

**Notes:**

- **Buys:** spades, both high and low.
- **Threat:** opponents strip trumps by leading spades.

### Ranks

**Plan.** Collect one rank, usually aces, and hold, lead, and win with it. Aces
win in every side suit and cost the most. A few sigils pay for kings instead.

**Enablers:**

- [C] Aces cost you 25 less.
- [U, rule] Your kings count as aces for your sigils.
- [R, rule] Your aces can't be trumped.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Win | [C, points] Tricks you win with an ace: +20 contract points. |
| Hold | [C, points] +20 contract points for each ace your team holds. |
| Lead | [C, points] Tricks you lead with an ace: +15 contract points. |
| Win | [C, points] Tricks you win with a king: +15 contract points. |
| Hold | [U, +mult] +2× contract multiplier if your team holds all four aces. |
| Win | [U, ×mult] ×1.5 contract multiplier if your team wins three or more tricks with aces. |
| Win | [R, +mult] +1× contract multiplier for each trick you win with an ace. |

**Notes:**

- **Buys:** aces, kings to Raise, and Crowned cards.
- **Threat:** aces cost the most, and they get trumped once a suit runs out.

### Bid High

**Plan.** Bid high contracts, 8 or more, and make them. Bid High pays for
bidding high, so its risk lives in the bidding, and a set multiplies against
you.

**Enablers:**

- [C] Cards ranked 10 through ace cost you 10 less.
- [U, rule] Your high contracts are made with one trick fewer.
- [R, rule] Tricks you win with an ace count as two tricks toward your
  contract.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Bid | [C, points] +5 contract points for each trick in your contract. |
| Bid | [C, points] Your high contracts: +60 contract points. |
| Bid | [C, +mult] +1× contract multiplier for your high contracts. |
| Bid | [U, +mult] +2× contract multiplier if your contract is 10 or more. |
| Bid | [U, points] +15 contract points for each trick in your contract beyond seven. |
| Make | [R, +mult] +1× contract multiplier for each high contract your team has made this run. |
| Bid | [R, ×mult] ×2 contract multiplier if your contract is 11 or more. |

**Notes:**

- **Buys:** high cards and spades.
- **Threat:** a set multiplies against you, and opponents defend hard against
  big contracts.

### Nil

**Plan.** Bid nil and make it. Nil points join the base, so every multiplier
also grows your nils, and a failed nil costs more as the build grows.

**Enablers:**

- [C] Cards ranked 2 through 6 cost you 5 less.
- [U, movement] Before bidding, each of you may pass a card to your partner.
- [U, rule] Your nil bidders' aces count as twos.

**Payoffs:**

| Trigger | Example |
| --- | --- |
| Make | [C, points] Your made nils are worth +50 more. |
| Bid | [C, +mult] +1× contract multiplier if a player on your team bids nil. |
| Make | [U, ×mult] ×1.5 contract multiplier if your team makes a nil. |
| Make | [R, points] Your made nils are worth +40 more for each nil your team has made this run. |

**Notes:**

- **Buys:** low cards. Nil teams avoid owning aces and high spades.
- **Threat:** opponents lead low to force the nil bidder to win a trick.

### Low Cards

**Plan.** Win tricks with 2s through 10s, the cheapest cards in the shop. Low
cards win through length, by trumping, and after the honors are gone.

- **Enablers:**
  - [C] Low cards cost you 5 less.
  - [U, rule] Your jacks count as low cards for your sigils.
- **Payoffs:**
  - [C, points] Tricks you win with a low card: +15 contract points.
  - [C, points] Tricks you lead with a low card: +10 contract points.
  - [U, +mult] +1× contract multiplier if your team wins four or more tricks
    with low cards.
  - [R, points] Tricks you win with a 2: +150 contract points.

### Rainbow

**Plan.** Win tricks with cards of all four suits. One Wild card can complete
the set, which makes Wild the build's key engraving.

- **Enablers:**
  - [C] Your card offers carry Wild engravings twice as often.
  - [U, rule] Tricks you win by trumping count as the suit led for your
    sigils.
- **Payoffs:**
  - [C, points] The first trick you win with each suit: +25 contract points.
  - [C, +mult] +1× contract multiplier if your team wins tricks with all four
    suits.
  - [U, ×mult] ×1.5 contract multiplier if your team leads all four suits.
  - [R, ×mult] ×2 contract multiplier if your team wins two or more tricks
    with each suit.

### Streaks

**Plan.** Win tricks in a row. Each hand becomes a sequencing puzzle: give up
your losers at the right moment, then keep the lead.

- **Enablers:**
  - [C] The first trick your team loses each round doesn't break a streak for
    your sigils.
  - [U, rule] Your team leads the first trick.
- **Payoffs:**
  - [C, points] Each trick you win: +10 contract points for each trick you won
    in a row before it.
  - [C, +mult] +1× contract multiplier if your team wins the last trick.
  - [U, +mult] +2× contract multiplier if your team wins four tricks in a row.
  - [R, ×mult] ×2 contract multiplier if your team wins every trick after the
    first one it loses.

### Exact

**Plan.** Make exactly your contract. Overtricks already score nothing, and
Exact pays for avoiding them. Exact pairs naturally with Nil and Low Cards.

- **Enablers:**
  - [C] One card offer in each of your shops is a 2 through 5.
  - [U, rule] Once your team has made its contract, its aces and kings count
    as twos.
- **Payoffs:**
  - [C, points] +50 contract points if your team makes exactly its contract.
  - [C, +mult] +1× contract multiplier if your team makes exactly its
    contract.
  - [U, ×mult] ×1.5 contract multiplier if each partner wins exactly their own
    bid.
  - [R, +mult] +1× contract multiplier for each exact contract your team has
    made this run.

### Generic

Generic point sigils fit any build and carry runs while a plan comes together:

- [C, points] +5 contract points for each trick your team wins.
- [C, points] +40 contract points.
- [U, +mult] +1× contract multiplier.
- [R, ×mult] ×1.5 contract multiplier.
- [L, ×mult] ×2 contract multiplier.

Generic enablers steer offers toward whatever you're already building. They
are also the [card-lean lever](#15-risks-and-tuning-levers), offered as
sigils:

- [U] Your card offers favor cards that your sigils mention.
- [U] Your shops offer a fourth sigil.

Utility examples:

- **Economy and shop:** the first reroll in each shop is free; your interest
  cap rises by 50; sigils cost you 10 less; this sigil's sell value rises by
  15 after each round.
- **Legendary utility:** +1 sigil slot.

## 10. Sigil pool skeleton

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
| Suit cycles (♣ ♦ ♥ ♠) | 8 | 4 | — | 12 |
| Adaptive suits | 3 | 2 | 1 | 6 |
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

Spades also gets the three ♠ cycle members, so it effectively has 11 point
sigils.

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
| Common | +15 to +20 per narrow event; +5 per trick in the contract; +40 to +60 flat | +1× on a condition met in about a third of made contracts | ×1.5 on a narrow condition |
| Uncommon | +30 per event, or +15 per card beyond a threshold | +2× on a moderate condition, or +1× broadly | ×1.5 broadly, or ×2 narrowly |
| Rare | +50 per event, or a run-long counter | +1× per counter step | ×2 broadly, or ×3 narrowly |
| Legendary | — | — | ×2 to ×3 broadly |

Checks against par:

- **Round 4 (par 160).** A team holds three commons: "aces: +20" (firing
  twice), "+5 per trick in your contract", and a +1× that holds in a third of
  made contracts. A made 8 averages (80 + 40 + 40) × 1.33 ≈ 215. With one set
  in five at −80 × 1.33 ≈ −105, it averages about 150 per round.
- **Round 8 (par 600).** A team holds two contract-point sigils worth +85 on a
  made 9, an uncommon +2× and a common +1× that both hold, and a ×1.5 that
  holds half the time. A made contract scores (90 + 85) × 4 × 1.25 ≈ 875. With
  one set in four at −90 × 4 × 1.25 = −450, it averages about 540.

## 11. Multiplayer

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
| 4. Close and live | 30 | Close and meaningful at every stage, with no runaway leader | Median final margin at most 20% of the winner's score; the team trailing after round 5 wins at least 25% of runs; rounds 1–4 hold at least 20% of all points |
| 5. Commitment works | 20 | I can "do the thing" if I commit | Builds committed by shop 2 are online by round 4 in at least 60% of runs; committed teams win at least as often as flexible ones |
| 6. Archetypes viable | 20 | Every archetype is viable, no build is forced, nothing railroads | Each archetype's committed win rate is 40–60% against a flexible field; no archetype appears in more than 25% of winning flexible builds; two-archetype builds win within 5 points of pure ones |
| 7. Synergy and combos | 15 | Pieces multiply each other, and broken combos are there to discover | Same-archetype pairs beat the sum of their parts by at least 25% on average; at least 15 strong pairs (50% over their parts), at least 5 of them crossing archetypes |
| 8. Skill and bidding | 15 | Good play wins, and bidding stays tense | Tier 2 beats tier 0 in at least 65% of paired runs; late-run (rounds 6–8) overtricks average at most 1 per made contract; set rate 10–25% |

- **Scoring the families.** Each sub-metric scores 1 inside its band and falls
  linearly to 0 at a tolerance listed with it. A family scores the mean of its
  sub-metrics. The fun score is the weighted sum, from 0 to 100.
- **Using it.** A change to the pool or a lever is kept when every gate still
  passes and the fun score doesn't fall.
- **Online** means holding two of the archetype's payoffs and one of its
  enablers. Three owned cards that its payoffs reward count as an enabler.
- **Flexible and committed teams.** A flexible team buys by plain value. A
  committed team adds a bonus for its archetype's sigils and for the cards
  they reward ([shop AI](#shop-ai)).

### Diagnostics

These are tracked but not weighted:

- **Input randomness:** random cards per hand, which the card cap keeps at 5
  or more. There's no randomness after the deal, by rule.
- **Economy:** gold unspent at the end of a run; purchases and rerolls per
  shop.
- **Simplicity:** the text lint ([§5](#simplicity-rules)).
- **AI play quality:** 1.0's bench checks, such as wasted overtakes, missed
  nil covers, and nil suicides.

### Results

- Each sigil's data file records its latest gate numbers and the stage that
  produced them, so later changes are compared against numbers.
- A sigil that fails is retuned, usually by a number sweep, or cut.
- An archetype whose sigils keep failing drops to one-offs.

## 13. Simulation

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

generates "Tricks you win with an ace: +20 contract points." A rule bender
names one of a small set of rule hooks written in code:

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
- **Card filters:** rank, rank range, suit, longest suit, and spades.
- **Scaling:** per card held, per trick in the contract, and per run-long
  counter.
- **Thresholds.**
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
cards. No tier ever reads hidden cards.

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
| 4. Engravings | The seven-type catalog on card offers | The same gates, by engraving type |
| 5. Uncommons | Hand-designed waves of about 1.5 candidates per slot, seeded from 1.0, including most enablers and rule benders | The same gates, plus enabler lift |
| 6. Rares and legendaries | The last 25 sigils | The same gates, plus the power ceiling |
| 7. Validation | The full pool at tier 2 | Thresholds recalibrated; tiers calibrated; the two-human configuration checked; human playtests |

- **Choosing the pool.** Simulation decides pass or fail and the numbers.
  Designers choose among passing candidates for variety, clarity, and
  archetype coverage. For example, the 42 common payoffs are picked from the
  enumerated survivors.
- **Hand play from stage 1.** The ported UI lets the designer play each
  stage's pool by hand. Hand play checks the goals simulation can't measure,
  such as simplicity and the feel of railroading, and catches AI blind spots
  early.
- **Mining 1.0.** Higher-rarity waves start from the 1.0 sigils that fit
  2.0's rules ([Appendix D](#appendix-d-seeds-from-rogue-spades-10)).

## 15. Risks and tuning levers

| Risk | Lever |
| --- | --- |
| Purely random offers leave committed builds thin | Run pools (K random archetypes per run, never named), affinity weighting, card lean, a fourth sigil offer, cheaper rerolls |
| Late-run teams bid safe despite the bid-scaled pool | Share of bid-scaled sigils, Bid High budgets, and the set penalty |
| Low-rarity ×multipliers overshoot the ×10 curve | ×Multiplier counts and sizes per rarity |
| Long side suits get trumped, so the Suits length line fails | Hold and Lead payoffs, "can't be trumped" enablers, and Converted engravings |
| Hidden sigils make AI opponents misread builds | A prior over unrevealed sigils, built from offer odds |
| The 8-card cap is too loose or too tight for input randomness | The cap, between 6 and 10 |
| Two-human teams (16 cards) outscore single-player par | A per-mode cap; otherwise accepted as a secondary configuration |
| Rescoring undervalues enablers and rule benders | Fitted values and committed-policy bonuses |
| Tiers 0 and 1 rank sigils differently from tier 2 | Calibration at each stage; borderline sigils promoted to tier 2 trials |
| Partners knowing each other's owned cards distorts Spades play | Accepted; the AI uses the same information |
| Nil is weak in single-player, where the AI partner decides its own nils | The partner bids nil by expected score, including team sigils |
| Experiments run too slowly | Tier budgets, and fewer runs thanks to paired seeds |

## Appendix A: Decision log

| # | Question | Decision |
| --- | --- | --- |
| 1 | Which tricks score | Overtricks score nothing, and there are no bags. First decided as "the team scores its B best tricks"; that became 10 × B once #3 dropped per-trick values |
| 2 | Nil scoring | Nil points join the base and are multiplied with it |
| 3 | Scoring categories | +Contract points, +contract multiplier, and ×contract multiplier, with ×multipliers at any rarity. This revised an earlier "exactly three layers: trick value, contract value, contract multiplier" |
| 4 | Score growth | Par about ×10 from round 1 to round 8, 60 to 600 |
| 5 | Card ownership | Each player owns at most 8 cards, all dealt to them every round |
| 6 | Engravings | A small fixed catalog of scoring and identity engravings on card offers from shop 3 |
| 7 | Offer density | Purely random within rarity; density levers held in reserve for the metrics |
| 8 | Bid tension | A set loses all contract points; the pool scales with contract size; a bidding-health metric watches late rounds |
| 9 | Income | Base gold + 10 per trick in a made contract, plus interest |
| 10 | Sigil visibility | Opponents' sigils are hidden until they first trigger |
| 11 | Archetype roster | Majors: Suits, Spades, Ranks, Bid High, Nil. Minors: Low Cards, Rainbow, Streaks, Exact |
| 12 | Suit structure | Three-template ♣ ♦ ♥ cycles (with ♠ where the text fits) plus about 6 "longest suit" sigils |
| 13 | Effect families | Rule benders and pre-bid card movement allowed; information and post-deal randomness excluded |
| 14 | Metric structure | Per-sigil gates plus a weighted fun score: Close and live 30, Commitment 20, Archetypes viable 20, Synergy 15, Skill and bidding 15 |
| 15 | Sigil representation | Declarative data with generated text and named rule hooks |
| 16 | AI architecture | One rules kernel shared by the game and the AI search |
| 17 | Shop AI | Value offers by playing sampled hands with and without them; fitted values for rule benders and economy |
| 18 | Throughput | Three AI tiers; a standard experiment takes about 10 minutes |
| 19 | Simulation configuration | Single-player first: one card-owning seat per team |
| 20 | Staging | Kernel, cards, commons (hand level, then run level), engravings, uncommons, rares, validation; UI port at stage 1 |
| 21 | Candidate generation | Enumerate commons from the grammar; hand-design higher rarities in waves seeded from 1.0 |

## Appendix B: Calls made without a dedicated question

These are vetoable defaults.

- A set team scores (−10 × B + nil points) × its multipliers and loses all
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
- Nil bonuses raise nil points rather than contract points, so they survive a
  set.
- "High contract" (8 or more) and "low card" (2 through 10) are keywords, so
  commons can name them as one noun.
- Ties go to the first card played.
- A hidden sigil reveals itself the first time it changes a score, a legal
  play, or a trick's winner.
- Engravings are hidden from opponents until their card is played, and are
  destroyed when the card is sold. A gold engraving was left out to keep the
  catalog to the agreed two families.
- A run is one 8-round match. Equal totals draw, scores can go negative, and
  meta-progression is out of scope.
- Each AI team's card-owning seat is chosen at random per run. On a team of
  two humans, each offer's tag is random.
- Rarity odds are 69/25/5/1, with legendaries in normal offers; prices are
  50/75/100/150.
- "Longest suit" is judged per player at the start of bidding. Ties count
  every tied suit, spades count, and wild cards don't.
- Hold sigils count both partners' hands at the start of bidding.
- Both teams may own the same sigil.
- Income is 100 base gold, plus 10 per contract trick if made, plus 50 per
  made nil, with interest of 10 per 50 held up to 50.
- Lead sigils pay on the lead, whether or not the trick is won.
- In multiplayer, the first partner to act on an offer gets it, and a team's
  shop closes when both partners press Done.
- Conflicting rule benders resolve by a fixed priority defined in the kernel.

## Appendix C: One-offs, not archetypes

These ideas aren't fun as archetypes, but they can appear as single sigils from
the generic budget.

| Idea | As a one-off |
| --- | --- |
| Bidding blind | A sigil that grants blind nil, worth ±200 |
| Throwing off cards | A generic discard payoff or two |
| While held | At most one or two sigils; no engraving uses it |
| Swap meet | Pre-bid card movement sigils, mostly serving as Nil enablers |
| Your partner winning tricks | Mostly moot, since team-scoped sigils treat both partners' tricks alike |
| Bonus chaser | Generic contract-point sigils |
| Contract attacker | A few "if the opponents are set" sigils; gate 3 applies |
| Nil guard | Pre-bid passing serves it as a Nil enabler |
| Gold miner | Economy utility only; no sigil scores from gold held |

## Appendix D: Seeds from Rogue Spades 1.0

These 1.0 sigils fit 2.0's rules with light rewording, and they seed the
hand-designed waves. Colors and resonances are dropped.

| 1.0 sigil | 1.0 text | 2.0 seed |
| --- | --- | --- |
| Unclouded Sun (DU-S01) | Your aces can't be trumped. | Ranks rule-bender enabler |
| Regal Summit (RE-C02) | Your kings become aces. | Ranks: "Your kings count as aces for your sigils" |
| Rosy Spectacles (BL-U11) | Your team's hearts can't be trumped. | Suits rule bender, with ♣ ♦ ♥ versions as an option |
| Arena's Law (RE-U01) | You can lead spades at any time. | Spades rule-bender enabler |
| Mauling Bear (GR-U03) | The first time each round you trump, you win that trick. | Spades: "The first trick you trump each round can't be overtrumped" |
| Headsman's Axe (RE-C08) | Whenever you win a trick with a spade, gain +10 contract value. | Suit cycle Win template, ♠ member |
| Schooling Fish (GR-C13) | If you win three or more tricks with diamonds in a round, gain +40 contract value. | Suit cycle, uncommon threshold variant |
| Evening Melody (GR-R03) | If you win three or more tricks with hearts in a round, gain +1× contract multiplier. | Suit cycle +mult variant |
| Laurel Finale (DU-R01) | If your team wins the last trick of a round with a heart, gain +1× contract multiplier. | Streaks or Suits |
| Rising Flame (RE-C10) | Whenever you win a trick after winning the previous one, gain +10 contract value. | Streaks common |
| Early Sprint (RE-C05) | Whenever you win one of the first three tricks, gain +10 contract value. | Streaks common |
| Honest Ruler (BL-C10) | If your team makes its contract exactly, gain +30 contract value. | Exact common |
| True Aim (BL-R04) | If your team makes its contract exactly, gain +1× contract multiplier. | Exact +mult |
| Balanced Yin-Yang (DU-S13) | If you take exactly your own bid, gain +1× contract multiplier. | Exact: "each partner wins exactly their own bid" |
| Square Meal (GY-R04) | After bidding, if your team's contract is 9 or more tricks, gain +1× contract multiplier. | Bid High: "+1× contract multiplier for your high contracts" |
| Gambler's Wheel (OR-R01) | If your team's contract is 10 or more tricks, gain +2× contract multiplier. | Bid High uncommon +mult |
| Planner's Whiteboard (GY-C18) | Gain +5 contract value for each trick your partner bids. | Bid High: "+5 contract points for each trick in your contract" |
| Vigil Candle (PU-C10) | Gain +30 nil value. | Nil: "Your made nils are worth +50 more" |
| Widening Circle (PU-R04) | Your nil value is doubled. | Nil ×mult |
| Sealed Letter (TE-C02) | Before bidding, pass a card to your partner. | Nil enabler (pre-bid movement) |
| Tricolor Triangle (RE-U07) | If you lead three different suits in a round, gain +50 contract value. | Rainbow lead payoff |
| Clean Bullseye (BL-U03) | If you win four or more tricks with cards that aren't spades in a round, gain +1× contract multiplier. | Suits or Rainbow +mult |
| Ticking Bomb (RE-R02) | Each round, once you've trumped three times, gain +1× contract multiplier. | Spades +mult |
| Crown Jewel (RE-C07) | When this card wins a trick, gain +25 contract value. | Bonus engraving |
| Opening Bell (OR-C02) | When you lead with this card, gain +20 contract value. | Herald engraving |
| Faithful Dog (GR-C04) | This card counts as all suits. | Wild engraving |
| Honed Edge (RE-C01) | This card gains 1 rank. | Raised engraving |
| Loaded Dice (GY-C01) | Shop rerolls cost 20 gold less. | Economy common |
| Collector's Album (GY-C05) | At least one of your shop offers is always uncommon or rare. | Economy common |
| Grand Treasury (OR-U05) | You can earn up to 100 gold of interest each round instead of 50. | Economy uncommon |
