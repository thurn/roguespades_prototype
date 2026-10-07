//! The §12 simplicity rubric, scored from sigil data.

use crate::sigil::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Item {
    pub burden: &'static str,
    pub detail: String,
    pub cost: u32,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub items: Vec<Item>,
    #[serde(rename = "C")]
    pub c: u32,
    #[serde(rename = "S")]
    pub s: f64,
}

struct Acc(Vec<Item>);
impl Acc {
    fn num(&mut self, d: impl Into<String>) {
        self.0.push(Item {
            burden: "numeric value",
            detail: d.into(),
            cost: 1,
        });
    }
    fn check(&mut self, d: impl Into<String>) {
        self.0.push(Item {
            burden: "boolean condition",
            detail: d.into(),
            cost: 1,
        });
    }
    fn state(&mut self, d: impl Into<String>) {
        self.0.push(Item {
            burden: "tracked state",
            detail: d.into(),
            cost: 1,
        });
    }
    fn selector(&mut self, d: impl Into<String>) {
        self.0.push(Item {
            burden: "computed selector",
            detail: d.into(),
            cost: 1,
        });
    }
    fn rider(&mut self, d: impl Into<String>) {
        self.0.push(Item {
            burden: "arithmetic rider",
            detail: d.into(),
            cost: 2,
        });
    }
    fn term(&mut self, d: impl Into<String>, common: bool) {
        self.0.push(Item {
            burden: "new term",
            detail: d.into(),
            cost: if common { 4 } else { 2 },
        });
    }
    fn filter(&mut self, f: &Option<Filter>) {
        let Some(f) = f else { return };
        if let (Some(s), Some(r)) = (&f.suit, &f.rank) {
            self.check(format!("suit is {s}"));
            self.check(format!("rank is {r}"));
            self.num(format!("rank literal {r}"));
            return;
        }
        if let Some(s) = &f.suit {
            self.check(format!("suit is {s}"));
        }
        if let Some(r) = &f.rank {
            self.check(format!("rank is {r}"));
            self.num(format!("rank literal {r}"));
        }
        if let Some([a, b]) = &f.ranks {
            self.num(format!("range endpoint {a}"));
            self.num(format!("range endpoint {b}"));
            self.check(format!("rank in {a}-{b}"));
        }
        if let Some(s) = &f.not_suit {
            self.check(format!("suit is not {s}"));
        }
    }
    fn trigger(&mut self, t: &Option<Trigger>) {
        let Some(t) = t else { return };
        match t.event.as_str() {
            "win" => {
                self.check(if t.whose.is_some() {
                    "the opponents win the trick"
                } else {
                    "your team wins the trick"
                });
                self.filter(&t.card);
                if t.led.is_some() {
                    self.filter(&t.led);
                }
                if t.by.is_some() {
                    self.check("won by trumping");
                }
                if t.trick.is_some() {
                    self.check(format!(
                        "is the {} trick",
                        t.trick.clone().unwrap_or_default()
                    ));
                }
                if t.consecutive == Some(true) {
                    self.check("your team won the previous trick");
                }
                if let Some(n) = t.in_row {
                    self.num(format!("{n} in a row"));
                    self.check(format!("run of wins reaches {n}"));
                    self.state("consecutive-win count");
                }
                if t.distinct.is_some() {
                    self.check("first win with this suit");
                    self.state("suits already won");
                }
                if let Some(n) = t.count {
                    self.num(format!("milestone {n}"));
                    self.check(format!("count reaches {n}"));
                    self.state("milestone count");
                }
            }
            "lead" => {
                self.check("your team leads");
                self.filter(&t.card);
                if let Some(n) = t.count {
                    self.num(format!("milestone {n}"));
                    self.check(format!("count reaches {n}"));
                    self.state("milestone count");
                }
            }
            "hold" => {
                self.check("held by your team");
                self.filter(&t.card);
                if let Some(n) = t.count {
                    self.num(format!("threshold {n}"));
                    self.check(format!("count at least {n}"));
                }
                if let Some(n) = t.beyond {
                    self.num(format!("cutoff {n}"));
                    self.check(format!("count above {n}"));
                    self.rider("subtract the cutoff and floor at zero");
                }
            }
            "bid" => {
                if t.nil == Some(true) {
                    self.check("bids nil");
                }
                if t.vs.is_some() {
                    self.check("contract larger than the opponents'");
                }
                if let Some(n) = t.min {
                    self.num(format!("threshold {n}"));
                    self.check(format!("contract at least {n}"));
                }
                if let Some(n) = t.max {
                    self.num(format!("threshold {n}"));
                    self.check(format!("contract at most {n}"));
                }
            }
            "make" => {
                if t.exact == Some(true) {
                    self.check("makes its contract exactly");
                } else {
                    self.check("makes its contract");
                }
                if let Some(n) = t.min {
                    self.num(format!("threshold {n}"));
                    self.check(format!("contract at least {n}"));
                }
            }
            "nilMade" => self.check("makes a nil"),
            "suits" => {
                let n = t.count.unwrap_or(4);
                self.check(if t.action.as_deref() == Some("lead") {
                    "your team leads"
                } else {
                    "your team wins the trick"
                });
                self.num(format!("{n} suits"));
                self.check(format!("distinct suits reach {n}"));
                self.state("suits seen");
            }
            "opponentsSet" => self.check("opponents miss their contract"),
            "behind" => self.check("your team has fewer points"),
            _ => {}
        }
    }
}

