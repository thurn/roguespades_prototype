//! An interactive run for the game client: you sit South (seat 0) with an AI partner North,
//! against an AI team East–West. It drives the same `Runner` steps as the simulator.
//!
//! Every decision is an explicit action recorded in the session log, AI decisions included, so
//! feeding a log's actions back with `force` replays the game exactly.

use crate::ai::search::Tier;
use crate::cards::*;
use crate::game::*;
use crate::kernel::*;
use crate::model::Model;
use crate::rng::Rng;
use crate::sigil::*;
use serde::Deserialize;
use serde_json::{json, Map, Value};

pub const HUMAN: u8 = 0;
const ROUNDS: u8 = 8;
pub const SEAT_NAMES: [&str; 4] = ["South (you)", "Nova (W)", "Sage (N)", "Rook (E)"];
const TEAM_NAMES: [&str; 2] = ["Us", "Them"];

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub seed: u64,
    #[serde(default = "default_tier")]
    pub tier: u8,
    /// The AI also plays your seat and shops for your team.
    #[serde(default)]
    pub auto: bool,
    /// Sigil ids the shop offers.
    pub offerable: Vec<String>,
}

fn default_tier() -> u8 {
    1
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Phase {
    Shop,
    Opening,
    Bidding,
    Playing,
    RoundOver,
    GameOver,
}

impl Phase {
    fn name(self) -> &'static str {
        match self {
            Phase::Shop => "shop",
            Phase::Opening => "opening",
            Phase::Bidding => "bidding",
            Phase::Playing => "playing",
            Phase::RoundOver => "roundOver",
            Phase::GameOver => "gameOver",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Shop,
    Swap,
    Bid,
    Play,
    Lead,
    Next,
}

#[derive(Clone, Copy, Debug)]
pub struct Pending {
    pub kind: Kind,
    pub seat: u8,
    /// Swap: cards each partner gives.
    pub n: u8,
}

struct HumanShop {
    sig: Vec<Option<usize>>,
    cards: Vec<Option<CardOffer>>,
    rerolls: u64,
    /// Card slots the AI team was offered, never offered to us.
    exclude: Mask,
}

struct SwapState {
    kt: usize,
    k: usize,
    n: u8,
    seats: [usize; 2],
    give: [Option<Mask>; 2],
}

pub struct Session {
    r: Runner<'static>,
    auto: bool,
    tier: u8,
    round: u8,
    phase: Phase,
    rs: Option<RoundState>,
    shop: Option<HumanShop>,
    op: usize,
    swap: Option<SwapState>,
    nbids: u8,
    result: Option<Value>,
    last_trick: Option<Value>,
    fresh: bool,
    /// Lines only you can see this round (your Opening changes and swaps).
    notes: Vec<Value>,
    events: Vec<Value>,
    seq: u64,
    pub step: u64,
}

fn card_s(id: &Identity, c: u8) -> String {
    card_str(id.suit[c as usize], id.rank[c as usize])
}

fn hand_s(id: &Identity, m: Mask) -> String {
    let mut v: Vec<u8> = Bits(m).collect();
    v.sort_by_key(|&c| std::cmp::Reverse((id.suit[c as usize], id.rank[c as usize])));
    v.iter()
        .map(|&c| card_s(id, c))
        .collect::<Vec<_>>()
        .join(" ")
}

fn bid_s(b: i8) -> String {
    match b {
        0 => "nil".into(),
        b if b < 0 => "–".into(),
        b => b.to_string(),
    }
}

fn round3(v: f64) -> f64 {
    (v * 1000.0).round() / 1000.0
}

fn num(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{v:.0}")
    } else {
        format!("{v:.2}")
    }
}

fn team_name(t: usize) -> &'static str {
    TEAM_NAMES[t]
}

impl Session {
    pub fn new(pool: &'static Pool, model: &'static Model, cfg: Config) -> Session {
        let offerable: Vec<usize> = cfg
            .offerable
            .iter()
            .filter_map(|id| pool.by_id.get(id).copied())
            .collect();
        let tier = Tier::get(cfg.tier);
        let rc: &'static RunCfg<'static> = Box::leak(Box::new(RunCfg {
            pool,
            model,
            tiers: [tier, tier],
            offerable,
            sigil_shop: true,
            teams: [
                TeamCfg {
                    policy: Policy::Flexible,
                    grants: None,
                },
                TeamCfg {
                    policy: Policy::Flexible,
                    grants: None,
                },
            ],
            explore: 0.0,
            perturb: 0.0,
            perturb_amount: 0,
            rounds: ROUNDS,
            cards: true,
            validate: 0.0,
            version: "client".into(),
        }));
        let mut r = Runner::new(rc, cfg.seed, 0);
        // In single-player your team's cards are always yours.
        r.teams[0].owner = 0;
        r.shop_log = Some(vec![]);
        let mut s = Session {
            r,
            auto: cfg.auto,
            tier: cfg.tier,
            round: 1,
            phase: Phase::Shop,
            rs: None,
            shop: None,
            op: 0,
            swap: None,
            nbids: 0,
            result: None,
            last_trick: None,
            fresh: false,
            notes: vec![],
            events: vec![],
            seq: 0,
            step: 0,
        };
        let owner = 1 + 2 * s.r.teams[1].owner;
        s.log(
            "start",
            format!(
                "New game: seed {}, AI tier {}{}; {} sigils offerable; the opponents' cards go to {}",
                cfg.seed,
                cfg.tier,
                if cfg.auto { ", autoplay" } else { "" },
                rc.offerable.len(),
                SEAT_NAMES[owner as usize]
            ),
            json!({"seed": cfg.seed, "tier": cfg.tier, "auto": cfg.auto, "offerable": rc.offerable.len(), "opponentCardSeat": owner}),
        );
        s.open_shop();
        s
    }

    // ----- Log -----

    fn log(&mut self, ty: &str, msg: String, data: Value) {
        let mut m = Map::new();
        m.insert("seq".into(), json!(self.seq));
        m.insert("step".into(), json!(self.step));
        m.insert("round".into(), json!(self.round));
        m.insert("phase".into(), json!(self.phase.name()));
        m.insert("type".into(), json!(ty));
        m.insert("msg".into(), json!(msg));
        if let Value::Object(d) = data {
            for (k, v) in d {
                m.insert(k, v);
            }
        }
        self.seq += 1;
        self.events.push(Value::Object(m));
    }

    /// Events logged since the last call.
    pub fn take_events(&mut self) -> Vec<Value> {
        std::mem::take(&mut self.events)
    }

