//! Determinized search: tier 0 flat Monte Carlo, tiers 1–2 information-set MCTS, and bidding by
//! expected win probability. No component knows any sigil: it searches the real rules and scores.

use super::heuristic::*;
use crate::cards::*;
use crate::kernel::*;
use crate::rng::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Tier {
    pub level: u8,
    /// Tier 0: rollouts per legal move (over common deals).
    pub flat: u32,
    /// MCTS iterations (0 = flat Monte Carlo).
    pub iters: u32,
    pub deals_sample: u32,
    pub deals_keep: u32,
    pub bid_rollouts: u32,
}

impl Tier {
    pub fn get(level: u8) -> Tier {
        match level {
            0 => Tier {
                level,
                flat: 6,
                iters: 0,
                deals_sample: 6,
                deals_keep: 6,
                bid_rollouts: 10,
            },
            1 => Tier {
                level,
                flat: 0,
                iters: 200,
                deals_sample: 96,
                deals_keep: 32,
                bid_rollouts: 40,
            },
            _ => Tier {
                level,
                flat: 0,
                iters: 1500,
                deals_sample: 512,
                deals_keep: 64,
                bid_rollouts: 160,
            },
        }
    }
}

/// What a seat knows beyond its own hand.
#[derive(Clone, Copy)]
pub struct Knowledge {
    pub seat: u8,
    /// Partner's owned cards still in its hand.
    pub known: Mask,
    /// Engravings the seat can see (its team's cards and played cards).
    pub eng_known: Mask,
    pub voids: [[bool; 4]; 4],
}

/// The observer's utility: win probability over the run.
#[derive(Clone, Copy)]
pub struct Utility {
    /// Observer team's run score minus the opponents', before this round.
    pub margin: f64,
    /// Logistic scale for the margin after this round.
    pub scale: f64,
    /// Margin value of one growth step per kind (points, mult, xmult, nil points).
    pub growth: [f64; 4],
    /// Margin penalty for bidding nil, in points (rollouts are kind to nils).
    pub nil_handicap: f64,
    /// Win probability per gold of next round's income (0 in the last round).
    pub gold: f64,
}

impl Utility {
    #[inline]
    pub fn reward(&self, p: &Play, rules: &Rules, team: usize) -> f64 {
        let sc = p.score(rules);
        let mut d = sc[team].score - sc[1 - team].score;
        for (t, sign) in [(team, 1.0), (1 - team, -1.0)] {
            for &(slot, po) in &rules.teams[t].growth {
                let g = sc[t].acc.grow[slot] as f64;
                if g > 0.0 {
                    d += sign * g * po.amount * self.growth[po.kind as usize];
                }
            }
        }
        let income = |s: &TeamScore| {
            (if s.made {
                10.0 * s.contract as f64
            } else {
                0.0
            }) + 50.0 * s.nil_made as f64
        };
        let g = self.gold * (income(&sc[team]) - income(&sc[1 - team]));
        1.0 / (1.0 + (-(self.margin + d) / self.scale).exp()) + g
    }
}

/// Samples hidden hands for the other seats consistent with what `k.seat` knows.
pub fn determinize(truth: &Play, k: &Knowledge, rng: &mut Rng) -> Play {
    let me = k.seat as usize;
    let mut p = *truth;
    let mut id = Identity::standard();
    let visible = truth.hands[me] | truth.played | k.known;
    for c in Bits(visible) {
        id.set(c, truth.id.suit[c as usize], truth.id.rank[c as usize]);
    }
    p.id = id;
    for c in 0..52 {
        if k.eng_known & bit(c) == 0 && truth.played & bit(c) == 0 {
            p.eng[c as usize] = ENG_NONE;
        }
    }
    let hidden = FULL_DECK & !visible;
    let partner = (me + 2) % 4;
    let mut room = [0u32; 4];
    let mut hands = [0u64; 4];
    hands[me] = truth.hands[me];
    hands[partner] = k.known;
    for s in 0..4 {
        if s != me {
            room[s] = truth.hands[s].count_ones() - hands[s].count_ones();
        }
    }
    let mut cards: Vec<u8> = Bits(hidden).collect();
    for attempt in 0..20 {
        let respect = attempt < 19;
        rng.shuffle(&mut cards);
        let open = |c: u8| {
            (0..4)
                .filter(|&s| s != me && !(respect && k.voids[s][p.id.suit[c as usize] as usize]))
                .count()
        };
        cards.sort_by_key(|&c| open(c));
        let mut h = hands;
        let mut left = room;
        let mut ok = true;
        for &c in &cards {
            let su = p.id.suit[c as usize] as usize;
            let mut elig = [false; 4];
            let mut total = 0;
            for s in 0..4 {
                if s != me && left[s] > 0 && !(respect && k.voids[s][su]) {
                    elig[s] = true;
                    total += left[s];
                }
            }
            if total == 0 {
                if respect {
                    ok = false;
                    break;
                }
                for s in 0..4 {
                    if s != me && left[s] > 0 {
                        elig[s] = true;
                        total += left[s];
                    }
                }
            }
            let mut pick = rng.below(total as usize) as u32;
            for s in 0..4 {
                if !elig[s] {
                    continue;
                }
                if pick < left[s] {
                    h[s] |= bit(c);
                    left[s] -= 1;
                    break;
                }
                pick -= left[s];
            }
        }
        if ok
            && h.iter()
                .zip(truth.hands.iter())
                .all(|(a, b)| a.count_ones() == b.count_ones())
        {
            p.hands = h;
            return p;
        }
    }
    // Fallback: plain fill.
    let mut h = hands;
    for &c in &cards {
        for s in 0..4 {
            if s != me && h[s].count_ones() < truth.hands[s].count_ones() {
                h[s] |= bit(c);
                break;
            }
        }
    }
    p.hands = h;
    p
}

