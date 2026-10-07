//! The fitted value model the shop policy and the AI read: sigil values, card values, the value
//! of gold, and the margin-to-win curve. `data/models/<step>.json`.

use crate::sigil::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct SigilValue {
    pub beta: f64,
    #[serde(default)]
    pub shop: f64,
    #[serde(default)]
    pub coh: f64,
    #[serde(default)]
    pub v: Vec<f64>,
    /// Standard error, for adaptive grant allocation.
    #[serde(default)]
    pub se: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub step: String,
    /// Value of one gold at shops 1–8 (index 0 unused), in smoothed-outcome units.
    pub lambda: Vec<f64>,
    pub sigils: HashMap<String, SigilValue>,
    /// Prior values for unmeasured sigils by "<rarity>/<category>".
    pub prior: HashMap<String, f64>,
    /// Card values by class ("S-A", "X-low", ...), engraving ("eng-bonus", ...), and
    /// "match" (per owned payoff whose filter the card matches).
    pub cards: HashMap<String, f64>,
    #[serde(default)]
    pub card_shop: f64,
    /// Logistic scale of the run margin by rounds left after the current round (0..=8).
    pub w_scale: Vec<f64>,
    /// The smoothing scale W for final margins.
    pub w_final: f64,
    /// Margin penalty for nil bids in bidding rollouts, in points.
    pub nil_handicap: f64,
    /// Expected round score by round 1–8 (index 0 unused), for valuing growth.
    pub round_scale: Vec<f64>,
}

impl Model {
    /// The hand-set starting model, used until the first fit.
    pub fn starting() -> Model {
        let mut prior = HashMap::new();
        for r in Rarity::all() {
            let base = match r {
                Rarity::Common => 0.05,
                Rarity::Uncommon => 0.07,
                Rarity::Rare => 0.09,
                Rarity::Legendary => 0.13,
            };
            for c in ["points", "mult", "xmult", "enabler"] {
                prior.insert(format!("{}/{}", r.name(), c), base);
            }
        }
        let mut cards = HashMap::new();
        for (k, v) in [
            ("X-A", 0.06),
            ("X-K", 0.04),
            ("X-Q", 0.025),
            ("X-J", 0.018),
            ("X-10", 0.012),
            ("X-low", 0.006),
            ("S-A", 0.09),
            ("S-K", 0.07),
            ("S-Q", 0.05),
            ("S-J", 0.035),
            ("S-10", 0.03),
            ("S-low", 0.02),
            ("eng-bonus", 0.02),
            ("eng-herald", 0.015),
            ("eng-mult", 0.04),
            ("eng-synthetic", 0.0),
            ("match", 0.015),
        ] {
            cards.insert(k.to_string(), v);
        }
        // Plain-Spades round scores grow with the par curve once sigils arrive; until a fit,
        // assume par.
        let par: [f64; 9] = [
            0.0, 600.0, 850.0, 1150.0, 1600.0, 2250.0, 3100.0, 4300.0, 6000.0,
        ];
        let mut w_scale = vec![0.0; 9];
        // Rounds left r: remaining rounds are 9-r..=8; spread ~ 1.2 x par per round.
        for (r, w) in w_scale.iter_mut().enumerate() {
            let var: f64 = (9 - r.min(8)..=8)
                .filter(|&k| r > 0 && k >= 1)
                .map(|k| (1.2 * par[k]).powi(2))
                .sum();
            *w = if r == 0 { 50.0 } else { 0.588 * var.sqrt() };
        }
        Model {
            step: "starting".into(),
            lambda: vec![0.0006; 9],
            sigils: HashMap::new(),
            prior,
            cards,
            card_shop: 0.0,
            w_scale,
            w_final: 0.588 * 1.2 * 6000.0,
            nil_handicap: 300.0,
            round_scale: par.to_vec(),
        }
    }

    pub fn load(path: &str) -> Model {
        let s = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read model {path}: {e}"));
        serde_json::from_str(&s).unwrap_or_else(|e| panic!("parse model {path}: {e}"))
    }

    pub fn sigil(&self, def: &SigilDef) -> SigilValue {
        if let Some(v) = self.sigils.get(&def.id) {
            return v.clone();
        }
        let key = format!("{}/{}", def.rarity.name(), def.effect.category());
        SigilValue {
            beta: *self.prior.get(&key).unwrap_or(&0.05),
            shop: -0.005,
            ..Default::default()
        }
    }

    pub fn card(&self, key: &str) -> f64 {
        *self.cards.get(key).unwrap_or(&0.0)
    }

    pub fn w(&self, margin: f64) -> f64 {
        1.0 / (1.0 + (-margin / self.w_final).exp())
    }
}

/// The card class key for value lookups.
pub fn card_class(suit: u8, rank: u8) -> String {
    let s = if suit == crate::cards::SPADES {
        "S"
    } else {
        "X"
    };
    let r = match rank {
        14 => "A",
        13 => "K",
        12 => "Q",
        11 => "J",
        10 => "10",
        _ => "low",
    };
    format!("{s}-{r}")
}