    fn def(&self, d: usize) -> &SigilDef {
        &self.r.cfg.pool.defs[d]
    }

    fn sigil_label(&self, d: usize) -> String {
        let def = self.def(d);
        match &def.name {
            Some(n) => format!("{n} [{}]", def.id),
            None => def.id.clone(),
        }
    }

    fn drain_shop_log(&mut self) {
        let entries = self
            .r
            .shop_log
            .as_mut()
            .map(std::mem::take)
            .unwrap_or_default();
        for e in entries {
            let team = e["team"].as_u64().unwrap_or(0) as usize;
            let who = team_name(team);
            let msg = match e["ev"].as_str().unwrap_or("") {
                "offers" => format!(
                    "{who} shop offers (gold {}): sigils {:?}; cards {:?}",
                    e["gold"], e["sigils"], e["cards"]
                ),
                "buy" => format!(
                    "{who} AI buys {} {} for {}g (options by net value: {})",
                    e["kind"].as_str().unwrap_or(""),
                    e["offer"].as_str().unwrap_or(""),
                    e["price"],
                    e["options"]
                        .as_array()
                        .map(|a| a
                            .iter()
                            .map(|o| format!("{} {}", o["offer"].as_str().unwrap_or(""), o["net"]))
                            .collect::<Vec<_>>()
                            .join(", "))
                        .unwrap_or_default()
                ),
                "sell" => format!(
                    "{who} AI sells sigil {} to make room",
                    e["sigil"].as_str().unwrap_or("")
                ),
                "sellCard" => format!(
                    "{who} AI sells card {} to make room",
                    e["card"].as_str().unwrap_or("")
                ),
                "reroll" => format!("{who} AI rerolls for {}g", e["cost"]),
                _ => e.to_string(),
            };
            self.log("aiShop", msg, json!({"detail": e}));
        }
    }

    // ----- Decisions -----

    fn is_ai(&self, p: &Pending) -> bool {
        match p.kind {
            Kind::Shop | Kind::Next => self.auto,
            _ => p.seat != HUMAN || self.auto,
        }
    }

    pub fn pending(&self) -> Option<Pending> {
        let p = |kind, seat| Some(Pending { kind, seat, n: 0 });
        match self.phase {
            Phase::Shop => p(Kind::Shop, HUMAN),
            Phase::Opening => {
                let sw = self.swap.as_ref()?;
                let i = sw.give.iter().position(|g| g.is_none())?;
                Some(Pending {
                    kind: Kind::Swap,
                    seat: sw.seats[i] as u8,
                    n: sw.n,
                })
            }
            Phase::Bidding => {
                let rs = self.rs.as_ref()?;
                p(Kind::Bid, (rs.play.dealer + 1 + self.nbids) % 4)
            }
            Phase::Playing => {
                let rs = self.rs.as_ref()?;
                if rs.play.done() {
                    return None;
                }
                let kind = if rs.play.choosing >= 0 {
                    Kind::Lead
                } else {
                    Kind::Play
                };
                p(kind, rs.mover())
            }
            Phase::RoundOver => p(Kind::Next, HUMAN),
            Phase::GameOver => None,
        }
    }

    /// Applies an action. `force` lets a replay make an AI seat's decision.
    pub fn act(&mut self, a: &Value, force: bool) -> Result<(), String> {
        let t = a["t"].as_str().ok_or("action needs a type")?.to_string();
        let res = match t.as_str() {
            "advance" => self.advance(),
            "give" | "take" | "giveCard" | "gold" | "tier" => self.sandbox(a),
            _ => {
                let p = self.pending().ok_or("no decision is pending")?;
                let seat = a["seat"].as_u64().map(|v| v as u8).unwrap_or(p.seat);
                if seat != p.seat {
                    return Err(format!(
                        "it is {}'s decision, not {}'s",
                        SEAT_NAMES[p.seat as usize], SEAT_NAMES[seat as usize]
                    ));
                }
                if self.is_ai(&p) && !force {
                    return Err("that decision belongs to an AI".into());
                }
                self.decide(&t, a, p, "human")
            }
        };
        if let Err(e) = &res {
            self.log(
                "rejected",
                format!("Rejected action {a}: {e}"),
                json!({"action": a, "error": e}),
            );
        }
        res
    }

    fn decide(&mut self, t: &str, a: &Value, p: Pending, by: &str) -> Result<(), String> {
        let u = |k: &str| a[k].as_u64().map(|v| v as usize);
        match (t, p.kind) {
            ("buySigil", Kind::Shop) => self.buy_sigil(u("i").ok_or("needs i")?),
            ("buyCard", Kind::Shop) => self.buy_card(u("i").ok_or("needs i")?),
            ("sellSigil", Kind::Shop) => self.sell_sigil(u("k").ok_or("needs k")?),
            ("sellCard", Kind::Shop) => self.sell_card(u("k").ok_or("needs k")?),
            ("reroll", Kind::Shop) => self.reroll(),
            ("shopDone", Kind::Shop) => {
                self.log(
                    "shopDone",
                    format!("Us: done shopping with {}g", self.r.teams[0].gold),
                    json!({"action": {"t": "shopDone"}, "by": by}),
                );
                self.step += 1;
                self.begin_round();
                Ok(())
            }
            ("autoShop", Kind::Shop) => {
                self.r
                    .shop(0, self.round, self.shop.as_ref().map_or(0, |s| s.exclude));
                self.drain_shop_log();
                self.log(
                    "shopDone",
                    format!("Us (AI): done shopping with {}g", self.r.teams[0].gold),
                    json!({"action": {"t": "autoShop"}, "by": by}),
                );
                self.step += 1;
                self.begin_round();
                Ok(())
            }
            ("swap", Kind::Swap) => {
                let cards: Vec<u8> = a["cards"]
                    .as_array()
                    .ok_or("needs cards")?
                    .iter()
                    .filter_map(|v| v.as_u64().map(|c| c as u8))
                    .collect();
                self.apply_swap(p, cards, by)
            }
            ("bid", Kind::Bid) => {
                let b = a["bid"].as_i64().ok_or("needs bid")?;
                if !(0..=13).contains(&b) {
                    return Err("bids are nil (0) or 1–13".into());
                }
                self.apply_bid(p.seat, b as i8, None, by);
                Ok(())
            }
            ("play", Kind::Play) => {
                let c = a["card"].as_u64().ok_or("needs card")? as u8;
                self.apply_move(p.seat, c, None, by)
            }
            ("lead", Kind::Lead) => {
                let m = if a["pass"].as_bool().unwrap_or(false) {
                    MOVE_PASS
                } else {
                    MOVE_KEEP
                };
                self.apply_move(p.seat, m, None, by)
            }
            ("next", Kind::Next) => {
                self.step += 1;
                self.next_round(by);
                Ok(())
            }
            _ => Err(format!("{t} is not a legal action now")),
        }
    }

