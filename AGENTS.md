# Working on the Rogue Spades Prototype

This is a throwaway web-based prototype for a roguelike card game based on Spades. Do not write tests. Do not overarchitect. Treat all code as disposable.

Validate with `scripts/ci`. Immediately commit all changes using Conventional Commits syntax, then deliver them through Tollgate: on master run `tg push-master --wait`; in a Tollgate worktree submit with `tg candidate HEAD`, approve, and wait for promotion. Tollgate owns pushes to remote master. Changes of any size are delivered without warden review.

When completing tasks, provide a short summary of the work with the title "# Summary" as flat bullet points, focusing on surprising or interesting decisions you made, problems encountered, and action items for me. Flat means no nested bullets, and each bullet is a single sentence. Reduce the amount of summary text you think you need to write by half.

When you get the usage limit warning, please write a short HANDOFF.md describing current state for a fresh agent to resume.

Game Invariants:

- All game design decisions are empirically validated via simulation against our target metrics
- Sigils do not have "activated abilities"
