//! A full run: eight shops and rounds between two teams, with grants, run records, and ledgers.

use crate::ai::search::*;
use crate::cards::*;
use crate::kernel::*;
use crate::model::*;
use crate::rng::*;
use crate::rules::rules;
use crate::sigil::*;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;

pub const ENG_SYNTH: u8 = 4;

// Stream labels.
const S_DEAL: u64 = 1;
pub(crate) const S_OFFER: u64 = 2;
const S_TEAM: u64 = 3;
const S_OPENING: u64 = 4;
const S_AI: u64 = 5;
const S_SETUP: u64 = 6;
const S_GRANT: u64 = 7;

pub struct Pool {
    pub defs: Vec<SigilDef>,
    pub by_id: HashMap<String, usize>,
}

impl Pool {
    pub fn new(defs: Vec<SigilDef>) -> Pool {
        let by_id = defs
            .iter()
            .enumerate()
            .map(|(i, d)| (d.id.clone(), i))
            .collect();
        Pool { defs, by_id }
    }
    pub fn load_dir(dir: &str) -> Pool {
        let mut defs = vec![];
        let mut paths: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        paths.sort();
        for p in paths {
            if p.extension().and_then(|x| x.to_str()) != Some("json") {
                continue;
            }
            let s = std::fs::read_to_string(&p).unwrap();
            let d: SigilDef =
                serde_json::from_str(&s).unwrap_or_else(|e| panic!("{}: {e}", p.display()));
            defs.push(d);
        }
        Pool::new(defs)
    }
    pub fn idx(&self, id: &str) -> usize {
        *self
            .by_id
            .get(id)
            .unwrap_or_else(|| panic!("unknown sigil {id}"))
    }
}

#[derive(Clone, Debug)]
pub enum Policy {
    Flexible,
    Committed(String),
    Chaser(Vec<usize>),
    /// A degenerate-strategy hunter that never buys anything.
    Hoard,
}

/// AI and measurement options per team (index = run team, not kernel team).
#[derive(Clone, Debug)]
pub struct AiCfg {
    /// Risk-neutral utility (expected margin) instead of win probability.
    pub risk_neutral: bool,
    /// Added to the team's second (non-nil) bid, clamped to 1–13: shifts the team contract.
    pub bid_offset: [i8; 2],
    /// Bid nil whenever the partner has not.
    pub always_nil: [bool; 2],
    /// Scale on the committed shopper's archetype bonus.
    pub commit_bonus: [f64; 2],
    /// The AI searches with its own team's sigils (false: base rules for its own team).
    pub aware: [bool; 2],
    /// Mixed into the AI's random streams (A/A tests: same deals and offers, other AI luck).
    pub ai_salt: u64,
    /// Determinization keeps hidden cards' true identities.
    pub true_ids: bool,
    /// Overrides the tier's bid discount and its growth per round.
    pub bid_discount: Option<f64>,
    pub bid_discount_slope: Option<f64>,
    /// The AI ignores bags when it searches (a control for bag awareness).
    pub bag_blind: [bool; 2],
    /// Never bid nil (a control for nil bidding).
    pub never_nil: [bool; 2],
}

impl Default for AiCfg {
    fn default() -> Self {
        AiCfg {
            risk_neutral: false,
            bid_offset: [0, 0],
            always_nil: [false, false],
            commit_bonus: [1.0, 1.0],
            aware: [true, true],
            ai_salt: 0,
            true_ids: false,
            bid_discount: None,
            bid_discount_slope: None,
            bag_blind: [false, false],
            never_nil: [false, false],
        }
    }
}

#[derive(Clone, Debug)]
pub struct GrantCfg {
    pub prob: f64,
    pub max_shop: u8,
    pub coherent: f64,
    /// Measured sigils and their grant weights.
    pub measured: Vec<(usize, f64)>,
    pub jitter: bool,
    pub card_prob: f64,
    /// Joint grants: (a, b) granted together with this probability per shop.
    pub pairs: Vec<(usize, usize)>,
    pub pair_prob: f64,
    /// Sigils granted at shop 1 in every run (pair arms).
    pub fixed: Vec<usize>,
    /// Grants cost no gold.
    pub free: bool,
}

#[derive(Clone, Debug)]
pub struct TeamCfg {
    pub policy: Policy,
    pub grants: Option<GrantCfg>,
}

#[derive(Clone)]
pub struct RunCfg<'a> {
    pub pool: &'a Pool,
    pub model: &'a Model,
    pub tiers: [Tier; 2],
    pub offerable: Vec<usize>,
    pub sigil_shop: bool,
    pub teams: [TeamCfg; 2],
    pub explore: f64,
    pub perturb: f64,
    pub perturb_amount: i32,
    pub rounds: u8,
    /// Whether the shop sells cards.
    pub cards: bool,
    /// Share of shop visits checked by tier-0 rollout rescoring.
    pub validate: f64,
    /// Code and pool versions, copied into every record.
    pub version: String,
    pub ai: AiCfg,
}

#[derive(Clone, Debug)]
pub(crate) struct OwnedSigil {
    pub(crate) def: usize,
    pub(crate) amount: Option<f64>,
    pub(crate) ratio: f64,
    pub(crate) shop: u8,
    pub(crate) grant: bool,
    pub(crate) price: i32,
    pub(crate) grow: f64,
    pub(crate) revealed: bool,
    pub(crate) sold: Option<u8>,
    pub(crate) fires: u32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct OwnedCard {
    pub(crate) ident: u8,
    pub(crate) slot: u8,
    pub(crate) eng: u8,
    pub(crate) price: i32,
    pub(crate) grant: bool,
    pub(crate) shop: u8,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CardOffer {
    pub(crate) ident: u8,
    pub(crate) slot: u8,
    pub(crate) eng: u8,
    pub(crate) price: i32,
}

#[derive(Clone)]
pub(crate) struct Team {
    pub(crate) sigils: Vec<OwnedSigil>,
    /// Sigils granted so far while planning, in order.
    pub(crate) granted: Vec<usize>,
    /// The planned grants (shop, sigil), fixed by the board seed so both runs of a board match.
    pub(crate) schedule: Vec<(u8, usize)>,
    pub(crate) history: Vec<OwnedSigil>,
    pub(crate) cards: Vec<OwnedCard>,
    pub(crate) sold_cards: Vec<OwnedCard>,
    pub(crate) owner: u8,
    pub(crate) gold: i32,
    pub(crate) score: f64,
    /// Bags carried toward the next penalty.
    pub(crate) bags: u8,
    /// Sigils offered to the team so far this run (for draws without replacement).
    pub(crate) offered: Vec<usize>,
}

impl Team {
    pub(crate) fn active(&self) -> impl Iterator<Item = &OwnedSigil> {
        self.sigils.iter()
    }
}

// ---------------------------------------------------------------------------------------------
// Records

#[derive(Serialize, Default, Clone)]
pub struct SigilRec {
    pub id: String,
    pub grant: bool,
    pub shop: u8,
    pub ratio: f64,
    pub amount: Option<f64>,
    pub price: i32,
    pub sold: Option<u8>,
    pub fires: u32,
}

#[derive(Serialize, Default, Clone)]
pub struct SigilRound {
    /// Index into the team's `sigils` record list.
    pub i: usize,
    pub f: u8,
    pub cp: f64,
    pub add: f64,
    pub x: f64,
    pub nilp: f64,
}

#[derive(Serialize, Default, Clone)]
pub struct RoundRec {
    pub bids: [i8; 2],
    pub contract: u8,
    pub tricks: u8,
    pub made: bool,
    pub nil_bids: u8,
    pub nil_made: u8,
    /// The round score, bag penalty included.
    pub score: f64,
    /// Bags taken this round and the penalty they caused (≤ 0).
    pub bags: u8,
    pub bag_pen: f64,
    pub eng_cp: f64,
    pub eng_add: f64,
    pub sig: Vec<SigilRound>,
    /// The predicted make probability of the team's last non-nil bid (bid calibration).
    pub pm: Option<f64>,
    /// The predicted contract tricks behind that bid.
    pub pt: Option<f64>,
}

#[derive(Serialize, Default, Clone)]
pub struct Bench {
    pub nil_plays: u32,
    pub nil_suicides: u32,
    pub overtake_opps: u32,
    pub overtakes: u32,
    pub trump_overtakes: u32,
}

#[derive(Serialize, Default, Clone)]
pub struct TeamRec {
    pub final_score: f64,
    pub sigils: Vec<SigilRec>,
    pub card_grants: Vec<(String, u8)>,
    /// (class, shop, grant, card index of its identity, still owned at the end)
    pub cards: Vec<(String, u8, bool, u8, bool)>,
    pub perturb: Vec<(u8, i32)>,
    pub gold: Vec<i32>,
    pub rerolls: u32,
    /// Every sigil offer the shop drew, rerolls included: (shop, offered ids).
    pub offers: Vec<(u8, Vec<String>)>,
    pub rounds: Vec<RoundRec>,
    pub bench: Bench,
}

#[derive(Serialize, Default, Clone)]
pub struct RunRec {
    pub board: u64,
    pub orient: u8,
    pub seed: u64,
    pub teams: [TeamRec; 2],
    /// Team A's result: 1 win, 0.5 draw, 0 loss.
    pub win: f64,
    pub margin: f64,
    pub clean: bool,
    pub checks: Vec<ShopCheck>,
    pub version: String,
}

// ---------------------------------------------------------------------------------------------

pub fn card_price(suit: u8, rank: u8) -> i32 {
    let side = match rank {
        14 => 80,
        13 => 60,
        12 => 45,
        11 => 35,
        10 => 25,
        _ => 15,
    };
    let sp = match rank {
        14 => 120,
        13 => 90,
        12 => 70,
        11 => 50,
        10 => 40,
        _ => 25,
    };
    rules().card_price(if suit == SPADES { sp } else { side })
}

fn eng_key(e: u8) -> &'static str {
    match e {
        ENG_BONUS => "eng-bonus",
        ENG_HERALD => "eng-herald",
        ENG_MULT => "eng-mult",
        ENG_SYNTH => "eng-synthetic",
        _ => "",
    }
}

pub(crate) fn sell_value(price: i32) -> i32 {
    rules().sell_value(price)
}

pub(crate) fn max_sigils() -> usize {
    rules().slots
}

pub(crate) fn max_cards() -> usize {
    rules().max_cards
}

#[derive(Clone)]
pub struct Runner<'a> {
    pub(crate) cfg: &'a RunCfg<'a>,
    pub(crate) seed: u64,
    pub(crate) orient: u8,
    pub(crate) teams: [Team; 2],
    pub(crate) taken: Mask,
    pub(crate) recs: [TeamRec; 2],
    pub(crate) rule_cache: Vec<Compiled>,
    pub(crate) tier_override: Option<Tier>,
    pub(crate) checks: Vec<ShopCheck>,
    /// When set, AI shop decisions are appended here (for the game client's session log).
    pub(crate) shop_log: Option<Vec<serde_json::Value>>,
}