    /// Lets the AI make the pending decision, if it belongs to an AI.
    pub fn advance(&mut self) -> Result<(), String> {
        let p = self.pending().ok_or("no decision is pending")?;
        if !self.is_ai(&p) {
            return Err("waiting for you".into());
        }
        match p.kind {
            Kind::Shop => self.decide("autoShop", &json!({}), p, "ai"),
            Kind::Next => self.decide("next", &json!({}), p, "ai"),
            Kind::Swap => {
                let rs = self.rs.as_ref().unwrap();
                let g = self.r.swap_ai(rs, p.seat, p.n);
                self.apply_swap(p, Bits(g).collect(), "ai")
            }
            Kind::Bid => {
                let rs = self.rs.as_mut().unwrap();
                let (b, vals) = self.r.bid_ai(rs, p.seat);
                self.apply_bid(p.seat, b, Some(vals), "ai");
                Ok(())
            }
            Kind::Play | Kind::Lead => {
                let rs = self.rs.as_mut().unwrap();
                let (m, vals) = self.r.move_ai(rs, p.seat);
                self.apply_move(p.seat, m, Some(vals), "ai")
            }
        }
    }

    // ----- Shop -----

    fn open_shop(&mut self) {
        self.phase = Phase::Shop;
        self.rs = None;
        self.last_trick = None;
        self.fresh = false;
        // The AI team shops first; card offers never overlap, so its offers are excluded from ours.
        let offered = self.r.shop(1, self.round, 0);
        self.drain_shop_log();
        self.shop = Some(HumanShop {
            sig: vec![],
            cards: vec![],
            rerolls: 0,
            exclude: offered,
        });
        self.roll_offers();
    }

    fn roll_offers(&mut self) {
        let round = self.round;
        let sh = self.shop.as_mut().unwrap();
        let mut orng = Rng::stream(self.r.seed, &[S_OFFER, 0, round as u64, sh.rerolls]);
        let sig = self.r.sigil_offers(0, &mut orng);
        let cards = self.r.card_offers(round, &mut orng, sh.exclude);
        sh.sig = sig.iter().map(|&d| Some(d)).collect();
        sh.cards = cards.iter().map(|&c| Some(c)).collect();
        let msg = format!(
            "Us shop {} offers (gold {}): sigils [{}]; cards [{}]",
            round,
            self.r.teams[0].gold,
            sig.iter()
                .map(|&d| format!("{} {}g", self.sigil_label(d), self.def(d).price()))
                .collect::<Vec<_>>()
                .join(", "),
            cards
                .iter()
                .map(|o| format!("{} {}g", offer_str(o.ident, o.slot, o.eng), o.price))
                .collect::<Vec<_>>()
                .join(", ")
        );
        self.log(
            "offers",
            msg,
            json!({
                "sigils": sig.iter().map(|&d| self.def(d).id.clone()).collect::<Vec<_>>(),
                "cards": cards.iter().map(|o| offer_str(o.ident, o.slot, o.eng)).collect::<Vec<_>>(),
            }),
        );
    }

    fn buy_sigil(&mut self, i: usize) -> Result<(), String> {
        let d = self
            .shop
            .as_ref()
            .and_then(|s| s.sig.get(i).copied().flatten())
            .ok_or("no sigil offer there")?;
        let price = self.def(d).price();
        let team = &self.r.teams[0];
        if team.gold < price {
            return Err("not enough gold".into());
        }
        if team.sigils.len() >= MAX_SIGILS {
            return Err("all 7 sigil slots are full; sell one first".into());
        }
        if team.sigils.iter().any(|o| o.def == d) {
            return Err("your team already owns that sigil".into());
        }
        self.r.teams[0].gold -= price;
        self.r.teams[0].sigils.push(owned(d, self.round, price));
        self.shop.as_mut().unwrap().sig[i] = None;
        self.step += 1;
        let msg = format!(
            "Us: buy sigil {} for {}g (gold now {})",
            self.sigil_label(d),
            price,
            self.r.teams[0].gold
        );
        self.log(
            "buy",
            msg,
            json!({"action": {"t": "buySigil", "i": i}, "sigil": self.def(d).id, "price": price}),
        );
        Ok(())
    }

    fn buy_card(&mut self, i: usize) -> Result<(), String> {
        let o = self
            .shop
            .as_ref()
            .and_then(|s| s.cards.get(i).copied().flatten())
            .ok_or("no card offer there")?;
        let team = &self.r.teams[0];
        if team.gold < o.price {
            return Err("not enough gold".into());
        }
        if team.cards.len() >= MAX_CARDS {
            return Err("you already own 8 cards; sell one first".into());
        }
        if self.r.taken & bit(o.slot) != 0 {
            return Err("that card is already owned".into());
        }
        self.r.buy_card(0, o, self.round, false);
        self.shop.as_mut().unwrap().cards[i] = None;
        self.step += 1;
        let msg = format!(
            "Us: buy card {} for {}g (gold now {})",
            offer_str(o.ident, o.slot, o.eng),
            o.price,
            self.r.teams[0].gold
        );
        self.log(
            "buy",
            msg,
            json!({"action": {"t": "buyCard", "i": i}, "card": offer_str(o.ident, o.slot, o.eng), "price": o.price}),
        );
        Ok(())
    }

    fn sell_sigil(&mut self, k: usize) -> Result<(), String> {
        let o = self.r.teams[0].sigils.get(k).ok_or("no sigil there")?;
        let (d, v) = (o.def, sell_value(o.price));
        self.r.sell_sigil(0, k, self.round);
        self.step += 1;
        let msg = format!(
            "Us: sell sigil {} for {}g (gold now {})",
            self.sigil_label(d),
            v,
            self.r.teams[0].gold
        );
        self.log(
            "sell",
            msg,
            json!({"action": {"t": "sellSigil", "k": k}, "sigil": self.def(d).id}),
        );
        Ok(())
    }

    fn sell_card(&mut self, k: usize) -> Result<(), String> {
        let c = *self.r.teams[0].cards.get(k).ok_or("no card there")?;
        self.r.sell_card(0, k);
        self.step += 1;
        let msg = format!(
            "Us: sell card {} for {}g (gold now {})",
            offer_str(c.ident, c.slot, c.eng),
            sell_value(c.price),
            self.r.teams[0].gold
        );
        self.log("sell", msg, json!({"action": {"t": "sellCard", "k": k}}));
        Ok(())
    }

