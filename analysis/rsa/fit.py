"""The outcome model: one regression over all boards of a grant tournament.

u_d = S(A) - S(B) + e_d, with per-sigil value, amount slope, shop slope, and coherence terms,
card-grant classes, the value of gold by shop, and a factorization machine for pairs. Ridge
regression with empirical-Bayes prior means by rarity and category, cross-validated strength,
alternating least squares for the pair vectors, and a cluster bootstrap over boards.
"""

import math
from dataclasses import dataclass

import numpy as np
from scipy import sparse

from .common import CARD_CLASSES, category

N_TERMS = 4  # beta, amount slope, shop slope, coherence


def w_curve(margin: np.ndarray, scale: float) -> np.ndarray:
    return 1.0 / (1.0 + np.exp(-np.clip(margin / scale, -50, 50)))


@dataclass
class Design:
    ids: list  # measured sigil ids, column blocks
    X: sparse.csr_matrix  # boards x features
    u: np.ndarray  # smoothed outcome per board
    win: np.ndarray  # team A board result in [0, 1]
    board_ids: np.ndarray
    pairs: list  # per board: list of (sign*0.5, [sigil indices of one team's grants in one run])
    n_cols: int
    exposures: np.ndarray
    pair_cols: list
    pair_off: int


def col_beta(i):
    return i * N_TERMS


def build(
    runs_by_board: dict, scale: float, measured: list | None = None, orients=(0, 1), pair_cols=None
) -> Design:
    """Builds the design from non-clean boards. `measured` fixes the sigil column order."""
    ids = set()
    for runs in runs_by_board.values():
        for r in runs:
            for t in r.teams:
                ids.update(g[0] for g in t.grants)
    ids = sorted(ids) if measured is None else list(measured) + sorted(ids - set(measured))
    index = {s: i for i, s in enumerate(ids)}
    n = len(ids)
    card_off = n * N_TERMS
    gold_off = card_off + len(CARD_CLASSES)
    pair_off = gold_off + 8
    pair_cols = list(pair_cols or [])
    icpt = pair_off + len(pair_cols)
    n_cols = icpt + 1
    rows, cols, vals = [], [], []
    u, win, bids, pairs = [], [], [], []
    expo = np.zeros(n)
    d = 0
    for b, runs in runs_by_board.items():
        runs = [r for r in runs if r.orient in orients]
        if not runs or runs[0].clean or len(runs) != len(orients):
            continue
        acc: dict = {}
        bp = []
        for r in runs:
            for ti, t in enumerate(r.teams):
                sign = (1.0 if ti == 0 else -1.0) / len(runs)
                held_ids = {g[0] for g in t.grants}
                for pc, (pa, pb) in enumerate(pair_cols):
                    if pa in held_ids and pb in held_ids:
                        acc[pair_off + pc] = acc.get(pair_off + pc, 0.0) + sign
                gi = []
                for sid, shop, ratio, held, coh in t.grants:
                    i = index[sid]
                    gi.append(i)
                    for k, v in enumerate((held, held * ratio, held * (shop - 3.5), held * coh)):
                        acc[i * N_TERMS + k] = acc.get(i * N_TERMS + k, 0.0) + sign * v
                    if r.orient == orients[0]:
                        expo[i] += 1
                if len(gi) >= 2:
                    bp.append((sign, gi))
                for cls, _shop in t.card_grants:
                    c = card_off + CARD_CLASSES.index(cls)
                    acc[c] = acc.get(c, 0.0) + sign
                for shop, amt in t.perturb:
                    c = gold_off + shop - 1
                    acc[c] = acc.get(c, 0.0) + sign * amt
        acc[icpt] = 1.0
        for c, v in acc.items():
            if v != 0.0:
                rows.append(d)
                cols.append(c)
                vals.append(v)
        m = np.array([r.margin for r in runs])
        u.append(float(np.mean(2 * w_curve(m, scale) - 1)))
        win.append(float(np.mean([r.win for r in runs])))
        bids.append(b)
        pairs.append(bp)
        d += 1
    X = sparse.csr_matrix((vals, (rows, cols)), shape=(d, n_cols))
    return Design(
        ids, X, np.array(u), np.array(win), np.array(bids), pairs, n_cols, expo, pair_cols, pair_off
    )


