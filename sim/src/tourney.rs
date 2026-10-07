//! Experiments: benches, ladders, throughput, grant tournaments and arms, screens, enumeration.

use clap::Args;
use rayon::prelude::*;
use rsim::ai::search::Tier;
use rsim::game::*;
use rsim::model::Model;
use std::time::Instant;

#[derive(Args)]
pub struct BenchArgs {
    #[arg(long, default_value_t = 1)]
    pub tier: u8,
    #[arg(long, default_value_t = 100)]
    pub runs: u64,
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    #[arg(long)]
    pub model: Option<String>,
    #[arg(long)]
    pub nil_handicap: Option<f64>,
}

#[derive(Args)]
pub struct LadderArgs {
    #[arg(long, default_value_t = 2)]
    pub a: u8,
    #[arg(long, default_value_t = 0)]
    pub b: u8,
    #[arg(long, default_value_t = 100)]
    pub boards: u64,
    #[arg(long, default_value_t = 1)]
    pub seed: u64,
    #[arg(long)]
    pub model: Option<String>,
}

#[derive(Args)]
pub struct RunArgs {
    /// Experiment config (JSON).
    pub config: String,
}

#[derive(Args)]
pub struct ScreenArgs {
    pub config: String,
}

#[derive(Args)]
pub struct ThroughputArgs {
    #[arg(long, default_value_t = 0)]
    pub tier: u8,
    #[arg(long, default_value_t = 36)]
    pub runs: u64,
    #[arg(long, default_value = "../data/sigils")]
    pub dir: String,
}

fn plain_cfg<'a>(pool: &'a Pool, model: &'a Model, tiers: [Tier; 2]) -> RunCfg<'a> {
    RunCfg {
        pool,
        model,
        tiers,
        offerable: vec![],
        sigil_shop: false,
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
        rounds: 8,
        cards: false,
    }
}

#[derive(Default, Clone)]
struct Tally {
    rounds: u64,
    sets: u64,
    made: u64,
    nil_bids: u64,
    nil_made: u64,
    nil_plays: u64,
    nil_suicides: u64,
    overtake_opps: u64,
    overtakes: u64,
    trump_overtakes: u64,
    overtricks: u64,
    bid_err: f64,
    points: f64,
}

fn tally(recs: &[RunRec]) -> Tally {
    let mut t = Tally::default();
    for r in recs {
        for tm in &r.teams {
            for rd in &tm.rounds {
                if rd.contract > 0 {
                    t.rounds += 1;
                    if rd.made {
                        t.made += 1;
                        t.overtricks += (rd.tricks - rd.contract) as u64;
                    } else {
                        t.sets += 1;
                    }
                    t.bid_err += (rd.tricks as f64 - rd.contract as f64).abs();
                }
                t.nil_bids += rd.nil_bids as u64;
                t.nil_made += rd.nil_made as u64;
                t.points += rd.score;
            }
            t.nil_plays += tm.bench.nil_plays as u64;
            t.nil_suicides += tm.bench.nil_suicides as u64;
            t.overtake_opps += tm.bench.overtake_opps as u64;
            t.overtakes += tm.bench.overtakes as u64;
            t.trump_overtakes += tm.bench.trump_overtakes as u64;
        }
    }
    t
}

fn pct(a: u64, b: u64) -> String {
    format!("{:.1}% ({a}/{b})", 100.0 * a as f64 / b.max(1) as f64)
}

