//! A full run: eight shops and rounds between two teams, with grants, run records, and ledgers.

use crate::ai::heuristic::swap_choice;
use crate::ai::search::*;
use crate::cards::*;
use crate::kernel::*;
use crate::model::*;
use crate::rng::*;
use crate::sigil::*;
use serde::Serialize;
use std::collections::HashMap;

pub const ENG_SYNTH: u8 = 4;
const MAX_SIGILS: usize = 7;
const MAX_CARDS: usize = 8;

// Stream labels.
const S_DEAL: u64 = 1;
const S_OFFER: u64 = 2;
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
}

#[derive(Clone, Debug)]
struct OwnedSigil {
    def: usize,
    amount: Option<f64>,
    ratio: f64,
    shop: u8,
    grant: bool,
    price: i32,
    grow: f64,
    revealed: bool,
    sold: Option<u8>,
    fires: u32,
}

#[derive(Clone, Copy, Debug)]
struct OwnedCard {
    ident: u8,
    slot: u8,
    eng: u8,
    price: i32,
    grant: bool,
    shop: u8,
}

#[derive(Clone, Copy, Debug)]
struct CardOffer {
    ident: u8,
    slot: u8,
    eng: u8,
    price: i32,
}

struct Team {
    sigils: Vec<OwnedSigil>,
    history: Vec<OwnedSigil>,
    cards: Vec<OwnedCard>,
    sold_cards: Vec<OwnedCard>,
    owner: u8,
    gold: i32,
    score: f64,
}

impl Team {
    fn active(&self) -> impl Iterator<Item = &OwnedSigil> {
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
    pub score: f64,
    pub eng_cp: f64,
    pub eng_add: f64,
    pub sig: Vec<SigilRound>,
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
    pub cards: Vec<(String, u8, bool)>,
    pub perturb: Vec<(u8, i32)>,
    pub gold: Vec<i32>,
    pub rerolls: u32,
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
    if suit == SPADES {
        sp
    } else {
        side
    }
}

fn eng_premium(e: u8) -> i32 {
    match e {
        ENG_BONUS | ENG_HERALD | ENG_SYNTH => 25,
        ENG_MULT => 50,
        _ => 0,
    }
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

fn sell_value(price: i32) -> i32 {
    (price / 2) / 5 * 5
}

pub struct Runner<'a> {
    cfg: &'a RunCfg<'a>,
    seed: u64,
    orient: u8,
    teams: [Team; 2],
    taken: Mask,
    recs: [TeamRec; 2],
    rule_cache: Vec<Compiled>,
}

impl<'a> Runner<'a> {
    pub fn new(cfg: &'a RunCfg<'a>, seed: u64, orient: u8) -> Self {
        let mk = |ti: u64| {
            let mut r = Rng::stream(seed, &[S_SETUP, ti]);
            Team {
                sigils: vec![],
                history: vec![],
                cards: vec![],
                sold_cards: vec![],
                owner: r.below(2) as u8,
                gold: 150,
                score: 0.0,
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
        }
    }

    fn kteam(&self, ti: usize) -> usize {
        ti ^ self.orient as usize
    }
    fn tidx(&self, kteam: usize) -> usize {
        kteam ^ self.orient as usize
    }

    // ----- Values -----

    fn sigil_value(&self, ti: usize, def: usize, shop: u8, exclude: Option<usize>) -> f64 {
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
            Policy::Committed(a) if d.archetypes.contains(a) => v += 0.05,
            Policy::Chaser(target) if target.contains(&def) => v += 1.0,
            _ => {}
        }
        v
    }

