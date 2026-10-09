#![allow(
    clippy::needless_range_loop,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
use clap::{Parser, Subcommand};
use rsim::*;
use serde_json::{json, Value};
use std::path::Path;

mod checks;
mod tourney;

#[derive(Parser)]
#[command(name = "rsim", about = "Rogue Spades 2.0 simulator")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Regenerate rules text, category, signature, and simplicity in every data file.
    GenText {
        #[arg(long, default_value = "../data/sigils")]
        dir: String,
    },
    /// Write the compact registry of every candidate.
    Registry {
        #[arg(long, default_value = "../data/sigils")]
        dir: String,
        #[arg(long, default_value = "../docs/sigils/registry.md")]
        out: String,
    },
    /// Print the grammar reference.
    Grammar,
    /// Phase 0a done checks: seed texts, rubric rows, worked examples.
    Check {
        #[arg(long, default_value = "../data/sigils")]
        dir: String,
    },
    /// Random-play benchmark: card plays per second per core.
    BenchRandom {
        #[arg(long, default_value_t = 200000)]
        rounds: u64,
    },
    /// Plain-Spades AI bench (no sigils, no shop).
    BenchPlain(tourney::BenchArgs),
    /// Tier-vs-tier ladder on plain Spades.
    Ladder(tourney::LadderArgs),
    /// Run an experiment from a config file, writing run records.
    Run(tourney::RunArgs),
    /// Hand-level screen: trigger rates on fixed builds over single rounds.
    Screen(tourney::ScreenArgs),
    /// Throughput per tier with the seed pool.
    Throughput(tourney::ThroughputArgs),
    /// Enumerate common payoffs from the grammar.
    Enumerate {
        #[arg(long, default_value = "../data/sigils")]
        dir: String,
        #[arg(long, default_value = "draft-common")]
        step: String,
    },
    /// Play a client session natively: autoplay a game, or replay a game client log.
    Play {
        #[arg(long, default_value_t = 1)]
        seed: u64,
        #[arg(long, default_value_t = 1)]
        tier: u8,
        #[arg(long, default_value = "../data/sigils")]
        dir: String,
        #[arg(long, default_value = "../data/models/search-final.json")]
        model: String,
        /// A game client log (logs/*.jsonl) to replay instead of autoplaying.
        #[arg(long)]
        replay: Option<String>,
        /// Print the table view (all hands) after this step.
        #[arg(long)]
        view_at: Option<u64>,
        /// Also write the session log (JSONL) here.
        #[arg(long)]
        out: Option<String>,
        /// A rules file to play under (default: data/rules.json as compiled in).
        #[arg(long)]
        rules: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::GenText { dir } => gen_text(&dir),
        Cmd::Registry { dir, out } => registry(&dir, &out),
        Cmd::Grammar => print!("{}", GRAMMAR),
        Cmd::Check { dir } => checks::check(&dir),
        Cmd::BenchRandom { rounds } => checks::bench_random(rounds),
        Cmd::BenchPlain(a) => tourney::bench_plain(a),
        Cmd::Ladder(a) => tourney::ladder(a),
        Cmd::Run(a) => tourney::run(a),
        Cmd::Screen(a) => tourney::screen(a),
        Cmd::Throughput(a) => tourney::throughput(a),
        Cmd::Enumerate { dir, step } => tourney::enumerate(&dir, &step),
        Cmd::Play {
            seed,
            tier,
            dir,
            model,
            replay,
            view_at,
            out,
            rules,
        } => {
            if let Some(r) = rules {
                rsim::rules::install(rsim::rules::load(&r));
            }
            play(seed, tier, &dir, &model, replay, view_at, out)
        }
    }
}

const GRAMMAR: &str = "Effect types: points, mult, xmult, nilPoints (payoffs); grow (kind + step);
  swap {count}, become {count, rank|suit}, raise {amount}, leadChoice, anySuit {last}.
Payoff scaling: per = contractTrick.
Trigger events: win {card, by=trump, trick=first|last, consecutive, inRow, distinct=suit, count},
  lead {card, count}, hold {card, count}, bid {min|max|nil}, make {min|exact}, nilMade,
  suits {action=win|lead, count}, opponentsSet.
