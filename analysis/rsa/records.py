"""Run records: streaming load, ledger rescoring, per-sigil counters, and Parquet tables."""

from collections import defaultdict
from dataclasses import dataclass, field
from pathlib import Path

import orjson
import pyarrow as pa
import pyarrow.parquet as pq

from .common import tags

RULES: dict = {}


def set_rules(path=None) -> None:
    """The rules file the records were played under (default: data/rules.json)."""
    from .common import load_rules

    RULES.clear()
    RULES.update(load_rules(path))


def round_score(cp, add, x, nilp, contract, made, nil_bids, nil_made, tricks=None) -> float:
    if not RULES:
        set_rules()
    R = RULES
    nil = nil_made * (R["nilValue"] + nilp) - (nil_bids - nil_made) * R["nilValue"]
    add = add * R["multRewardScale"]
    if contract == 0:
        base = 0.0
    elif made:
        base = R["trickValue"] * contract + cp
    else:
        base = -(R["trickValue"] * contract + cp)
        if R.get("flatSet"):
            return round(-R["trickValue"] * 10 * contract + nil * ((R["baseMult"] + add) * x))
    return round((base + nil) * ((R["baseMult"] + add) * x))


def rescore(rd: dict, drop: int | None) -> float:
    cp, add, x, nilp = rd["eng_cp"], rd["eng_add"], 1.0, 0.0
    for s in rd["sig"]:
        if s["i"] == drop:
            continue
        cp += s["cp"]
        add += s["add"]
        x *= s["x"]
        nilp += s["nilp"]
    return rd.get("bag_pen", 0.0) + round_score(
        cp, add, x, nilp, rd["contract"], rd["made"], rd["nil_bids"], rd["nil_made"], rd["tricks"]
    )


@dataclass
class TeamRun:
    final: float
    # (sid, shop, ratio, held fraction, coherence with other grants)
    grants: list
    card_grants: list
    perturb: list
    # every sigil held at any point: (sid, shop, sold, grant)
    held: list
    cards: list
    round_scores: list
    sets: int
    made: int
    overtricks_late: int
    made_late: int
    rerolls: int
    gold_end: int


@dataclass
class Run:
    board: int
    orient: int
    clean: bool
    margin: float
    win: float
    teams: list
    checks: list


@dataclass
class SigilCounters:
    held_rounds: int = 0
    fired_rounds: int = 0
    held_rounds_c: int = 0
    fired_rounds_c: int = 0
    held_rounds_g0: int = 0
    fired_rounds_g0: int = 0
    wins_held: int = 0
    decisive: int = 0
    fires: int = 0


@dataclass
class Loaded:
    runs: list
    counters: dict = field(default_factory=lambda: defaultdict(SigilCounters))
    rescore_mismatch: int = 0
    rounds_checked: int = 0


def load(paths: list[Path], sigils: dict, parquet_dir: Path | None = None) -> Loaded:
    out = Loaded(runs=[])
    tagmap = {k: tags(v) for k, v in sigils.items()}
    rows_runs, rows_grants, rows_rounds = defaultdict(list), defaultdict(list), defaultdict(list)
    for path in paths:
        with open(path, "rb") as f:
            for line in f:
                r = orjson.loads(line)
                out.runs.append(_ingest(r, out, tagmap, rows_grants, rows_rounds))
                for k, v in (
                    ("board", r["board"]),
                    ("orient", r["orient"]),
                    ("clean", r["clean"]),
                    ("margin", r["margin"]),
                    ("win", r["win"]),
                    ("final_a", r["teams"][0]["final_score"]),
                    ("final_b", r["teams"][1]["final_score"]),
                ):
                    rows_runs[k].append(v)
    if parquet_dir is not None:
        parquet_dir.mkdir(parents=True, exist_ok=True)
        for name, rows in (("runs", rows_runs), ("sigils", rows_grants), ("rounds", rows_rounds)):
            if rows:
                pq.write_table(pa.table(dict(rows)), parquet_dir / f"{name}.parquet")
    return out