/// How badly a deal fits the other seats' bids; lower is better.
fn misfit(p: &Play, me: usize) -> f64 {
    let mut total = 0.0;
    for s in 0..4 {
        let b = p.bids[s];
        if s == me || b < 0 {
            continue;
        }
        let est = estimate_tricks(p.hands[s], &p.id);
        if b == 0 {
            if p.won[s] == 0 {
                total += (est - 0.5).max(0.0);
            }
        } else {
            total += (est - (b as f64 - p.won[s] as f64).max(0.0)).abs();
        }
    }
    total
}

pub fn deal_pool(truth: &Play, k: &Knowledge, rng: &mut Rng, sample: u32, keep: u32) -> Vec<Play> {
    let mut deals: Vec<(f64, Play)> = (0..sample)
        .map(|_| {
            let d = determinize(truth, k, rng);
            (misfit(&d, k.seat as usize), d)
        })
        .collect();
    if keep < sample {
        deals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        deals.truncate(keep as usize);
    }
    deals.into_iter().map(|d| d.1).collect()
}

/// Chooses a move (card or lead choice) for the seat to act.
pub fn choose_move(
    truth: &Play,
    rules: &Rules,
    k: &Knowledge,
    u: &Utility,
    tier: &Tier,
    rng: &mut Rng,
) -> u8 {
    let legal = truth.legal();
    if legal.count_ones() == 1 {
        return legal.trailing_zeros() as u8;
    }
    let team = (k.seat % 2) as usize;
    let deals = deal_pool(truth, k, rng, tier.deals_sample, tier.deals_keep);
    let moves: Vec<u8> = Bits(legal).collect();
    let (means, spread) = if tier.iters == 0 {
        flat(&deals, &moves, rules, u, team, tier.flat, rng)
    } else {
        mcts(&deals, &moves, rules, u, team, tier.iters, rng)
    };
    pick(truth, &deals[0], &moves, &means, spread)
}

/// Among moves within a hair of the best mean, prefer the heuristic's move, then the cheapest card.
fn pick(truth: &Play, d0: &Play, moves: &[u8], means: &[f64], spread: f64) -> u8 {
    let best = means.iter().cloned().fold(f64::MIN, f64::max);
    let band = 0.1 * spread;
    let near: Vec<u8> = moves
        .iter()
        .zip(means)
        .filter(|(_, &m)| m >= best - band)
        .map(|(&c, _)| c)
        .collect();
    let h = policy(d0, None);
    if near.contains(&h) {
        return h;
    }
    if near.len() == 1 || near[0] >= 52 {
        return near[0];
    }
    let mut mask = 0;
    for &c in &near {
        mask |= bit(c);
    }
    lowest(truth, mask)
}

fn flat(
    deals: &[Play],
    moves: &[u8],
    rules: &Rules,
    u: &Utility,
    team: usize,
    n: u32,
    rng: &mut Rng,
) -> (Vec<f64>, f64) {
    let mut means = vec![0.0; moves.len()];
    let mut all = Stats::default();
    let base = rng.next_u64();
    for (i, &m) in moves.iter().enumerate() {
        let mut sum = 0.0;
        for j in 0..n {
            let mut p = deals[j as usize % deals.len()];
            let mut r = Rng::new(base ^ (j as u64).wrapping_mul(0x9E3779B97F4A7C15));
            p.apply(m, rules);
            rollout(&mut p, rules, &mut r);
            let v = u.reward(&p, rules, team);
            all.add(v);
            sum += v;
        }
        means[i] = sum / n as f64;
    }
    (means, all.sd().max(1e-6))
}

