"""Design iteration: variants, seed sets, the fun score with an interval for every family, and paired
comparisons of variants on the same boards.

A variant is a JSON spec in `data/search/variants/<name>.json`:

    {"rules": {...overrides of data/rules.json...}, "sigils": {id: {...overrides...}},
     "add": [new sigil defs], "pool": [ids] | null, "model": path | null}

An environment is (tier, utility, seed set, replicate). Every component of the fun score runs on
boards fixed by the environment, so two variants measured in one environment are paired board by
board.
"""

import hashlib
import json
import shutil
from dataclasses import dataclass, field
from pathlib import Path

import numpy as np

from . import arms as R
from .common import MODELS, ROOT, RUNS, SIGILS, load_rules, rsim
from .funscore import FAMILIES, WEIGHTS, families, total

ITER = ROOT / "data" / "search"
VARIANTS = ITER / "variants"
OUT = RUNS / "search"
ARCHES = R.ARCH
DEFAULT_MODEL = str(MODELS / "round-3.json")
COMPONENT = {"field": 1, "ladder": 2, "pairs": 50}


# ---------------------------------------------------------------------------------------------
# Variants and environments


def deep_merge(a: dict, b: dict) -> dict:
    out = json.loads(json.dumps(a))
    for k, v in b.items():
        if isinstance(v, dict) and isinstance(out.get(k), dict):
            out[k] = deep_merge(out[k], v)
        else:
            out[k] = v
    return out


def sigils_at(rev: str) -> dict:
    """The sigil data files at a git revision (cached under runs/iter/git-<rev>)."""
    import subprocess
    import tarfile

    cache = OUT / f"git-{rev}"
    if not cache.exists():
        cache.mkdir(parents=True)
        tar = cache / "s.tar"
        subprocess.run(["git", "archive", "-o", str(tar), rev, "data/sigils"], cwd=ROOT, check=True)
        with tarfile.open(tar) as t:
            t.extractall(cache, filter="data")
        tar.unlink()
    defs = {}
    for p in sorted((cache / "data" / "sigils").glob("*.json")):
        d = json.loads(p.read_text())
        defs[d["id"]] = d
    return defs


@dataclass
class Variant:
    name: str
    spec: dict = field(default_factory=dict)

    @staticmethod
    def load(name: str) -> "Variant":
        p = VARIANTS / f"{name}.json"
        spec = json.loads(p.read_text()) if p.exists() else {}
        return Variant(name, spec)

    def rules(self) -> dict:
        base = load_rules()
        if self.spec.get("base"):
            base = Variant.load(self.spec["base"]).rules()
        return deep_merge(base, self.spec.get("rules", {}))

    def sigil_defs(self) -> dict:
        defs = {}
        base = self.spec.get("base")
        if base:
            defs = Variant.load(base).sigil_defs()
        elif self.spec.get("sigilsFrom"):
            defs = sigils_at(self.spec["sigilsFrom"])
        else:
            for p in sorted(SIGILS.glob("*.json")):
                d = json.loads(p.read_text())
                defs[d["id"]] = d
        for sid, over in self.spec.get("sigils", {}).items():
            defs[sid] = deep_merge(defs[sid], over)
        for d in self.spec.get("add", []):
            defs[d["id"]] = d
        return defs

    def pool(self, defs: dict) -> list:
        if self.spec.get("pool"):
            return sorted(self.spec["pool"])
        return sorted(s for s, d in defs.items() if d.get("status") == "kept")

    def exp(self) -> dict:
        base = Variant.load(self.spec["base"]).exp() if self.spec.get("base") else {}
        return deep_merge(base, self.spec.get("exp", {}))

    def combos(self) -> list:
        base = Variant.load(self.spec["base"]).combos() if self.spec.get("base") else []
        return base + self.spec.get("combos", [])

    def model(self) -> str:
        m = self.spec.get("model")
        if m is None and self.spec.get("base"):
            return Variant.load(self.spec["base"]).model()
        return str(ROOT / m) if m else DEFAULT_MODEL

    def materialize(self) -> "Built":
        """Writes the variant's rules and sigil directory; returns paths and a content hash."""
        d = OUT / "variants" / self.name
        d.mkdir(parents=True, exist_ok=True)
        rules = self.rules()
        defs = self.sigil_defs()
        pool = self.pool(defs)
        rp = d / "rules.json"
        rtext = json.dumps(rules, indent=1)
        if not rp.exists() or rp.read_text() != rtext:
            rp.write_text(rtext)
        sdir = d / "sigils"
        sig_text = {sid: json.dumps(v, ensure_ascii=False) for sid, v in defs.items()}
        h = hashlib.sha1()
        h.update(rtext.encode())
        for sid in sorted(sig_text):
            h.update(sid.encode())
            h.update(json.dumps(defs[sid].get("effect"), sort_keys=True).encode())
            h.update(str(defs[sid].get("status")).encode())
        h.update(json.dumps(pool).encode())
        h.update(self.model().encode())
        h.update(json.dumps(self.exp(), sort_keys=True).encode())
        digest = h.hexdigest()[:12]
        stamp = sdir / ".hash"
        if not stamp.exists() or stamp.read_text() != digest:
            if sdir.exists():
                shutil.rmtree(sdir)
            sdir.mkdir()
            for sid, t in sig_text.items():
                (sdir / f"{sid}.json").write_text(t)
            # Generated fields (text, simplicity) for new or edited sigils.
            rsim("gen-text", "--dir", str(sdir), quiet=True)
            stamp.write_text(digest)
        defs = {json.loads(p.read_text())["id"]: json.loads(p.read_text()) for p in sdir.glob("*.json")}
        return Built(self.name, rp, sdir, pool, defs, self.model(), digest, self.exp(), self.combos())


