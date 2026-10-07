//! The rules kernel: one round of play, trick resolution, sigil triggers, and scoring with a
//! per-sigil ledger. `Play` is a small `Copy` struct, so search copies it instead of allocating.

use crate::cards::*;
use crate::sigil::*;

pub const MAXP: usize = 8;
/// Move ids for the lead-choice decision: keep the lead, or pass it to the partner.
pub const MOVE_KEEP: u8 = 52;
pub const MOVE_PASS: u8 = 53;

pub const ENG_NONE: u8 = 0;
pub const ENG_BONUS: u8 = 1;
pub const ENG_HERALD: u8 = 2;
pub const ENG_MULT: u8 = 3;
pub const ENG_BONUS_POINTS: f64 = 20.0;
pub const ENG_HERALD_POINTS: f64 = 20.0;
pub const ENG_MULT_AMOUNT: f64 = 5.0;
pub const BASE_MULT: f64 = 10.0;
pub const NIL_VALUE: f64 = 100.0;

#[derive(Clone, Debug)]
pub struct Slot {
    pub payoff: Option<Payoff>,
    pub rule: Option<Rule>,
    /// Growth accumulated so far this run (the stored value, before this round).
    pub grow_value: f64,
}

/// One team's sigils, compiled.
#[derive(Clone, Debug, Default)]
pub struct TeamProgram {
    pub slots: Vec<Slot>,
    pub trick: Vec<(usize, Payoff)>,
    pub post_bid: Vec<(usize, Payoff)>,
    pub result: Vec<(usize, Payoff)>,
    pub growth: Vec<(usize, Payoff)>,
    pub any_suit: u8,
    pub lead_choice: bool,
}

impl TeamProgram {
    pub fn new(slots: Vec<Slot>) -> Self {
        let mut p = TeamProgram {
            slots,
            ..Default::default()
        };
        for (i, s) in p.slots.iter().enumerate() {
            if let Some(po) = s.payoff {
                if po.grow {
                    p.growth.push((i, po));
                }
                match po.trig {
                    t if t.is_trick() => p.trick.push((i, po)),
                    Trig::Always | Trig::Hold { .. } | Trig::Bid { .. } => p.post_bid.push((i, po)),
                    _ => p.result.push((i, po)),
                }
            }
            match s.rule {
                Some(Rule::AnySuit(n)) => p.any_suit = p.any_suit.max(n),
                Some(Rule::LeadChoice) => p.lead_choice = true,
                _ => {}
            }
        }
        p
    }
    /// The subset of slots an observer can see.
    pub fn visible(&self, revealed: &[bool]) -> TeamProgram {
        let slots = self
            .slots
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if revealed.get(i).copied().unwrap_or(false) {
                    s.clone()
                } else {
                    Slot {
                        payoff: None,
                        rule: None,
                        grow_value: 0.0,
                    }
                }
            })
            .collect();
        TeamProgram::new(slots)
    }
}

pub struct Rules {
    pub teams: [TeamProgram; 2],
}

/// One team's scoring accumulators for the round, per sigil slot.
#[derive(Clone, Copy, Debug)]
pub struct TeamAcc {
    pub cp: [f64; MAXP],
    pub add: [f64; MAXP],
    pub x: [f64; MAXP],
    pub nilp: [f64; MAXP],
    pub fires: [u8; MAXP],
    pub grow: [u8; MAXP],
    pub cnt: [u8; MAXP],
    pub done: u16,
    pub eng_cp: f64,
    pub eng_add: f64,
    pub win_suits: u8,
    pub lead_suits: u8,
    pub run: u8,
    pub won_prev: bool,
    /// A rule hook or payoff fired (for reveals), per slot.
    pub used: u16,
}

impl Default for TeamAcc {
    fn default() -> Self {
        TeamAcc {
            cp: [0.0; MAXP],
            add: [0.0; MAXP],
            x: [1.0; MAXP],
            nilp: [0.0; MAXP],
            fires: [0; MAXP],
            grow: [0; MAXP],
            cnt: [0; MAXP],
            done: 0,
            eng_cp: 0.0,
            eng_add: 0.0,
            win_suits: 0,
            lead_suits: 0,
            run: 0,
            won_prev: false,
            used: 0,
        }
    }
}

