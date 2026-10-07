//! Rules text generated from sigil data, its lint, and the effect signature.

use crate::cards::*;
use crate::sigil::*;

pub fn number_word(n: u8) -> String {
    const W: [&str; 14] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen",
    ];
    W.get(n as usize)
        .map(|s| s.to_string())
        .unwrap_or_else(|| n.to_string())
}

fn cap(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

pub fn fmt_num(x: f64) -> String {
    if (x - x.round()).abs() < 1e-9 {
        format!("{}", x.round() as i64)
    } else {
        let s = format!("{:.2}", x);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

fn rank_token(r: &str) -> String {
    format!("[{r}]")
}
fn suit_token(s: &str) -> String {
    format!("[{}]", suit_symbol(suit_from_str(s).unwrap_or(0)))
}
fn article(rank: &str) -> &'static str {
    if rank == "A" || rank == "8" {
        "an"
    } else {
        "a"
    }
}

/// "an [A]", "[♦]", "a [2] through [10]", "the [A♠]".
fn singular(f: &Filter) -> String {
    match (&f.suit, &f.rank, &f.ranks, &f.not_suit) {
        (Some(s), Some(r), _, _) => {
            format!("the [{}{}]", r, suit_symbol(suit_from_str(s).unwrap_or(0)))
        }
        (Some(s), None, Some([a, b]), _) => {
            format!(
                "{} {} through {} of {}",
                article(a),
                rank_token(a),
                rank_token(b),
                suit_token(s)
            )
        }
        (Some(s), None, None, _) => suit_token(s),
        (None, Some(r), _, _) => format!("{} {}", article(r), rank_token(r)),
        (None, None, Some([a, b]), _) => {
            format!("{} {} through {}", article(a), rank_token(a), rank_token(b))
        }
        (None, None, None, Some(n)) => format!("a card other than {}", suit_token(n)),
        _ => "a card".into(),
    }
}

/// "[A]s", "[♦]", "[2]s through [10]s".
fn plural(f: &Filter, suit_s: bool) -> String {
    let ss = if suit_s { "s" } else { "" };
    match (&f.suit, &f.rank, &f.ranks, &f.not_suit) {
        (Some(s), Some(r), _, _) => {
            format!("[{}{}]s", r, suit_symbol(suit_from_str(s).unwrap_or(0)))
        }
        (Some(s), None, Some([a, b]), _) => {
            format!(
                "{}s through {}s of {}",
                rank_token(a),
                rank_token(b),
                suit_token(s)
            )
        }
        (Some(s), None, None, _) => format!("{}{ss}", suit_token(s)),
        (None, Some(r), _, _) => format!("{}s", rank_token(r)),
        (None, None, Some([a, b]), Some(n)) => {
            format!(
                "{}s through {}s other than {}",
                rank_token(a),
                rank_token(b),
                suit_token(n)
            )
        }
        (None, None, Some([a, b]), _) => format!("{}s through {}s", rank_token(a), rank_token(b)),
        (None, None, None, Some(n)) => format!("cards other than {}", suit_token(n)),
        _ => "cards".into(),
    }
}

/// The noun counted by "for each X your team holds".
fn hold_noun(f: &Filter) -> String {
    match (&f.suit, &f.rank, &f.ranks, &f.not_suit) {
        (Some(s), Some(r), _, _) => {
            format!("[{}{}]", r, suit_symbol(suit_from_str(s).unwrap_or(0)))
        }
        (Some(s), None, None, _) => suit_token(s),
        (None, Some(r), _, _) => rank_token(r),
        (None, None, Some([a, b]), _) => format!("{} through {}", rank_token(a), rank_token(b)),
        (Some(s), None, Some([a, b]), _) => {
            format!(
                "{} through {} of {}",
                rank_token(a),
                rank_token(b),
                suit_token(s)
            )
        }
        (None, None, None, Some(n)) => format!("card other than {}", suit_token(n)),
        _ => "card".into(),
    }
}

fn benefit(kind: &str, amount: f64) -> String {
    match kind {
        "points" => format!("+{} contract points", fmt_num(amount)),
        "mult" => format!("+{} contract multiplier", fmt_num(amount)),
        "xmult" => format!("×{} contract multiplier", fmt_num(amount)),
        "nilPoints" => format!("+{} nil points", fmt_num(amount)),
        _ => "?".into(),
    }
}

/// The trigger as a clause after "when"/"if", plus which connective it takes.
/// Returns (connective, clause), e.g. ("when", "your team wins a trick with an [A]").
fn clause(t: &Trigger) -> (&'static str, String) {
    let card = t.card.as_ref();
    match t.event.as_str() {
        "win" => {
            if let Some(n) = t.in_row {
                return (
                    "when",
                    format!("your team wins {} tricks in a row", number_word(n)),
                );
            }
            if t.consecutive == Some(true) {
                return ("when", "your team wins consecutive tricks".into());
            }
            if t.distinct.as_deref() == Some("suit") {
                return (
                    "when",
                    "your team wins its first trick with each suit".into(),
                );
            }
            if let Some(n) = t.count {
                let how = if t.by.as_deref() == Some("trump") {
                    " by trumping".to_string()
                } else if let Some(f) = card {
                    format!(" with {}", plural(f, false))
                } else {
                    String::new()
                };
                return (
                    "when",
                    format!("your team wins {} tricks{how}", number_word(n)),
                );
            }
            if t.whose.as_deref() == Some("opponents") {
                let how = card
                    .map(|f| format!(" with {}", singular(f)))
                    .unwrap_or_default();
                return ("when", format!("the opponents win a trick{how}"));
            }
            if let Some(l) = &t.led {
                return (
                    "when",
                    format!("your team wins a trick led with {}", singular(l)),
                );
            }
            let which = match t.trick.as_deref() {
                Some("last") => "the last trick of a round",
                Some("first") => "the first trick of a round",
                _ => "a trick",
            };
            let how = if t.by.as_deref() == Some("trump") {
                " by trumping".to_string()
            } else if let Some(f) = card {
                format!(" with {}", singular(f))
            } else {
                String::new()
            };
            ("when", format!("your team wins {which}{how}"))
        }
        "lead" => {
            let f = card.cloned().unwrap_or_default();
            match t.count {
                Some(n) => (
                    "when",
                    format!("your team leads {} {} times", singular(&f), number_word(n)),
                ),
                None => ("when", format!("your team leads {}", singular(&f))),
            }
        }
        "hold" => {
            let f = card.cloned().unwrap_or_default();
            match t.count {
                Some(n) => (
                    "if",
                    format!("your team holds {} {}", number_word(n), plural(&f, true)),
                ),
                None => ("for each", format!("{} your team holds", hold_noun(&f))),
            }
        }
        "bid" => {
            if t.vs.as_deref() == Some("opponents") {
                ("when", "your team bids more than the opponents".into())
            } else if t.nil == Some(true) {
                ("when", "your team bids nil".into())
            } else if let Some(m) = t.min {
                ("when", format!("your team bids {m} or more"))
            } else if let Some(m) = t.max {
                ("when", format!("your team bids {m} or less"))
            } else {
                ("when", "your team bids".into())
            }
        }
        "make" => {
            if t.exact == Some(true) {
                ("if", "your team makes its contract exactly".into())
            } else if let Some(m) = t.min {
                ("if", format!("your team makes a contract of {m} or more"))
            } else {
                ("if", "your team makes its contract".into())
            }
        }
        "nilMade" => ("if", "your team makes a nil".into()),
        "suits" => {
            let n = t.count.unwrap_or(4);
            let suits = if n == 4 {
                "all four suits".to_string()
            } else {
                format!("{} suits", number_word(n))
            };
            if t.action.as_deref() == Some("lead") {
                ("if", format!("your team leads {suits}"))
            } else {
                ("if", format!("your team wins tricks with {suits}"))
            }
        }
        "opponentsSet" => ("if", "the opponents miss their contract".into()),
        "behind" => ("if", "your team is behind when the round begins".into()),
        x => ("when", format!("<{x}>")),
    }
}

pub fn generate(e: &Effect) -> String {
    match e.ty.as_str() {
        "points" | "mult" | "xmult" | "nilPoints" => {
            let b = benefit(&e.ty, e.amount.unwrap_or(0.0));
            let mut s = b;
            if e.per.as_deref() == Some("contractTrick") {
                s += " for each trick in your team's contract";
            }
            if let Some(t) = &e.on {
                let (conn, cl) = clause(t);
                s += &format!(" {conn} {cl}");
                if let Some(n) = t.beyond {
                    s += &format!(" beyond {}", number_word(n));
                }
            }
            s
        }
        "grow" => {
            let kind = e.kind.as_deref().unwrap_or("mult");
            let step = benefit(kind, e.step.unwrap_or(0.0));
            let (_, cl) = clause(e.on.as_ref().unwrap_or(&Trigger::default()));
            let current = if kind == "xmult" { "×1" } else { "+0" };
            format!("This sigil gains {step} every time {cl} (currently {current})")
        }
        "swap" => {
            let n = e.count.unwrap_or(0);
            let unit = if n == 1 { "card" } else { "cards" };
            format!("Opening: Swap {} {unit} with your partner", number_word(n))
        }
        "become"
            if e.whose.as_deref() == Some("opponents") && e.from.is_some() && e.count.is_none() =>
        {
            let from = e.from.clone().unwrap_or_default();
            let dest = if let Some(r) = &e.rank {
                format!("{} {}", article(r), rank_token(r))
            } else {
                format!("a {}", suit_token(e.suit.as_deref().unwrap_or("S")))
            };
            format!(
                "Opening: Every {} the opponents hold becomes {dest}",
                hold_noun(&from)
            )
        }
        "become" if e.whose.as_deref() == Some("opponents") => {
            let n = e.count.unwrap_or(0);
            let dest = if let Some(r) = &e.rank {
                if n == 1 {
                    format!("{} {}", article(r), rank_token(r))
                } else {
                    format!("{}s", rank_token(r))
                }
            } else if let Some(su) = &e.suit {
                if n == 1 {
                    format!("a {}", suit_token(su))
                } else {
                    format!("{}s", suit_token(su))
                }
            } else {
                "cards".into()
            };
            let (cards, verb) = if n == 1 {
                ("card", "becomes")
            } else {
                ("cards", "become")
            };
            format!(
                "Opening: {} {cards} the opponents hold {verb} {dest}",
                cap(&number_word(n))
            )
        }
        "become" if e.from.is_some() => {
            let from = e.from.clone().unwrap_or_default();
            let (dest1, destn) = if let Some(r) = &e.rank {
                (
                    format!("{} {}", article(r), rank_token(r)),
                    format!("{}s", rank_token(r)),
                )
            } else if let Some(su) = &e.suit {
                (
                    format!("a {}", suit_token(su)),
                    format!("{}s", suit_token(su)),
                )
            } else {
                ("a card".into(), "cards".into())
            };
            match e.count {
                None | Some(0) => format!(
                    "Opening: Every {} your team holds becomes {dest1}",
                    hold_noun(&from)
                ),
                Some(n) => format!(
                    "Opening: {} {} your team holds become {destn}",
                    cap(&number_word(n)),
                    plural(&from, true)
                ),
            }
        }
        "become" if e.count == Some(1) => {
            let dest = if let Some(r) = &e.rank {
                format!("{} {}", article(r), rank_token(r))
            } else if let Some(su) = &e.suit {
                format!("a {}", suit_token(su))
            } else {
                e.term.clone().unwrap_or_else(|| "a low card".into())
            };
            format!("Opening: One card your team holds becomes {dest}")
        }
        "become" => {
            let n = cap(&number_word(e.count.unwrap_or(0)));
            let dest = if let Some(r) = &e.rank {
                format!("{}s", rank_token(r))
            } else if let Some(s) = &e.suit {
                format!("{}s", suit_token(s))
            } else {
                e.term.clone().unwrap_or_else(|| "low cards".into())
            };
            format!("Opening: {n} cards your team holds become {dest}")
        }
        "raise" if e.from.is_some() => {
            let n = e.amount.unwrap_or(0.0) as u8;
            let unit = if n == 1 { "rank" } else { "ranks" };
            format!(
                "Opening: Raise every {} your team holds by {} {unit}",
                hold_noun(&e.from.clone().unwrap_or_default()),
                number_word(n)
            )
        }
        "anySuit" if e.card.is_some() => format!(
            "Your team can play {} even when it can follow suit",
            plural(&e.card.clone().unwrap_or_default(), true)
        ),
        "beats" if e.over.as_deref() == Some("trump") => format!(
            "Your team's {} win every trick they are played to",
            plural(&e.card.clone().unwrap_or_default(), true)
        ),
        "beats" => format!(
            "Your team's {} beat every other card of their suit",
            plural(&e.card.clone().unwrap_or_default(), true)
        ),
        "untrumpable" => format!(
            "Your team's {} can't be trumped",
            plural(&e.from.clone().unwrap_or_default(), true)
        ),
        "raise" => {
            let n = e.amount.unwrap_or(0.0) as u8;
            let unit = if n == 1 { "rank" } else { "ranks" };
            format!(
                "Opening: Raise every card your team holds by {} {unit}",
                number_word(n)
            )
        }
        "leadChoice" => "Choose which partner leads after your team wins a trick".into(),
        "anySuit" if e.after.as_deref() == Some("contractMade") => {
            "Your team can play any suit once it has won the tricks in its contract".into()
        }
        "anySuit" if e.first.is_some() => format!(
            "Your team can play any suit on the first {} tricks",
            number_word(e.first.unwrap_or(0))
        ),
        "leadSpades" => "Your team can lead [♠] before [♠]s are broken".into(),
        "firstLead" => "Your team leads the first trick of a round".into(),
        "anySuit" => format!(
            "Your team can play any suit on the last {} tricks",
            number_word(e.last.unwrap_or(0))
        ),
        x => format!("<unknown effect {x}>"),
    }
}

/// Lint findings for generated text: the §5 conventions that construction doesn't guarantee.
pub fn lint(text: &str, e: &Effect) -> Vec<String> {
    let mut out = vec![];
    let lower = text.to_lowercase();
    let first = text.chars().next().unwrap_or(' ');
    let opening = matches!(e.ty.as_str(), "swap" | "become" | "raise");
    if opening && !text.starts_with("Opening: ") {
        out.push("before-bidding effect must start with Opening:".into());
    }
    if e.is_payoff() && e.ty != "grow" && first != '+' && first != '×' {
        out.push("payoff must lead with its amount".into());
    }
    for bad in [
        " you ",
        " your bid",
        "random",
        "each round",
        "before bidding",
        " add ",
        "gain ",
        " bags",
    ] {
        if lower.contains(bad) && !(bad == "gain " && e.ty == "grow") {
            out.push(format!("forbidden wording: {}", bad.trim()));
        }
    }
    if e.on.is_some() && !lower.contains("your team") && !lower.contains("the opponents") {
        out.push("must name \"your team\"".into());
    }
    if text.matches('[').count() != text.matches(']').count() {
        out.push("unbalanced brackets".into());
    }
    if text.contains("+") && text.contains("×") && e.ty == "mult" {
        out.push("additive multipliers use +N, never +N×".into());
    }
    if text.contains("<") {
        out.push("unrenderable effect".into());
    }
    if let Some(t) = &e.on {
        if t.beyond.is_some() {
            out.push("\"beyond N\" arithmetic is excluded from the shipped pool".into());
        }
    }
    if e.term.is_some() {
        out.push("invented shorthand term".into());
    }
    out
}

fn filter_sig(f: &Option<Filter>, generic: bool) -> String {
    let Some(f) = f else { return "-".into() };
    let mut parts = vec![];
    if let Some(s) = &f.suit {
        parts.push(format!("suit={}", if generic { "*" } else { s }));
    }
    if let Some(r) = &f.rank {
        parts.push(format!("rank={}", if generic { "*" } else { r }));
    }
    if let Some([a, b]) = &f.ranks {
        parts.push(format!("ranks={a}-{b}"));
    }
    if let Some(s) = &f.not_suit {
        parts.push(format!("notSuit={s}"));
    }
    if parts.is_empty() {
        "-".into()
    } else {
        parts.join(",")
    }
}

fn trigger_sig(t: &Option<Trigger>) -> String {
    let Some(t) = t else { return "always".into() };
    let mut mods = vec![];
    if let Some(b) = &t.by {
        mods.push(format!("by={b}"));
    }
    if t.led.is_some() {
        mods.push(format!("led={}", filter_sig(&t.led, false)));
    }
    if let Some(x) = &t.trick {
        mods.push(format!("trick={x}"));
    }
    if t.consecutive == Some(true) {
        mods.push("consecutive".into());
    }
    if let Some(n) = t.in_row {
        mods.push(format!("inRow={n}"));
    }
    if let Some(d) = &t.distinct {
        mods.push(format!("distinct={d}"));
    }
    if let Some(n) = t.count {
        mods.push(format!("count={n}"));
    }
    if let Some(n) = t.min {
        mods.push(format!("min={n}"));
    }
    if let Some(n) = t.max {
        mods.push(format!("max={n}"));
    }
    if t.nil == Some(true) {
        mods.push("nil".into());
    }
    if let Some(w) = &t.whose {
        mods.push(format!("whose={w}"));
    }
    if let Some(v) = &t.vs {
        mods.push(format!("vs={v}"));
    }
    if t.exact == Some(true) {
        mods.push("exact".into());
    }
    if let Some(a) = &t.action {
        mods.push(format!("action={a}"));
    }
    if let Some(n) = t.beyond {
        mods.push(format!("beyond={n}"));
    }
    if mods.is_empty() {
        t.event.clone()
    } else {
        format!("{}({})", t.event, mods.join(","))
    }
}

/// trigger | filter | scaling | effect type. With `generic`, named suits and ranks become `*`,
/// so near-duplicates share a family signature.
pub fn signature(e: &Effect, generic: bool) -> String {
    let card = e.on.as_ref().and_then(|t| t.card.clone());
    match e.ty.as_str() {
        "points" | "mult" | "xmult" | "nilPoints" => format!(
            "{}|{}|{}|{}",
            trigger_sig(&e.on),
            filter_sig(&card, generic),
            e.per.clone().unwrap_or_else(|| "-".into()),
            e.ty
        ),
        "grow" => format!(
            "{}|{}|growth|{}",
            trigger_sig(&e.on),
            filter_sig(&card, generic),
            e.kind.clone().unwrap_or_default()
        ),
        "become" if e.from.is_some() => {
            let dest = if let Some(r) = &e.rank {
                format!(
                    "rank={}{}",
                    if generic { "*" } else { r },
                    if e.whose.is_some() { ",opponents" } else { "" }
                )
            } else {
                format!("suit={}", e.suit.clone().unwrap_or_default())
            };
            format!(
                "opening|from:{}|{}|become",
                filter_sig(&e.from, generic),
                dest
            )
        }
        "become" => {
            let dest = if let Some(r) = &e.rank {
                format!("rank={}", if generic { "*" } else { r })
            } else if let Some(s) = &e.suit {
                format!("suit={}", if generic { "*" } else { s })
            } else {
                "range".into()
            };
            let whose = if e.whose.as_deref() == Some("opponents") {
                "opponents"
            } else {
                "opening"
            };
            format!("{whose}|{dest}|-|become")
        }
        "raise" if e.from.is_some() => {
            format!("opening|from:{}|-|raise", filter_sig(&e.from, generic))
        }
        "untrumpable" => format!("play|{}|-|untrumpable", filter_sig(&e.from, generic)),
        "beats" => format!(
            "play|{}|{}|beats",
            filter_sig(&e.card, generic),
            e.over.clone().unwrap_or_default()
        ),
        "swap" | "raise" => format!("opening|-|-|{}", e.ty),
        "anySuit" => {
            let w = if e.card.is_some() {
                format!("cards:{}", filter_sig(&e.card, generic))
            } else if let Some(n) = e.last {
                format!("last={n}")
            } else if let Some(n) = e.first {
                format!("first={n}")
            } else {
                format!("after={}", e.after.clone().unwrap_or_default())
            };
            format!("play|{w}|-|anySuit")
        }
        x => format!("play|-|-|{x}"),
    }
}