Card filters: suit C|D|H|S, rank 2..10|J|Q|K|A, ranks [lo, hi], notSuit.
See docs/sigils/data-format.md for text templates.
";

fn play(
    seed: u64,
    tier: u8,
    dir: &str,
    model: &str,
    replay: Option<String>,
    view_at: Option<u64>,
    out: Option<String>,
) {
    use rsim::session::{Config, Session};
    let pool: &'static game::Pool = Box::leak(Box::new(game::Pool::load_dir(dir)));
    let model: &'static model::Model = Box::leak(Box::new(model::Model::load(model)));
    let offerable: Vec<String> = pool
        .defs
        .iter()
        .filter(|d| d.status == "kept")
        .map(|d| d.id.clone())
        .collect();
    let log: Vec<Value> = replay
        .as_deref()
        .map(|p| {
            std::fs::read_to_string(p)
                .unwrap()
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|l| serde_json::from_str(l).unwrap())
                .collect()
        })
        .unwrap_or_default();
    let start = log.iter().find(|e| e["type"] == "start");
    let cfg = Config {
        seed: start.and_then(|e| e["seed"].as_u64()).unwrap_or(seed),
        tier: start
            .and_then(|e| e["tier"].as_u64())
            .map_or(tier, |t| t as u8),
        auto: replay.is_none() || start.and_then(|e| e["auto"].as_bool()).unwrap_or(false),
        offerable,
    };
    let mut s = Session::new(pool, model, cfg);
    let mut lines = String::new();
    let mut print = |s: &mut Session| {
        for e in s.take_events() {
            println!("[{}] {}", e["step"], e["msg"].as_str().unwrap_or(""));
            lines.push_str(&e.to_string());
            lines.push('\n');
        }
        if view_at == Some(s.step) {
            println!("{}", serde_json::to_string_pretty(&s.view(true)).unwrap());
        }
    };
    print(&mut s);
    if replay.is_some() {
        for e in log
            .iter()
            .filter(|e| e.get("action").is_some() && e["type"] != "rejected")
        {
            if let Err(err) = s.act(&e["action"], true) {
                println!(
                    "REPLAY DIVERGED at step {}: {err} (action {})",
                    e["step"], e["action"]
                );
                break;
            }
            print(&mut s);
        }
    } else {
        while s.pending().is_some() {
            if let Err(err) = s.advance() {
                println!("ERROR: {err}");
                break;
            }
            print(&mut s);
        }
    }
    if let Some(o) = out {
        std::fs::write(o, lines).unwrap();
    }
}

pub fn read_json(p: &Path) -> Value {
    serde_json::from_str(&std::fs::read_to_string(p).unwrap())
        .unwrap_or_else(|e| panic!("{}: {e}", p.display()))
}

pub fn write_json(p: &Path, v: &Value) {
    let mut s = serde_json::to_string_pretty(v).unwrap();
    s.push('\n');
    std::fs::write(p, s).unwrap();
}

pub fn data_files(dir: &str) -> Vec<std::path::PathBuf> {
    let mut v: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().and_then(|x| x.to_str()) == Some("json"))
        .collect();
    v.sort();
    v
}

/// Fills the generated fields of one data file value. Returns lint findings.
pub fn derive_fields(v: &mut Value) -> Result<Vec<String>, String> {
    let def: sigil::SigilDef = serde_json::from_value(v.clone()).map_err(|e| e.to_string())?;
    sigil::compile(&def.effect, None)?;
    let text = text::generate(&def.effect);
    let lint = text::lint(&text, &def.effect);
    let simp = simplicity::score(&def.effect, def.rarity);
    let category = match def.role.as_str() {
        "enabler" => "enabler",
        "hybrid" => "hybrid",
        _ => def.effect.category(),
    };
    let o = v.as_object_mut().unwrap();
    o.insert("price".into(), json!(def.price()));
    o.insert("text".into(), json!(text));
    o.insert("category".into(), json!(category));
    o.insert(
        "signature".into(),
        json!(text::signature(&def.effect, false)),
    );
    o.insert("family".into(), json!(text::signature(&def.effect, true)));
    o.insert(
        "touchesOpponents".into(),
        json!(sigil::touches_opponents(&def.effect)),
    );
    o.insert("simplicity".into(), serde_json::to_value(&simp).unwrap());
    o.insert("lint".into(), json!(lint));
    Ok(lint)
}