@dataclass
class Built:
    name: str
    rules: Path
    dir: Path
    pool: list
    defs: dict
    model: str
    digest: str
    exp: dict = field(default_factory=dict)
    combos: list = field(default_factory=list)


@dataclass(frozen=True)
class Env:
    tier: int = 1
    util: str = "wp"  # wp (win probability) | rn (risk-neutral)
    seeds: str = "dev"
    rep: int = 0

    def key(self) -> str:
        return f"{self.seeds}{self.rep}-t{self.tier}-{self.util}"

    def seed(self, comp: int) -> int:
        s = json.loads((ITER / "seeds.json").read_text())[self.seeds]
        block = s.get("block", 0)
        return int(s["base"]) + 100_000 * block + 1000 * self.rep + comp


@dataclass
class Sizes:
    field: int = 1500
    commit: int = 400
    ladder: int = 400
    ladder_on: bool = True


# ---------------------------------------------------------------------------------------------
# Running components


def _run(cfg: dict, out: Path) -> Path:
    out.parent.mkdir(parents=True, exist_ok=True)
    cfg = dict(cfg)
    cfg["out"] = str(out)
    p = out.with_suffix(".config.json")
    text = json.dumps(cfg, indent=1)
    if out.exists() and p.exists() and p.read_text() == text:
        n = sum(1 for _ in open(out, "rb"))
        if n == 2 * cfg["boards"]:
            return out
    p.write_text(text)
    rsim("run", str(p), quiet=True)
    return out


_CODE: dict = {}


def code_version() -> str:
    """A hash of the rsim binary: cached runs are reused only by the same simulator build."""
    from .common import RSIM

    st = RSIM.stat()
    key = (st.st_size, st.st_mtime_ns)
    if _CODE.get("key") != key:
        _CODE["key"] = key
        _CODE["v"] = hashlib.sha1(RSIM.read_bytes()).hexdigest()[:10]
    return _CODE["v"]


def base_cfg(b: Built, env: Env) -> dict:
    return {
        "dir": str(b.dir),
        "rules": str(b.rules),
        "model": b.model,
        "riskNeutral": env.util == "rn",
        "offerable": sorted(b.pool + [f"control-{r}" for r in ("common", "uncommon", "rare", "legendary")]),
        "perturb": 0.0,
        "version": f"{b.digest} code {code_version()}",
    } | b.exp


