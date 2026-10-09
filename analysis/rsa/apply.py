"""Writes a search variant into the game's data: data/rules.json, data/sigils/, and its shop model as
data/models/search-final.json (read by the game client and `rsim play`).

    uv run python -m rsa.apply <variant>

Pool sigils are written with status "kept" and their variant effect and rarity; kept sigils that
left the pool become "cut"; every change gets a history note.
"""

import json
import shutil
import sys

from . import it
from .common import MODELS, ROOT, RULES_PATH, SIGILS

NAMES = {
    "s-aces-untrumpable": ("Steady Anchor", "anchor"),
    "s-exact-x": ("True Target", "target"),
}


def main(name: str) -> None:
    b = it.Variant.load(name).materialize()
    rules = json.loads(b.rules.read_text())
    RULES_PATH.write_text(json.dumps(rules, indent=2) + "\n")
    pool = set(b.pool)
    changed = 0
    for sid, d in b.defs.items():
        p = SIGILS / f"{sid}.json"
        old = json.loads(p.read_text()) if p.exists() else None
        if old is None:
            if sid not in pool:
                continue
            new = dict(d)
            nm, icon = NAMES.get(sid, (None, None))
            new.update(
                {"name": nm, "icon": icon, "iconFamily": icon, "iconWord": icon.title() if icon else None}
            )
            new["history"] = [{"step": "design-search", "note": f"Added in {name}."}]
        else:
            new = dict(old)
            notes = []
            for k in ("effect", "rarity", "archetypes"):
                if json.dumps(old.get(k), sort_keys=True) != json.dumps(d.get(k), sort_keys=True):
                    new[k] = d[k]
                    notes.append(f"{k} {json.dumps(old.get(k))} -> {json.dumps(d.get(k))}")
            status = "kept" if sid in pool else ("cut" if old.get("status") == "kept" else old.get("status"))
            if status != old.get("status"):
                notes.append(f"status {old.get('status')} -> {status}")
                new["status"] = status
            if not notes:
                continue
            new.setdefault("history", []).append(
                {"step": "design-search", "note": f"{name}: " + "; ".join(notes)}
            )
        p.write_text(json.dumps(new, indent=2, ensure_ascii=False) + "\n")
        changed += 1
    shutil.copy(b.model, MODELS / "search-final.json")
    print(f"wrote rules, {changed} sigil files, and data/models/search-final.json from {name} ({ROOT})")


if __name__ == "__main__":
    main(sys.argv[1])