pub fn score(e: &Effect, rarity: Rarity) -> Score {
    let common = rarity == Rarity::Common;
    let mut a = Acc(vec![]);
    match e.ty.as_str() {
        "points" | "mult" | "xmult" | "nilPoints" => {
            a.num(format!(
                "payout {}",
                crate::text::fmt_num(e.amount.unwrap_or(0.0))
            ));
            if e.per.is_some() {
                a.check("each trick in your team's contract");
            }
            a.trigger(&e.on);
        }
        "grow" => {
            a.num(format!(
                "growth step {}",
                crate::text::fmt_num(e.step.unwrap_or(0.0))
            ));
            a.num("currently display");
            a.state("growth counter carried between rounds");
            a.trigger(&e.on);
        }
        "swap" => {
            a.term("Opening:", common);
            a.num(format!("count {}", e.count.unwrap_or(0)));
        }
        "become" if e.from.is_some() => {
            a.term("Opening:", common);
            if let Some(n) = e.count.filter(|&n| n > 0) {
                a.num(format!("count {n}"));
            }
            a.filter(&e.from);
            if let Some(r) = &e.rank {
                a.num(format!("rank literal {r}"));
            }
            a.check(if e.whose.is_some() {
                "held by the opponents"
            } else {
                "held by your team"
            });
        }
        "become" => {
            a.term("Opening:", common);
            a.num(format!("count {}", e.count.unwrap_or(0)));
            if let Some(r) = &e.rank {
                a.num(format!("rank literal {r}"));
            }
            if let Some([x, y]) = &e.ranks {
                a.num(format!("hidden endpoint {x}"));
                a.num(format!("hidden endpoint {y}"));
                a.selector("destination rank selection");
            }
            if let Some(t) = &e.term {
                a.term(format!("\"{t}\""), common);
            }
            a.check(if e.whose.as_deref() == Some("opponents") {
                "held by the opponents"
            } else {
                "held by your team"
            });
        }
        "raise" if e.from.is_some() => {
            a.term("Opening:", common);
            a.num(format!("raise {}", e.amount.unwrap_or(0.0)));
            a.filter(&e.from);
            a.check("held by your team");
        }
        "beats" => {
            a.check("your team played it");
            a.filter(&e.card);
            if e.over.is_some() {
                a.check("beats trumps too");
            }
            a.selector("ranks above the [A] of its suit");
        }
        "anySuit" if e.card.is_some() => {
            a.check("your team could follow suit");
            a.filter(&e.card);
        }
        "untrumpable" => {
            a.check("your team played it");
            a.filter(&e.from);
            a.check("a [♠] played to the trick");
        }
        "raise" => {
            a.term("Opening:", common);
            a.num(format!("raise {}", e.amount.unwrap_or(0.0)));
            a.check("held by your team");
        }
        "leadChoice" => {
            a.check("your team wins the trick");
            a.selector("choice of leader");
        }
        "anySuit" if e.after.is_some() => {
            a.check("your team has won its contract's tricks");
            a.check("tricks counted against the contract");
        }
        "leadSpades" => {
            a.check("[♠] not yet broken");
            a.check("the lead is a [♠]");
        }
        "firstLead" => a.check("the first trick of a round"),
        "anySuit" => {
            a.num(format!("window {}", e.last.or(e.first).unwrap_or(0)));
            a.check("trick is among the last tricks");
        }
        _ => {}
    }
    let c: u32 = a.0.iter().map(|i| i.cost).sum();
    Score {
        items: a.0,
        c,
        s: 1.0 / (1.0 + c as f64),
    }
}
