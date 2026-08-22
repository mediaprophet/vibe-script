# VibeScript for VS Code

Syntax highlighting for `.vibe` and an optional LSP client.

1. `cargo install --path crates/vibe-lsp` from this repo (needs QualiaDB `crates/vibe` on disk at `C:\Projects\qualia-27062026`).
2. Set `vibe.lsp.path` to that binary if it is not on `PATH`.
3. Open a `.vibe` file.

The language engine is **not** vendored here. Tools depend on QualiaDB `vibe-0.1`.
