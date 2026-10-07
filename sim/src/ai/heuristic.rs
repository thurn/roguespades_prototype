//! 1.0's rollout policy and hand evaluation, ported to masks.

use crate::cards::*;
use crate::kernel::*;
use crate::rng::Rng;

pub const EPSILON: f64 = 0.08;

#[inline]
pub fn lowest(p: &Play, m: Mask) -> u8 {
    let mut best = 255u8;
    let mut bp = 255u8;
    for c in Bits(m) {
        let v = p.id.power(c);
        if v < bp {
            bp = v;
            best = c;
        }
    }
    best
}
#[inline]
pub fn highest(p: &Play, m: Mask) -> u8 {
    let mut best = 255u8;
    let mut bp = 0u8;
    for c in Bits(m) {
        let v = p.id.power(c);
        if v >= bp {
            bp = v;
            best = c;
        }
    }
    best
}

/// No other hand holds a higher card of this card's suit.
#[inline]
pub fn is_boss(p: &Play, c: u8, seat: u8) -> bool {
    let s = p.id.suit[c as usize];
    let r = p.id.rank[c as usize];
    let mut others = 0;
    for x in 0..4 {
        if x != seat as usize {
            others |= p.hands[x];
        }
    }
    for o in Bits(others & p.id.suit_mask[s as usize]) {
        if p.id.rank[o as usize] > r {
            return false;
        }
    }
    true
}

#[inline]
fn beats_now(p: &Play, c: u8) -> bool {
    if p.tlen == 0 {
        return true;
    }
    p.beats(c, p.trick[p.winning_index()])
}

fn need(p: &Play, team: usize) -> i32 {
    p.contract(team) as i32 - p.contract_tricks(team) as i32
}

/// Seats still to play in the current trick after `seat`.
fn later_seats(p: &Play, seat: u8) -> impl Iterator<Item = u8> {
    let remaining = 3 - p.tlen as i32;
    (1..=remaining.max(0)).map(move |k| (seat + k as u8) % 4)
}

fn later_can_trump(p: &Play, seat: u8) -> bool {
    let led = p.id.suit[p.trick[0] as usize];
    if led == SPADES {
        return false;
    }
    later_seats(p, seat).any(|s| {
        let h = p.hands[s as usize];
        h & p.id.suit_mask[led as usize] == 0 && h & p.id.suit_mask[SPADES as usize] != 0
    })
}

/// Partner keeps the lead when it holds more off-suit winners, unless it bid nil.
pub fn lead_choice(p: &Play) -> u8 {
    let s = p.choosing as u8;
    let partner = (s + 2) % 4;
    if p.bids[partner as usize] == 0 {
        return MOVE_KEEP;
    }
    let bosses = |seat: u8| {
        Bits(p.hands[seat as usize] & !p.id.suit_mask[SPADES as usize])
            .filter(|&c| is_boss(p, c, seat))
            .count()
    };
    if bosses(partner) > bosses(s) {
        MOVE_PASS
    } else {
        MOVE_KEEP
    }
}

/// The rollout heuristic; with no rng it is deterministic (no exploration).
pub fn policy(p: &Play, rng: Option<&mut Rng>) -> u8 {
    if p.choosing >= 0 {
        return lead_choice(p);
    }
    let legal = p.legal();
    if legal.count_ones() == 1 {
        return legal.trailing_zeros() as u8;
    }
    if let Some(r) = rng {
        if r.chance(EPSILON) {
            return r.pick_bit(legal);
        }
    }
    let seat = p.turn;
    let partner = (seat + 2) % 4;
    let team = (seat % 2) as usize;
    let nil = p.bids[seat as usize] == 0;
    let cover = p.bids[partner as usize] == 0 && p.won[partner as usize] == 0;
    let remaining = 13 - p.ntricks as i32;
    let ours = need(p, team);
    let theirs = need(p, 1 - team);
    let want = !nil && (ours > 0 || (theirs > 0 && theirs > remaining - 3) || cover);
    let spades = p.id.suit_mask[SPADES as usize];

    if p.tlen == 0 {
        if nil {
            return lowest(p, legal);
        }
        if want {
            let mut bosses = 0;
            for c in Bits(legal) {
                if is_boss(p, c, seat) {
                    bosses |= bit(c);
                }
            }
            if bosses & !spades != 0 {
                return highest(p, bosses & !spades);
            }
            if bosses != 0 {
                return highest(p, bosses);
            }
            if cover {
                return highest(p, legal);
            }
            return lowest(p, legal);
        }
        let mut safe = 0;
        for c in Bits(legal) {
            if !is_boss(p, c, seat) {
                safe |= bit(c);
            }
        }
        return lowest(p, if safe != 0 { safe } else { legal });
    }

    let wi = p.winning_index();
    let wseat = p.tseat[wi];
    let wcard = p.trick[wi];
    let last = p.tlen == 3;
    let mut winners = 0;
    for c in Bits(legal) {
        if beats_now(p, c) {
            winners |= bit(c);
        }
    }
    let losers = legal & !winners;

    if nil {
        if losers != 0 {
            return highest(p, losers);
        }
        return if last {
            highest(p, winners)
        } else {
            lowest(p, winners)
        };
    }
    let partner_played = p.tseat[..p.tlen as usize].contains(&partner);
    if cover && (wseat == partner || !partner_played) && winners != 0 {
        return if last {
            lowest(p, winners)
        } else {
            highest(p, winners)
        };
    }
    if want {
        let partner_winning =
            wseat == partner && (last || (is_boss(p, wcard, partner) && !later_can_trump(p, seat)));
        if partner_winning || winners == 0 {
            return lowest(p, if losers != 0 { losers } else { legal });
        }
        if last {
            return lowest(p, winners);
        }
        let mut boss_winners = 0;
        for c in Bits(winners) {
            if is_boss(p, c, seat) {
                boss_winners |= bit(c);
            }
        }
        if boss_winners != 0 {
            return lowest(p, boss_winners);
        }
        return if p.tlen >= 2 {
            lowest(p, winners)
        } else {
            lowest(p, legal)
        };
    }
    if losers != 0 {
        return highest(p, losers);
    }
    if last {
        highest(p, winners)
    } else {
        lowest(p, winners)
    }
}