@dataclass
class Fit:
    ids: list
    coef: np.ndarray
    V: np.ndarray
    prior: np.ndarray
    alpha: float
    boot: np.ndarray | None = None
    boot_raw: np.ndarray | None = None
    resid_sd: float = 0.0
    fm_pred: np.ndarray | None = None

    def term(self, sid: str, k: int) -> float:
        return float(self.coef[self.ids.index(sid) * N_TERMS + k])


def _penalty(n_sig: int, n_cols: int, alpha: float) -> np.ndarray:
    p = np.zeros(n_cols)
    p[: n_sig * N_TERMS] = alpha
    # Amount, shop, and coherence terms are noisier; shrink them a little harder.
    for i in range(n_sig):
        p[i * N_TERMS + 1 : i * N_TERMS + 4] = alpha * 2.0
    p[n_sig * N_TERMS : n_sig * N_TERMS + len(CARD_CLASSES)] = alpha * 0.5
    p[n_sig * N_TERMS + len(CARD_CLASSES) : n_sig * N_TERMS + len(CARD_CLASSES) + 8] = 1e-3
    p[n_sig * N_TERMS + len(CARD_CLASSES) + 8 : -1] = alpha
    return p


def _solve(XtX, Xty, pen, prior):
    A = XtX + np.diag(pen)
    return np.linalg.solve(A, Xty + pen * prior)


def group_prior(ids: list, sigils: dict, coef: np.ndarray, expo: np.ndarray) -> np.ndarray:
    """Empirical-Bayes prior means: each beta's group (rarity x category) mean, exposure-weighted."""
    prior = np.zeros_like(coef)
    groups: dict = {}
    for i, s in enumerate(ids):
        d = sigils.get(s, {})
        key = (d.get("rarity"), "control" if d.get("source") == "control" else category(d))
        groups.setdefault(key, []).append(i)
    for key, members in groups.items():
        if key[1] == "control":
            continue
        w = expo[members] + 1.0
        m = float(np.sum(coef[[i * N_TERMS for i in members]] * w) / np.sum(w))
        for i in members:
            prior[i * N_TERMS] = m
    return prior


def pair_term(des: Design, V: np.ndarray) -> np.ndarray:
    out = np.zeros(len(des.u))
    if V is None or not np.any(V):
        return out
    for d, bp in enumerate(des.pairs):
        s = 0.0
        for sign, gi in bp:
            Vg = V[gi]
            tot = Vg.sum(axis=0)
            s += sign * 0.5 * (float(tot @ tot) - float(np.sum(Vg * Vg)))
        out[d] = s
    return out


def fit_fm(
    des: Design, resid: np.ndarray, rank: int, lam: float, sweeps: int = 4, seed: int = 0
) -> np.ndarray:
    """Alternating least squares for the pair vectors on the linear model's residuals."""
    n = len(des.ids)
    rng = np.random.default_rng(seed)
    V = rng.normal(0, 0.01, (n, rank))
    appear: dict = {}
    for d, bp in enumerate(des.pairs):
        for sign, gi in bp:
            for i in gi:
                appear.setdefault(i, []).append((d, sign, gi))
    for _ in range(sweeps):
        cur = pair_term(des, V)
        for i, occ in appear.items():
            if len(occ) < 5:
                continue
            Z = np.zeros((len(occ), rank))
            y = np.zeros(len(occ))
            for k, (d, sign, gi) in enumerate(occ):
                others = [h for h in gi if h != i]
                z = sign * V[others].sum(axis=0)
                Z[k] = z
                y[k] = resid[d] - cur[d] + float(z @ V[i])
            new = np.linalg.solve(Z.T @ Z + lam * np.eye(rank), Z.T @ y)
            for k, (d, _sign, _gi) in enumerate(occ):
                cur[d] += float(Z[k] @ (new - V[i]))
            V[i] = new
    return V