def run_field(b: Built, env: Env, n: int, extra: dict | None = None, tag: str = "field") -> Path:
    cfg = base_cfg(b, env) | {
        "seed": env.seed(COMPONENT["field"]),
        "boards": n,
        "tier": env.tier,
        "measured": {},
        "grantProb": 0.0,
        "cleanShare": 1.0,
        "arm": {"type": "tournament"},
    }
    cfg.update(extra or {})
    return _run(cfg, OUT / b.name / b.digest / env.key() / f"{tag}.jsonl")


def run_commit(b: Built, env: Env, arch: str, n: int, extra: dict | None = None) -> Path:
    counter = {s: 1.0 for s in b.pool if b.defs[s].get("touchesOpponents")}
    cfg = base_cfg(b, env) | {
        "seed": env.seed(10 + ARCHES.index(arch)),
        "boards": n,
        "tier": env.tier,
        "cleanShare": 0.0,
        "arm": {
            "type": "commitment",
            "archetype": arch,
            "counter": counter,
            "counterProb": 0.4 if counter else 0.0,
        },
    }
    cfg.update(extra or {})
    tag = (
        "commit-"
        + arch
        + ("" if not extra else "-" + hashlib.sha1(json.dumps(extra).encode()).hexdigest()[:6])
    )
    return _run(cfg, OUT / b.name / b.digest / env.key() / f"{tag}.jsonl")


def run_ladder(b: Built, env: Env, n: int) -> Path:
    # The ladder is tier 2 against tier 0 whatever the environment's tier (fun score v1 fix 2).
    cfg = base_cfg(b, env) | {
        "seed": env.seed(COMPONENT["ladder"]),
        "boards": n,
        "tier": 2,
        "tierB": 0,
        "measured": {},
        "grantProb": 0.0,
        "cleanShare": 1.0,
        "arm": {"type": "tournament"},
    }
    e2 = Env(2, env.util, env.seeds, env.rep)
    return _run(cfg, OUT / b.name / b.digest / e2.key() / "ladder.jsonl")


def archetypes_of(b: Built) -> list:
    return [a for a in ARCHES if any(a in b.defs[s]["archetypes"] for s in b.pool)]


# ---------------------------------------------------------------------------------------------
# Per-board arrays


def _lines(p: Path):
    import orjson

    with open(p, "rb") as f:
        for line in f:
            yield orjson.loads(line)


@dataclass
class FieldArrays:
    """One row per run (two per board), grouped by board."""

    board: np.ndarray
    share: np.ndarray  # median-margin numerator: |a-b| / winner, NaN when the winner has <= 0
    trail_valid: np.ndarray
    trail_won: np.ndarray
    early: np.ndarray
    total: np.ndarray
    sets: np.ndarray
    made: np.ndarray
    ot_late: np.ndarray
    made_late: np.ndarray
    bags: np.ndarray  # bags per team per run (mean of both teams)
    bag_pen: np.ndarray  # bag penalties per team per run, in penalties
    win_arch: np.ndarray  # winner's build archetype index, -1 none, -2 draw
    round_mean: np.ndarray  # per run: mean of both teams' round scores, rounds 1..8
    sets_by_round: np.ndarray
    contracts_by_round: np.ndarray
    pm: list  # (predicted, made, round, behind) for calibration


class TeamView:
    """The parts of a team record `arms.online` and `arms.build_archetype` read."""

    def __init__(self, t: dict):
        self.held = [(s["id"], s["shop"], s["sold"] or 9, s["grant"]) for s in t["sigils"]]
        self.cards = [tuple(x) for x in t["cards"]]


