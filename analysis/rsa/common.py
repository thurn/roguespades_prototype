"""Paths, data files, and the rsim binary."""

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SIGILS = ROOT / "data" / "sigils"
MODELS = ROOT / "data" / "models"
RUNS = ROOT / "runs"
REPORTS = ROOT / "reports"
RSIM = ROOT / "sim" / "target" / "release" / "rsim"

RARITIES = ["common", "uncommon", "rare", "legendary"]
PRICE = {"common": 50, "uncommon": 75, "rare": 100, "legendary": 150}
ODDS = {"common": 0.69, "uncommon": 0.25, "rare": 0.05, "legendary": 0.01}
CONTROL = {r: f"control-{r}" for r in RARITIES}
CARD_CLASSES = [f"{s}-{r}" for s in "XS" for r in ["A", "K", "Q", "J", "10", "low"]]
PAR = [0, 600, 850, 1150, 1600, 2250, 3100, 4300, 6000]


def load_sigils() -> dict[str, dict]:
    out = {}
    for p in sorted(SIGILS.glob("*.json")):
        d = json.loads(p.read_text())
        out[d["id"]] = d
    return out


def save_sigil(d: dict) -> None:
    (SIGILS / f"{d['id']}.json").write_text(json.dumps(d, indent=2, ensure_ascii=False) + "\n")


def rsim(*args: str, quiet: bool = False) -> str:
    """Runs the simulator from sim/ so its relative paths resolve."""
    r = subprocess.run([str(RSIM), *args], cwd=ROOT / "sim", capture_output=True, text=True, check=False)
    if r.returncode != 0:
        raise RuntimeError(f"rsim {' '.join(args)} failed:\n{r.stdout}\n{r.stderr}")
    if not quiet and r.stdout.strip():
        print(r.stdout.strip())
    return r.stdout


def build_rsim() -> None:
    subprocess.run(["cargo", "build", "--release", "-q"], cwd=ROOT / "sim", check=True)


def tags(d: dict) -> set[str]:
    return {a for a in d.get("archetypes", []) if a != "Generic"}


def category(d: dict) -> str:
    return d.get("category") or "points"


def history(d: dict, step: str, note: str) -> None:
    d.setdefault("history", []).append({"step": step, "note": note})