impl TeamAcc {
    #[inline]
    fn fire(&mut self, slot: usize, p: &Payoff, times: u32, contract: u8) {
        if times == 0 {
            return;
        }
        self.fires[slot] = self.fires[slot].saturating_add(times as u8);
        self.used |= 1 << slot;
        if p.grow {
            self.grow[slot] = self.grow[slot].saturating_add(times as u8);
            return;
        }
        let amt = if p.per_contract {
            p.amount * contract as f64
        } else {
            p.amount
        };
        match p.kind {
            Kind::Points => self.cp[slot] += amt * times as f64,
            Kind::Mult => self.add[slot] += amt * times as f64,
            Kind::XMult => self.x[slot] *= amt.powi(times as i32),
            Kind::NilPoints => self.nilp[slot] += amt * times as f64,
        }
    }
    /// Adds a growth sigil's stored value at the start of the round.
    fn grow_base(&mut self, slot: usize, p: &Payoff, value: f64) {
        match p.kind {
            Kind::Points => self.cp[slot] += value,
            Kind::Mult => self.add[slot] += value,
            Kind::XMult => self.x[slot] *= 1.0 + value,
            Kind::NilPoints => self.nilp[slot] += value,
        }
    }
}

/// The state of one round from the deal to the last trick.
#[derive(Clone, Copy)]
pub struct Play {
    pub id: Identity,
    pub eng: [u8; 52],
    pub hands: [Mask; 4],
    pub played: Mask,
    /// -1 before bidding; 0 is nil.
    pub bids: [i8; 4],
    pub won: [u8; 4],
    pub trick: [u8; 4],
    pub tseat: [u8; 4],
    pub tlen: u8,
    pub leader: u8,
    pub turn: u8,
    pub broken: bool,
    pub ntricks: u8,
    pub dealer: u8,
    /// The seat choosing who leads next, if a lead choice is pending.
    pub choosing: i8,
    pub any_suit: [u8; 2],
    pub lead_choice: [bool; 2],
    pub acc: [TeamAcc; 2],
}

pub struct TrickResult {
    pub winner: u8,
    pub cards: [u8; 4],
    pub seats: [u8; 4],
}

impl Play {
    pub fn new(id: Identity, hands: [Mask; 4], eng: [u8; 52], dealer: u8, rules: &Rules) -> Play {
        let first = (dealer + 1) % 4;
        Play {
            id,
            eng,
            hands,
            played: 0,
            bids: [-1; 4],
            won: [0; 4],
            trick: [0; 4],
            tseat: [0; 4],
            tlen: 0,
            leader: first,
            turn: first,
            broken: false,
            ntricks: 0,
            dealer,
            choosing: -1,
            any_suit: [rules.teams[0].any_suit, rules.teams[1].any_suit],
            lead_choice: [rules.teams[0].lead_choice, rules.teams[1].lead_choice],
            acc: [TeamAcc::default(), TeamAcc::default()],
        }
    }

    pub fn done(&self) -> bool {
        self.ntricks >= 13
    }

    pub fn contract(&self, team: usize) -> u8 {
        let mut b = 0;
        for s in [team, team + 2] {
            if self.bids[s] > 0 {
                b += self.bids[s] as u8;
            }
        }
        b
    }
    /// Tricks won by the team's non-nil bidders.
    pub fn contract_tricks(&self, team: usize) -> u8 {
        let mut t = 0;
        for s in [team, team + 2] {
            if self.bids[s] != 0 {
                t += self.won[s];
            }
        }
        t
    }

    #[inline]
    fn any_suit_now(&self, seat: u8) -> bool {
        let n = self.any_suit[(seat % 2) as usize];
        n > 0 && self.ntricks + n >= 13
    }

    /// Legal moves as a mask; bits 52 and 53 are the lead-choice moves.
    pub fn legal(&self) -> Mask {
        if self.choosing >= 0 {
            return bit(MOVE_KEEP) | bit(MOVE_PASS);
        }
        let hand = self.hands[self.turn as usize];
        if self.tlen == 0 {
            if self.broken {
                return hand;
            }
            let non = hand & !self.id.suit_mask[SPADES as usize];
            return if non != 0 { non } else { hand };
        }
        if self.any_suit_now(self.turn) {
            return hand;
        }
        let led = self.id.suit[self.trick[0] as usize];
        let follow = hand & self.id.suit_mask[led as usize];
        if follow != 0 {
            follow
        } else {
            hand
        }
    }