def field_arrays(p: Path, defs: dict) -> FieldArrays:
    cols = {k: [] for k in FieldArrays.__dataclass_fields__}
    nr = 8
    for r in _lines(p):
        a, b = r["teams"]
        ra = [x["score"] for x in a["rounds"]]
        rb = [x["score"] for x in b["rounds"]]
        nr = len(ra)
        fa, fb = a["final_score"], b["final_score"]
        w = max(fa, fb)
        cols["board"].append(r["board"])
        cols["share"].append(abs(fa - fb) / w if w > 0 else np.nan)
        k = min(5, nr)
        m5 = sum(ra[:k]) - sum(rb[:k])
        cols["trail_valid"].append(m5 != 0)
        cols["trail_won"].append((m5 < 0 and fa > fb) or (m5 > 0 and fb > fa))
        cols["early"].append(sum(max(0, x) for x in ra[:4]) + sum(max(0, x) for x in rb[:4]))
        cols["total"].append(sum(max(0, x) for x in ra) + sum(max(0, x) for x in rb))
        s = m = ot = ml = 0
        sbr = np.zeros(nr)
        cbr = np.zeros(nr)
        for t in (a, b):
            for rn, rd in enumerate(t["rounds"], start=1):
                if rd["contract"] > 0:
                    cbr[rn - 1] += 1
                    if rd["made"]:
                        m += 1
                        if rn >= nr - 2:
                            ml += 1
                            ot += rd["tricks"] - rd["contract"]
                    else:
                        s += 1
                        sbr[rn - 1] += 1
        cols["bags"].append(sum(rd.get("bags", 0) for t in (a, b) for rd in t["rounds"]) / 2)
        cols["bag_pen"].append(sum(rd.get("bag_pen", 0) < 0 for t in (a, b) for rd in t["rounds"]) / 2)
        cols["sets"].append(s)
        cols["made"].append(m)
        cols["ot_late"].append(ot)
        cols["made_late"].append(ml)
        if fa == fb:
            cols["win_arch"].append(-2)
        else:
            arch = R.build_archetype(TeamView(a if fa > fb else b), defs)
            cols["win_arch"].append(ARCHES.index(arch) if arch else -1)
        cols["round_mean"].append([(x + y) / 2 for x, y in zip(ra, rb, strict=True)])
        cols["sets_by_round"].append(sbr)
        cols["contracts_by_round"].append(cbr)
        for ti, t in enumerate((a, b)):
            cum = 0.0
            ocum = 0.0
            o = b if ti == 0 else a
            for rn, rd in enumerate(t["rounds"], start=1):
                if rd.get("pm") is not None and rd["contract"] > 0:
                    cols["pm"].append((rd["pm"], rd["made"], rn, cum < ocum))
                cum += rd["score"]
                ocum += o["rounds"][rn - 1]["score"]
    out = {}
    for k, v in cols.items():
        out[k] = v if k == "pm" else np.array(v, dtype=float)
    return FieldArrays(**out)


@dataclass
class CommitArrays:
    board: np.ndarray
    win: np.ndarray
    online: np.ndarray


def commit_arrays(p: Path, arch: str, defs: dict) -> CommitArrays:
    bd, w, on = [], [], []
    for r in _lines(p):
        bd.append(r["board"])
        w.append(r["win"])
        on.append(R.online(TeamView(r["teams"][0]), arch, defs))
    return CommitArrays(np.array(bd), np.array(w, dtype=float), np.array(on, dtype=float))


def ladder_arrays(p: Path) -> tuple:
    by: dict = {}
    for r in _lines(p):
        by.setdefault(r["board"], []).append(r["margin"])
    boards = sorted(by)
    return np.array(boards), np.array(
        [1.0 if sum(by[k]) > 0 else 0.0 if sum(by[k]) < 0 else 0.5 for k in boards]
    )


# ---------------------------------------------------------------------------------------------
# The fun score from arrays (v1), resampled by board


def _board_index(boards: np.ndarray) -> tuple:
    keys, inv = np.unique(boards, return_inverse=True)
    rows = [np.flatnonzero(inv == i) for i in range(len(keys))]
    return keys, rows


def _rows(rows: list, pick: np.ndarray | None) -> np.ndarray:
    if pick is None:
        return np.concatenate(rows)
    return np.concatenate([rows[i] for i in pick])