/// One rollout-rescoring check of the shop model at a shop visit.
#[derive(Serialize, Default, Clone)]
pub struct ShopCheck {
    pub team: u8,
    pub shop: u8,
    pub ids: Vec<String>,
    pub model: Vec<f64>,
    pub rescored: Vec<f64>,
}

impl<'a> Runner<'a> {
    pub fn new(cfg: &'a RunCfg<'a>, seed: u64, orient: u8) -> Self {
        let mk = |ti: u64| {
            // Duplicate play swaps offer luck too: in the second run each team gets the other's
            // setup, offer, and exploration streams, while grants stay with the team.
            let mut r = Rng::stream(seed, &[S_SETUP, ti ^ orient as u64]);
            Team {
                sigils: vec![],
                granted: vec![],
                schedule: vec![],
                history: vec![],
                cards: vec![],
                sold_cards: vec![],
                owner: r.below(2) as u8,
                gold: rules().start_gold,
                score: 0.0,
                bags: 0,
                offered: vec![],
            }
        };
        let rule_cache = cfg
            .pool
            .defs
            .iter()
            .map(|d| compile(&d.effect, None).unwrap())
            .collect();
        Runner {
            cfg,
            seed,
            orient,
            teams: [mk(0), mk(1)],
            taken: 0,
            recs: Default::default(),
            rule_cache,
            tier_override: None,
            checks: vec![],
            shop_log: None,
        }
    }

    pub(crate) fn kteam(&self, ti: usize) -> usize {
        ti ^ self.orient as usize
    }
    pub(crate) fn tidx(&self, kteam: usize) -> usize {
        kteam ^ self.orient as usize
    }

    // ----- Values -----

    pub(crate) fn sigil_value(
        &self,
        ti: usize,
        def: usize,
        shop: u8,
        exclude: Option<usize>,
    ) -> f64 {
        let d = &self.cfg.pool.defs[def];
        let sv = self.cfg.model.sigil(d);
        let team = &self.teams[ti];
        let mut coh = 0;
        let mut pair = 0.0;
        for (k, o) in team.sigils.iter().enumerate() {
            if Some(k) == exclude || o.def == def {
                continue;
            }
            let od = &self.cfg.pool.defs[o.def];
            if od
                .archetypes
                .iter()
                .any(|a| a != "Generic" && d.archetypes.contains(a))
            {
                coh += 1;
            }
            if !sv.v.is_empty() {
                let ov = self.cfg.model.sigil(od);
                pair +=
                    sv.v.iter()
                        .zip(ov.v.iter())
                        .map(|(a, b)| a * b)
                        .sum::<f64>();
            }
        }
        let mut v = sv.beta + sv.shop * (shop as f64 - 3.5) + sv.coh * coh.min(2) as f64 + pair;
        match &self.cfg.teams[ti].policy {
            Policy::Committed(a) if d.archetypes.contains(a) => {
                v += 0.05 * self.cfg.ai.commit_bonus[ti]
            }
            Policy::Chaser(target) if target.contains(&def) => v += 1.0,
            _ => {}
        }
        v
    }

    pub(crate) fn card_value(&self, ti: usize, ident: u8, eng: u8, shop: u8) -> f64 {
        let m = self.cfg.model;
        let (s, r) = (nominal_suit(ident), nominal_rank(ident));
        let mut v = m.card(&card_class(s, r)) + m.card_shop * (shop as f64 - 3.5);
        if eng != ENG_NONE {
            v += m.card(eng_key(eng));
        }
        let mut matches = 0;
        for o in self.teams[ti].active() {
            if let Compiled::Payoff(p) = self.rule_cache[o.def] {
                let f = match p.trig {
                    Trig::Win { filt, .. } | Trig::Lead { filt, .. } | Trig::Hold { filt, .. } => {
                        filt
                    }
                    _ => continue,
                };
                if f != CardFilter::ANY && f.matches(s, r) {
                    matches += 1;
                }
            }
        }
        v += m.card("match") * matches as f64;
        if let Policy::Committed(a) = &self.cfg.teams[ti].policy {
            if committed_card(a, s, r) {
                v += 0.02 * self.cfg.ai.commit_bonus[ti];
            }
        }
        v
    }

    /// The owned sigil the shop would sell first. Granted sigils hold their slot for the run, so
    /// the shop never sells one to buy something else; only a new grant can displace a grant.
    pub(crate) fn weakest_sigil(
        &self,
        ti: usize,
        shop: u8,
        protect: Option<usize>,
    ) -> Option<(usize, f64)> {
        self.weakest_sigil_of(ti, shop, protect, false)
    }

