# VibeScript for VS Code

Syntax highlighting for `.vibe` and an optional LSP client. Extension version **0.0.1** (same as the workspace — do not bump for catalog alignment).

1. Check out QualiaDB tree [`0.0.41`](https://github.com/mediaprophet/qualiaDB) so `crates/vibe` is the sibling the LSP path-dep expects (`../qualiaDB/crates/vibe` on disk).
2. `cargo install --path crates/vibe-lsp` from this repo.
3. Set `vibe.lsp.path` to that binary if it is not on `PATH`.
4. Open a `.vibe` file.

The language engine is **not** vendored here. Current dialect: `using Family;` then `Family.method(...)`. Grammar may grow when versioned. Tooling diagnose: **held / not yet**.
