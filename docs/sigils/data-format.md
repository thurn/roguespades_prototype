# Sigil data files

Every candidate sigil, including cut and replaced ones, has one file at
`data/sigils/<id>.json`. The simulator (`sim/`) owns the format: `rsim
gen-text` fills the derived fields (`text`, `category`, `signature`,
`simplicity`, `touchesOpponents`), the analysis pipeline fills `estimates`,
and the orchestrator sets `status`, `history`, and naming fields.

## Fields

```jsonc
{
  "id": "c-diamond-win",            // stable; a structurally changed sigil gets a new id
  "rarity": "common",               // common | uncommon | rare | legendary
  "price": 50,                      // 50 / 75 / 100 / 150 by rarity
  "role": "payoff",                 // payoff | enabler | hybrid
  "archetypes": ["Suits"],          // internal tags: Suits, Spades, Ranks, BidHigh, Nil,
                                    // LowCards, Rainbow, Streaks, Exact, Generic
  "source": "designed",             // enumerated | designed | gdd-seed | control
  "createdIn": "draft-common",      // the step that created it
  "effect": { ... },                // the grammar below
  "text": "...",                    // generated rules text (never hand-written)
  "category": "points",             // generated: points | mult | xmult | enabler | hybrid
  "signature": "win|suit=D|-|points", // generated: trigger | filter | scaling | effect type
  "touchesOpponents": false,        // generated
  "design": {
    "decision": "...",              // the choice it changes for its owner
    "opponent": "...",              // how it looks from the other side of the table
    "rationale": "...",             // including any guideline departure and why
    "partners": ["..."]             // likely partners (ids or descriptions)
  },
  "status": "candidate",            // candidate | kept | cut | replaced
  "replacedBy": null,               // id, when status is replaced
  "history": [ { "step": "draft-common", "note": "Kept: lift +2.1 [0.8, 3.4] ..." } ],
  "name": null, "icon": null, "iconFamily": null, "iconWord": null,
  "simplicity": {
    "items": [ { "burden": "numeric value", "detail": "+20", "cost": 1 } ],
    "C": 4, "S": 0.2
  },
  "estimates": {                    // absent until measured
    "step": "round-1",
    "exposures": 2100,
    "metrics": [
      { "key": "lift", "label": "Choices matter (win-rate lift, pts)",
        "value": 2.1, "lo": 0.8, "hi": 3.4, "bandLo": 0, "bandHi": 12, "n": 2100 }
    ],
    "amountHistory": [ { "step": "draft-common", "amount": 20 } ],
    "amountSlope": { "value": 1.2, "lo": 0.4, "hi": 2.0 },
    "skillGradient": { "value": 0.4, "lo": -0.8, "hi": 1.6 },
    "pairs": [ { "with": "c-diamond-hold", "synergy": 1.3 } ]
  },
  "reports": ["reports/round-1-tournament.md"]
}
```

Metrics with `bandLo`/`bandHi` of `null` are open on that side.

## Effect grammar

### Payoffs

```jsonc
{ "type": "points" | "mult" | "xmult" | "nilPoints",
  "amount": 20,
  "on": Trigger,          // omitted: always on ("+40 contract points")
  "per": "contractTrick"  // optional: "for each trick in your team's contract"
}
```

- `points` adds contract points (lost on a set), `mult` adds to the +contract
  multiplier (starting 10), `xmult` compounds, `nilPoints` adds to each made
  nil.

### Growth

```jsonc
{ "type": "grow", "kind": "mult" | "points" | "nilPoints" | "xmult",
  "step": 5, "on": Trigger }
```

"This sigil gains +5 contract multiplier every time your team makes its
contract exactly (currently +0)". The accumulated value persists for the run.

### Triggers

All triggers are scoped to "your team". Tricks a nil bidder wins never count.

| Trigger | Text |
| --- | --- |
| `{ "event": "win" }` | when your team wins a trick |
| `{ "event": "win", "card": F }` | when your team wins a trick with F |
| `{ "event": "win", "by": "trump" }` | when your team wins a trick by trumping |
| `{ "event": "win", "trick": "last" }` | when your team wins the last trick of a round |
| `{ "event": "win", "trick": "first" }` | when your team wins the first trick of a round |
| `{ "event": "win", "consecutive": true }` | when your team wins consecutive tricks |
| `{ "event": "win", "inRow": 4 }` | when your team wins four tricks in a row (milestone) |
| `{ "event": "win", "distinct": "suit" }` | when your team wins its first trick with each suit |
| `{ "event": "win", "card": F, "count": 3 }` | when your team wins three tricks with F (milestone, pays once) |
| `{ "event": "lead", "card": F }` | when your team leads F |
| `{ "event": "lead", "card": F, "count": 3 }` | when your team leads three F (milestone) |
| `{ "event": "hold", "card": F }` | for each F your team holds (counted once after bidding) |
| `{ "event": "hold", "card": F, "count": 4 }` | if your team holds four F |
| `{ "event": "bid", "min": 8 }` | when your team bids 8 or more |
| `{ "event": "bid", "max": 4 }` | when your team bids 4 or less |
| `{ "event": "bid", "nil": true }` | when your team bids nil (once per nil bidder) |
| `{ "event": "make" }` | if your team makes its contract |
| `{ "event": "make", "min": 8 }` | if your team makes a contract of 8 or more |
| `{ "event": "make", "exact": true }` | if your team makes its contract exactly |
| `{ "event": "nilMade" }` | if your team makes a nil |
| `{ "event": "suits", "action": "win", "count": 4 }` | if your team wins tricks with all four suits |
| `{ "event": "suits", "action": "lead", "count": 4 }` | if your team leads all four suits |
| `{ "event": "opponentsSet" }` | if the opponents miss their contract |

Card filters `F`: `{ "suit": "C"|"D"|"H"|"S" }`, `{ "rank": "2".."10"|"J"|"Q"|"K"|"A" }`,
`{ "ranks": ["2", "10"] }` (a range), `{ "notSuit": "S" }`, or a suit with a
rank or range.

### Enablers and rule benders

| Effect | Text |
| --- | --- |
| `{ "type": "swap", "count": 3 }` | Opening: Swap three cards with your partner |
| `{ "type": "become", "count": 4, "rank": "K" }` | Opening: Four cards your team holds become [K]s |
| `{ "type": "become", "count": 4, "suit": "H" }` | Opening: Four cards your team holds become [♥]s |
| `{ "type": "raise", "amount": 2 }` | Opening: Raise every card your team holds by two ranks |
| `{ "type": "leadChoice" }` | Choose which partner leads after your team wins a trick |
| `{ "type": "anySuit", "last": 3 }` | Your team can play any suit on the last three tricks |

New hooks are added as general mechanisms when a design needs one; this
table lists every hook the simulator implements (see `rsim grammar`).