    /// Whether `c` would beat `b` given the led suit (ties go to the card played first).
    #[inline]
    pub fn beats(&self, c: u8, b: u8) -> bool {
        let (cs, bs) = (self.id.suit[c as usize], self.id.suit[b as usize]);
        if cs == bs {
            self.id.rank[c as usize] > self.id.rank[b as usize]
        } else {
            cs == SPADES
        }
    }

    /// Index in the current trick of the card currently winning it.
    #[inline]
    pub fn winning_index(&self) -> usize {
        let mut best = 0;
        for i in 1..self.tlen as usize {
            if self.beats(self.trick[i], self.trick[best]) {
                best = i;
            }
        }
        best
    }

    /// Applies a move. Returns the completed trick when this move ended one.
    pub fn apply(&mut self, m: u8, rules: &Rules) -> Option<TrickResult> {
        if m >= 52 {
            let s = self.choosing as u8;
            let lead = if m == MOVE_KEEP { s } else { (s + 2) % 4 };
            if lead != s {
                self.acc[(s % 2) as usize].used |= 1 << 15;
            }
            self.leader = lead;
            self.turn = lead;
            self.choosing = -1;
            return None;
        }
        let seat = self.turn;
        self.hands[seat as usize] &= !bit(m);
        self.played |= bit(m);
        if self.tlen > 0 && self.any_suit_now(seat) {
            let led = self.id.suit[self.trick[0] as usize];
            if self.id.suit[m as usize] != led
                && self.hands[seat as usize] & self.id.suit_mask[led as usize] != 0
            {
                self.acc[(seat % 2) as usize].used |= 1 << 14;
            }
        }
        if self.id.suit[m as usize] == SPADES {
            self.broken = true;
        }
        self.trick[self.tlen as usize] = m;
        self.tseat[self.tlen as usize] = seat;
        self.tlen += 1;
        if self.tlen < 4 {
            self.turn = (seat + 1) % 4;
            return None;
        }
        Some(self.finish_trick(rules))
    }

