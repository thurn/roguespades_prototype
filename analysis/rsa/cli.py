"""`uv run analysis <step>`: one command per plan step."""

import argparse

from .common import build_rsim


def main() -> None:
    ap = argparse.ArgumentParser(prog="analysis")
    sub = ap.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("pilot", help="Phase 0d pilot on the seed pool")
    p.add_argument("--boards", type=int, default=8000)
    a = ap.parse_args()
    build_rsim()
    if a.cmd == "pilot":
        from . import pilot

        pilot.run(a.boards)
