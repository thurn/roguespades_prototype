//! The declarative sigil grammar: JSON types, validation, and compilation into the compact form
//! the kernel interprets.

use crate::cards::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Filter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranks: Option<[String; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_suit: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Trigger {
    pub event: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Filter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consecutive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_row: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinct: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nil: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// Control experiments only: the excluded "beyond N" rider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beyond: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Effect {
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<Trigger>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per: Option<String>,
    /// Growth: what grows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suit: Option<String>,
    /// Control experiments only: a rank range destination ("low cards").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranks: Option<[String; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<u8>,
}

impl Effect {
    /// The tunable amount: the payout, the growth step, or the enabler's count.
    pub fn tunable(&self) -> Option<f64> {
        match self.ty.as_str() {
            "grow" => self.step,
            "points" | "mult" | "xmult" | "nilPoints" => self.amount,
            _ => None,
        }
    }
    pub fn set_tunable(&mut self, v: f64) {
        match self.ty.as_str() {
            "grow" => self.step = Some(v),
            "points" | "mult" | "xmult" | "nilPoints" => self.amount = Some(v),
            _ => {}
        }
    }
    pub fn is_payoff(&self) -> bool {
        matches!(
            self.ty.as_str(),
            "points" | "mult" | "xmult" | "nilPoints" | "grow"
        )
    }
    /// The payoff category: points, mult, xmult, or enabler.
    pub fn category(&self) -> &'static str {
        let k = if self.ty == "grow" {
            self.kind.as_deref().unwrap_or("")
        } else {
            &self.ty
        };
        match k {
            "points" | "nilPoints" => "points",
            "mult" => "mult",
            "xmult" => "xmult",
            _ => "enabler",
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Compiled form

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Points,
    Mult,
    XMult,
    NilPoints,
}

/// A card filter on effective identity: allowed suits as bits, and an inclusive rank range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CardFilter {
    pub suits: u8,
    pub lo: u8,
    pub hi: u8,
}
impl CardFilter {
    pub const ANY: CardFilter = CardFilter {
        suits: 0b1111,
        lo: 2,
        hi: 14,
    };
    #[inline]
    pub fn matches(&self, suit: u8, rank: u8) -> bool {
        self.suits & (1 << suit) != 0 && rank >= self.lo && rank <= self.hi
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrickPos {
    Any,
    First,
    Last,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    Always,
    Win {
        filt: CardFilter,
        by_trump: bool,
        pos: TrickPos,
        consecutive: bool,
        in_row: u8,
        distinct_suit: bool,
        count: u8,
    },
    Lead {
        filt: CardFilter,
        count: u8,
    },
    Hold {
        filt: CardFilter,
        count: u8,
        beyond: u8,
    },
    Bid {
        min: u8,
        max: u8,
        nil: bool,
    },
    Make {
        min: u8,
        exact: bool,
    },
    NilMade,
    Suits {
        lead: bool,
        count: u8,
    },
    OpponentsSet,
}

impl Trig {
    /// Fires during trick play (as opposed to after bidding or at scoring).
    pub fn is_trick(&self) -> bool {
        matches!(
            self,
            Trig::Win { .. } | Trig::Lead { .. } | Trig::Suits { .. }
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Payoff {
    pub kind: Kind,
    pub amount: f64,
    pub trig: Trig,
    pub per_contract: bool,
    pub grow: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Rule {
    Swap(u8),
    BecomeRank(u8, u8),
    BecomeSuit(u8, u8),
    BecomeRange(u8, u8, u8),
    Raise(u8),
    LeadChoice,
    AnySuit(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Compiled {
    Payoff(Payoff),
    Rule(Rule),
}

fn parse_suit(s: &str) -> Result<u8, String> {
    suit_from_str(s).ok_or_else(|| format!("bad suit {s}"))
}
fn parse_rank(s: &str) -> Result<u8, String> {
    rank_from_str(s).ok_or_else(|| format!("bad rank {s}"))
}

pub fn compile_filter(f: &Option<Filter>) -> Result<CardFilter, String> {
    let Some(f) = f else {
        return Ok(CardFilter::ANY);
    };
    let mut cf = CardFilter::ANY;
    if let Some(s) = &f.suit {
        cf.suits = 1 << parse_suit(s)?;
    }
    if let Some(s) = &f.not_suit {
        cf.suits &= !(1 << parse_suit(s)?);
    }
    if let Some(r) = &f.rank {
        let r = parse_rank(r)?;
        cf.lo = r;
        cf.hi = r;
    }
    if let Some([a, b]) = &f.ranks {
        cf.lo = parse_rank(a)?;
        cf.hi = parse_rank(b)?;
        if cf.lo > cf.hi {
            return Err("empty rank range".into());
        }
    }
    Ok(cf)
}

pub fn compile_trigger(t: &Option<Trigger>) -> Result<Trig, String> {
    let Some(t) = t else { return Ok(Trig::Always) };
    let filt = compile_filter(&t.card)?;
    let count = t.count.unwrap_or(0);
    Ok(match t.event.as_str() {
        "win" => {
            let by_trump = match t.by.as_deref() {
                None => false,
                Some("trump") => true,
                Some(x) => return Err(format!("bad by {x}")),
            };
            let pos = match t.trick.as_deref() {
                None => TrickPos::Any,
                Some("last") => TrickPos::Last,
                Some("first") => TrickPos::First,
                Some(x) => return Err(format!("bad trick {x}")),
            };
            let distinct_suit = match t.distinct.as_deref() {
                None => false,
                Some("suit") => true,
                Some(x) => return Err(format!("bad distinct {x}")),
            };
            Trig::Win {
                filt,
                by_trump,
                pos,
                consecutive: t.consecutive.unwrap_or(false),
                in_row: t.in_row.unwrap_or(0),
                distinct_suit,
                count,
            }
        }
        "lead" => Trig::Lead { filt, count },
        "hold" => {
            if t.card.is_none() {
                return Err("hold needs a card filter".into());
            }
            Trig::Hold {
                filt,
                count,
                beyond: t.beyond.unwrap_or(0),
            }
        }
        "bid" => Trig::Bid {
            min: t.min.unwrap_or(0),
            max: t.max.unwrap_or(0),
            nil: t.nil.unwrap_or(false),
        },
        "make" => Trig::Make {
            min: t.min.unwrap_or(0),
            exact: t.exact.unwrap_or(false),
        },
        "nilMade" => Trig::NilMade,
        "suits" => Trig::Suits {
            lead: match t.action.as_deref() {
                Some("win") => false,
                Some("lead") => true,
                _ => return Err("suits needs action win or lead".into()),
            },
            count: if count == 0 { 4 } else { count },
        },
        "opponentsSet" => Trig::OpponentsSet,
        x => return Err(format!("unknown event {x}")),
    })
}

fn kind_of(s: &str) -> Result<Kind, String> {
    Ok(match s {
        "points" => Kind::Points,
        "mult" => Kind::Mult,
        "xmult" => Kind::XMult,
        "nilPoints" => Kind::NilPoints,
        x => return Err(format!("unknown payoff kind {x}")),
    })
}

/// Compiles an effect, with `amount` overriding the data's tunable amount when given.
pub fn compile(e: &Effect, amount: Option<f64>) -> Result<Compiled, String> {
    let need = |o: Option<u8>, what: &str| o.ok_or_else(|| format!("{} needs {what}", e.ty));
    Ok(match e.ty.as_str() {
        "points" | "mult" | "xmult" | "nilPoints" => {
            let per_contract = match e.per.as_deref() {
                None => false,
                Some("contractTrick") => true,
                Some(x) => return Err(format!("bad per {x}")),
            };
            Compiled::Payoff(Payoff {
                kind: kind_of(&e.ty)?,
                amount: amount.or(e.amount).ok_or("payoff needs amount")?,
                trig: compile_trigger(&e.on)?,
                per_contract,
                grow: false,
            })
        }
        "grow" => Compiled::Payoff(Payoff {
            kind: kind_of(e.kind.as_deref().ok_or("grow needs kind")?)?,
            amount: amount.or(e.step).ok_or("grow needs step")?,
            trig: compile_trigger(&e.on)?,
            per_contract: false,
            grow: true,
        }),
        "swap" => Compiled::Rule(Rule::Swap(need(e.count, "count")?)),
        "become" => {
            let n = need(e.count, "count")?;
            if let Some(r) = &e.rank {
                Compiled::Rule(Rule::BecomeRank(n, parse_rank(r)?))
            } else if let Some(s) = &e.suit {
                Compiled::Rule(Rule::BecomeSuit(n, parse_suit(s)?))
            } else if let Some([a, b]) = &e.ranks {
                Compiled::Rule(Rule::BecomeRange(n, parse_rank(a)?, parse_rank(b)?))
            } else {
                return Err("become needs rank, suit, or ranks".into());
            }
        }
        "raise" => Compiled::Rule(Rule::Raise(e.amount.ok_or("raise needs amount")? as u8)),
        "leadChoice" => Compiled::Rule(Rule::LeadChoice),
        "anySuit" => Compiled::Rule(Rule::AnySuit(need(e.last, "last")?)),
        x => return Err(format!("unknown effect type {x}")),
    })
}

/// Whether an effect touches the opponents (for counter scans).
pub fn touches_opponents(e: &Effect) -> bool {
    e.on.as_ref().is_some_and(|t| t.event == "opponentsSet")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}
impl Rarity {
    pub fn price(self) -> i32 {
        match self {
            Rarity::Common => 50,
            Rarity::Uncommon => 75,
            Rarity::Rare => 100,
            Rarity::Legendary => 150,
        }
    }
    pub fn odds(self) -> f64 {
        match self {
            Rarity::Common => 0.69,
            Rarity::Uncommon => 0.25,
            Rarity::Rare => 0.05,
            Rarity::Legendary => 0.01,
        }
    }
    pub fn all() -> [Rarity; 4] {
        [
            Rarity::Common,
            Rarity::Uncommon,
            Rarity::Rare,
            Rarity::Legendary,
        ]
    }
    pub fn name(self) -> &'static str {
        match self {
            Rarity::Common => "common",
            Rarity::Uncommon => "uncommon",
            Rarity::Rare => "rare",
            Rarity::Legendary => "legendary",
        }
    }
    pub fn index(self) -> usize {
        self as usize
    }
}

/// The parts of a sigil data file the simulator reads.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SigilDef {
    pub id: String,
    pub rarity: Rarity,
    #[serde(default)]
    pub price: Option<i32>,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub archetypes: Vec<String>,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub status: String,
    pub effect: Effect,
}

impl SigilDef {
    pub fn price(&self) -> i32 {
        self.price.unwrap_or(self.rarity.price())
    }
}

pub const ARCHETYPES: [&str; 10] = [
    "Suits", "Spades", "Ranks", "BidHigh", "Nil", "LowCards", "Rainbow", "Streaks", "Exact",
    "Generic",
];

pub fn archetype_index(a: &str) -> Option<usize> {
    ARCHETYPES.iter().position(|x| *x == a)
}