    fn finish_trick(&mut self, rules: &Rules) -> TrickResult {
        let wi = self.winning_index();
        let wseat = self.tseat[wi];
        let wcard = self.trick[wi];
        let wteam = (wseat % 2) as usize;
        let led_card = self.trick[0];
        let led = self.id.suit[led_card as usize];
        let lteam = (self.tseat[0] % 2) as usize;
        self.won[wseat as usize] += 1;
        let team_win = self.bids[wseat as usize] != 0;
        let ws = self.id.suit[wcard as usize];
        let wr = self.id.rank[wcard as usize];
        let by_trump = ws == SPADES && led != SPADES;
        let last = self.ntricks == 12;
        let first = self.ntricks == 0;
        let contract = [self.contract(0), self.contract(1)];

        // Engravings: the winning card's and the lead's.
        match self.eng[wcard as usize] {
            ENG_BONUS if team_win => self.acc[wteam].eng_cp += ENG_BONUS_POINTS,
            ENG_MULT if team_win => self.acc[wteam].eng_add += ENG_MULT_AMOUNT,
            _ => {}
        }
        if self.eng[led_card as usize] == ENG_HERALD {
            self.acc[lteam].eng_cp += ENG_HERALD_POINTS;
        }

        // Lead triggers.
        {
            let ls = self.id.suit[led_card as usize];
            let lr = self.id.rank[led_card as usize];
            let acc = &mut self.acc[lteam];
            let before = acc.lead_suits;
            acc.lead_suits |= 1 << ls;
            for &(slot, p) in &rules.teams[lteam].trick {
                match p.trig {
                    Trig::Lead { filt, count } if filt.matches(ls, lr) => {
                        if count == 0 {
                            acc.fire(slot, &p, 1, contract[lteam]);
                        } else if acc.done & (1 << slot) == 0 {
                            acc.cnt[slot] += 1;
                            if acc.cnt[slot] >= count {
                                acc.done |= 1 << slot;
                                acc.fire(slot, &p, 1, contract[lteam]);
                            }
                        }
                    }
                    Trig::Suits { lead: true, count }
                        if before != acc.lead_suits
                            && acc.lead_suits.count_ones() as u8 >= count
                            && acc.done & (1 << slot) == 0 =>
                    {
                        acc.done |= 1 << slot;
                        acc.fire(slot, &p, 1, contract[lteam]);
                    }
                    _ => {}
                }
            }
        }

        // Win triggers.
        if team_win {
            let acc = &mut self.acc[wteam];
            let prev = acc.won_prev;
            acc.run += 1;
            let new_suit = acc.win_suits & (1 << ws) == 0;
            acc.win_suits |= 1 << ws;
            for &(slot, p) in &rules.teams[wteam].trick {
                match p.trig {
                    Trig::Win {
                        filt,
                        by_trump: bt,
                        pos,
                        consecutive,
                        in_row,
                        distinct_suit,
                        count,
                    } => {
                        if !filt.matches(ws, wr) || (bt && !by_trump) {
                            continue;
                        }
                        match pos {
                            TrickPos::Last if !last => continue,
                            TrickPos::First if !first => continue,
                            _ => {}
                        }
                        if consecutive && !prev {
                            continue;
                        }
                        if distinct_suit && !new_suit {
                            continue;
                        }
                        if in_row > 0 {
                            if acc.run >= in_row && acc.done & (1 << slot) == 0 {
                                acc.done |= 1 << slot;
                                acc.fire(slot, &p, 1, contract[wteam]);
                            }
                            continue;
                        }
                        if count > 0 {
                            if acc.done & (1 << slot) == 0 {
                                acc.cnt[slot] += 1;
                                if acc.cnt[slot] >= count {
                                    acc.done |= 1 << slot;
                                    acc.fire(slot, &p, 1, contract[wteam]);
                                }
                            }
                            continue;
                        }
                        acc.fire(slot, &p, 1, contract[wteam]);
                    }
                    Trig::Suits { lead: false, count }
                        if new_suit
                            && acc.win_suits.count_ones() as u8 >= count
                            && acc.done & (1 << slot) == 0 =>
                    {
                        acc.done |= 1 << slot;
                        acc.fire(slot, &p, 1, contract[wteam]);
                    }
                    _ => {}
                }
            }
            acc.won_prev = true;
            let other = &mut self.acc[1 - wteam];
            other.won_prev = false;
            other.run = 0;
        } else {
            for a in self.acc.iter_mut() {
                a.won_prev = false;
                a.run = 0;
            }
        }

        let result = TrickResult {
            winner: wseat,
            cards: self.trick,
            seats: self.tseat,
        };
        self.ntricks += 1;
        self.tlen = 0;
        self.leader = wseat;
        self.turn = wseat;
        if self.lead_choice[wteam] && team_win && self.ntricks < 13 {
            self.choosing = wseat as i8;
        }
        result
    }

    /// Fires the after-bidding triggers: always-on, holdings, bids, and growth bases.
    pub fn post_bid(&mut self, rules: &Rules) {
        for team in 0..2 {
            let contract = self.contract(team);
            let held = self.hands[team] | self.hands[team + 2];
            let nil_bidders = [team, team + 2]
                .iter()
                .filter(|&&s| self.bids[s] == 0)
                .count() as u32;
            let prog = &rules.teams[team];
            let acc = &mut self.acc[team];
            for &(slot, p) in &prog.growth {
                acc.grow_base(slot, &p, prog.slots[slot].grow_value);
            }
            for &(slot, p) in &prog.post_bid {
                let times = match p.trig {
                    Trig::Always => 1,
                    Trig::Hold {
                        filt,
                        count,
                        beyond,
                    } => {
                        let n = Bits(held)
                            .filter(|&c| {
                                filt.matches(self.id.suit[c as usize], self.id.rank[c as usize])
                            })
                            .count() as u32;
                        if count > 0 {
                            (n >= count as u32) as u32
                        } else {
                            n.saturating_sub(beyond as u32)
                        }
                    }
                    Trig::Bid { min, max, nil } => {
                        if nil {
                            nil_bidders
                        } else {
                            (contract > 0
                                && (min == 0 || contract >= min)
                                && (max == 0 || contract <= max)) as u32
                        }
                    }
                    _ => 0,
                };
                if p.grow {
                    if times > 0 {
                        acc.grow[slot] = acc.grow[slot].saturating_add(times as u8);
                        acc.fires[slot] = acc.fires[slot].saturating_add(times as u8);
                    }
                } else {
                    acc.fire(slot, &p, times, contract);
                }
            }
        }
    }