    fn reroll_cost(&self) -> i32 {
        50 + 10 * self.shop.as_ref().map_or(0, |s| s.rerolls) as i32
    }

    fn reroll(&mut self) -> Result<(), String> {
        let cost = self.reroll_cost();
        if self.r.teams[0].gold < cost {
            return Err("not enough gold".into());
        }
        self.r.teams[0].gold -= cost;
        self.r.recs[0].rerolls += 1;
        self.shop.as_mut().unwrap().rerolls += 1;
        self.step += 1;
        self.log(
            "reroll",
            format!("Us: reroll for {cost}g"),
            json!({"action": {"t": "reroll"}}),
        );
        self.roll_offers();
        Ok(())
    }

    // ----- Sandbox -----

    fn sandbox(&mut self, a: &Value) -> Result<(), String> {
        let t = a["t"].as_str().unwrap_or("");
        let team = a["team"].as_u64().unwrap_or(0) as usize;
        if team > 1 {
            return Err("team is 0 (us) or 1 (them)".into());
        }
        let edits_build = matches!(t, "give" | "take" | "giveCard");
        if edits_build && self.phase != Phase::Shop {
            return Err("sigils and cards can only be granted or removed during a shop".into());
        }
        let msg = match t {
            "give" => {
                let id = a["sigil"].as_str().ok_or("needs sigil")?;
                let d = *self.r.cfg.pool.by_id.get(id).ok_or("unknown sigil")?;
                let tm = &self.r.teams[team];
                if tm.sigils.len() >= MAX_SIGILS {
                    return Err("all 7 sigil slots are full".into());
                }
                if tm.sigils.iter().any(|o| o.def == d) {
                    return Err("that team already owns it".into());
                }
                self.r.teams[team]
                    .sigils
                    .push(owned(d, self.round, self.def(d).price()));
                format!(
                    "Sandbox: grant {} to {}",
                    self.sigil_label(d),
                    team_name(team)
                )
            }
            "take" => {
                let k = a["k"].as_u64().ok_or("needs k")? as usize;
                if k >= self.r.teams[team].sigils.len() {
                    return Err("no sigil there".into());
                }
                let o = self.r.teams[team].sigils.remove(k);
                format!(
                    "Sandbox: remove {} from {}",
                    self.sigil_label(o.def),
                    team_name(team)
                )
            }
            "giveCard" => {
                let suit = a["suit"].as_u64().ok_or("needs suit")? as u8;
                let rank = a["rank"].as_u64().ok_or("needs rank")? as u8;
                let eng = a["eng"].as_u64().unwrap_or(0) as u8;
                if suit > 3 || !(2..=14).contains(&rank) || eng > ENG_SYNTH {
                    return Err("bad card".into());
                }
                if self.r.teams[team].cards.len() >= MAX_CARDS {
                    return Err("that team already owns 8 cards".into());
                }
                let ident = card(suit, rank);
                let slot = if eng == ENG_SYNTH {
                    let free: Vec<u8> = (2..=9)
                        .map(|r| card(suit, r))
                        .filter(|&c| self.r.taken & bit(c) == 0)
                        .collect();
                    *free
                        .first()
                        .ok_or("no free low card of that suit to replace")?
                } else {
                    if self.r.taken & bit(ident) != 0 {
                        return Err("that card is already owned".into());
                    }
                    ident
                };
                self.r.taken |= bit(slot);
                self.r.teams[team].cards.push(OwnedCard {
                    ident,
                    slot,
                    eng,
                    price: card_price(suit, rank),
                    grant: true,
                    shop: self.round,
                });
                format!(
                    "Sandbox: grant card {} to {}",
                    offer_str(ident, slot, eng),
                    team_name(team)
                )
            }
            "gold" => {
                let amt = a["amount"].as_i64().ok_or("needs amount")? as i32;
                self.r.teams[team].gold += amt;
                format!("Sandbox: {amt:+}g to {}", team_name(team))
            }
            "tier" => {
                let tier = a["tier"].as_u64().ok_or("needs tier")? as u8;
                self.tier = tier.min(2);
                self.r.tier_override = Some(Tier::get(self.tier));
                format!("Sandbox: AI tier set to {}", self.tier)
            }
            _ => return Err("unknown sandbox action".into()),
        };
        self.step += 1;
        self.log("sandbox", msg, json!({"action": a}));
        Ok(())
    }

    // ----- Round -----

    fn begin_round(&mut self) {
        self.shop = None;
        self.r.sync_records();
        let rs = self.r.deal(self.round);
        let lines: Vec<String> = (0..4)
            .map(|s| {
                format!(
                    "{}: {}",
                    SEAT_NAMES[s],
                    hand_s(&rs.base_id, rs.play.hands[s])
                )
            })
            .collect();
        let owned: Vec<String> = (0..2)
            .flat_map(|t| {
                self.r.teams[t]
                    .cards
                    .iter()
                    .map(move |c| (t, *c))
                    .collect::<Vec<_>>()
            })
            .map(|(t, c)| {
                format!(
                    "{} owns {}",
                    team_name(t),
                    offer_str(c.ident, c.slot, c.eng)
                )
            })
            .collect();
        let dealer = rs.play.dealer;
        self.rs = Some(rs);
        self.op = 0;
        self.swap = None;
        self.nbids = 0;
        self.notes.clear();
        self.last_trick = None;
        self.fresh = false;
        self.phase = Phase::Opening;
        self.log(
            "deal",
            format!(
                "Round {} deal (dealer {}): {}",
                self.round,
                SEAT_NAMES[dealer as usize],
                lines.join(" | ")
            ),
            json!({"dealer": dealer, "hands": lines, "owned": owned}),
        );
        self.settle();
    }