    fn card_value(&self, ti: usize, ident: u8, eng: u8, shop: u8) -> f64 {
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
                v += 0.02;
            }
        }
        v
    }

    fn weakest_sigil(&self, ti: usize, shop: u8, protect: Option<usize>) -> Option<(usize, f64)> {
        let team = &self.teams[ti];
        (0..team.sigils.len())
            .filter(|&k| Some(k) != protect)
            .map(|k| (k, self.sigil_value(ti, team.sigils[k].def, shop, Some(k))))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
    }

    fn weakest_card(&self, ti: usize, shop: u8) -> Option<(usize, f64)> {
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

    fn sell_sigil(&mut self, ti: usize, k: usize, shop: u8) {
        let mut o = self.teams[ti].sigils.remove(k);
        self.teams[ti].gold += sell_value(o.price);
        o.sold = Some(shop);
        self.teams[ti].history.push(o);
    }

    fn sell_card(&mut self, ti: usize, k: usize) {
        let c = self.teams[ti].cards.remove(k);
        self.teams[ti].gold += sell_value(c.price);
        self.taken &= !bit(c.slot);
        self.teams[ti].sold_cards.push(c);
    }

    // ----- Offers -----

    fn sigil_offers(&self, ti: usize, rng: &mut Rng) -> Vec<usize> {
        if !self.cfg.sigil_shop {
            return vec![];
        }
        let owned: Vec<usize> = self.teams[ti].sigils.iter().map(|o| o.def).collect();
        let mut out = vec![];
        for _ in 0..3 {
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

    fn card_offer(&self, shop: u8, rng: &mut Rng, exclude: Mask) -> Option<CardOffer> {
        let engraved_share = if shop >= 3 {
            (0.25 + 0.1 * (shop as f64 - 3.0)).min(0.75)
        } else {
            0.0
        };
        let mut eng = ENG_NONE;
        if rng.chance(engraved_share) {
            eng = [ENG_BONUS, ENG_HERALD, ENG_MULT, ENG_SYNTH][rng.weighted(&[2.0, 2.0, 1.0, 2.0])];
        }
        let free = FULL_DECK & !self.taken & !exclude;
        if eng == ENG_SYNTH {
            let suit = rng.below(4) as u8;
            let rank = 11 + rng.below(4) as u8;
            let mut low = 0;
            for r in 2..=9 {
                low |= bit(card(suit, r));
            }
            let slots = low & free;
            if slots == 0 {
                return None;
            }
            let slot = rng.pick_bit(slots);
            return Some(CardOffer {
                ident: card(suit, rank),
                slot,
                eng,
                price: card_price(suit, rank) + 25,
            });
        }
        if free == 0 {
            return None;
        }
        let c = rng.pick_bit(free);
        Some(CardOffer {
            ident: c,
            slot: c,
            eng,
            price: card_price(nominal_suit(c), nominal_rank(c)) + eng_premium(eng),
        })
    }

    fn card_offers(&self, shop: u8, rng: &mut Rng, exclude: Mask) -> Vec<CardOffer> {
        let mut out: Vec<CardOffer> = vec![];
        if !self.cfg.cards {
            return out;
        }
        let mut ex = exclude;
        for _ in 0..3 {
            if let Some(o) = self.card_offer(shop, rng, ex) {
                ex |= bit(o.slot);
                out.push(o);
            }
        }
        out
    }

    // ----- Grants -----

    fn draw_grant(&self, ti: usize, g: &GrantCfg, rng: &mut Rng) -> Option<usize> {
        let owned: Vec<usize> = self.teams[ti].sigils.iter().map(|o| o.def).collect();
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
            for o in &self.teams[ti].sigils {
                for t in &self.cfg.pool.defs[o.def].archetypes {
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

    fn grant(&mut self, ti: usize, def: usize, shop: u8, rng: &mut Rng, jitter: bool) {
        if self.teams[ti].sigils.len() >= MAX_SIGILS {
            if let Some((k, _)) = self.weakest_sigil(ti, shop, None) {
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
        self.teams[ti].gold -= price;
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

    fn shop(&mut self, ti: usize, shop: u8, other_offers: Mask) -> Mask {
        let mut rng = Rng::stream(self.seed, &[S_TEAM, ti as u64, shop as u64]);
        let mut grng = Rng::stream(self.seed, &[S_GRANT, ti as u64, shop as u64]);
        self.recs[ti].gold.push(self.teams[ti].gold);
        // Gold perturbations, sigil grants, and card grants come first.
        if self.cfg.perturb > 0.0 && grng.chance(self.cfg.perturb) {
            let amt = if grng.chance(0.5) {
                self.cfg.perturb_amount
            } else {
                -self.cfg.perturb_amount
            };
            self.teams[ti].gold += amt;
            self.recs[ti].perturb.push((shop, amt));
        }
        if let Some(g) = self.cfg.teams[ti].grants.clone() {
            if shop <= g.max_shop {
                if !g.pairs.is_empty() && grng.chance(g.pair_prob) {
                    let (a, b) = g.pairs[grng.below(g.pairs.len())];
                    for d in [a, b] {
                        if !self.teams[ti].sigils.iter().any(|o| o.def == d) {
                            self.grant(ti, d, shop, &mut grng, g.jitter);
                        }
                    }
                } else if grng.chance(g.prob) {
                    if let Some(d) = self.draw_grant(ti, &g, &mut grng) {
                        self.grant(ti, d, shop, &mut grng, g.jitter);
                    }
                }
                if shop <= 7 && grng.chance(g.card_prob) {
                    if let Some(o) = self.card_offer(shop, &mut grng, other_offers) {
                        if self.teams[ti].cards.len() >= MAX_CARDS {
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

        let lambda = self.cfg.model.lambda[shop as usize];
        let mut reroll = 0u64;
        let mut orng = Rng::stream(self.seed, &[S_OFFER, ti as u64, shop as u64, reroll]);
        let mut sig = self.sigil_offers(ti, &mut orng);
        let mut cards = self.card_offers(shop, &mut orng, other_offers);
        for _ in 0..30 {
            // (net gain, is_sigil, offer index)
            let mut opts: Vec<(f64, bool, usize)> = vec![];
            for (i, &d) in sig.iter().enumerate() {
                let price = self.cfg.pool.defs[d].price();
                if self.teams[ti].gold < price {
                    continue;
                }
                let v = self.sigil_value(ti, d, shop, None);
                let net = if self.teams[ti].sigils.len() < MAX_SIGILS {
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
                let net = if self.teams[ti].cards.len() < MAX_CARDS {
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
                    if choice.1 {
                        let d = sig.remove(choice.2);
                        if self.teams[ti].sigils.len() >= MAX_SIGILS {
                            if let Some((k, _)) = self.weakest_sigil(ti, shop, None) {
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
                        if self.teams[ti].cards.len() >= MAX_CARDS {
                            if let Some((k, _)) = self.weakest_card(ti, shop) {
                                self.sell_card(ti, k);
                            }
                        }
                        self.buy_card(ti, o, shop, false);
                    }
                }
                _ => {
                    let cost = 50 + 10 * reroll as i32;
                    if reroll == 0 && self.teams[ti].gold - cost >= 100 {
                        self.teams[ti].gold -= cost;
                        reroll += 1;
                        self.recs[ti].rerolls += 1;
                        orng = Rng::stream(self.seed, &[S_OFFER, ti as u64, shop as u64, reroll]);
                        sig = self.sigil_offers(ti, &mut orng);
                        cards = self.card_offers(shop, &mut orng, other_offers);
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

    fn buy_card(&mut self, ti: usize, o: CardOffer, shop: u8, grant: bool) {
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

    fn rules_for(&self) -> Rules {
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

    fn view_rules(&self, rules: &Rules, kt: usize) -> Rules {
        let opp = 1 - kt;
        let revealed: Vec<bool> = self.teams[self.tidx(opp)]
            .sigils
            .iter()
            .map(|o| o.revealed)
            .collect();
        let mut teams = [rules.teams[0].clone(), rules.teams[1].clone()];
        teams[opp] = rules.teams[opp].visible(&revealed);
        Rules { teams }
    }

    fn utility(&self, kt: usize, round: u8) -> Utility {
        let m = self.cfg.model;
        let ti = self.tidx(kt);
        let margin = self.teams[ti].score - self.teams[1 - ti].score;
        let left = (self.cfg.rounds - round) as usize;
        let future: f64 = ((round + 1)..=self.cfg.rounds)
            .map(|k| m.round_scale[k as usize])
            .sum();
        let nfut = left as f64;
        Utility {
            margin,
            scale: m.w_scale[left.min(8)],
            growth: [8.0 * nfut, future / 15.0, future, 2.0 * nfut],
            nil_handicap: m.nil_handicap,
        }
    }

    fn play_round(&mut self, round: u8) {
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
        let mut seat = (dealer + 1) % 4;
        for &c in &perm {
            if owned_mask & bit(c) != 0 {
                continue;
            }
            let mut guard = 0;
            while hands[seat as usize].count_ones() >= 13 && guard < 4 {
                seat = (seat + 1) % 4;
                guard += 1;
            }
            hands[seat as usize] |= bit(c);
            seat = (seat + 1) % 4;
        }

        let rules = self.rules_for();
        let mut play = Play::new(id, hands, eng, dealer, &rules);

        // Opening, in the kernel's fixed priority.
        for kt in 0..2 {
            let ti = self.tidx(kt);
            let mut order: Vec<(u8, usize, Rule)> = vec![];
            for (k, s) in rules.teams[kt].slots.iter().enumerate() {
                if let Some(r) = s.rule {
                    let pri = match r {
                        Rule::Swap(_) => 0,
                        Rule::BecomeSuit(..) => 1,
                        Rule::BecomeRank(..) => 2,
                        Rule::BecomeRange(..) => 3,
                        Rule::Raise(_) => 4,
                        _ => continue,
                    };
                    order.push((pri, k, r));
                }
            }
            order.sort_by_key(|x| (x.0, x.1));
            for (_, k, r) in order {
                let sid = hash_str(&self.cfg.pool.defs[self.teams[ti].sigils[k].def].id);
                let mut orng = Rng::stream(self.seed, &[S_OPENING, round as u64, ti as u64, sid]);
                let (a, b) = (kt, kt + 2);
                match r {
                    Rule::Swap(n) => {
                        let ga = swap_choice(&play, a as u8, n);
                        let gb = swap_choice(&play, b as u8, n);
                        let n = ga.count_ones().min(gb.count_ones());
                        let (ga, gb) = (trim(ga, n), trim(gb, n));
                        play.hands[a] = (play.hands[a] & !ga) | gb;
                        play.hands[b] = (play.hands[b] & !gb) | ga;
                        known[a] = (known[a] & !ga) | (known[b] & gb);
                        known[b] = (known[b] & !gb) | (known[a] & ga);
                    }
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
                    _ => {}
                }
                self.teams[ti].sigils[k].revealed = true;
            }
        }

        // Bidding.
        let mut voids = [[false; 4]; 4];
        let views = [self.view_rules(&rules, 0), self.view_rules(&rules, 1)];
        let mut ai: Vec<Rng> = (0..4)
            .map(|s| Rng::stream(self.seed, &[S_AI, round as u64, s as u64]))
            .collect();
        for k in 0..4 {
            let s = ((dealer + 1 + k) % 4) as usize;
            let kt = s % 2;
            let tier = self.cfg.tiers[self.tidx(kt)];
            let know = self.knowledge(&play, s as u8, &known, &voids);
            let u = self.utility(kt, round);
            let b = choose_bid(&play, &views[kt], &know, &u, &tier, &mut ai[s]);
            play.bids[s] = b;
        }
        play.post_bid(&rules);

        // Play.
        while !play.done() {
            let s = if play.choosing >= 0 {
                play.choosing as u8
            } else {
                play.turn
            };
            let kt = (s % 2) as usize;
            let tier = self.cfg.tiers[self.tidx(kt)];
            let views = [self.view_rules(&rules, 0), self.view_rules(&rules, 1)];
            let know = self.knowledge(&play, s, &known, &voids);
            let u = self.utility(kt, round);
            let m = choose_move(&play, &views[kt], &know, &u, &tier, &mut ai[s as usize]);
            if m < 52 {
                self.bench(&play, s, m);
                if play.tlen > 0 {
                    let led = play.id.suit[play.trick[0] as usize];
                    let n = play.any_suit[kt];
                    let any = n > 0 && play.ntricks + n >= 13;
                    if play.id.suit[m as usize] != led && !any {
                        voids[s as usize][led as usize] = true;
                    }
                }
            }
            if play.apply(m, &rules).is_some() {
                self.reveal(&play, &rules);
            }
        }

        // Score, ledger, growth, income.
        let sc = play.score(&rules);
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
                score: s.score,
                eng_cp: s.acc.eng_cp,
                eng_add: s.acc.eng_add,
                sig: vec![],
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
            team.score += s.score;
            let interest = if team.gold > 0 {
                (10 * (team.gold / 50)).min(50)
            } else {
                0
            };
            team.gold += interest
                + 100
                + if s.made { 10 * s.contract as i32 } else { 0 }
                + 50 * s.nil_made as i32;
        }
    }

    /// Index of an owned sigil in the team's record list (assigned on first sight).
    fn record_index(&self, ti: usize, o: &OwnedSigil) -> usize {
        let id = &self.cfg.pool.defs[o.def].id;
        self.recs[ti]
            .sigils
            .iter()
            .position(|r| &r.id == id && r.shop == o.shop && r.sold.is_none())
            .unwrap_or(usize::MAX)
    }

    fn sync_records(&mut self) {
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

    fn knowledge(
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
        }
    }

    fn reveal(&mut self, play: &Play, rules: &Rules) {
        for kt in 0..2 {
            let ti = self.tidx(kt);
            let used = play.acc[kt].used;
            for (k, s) in rules.teams[kt].slots.iter().enumerate() {
                let hit = used & (1 << k) != 0
                    || (matches!(s.rule, Some(Rule::AnySuit(_))) && used & (1 << 14) != 0)
                    || (matches!(s.rule, Some(Rule::LeadChoice)) && used & (1 << 15) != 0);
                if hit && s.payoff.is_none_or(|p| p.trig.is_trick()) {
                    self.teams[ti].sigils[k].revealed = true;
                }
            }
        }
    }

    fn bench(&mut self, play: &Play, s: u8, m: u8) {
        let kt = (s % 2) as usize;
        let ti = self.tidx(kt);
        let b = &mut self.recs[ti].bench;
        let legal = play.legal();
        let would_win = |c: u8| play.tlen == 0 || play.beats(c, play.trick[play.winning_index()]);
        if play.bids[s as usize] == 0 {
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

    pub fn run(mut self, board: u64) -> RunRec {
        for round in 1..=self.cfg.rounds {
            // Teams shop at once; card offers never overlap.
            let offered_a = self.shop(0, round, 0);
            self.shop(1, round, offered_a);
            self.sync_records();
            self.play_round(round);
        }
        self.sync_records();
        for ti in 0..2 {
            self.recs[ti].final_score = self.teams[ti].score;
            let mut cards: Vec<(String, u8, bool)> = vec![];
            for c in self.teams[ti]
                .cards
                .iter()
                .chain(self.teams[ti].sold_cards.iter())
            {
                let mut k = card_class(nominal_suit(c.ident), nominal_rank(c.ident));
                if c.eng != ENG_NONE {
                    k = format!("{k}+{}", eng_key(c.eng));
                }
                cards.push((k, c.shop, c.grant));
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
        }
    }
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

/// Cards a committed team wants for its archetype.
fn committed_card(a: &str, suit: u8, rank: u8) -> bool {
    match a {
        "Suits" => suit == DIAMONDS,
        "Spades" | "BidHigh" => suit == SPADES || rank >= 13,
        "Ranks" => rank == ACE,
        "Nil" | "LowCards" | "Exact" => rank <= 10 && suit != SPADES,
        "Rainbow" => rank >= 13,
        "Streaks" => rank >= 12,
        _ => false,
    }
}