    /// Scores both teams. Only slots in `rules` count, so an observer's view scores what it sees.
    pub fn score(&self, rules: &Rules) -> [TeamScore; 2] {
        let mut out = [TeamScore::default(), TeamScore::default()];
        let sets = [self.is_set(0), self.is_set(1)];
        for team in 0..2 {
            out[team] = self.score_team(team, rules, sets[1 - team]);
        }
        out
    }

    fn is_set(&self, team: usize) -> bool {
        let b = self.contract(team);
        b > 0 && self.contract_tricks(team) < b
    }

    pub fn score_team(&self, team: usize, rules: &Rules, opp_set: bool) -> TeamScore {
        let prog = &rules.teams[team];
        let mut acc = self.acc[team];
        let b = self.contract(team);
        let t = self.contract_tricks(team);
        let made = b > 0 && t >= b;
        let exact = made && t == b;
        let mut nil_made = 0u32;
        let mut nil_bids = 0;
        for s in [team, team + 2] {
            if self.bids[s] == 0 {
                nil_bids += 1;
                if self.won[s] == 0 {
                    nil_made += 1;
                }
            }
        }
        for &(slot, p) in &prog.result {
            let times = match p.trig {
                Trig::Make { min, exact: ex } => {
                    (made && (!ex || exact) && (min == 0 || b >= min)) as u32
                }
                Trig::NilMade => (nil_made > 0) as u32,
                Trig::OpponentsSet => opp_set as u32,
                _ => 0,
            };
            acc.fire(slot, &p, times, b);
        }
        let mut cp = acc.eng_cp;
        let mut add = acc.eng_add;
        let mut x = 1.0;
        let mut nilp = 0.0;
        for (slot, s) in prog.slots.iter().enumerate() {
            if s.payoff.is_some() {
                cp += acc.cp[slot];
                add += acc.add[slot];
                x *= acc.x[slot];
                nilp += acc.nilp[slot];
            }
        }
        let nil_score =
            nil_made as f64 * (NIL_VALUE + nilp) - (nil_bids - nil_made as i32) as f64 * NIL_VALUE;
        let base = if b == 0 {
            0.0
        } else if made {
            10.0 * b as f64 + cp
        } else {
            -10.0 * b as f64
        };
        let mult = (BASE_MULT + add) * x;
        TeamScore {
            score: ((base + nil_score) * mult).round(),
            contract: b,
            tricks: t,
            made,
            exact,
            set: b > 0 && !made,
            nil_bids: nil_bids as u8,
            nil_made: nil_made as u8,
            cp: if made { cp } else { 0.0 },
            add,
            x,
            nil_score,
            base,
            acc,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct TeamScore {
    pub score: f64,
    pub contract: u8,
    pub tricks: u8,
    pub made: bool,
    pub exact: bool,
    pub set: bool,
    pub nil_bids: u8,
    pub nil_made: u8,
    pub cp: f64,
    pub add: f64,
    pub x: f64,
    pub nil_score: f64,
    pub base: f64,
    /// The accumulators after result triggers, for the ledger.
    pub acc: TeamAcc,
}

/// Recomputes a round score with some sigil slots removed, from the ledger (no replay).
pub fn rescore_without(s: &TeamScore, prog: &TeamProgram, drop: &[usize]) -> f64 {
    let acc = &s.acc;
    let mut cp = acc.eng_cp;
    let mut add = acc.eng_add;
    let mut x = 1.0;
    let mut nilp = 0.0;
    for (slot, sl) in prog.slots.iter().enumerate() {
        if sl.payoff.is_some() && !drop.contains(&slot) {
            cp += acc.cp[slot];
            add += acc.add[slot];
            x *= acc.x[slot];
            nilp += acc.nilp[slot];
        }
    }
    let nil_failed = s.nil_bids - s.nil_made;
    let nil_score = s.nil_made as f64 * (NIL_VALUE + nilp) - nil_failed as f64 * NIL_VALUE;
    let base = if s.contract == 0 {
        0.0
    } else if s.made {
        10.0 * s.contract as f64 + cp
    } else {
        -10.0 * s.contract as f64
    };
    ((base + nil_score) * (BASE_MULT + add) * x).round()
}
