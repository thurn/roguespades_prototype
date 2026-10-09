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

# Names and icons for pool sigils that have none (data/search/pool/rec2-names.tsv: id, name, icon).
NAMES = {
    r[0]: (r[1], r[2])
    for r in (
        line.split("\t")
        for line in (ROOT / "data" / "search" / "pool" / "rec2-names.tsv").read_text().strip().splitlines()
    )
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
            nm, icon = (d["name"], d.get("icon")) if d.get("name") else NAMES.get(sid, (None, None))
            new.update(
                {"name": nm, "icon": icon, "iconFamily": icon, "iconWord": icon.title() if icon else None}
            )
            new["status"] = "kept"
            new["history"] = [{"step": "design-search-2", "note": f"Added in {name}."}]
        else:
            new = dict(old)
            notes = []
            if sid in pool and not old.get("name") and (d.get("name") or sid in NAMES):
                nm, icon = (d["name"], d.get("icon")) if d.get("name") else NAMES[sid]
                new.update(
                    {"name": nm, "icon": icon, "iconFamily": icon, "iconWord": icon.title() if icon else None}
                )
                notes.append(f"named {nm} ({icon})")
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
                {"step": "design-search-2", "note": f"{name}: " + "; ".join(notes)}
            )
        p.write_text(json.dumps(new, indent=2, ensure_ascii=False) + "\n")
        changed += 1
    shutil.copy(b.model, MODELS / "search-final.json")
    print(f"wrote rules, {changed} sigil files, and data/models/search-final.json from {name} ({ROOT})")


if __name__ == "__main__":
    main(sys.argv[1])