pub fn bench_plain(a: BenchArgs) {
    let pool = Pool::new(vec![]);
    let mut model = a
        .model
        .as_ref()
        .map(|p| Model::load(p))
        .unwrap_or_else(Model::starting);
    if let Some(h) = a.nil_handicap {
        model.nil_handicap = h;
    }
    let tier = Tier::get(a.tier);
    let cfg = plain_cfg(&pool, &model, [tier, tier]);
    let t0 = Instant::now();
    let recs: Vec<RunRec> = (0..a.runs)
        .into_par_iter()
        .map(|i| Runner::new(&cfg, rsim::rng::derive(a.seed, &[i]), 0).run(i))
        .collect();
    let dt = t0.elapsed().as_secs_f64();
    let t = tally(&recs);
    println!(
        "Plain Spades bench: tier {}, {} runs ({} team-rounds) in {dt:.1}s = {:.2} runs/s",
        a.tier,
        a.runs,
        t.rounds,
        a.runs as f64 / dt
    );
    println!("| Metric | Value |\n| --- | --- |");
    println!("| Team set rate | {} |", pct(t.sets, t.rounds));
    println!("| Nil success | {} |", pct(t.nil_made, t.nil_bids));
    println!(
        "| Nil bids per team-round | {:.3} |",
        t.nil_bids as f64 / (a.runs * 16) as f64
    );
    println!("| Nil suicides | {} |", pct(t.nil_suicides, t.nil_plays));
    println!(
        "| Wasted overtakes per opportunity | {} + {} trumps / {} |",
        t.overtakes, t.trump_overtakes, t.overtake_opps
    );
    println!(
        "| Overtricks per made contract | {:.2} |",
        t.overtricks as f64 / t.made.max(1) as f64
    );
    println!(
        "| Mean abs(bid - tricks) | {:.2} |",
        t.bid_err / t.rounds.max(1) as f64
    );
    println!(
        "| Points per team-round | {:.0} |",
        t.points / (a.runs * 16) as f64
    );
}

pub fn ladder(a: LadderArgs) {
    let pool = Pool::new(vec![]);
    let model = a
        .model
        .as_ref()
        .map(|p| Model::load(p))
        .unwrap_or_else(Model::starting);
    let cfg = plain_cfg(&pool, &model, [Tier::get(a.a), Tier::get(a.b)]);
    let t0 = Instant::now();
    let boards: Vec<(RunRec, RunRec)> = (0..a.boards)
        .into_par_iter()
        .map(|i| {
            let seed = rsim::rng::derive(a.seed, &[i]);
            (
                Runner::new(&cfg, seed, 0).run(i),
                Runner::new(&cfg, seed, 1).run(i),
            )
        })
        .collect();
    let dt = t0.elapsed().as_secs_f64();
    let mut wins = 0.0;
    let mut runs = 0.0;
    let mut board_wins = 0.0;
    for (x, y) in &boards {
        wins += x.win + y.win;
        runs += 2.0;
        let m = x.margin + y.margin;
        board_wins += if m > 0.0 {
            1.0
        } else if m < 0.0 {
            0.0
        } else {
            0.5
        };
    }
    println!(
        "Ladder: tier {} vs tier {}: {} boards in {dt:.0}s; tier {} wins {:.1}% of runs and {:.1}% of boards (by board margin)",
        a.a,
        a.b,
        a.boards,
        a.a,
        100.0 * wins / runs,
        100.0 * board_wins / a.boards as f64
    );
}

pub fn throughput(a: ThroughputArgs) {
    let pool = Pool::load_dir(&a.dir);
    let model = Model::starting();
    let tier = Tier::get(a.tier);
    let offerable: Vec<usize> = (0..pool.defs.len()).collect();
    let mut cfg = plain_cfg(&pool, &model, [tier, tier]);
    cfg.offerable = offerable;
    cfg.sigil_shop = true;
    cfg.cards = true;
    let t0 = Instant::now();
    let n: usize = (0..a.runs)
        .into_par_iter()
        .map(|i| {
            Runner::new(&cfg, rsim::rng::derive(99, &[i]), 0)
                .run(i)
                .teams[0]
                .sigils
                .len()
        })
        .sum();
    let dt = t0.elapsed().as_secs_f64();
    println!(
        "Throughput tier {}: {} runs in {dt:.1}s = {:.2} runs/s on {} threads ({} sigils held)",
        a.tier,
        a.runs,
        a.runs as f64 / dt,
        rayon::current_num_threads(),
        n
    );
}

pub fn run(_a: RunArgs) {
    unimplemented!("0c")
}

pub fn screen(_a: ScreenArgs) {
    unimplemented!("0c")
}

pub fn enumerate(_dir: &str, _step: &str) {
    unimplemented!("0c")
}