#[derive(Default)]
struct Stats {
    n: f64,
    s: f64,
    ss: f64,
}
impl Stats {
    fn add(&mut self, v: f64) {
        self.n += 1.0;
        self.s += v;
        self.ss += v * v;
    }
    fn sd(&self) -> f64 {
        if self.n < 2.0 {
            return 0.1;
        }
        let m = self.s / self.n;
        (self.ss / self.n - m * m).max(0.0).sqrt()
    }
}

struct Node {
    children: Vec<(u8, u32)>,
    visits: f64,
    avail: f64,
    reward: f64,
    player: i8,
    parent: u32,
}

fn mover(p: &Play) -> u8 {
    if p.choosing >= 0 {
        p.choosing as u8
    } else {
        p.turn
    }
}

fn mcts(
    deals: &[Play],
    root_moves: &[u8],
    rules: &Rules,
    u: &Utility,
    team: usize,
    iters: u32,
    rng: &mut Rng,
) -> (Vec<f64>, f64) {
    let mut nodes: Vec<Node> = Vec::with_capacity(iters as usize + 1);
    nodes.push(Node {
        children: vec![],
        visits: 0.0,
        avail: 1.0,
        reward: 0.0,
        player: -1,
        parent: u32::MAX,
    });
    let mut stats = Stats::default();
    let mut root_mask = 0u64;
    for &m in root_moves {
        root_mask |= bit(m);
    }
    for it in 0..iters {
        let mut sim = deals[it as usize % deals.len()];
        let mut node = 0u32;
        let mut first = true;
        let explore = 1.0 * stats.sd().max(0.02);
        while !sim.done() {
            let legal = if first { root_mask } else { sim.legal() };
            first = false;
            let mut untried = legal;
            for &(m, _) in &nodes[node as usize].children {
                untried &= !bit(m);
            }
            let kids: Vec<(u8, u32)> = nodes[node as usize]
                .children
                .iter()
                .filter(|(m, _)| legal & bit(*m) != 0)
                .cloned()
                .collect();
            for &(_, c) in &kids {
                nodes[c as usize].avail += 1.0;
            }
            if untried != 0 {
                let m = rng.pick_bit(untried);
                let player = mover(&sim) as i8;
                let idx = nodes.len() as u32;
                nodes.push(Node {
                    children: vec![],
                    visits: 0.0,
                    avail: 1.0,
                    reward: 0.0,
                    player,
                    parent: node,
                });
                nodes[node as usize].children.push((m, idx));
                sim.apply(m, rules);
                node = idx;
                break;
            }
            let mut best = (0u8, 0u32);
            let mut bs = f64::MIN;
            for &(m, c) in &kids {
                let n = &nodes[c as usize];
                let score = n.reward / n.visits + explore * (n.avail.ln() / n.visits).sqrt();
                if score > bs {
                    bs = score;
                    best = (m, c);
                }
            }
            sim.apply(best.0, rules);
            node = best.1;
        }
        rollout(&mut sim, rules, rng);
        let r0 = u.reward(&sim, rules, team);
        stats.add(r0);
        let mut n = node;
        while n != u32::MAX {
            let nd = &mut nodes[n as usize];
            nd.visits += 1.0;
            if nd.player >= 0 {
                let t = (nd.player % 2) as usize;
                nd.reward += if t == team { r0 } else { 1.0 - r0 };
            }
            n = nd.parent;
        }
    }
    let root = &nodes[0];
    let most = root
        .children
        .iter()
        .map(|&(_, c)| nodes[c as usize].visits)
        .fold(0.0, f64::max);
    let means = root_moves
        .iter()
        .map(|&m| {
            root.children
                .iter()
                .find(|(x, _)| *x == m)
                .map(|&(_, c)| {
                    let n = &nodes[c as usize];
                    if n.visits >= most * 0.1 && n.visits > 0.0 {
                        n.reward / n.visits
                    } else {
                        f64::MIN / 2.0
                    }
                })
                .unwrap_or(f64::MIN / 2.0)
        })
        .collect();
    (means, stats.sd().max(1e-6))
}

