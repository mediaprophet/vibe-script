# VibeScript for VS Code

Syntax highlighting for `.vibe` and an optional LSP client. Extension version **0.0.1** (same as the workspace — do not bump for catalog alignment).

1. Check out QualiaDB tree [`0.0.38`](https://github.com/mediaprophet/qualiaDB/tree/0.0.38) so `crates/vibe` is the sibling the LSP path-dep expects (`../qualia-27062026/crates/vibe` on Timothy’s disk).
2. `cargo install --path crates/vibe-lsp` from this repo.
3. Set `vibe.lsp.path` to that binary if it is not on `PATH`.
4. Open a `.vibe` file.

The language engine is **not** vendored here. Workshop dialect: `using LinearAlgebra;` then `LinearAlgebra.dot(...)`. Diagnose voice: **live** / **planned**. Grammar may grow (versioned, documented).