def _ingest(r: dict, out: Loaded, tagmap: dict, rows_grants, rows_rounds) -> Run:
    teams = []
    finals = [t["final_score"] for t in r["teams"]]
    for ti, t in enumerate(r["teams"]):
        sigs = t["sigils"]
        grants_idx = [k for k, s in enumerate(sigs) if s["grant"]]
        grants = []
        for k in grants_idx:
            s = sigs[k]
            sold = s["sold"] or 9
            held = (sold - s["shop"]) / (9 - s["shop"])
            mine = tagmap.get(s["id"], set())
            coh = sum(1 for j in grants_idx if j != k and mine & tagmap.get(sigs[j]["id"], set()))
            grants.append((s["id"], s["shop"], s["ratio"], held, coh))
        held_list = [(s["id"], s["shop"], s["sold"] or 9, s["grant"]) for s in sigs]
        # Ledger: fire rates and decisive share.
        rounds = t["rounds"]
        fired = defaultdict(set)
        for rn, rd in enumerate(rounds, start=1):
            out.rounds_checked += 1
            if abs(rescore(rd, None) - rd["score"]) > 1.0:
                out.rescore_mismatch += 1
            for s in rd["sig"]:
                if s["f"] > 0:
                    fired[s["i"]].add(rn)
        won = finals[ti] > finals[1 - ti]
        for k, s in enumerate(sigs):
            c = out.counters[s["id"]]
            sold = s["sold"] or 9
            hr = [rn for rn in range(s["shop"], sold) if rn <= len(rounds)]
            mine = tagmap.get(s["id"], set())
            others = sum(
                1
                for j, o in enumerate(sigs)
                if j != k
                and o["shop"] <= s["shop"] + 2
                and (o["sold"] or 9) > s["shop"]
                and mine & tagmap.get(o["id"], set())
            )
            nf = sum(1 for rn in hr if rn in fired[k])
            c.held_rounds += len(hr)
            c.fired_rounds += nf
            c.fires += s["fires"]
            if others >= 2:
                c.held_rounds_c += len(hr)
                c.fired_rounds_c += nf
            if s["grant"] and others == 0:
                c.held_rounds_g0 += len(hr)
                c.fired_rounds_g0 += nf
            if won and hr:
                c.wins_held += 1
                delta = sum(rounds[rn - 1]["score"] - rescore(rounds[rn - 1], k) for rn in hr)
                if finals[ti] - delta <= finals[1 - ti]:
                    c.decisive += 1
            rows_grants["board"].append(r["board"])
            rows_grants["orient"].append(r["orient"])
            rows_grants["team"].append(ti)
            rows_grants["id"].append(s["id"])
            rows_grants["shop"].append(s["shop"])
            rows_grants["sold"].append(s["sold"] or 0)
            rows_grants["grant"].append(s["grant"])
            rows_grants["ratio"].append(s["ratio"])
            rows_grants["fires"].append(s["fires"])
        sets = made = ot_late = made_late = 0
        for rn, rd in enumerate(rounds, start=1):
            if rd["contract"] > 0:
                if rd["made"]:
                    made += 1
                    if rn >= 6:
                        made_late += 1
                        ot_late += rd["tricks"] - rd["contract"]
                else:
                    sets += 1
            rows_rounds["board"].append(r["board"])
            rows_rounds["orient"].append(r["orient"])
            rows_rounds["team"].append(ti)
            rows_rounds["round"].append(rn)
            rows_rounds["contract"].append(rd["contract"])
            rows_rounds["tricks"].append(rd["tricks"])
            rows_rounds["made"].append(rd["made"])
            rows_rounds["nil_bids"].append(rd["nil_bids"])
            rows_rounds["nil_made"].append(rd["nil_made"])
            rows_rounds["score"].append(rd["score"])
        teams.append(
            TeamRun(
                final=t["final_score"],
                grants=grants,
                card_grants=[tuple(x) for x in t["card_grants"]],
                perturb=[tuple(x) for x in t["perturb"]],
                held=held_list,
                cards=[tuple(x) for x in t["cards"]],
                round_scores=[rd["score"] for rd in rounds],
                sets=sets,
                made=made,
                overtricks_late=ot_late,
                made_late=made_late,
                rerolls=t["rerolls"],
                gold_end=t["gold"][-1] if t["gold"] else 0,
            )
        )
    return Run(
        board=r["seed"],
        orient=r["orient"],
        clean=r["clean"],
        margin=r["margin"],
        win=r["win"],
        teams=teams,
        checks=r.get("checks", []),
    )


def boards(runs: list) -> dict:
    """Groups runs into boards by board id."""
    b = defaultdict(list)
    for r in runs:
        b[r.board].append(r)
    return b