/// Bids by expected win probability over rollouts. `truth` has the bids so far.
pub fn choose_bid(
    truth: &Play,
    rules: &Rules,
    k: &Knowledge,
    u: &Utility,
    tier: &Tier,
    rng: &mut Rng,
) -> i8 {
    let me = k.seat as usize;
    let partner = (me + 2) % 4;
    let team = me % 2;
    let est = estimate_tricks(truth.hands[me], &truth.id);
    let base = (est.round() as i8).clamp(1, 13);
    let mut cands: Vec<i8> = vec![base - 1, base, base + 1]
        .into_iter()
        .filter(|b| (1..=13).contains(b))
        .collect();
    if truth.bids[partner] != 0 {
        cands.push(0);
    }
    let n = tier.bid_rollouts.max(4);
    let sample = if tier.level == 0 { n } else { n * 2 };
    let deals = deal_pool(truth, k, rng, sample, n);
    let mut best = base;
    let mut bv = f64::MIN;
    let seed = rng.next_u64();
    for &b in &cands {
        let mut uu = *u;
        if b == 0 {
            uu.margin -= u.nil_handicap;
        }
        let mut sum = 0.0;
        for (j, d) in deals.iter().enumerate() {
            let mut p = *d;
            p.bids[me] = b;
            let first = p.first_leader;
            for s in 0..4 {
                if p.bids[s] < 0 {
                    p.bids[s] = heuristic_bid(p.hands[s], &p.id);
                }
            }
            p.post_bid(rules);
            p.leader = first;
            p.turn = first;
            let mut r = Rng::new(seed ^ (j as u64).wrapping_mul(0x9E3779B97F4A7C15));
            rollout(&mut p, rules, &mut r);
            sum += uu.reward(&p, rules, team);
        }
        let v = sum / deals.len() as f64;
        if v > bv {
            bv = v;
            best = b;
        }
    }
    best
}

/// Opening swap: the heuristic alone at tier 0; at higher tiers, about six candidate give-sets
/// evaluated by rollouts over deals sampled from this seat's information.
pub fn choose_swap(
    truth: &Play,
    rules: &Rules,
    k: &Knowledge,
    u: &Utility,
    tier: &Tier,
    rng: &mut Rng,
    n: u8,
) -> Mask {
    let me = k.seat;
    let heur = swap_choice(truth, me, n);
    if tier.level == 0 {
        return heur;
    }
    let hand = truth.hands[me as usize];
    let take = |order: &mut dyn FnMut(Mask) -> u8, from: Mask| {
        let mut out: Mask = 0;
        let mut h = from;
        while out.count_ones() < (n as u32).min(hand.count_ones()) && h != 0 {
            let c = order(h);
            out |= bit(c);
            h &= !bit(c);
        }
        let mut rest = hand & !out;
        while out.count_ones() < (n as u32).min(hand.count_ones()) {
            let c = lowest(truth, rest);
            out |= bit(c);
            rest &= !bit(c);
        }
        out
    };
    let spades = truth.id.suit_mask[SPADES as usize];
    let mut cands = vec![
        heur,
        take(&mut |h| highest(truth, h), hand),
        take(&mut |h| lowest(truth, h), hand),
        take(&mut |h| highest(truth, h), hand & spades),
        take(&mut |h| lowest(truth, h), hand & !spades),
        take(&mut |h| highest(truth, h), hand & !spades),
    ];
    cands.sort();
    cands.dedup();
    let team = (me % 2) as usize;
    let partner = ((me + 2) % 4) as usize;
    let nd = (tier.bid_rollouts / 2).max(8);
    let deals = deal_pool(truth, k, rng, nd, nd);
    let seed = rng.next_u64();
    let mut best = heur;
    let mut bv = f64::MIN;
    for &g in &cands {
        let mut sum = 0.0;
        for (j, d) in deals.iter().enumerate() {
            let mut p = *d;
            let pg = swap_choice(&p, partner as u8, n);
            let cnt = g.count_ones().min(pg.count_ones());
            let (g2, pg2) = (first_bits(g, cnt), first_bits(pg, cnt));
            p.hands[me as usize] = (p.hands[me as usize] & !g2) | pg2;
            p.hands[partner] = (p.hands[partner] & !pg2) | g2;
            for s in 0..4 {
                p.bids[s] = heuristic_bid(p.hands[s], &p.id);
            }
            p.post_bid(rules);
            let mut r = Rng::new(seed ^ (j as u64).wrapping_mul(0x9E3779B97F4A7C15));
            rollout(&mut p, rules, &mut r);
            sum += u.reward(&p, rules, team);
        }
        if sum > bv {
            bv = sum;
            best = g;
        }
    }
    best
}

fn first_bits(m: Mask, n: u32) -> Mask {
    let mut out = 0;
    for c in Bits(m).take(n as usize) {
        out |= bit(c);
    }
    out
}
