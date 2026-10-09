//! Phase 0a done checks and the random-play benchmark.

use crate::{data_files, read_json};
use rsim::cards::*;
use rsim::kernel::*;
use rsim::rng::Rng;
use rsim::sigil::*;
use rsim::{simplicity, text};
use serde_json::json;
use std::time::Instant;

fn eff(v: serde_json::Value) -> Effect {
    serde_json::from_value(v).unwrap()
}

pub fn check(dir: &str) {
    let mut fails = 0;

    println!("## Seed texts against the GDD");
    let (mut exact, mut typos) = (0, 0);
    for p in data_files(dir) {
        let v = read_json(&p);
        let Some(gdd) = v["gddText"].as_str() else {
            continue;
        };
        // Compare the seed as the GDD wrote it; retuning may since have moved its amount.
        let effect: Effect =
            serde_json::from_value(v.get("gddEffect").unwrap_or(&v["effect"]).clone()).unwrap();
        let gen_s = text::generate(&effect);
        let gen = gen_s.as_str();
        if gen == gdd {
            exact += 1;
        } else if let Some(t) = v["gddTypo"].as_str() {
            typos += 1;
            println!(
                "- typo {}: GDD \"{gdd}\" vs generated \"{gen}\" ({t})",
                v["id"].as_str().unwrap()
            );
        } else {
            fails += 1;
            println!(
                "- MISMATCH {}: GDD \"{gdd}\" vs generated \"{gen}\"",
                v["id"].as_str().unwrap()
            );
        }
    }
    println!("{exact} exact, {typos} listed GDD typos\n");

    println!("## Simplicity rubric rows (§12)");
    let rows = [
        (json!({"type": "points", "amount": 40}), Rarity::Common, 1),
        (
            json!({"type": "xmult", "amount": 1.5, "on": {"event": "make", "exact": true}}),
            Rarity::Common,
            2,
        ),
        (
            json!({"type": "points", "amount": 20, "on": {"event": "win", "card": {"rank": "A"}}}),
            Rarity::Common,
            4,
        ),
        (
            json!({"type": "points", "amount": 5, "on": {"event": "hold", "card": {"suit": "S"}}}),
            Rarity::Common,
            3,
        ),
        (
            json!({"type": "points", "amount": 5, "on": {"event": "hold", "card": {"suit": "S"}, "beyond": 5}}),
            Rarity::Common,
            7,
        ),
        (
            json!({"type": "become", "count": 4, "rank": "2"}),
            Rarity::Common,
            3,
        ),
        (
            json!({"type": "become", "count": 4, "ranks": ["2", "10"], "term": "low cards"}),
            Rarity::Common,
            9,
        ),
    ];
    for (e, r, want) in rows {
        let e = eff(e);
        let s = simplicity::score(&e, r);
        let ok = s.c == want;
        if !ok {
            fails += 1;
        }
        println!(
            "- {} | C = {} (want {}) {}",
            text::generate(&e),
            s.c,
            want,
            if ok { "ok" } else { "FAIL" }
        );
    }

    println!("\n## §4 worked examples");
    for (name, got, want) in worked_examples() {
        let ok = (got - want).abs() < 0.5;
        if !ok {
            fails += 1;
        }
        println!(
            "- {name}: {got} (want {want}) {}",
            if ok { "ok" } else { "FAIL" }
        );
    }

    println!("\n## A plain round plays legally to the end");
    let ok = plain_round_check(2000);
    if !ok {
        fails += 1;
    }
    println!(
        "{}",
        if ok {
            "2000 random rounds: 13 tricks each, every play legal, tricks sum to 13"
        } else {
            "FAIL"
        }
    );

    if fails > 0 {
        println!("\n{fails} checks failed");
        std::process::exit(1);
    }
    println!("\nAll checks passed");
}

/// Builds a play state after 13 tricks with given bids and tricks won, then scores it.
fn scored(bids: [i8; 4], won: [u8; 4], slots: Vec<(Effect, f64)>, fires: &[(usize, u32)]) -> f64 {
    let progs: Vec<Slot> = slots
        .iter()
        .map(|(e, _)| match compile(e, None).unwrap() {
            Compiled::Payoff(p) => Slot {
                payoff: Some(p),
                rule: None,
                grow_value: 0.0,
            },
            Compiled::Rule(r) => Slot {
                payoff: None,
                rule: Some(r),
                grow_value: 0.0,
            },
        })
        .collect();
    let rules = Rules {
        teams: [TeamProgram::new(progs), TeamProgram::new(vec![])],
    };
    let mut p = Play::new(Identity::standard(), [0; 4], [0; 52], 3, &rules);
    p.bids = bids;
    p.won = won;
    p.ntricks = 13;
    p.post_bid(&rules);
    // Trick events, fired as the kernel would.
    for &(slot, times) in fires {
        if let Some(po) = rules.teams[0].slots[slot].payoff {
            for _ in 0..times {
                match po.kind {
                    Kind::Points => p.acc[0].cp[slot] += po.amount,
                    Kind::Mult => p.acc[0].add[slot] += po.amount,
                    Kind::XMult => p.acc[0].x[slot] *= po.amount,
                    Kind::NilPoints => p.acc[0].nilp[slot] += po.amount,
                }
            }
        }
    }
    p.score(&rules)[0].score
}

