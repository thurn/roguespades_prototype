"""Claims names and icons from the namer's options: the first pair still free in name, icon
family, and icon word, with the icon present in docs/sigils/icons.txt."""

import json
import sys

from .common import ROOT, history, load_sigils, save_sigil


def assign(path: str, step: str) -> list:
    icons = set((ROOT / "docs" / "sigils" / "icons.txt").read_text().split())
    opts = json.loads(open(path).read())
    sig = load_sigils()
    used_n, used_f, used_w = set(), set(), set()
    for d in sig.values():
        if d["status"] == "kept" and d.get("name"):
            used_n.add(d["name"].lower())
            used_f.add((d.get("iconFamily") or "").lower())
            used_w.add((d.get("iconWord") or "").lower())
    unnamed = []
    for o in opts:
        d = sig.get(o["id"])
        if d is None or d["status"] != "kept" or d.get("name"):
            continue
        for op in o["options"]:
            n, f, w = op["name"].lower(), op["iconFamily"].lower(), op["iconWord"].lower()
            if op["icon"] in icons and n not in used_n and f not in used_f and w not in used_w:
                d.update(
                    name=op["name"], icon=op["icon"], iconFamily=op["iconFamily"], iconWord=op["iconWord"]
                )
                history(d, step, f"{step}: named {op['name']} ({op['icon']}).")
                save_sigil(d)
                used_n.add(n)
                used_f.add(f)
                used_w.add(w)
                break
        else:
            unnamed.append(o["id"])
    return unnamed


if __name__ == "__main__":
    print("unnamed:", assign(sys.argv[1], sys.argv[2]))