def field_stats(f: FieldArrays, idx: np.ndarray) -> dict:
    sh = f.share[idx]
    sh = sh[~np.isnan(sh)]
    tv = f.trail_valid[idx] > 0
    wa = f.win_arch[idx]
    nondraw = wa != -2
    n = max(1, int(nondraw.sum()))
    shares = {a: float(np.sum(wa == i) / n) for i, a in enumerate(ARCHES)}
    return {
        "medianMarginShare": float(np.median(sh)) if len(sh) else float("nan"),
        "trailerAfter5Wins": float(f.trail_won[idx][tv].sum() / max(1, tv.sum())),
        "earlyShare": float(f.early[idx].sum() / max(1.0, f.total[idx].sum())),
        "setRate": float(f.sets[idx].sum() / max(1.0, f.sets[idx].sum() + f.made[idx].sum())),
        "lateOvertricks": float(f.ot_late[idx].sum() / max(1.0, f.made_late[idx].sum())),
        "bagsPerRun": float(f.bags[idx].mean()),
        "bagPenaltiesPerRun": float(f.bag_pen[idx].mean()),
        "winningShare": shares,
    }


@dataclass
class Measured:
    variant: str
    env: Env
    field: FieldArrays
    commits: dict  # arch -> CommitArrays
    ladder: tuple | None
    simplicity: float  # the pool's mean complexity C
    synergy: object = None  # a synergy estimator with .sample(rng) and .point()

    def components(self):
        f_keys, f_rows = _board_index(self.field.board)
        c = {a: _board_index(ca.board) for a, ca in self.commits.items()}
        return f_keys, f_rows, c

    def compute(self, picks: dict | None = None) -> dict:
        """The fun score; `picks` holds board resamples per component (None = all boards)."""
        picks = picks or {}
        f_keys, f_rows = self._fidx
        idx = _rows(f_rows, picks.get("field"))
        gs = field_stats(self.field, idx)
        per, onl = {}, []
        for a, ca in self.commits.items():
            _k, rows = self._cidx[a]
            ix = _rows(rows, picks.get("commit-" + a))
            per[a] = float(ca.win[ix].mean())
            onl.append(ca.online[ix])
        commit = {
            "per_archetype": per,
            "committed_win": float(np.mean(list(per.values()))),
            "online": float(np.concatenate(onl).mean()),
        }
        if self.ladder is not None:
            lk, lw = self.ladder
            lp = picks.get("ladder")
            lad = float(lw.mean() if lp is None else lw[lp].mean())
        else:
            lad = float("nan")
        syn = None
        if self.synergy is not None:
            syn = self.synergy.value(picks.get("syn"))
        fams = families(gs, commit, syn, lad, self.simplicity)
        return {
            "score": total(fams),
            "families": fams,
            "game": gs,
            "commit": commit,
            "ladder": lad,
            "synergy": syn,
            "meanC": self.simplicity,
        }

    def prepare(self):
        self._fidx = _board_index(self.field.board)
        self._cidx = {a: _board_index(ca.board) for a, ca in self.commits.items()}
        return self


def draw_picks(m: Measured, rng) -> dict:
    """One cluster-bootstrap resample of boards for every component (shared across paired variants
    because the board sets are identical)."""
    picks = {"field": rng.integers(0, len(m._fidx[0]), len(m._fidx[0]))}
    for a, (keys, _rows) in m._cidx.items():
        picks["commit-" + a] = rng.integers(0, len(keys), len(keys))
    if m.ladder is not None:
        n = len(m.ladder[0])
        picks["ladder"] = rng.integers(0, n, n)
    if m.synergy is not None:
        picks["syn"] = m.synergy.resample(rng)
    return picks


def simplicity_of(b: Built) -> float:
    """The pool's mean complexity C."""
    vals = [b.defs[s].get("simplicity", {}).get("C", 0.0) for s in b.pool]
    return float(np.mean(vals)) if vals else 0.0


def measure(name: str, env: Env, sizes: Sizes | None = None, synergy=None) -> Measured:
    sizes = sizes or Sizes()
    b = Variant.load(name).materialize()
    fp = run_field(b, env, sizes.field)
    fa = field_arrays(fp, b.defs)
    commits = {}
    for a in archetypes_of(b):
        cp = run_commit(b, env, a, sizes.commit)
        commits[a] = commit_arrays(cp, a, b.defs)
    lad = ladder_arrays(run_ladder(b, env, sizes.ladder)) if sizes.ladder_on else None
    syn = synergy(b, env) if synergy else None
    return Measured(name, env, fa, commits, lad, simplicity_of(b), syn).prepare()