fn gen_text(dir: &str) {
    let mut bad = 0;
    for p in data_files(dir) {
        let mut v = read_json(&p);
        match derive_fields(&mut v) {
            Ok(lint) => {
                let status = v["status"].as_str().unwrap_or("");
                if !lint.is_empty() && status != "cut" && v["source"] != "control" {
                    println!("{}: {}", p.display(), lint.join("; "));
                }
                write_json(&p, &v);
            }
            Err(e) => {
                bad += 1;
                println!("ERROR {}: {e}", p.display());
            }
        }
    }
    if bad > 0 {
        std::process::exit(1);
    }
}

fn metric(v: &Value, key: &str) -> String {
    let Some(ms) = v["estimates"]["metrics"].as_array() else {
        return String::new();
    };
    for m in ms {
        if m["key"] == key {
            let f = |x: &Value| x.as_f64().map(|x| format!("{x:.1}")).unwrap_or_default();
            if m["lo"].is_null() {
                return f(&m["value"]);
            }
            return format!("{} [{}, {}]", f(&m["value"]), f(&m["lo"]), f(&m["hi"]));
        }
    }
    String::new()
}

fn registry(dir: &str, out: &str) {
    let mut rows: Vec<Value> = data_files(dir).iter().map(|p| read_json(p)).collect();
    let rank = |s: &str| match s {
        "kept" => 0,
        "candidate" => 1,
        "replaced" => 2,
        "cut" => 3,
        _ => 4,
    };
    let rar = |s: &str| match s {
        "common" => 0,
        "uncommon" => 1,
        "rare" => 2,
        _ => 3,
    };
    rows.sort_by_key(|v| {
        (
            rank(v["status"].as_str().unwrap_or("")),
            rar(v["rarity"].as_str().unwrap_or("")),
            v["id"].as_str().unwrap_or("").to_string(),
        )
    });
    let mut s = String::from(
        "# Sigil registry\n\nGenerated by `rsim registry` from `data/sigils/`. Do not edit by hand.\n\n",
    );
    let mut counts = std::collections::BTreeMap::new();
    for v in &rows {
        *counts
            .entry((
                v["status"].as_str().unwrap_or("").to_string(),
                v["rarity"].as_str().unwrap_or("").to_string(),
            ))
            .or_insert(0) += 1;
    }
    s += "| Status | Rarity | Count |\n| --- | --- | --- |\n";
    for ((st, r), n) in &counts {
        s += &format!("| {st} | {r} | {n} |\n");
    }
    s += "\nLift is win-rate points over the same-rarity control with a 90% interval; fire is the share of held rounds the payoff fires.\n\n";
    s += "| Id | Status | Rarity | Name | Icon family | Archetypes | Text | C | Lift | Fire | Signature |\n";
    s += "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n";
    for v in &rows {
        let arch: Vec<String> = v["archetypes"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|x| x.as_str().unwrap_or("").to_string())
                    .collect()
            })
            .unwrap_or_default();
        s += &format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | `{}` |\n",
            v["id"].as_str().unwrap_or(""),
            v["status"].as_str().unwrap_or(""),
            v["rarity"].as_str().unwrap_or(""),
            v["name"].as_str().unwrap_or(""),
            v["iconFamily"].as_str().unwrap_or(""),
            arch.join(", "),
            v["text"].as_str().unwrap_or("").replace('|', "\\|"),
            v["simplicity"]["C"],
            metric(v, "lift"),
            metric(v, "fire"),
            v["signature"].as_str().unwrap_or(""),
        );
    }
    std::fs::write(out, s).unwrap();
    println!("wrote {out} ({} sigils)", rows.len());
}