/// Plays the round to the end with the heuristic.
pub fn rollout(p: &mut Play, rules: &Rules, rng: &mut Rng) {
    while !p.done() {
        let m = policy(p, Some(rng));
        p.apply(m, rules);
    }
}

/// 1.0's trick estimate on effective identities.
pub fn estimate_tricks(hand: Mask, id: &Identity) -> f64 {
    let mut by: [[bool; 15]; 4] = [[false; 15]; 4];
    let mut len = [0usize; 4];
    for c in Bits(hand) {
        let s = id.suit[c as usize] as usize;
        by[s][id.rank[c as usize] as usize] = true;
        len[s] += 1;
    }
    let mut t = 0.0;
    for s in 0..3 {
        let l = len[s];
        if by[s][14] {
            t += if l <= 6 { 1.0 } else { 0.5 };
        }
        if by[s][13] {
            t += if l >= 2 {
                if l <= 5 {
                    0.8
                } else {
                    0.4
                }
            } else {
                0.2
            };
        }
        if by[s][12] && (3..=4).contains(&l) {
            t += 0.4;
        }
    }
    let n = len[3];
    if by[3][14] {
        t += 1.0;
    }
    if by[3][13] {
        t += if n >= 2 { 1.0 } else { 0.4 };
    }
    if by[3][12] {
        t += if n >= 3 { 0.8 } else { 0.3 };
    }
    if by[3][11] && n >= 4 {
        t += 0.5;
    }
    t += (n as f64 - 3.0).max(0.0) * 0.9;
    let mut spare = n as i32 - 1;
    for s in 0..3 {
        if spare <= 0 {
            break;
        }
        if len[s] == 0 {
            t += spare.min(2) as f64 * 0.7;
            spare -= 2;
        } else if len[s] == 1 {
            t += 0.5;
            spare -= 1;
        }
    }
    let size = hand.count_ones() as f64;
    if size > 13.0 {
        t *= size / 13.0;
    }
    t
}

pub fn heuristic_bid(hand: Mask, id: &Identity) -> i8 {
    (estimate_tricks(hand, id).round() as i8).clamp(1, 13)
}

/// Opening swap: a weak hand passes its highest cards; otherwise pass the shortest side suit's
/// low cards to make a void, then the lowest cards.
pub fn swap_choice(p: &Play, seat: u8, n: u8) -> Mask {
    let hand = p.hands[seat as usize];
    let n = (n as u32).min(hand.count_ones());
    let mut out: Mask = 0;
    if estimate_tricks(hand, &p.id) < 1.2 {
        let mut h = hand;
        while out.count_ones() < n {
            let c = highest(p, h);
            out |= bit(c);
            h &= !bit(c);
        }
        return out;
    }
    let mut best_suit = None;
    let mut best_len = 99;
    for s in 0..3u8 {
        let l = (hand & p.id.suit_mask[s as usize]).count_ones();
        if l > 0 && l < best_len {
            best_len = l;
            best_suit = Some(s);
        }
    }
    if let Some(s) = best_suit {
        if best_len <= n {
            out |= hand & p.id.suit_mask[s as usize];
        }
    }
    let mut h = hand & !out & !p.id.suit_mask[SPADES as usize];
    while out.count_ones() < n && h != 0 {
        let c = lowest(p, h);
        out |= bit(c);
        h &= !bit(c);
    }
    let mut h = hand & !out;
    while out.count_ones() < n {
        let c = lowest(p, h);
        out |= bit(c);
        h &= !bit(c);
    }
    out
}