def fit(
    des: Design,
    sigils: dict,
    alphas=(20.0, 60.0, 200.0, 600.0),
    folds: int = 5,
    fm_rank: int = 4,
    fm_lam: float = 400.0,
    boot: int = 200,
    seed: int = 1,
    alpha: float | None = None,
) -> Fit:
    X, u = des.X, des.u
    n_sig = len(des.ids)
    rng = np.random.default_rng(seed)
    fold = rng.integers(0, folds, X.shape[0])

    def ridge(Xa, ua, a, prior=None, w=None):
        Xw = Xa if w is None else Xa.multiply(w[:, None]).tocsr()
        XtX = (Xw.T @ Xa).toarray()
        Xty = Xw.T @ ua
        pen = _penalty(n_sig, des.n_cols, a)
        pr = np.zeros(des.n_cols) if prior is None else prior
        c = _solve(XtX, Xty, pen, pr)
        # One empirical-Bayes pass: pull betas toward their group means.
        if prior is None:
            pr = group_prior(des.ids, sigils, c, des.exposures)
            c = _solve(XtX, Xty, pen, pr)
        return c, pr

    if alpha is None:
        best = None
        for a in alphas:
            err = 0.0
            for f in range(folds):
                tr, te = fold != f, fold == f
                c, _ = ridge(X[tr], u[tr], a)
                err += float(np.sum((u[te] - X[te] @ c) ** 2))
            if best is None or err < best[0]:
                best = (err, a)
        alpha = best[1]
    coef, prior = ridge(X, u, alpha)
    V = fit_fm(des, u - X @ coef, fm_rank, fm_lam)
    fm = pair_term(des, V)
    for _ in range(2):
        coef, prior = ridge(X, u - fm, alpha)
        V = fit_fm(des, u - X @ coef, fm_rank, fm_lam)
        fm = pair_term(des, V)
    resid = u - X @ coef - fm
    out = Fit(des.ids, coef, V, prior, alpha, resid_sd=float(np.std(resid)), fm_pred=fm)
    if boot:
        # Two bootstraps on the same resamples: the shrunk estimator, and a nearly unpenalized one.
        # Empirical-Bayes intervals use the geometric mean of their spreads (see `interval`).
        B = np.zeros((boot, des.n_cols))
        Braw = np.zeros((boot, des.n_cols))
        y = u - fm
        for b in range(boot):
            w = rng.poisson(1.0, X.shape[0]).astype(float)
            B[b], _ = ridge(X, y, alpha, prior=prior, w=w)
            Braw[b], _ = ridge(X, y, RAW_ALPHA, prior=prior, w=w)
        out.boot = B
        out.boot_raw = Braw
    return out


RAW_ALPHA = 1.0


def interval(samples: np.ndarray, point: float, raw: np.ndarray | None = None) -> tuple:
    """90% interval around the shrunk point estimate.

    A ridge bootstrap's spread is (1 - B) times the raw standard error, where B is the shrinkage
    factor, but the empirical-Bayes posterior sd is sqrt(1 - B) times it: the geometric mean of the
    shrunk and raw spreads. Without a raw bootstrap, fall back to the shrunk spread."""
    sd = float(np.std(samples))
    if raw is not None:
        sd = math.sqrt(sd * float(np.std(raw)))
    return point - 1.645 * sd, point + 1.645 * sd


def sd_eb(samples: np.ndarray, raw: np.ndarray | None) -> float:
    sd = float(np.std(samples))
    return math.sqrt(sd * float(np.std(raw))) if raw is not None else sd


def win_calibration(des: Design) -> float:
    """Slope k mapping smoothed-outcome differences to win-rate differences: (2 win - 1) ~ k u."""
    y = 2 * des.win - 1
    k = float(np.dot(des.u, y) / max(np.dot(des.u, des.u), 1e-9))
    return k


def to_points(x, k: float):
    """Smoothed-outcome units to win-rate points."""
    return 100.0 * k * np.asarray(x) / 2.0
