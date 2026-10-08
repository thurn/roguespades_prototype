//! Gameplay levers read from `data/rules.json`: scoring, the set penalty, the economy, and the
//! shop. The file is compiled in as the default; experiments and the game client may install a
//! different one before play starts.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicPtr, Ordering};

/// Bags: every `BAG_LIMIT` a team accumulates cost `BAG_PENALTY` points, flat (fixed rules).
pub const BAG_LIMIT: u8 = 10;
pub const BAG_PENALTY: f64 = 1000.0;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", default)]
pub struct GameRules {
    pub rounds: u8,
    /// Starting contract multiplier.
    pub base_mult: f64,
    /// Scale on every +contract multiplier.
    pub mult_reward_scale: f64,
    /// Global scale on every sigil payoff amount (×multipliers scale their excess over 1).
    pub amount_scale: f64,
    /// Points per contract trick when made (fixed).
    pub trick_value: f64,
    /// The base nil score (fixed).
    pub nil_value: f64,
    /// Flat set: a set scores −10 × base 10 × B, untouched by sigils. Otherwise the set is
    /// symmetric: −(10 × B + contract points), times every fired multiplier.
    pub flat_set: bool,
    pub start_gold: i32,
    pub base_income: i32,
    /// Gold per contract trick when made.
    pub contract_gold: i32,
    pub nil_gold: i32,
    pub sigil_offers: u8,
    pub card_offers: u8,
    /// Sigil offer draws: "replacement" (never one the team owns), "team" (never one already
    /// offered to the team this run), or "shared" (never one either team owns).
    pub offer_draws: String,
    /// Rarity odds: common, uncommon, rare, legendary.
    pub rarity_odds: [f64; 4],
    pub prices: [i32; 4],
    /// Scale on every card price (rounded to a multiple of 5).
    pub card_price_scale: f64,
    /// Sell value as a share of price, rounded down to a multiple of 5.
    pub sell_share: f64,
    pub reroll_cost: i32,
    pub reroll_step: i32,
    pub slots: usize,
    pub max_cards: usize,
    /// Purchases per shop (0 = unlimited).
    pub purchases: u8,
    /// The par curve (rounds 1–8), the target of the global amount scale.
    pub par: Vec<f64>,
    /// The round's deck composition (every card identity after Opening changes, but not who
    /// holds it) is public, so searches sample hidden hands from it.
    pub public_deck: bool,
}

impl Default for GameRules {
    fn default() -> Self {
        GameRules {
            rounds: 8,
            base_mult: 10.0,
            mult_reward_scale: 1.0,
            amount_scale: 1.0,
            trick_value: 10.0,
            nil_value: 100.0,
            flat_set: false,
            start_gold: 150,
            base_income: 100,
            contract_gold: 10,
            nil_gold: 100,
            sigil_offers: 3,
            card_offers: 3,
            offer_draws: "replacement".into(),
            rarity_odds: [0.69, 0.25, 0.05, 0.01],
            prices: [50, 75, 100, 150],
            card_price_scale: 1.0,
            sell_share: 0.5,
            reroll_cost: 50,
            reroll_step: 10,
            slots: 7,
            max_cards: 8,
            purchases: 0,
            par: vec![600.0, 850.0, 1150.0, 1600.0, 2250.0, 3100.0, 4300.0, 6000.0],
            public_deck: true,
        }
    }
}

const DEFAULT_JSON: &str = include_str!("../../data/rules.json");

static RULES: AtomicPtr<GameRules> = AtomicPtr::new(std::ptr::null_mut());

/// The active rules: the installed file, else `data/rules.json` as compiled in.
#[inline]
pub fn rules() -> &'static GameRules {
    let p = RULES.load(Ordering::Acquire);
    if !p.is_null() {
        return unsafe { &*p };
    }
    install(parse(DEFAULT_JSON).expect("data/rules.json"));
    unsafe { &*RULES.load(Ordering::Acquire) }
}

pub fn parse(s: &str) -> Result<GameRules, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}

/// Installs rules for the rest of the process (the old ones are leaked; installs are rare).
pub fn install(r: GameRules) {
    RULES.store(Box::into_raw(Box::new(r)), Ordering::Release);
}

pub fn load(path: &str) -> GameRules {
    let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read rules {path}: {e}"));
    parse(&s).unwrap_or_else(|e| panic!("parse rules {path}: {e}"))
}

impl GameRules {
    pub fn sell_value(&self, price: i32) -> i32 {
        ((price as f64 * self.sell_share) as i32) / 5 * 5
    }

    pub fn reroll(&self, n: u64) -> i32 {
        self.reroll_cost + self.reroll_step * n as i32
    }

    /// The base, effective +mult sum, and ×mult product of a round score. `cp` is the contract
    /// points earned (lost when set); `add` is the raw +mult sum before `mult_reward_scale`.
    pub fn parts(&self, contract: u8, tricks: u8, cp: f64, add: f64, x: f64) -> (f64, f64, f64) {
        let b = contract as f64;
        let add = add * self.mult_reward_scale;
        if contract == 0 {
            return (0.0, add, x);
        }
        if tricks >= contract {
            return (self.trick_value * b + cp, add, x);
        }
        (-(self.trick_value * b + cp), add, x)
    }

    /// A team's round score: (base + nil score) × (base mult + +mults) × ×mults. A flat set scores
    /// its base at the plain ×10, untouched by sigils; its nil score is still multiplied.
    pub fn round_score(
        &self,
        contract: u8,
        tricks: u8,
        cp: f64,
        add: f64,
        x: f64,
        nil_score: f64,
    ) -> f64 {
        let (base, add, x) = self.parts(contract, tricks, cp, add, x);
        let mult = (self.base_mult + add) * x;
        if self.flat_set && contract > 0 && tricks < contract {
            return (-(self.trick_value * 10.0 * contract as f64) + nil_score * mult).round();
        }
        ((base + nil_score) * mult).round()
    }

    /// Bags a team takes this round, and the penalty when its carried total crosses the limit.
    pub fn bags(&self, carried: u8, new: u8) -> (u8, f64) {
        let total = carried as u32 + new as u32;
        let crossings = total / BAG_LIMIT as u32;
        (
            (total % BAG_LIMIT as u32) as u8,
            -(crossings as f64) * BAG_PENALTY,
        )
    }

    pub fn card_price(&self, base: i32) -> i32 {
        ((base as f64 * self.card_price_scale / 5.0).round() as i32 * 5).max(5)
    }
}