fn worked_examples() -> Vec<(&'static str, f64, f64)> {
    // A flat set scores −100 × B whatever the sigils; a symmetric set flips the multiplied score.
    let flat = crate::rules::rules().flat_set;
    let aces =
        eff(json!({"type": "points", "amount": 20, "on": {"event": "win", "card": {"rank": "A"}}}));
    let plus15 = eff(json!({"type": "mult", "amount": 15}));
    let nil50 = eff(json!({"type": "nilPoints", "amount": 50}));
    let cp210 = eff(json!({"type": "points", "amount": 210}));
    let plus35 = eff(json!({"type": "mult", "amount": 35}));
    let x15 = eff(json!({"type": "xmult", "amount": 1.5}));
    vec![
        (
            "Bids 4 + 3, win 9, no sigils",
            scored([4, 0, 3, 0], [5, 0, 4, 0], vec![], &[]),
            700.0,
        ),
        (
            "Bids 4 + 3, win 6, no sigils",
            scored([4, 0, 3, 0], [3, 0, 3, 0], vec![], &[]),
            -700.0,
        ),
        (
            "Bid 7, win 8; [A]s +20 fires twice; one +15",
            scored(
                [4, 0, 3, 0],
                [4, 0, 4, 0],
                vec![(aces.clone(), 0.0), (plus15.clone(), 0.0)],
                &[(0, 2)],
            ),
            2750.0,
        ),
        (
            "Same, but win 6",
            scored(
                [4, 0, 3, 0],
                [3, 0, 3, 0],
                vec![(aces, 0.0), (plus15.clone(), 0.0)],
                &[(0, 2)],
            ),
            if flat { -700.0 } else { -2750.0 },
        ),
        (
            "Partner bids 5, wins 6; nil made; +50 nil points; one +15",
            scored(
                [0, 0, 5, 0],
                [0, 0, 6, 0],
                vec![(nil50.clone(), 0.0), (plus15.clone(), 0.0)],
                &[],
            ),
            5000.0,
        ),
        (
            "Same, but the nil fails",
            scored(
                [0, 0, 5, 0],
                [1, 0, 6, 0],
                vec![(nil50, 0.0), (plus15, 0.0)],
                &[],
            ),
            -1250.0,
        ),
        (
            "Bid 9, made; 210 contract points; +35; one ×1.5",
            scored(
                [5, 0, 4, 0],
                [5, 0, 4, 0],
                vec![
                    (cp210.clone(), 0.0),
                    (plus35.clone(), 0.0),
                    (x15.clone(), 0.0),
                ],
                &[],
            ),
            20250.0,
        ),
        (
            "Same, set",
            scored(
                [5, 0, 4, 0],
                [4, 0, 4, 0],
                vec![(cp210, 0.0), (plus35, 0.0), (x15, 0.0)],
                &[],
            ),
            if flat { -900.0 } else { -20250.0 },
        ),
    ]
}

fn random_round(rng: &mut Rng, rules: &Rules) -> (Play, u32) {
    let mut perm: Vec<u8> = (0..52).collect();
    rng.shuffle(&mut perm);
    let mut hands = [0u64; 4];
    for (i, &c) in perm.iter().enumerate() {
        hands[i % 4] |= bit(c);
    }
    let mut p = Play::new(
        Identity::standard(),
        hands,
        [0; 52],
        rng.below(4) as u8,
        rules,
    );
    for s in 0..4 {
        p.bids[s] = rng.below(5) as i8;
    }
    p.post_bid(rules);
    let mut plays = 0;
    while !p.done() {
        let legal = p.legal();
        let m = rng.pick_bit(legal);
        p.apply(m, rules);
        plays += 1;
    }
    (p, plays)
}

fn plain_round_check(n: u32) -> bool {
    let rules = Rules {
        teams: [TeamProgram::default(), TeamProgram::default()],
    };
    let mut rng = Rng::new(7);
    for _ in 0..n {
        let (p, plays) = random_round(&mut rng, &rules);
        if plays != 52
            || p.won.iter().map(|&x| x as u32).sum::<u32>() != 13
            || p.hands.iter().any(|&h| h != 0)
        {
            return false;
        }
    }
    true
}

pub fn bench_random(rounds: u64) {
    let aces =
        eff(json!({"type": "points", "amount": 20, "on": {"event": "win", "card": {"rank": "A"}}}));
    let slot = match compile(&aces, None).unwrap() {
        Compiled::Payoff(p) => Slot {
            payoff: Some(p),
            rule: None,
            grow_value: 0.0,
        },
        _ => unreachable!(),
    };
    let rules = Rules {
        teams: [
            TeamProgram::new(vec![slot.clone(), slot]),
            TeamProgram::default(),
        ],
    };
    let mut rng = Rng::new(1);
    let t = Instant::now();
    let mut plays = 0u64;
    let mut sink = 0.0;
    for _ in 0..rounds {
        let (p, n) = random_round(&mut rng, &rules);
        sink += p.score(&rules)[0].score;
        plays += n as u64;
    }
    let dt = t.elapsed().as_secs_f64();
    println!(
        "random play: {rounds} rounds, {plays} card plays in {dt:.2}s = {:.0} card plays per second on one core (checksum {sink})",
        plays as f64 / dt
    );
}