    /// Runs every automatic step until a decision is pending.
    fn settle(&mut self) {
        loop {
            match self.phase {
                Phase::Opening => {
                    if self.swap.is_some() {
                        return;
                    }
                    let rs = self.rs.as_ref().unwrap();
                    if self.op >= rs.opening.len() {
                        self.phase = Phase::Bidding;
                        self.nbids = 0;
                        continue;
                    }
                    let (kt, k, rule) = rs.opening[self.op];
                    let seats = self.r.opening_seats(kt, k);
                    if let Rule::Swap(n) = rule {
                        self.swap = Some(SwapState {
                            kt,
                            k,
                            n,
                            seats: [seats.0, seats.1],
                            give: [None, None],
                        });
                        return;
                    }
                    let before = rs.play.id;
                    let rs = self.rs.as_mut().unwrap();
                    self.r.apply_opening(rs, kt, k, rule);
                    let after = rs.play.id;
                    let hands = rs.play.hands;
                    self.op += 1;
                    let d = self.r.teams[kt].sigils[k].def;
                    let mut changes = vec![];
                    for seat in [seats.0, seats.1] {
                        let ch: Vec<String> = Bits(hands[seat])
                            .filter(|&c| {
                                before.suit[c as usize] != after.suit[c as usize]
                                    || before.rank[c as usize] != after.rank[c as usize]
                            })
                            .map(|c| format!("{}→{}", card_s(&before, c), card_s(&after, c)))
                            .collect();
                        if seat == HUMAN as usize && !ch.is_empty() {
                            self.notes.push(json!({"sigil": self.def(d).id, "text": format!("Your cards changed: {}", ch.join(", "))}));
                        }
                        changes.push(format!(
                            "{}: {}",
                            SEAT_NAMES[seat],
                            if ch.is_empty() {
                                "no change".into()
                            } else {
                                ch.join(", ")
                            }
                        ));
                    }
                    let msg = format!(
                        "Opening: {}'s {} — {}",
                        team_name(kt),
                        self.sigil_label(d),
                        changes.join("; ")
                    );
                    self.log(
                        "opening",
                        msg,
                        json!({"team": kt, "sigil": self.def(d).id, "changes": changes}),
                    );
                }
                Phase::Bidding => {
                    if self.nbids < 4 {
                        return;
                    }
                    let rs = self.rs.as_mut().unwrap();
                    let before = rs.play.acc;
                    self.r.finish_bidding(rs);
                    let after = rs.play.acc;
                    let held: Vec<String> = (0..2)
                        .map(|t| format!("{} contract {}", team_name(t), rs.play.contract(t)))
                        .collect();
                    self.phase = Phase::Playing;
                    self.log(
                        "bidsDone",
                        format!("Bidding done: {}", held.join(", ")),
                        json!({}),
                    );
                    self.log_fires(&before, &after, "after bidding");
                }
                Phase::Playing => {
                    if !self.rs.as_ref().unwrap().play.done() {
                        return;
                    }
                    self.end_round();
                }
                _ => return,
            }
        }
    }

    fn apply_swap(&mut self, p: Pending, cards: Vec<u8>, by: &str) -> Result<(), String> {
        let rs = self.rs.as_ref().unwrap();
        let sw = self.swap.as_ref().unwrap();
        let hand = rs.play.hands[p.seat as usize];
        let mut g: Mask = 0;
        for &c in &cards {
            if c >= 52 || hand & bit(c) == 0 {
                return Err("you can only give cards in your hand".into());
            }
            g |= bit(c);
        }
        if g.count_ones() > sw.n as u32 {
            return Err(format!("give at most {} cards", sw.n));
        }
        let i = sw.seats.iter().position(|&s| s == p.seat as usize).unwrap();
        let id = rs.play.id;
        self.swap.as_mut().unwrap().give[i] = Some(g);
        self.step += 1;
        let list: Vec<u64> = Bits(g).map(|c| c as u64).collect();
        self.log(
            "swapChoice",
            format!(
                "{} chooses to give {}",
                SEAT_NAMES[p.seat as usize],
                hand_s(&id, g)
            ),
            json!({"action": {"t": "swap", "seat": p.seat, "cards": list}, "by": by}),
        );
        let sw = self.swap.as_ref().unwrap();
        if let [Some(ga), Some(gb)] = sw.give {
            let (kt, k, [a, b]) = (sw.kt, sw.k, sw.seats);
            let n = ga.count_ones().min(gb.count_ones());
            let rs = self.rs.as_mut().unwrap();
            self.r.apply_swap(rs, a, b, ga, gb);
            self.r.teams[self.r.tidx(kt)].sigils[k].revealed = true;
            // Equal counts change hands; the first `n` of each choice by card order.
            let (ta, tb) = (first_n(ga, n), first_n(gb, n));
            let d = self.r.teams[kt].sigils[k].def;
            let msg = format!(
                "Opening swap ({}): {} gives {}; {} gives {}",
                self.sigil_label(d),
                SEAT_NAMES[a],
                hand_s(&id, ta),
                SEAT_NAMES[b],
                hand_s(&id, tb)
            );
            if a == HUMAN as usize || b == HUMAN as usize {
                let (mine, theirs) = if a == HUMAN as usize {
                    (ta, tb)
                } else {
                    (tb, ta)
                };
                self.notes.push(json!({"sigil": self.def(d).id, "text": format!("You gave {} and received {}", hand_s(&id, mine), hand_s(&id, theirs))}));
            }
            self.log("swap", msg, json!({"sigil": self.def(d).id}));
            self.swap = None;
            self.op += 1;
            self.settle();
        }
        Ok(())
    }

    fn apply_bid(&mut self, seat: u8, b: i8, vals: Option<Vec<(i8, f64)>>, by: &str) {
        let rs = self.rs.as_mut().unwrap();
        rs.play.bids[seat as usize] = b;
        let hand = hand_s(&rs.play.id, rs.play.hands[seat as usize]);
        self.nbids += 1;
        self.step += 1;
        let cands = vals.as_ref().map(|v| {
            v.iter()
                .map(|(b, x)| format!("{}={:.3}", bid_s(*b), x))
                .collect::<Vec<_>>()
                .join(", ")
        });
        let msg = format!(
            "{} bids {}{} [hand: {}]",
            SEAT_NAMES[seat as usize],
            bid_s(b),
            cands
                .map(|c| format!(" (win-prob value by bid: {c})"))
                .unwrap_or_default(),
            hand
        );
        let cand_json: Option<Vec<Value>> = vals.map(|v| {
            v.iter()
                .map(|(b, x)| json!({"bid": b, "value": round3(*x)}))
                .collect()
        });
        self.log(
            "bid",
            msg,
            json!({"action": {"t": "bid", "seat": seat, "bid": b}, "by": by, "candidates": cand_json}),
        );
        self.settle();
    }

