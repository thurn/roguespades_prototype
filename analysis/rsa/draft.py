"""Draft-pass helpers: designer intake, enumerated amounts, screening, and keep decisions."""

import json
import math
import re

from .analyze import nice
from .common import SIGILS, history, load_sigils, rsim, save_sigil


def parse_designer(text: str) -> list:
    """Extracts the JSON array from a designer's reply."""
    m = re.search(r"\[\s*\{.*\}\s*\]", text, re.S)
    if not m:
        raise ValueError("no JSON array in designer output")
    raw = m.group(0)
    # Designers occasionally close one brace too many before "draftText"; repair that case.
    for _ in range(20):
        try:
            return json.loads(raw)
        except json.JSONDecodeError as e:
            cut = raw.rfind("}", 0, e.pos + 1)
            if cut < 0:
                raise
            raw = raw[:cut] + raw[cut + 1 :]
    return json.loads(raw)


def intake(cands: list, step: str, source: str = "designed") -> list:
    """Writes designer candidates as data files. Returns (id, error) for ones that failed."""
    existing = load_sigils()
    written = []
    for c in cands:
        sid = c["id"]
        if sid in existing:
            sid = sid + "-2"
        d = {
            "id": sid,
            "rarity": c.get("rarity", "common"),
            "role": c.get("role", "payoff"),
            "archetypes": c.get("archetypes", ["Generic"]),
            "source": source,
            "createdIn": step,
            "effect": c["effect"],
            "design": {
                "decision": c.get("decision", ""),
                "opponent": c.get("opponent", ""),
                "rationale": c.get("rationale", ""),
                "partners": c.get("partners", []),
                "cluster": c.get("cluster"),
                "draftText": c.get("draftText"),
                "hooks": c.get("hooks", "none"),
            },
            "status": "candidate",
            "replacedBy": None,
            "history": [{"step": step, "note": f"Designed (cluster {c.get('cluster')})."}],
            "name": None,
            "icon": None,
            "iconFamily": None,
            "iconWord": None,
        }
        save_sigil(d)
        written.append(sid)
    out = rsim("gen-text", "--dir", "../data/sigils", quiet=True)
    errors = [ln for ln in out.splitlines() if ln.startswith("ERROR")]
    return written, errors


def screen_amount(d: dict, st: dict) -> float | None:
    """Sets an enumerated candidate's amount from its hand-level screen so its expected
    contribution matches the common budget."""
    e = d["effect"]
    t = e["type"]
    fires = max(st["fires"], 1e-3)
    if e.get("per") == "contractTrick":
        return nice(45 / 7, "points", "points") if t == "points" else nice(6 / 7, "mult", "mult")
    if t == "points":
        return min(150, nice(45 / fires, "points", "points"))
    if t == "mult":
        return min(40, nice(7 / fires, "mult", "mult"))
    if t == "xmult":
        return max(1.25, min(3.0, nice(math.exp(0.37 / fires), "xmult", "xmult")))
    if t == "nilPoints":
        return 50
    return None


def set_aside(st: dict, d: dict) -> str | None:
    """The screen sets aside candidates that almost never fire, or whose boolean condition almost
    always holds (a stat stick with extra text)."""
    e = d["effect"]
    on = e.get("on") or {}
    if not on:
        return None
    if st["fire"] < 0.03:
        return f"fires in {100 * st['fire']:.1f}% of rounds"
    boolean = (
        on.get("event") in ("make", "bid", "nilMade", "suits", "opponentsSet")
        or "count" in on
        or "inRow" in on
    )
    if boolean and st["fire"] > 0.97:
        return f"its condition holds in {100 * st['fire']:.1f}% of rounds"
    return None


def cut(d: dict, step: str, note: str) -> None:
    d["status"] = "cut"
    history(d, step, note)
    save_sigil(d)


def all_files() -> list:
    return sorted(p.stem for p in SIGILS.glob("*.json"))
