"""Experiment configs and runs: grant tournaments, calibration, arms, screens."""

import json
from pathlib import Path

from .common import CONTROL, RUNS, rsim

BLANK = {r: f"control-blank-{r}" for r in CONTROL}


def version() -> str:
    """Code version (git HEAD, with a mark when dirty) and pool version (hash of the effects)."""
    import hashlib
    import subprocess

    from .common import ROOT, load_sigils

    head = subprocess.run(
        ["git", "rev-parse", "--short", "HEAD"], cwd=ROOT, capture_output=True, text=True
    ).stdout.strip()
    dirty = subprocess.run(
        ["git", "status", "--porcelain", "sim"], cwd=ROOT, capture_output=True, text=True
    ).stdout.strip()
    pool = hashlib.sha1(
        json.dumps({k: v["effect"] for k, v in sorted(load_sigils().items())}, sort_keys=True).encode()
    ).hexdigest()[:10]
    return f"code {head}{'+' if dirty else ''} pool {pool}"


def write_config(step: str, name: str, cfg: dict) -> Path:
    d = RUNS / step
    d.mkdir(parents=True, exist_ok=True)
    cfg = dict(cfg)
    cfg["name"] = f"{step}/{name}"
    cfg.setdefault("out", str(d / f"{name}.jsonl"))
    cfg.setdefault("dir", "../data/sigils")
    cfg.setdefault("version", version())
    p = d / f"{name}.config.json"
    text = json.dumps(cfg, indent=1)
    if not p.exists() or p.read_text() != text:
        p.write_text(text)
    return p


def run(step: str, name: str, cfg: dict, reuse: bool = True) -> Path:
    p = write_config(step, name, cfg)
    out = Path(json.loads(p.read_text())["out"])
    if reuse and out.exists() and out.stat().st_mtime > p.stat().st_mtime - 1 and _complete(out, cfg):
        print(f"reusing {out}")
        return out
    rsim("run", str(p))
    return out


def _complete(out: Path, cfg: dict) -> bool:
    n = sum(1 for _ in open(out, "rb"))
    return n == 2 * cfg.get("boards", 0)


def tournament(
    step,
    name,
    *,
    offerable,
    measured,
    boards,
    tier=0,
    seed=1,
    model=None,
    board_offset=0,
    pairs=None,
    pair_prob=0.0,
    validate=0.0,
    clean_share=0.1,
    grant_prob=0.5,
    **kw,
) -> Path:
    cfg = {
        "seed": seed,
        "boards": boards,
        "boardOffset": board_offset,
        "tier": tier,
        "model": str(model) if model else None,
        "offerable": sorted(offerable),
        "measured": measured,
        "grantProb": grant_prob,
        "cleanShare": clean_share,
        "pairs": pairs or [],
        "pairProb": pair_prob,
        "validate": validate,
        "arm": {"type": "tournament"},
    }
    cfg.update(kw)
    return run(step, name, cfg)


def commitment(step, archetype, *, offerable, boards, tier=0, seed=7, model=None, counter=None) -> Path:
    cfg = {
        "seed": seed,
        "boards": boards,
        "tier": tier,
        "model": str(model) if model else None,
        "offerable": sorted(offerable),
        "cleanShare": 0.0,
        "perturb": 0.0,
        "arm": {
            "type": "commitment",
            "archetype": archetype,
            "counter": counter or {},
            "counterProb": 0.4 if counter else 0.0,
        },
    }
    return run(step, f"commit-{archetype}", cfg)


def flexible(step, *, offerable, boards, tier=0, seed=8, model=None) -> Path:
    """A flexible-vs-flexible field with no grants: the game as played."""
    cfg = {
        "seed": seed,
        "boards": boards,
        "tier": tier,
        "model": str(model) if model else None,
        "offerable": sorted(offerable),
        "measured": {},
        "grantProb": 0.0,
        "cleanShare": 1.0,
        "perturb": 0.0,
        "arm": {"type": "tournament"},
    }
    return run(step, "field", cfg)


def standalone(step, *, enablers, boards, seed=9, model=None, tier=0) -> Path:
    measured = {e: 1.0 for e in enablers}
    for r in CONTROL:
        measured[CONTROL[r]] = 1.0
        measured[BLANK[r]] = 1.0
    cfg = {
        "seed": seed,
        "boards": boards,
        "tier": tier,
        "model": str(model) if model else None,
        "offerable": [],
        "measured": measured,
        "grantProb": 0.35,
        "grantMaxShop": 5,
        "coherent": 0.0,
        "jitter": False,
        "cleanShare": 0.0,
        "arm": {"type": "standalone"},
    }
    return run(step, "standalone", cfg)


def chaser(step, name, build, *, offerable, boards, tier=1, seed=11, model=None, board_offset=0) -> Path:
    cfg = {
        "seed": seed,
        "boards": boards,
        "boardOffset": board_offset,
        "tier": tier,
        "model": str(model) if model else None,
        "offerable": sorted(offerable),
        "cleanShare": 0.0,
        "perturb": 0.0,
        "arm": {"type": "chaser", "build": build},
    }
    return run(step, name, cfg)


def screen(step, ids, rounds=300, seed=5) -> list:
    d = RUNS / step
    d.mkdir(parents=True, exist_ok=True)
    out = d / "screen.json"
    p = d / "screen.config.json"
    p.write_text(
        json.dumps({"ids": ids, "rounds": rounds, "seed": seed, "out": str(out), "dir": "../data/sigils"})
    )
    rsim("screen", str(p))
    return json.loads(out.read_text())
