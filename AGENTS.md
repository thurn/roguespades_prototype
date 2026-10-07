# Working on the Rogue Spades Prototype

This is a throwaway web-based prototype for a roguelike card game based on Spades. Do not write tests. Do not overarchitect. Treat all code as disposable.

Validate with `scripts/ci`. Immediately commit all changes using Conventional Commits syntax, then deliver them through Tollgate: on master run `tg push-master --wait`; in a Tollgate worktree submit with `tg candidate HEAD`, approve, and wait for promotion. Tollgate owns pushes to remote master. Changes of any size are delivered without warden review.

When completing tasks, provide a short summary of the work with the title "# Summary" as flat bullet points, focusing on surprising or interesting decisions you made, problems encountered, and action items for me. Flat means no nested bullets, and each bullet is a single sentence. Reduce the amount of summary text you think you need to write by half.

When you get the usage limit warning, please write a short HANDOFF.md describing current state for a fresh agent to resume.

Game client:

- `npm --prefix viewer run dev`, then `http://localhost:5173/` (the sigil viewer is at `/sigils`). It runs `sim/src/session.rs` compiled to wasm (`sim/web`); the dev server rebuilds the wasm when Rust changes.
- URL params: `seed`, `tier` (0–2), `auto`, `fast`, `sandbox`, `reveal`, `give`/`giveThem` (sigil ids), `cards` (`AS,10H,QC*,KDb`), `gold`, `replay` (a log name).
- Every session writes `logs/<time>-seed<seed>.jsonl`: one event per line with a readable `msg`, including deals, every bid and play with the AI's candidate values, sigil fires, scoring formulas, and shop decisions. Events with an `action` replay the game exactly.
- To investigate a session: `grep '"msg"'` the log, or replay it natively with `sim/target/release/rsim play --replay logs/<file> [--view-at <step>]` from `sim/`.

Game Invariants:

- All game design decisions are empirically validated via simulation against our target metrics
- Sigils do not have "activated abilities"
