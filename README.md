# VibeScript tools

**Vibe** is the language (`vibe-0.1`). This repository is the **ecosystem**: LSP, syntax highlighting, VS Code, playground, agent skill, docs pages.

The language engine is **not** here. It lives in QualiaDB:

`C:\Projects\qualia-27062026\crates\vibe`

Poet is a host. This is not a JavaScript clone.

Copyright © 2026 Timothy Charles Holborn.

## Layout

| Path | What |
|---|---|
| `crates/vibe-lsp` | LSP (stdin/stdout JSON-RPC). Depends on QualiaDB `vibe`. |
| `editors/vscode` | `.vibe` highlighting + language config |
| `grammar/` | Copy of engine EBNF/GBNF/schema for tools |
| `playground/` | Static samples (WASM eval is JOBS E4) |
| `skills/SKILL.md` | Agent authoring rules |
| `JOBS.md` | Remaining work |

## Build the LSP

From this directory, with QualiaDB checked out as a sibling:

```
cargo test -p vibe-lsp
cargo run -p vibe-lsp
```

Point VS Code `vibe.lsp.path` at the resulting `vibe-lsp` binary when the LanguageClient wiring (JOBS E3) is packaged.

## Dialect (workshop)

```vibe
using Animation, HID;

effect fn spin(t: f64) {
    return Animation.orbit_spin(t);
}
```

Unknown `Family.method` names fail closed. Modal verbs stay terms unless the engine family is leased.
