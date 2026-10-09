//! Experiments: benches, ladders, throughput, grant tournaments and arms, screens, enumeration.

use clap::Args;
use rayon::prelude::*;
use rsim::ai::search::Tier;
use rsim::game::*;
use rsim::model::Model;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
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
        rounds: rsim::rules::rules().rounds,
        cards: false,
        validate: 0.0,
        version: String::new(),
        ai: AiCfg::default(),
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

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct ExpCfg {
    pub name: String,
    pub seed: u64,
    pub boards: u64,
    pub board_offset: u64,
    pub tier: u8,
    pub tier_b: Option<u8>,
    pub model: Option<String>,
    pub dir: String,
    /// Offered in the shop: sigil ids.
    pub offerable: Vec<String>,
    pub sigil_shop: bool,
    pub cards: bool,
    /// Grant weights by sigil id (the measured set).
    pub measured: HashMap<String, f64>,
    pub grant_prob: f64,
    pub grant_max_shop: u8,
    pub coherent: f64,
    pub jitter: bool,
    pub card_grant_prob: f64,
    pub pairs: Vec<(String, String)>,
    pub pair_prob: f64,
    pub clean_share: f64,
    pub perturb: f64,
    pub perturb_amount: i32,
    pub explore: f64,
    pub validate: f64,
    pub version: String,
    pub arm: Arm,
    pub out: String,
    /// Rules file (default: data/rules.json as compiled in).
    pub rules: Option<String>,
    pub risk_neutral: bool,
    /// Per run team (A, B).
    pub bid_offset: [i8; 2],
    pub always_nil: [bool; 2],
    pub commit_bonus: [f64; 2],
    pub aware: [bool; 2],
    pub ai_salt: u64,
    pub true_ids: bool,
    pub bid_discount: Option<f64>,
    pub bid_discount_slope: Option<f64>,
    /// Per run team: the AI ignores bags when it searches (a control for bag awareness).
    pub bag_blind: [bool; 2],
    /// Per run team: never bid nil (a control for nil bidding).
    pub never_nil: [bool; 2],
    /// Team B's policy override: "hoard" or a committed archetype ("commit:Spades").
    pub policy_b: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Arm {
    /// tournament | commitment | chaser | standalone
    #[serde(rename = "type")]
    pub ty: String,
    pub archetype: Option<String>,
    pub build: Vec<String>,
    /// Commitment arms: grants the opponents receive (sigils that touch opponents).
    pub counter: HashMap<String, f64>,
    pub counter_prob: f64,
    /// Grant arms: the shop at which the build is granted.
    pub shop: Option<u8>,
}

impl Default for ExpCfg {
    fn default() -> Self {
        ExpCfg {
            name: "exp".into(),
            seed: 1,
            boards: 100,
            board_offset: 0,
            tier: 0,
            tier_b: None,
            model: None,
            dir: "../data/sigils".into(),
            offerable: vec![],
            sigil_shop: true,
            cards: true,
            measured: HashMap::new(),
            grant_prob: 0.5,
            grant_max_shop: 6,
            coherent: 0.6,
            jitter: true,
            card_grant_prob: 0.3,
            pairs: vec![],
            pair_prob: 0.0,
            clean_share: 0.1,
            perturb: 0.15,
            perturb_amount: 50,
            explore: 0.05,
            validate: 0.0,
            version: String::new(),
            arm: Arm {
                ty: "tournament".into(),
                ..Default::default()
            },
            out: "../runs/exp.jsonl".into(),
            rules: None,
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
            policy_b: None,
        }
    }
}

fn grant_cfg(
    pool: &Pool,
    c: &ExpCfg,
    measured: &HashMap<String, f64>,
    prob: f64,
) -> Option<GrantCfg> {
    if measured.is_empty() || prob <= 0.0 {
        return None;
    }
    let mut m: Vec<(usize, f64)> = measured.iter().map(|(k, &w)| (pool.idx(k), w)).collect();
    m.sort_by_key(|x| x.0);
    Some(GrantCfg {
        prob,
        max_shop: c.grant_max_shop,
        coherent: c.coherent,
        measured: m,
        jitter: c.jitter,
        card_prob: c.card_grant_prob,
        pairs: c
            .pairs
            .iter()
            .map(|(a, b)| (pool.idx(a), pool.idx(b)))
            .collect(),
        pair_prob: c.pair_prob,
        fixed: vec![],
        free: false,
    })
}

pub fn run(a: RunArgs) {
    let c: ExpCfg =
        serde_json::from_str(&std::fs::read_to_string(&a.config).unwrap()).expect("config");
    if let Some(r) = &c.rules {
        rsim::rules::install(rsim::rules::load(r));
    }
    let pool = Pool::load_dir(&c.dir);
    let model = c
        .model
        .as_ref()
        .map(|p| Model::load(p))
        .unwrap_or_else(Model::starting);
    let offerable: Vec<usize> = c.offerable.iter().map(|id| pool.idx(id)).collect();
    let tiers = [Tier::get(c.tier), Tier::get(c.tier_b.unwrap_or(c.tier))];
    let flex = |g: Option<GrantCfg>| TeamCfg {
        policy: Policy::Flexible,
        grants: g,
    };
    let (team_a, team_b, sigil_shop) = match c.arm.ty.as_str() {
        "tournament" => {
            let g = grant_cfg(&pool, &c, &c.measured, c.grant_prob);
            (flex(g.clone()), flex(g), c.sigil_shop)
        }
        "standalone" => {
            let g = grant_cfg(&pool, &c, &c.measured, c.grant_prob);
            (flex(g.clone()), flex(g), false)
        }
        "commitment" => {
            let arch = c.arm.archetype.clone().expect("archetype");
            let g = grant_cfg(&pool, &c, &c.arm.counter, c.arm.counter_prob);
            (
                TeamCfg {
                    policy: Policy::Committed(arch),
                    grants: None,
                },
                flex(g),
                c.sigil_shop,
            )
        }
        "chaser" => {
            let build: Vec<usize> = c.arm.build.iter().map(|id| pool.idx(id)).collect();
            (
                TeamCfg {
                    policy: Policy::Chaser(build),
                    grants: None,
                },
                flex(None),
                c.sigil_shop,
            )
        }
        "grant" => {
            let fixed: Vec<usize> = c.arm.build.iter().map(|id| pool.idx(id)).collect();
            let g = GrantCfg {
                prob: 0.0,
                max_shop: c.arm.shop.unwrap_or(1),
                coherent: 0.0,
                measured: vec![],
                jitter: false,
                card_prob: 0.0,
                pairs: vec![],
                pair_prob: 0.0,
                fixed,
                free: true,
            };
            (flex(Some(g)), flex(None), c.sigil_shop)
        }
        "hunter" => {
            let p = match c.arm.archetype.as_deref() {
                Some("hoard") => Policy::Hoard,
                Some(a) => Policy::Committed(a.to_string()),
                None => Policy::Flexible,
            };
            (
                TeamCfg {
                    policy: p,
                    grants: None,
                },
                flex(None),
                c.sigil_shop,
            )
        }
        x => panic!("unknown arm {x}"),
    };
    let (team_a, team_b) = match c.policy_b.as_deref() {
        Some("hoard") => (
            team_a,
            TeamCfg {
                policy: Policy::Hoard,
                grants: None,
            },
        ),
        _ => (team_a, team_b),
    };
    let base = RunCfg {
        pool: &pool,
        model: &model,
        tiers,
        offerable,
        sigil_shop,
        teams: [team_a, team_b],
        explore: c.explore,
        perturb: c.perturb,
        perturb_amount: c.perturb_amount,
        rounds: rsim::rules::rules().rounds,
        cards: c.cards,
        validate: c.validate,
        version: c.version.clone(),
        ai: AiCfg {
            risk_neutral: c.risk_neutral,
            bid_offset: c.bid_offset,
            always_nil: c.always_nil,
            commit_bonus: c.commit_bonus,
            aware: c.aware,
            ai_salt: c.ai_salt,
            true_ids: c.true_ids,
            bid_discount: c.bid_discount,
            bid_discount_slope: c.bid_discount_slope,
            bag_blind: c.bag_blind,
            never_nil: c.never_nil,
        },
    };
    let mut clean = base.clone();
    clean.teams[0].grants = None;
    clean.teams[1].grants = None;
    clean.perturb = 0.0;
    clean.validate = 0.0;

    if let Some(dir) = std::path::Path::new(&c.out).parent() {
        std::fs::create_dir_all(dir).unwrap();
    }
    let mut out = std::io::BufWriter::new(std::fs::File::create(&c.out).unwrap());
    let t0 = Instant::now();
    let chunk = 2000u64;
    let mut done = 0u64;
    while done < c.boards {
        let n = chunk.min(c.boards - done);
        let lines: Vec<String> = (done..done + n)
            .into_par_iter()
            .map(|i| {
                let b = i + c.board_offset;
                let seed = rsim::rng::derive(c.seed, &[b]);
                let is_clean = rsim::rng::Rng::stream(seed, &[77]).chance(c.clean_share);
                let cfg = if is_clean { &clean } else { &base };
                let mut s = String::new();
                for orient in 0..2 {
                    let mut r = Runner::new(cfg, seed, orient).run(b);
                    r.clean = is_clean;
                    s += &serde_json::to_string(&r).unwrap();
                    s.push('\n');
                }
                s
            })
            .collect();
        for l in lines {
            out.write_all(l.as_bytes()).unwrap();
        }
        done += n;
        eprintln!(
            "{}: {done}/{} boards, {:.0}s",
            c.name,
            c.boards,
            t0.elapsed().as_secs_f64()
        );
    }
    out.flush().unwrap();
    let dt = t0.elapsed().as_secs_f64();
    println!(
        "{}: {} boards ({} runs) at tier {} in {dt:.1}s = {:.1} runs/s -> {}",
        c.name,
        c.boards,
        2 * c.boards,
        c.tier,
        2.0 * c.boards as f64 / dt,
        c.out
    );
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct ScreenCfg {
    pub dir: String,
    pub ids: Vec<String>,
    pub rounds: u64,
    pub round: u8,
    pub seed: u64,
    pub tier: u8,
    /// Also screen with three owned cards matching the payoff's filter ("committed").
    pub committed: bool,
    pub out: String,
}
impl Default for ScreenCfg {
    fn default() -> Self {
        ScreenCfg {
            dir: "../data/sigils".into(),
            ids: vec![],
            rounds: 400,
            round: 4,
            seed: 5,
            tier: 0,
            committed: true,
            out: "../runs/screen.json".into(),
        }
    }
}

#[derive(Serialize, Default)]
struct ScreenStat {
    id: String,
    rounds: u64,
    /// Share of rounds with at least one firing.
    fire: f64,
    /// Mean firings per round.
    fires: f64,
    committed_fire: Option<f64>,
    committed_fires: Option<f64>,
    set_rate: f64,
    mean_score: f64,
    /// Mean score with the sigil's ledger removed (same play).
    mean_score_without: f64,
}

/// Cards matching a payoff's card filter, highest first, for a "committed" fixed build.
fn matching_cards(p: &rsim::sigil::Compiled) -> Vec<u8> {
    use rsim::cards::*;
    use rsim::sigil::*;
    let filt = match p {
        Compiled::Payoff(po) => match po.trig {
            Trig::Win { filt, .. } | Trig::Lead { filt, .. } | Trig::Hold { filt, .. } => filt,
            _ => return vec![],
        },
        _ => return vec![],
    };
    if filt == CardFilter::ANY {
        return vec![];
    }
    let mut cs: Vec<u8> = (0..52u8)
        .filter(|&c| filt.matches(nominal_suit(c), nominal_rank(c)))
        .collect();
    cs.sort_by_key(|&c| std::cmp::Reverse(nominal_rank(c)));
    cs.truncate(3);
    cs
}

pub fn screen(a: ScreenArgs) {
    let c: ScreenCfg =
        serde_json::from_str(&std::fs::read_to_string(&a.config).unwrap()).expect("config");
    let pool = Pool::load_dir(&c.dir);
    let model = Model::starting();
    let tier = Tier::get(c.tier);
    let cfg = plain_cfg(&pool, &model, [tier, tier]);
    let ids: Vec<String> = if c.ids.is_empty() {
        pool.defs.iter().map(|d| d.id.clone()).collect()
    } else {
        c.ids.clone()
    };
    let t0 = Instant::now();
    let stats: Vec<ScreenStat> = ids
        .par_iter()
        .map(|id| {
            let d = pool.idx(id);
            let comp = rsim::sigil::compile(&pool.defs[d].effect, None).unwrap();
            let mut st = ScreenStat {
                id: id.clone(),
                rounds: c.rounds,
                ..Default::default()
            };
            let mut sets = 0;
            for (variant, cards) in [(0, vec![]), (1, matching_cards(&comp))] {
                if variant == 1 && (!c.committed || cards.is_empty()) {
                    continue;
                }
                let (mut fired, mut fires) = (0u64, 0u64);
                for k in 0..c.rounds {
                    let seed = rsim::rng::derive(c.seed, &[k]);
                    let rr =
                        Runner::new(&cfg, seed, (k % 2) as u8).screen_round(&[d], &cards, c.round);
                    let f = rr.sig.iter().map(|x| x.f as u64).sum::<u64>();
                    fires += f;
                    fired += (f > 0) as u64;
                    if variant == 0 {
                        st.mean_score += rr.score / c.rounds as f64;
                        sets += (!rr.made && rr.contract > 0) as u64;
                        st.mean_score_without += without(&rr) / c.rounds as f64;
                    }
                }
                let n = c.rounds as f64;
                if variant == 0 {
                    st.fire = fired as f64 / n;
                    st.fires = fires as f64 / n;
                } else {
                    st.committed_fire = Some(fired as f64 / n);
                    st.committed_fires = Some(fires as f64 / n);
                }
            }
            st.set_rate = sets as f64 / c.rounds as f64;
            st
        })
        .collect();
    std::fs::write(&c.out, serde_json::to_string_pretty(&stats).unwrap()).unwrap();
    println!(
        "screened {} sigils x {} rounds in {:.1}s -> {}",
        stats.len(),
        c.rounds,
        t0.elapsed().as_secs_f64(),
        c.out
    );
}

/// A round's score with every sigil's ledger removed (engravings kept).
fn without(rr: &RoundRec) -> f64 {
    let nil_failed = rr.nil_bids - rr.nil_made;
    let nil = rr.nil_made as f64 * 100.0 - nil_failed as f64 * 100.0;
    let base = if rr.contract == 0 {
        0.0
    } else if rr.made {
        10.0 * rr.contract as f64 + rr.eng_cp
    } else {
        -10.0 * rr.contract as f64
    };
    ((base + nil) * (10.0 + rr.eng_add)).round()
}

pub fn enumerate(dir: &str, step: &str) {
    let suits = ["C", "D", "H", "S"];
    let ranks = ["A", "K", "Q", "J", "10", "2"];
    let mut nouns: Vec<(String, Value)> = vec![];
    for s in suits {
        nouns.push((format!("suit-{}", s.to_lowercase()), json!({"suit": s})));
    }
    for r in ranks {
        nouns.push((format!("rank-{}", r.to_lowercase()), json!({"rank": r})));
    }
    nouns.push(("low".into(), json!({"ranks": ["2", "10"]})));
    nouns.push(("side".into(), json!({"notSuit": "S"})));
    let mut conds: Vec<(String, Value, Vec<&str>)> = vec![];
    let arch_of = |n: &str| -> Vec<&'static str> {
        if n.starts_with("suit-s") {
            vec!["Spades"]
        } else if n.starts_with("suit") {
            vec!["Suits"]
        } else if n == "low" || n == "rank-2" {
            vec!["LowCards"]
        } else if n == "side" {
            vec!["Suits", "Rainbow"]
        } else {
            vec!["Ranks"]
        }
    };
    for (n, f) in &nouns {
        conds.push((
            format!("win-{n}"),
            json!({"event": "win", "card": f}),
            arch_of(n),
        ));
        conds.push((
            format!("lead-{n}"),
            json!({"event": "lead", "card": f}),
            arch_of(n),
        ));
        conds.push((
            format!("hold-{n}"),
            json!({"event": "hold", "card": f}),
            arch_of(n),
        ));
        conds.push((
            format!("win3-{n}"),
            json!({"event": "win", "card": f, "count": 3}),
            arch_of(n),
        ));
    }
    let other: Vec<(&str, Value, Vec<&str>)> = vec![
        ("win", json!({"event": "win"}), vec!["Generic"]),
        (
            "trump",
            json!({"event": "win", "by": "trump"}),
            vec!["Spades"],
        ),
        (
            "trump3",
            json!({"event": "win", "by": "trump", "count": 3}),
            vec!["Spades"],
        ),
        ("bid8", json!({"event": "bid", "min": 8}), vec!["BidHigh"]),
        ("bid4less", json!({"event": "bid", "max": 4}), vec!["Exact"]),
        ("bidnil", json!({"event": "bid", "nil": true}), vec!["Nil"]),
        ("make", json!({"event": "make"}), vec!["Generic"]),
        (
            "exact",
            json!({"event": "make", "exact": true}),
            vec!["Exact"],
        ),
        ("make8", json!({"event": "make", "min": 8}), vec!["BidHigh"]),
        ("nilmade", json!({"event": "nilMade"}), vec!["Nil"]),
        (
            "last",
            json!({"event": "win", "trick": "last"}),
            vec!["Streaks"],
        ),
        (
            "first",
            json!({"event": "win", "trick": "first"}),
            vec!["Streaks"],
        ),
        (
            "consecutive",
            json!({"event": "win", "consecutive": true}),
            vec!["Streaks"],
        ),
        ("row3", json!({"event": "win", "inRow": 3}), vec!["Streaks"]),
        (
            "distinct",
            json!({"event": "win", "distinct": "suit"}),
            vec!["Rainbow"],
        ),
        (
            "suits-win",
            json!({"event": "suits", "action": "win", "count": 4}),
            vec!["Rainbow"],
        ),
        (
            "suits-lead",
            json!({"event": "suits", "action": "lead", "count": 4}),
            vec!["Rainbow"],
        ),
        ("oppset", json!({"event": "opponentsSet"}), vec!["Generic"]),
    ];
    for (n, t, a) in other {
        conds.push((n.to_string(), t, a));
    }
    let mut count = 0;
    for (cn, trig, arch) in &conds {
        for (ty, amt) in [("points", 1.0), ("mult", 1.0), ("xmult", 1.5)] {
            let id = format!("e-{cn}-{}", if ty == "points" { "pts" } else { ty });
            let eff = json!({"type": ty, "amount": amt, "on": trig});
            let v = json!({
                "id": id, "rarity": "common", "role": "payoff", "archetypes": arch, "source": "enumerated",
                "createdIn": step, "effect": eff,
                "design": {"decision": "", "opponent": "", "rationale": "Enumerated from the grammar (D21).", "partners": []},
                "status": "candidate", "replacedBy": null,
                "history": [{"step": step, "note": "Enumerated; amount set from the hand-level screen."}],
                "name": null, "icon": null, "iconFamily": null, "iconWord": null
            });
            let p = std::path::Path::new(dir).join(format!("{id}.json"));
            if !p.exists() {
                crate::write_json(&p, &v);
                count += 1;
            }
        }
    }
    for (id, ty, eff, arch) in [
        (
            "e-contract-pts",
            "points",
            json!({"type": "points", "amount": 1, "per": "contractTrick"}),
            "BidHigh",
        ),
        (
            "e-contract-mult",
            "mult",
            json!({"type": "mult", "amount": 1, "per": "contractTrick"}),
            "BidHigh",
        ),
        (
            "e-nil-points",
            "nilPoints",
            json!({"type": "nilPoints", "amount": 1}),
            "Nil",
        ),
    ] {
        let _ = ty;
        let v = json!({
            "id": id, "rarity": "common", "role": "payoff", "archetypes": [arch], "source": "enumerated",
            "createdIn": step, "effect": eff,
            "design": {"decision": "", "opponent": "", "rationale": "Enumerated from the grammar (D21).", "partners": []},
            "status": "candidate", "replacedBy": null,
            "history": [{"step": step, "note": "Enumerated; amount set from the hand-level screen."}],
            "name": null, "icon": null, "iconFamily": null, "iconWord": null
        });
        let p = std::path::Path::new(dir).join(format!("{id}.json"));
        if !p.exists() {
            crate::write_json(&p, &v);
            count += 1;
        }
    }
    println!(
        "enumerated {count} new common candidates ({} conditions)",
        conds.len()
    );
}