    pub(crate) fn weakest_sigil_of(
        &self,
        ti: usize,
        shop: u8,
        protect: Option<usize>,
        grants: bool,
    ) -> Option<(usize, f64)> {
        let team = &self.teams[ti];
        let any_free = team.sigils.iter().any(|o| !o.grant);
        (0..team.sigils.len())
            .filter(|&k| Some(k) != protect)
            .filter(|&k| !team.sigils[k].grant || (grants && !any_free))
            .map(|k| (k, self.sigil_value(ti, team.sigils[k].def, shop, Some(k))))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub(crate) fn weakest_card(&self, ti: usize, shop: u8) -> Option<(usize, f64)> {
        let team = &self.teams[ti];
        (0..team.cards.len())
            .map(|k| {
                (
                    k,
                    self.card_value(ti, team.cards[k].ident, team.cards[k].eng, shop),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    pub(crate) fn sell_sigil(&mut self, ti: usize, k: usize, shop: u8) {
        let mut o = self.teams[ti].sigils.remove(k);
        self.teams[ti].gold += sell_value(o.price);
        o.sold = Some(shop);
        self.teams[ti].history.push(o);
    }

    pub(crate) fn sell_card(&mut self, ti: usize, k: usize) {
        let c = self.teams[ti].cards.remove(k);
        self.teams[ti].gold += sell_value(c.price);
        self.taken &= !bit(c.slot);
        self.teams[ti].sold_cards.push(c);
    }

    // ----- Offers -----

    pub(crate) fn sigil_offers(&self, ti: usize, rng: &mut Rng) -> Vec<usize> {
        if !self.cfg.sigil_shop {
            return vec![];
        }
        let rl = rules();
        let mut owned: Vec<usize> = self.teams[ti].sigils.iter().map(|o| o.def).collect();
        owned.extend(self.teams[ti].schedule.iter().map(|x| x.1));
        match rl.offer_draws.as_str() {
            "team" => owned.extend(self.teams[ti].offered.iter().cloned()),
            "shared" => owned.extend(self.teams[1 - ti].sigils.iter().map(|o| o.def)),
            _ => {}
        }
        let mut out = vec![];
        for _ in 0..rl.sigil_offers {
            let roll = rng.f64();
            let mut acc = 0.0;
            let mut rar = Rarity::Common;
            for r in Rarity::all() {
                acc += r.odds();
                if roll < acc {
                    rar = r;
                    break;
                }
            }
            let mut ri = rar.index() as i32;
            while ri >= 0 {
                let r = Rarity::all()[ri as usize];
                let cands: Vec<usize> = self
                    .cfg
                    .offerable
                    .iter()
                    .cloned()
                    .filter(|&d| {
                        self.cfg.pool.defs[d].rarity == r
                            && !owned.contains(&d)
                            && !out.contains(&d)
                    })
                    .collect();
                if !cands.is_empty() {
                    out.push(cands[rng.below(cands.len())]);
                    break;
                }
                ri -= 1;
            }
        }
        out
    }

    /// Records offers as seen (for draws without replacement).
    pub(crate) fn note_offers(&mut self, ti: usize, sig: &[usize]) {
        for &d in sig {
            if !self.teams[ti].offered.contains(&d) {
                self.teams[ti].offered.push(d);
            }
        }
    }

    fn rec_offers(&mut self, ti: usize, shop: u8, sig: &[usize]) {
        if self.cfg.sigil_shop {
            let ids = sig
                .iter()
                .map(|&d| self.cfg.pool.defs[d].id.clone())
                .collect();
            self.recs[ti].offers.push((shop, ids));
        }
    }

    pub(crate) fn card_offer(&self, _shop: u8, rng: &mut Rng, exclude: Mask) -> Option<CardOffer> {
        let free = FULL_DECK & !self.taken & !exclude;
        if free == 0 {
            return None;
        }
        let c = rng.pick_bit(free);
        Some(CardOffer {
            ident: c,
            slot: c,
            eng: ENG_NONE,
            price: card_price(nominal_suit(c), nominal_rank(c)),
        })
    }

    pub(crate) fn card_offers(&self, shop: u8, rng: &mut Rng, exclude: Mask) -> Vec<CardOffer> {
        let mut out: Vec<CardOffer> = vec![];
        if !self.cfg.cards {
            return out;
        }
        let mut ex = exclude;
        for _ in 0..rules().card_offers {
            if let Some(o) = self.card_offer(shop, rng, ex) {
                ex |= bit(o.slot);
                out.push(o);
            }
        }
        out
    }

    // ----- Grants -----

    pub(crate) fn draw_grant(&self, ti: usize, g: &GrantCfg, rng: &mut Rng) -> Option<usize> {
        let owned: Vec<usize> = self.teams[ti].granted.clone();
        let pool: Vec<(usize, f64)> = g
            .measured
            .iter()
            .cloned()
            .filter(|(d, _)| !owned.contains(d))
            .collect();
        if pool.is_empty() {
            return None;
        }
        if rng.chance(g.coherent) {
            // An archetype weighted toward what the team holds.
            let mut w = vec![0.0; ARCHETYPES.len()];
            for (a, x) in w.iter_mut().enumerate() {
                if pool.iter().any(|(d, _)| {
                    self.cfg.pool.defs[*d]
                        .archetypes
                        .iter()
                        .any(|t| t == ARCHETYPES[a])
                }) {
                    *x = 0.5;
                }
            }
            for &o in &self.teams[ti].granted {
                for t in &self.cfg.pool.defs[o].archetypes {
                    if let Some(a) = archetype_index(t) {
                        if w[a] > 0.0 {
                            w[a] += 1.0;
                        }
                    }
                }
            }
            if w.iter().sum::<f64>() > 0.0 {
                let a = ARCHETYPES[rng.weighted(&w)];
                let tagged: Vec<(usize, f64)> = pool
                    .iter()
                    .cloned()
                    .filter(|(d, _)| self.cfg.pool.defs[*d].archetypes.iter().any(|t| t == a))
                    .collect();
                if !tagged.is_empty() {
                    let ws: Vec<f64> = tagged.iter().map(|x| x.1).collect();
                    return Some(tagged[rng.weighted(&ws)].0);
                }
            }
        }
        let ws: Vec<f64> = pool.iter().map(|x| x.1).collect();
        Some(pool[rng.weighted(&ws)].0)
    }

    pub(crate) fn grant(&mut self, ti: usize, def: usize, shop: u8, rng: &mut Rng, jitter: bool) {
        if self.teams[ti].sigils.len() >= max_sigils() {
            if let Some((k, _)) = self.weakest_sigil_of(ti, shop, None, true) {
                self.sell_sigil(ti, k, shop);
            }
        }
        let d = &self.cfg.pool.defs[def];
        let base = d.effect.tunable();
        let (amount, ratio) = match base {
            Some(a) if jitter => {
                let lr = (rng.f64() * 2.0 - 1.0) * std::f64::consts::LN_2;
                let ratio = lr.exp();
                let amt = if d.effect.category() == "xmult" {
                    1.0 + (a - 1.0) * ratio
                } else {
                    a * ratio
                };
                (Some(amt), lr)
            }
            _ => (None, 0.0),
        };
        let price = d.price();
        let free = self.cfg.teams[ti].grants.as_ref().is_some_and(|g| g.free);
        if !free {
            self.teams[ti].gold -= price;
        }
        self.teams[ti].sigils.push(OwnedSigil {
            def,
            amount,
            ratio,
            shop,
            grant: true,
            price,
            grow: 0.0,
            revealed: false,
            sold: None,
            fires: 0,
        });
    }

    // ----- Shop -----

    pub(crate) fn shop(&mut self, ti: usize, shop: u8, other_offers: Mask) -> Mask {
        let luck = (ti ^ self.orient as usize) as u64;
        let mut rng = Rng::stream(self.seed, &[S_TEAM, luck, shop as u64]);
        // Grant schedules depend only on the board seed and the team's earlier grants, so both runs
        // of a board apply the same treatments while the luck swaps.
        let seed = self.seed;
        let gs = |k: u64| Rng::stream(seed, &[S_GRANT, ti as u64, shop as u64, k]);
        self.recs[ti].gold.push(self.teams[ti].gold);
        let mut prng = gs(1);
        if self.cfg.perturb > 0.0 && prng.chance(self.cfg.perturb) {
            let amt = if prng.chance(0.5) {
                self.cfg.perturb_amount
            } else {
                -self.cfg.perturb_amount
            };
            self.teams[ti].gold += amt;
            self.recs[ti].perturb.push((shop, amt));
        }
        if let Some(g) = self.cfg.teams[ti].grants.clone() {
            if shop <= g.max_shop {
                let todo: Vec<usize> = self.teams[ti]
                    .schedule
                    .iter()
                    .filter(|x| x.0 == shop)
                    .map(|x| x.1)
                    .collect();
                for d in todo {
                    // Scheduled grants are never offered to this team, so it can't already own them.
                    if !self.teams[ti].sigils.iter().any(|o| o.def == d) {
                        let mut arng = Rng::stream(
                            self.seed,
                            &[
                                S_GRANT,
                                ti as u64,
                                shop as u64,
                                4,
                                hash_str(&self.cfg.pool.defs[d].id),
                            ],
                        );
                        self.grant(ti, d, shop, &mut arng, g.jitter);
                    }
                }
                let mut crng = gs(3);
                if shop <= 7 && crng.chance(g.card_prob) {
                    if let Some(o) = self.card_offer(shop, &mut crng, other_offers) {
                        if self.teams[ti].cards.len() >= max_cards() {
                            if let Some((k, _)) = self.weakest_card(ti, shop) {
                                self.sell_card(ti, k);
                            }
                        }
                        self.buy_card(ti, o, shop, true);
                        self.recs[ti].card_grants.push((
                            card_class(nominal_suit(o.ident), nominal_rank(o.ident)),
                            shop,
                        ));
                    }
                }
            }
        }

        let lambda = at(&self.cfg.model.lambda, shop as usize);
        let mut reroll = 0u64;
        let mut orng = Rng::stream(self.seed, &[S_OFFER, luck, shop as u64, reroll]);
        let mut sig = self.sigil_offers(ti, &mut orng);
        self.note_offers(ti, &sig);
        self.rec_offers(ti, shop, &sig);
        let mut cards = self.card_offers(shop, &mut orng, other_offers);
        self.log_offers(ti, shop, &sig, &cards);
        if self.cfg.validate > 0.0
            && sig.len() >= 2
            && Rng::stream(self.seed, &[S_TEAM, 99, ti as u64, shop as u64])
                .chance(self.cfg.validate)
        {
            self.check_offers(ti, shop, &sig);
        }
        let limit = match rules().purchases {
            0 => 30,
            n => n as usize,
        };
        let mut bought = 0;
        for _ in 0..30 {
            if bought >= limit || matches!(self.cfg.teams[ti].policy, Policy::Hoard) {
                break;
            }
            // (net gain, is_sigil, offer index)
            let mut opts: Vec<(f64, bool, usize)> = vec![];
            for (i, &d) in sig.iter().enumerate() {
                let price = self.cfg.pool.defs[d].price();
                if self.teams[ti].gold < price {
                    continue;
                }
                let v = self.sigil_value(ti, d, shop, None);
                let net = if self.teams[ti].sigils.len() < max_sigils() {
                    v - lambda * price as f64
                } else {
                    match self.weakest_sigil(ti, shop, None) {
                        Some((k, wv)) => {
                            v - wv
                                - lambda
                                    * (price - sell_value(self.teams[ti].sigils[k].price)) as f64
                        }
                        None => continue,
                    }
                };
                opts.push((net, true, i));
            }
            for (i, o) in cards.iter().enumerate() {
                if self.teams[ti].gold < o.price {
                    continue;
                }
                let v = self.card_value(ti, o.ident, o.eng, shop);
                let net = if self.teams[ti].cards.len() < max_cards() {
                    v - lambda * o.price as f64
                } else {
                    match self.weakest_card(ti, shop) {
                        Some((k, wv)) => {
                            v - wv
                                - lambda
                                    * (o.price - sell_value(self.teams[ti].cards[k].price)) as f64
                        }
                        None => continue,
                    }
                };
                opts.push((net, false, i));
            }
            let best = opts
                .iter()
                .cloned()
                .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            match best {
                Some(b) if b.0 > 0.0 => {
                    let choice = if rng.chance(self.cfg.explore) {
                        opts[rng.below(opts.len())]
                    } else {
                        b
                    };
                    self.log_choice(ti, &opts, choice, &sig, &cards);
                    bought += 1;
                    if choice.1 {
                        let d = sig.remove(choice.2);
                        if self.teams[ti].sigils.len() >= max_sigils() {
                            if let Some((k, _)) = self.weakest_sigil(ti, shop, None) {
                                self.log_shop(json!({"ev": "sell", "team": ti, "sigil": self.cfg.pool.defs[self.teams[ti].sigils[k].def].id}));
                                self.sell_sigil(ti, k, shop);
                            }
                        }
                        let price = self.cfg.pool.defs[d].price();
                        self.teams[ti].gold -= price;
                        self.teams[ti].sigils.push(OwnedSigil {
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
                        });
                    } else {
                        let o = cards.remove(choice.2);
                        if self.teams[ti].cards.len() >= max_cards() {
                            if let Some((k, _)) = self.weakest_card(ti, shop) {
                                let c = self.teams[ti].cards[k];
                                self.log_shop(json!({"ev": "sellCard", "team": ti, "card": offer_str(c.ident, c.slot, c.eng)}));
                                self.sell_card(ti, k);
                            }
                        }
                        self.buy_card(ti, o, shop, false);
                    }
                }
                _ => {
                    let cost = rules().reroll(reroll);
                    if reroll == 0 && self.teams[ti].gold - cost >= 100 {
                        self.teams[ti].gold -= cost;
                        reroll += 1;
                        self.recs[ti].rerolls += 1;
                        orng = Rng::stream(self.seed, &[S_OFFER, luck, shop as u64, reroll]);
                        sig = self.sigil_offers(ti, &mut orng);
                        self.note_offers(ti, &sig);
                        self.rec_offers(ti, shop, &sig);
                        cards = self.card_offers(shop, &mut orng, other_offers);
                        self.log_shop(json!({"ev": "reroll", "team": ti, "cost": cost}));
                        self.log_offers(ti, shop, &sig, &cards);
                    } else {
                        break;
                    }
                }
            }
        }
        let mut offered = 0;
        for o in &cards {
            offered |= bit(o.slot);
        }
        offered
    }

    fn log_shop(&mut self, v: serde_json::Value) {
        if let Some(l) = self.shop_log.as_mut() {
            l.push(v);
        }
    }

    fn log_offers(&mut self, ti: usize, shop: u8, sig: &[usize], cards: &[CardOffer]) {
        if self.shop_log.is_none() {
            return;
        }
        let v = json!({
            "ev": "offers", "team": ti, "shop": shop, "gold": self.teams[ti].gold,
            "sigils": sig.iter().map(|&d| self.cfg.pool.defs[d].id.clone()).collect::<Vec<_>>(),
            "cards": cards.iter().map(|o| format!("{} ({}g)", offer_str(o.ident, o.slot, o.eng), o.price)).collect::<Vec<_>>(),
        });
        self.log_shop(v);
    }

    /// Logs a purchase with every option the shop policy weighed (net value per option).
    fn log_choice(
        &mut self,
        ti: usize,
        opts: &[(f64, bool, usize)],
        choice: (f64, bool, usize),
        sig: &[usize],
        cards: &[CardOffer],
    ) {
        if self.shop_log.is_none() {
            return;
        }
        let name = |o: &(f64, bool, usize)| {
            if o.1 {
                self.cfg.pool.defs[sig[o.2]].id.clone()
            } else {
                let c = cards[o.2];
                offer_str(c.ident, c.slot, c.eng)
            }
        };
        let options: Vec<serde_json::Value> = opts
            .iter()
            .map(|o| json!({"offer": name(o), "net": (o.0 * 1e4).round() / 1e4}))
            .collect();
        let (price, kind) = if choice.1 {
            (self.cfg.pool.defs[sig[choice.2]].price(), "sigil")
        } else {
            (cards[choice.2].price, "card")
        };
        let v = json!({"ev": "buy", "team": ti, "kind": kind, "offer": name(&choice), "price": price, "options": options});
        self.log_shop(v);
    }

    pub(crate) fn buy_card(&mut self, ti: usize, o: CardOffer, shop: u8, grant: bool) {
        self.teams[ti].gold -= o.price;
        self.taken |= bit(o.slot);
        self.teams[ti].cards.push(OwnedCard {
            ident: o.ident,
            slot: o.slot,
            eng: o.eng,
            price: o.price,
            grant,
            shop,
        });
    }

    // ----- Rounds -----

    pub(crate) fn rules_for(&self) -> Rules {
        let mk = |kt: usize| {
            let ti = self.tidx(kt);
            let slots = self.teams[ti]
                .sigils
                .iter()
                .map(|o| {
                    let c = compile(&self.cfg.pool.defs[o.def].effect, o.amount).unwrap();
                    match c {
                        Compiled::Payoff(p) => Slot {
                            payoff: Some(p),
                            rule: None,
                            grow_value: o.grow,
                        },
                        Compiled::Rule(r) => Slot {
                            payoff: None,
                            rule: Some(r),
                            grow_value: 0.0,
                        },
                    }
                })
                .collect();
            TeamProgram::new(slots)
        };
        Rules {
            teams: [mk(0), mk(1)],
        }
    }

    /// What team `kt` searches with: every sigil is public, so both programs, unless the AI is
    /// told to ignore its own team's sigils.
    pub(crate) fn view_rules(&self, rules: &Rules, kt: usize) -> Rules {
        let mut teams = [rules.teams[0].clone(), rules.teams[1].clone()];
        if !self.cfg.ai.aware[self.tidx(kt)] {
            let n = rules.teams[kt].slots.len();
            teams[kt] = rules.teams[kt].visible(&vec![false; n]);
        }
        Rules { teams }
    }

    pub(crate) fn utility(&self, kt: usize, round: u8) -> Utility {
        let m = self.cfg.model;
        let ti = self.tidx(kt);
        let margin = self.teams[ti].score - self.teams[1 - ti].score;
        let left = (self.cfg.rounds - round) as usize;
        let future: f64 = ((round + 1)..=self.cfg.rounds)
            .map(|k| at(&m.round_scale, k as usize))
            .sum();
        let nfut = left as f64;
        Utility {
            linear: self.cfg.ai.risk_neutral,
            margin,
            scale: m.w_scale[left.min(8)],
            growth: [8.0 * nfut, future / 15.0, future, 2.0 * nfut],
            nil_handicap: m.nil_handicap,
            gold: if round < self.cfg.rounds {
                at(&m.lambda, (round + 1) as usize) / 2.0
            } else {
                0.0
            },
            // About 1.5 bags per round are still to come, so a carried bag is a full 100-point
            // share of the next penalty when enough rounds remain, and nothing after the last.
            bag_cost: crate::rules::BAG_PENALTY / crate::rules::BAG_LIMIT as f64
                * (1.5 * nfut / crate::rules::BAG_LIMIT as f64).min(1.0),
            bags: !self.cfg.ai.bag_blind[ti],
        }
    }

    pub(crate) fn play_round(&mut self, round: u8) {
        let mut rs = self.deal(round);
        for (kt, k, r) in rs.opening.clone() {
            if let Rule::Swap(n) = r {
                let (a, b) = self.opening_seats(kt, k);
                let ga = self.swap_ai(&rs, a as u8, n);
                let gb = self.swap_ai(&rs, b as u8, n);
                self.apply_swap(&mut rs, a, b, ga, gb);
                self.teams[self.tidx(kt)].sigils[k].revealed = true;
            } else {
                self.apply_opening(&mut rs, kt, k, r);
            }
        }
        for k in 0..4 {
            let s = ((rs.play.dealer + 1 + k) % 4) as usize;
            let (b, _) = self.bid_ai(&mut rs, s as u8);
            rs.play.bids[s] = b;
        }
        self.finish_bidding(&mut rs);
        while !rs.play.done() {
            let s = rs.mover();
            let (m, _) = self.move_ai(&mut rs, s);
            self.apply_move(&mut rs, s, m);
        }
        self.finish_round(&rs);
    }

    /// Deals a round: owned cards to their owners, the rest by stable dealing, and the Opening
    /// queue in the kernel's fixed priority.
    pub(crate) fn deal(&self, round: u8) -> RoundState {
        let dealer = ((Rng::stream(self.seed, &[S_SETUP, 9]).below(4) as u8) + round - 1) % 4;
        let mut deal_rng = Rng::stream(self.seed, &[S_DEAL, round as u64]);
        let mut perm: Vec<u8> = (0..52).collect();
        deal_rng.shuffle(&mut perm);

        let mut id = Identity::standard();
        let mut eng = [ENG_NONE; 52];
        let mut hands = [0u64; 4];
        let mut owned_mask = 0u64;
        let mut known = [0u64; 4];
        for ti in 0..2 {
            let kt = self.kteam(ti);
            let seat = kt + 2 * self.teams[ti].owner as usize;
            for c in &self.teams[ti].cards {
                hands[seat] |= bit(c.slot);
                owned_mask |= bit(c.slot);
                known[seat] |= bit(c.slot);
                if c.eng == ENG_SYNTH {
                    id.set(c.slot, nominal_suit(c.ident), nominal_rank(c.ident));
                } else {
                    eng[c.slot as usize] = c.eng;
                }
            }
        }
        // Stable dealing: permutation position i belongs to seat (dealer + 1 + i) mod 4. Owned
        // cards leave their positions; seats with too many cards pass the overflow, in
        // permutation order, to seats that are short. Small ownership changes move few cards.
        let mut overflow: Vec<u8> = vec![];
        for (i, &c) in perm.iter().enumerate() {
            if owned_mask & bit(c) != 0 {
                continue;
            }
            let seat = ((dealer as usize) + 1 + i) % 4;
            if hands[seat].count_ones() < 13 {
                hands[seat] |= bit(c);
            } else {
                overflow.push(c);
            }
        }
        let mut seat = (dealer as usize + 1) % 4;
        for c in overflow {
            let mut guard = 0;
            while hands[seat].count_ones() >= 13 && guard < 4 {
                seat = (seat + 1) % 4;
                guard += 1;
            }
            hands[seat] |= bit(c);
            seat = (seat + 1) % 4;
        }

        let rules = self.rules_for();
        let mut play = Play::new(id, hands, eng, dealer, &rules);
        for kt in 0..2 {
            play.bags[kt] = self.teams[self.tidx(kt)].bags;
        }

        let mut opening = vec![];
        for kt in 0..2 {
            let mut order: Vec<(u8, usize, Rule)> = vec![];
            for (k, s) in rules.teams[kt].slots.iter().enumerate() {
                if let Some(r) = s.rule {
                    let pri = match r {
                        Rule::Swap(_) => 0,
                        Rule::BecomeSuit(..) => 1,
                        Rule::BecomeFrom { suit: Some(_), .. } => 1,
                        Rule::BecomeRank(..) => 2,
                        Rule::BecomeFrom { .. } => 2,
                        Rule::BecomeRange(..) => 3,
                        Rule::Raise(_) | Rule::RaiseFrom(..) => 4,
                        _ => continue,
                    };
                    order.push((pri, k, r));
                }
            }
            order.sort_by_key(|x| (x.0, x.1));
            opening.extend(order.into_iter().map(|(_, k, r)| (kt, k, r)));
        }
        let ai = (0..4)
            .map(|s| match self.cfg.ai.ai_salt {
                0 => Rng::stream(self.seed, &[S_AI, round as u64, s as u64]),
                salt => Rng::stream(self.seed, &[S_AI, round as u64, s as u64, salt]),
            })
            .collect();
        RoundState {
            round,
            base_id: id,
            play,
            rules,
            known,
            voids: [[false; 4]; 4],
            ai,
            opening,
            pmake: [f64::NAN; 4],
            ptricks: [f64::NAN; 4],
            bid_order: vec![],
        }
    }

    /// The two seats an Opening effect acts on: the owning team's, unless it names the opponents.
    pub(crate) fn opening_seats(&self, kt: usize, k: usize) -> (usize, usize) {
        let ti = self.tidx(kt);
        let opp = self.cfg.pool.defs[self.teams[ti].sigils[k].def]
            .effect
            .whose
            .as_deref()
            == Some("opponents");
        if opp {
            (1 - kt, 3 - kt)
        } else {
            (kt, kt + 2)
        }
    }

    /// The cards an AI seat gives in an Opening swap.
    pub(crate) fn swap_ai(&self, rs: &RoundState, seat: u8, n: u8) -> Mask {
        let kt = (seat % 2) as usize;
        let tier = self.tier_override.unwrap_or(self.cfg.tiers[self.tidx(kt)]);
        let view = self.view_rules(&rs.rules, kt);
        let u = self.utility(kt, rs.round);
        let know = self.knowledge(&rs.play, seat, &rs.known, &[[false; 4]; 4]);
        let mut r = match self.cfg.ai.ai_salt {
            0 => Rng::stream(self.seed, &[S_AI, rs.round as u64, seat as u64, 77]),
            salt => Rng::stream(self.seed, &[S_AI, rs.round as u64, seat as u64, 77, salt]),
        };
        choose_swap(&rs.play, &view, &know, &u, &tier, &mut r, n)
    }

    /// Exchanges equal counts between two partners, keeping what each knows of owned cards.
    pub(crate) fn apply_swap(&self, rs: &mut RoundState, a: usize, b: usize, ga: Mask, gb: Mask) {
        let n = ga.count_ones().min(gb.count_ones());
        let (ga, gb) = (trim(ga, n), trim(gb, n));
        rs.play.hands[a] = (rs.play.hands[a] & !ga) | gb;
        rs.play.hands[b] = (rs.play.hands[b] & !gb) | ga;
        let (ka, kb) = (rs.known[a], rs.known[b]);
        rs.known[a] = (ka & !ga) | (kb & gb);
        rs.known[b] = (kb & !gb) | (ka & ga);
    }

    /// Resolves one random or fixed Opening change (every Opening rule except the swap).
    pub(crate) fn apply_opening(&mut self, rs: &mut RoundState, kt: usize, k: usize, r: Rule) {
        let ti = self.tidx(kt);
        let sid = hash_str(&self.cfg.pool.defs[self.teams[ti].sigils[k].def].id);
        let mut orng = Rng::stream(
            self.seed,
            &[
                S_OPENING,
                rs.round as u64,
                (ti ^ self.orient as usize) as u64,
                sid,
            ],
        );
        let (a, b) = self.opening_seats(kt, k);
        let play = &mut rs.play;
        match r {
            Rule::BecomeRank(n, rank) | Rule::BecomeRange(n, rank, _) => {
                let hi = if let Rule::BecomeRange(_, _, h) = r {
                    h
                } else {
                    rank
                };
                for c in sample(play.hands[a] | play.hands[b], n, &mut orng) {
                    let nr = rank + orng.below((hi - rank + 1) as usize) as u8;
                    play.id.set(c, play.id.suit[c as usize], nr);
                }
            }
            Rule::BecomeSuit(n, suit) => {
                for c in sample(play.hands[a] | play.hands[b], n, &mut orng) {
                    play.id.set(c, suit, play.id.rank[c as usize]);
                }
            }
            Rule::Raise(n) => {
                for c in Bits(play.hands[a] | play.hands[b]) {
                    let r = play.id.rank[c as usize];
                    play.id.set(c, play.id.suit[c as usize], (r + n).min(ACE));
                }
            }
            Rule::RaiseFrom(n, from) => {
                for c in Bits(play.hands[a] | play.hands[b]) {
                    let (su, r) = (play.id.suit[c as usize], play.id.rank[c as usize]);
                    if from.matches(su, r) {
                        play.id.set(c, su, (r + n).min(ACE));
                    }
                }
            }
            Rule::BecomeFrom {
                count,
                from,
                suit,
                rank,
            } => {
                let mut elig = 0;
                for c in Bits(play.hands[a] | play.hands[b]) {
                    if from.matches(play.id.suit[c as usize], play.id.rank[c as usize]) {
                        elig |= bit(c);
                    }
                }
                let picked: Vec<u8> = if count == 0 {
                    Bits(elig).collect()
                } else {
                    sample(elig, count, &mut orng)
                };
                for c in picked {
                    let ns = suit.unwrap_or(play.id.suit[c as usize]);
                    let nr = rank.unwrap_or(play.id.rank[c as usize]);
                    play.id.set(c, ns, nr);
                }
            }
            _ => {}
        }
        self.teams[ti].sigils[k].revealed = true;
    }

    /// An AI seat's bid, with the value of each candidate bid.
    pub(crate) fn bid_ai(&self, rs: &mut RoundState, s: u8) -> (i8, Vec<(i8, f64)>) {
        let kt = (s % 2) as usize;
        let mut tier = self.tier_override.unwrap_or(self.cfg.tiers[self.tidx(kt)]);
        if let Some(d) = self.cfg.ai.bid_discount {
            tier.bid_discount = d;
        }
        if let Some(sl) = self.cfg.ai.bid_discount_slope {
            tier.bid_discount_slope = sl;
        }
        // Heuristic rollouts overestimate the bidder's tricks more as sigils accumulate (both
        // teams' sigil counts are public).
        let held = self.teams[0].sigils.len() + self.teams[1].sigils.len();
        tier.bid_discount += tier.bid_discount_slope * held as f64;
        let view = self.view_rules(&rs.rules, kt);
        let know = self.knowledge(&rs.play, s, &rs.known, &rs.voids);
        let u = self.utility(kt, rs.round);
        let (mut b, v, pm) =
            choose_bid_full(&rs.play, &view, &know, &u, &tier, &mut rs.ai[s as usize]);
        let ti = self.tidx(kt);
        let ai = &self.cfg.ai;
        let partner = rs.play.bids[(s as usize + 2) % 4];
        if ai.always_nil[ti] && partner != 0 {
            b = 0;
        } else if ai.never_nil[ti] && b == 0 {
            b = v
                .iter()
                .filter(|x| x.0 != 0)
                .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap())
                .map_or(1, |x| x.0);
        } else if b != 0 && ai.bid_offset[ti] != 0 && partner >= 0 {
            // The team's second bidder shifts the team contract by the offset.
            b = (b + ai.bid_offset[ti]).clamp(1, 13);
        }
        rs.pmake[s as usize] = if b == 0 { f64::NAN } else { pm.pmake };
        rs.ptricks[s as usize] = if b == 0 { f64::NAN } else { pm.tricks };
        rs.bid_order.push(s);
        (b, v)
    }

    /// After the last bid: who is behind, then always-on, holding, bid, and growth triggers.
    pub(crate) fn finish_bidding(&self, rs: &mut RoundState) {
        for kt in 0..2 {
            let ti = self.tidx(kt);
            rs.play.behind[kt] = self.teams[ti].score < self.teams[1 - ti].score;
        }
        rs.play.post_bid(&rs.rules);
    }

    /// An AI seat's move (a card or a lead choice), with the search value of each legal move.
    pub(crate) fn move_ai(&self, rs: &mut RoundState, s: u8) -> (u8, Vec<(u8, f64)>) {
        let kt = (s % 2) as usize;
        let tier = self.tier_override.unwrap_or(self.cfg.tiers[self.tidx(kt)]);
        let view = self.view_rules(&rs.rules, kt);
        let know = self.knowledge(&rs.play, s, &rs.known, &rs.voids);
        let u = self.utility(kt, rs.round);
        choose_move_explain(&rs.play, &view, &know, &u, &tier, &mut rs.ai[s as usize])
    }

    /// Applies a move, recording voids and reveals. Returns the trick if this move ended one.
    pub(crate) fn apply_move(&mut self, rs: &mut RoundState, s: u8, m: u8) -> Option<TrickResult> {
        let play = &mut rs.play;
        if m < 52 {
            self.bench(play, s, m);
            if play.tlen > 0 {
                let led = play.id.suit[play.trick[0] as usize];
                let any = play.any_suit_now(s);
                if play.id.suit[m as usize] != led && !any {
                    rs.voids[s as usize][led as usize] = true;
                }
            }
        }
        let res = play.apply(m, &rs.rules);
        if res.is_some() {
            self.reveal(&rs.play, &rs.rules);
        }
        res
    }

    /// Scores the round, writes the ledger, banks growth, and pays income.
    pub(crate) fn finish_round(&mut self, rs: &RoundState) -> ([TeamScore; 2], [Income; 2]) {
        let play = &rs.play;
        let rules = &rs.rules;
        let sc = play.score(rules);
        let mut income = [Income::default(); 2];
        for kt in 0..2 {
            let ti = self.tidx(kt);
            let s = &sc[kt];
            let mut rr = RoundRec {
                bids: [play.bids[kt], play.bids[kt + 2]],
                contract: s.contract,
                tricks: s.tricks,
                made: s.made,
                nil_bids: s.nil_bids,
                nil_made: s.nil_made,
                score: s.score + s.bag_pen,
                bags: s.bags,
                bag_pen: s.bag_pen,
                eng_cp: s.acc.eng_cp,
                eng_add: s.acc.eng_add,
                sig: vec![],
                pm: last_bidder(rs, kt).map(|x| rs.pmake[x]),
                pt: last_bidder(rs, kt).map(|x| rs.ptricks[x]),
            };
            let rec_ids: Vec<usize> = self.teams[ti]
                .sigils
                .iter()
                .map(|o| self.record_index(ti, o))
                .collect();
            for (k, o) in self.teams[ti].sigils.iter_mut().enumerate() {
                let f = s.acc.fires[k];
                o.fires += f as u32;
                if s.acc.used & (1 << k) != 0 {
                    o.revealed = true;
                }
                if let Some(p) = rules.teams[kt].slots[k].payoff {
                    if p.grow {
                        o.grow += s.acc.grow[k] as f64 * p.amount;
                    }
                }
                let (cp, add, x, nilp) = (s.acc.cp[k], s.acc.add[k], s.acc.x[k], s.acc.nilp[k]);
                if f > 0 || cp != 0.0 || add != 0.0 || x != 1.0 || nilp != 0.0 {
                    rr.sig.push(SigilRound {
                        i: rec_ids[k],
                        f,
                        cp,
                        add,
                        x,
                        nilp,
                    });
                }
            }
            self.recs[ti].rounds.push(rr);
            let team = &mut self.teams[ti];
            team.score += s.score + s.bag_pen;
            team.bags = s.bags_after;
            let r = crate::rules::rules();
            let inc = Income {
                base: r.base_income,
                contract: if s.made {
                    r.contract_gold * s.contract as i32
                } else {
                    0
                },
                nil: r.nil_gold * s.nil_made as i32,
            };
            team.gold += inc.base + inc.contract + inc.nil;
            income[kt] = inc;
        }
        (sc, income)
    }

    /// Index of an owned sigil in the team's record list (assigned on first sight).
    pub(crate) fn record_index(&self, ti: usize, o: &OwnedSigil) -> usize {
        let id = &self.cfg.pool.defs[o.def].id;
        self.recs[ti]
            .sigils
            .iter()
            .position(|r| &r.id == id && r.shop == o.shop && r.sold.is_none())
            .unwrap_or(usize::MAX)
    }

    pub(crate) fn sync_records(&mut self) {
        for ti in 0..2 {
            for o in self.teams[ti]
                .sigils
                .iter()
                .chain(self.teams[ti].history.iter())
            {
                let id = self.cfg.pool.defs[o.def].id.clone();
                let pos = self.recs[ti]
                    .sigils
                    .iter()
                    .position(|r| r.id == id && r.shop == o.shop);
                let rec = SigilRec {
                    id,
                    grant: o.grant,
                    shop: o.shop,
                    ratio: o.ratio,
                    amount: o.amount,
                    price: o.price,
                    sold: o.sold,
                    fires: o.fires,
                };
                match pos {
                    Some(p) => self.recs[ti].sigils[p] = rec,
                    None => self.recs[ti].sigils.push(rec),
                }
            }
        }
    }

    pub(crate) fn knowledge(
        &self,
        play: &Play,
        seat: u8,
        known: &[Mask; 4],
        voids: &[[bool; 4]; 4],
    ) -> Knowledge {
        let partner = ((seat + 2) % 4) as usize;
        // Engravings on the seat's own cards and its partner's owned cards.
        let eng_known = play.hands[seat as usize] | known[partner];
        Knowledge {
            seat,
            known: known[partner] & play.hands[partner],
            eng_known,
            voids: *voids,
            true_ids: self.cfg.ai.true_ids || crate::rules::rules().public_deck,
        }
    }

    pub(crate) fn reveal(&mut self, play: &Play, rules: &Rules) {
        for kt in 0..2 {
            let ti = self.tidx(kt);
            let used = play.acc[kt].used;
            for (k, s) in rules.teams[kt].slots.iter().enumerate() {
                let hit = used & (1 << k) != 0
                    || (matches!(
                        s.rule,
                        Some(Rule::AnySuit(_))
                            | Some(Rule::AnySuitMade)
                            | Some(Rule::AnySuitFirst(_))
                    ) && used & (1 << 14) != 0)
                    || (matches!(s.rule, Some(Rule::LeadSpades)) && used & (1 << 13) != 0)
                    || (matches!(s.rule, Some(Rule::Untrumpable(_))) && used & (1 << 12) != 0)
                    || (matches!(s.rule, Some(Rule::FreeCards(_))) && used & (1 << 11) != 0)
                    || (matches!(s.rule, Some(Rule::FirstLead))
                        && play.first_leader != (play.dealer + 1) % 4)
                    || (matches!(s.rule, Some(Rule::LeadChoice)) && used & (1 << 15) != 0);
                if hit && s.payoff.is_none_or(|p| p.trig.is_trick()) {
                    self.teams[ti].sigils[k].revealed = true;
                }
            }
        }
    }

    pub(crate) fn bench(&mut self, play: &Play, s: u8, m: u8) {
        let kt = (s % 2) as usize;
        let ti = self.tidx(kt);
        let b = &mut self.recs[ti].bench;
        let legal = play.legal();
        let would_win = |c: u8| play.would_win(c);
        if play.bids[s as usize] == 0 && play.won[s as usize] == 0 {
            b.nil_plays += 1;
            if play.tlen > 0 && would_win(m) && Bits(legal).any(|c| !would_win(c)) {
                b.nil_suicides += 1;
            }
        }
        // Overtake opportunity: partner's card is certain to win and we hold a non-beating card.
        let partner = (s + 2) % 4;
        if play.tlen >= 2 && play.bids[partner as usize] != 0 && play.tlen == 3 {
            let wi = play.winning_index();
            if play.tseat[wi] == partner && Bits(legal).any(|c| !would_win(c)) {
                b.overtake_opps += 1;
                if would_win(m) {
                    if play.id.suit[m as usize] == SPADES
                        && play.id.suit[play.trick[0] as usize] != SPADES
                    {
                        b.trump_overtakes += 1;
                    } else {
                        b.overtakes += 1;
                    }
                }
            }
        }
    }

    /// Tier-0 rollout rescoring: plays this round on sampled deals with and without each offer.
    pub(crate) fn check_offers(&mut self, ti: usize, shop: u8, offers: &[usize]) {
        const K: u64 = 12;
        let model: Vec<f64> = offers
            .iter()
            .map(|&d| self.sigil_value(ti, d, shop, None))
            .collect();
        let margin = |extra: Option<usize>, k: u64| {
            let mut r = self.clone();
            r.seed = derive(self.seed, &[999, ti as u64, shop as u64, k]);
            r.tier_override = Some(Tier::get(0));
            r.checks.clear();
            if let Some(d) = extra {
                if r.teams[ti].sigils.len() >= max_sigils() {
                    if let Some((w, _)) = r.weakest_sigil(ti, shop, None) {
                        r.teams[ti].sigils.remove(w);
                    }
                }
                r.teams[ti].sigils.push(OwnedSigil {
                    def: d,
                    amount: None,
                    ratio: 0.0,
                    shop,
                    grant: false,
                    price: 0,
                    grow: 0.0,
                    revealed: false,
                    sold: None,
                    fires: 0,
                });
                r.teams[ti].gold -= self.cfg.pool.defs[d].price();
            }
            let before = r.teams[ti].score - r.teams[1 - ti].score;
            r.play_round(shop);
            r.teams[ti].score - r.teams[1 - ti].score - before
        };
        let base: f64 = (0..K).map(|k| margin(None, k)).sum::<f64>() / K as f64;
        let rescored: Vec<f64> = offers
            .iter()
            .map(|&d| (0..K).map(|k| margin(Some(d), k)).sum::<f64>() / K as f64 - base)
            .collect();
        self.checks.push(ShopCheck {
            team: ti as u8,
            shop,
            ids: offers
                .iter()
                .map(|&d| self.cfg.pool.defs[d].id.clone())
                .collect(),
            model,
            rescored,
        });
    }

    /// Hand-level screen: team A holds `sigils` and owns `cards` (identities) for one round.
    pub fn screen_round(mut self, sigils: &[usize], cards: &[u8], round: u8) -> RoundRec {
        for &d in sigils {
            self.teams[0].sigils.push(OwnedSigil {
                def: d,
                amount: None,
                ratio: 0.0,
                shop: round,
                grant: true,
                price: 0,
                grow: 0.0,
                revealed: false,
                sold: None,
                fires: 0,
            });
        }
        for &c in cards {
            self.taken |= bit(c);
            self.teams[0].cards.push(OwnedCard {
                ident: c,
                slot: c,
                eng: ENG_NONE,
                price: 0,
                grant: true,
                shop: round,
            });
        }
        self.sync_records();
        self.play_round(round);
        self.recs[0].rounds.pop().unwrap()
    }

    /// Plans every sigil grant of the run from the grant streams and earlier grants only.
    pub(crate) fn plan_grants(&mut self) {
        for ti in 0..2 {
            let Some(g) = self.cfg.teams[ti].grants.clone() else {
                continue;
            };
            for &d in &g.fixed {
                self.teams[ti].granted.push(d);
                self.teams[ti].schedule.push((g.max_shop, d));
            }
            for shop in 1..=g.max_shop.min(self.cfg.rounds) {
                let mut grng = Rng::stream(self.seed, &[S_GRANT, ti as u64, shop as u64, 2]);
                let mut todo: Vec<usize> = vec![];
                if !g.pairs.is_empty() && grng.chance(g.pair_prob) {
                    let (a, b) = g.pairs[grng.below(g.pairs.len())];
                    todo.extend([a, b]);
                } else if grng.chance(g.prob) {
                    if let Some(d) = self.draw_grant(ti, &g, &mut grng) {
                        todo.push(d);
                    }
                }
                for d in todo {
                    if !self.teams[ti].granted.contains(&d) {
                        self.teams[ti].granted.push(d);
                        self.teams[ti].schedule.push((shop, d));
                    }
                }
            }
        }
    }

    pub fn run(mut self, board: u64) -> RunRec {
        self.plan_grants();
        for round in 1..=self.cfg.rounds {
            // Teams shop at once; card offers never overlap. Which team's offers are drawn first
            // swaps with the orientation, like the rest of the luck.
            let first = self.orient as usize;
            let offered = self.shop(first, round, 0);
            self.shop(1 - first, round, offered);
            self.sync_records();
            self.play_round(round);
        }
        self.sync_records();
        for ti in 0..2 {
            self.recs[ti].final_score = self.teams[ti].score;
            let mut cards: Vec<(String, u8, bool, u8, bool)> = vec![];
            let owned = self.teams[ti].cards.len();
            for (n, c) in self.teams[ti]
                .cards
                .iter()
                .chain(self.teams[ti].sold_cards.iter())
                .enumerate()
            {
                let mut k = card_class(nominal_suit(c.ident), nominal_rank(c.ident));
                if c.eng != ENG_NONE {
                    k = format!("{k}+{}", eng_key(c.eng));
                }
                cards.push((k, c.shop, c.grant, c.ident, n < owned));
            }
            self.recs[ti].cards = cards;
        }
        let margin = self.teams[0].score - self.teams[1].score;
        RunRec {
            board,
            orient: self.orient,
            seed: self.seed,
            teams: std::mem::take(&mut self.recs),
            win: if margin > 0.0 {
                1.0
            } else if margin < 0.0 {
                0.0
            } else {
                0.5
            },
            margin,
            clean: false,
            checks: std::mem::take(&mut self.checks),
            version: self.cfg.version.clone(),
        }
    }
}

/// One round in progress, from the deal to the last trick.
pub struct RoundState {
    pub round: u8,
    /// Identities after the deal (synthetic copies applied), before any Opening change.
    pub base_id: Identity,
    pub play: Play,
    pub rules: Rules,
    /// Owned cards each seat's partner knows about.
    pub known: [Mask; 4],
    pub voids: [[bool; 4]; 4],
    pub ai: Vec<Rng>,
    /// Opening effects in resolution order: (kernel team, sigil slot, rule).
    pub opening: Vec<(usize, usize, Rule)>,
    /// Each seat's predicted make probability for its bid (AI seats), and the bidding order.
    pub pmake: [f64; 4],
    pub ptricks: [f64; 4],
    pub bid_order: Vec<u8>,
}

impl RoundState {
    /// The seat to act: a pending lead choice, else the seat whose turn it is.
    pub fn mover(&self) -> u8 {
        if self.play.choosing >= 0 {
            self.play.choosing as u8
        } else {
            self.play.turn
        }
    }
}

/// Gold paid to a team after a round.
#[derive(Clone, Copy, Default, Debug, Serialize)]
pub struct Income {
    pub base: i32,
    pub contract: i32,
    pub nil: i32,
}

fn trim(m: Mask, n: u32) -> Mask {
    let mut out = 0;
    for c in Bits(m).take(n as usize) {
        out |= bit(c);
    }
    out
}

fn sample(m: Mask, n: u8, rng: &mut Rng) -> Vec<u8> {
    let mut v: Vec<u8> = Bits(m).collect();
    rng.shuffle(&mut v);
    v.truncate(n as usize);
    v
}

/// A card offer as text: "A♠", "A♣ (synthetic, replaces 4♣)", or "K♥ +bonus".
pub fn offer_str(ident: u8, slot: u8, eng: u8) -> String {
    let c = card_str(nominal_suit(ident), nominal_rank(ident));
    match eng {
        ENG_SYNTH => format!(
            "{c} (synthetic, replaces {})",
            card_str(nominal_suit(slot), nominal_rank(slot))
        ),
        ENG_BONUS => format!("{c} +bonus engraving"),
        ENG_HERALD => format!("{c} +herald engraving"),
        ENG_MULT => format!("{c} +multiplier engraving"),
        _ => c,
    }
}

/// The team's last AI bidder with a non-nil bid.
fn last_bidder(rs: &RoundState, kt: usize) -> Option<usize> {
    rs.bid_order
        .iter()
        .rev()
        .find(|&&x| (x as usize) % 2 == kt && !rs.pmake[x as usize].is_nan())
        .map(|&x| x as usize)
}

/// A per-round model value, extended past the model's last round for longer runs.
fn at(v: &[f64], k: usize) -> f64 {
    v.get(k).or(v.last()).copied().unwrap_or(0.0)
}

/// Cards a committed team wants for its archetype.
fn committed_card(a: &str, suit: u8, rank: u8) -> bool {
    match a {
        "Suits" => suit != SPADES && rank >= 10,
        "Spades" | "BidHigh" => suit == SPADES || rank >= 13,
        "Ranks" => rank == ACE,
        "Nil" | "LowCards" | "Exact" => rank <= 10 && suit != SPADES,
        "Rainbow" => rank >= 13,
        "Streaks" => rank >= 12,
        _ => false,
    }
}