    fn apply_move(
        &mut self,
        seat: u8,
        m: u8,
        vals: Option<Vec<(u8, f64)>>,
        by: &str,
    ) -> Result<(), String> {
        let rs = self.rs.as_mut().unwrap();
        let legal = rs.play.legal();
        if m >= 64 || legal & bit(m) == 0 {
            return Err("that card can't be played now".into());
        }
        let id = rs.play.id;
        let before = rs.play.acc;
        let revealed: Vec<Vec<bool>> = (0..2)
            .map(|t| self.r.teams[t].sigils.iter().map(|o| o.revealed).collect())
            .collect();
        let hand = rs.play.hands[seat as usize];
        let trick_no = rs.play.ntricks + 1;
        let led = rs.play.tlen == 0;
        let res = self.r.apply_move(rs, seat, m);
        let after = rs.play.acc;
        let play = rs.play;
        self.step += 1;
        self.fresh = false;
        let mv = |c: u8| {
            if c == MOVE_KEEP {
                "keep the lead".to_string()
            } else if c == MOVE_PASS {
                "pass the lead to partner".to_string()
            } else {
                card_s(&id, c)
            }
        };
        let cands = vals.as_ref().filter(|v| !v.is_empty()).map(|v| {
            let mut v = v.clone();
            v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            v.iter()
                .map(|(c, x)| {
                    if *x < -1e300 {
                        format!("{}=unexplored", mv(*c))
                    } else {
                        format!("{}={:.3}", mv(*c), x)
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        });
        let (ty, action) = if m >= 52 {
            (
                "leadChoice",
                json!({"t": "lead", "seat": seat, "pass": m == MOVE_PASS}),
            )
        } else {
            ("play", json!({"t": "play", "seat": seat, "card": m}))
        };
        let msg = if m >= 52 {
            format!(
                "{} chooses to {}{}",
                SEAT_NAMES[seat as usize],
                mv(m),
                cands.map(|c| format!(" (values: {c})")).unwrap_or_default()
            )
        } else {
            format!(
                "Trick {}: {} {} {} [hand: {}; legal: {}]{}",
                trick_no,
                SEAT_NAMES[seat as usize],
                if led { "leads" } else { "plays" },
                card_s(&id, m),
                hand_s(&id, hand),
                hand_s(&id, legal),
                cands
                    .map(|c| format!(" (search values: {c})"))
                    .unwrap_or_default()
            )
        };
        let cand_json: Option<Vec<Value>> = vals.map(|v| {
            v.iter()
                .map(|(c, x)| json!({"move": mv(*c), "value": if *x < -1e300 { Value::Null } else { json!(round3(*x)) }}))
                .collect()
        });
        self.log(
            ty,
            msg,
            json!({"action": action, "by": by, "candidates": cand_json}),
        );
        if let Some(tr) = res {
            let w = tr.winner;
            let counts = play.bids[w as usize] != 0;
            let plays: Vec<Value> = (0..4)
                .map(|i| json!({"seat": tr.seats[i], "card": self.card_json(&id, tr.cards[i], &play)}))
                .collect();
            let wi = (0..4).find(|&i| tr.seats[i] == w).unwrap();
            self.last_trick =
                Some(json!({"n": play.ntricks, "plays": plays, "winner": w, "counts": counts}));
            self.fresh = true;
            let desc: Vec<String> = (0..4)
                .map(|i| {
                    format!(
                        "{} {}",
                        SEAT_NAMES[tr.seats[i] as usize],
                        card_s(&id, tr.cards[i])
                    )
                })
                .collect();
            let msg = format!(
                "Trick {} won by {} with {}{} [{}]; tricks won: {}",
                play.ntricks,
                SEAT_NAMES[w as usize],
                card_s(&id, tr.cards[wi]),
                if counts {
                    ""
                } else {
                    " (a nil bidder's trick)"
                },
                desc.join(", "),
                (0..4)
                    .map(|s| format!("{} {}", SEAT_NAMES[s], play.won[s]))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            self.log("trick", msg, json!({"winner": w}));
        }
        self.log_fires(&before, &after, &format!("trick {trick_no}"));
        for t in 0..2 {
            for (k, o) in self.r.teams[t].sigils.clone().iter().enumerate() {
                if o.revealed && !revealed[t].get(k).copied().unwrap_or(true) {
                    let msg = format!(
                        "{}'s sigil {} is revealed",
                        team_name(t),
                        self.sigil_label(o.def)
                    );
                    self.log(
                        "reveal",
                        msg,
                        json!({"team": t, "sigil": self.def(o.def).id}),
                    );
                }
            }
        }
        self.settle();
        Ok(())
    }

    /// Logs every sigil and engraving whose accumulators changed between two snapshots.
    fn log_fires(&mut self, before: &[TeamAcc; 2], after: &[TeamAcc; 2], when: &str) {
        for t in 0..2 {
            let (b, a) = (&before[t], &after[t]);
            for k in 0..self.r.teams[t].sigils.len().min(MAXP) {
                let df = a.fires[k].saturating_sub(b.fires[k]);
                let changed = df > 0
                    || a.cp[k] != b.cp[k]
                    || a.add[k] != b.add[k]
                    || a.x[k] != b.x[k]
                    || a.nilp[k] != b.nilp[k];
                if !changed {
                    continue;
                }
                let d = self.r.teams[t].sigils[k].def;
                let mut parts = vec![];
                if a.cp[k] != b.cp[k] {
                    parts.push(format!("{:+} contract points", a.cp[k] - b.cp[k]));
                }
                if a.add[k] != b.add[k] {
                    parts.push(format!("{:+} contract multiplier", a.add[k] - b.add[k]));
                }
                if a.x[k] != b.x[k] {
                    parts.push(format!("×{} contract multiplier", num(a.x[k] / b.x[k])));
                }
                if a.nilp[k] != b.nilp[k] {
                    parts.push(format!("{:+} nil points", a.nilp[k] - b.nilp[k]));
                }
                if a.grow[k] != b.grow[k] {
                    parts.push(format!("grows {} step(s)", a.grow[k] - b.grow[k]));
                }
                let msg = format!(
                    "{} sigil {} fires{} ({}): {}",
                    team_name(t),
                    self.sigil_label(d),
                    if df > 1 {
                        format!(" ×{df}")
                    } else {
                        String::new()
                    },
                    when,
                    if parts.is_empty() {
                        "rule used".into()
                    } else {
                        parts.join(", ")
                    }
                );
                self.log(
                    "fire",
                    msg,
                    json!({"team": t, "slot": k, "sigil": self.def(d).id, "times": df}),
                );
            }
            if a.eng_cp != b.eng_cp || a.eng_add != b.eng_add {
                let msg = format!(
                    "{} engraving fires ({}): {:+} contract points, {:+} contract multiplier",
                    team_name(t),
                    when,
                    a.eng_cp - b.eng_cp,
                    a.eng_add - b.eng_add
                );
                self.log("engraving", msg, json!({"team": t}));
            }
        }
    }

    fn end_round(&mut self) {
        let rs = self.rs.take().unwrap();
        let before = rs.play.acc;
        let rev0: Vec<bool> = self.r.teams[1].sigils.iter().map(|o| o.revealed).collect();
        let (sc, inc) = self.r.finish_round(&rs);
        let after = [sc[0].acc, sc[1].acc];
        self.log_fires(&before, &after, "at scoring");
        let mut teams = vec![];
        for t in 0..2 {
            let s = &sc[t];
            let mut ledger = vec![];
            for (k, o) in self.r.teams[t].sigils.iter().enumerate().take(MAXP) {
                let a = &s.acc;
                if a.fires[k] > 0
                    || a.cp[k] != 0.0
                    || a.add[k] != 0.0
                    || a.x[k] != 1.0
                    || a.nilp[k] != 0.0
                {
                    ledger.push(json!({
                        "k": k, "id": self.def(o.def).id, "fires": a.fires[k],
                        "cp": a.cp[k], "add": a.add[k], "x": a.x[k], "nilp": a.nilp[k],
                    }));
                }
            }
            let i = inc[t];
            let cp_raw: f64 = (0..self.r.teams[t].sigils.len().min(MAXP))
                .map(|k| s.acc.cp[k])
                .sum::<f64>()
                + s.acc.eng_cp;
            let mult = (BASE_MULT + s.add) * s.x;
            let formula = if s.contract == 0 && s.nil_bids == 0 {
                "no contract".to_string()
            } else {
                format!(
                    "({} base{} + {} nil) × ({} + {}) × {} = {}",
                    num(if s.made { 10.0 } else { -10.0 } * s.contract as f64),
                    if s.made {
                        format!(" + {} contract points", num(s.cp))
                    } else if cp_raw != 0.0 {
                        format!(" (set: {} contract points lost)", num(cp_raw))
                    } else {
                        String::new()
                    },
                    num(s.nil_score),
                    num(BASE_MULT),
                    num(s.add),
                    num(s.x),
                    num(s.score)
                )
            };
            let bids = [rs.play.bids[t], rs.play.bids[t + 2]];
            let msg = format!(
                "{} score: bids {}+{} = contract {}, won {} ({}); {}; run total {}; income {}g (interest {}, base {}, contract {}, nil {}) → gold {}",
                team_name(t),
                bid_s(bids[0]),
                bid_s(bids[1]),
                s.contract,
                s.tricks,
                if s.contract == 0 { "no contract" } else if s.made { if s.exact { "made exactly" } else { "made" } } else { "set" },
                formula,
                num(self.r.teams[t].score),
                i.interest + i.base + i.contract + i.nil,
                i.interest,
                i.base,
                i.contract,
                i.nil,
                self.r.teams[t].gold
            );
            let entry = json!({
                "bids": bids, "contract": s.contract, "tricks": s.tricks, "made": s.made,
                "exact": s.exact, "set": s.set, "nilBids": s.nil_bids, "nilMade": s.nil_made,
                "base": 10.0 * s.contract as f64 * if s.made { 1.0 } else { -1.0 },
                "cp": s.cp, "cpLost": if s.made { 0.0 } else { cp_raw },
                "add": s.add, "mult": BASE_MULT + s.add, "x": s.x, "multTotal": mult,
                "nilScore": s.nil_score, "score": s.score, "total": self.r.teams[t].score,
                "engCp": s.acc.eng_cp, "engAdd": s.acc.eng_add, "ledger": ledger,
                "income": {"interest": i.interest, "base": i.base, "contract": i.contract, "nil": i.nil,
                           "total": i.interest + i.base + i.contract + i.nil},
                "gold": self.r.teams[t].gold, "formula": formula,
            });
            self.log("score", msg, json!({"team": t, "result": entry.clone()}));
            teams.push(entry);
        }
        for (k, o) in self.r.teams[1].sigils.clone().iter().enumerate() {
            if o.revealed && !rev0.get(k).copied().unwrap_or(true) {
                let msg = format!("Them's sigil {} is revealed", self.sigil_label(o.def));
                self.log(
                    "reveal",
                    msg,
                    json!({"team": 1, "sigil": self.def(o.def).id}),
                );
            }
        }
        self.result = Some(json!({"round": self.round, "teams": teams}));
        self.phase = Phase::RoundOver;
    }

    fn next_round(&mut self, by: &str) {
        if self.round >= ROUNDS {
            self.phase = Phase::GameOver;
            let (a, b) = (self.r.teams[0].score, self.r.teams[1].score);
            let res = if a > b {
                "you win"
            } else if a < b {
                "you lose"
            } else {
                "draw"
            };
            self.log(
                "gameOver",
                format!("Game over: Us {} – Them {}: {res}", num(a), num(b)),
                json!({"action": {"t": "next"}, "by": by, "scores": [a, b]}),
            );
            return;
        }
        self.round += 1;
        self.log(
            "nextRound",
            format!("Round {} begins", self.round),
            json!({"action": {"t": "next"}, "by": by}),
        );
        self.open_shop();
    }

    // ----- View -----

    fn owned_card(&self, slot: u8) -> Option<(usize, OwnedCard)> {
        (0..2).find_map(|t| {
            self.r.teams[t]
                .cards
                .iter()
                .find(|c| c.slot == slot)
                .map(|c| (t, *c))
        })
    }

    fn card_json(&self, id: &Identity, c: u8, play: &Play) -> Value {
        let mut v = json!({"id": c, "s": id.suit[c as usize], "r": id.rank[c as usize], "eng": play.eng[c as usize]});
        if let Some((t, oc)) = self.owned_card(c) {
            v["own"] = json!(t);
            if oc.eng == ENG_SYNTH {
                v["syn"] = json!(card_str(nominal_suit(c), nominal_rank(c)));
            }
        }
        if let Some(rs) = &self.rs {
            let b = &rs.base_id;
            if b.suit[c as usize] != id.suit[c as usize]
                || b.rank[c as usize] != id.rank[c as usize]
            {
                v["was"] = json!(card_s(b, c));
            }
        }
        v
    }

    fn owned_json(&self, c: &OwnedCard) -> Value {
        let mut v = json!({
            "id": c.slot, "s": nominal_suit(c.ident), "r": nominal_rank(c.ident),
            "eng": if c.eng == ENG_SYNTH { ENG_NONE } else { c.eng },
            "price": c.price, "sell": sell_value(c.price),
        });
        if c.eng == ENG_SYNTH {
            v["syn"] = json!(card_str(nominal_suit(c.slot), nominal_rank(c.slot)));
        }
        v
    }

    /// The table from your seat. `all` shows every hand and hidden sigil (sandbox).
    pub fn view(&self, all: bool) -> Value {
        let pending = self.pending().map(|p| {
            let mut v = json!({
                "kind": format!("{:?}", p.kind).to_lowercase(),
                "seat": p.seat, "n": p.n, "ai": self.is_ai(&p),
            });
            if let Some(sw) = &self.swap {
                let d = self.r.teams[self.r.tidx(sw.kt)].sigils[sw.k].def;
                v["sigil"] = json!(self.def(d).id);
            }
            v
        });
        let round_fires = |t: usize, k: usize| {
            self.rs
                .as_ref()
                .map_or(0, |rs| rs.play.acc[t].fires.get(k).copied().unwrap_or(0))
        };
        let teams: Vec<Value> = (0..2)
            .map(|t| {
                let tm = &self.r.teams[t];
                let show = |o: &OwnedSigil| t == 0 || all || o.revealed;
                let sigils: Vec<Value> = tm
                    .sigils
                    .iter()
                    .enumerate()
                    .filter(|(_, o)| show(o))
                    .map(|(k, o)| {
                        json!({
                            "k": k, "id": self.def(o.def).id, "grow": o.grow, "fires": o.fires,
                            "rf": round_fires(t, k), "sell": sell_value(o.price),
                            "revealed": o.revealed,
                        })
                    })
                    .collect();
                let hidden = tm.sigils.iter().filter(|o| !show(o)).count();
                let cards: Vec<Value> = if t == 0 || all {
                    tm.cards.iter().map(|c| self.owned_json(c)).collect()
                } else {
                    vec![]
                };
                json!({
                    "score": tm.score, "gold": tm.gold, "sigils": sigils, "hidden": hidden,
                    "cards": cards, "cardCount": tm.cards.len(), "cardSeat": t as u8 + 2 * tm.owner,
                })
            })
            .collect();
        let mut v = json!({
            "step": self.step, "phase": self.phase.name(), "round": self.round, "rounds": ROUNDS,
            "auto": self.auto, "tier": self.tier, "teams": teams, "pending": pending,
            "notes": self.notes, "result": self.result, "lastTrick": self.last_trick,
            "fresh": self.fresh,
        });
        if let Some(rs) = &self.rs {
            let play = &rs.play;
            let id = play.id;
            let hands: Vec<Value> = (0..4)
                .map(|s| {
                    let m = play.hands[s];
                    let visible = if s == HUMAN as usize || all {
                        m
                    } else if s == 2 {
                        // Your partner's owned cards, which you know from shopping together.
                        rs.known[2] & m
                    } else {
                        0
                    };
                    json!({
                        "count": m.count_ones(),
                        "cards": Bits(visible).map(|c| self.card_json(&id, c, play)).collect::<Vec<_>>(),
                    })
                })
                .collect();
            let trick: Vec<Value> = (0..play.tlen as usize)
                .map(|i| json!({"seat": play.tseat[i], "card": self.card_json(&id, play.trick[i], play)}))
                .collect();
            let legal: Vec<u8> = match self.pending() {
                Some(p) if p.seat == HUMAN && matches!(p.kind, Kind::Play) => {
                    Bits(play.legal()).collect()
                }
                _ => vec![],
            };
            // Your team's running tally, as the round would score if the contract is made.
            let acc = &play.acc[0];
            let n = self.r.teams[0].sigils.len().min(MAXP);
            let cp: f64 = acc.cp[..n].iter().sum::<f64>() + acc.eng_cp;
            let add: f64 = acc.add[..n].iter().sum::<f64>() + acc.eng_add;
            let x: f64 = acc.x[..n].iter().product();
            let nilp: f64 = acc.nilp[..n].iter().sum();
            v["dealer"] = json!(play.dealer);
            v["hands"] = json!(hands);
            v["bids"] = json!(play.bids);
            v["won"] = json!(play.won);
            v["trick"] = json!(trick);
            v["turn"] = json!(play.turn);
            v["broken"] = json!(play.broken);
            v["tricks"] = json!(play.ntricks);
            v["legal"] = json!(legal);
            v["contracts"] = json!([play.contract(0), play.contract(1)]);
            v["contractTricks"] = json!([play.contract_tricks(0), play.contract_tricks(1)]);
            v["live"] =
                json!({"cp": cp, "add": add, "mult": BASE_MULT + add, "x": x, "nilp": nilp});
        }
        if let Some(sh) = &self.shop {
            let sig: Vec<Value> = sh
                .sig
                .iter()
                .enumerate()
                .map(|(i, d)| match d {
                    Some(d) => {
                        json!({"i": i, "id": self.def(*d).id, "price": self.def(*d).price()})
                    }
                    None => Value::Null,
                })
                .collect();
            let cards: Vec<Value> = sh
                .cards
                .iter()
                .enumerate()
                .map(|(i, o)| match o {
                    Some(o) => {
                        let mut c = json!({"i": i, "id": o.slot, "s": nominal_suit(o.ident), "r": nominal_rank(o.ident),
                                           "eng": if o.eng == ENG_SYNTH { ENG_NONE } else { o.eng }, "price": o.price});
                        if o.eng == ENG_SYNTH {
                            c["syn"] = json!(card_str(nominal_suit(o.slot), nominal_rank(o.slot)));
                        }
                        c
                    }
                    None => Value::Null,
                })
                .collect();
            v["shop"] = json!({"sigils": sig, "cards": cards, "reroll": self.reroll_cost()});
        }
        if self.phase == Phase::GameOver {
            let (a, b) = (self.r.teams[0].score, self.r.teams[1].score);
            v["winner"] = if a > b {
                json!(0)
            } else if a < b {
                json!(1)
            } else {
                json!("draw")
            };
        }
        v
    }
}

fn owned(d: usize, shop: u8, price: i32) -> OwnedSigil {
    OwnedSigil {
        def: d,
        amount: None,
        ratio: 0.0,
        shop,
        grant: false,
        price,
        grow: 0.0,
        revealed: false,
        sold: None,
        fires: 0,
    }
}

fn first_n(m: Mask, n: u32) -> Mask {
    let mut out = 0;
    for c in Bits(m).take(n as usize) {
        out |= bit(c);
    }
    out
}
