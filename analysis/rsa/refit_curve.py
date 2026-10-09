"""Refits the AI's margin-to-win curve and round scales from field records (a measurement fix:
the shipped model was fitted under the 1.0 rules, whose round scores were much smaller).

    uv run python -m rsa.refit_curve <field.jsonl> <in-model> <out-model>
"""

import json
import sys

import numpy as np

from . import it


def main(field: str, src: str, dst: str) -> None:
    sd_left = {k: [] for k in range(1, 9)}
    by_round = np.zeros(9)
    n_round = np.zeros(9)
    finals = []
    for r in it._lines(field):
        a, b = r["teams"]
        ra = [x["score"] for x in a["rounds"]]
        rb = [x["score"] for x in b["rounds"]]
        ca, cb = np.cumsum([0] + ra), np.cumsum([0] + rb)
        fm = ca[8] - cb[8]
        finals.append(fm)
        for left in range(1, 9):
            sd_left[left].append(fm - (ca[8 - left] - cb[8 - left]))
        for k in range(8):
            by_round[k + 1] += ra[k] + rb[k]
            n_round[k + 1] += 2
    m = json.loads(open(src).read())
    old = (list(m["wScale"]), m["wFinal"], list(m["roundScale"]))
    m["wScale"] = [50.0] + [0.588 * float(np.std(sd_left[k])) for k in range(1, 9)]
    m["wFinal"] = 0.588 * float(np.std(finals))
    m["roundScale"] = [0.0] + [max(300.0, float(x)) for x in (by_round / np.maximum(n_round, 1))[1:9]]
    m["step"] = dst.rsplit("/", 1)[-1].removesuffix(".json")
    open(dst, "w").write(json.dumps(m, indent=1) + "\n")
    print("wScale", [round(x) for x in old[0]], "->", [round(x) for x in m["wScale"]])
    print("wFinal", round(old[1]), "->", round(m["wFinal"]))
    print("roundScale", [round(x) for x in old[2]], "->", [round(x) for x in m["roundScale"]])


if __name__ == "__main__":
    main(*sys.argv[1:4])