def interval(m: Measured, boot: int = 300, seed: int = 7) -> dict:
    point = m.compute()
    rng = np.random.default_rng(seed)
    tots, fams = [], []
    for _ in range(boot):
        r = m.compute(draw_picks(m, rng))
        tots.append(r["score"])
        fams.append(r["families"])
    fams = np.array(fams)
    point["lo"], point["hi"] = (float(x) for x in np.quantile(tots, [0.05, 0.95]))
    point["sd"] = float(np.std(tots))
    point["fam_lo"] = np.nanquantile(fams, 0.05, axis=0).tolist()
    point["fam_hi"] = np.nanquantile(fams, 0.95, axis=0).tolist()
    point["fam_sd"] = np.nanstd(fams, axis=0).tolist()
    return point


def paired(a: Measured, b: Measured, boot: int = 300, seed: int = 11) -> dict:
    """B minus A on the same boards, with 90% intervals for the total and every family."""
    pa, pb = a.compute(), b.compute()
    rng = np.random.default_rng(seed)
    d_tot, d_fam = [], []
    for _ in range(boot):
        picks = draw_picks(a, rng)
        ra, rb = a.compute(picks), b.compute(picks)
        d_tot.append(rb["score"] - ra["score"])
        d_fam.append(np.array(rb["families"]) - np.array(ra["families"]))
    d_fam = np.array(d_fam) * np.array(WEIGHTS)
    return {
        "a": pa,
        "b": pb,
        "diff": pb["score"] - pa["score"],
        "lo": float(np.quantile(d_tot, 0.05)),
        "hi": float(np.quantile(d_tot, 0.95)),
        "fam_diff": [w * (y - x) for w, x, y in zip(WEIGHTS, pa["families"], pb["families"], strict=True)],
        "fam_lo": np.nanquantile(d_fam, 0.05, axis=0).tolist(),
        "fam_hi": np.nanquantile(d_fam, 0.95, axis=0).tolist(),
    }


# ---------------------------------------------------------------------------------------------
# Reports


def fmt_measure(name: str, r: dict) -> str:
    g = r["game"]
    fam = " | ".join(
        f"{f[:5]} {r['families'][i]:.2f}" + (f"±{1.645 * r['fam_sd'][i]:.2f}" if "fam_sd" in r else "")
        for i, f in enumerate(FAMILIES)
    )
    ci = f" [{r['lo']:.1f}, {r['hi']:.1f}]" if "lo" in r else ""
    return (
        f"{name}: fun {r['score']:.1f}{ci} | {fam}\n"
        f"    margin {g['medianMarginShare']:.3f} trail5 {g['trailerAfter5Wins']:.3f} early {g['earlyShare']:.3f} "
        f"set {g['setRate']:.3f} ot {g['lateOvertricks']:.2f} bags {g['bagsPerRun']:.1f}/{g['bagPenaltiesPerRun']:.2f} "
        f"meanC {r['meanC']:.2f} ladder {r['ladder']:.3f} "
        f"online {r['commit']['online']:.3f} cwin {r['commit']['committed_win']:.3f} "
        f"topshare {max(g['winningShare'].values()):.3f}"
        + (
            f" syn {r['synergy']['same_arch_gain_pts']:.2f}/{r['synergy']['strong_pts']}/{r['synergy']['strong_pts_cross']}"
            if r.get("synergy")
            else ""
        )
    )


def fmt_paired(p: dict) -> str:
    fam = " | ".join(
        f"{f[:5]} {p['fam_diff'][i]:+.2f} [{p['fam_lo'][i]:+.2f}, {p['fam_hi'][i]:+.2f}]"
        for i, f in enumerate(FAMILIES)
    )
    return f"diff {p['diff']:+.2f} [{p['lo']:+.2f}, {p['hi']:+.2f}] | {fam}"
