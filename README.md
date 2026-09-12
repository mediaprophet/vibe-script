# VibeScript tools

**Vibe** is the language (`vibe-0.1`). This repository is the **ecosystem**: LSP, syntax highlighting, VS Code, playground, agent skill, grammar copies, and docs pages.

The language engine is **not** here. Do not fork it into this repo. It lives in QualiaDB tree **`0.0.38`**:

- Engine: [`mediaprophet/qualiaDB` `crates/vibe`](https://github.com/mediaprophet/qualiaDB/tree/0.0.38/crates/vibe)
- Also on that tree: `crates/vibe-lsp`, `crates/vibe-wasm` (in-tree drafts stay until Timothy says to delete them; **new tool work lands here**)
- Normative language: [`docs/manuals/standards/vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md)
- Sprint / catalog policy: [`docs/manuals/standards/vibescript-sprint-deltas.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-sprint-deltas.md)

Poet is a host. A chatbot is a tool. This is not a JavaScript clone.

Workspace / crate version stays **`0.0.1`**. QualiaDB `0.0.38` is the engine tree this ecosystem tracks — not a version bump of vibe-script.

Copyright © 2026 Timothy Charles Holborn.

## Current dialect (workshop)

`vibe-0.1` is the **current dialect**. Grammar is **not** locked. It **may grow** when humans need a better form (versioned, documented). The catalog of named work also grows. `vibe-host-0.1` is an outcome of incorporating live libraries, not a freeze.

Authors write **human** sayables:

```vibe
using Animation, LinearAlgebra;

effect fn spin(t: f64) {
    return Animation.orbit_spin(t);
}

pure fn score(a: List, b: List) {
    return LinearAlgebra.dot({ a: a, b: b });
}
```

Unknown names fail. Modal verbs stay terms unless the family is leased. Do not invent dotted `qualia.*` IRIs ahead of the live catalog.

**Subject is the human.** Law keeps **person**. A name or login is not the person. Ask · Keep · Talk are teachable loops — not site nav.

Customer voice: **live** / **planned**. Pages chips: **Works here** · **Works in Poet / Desktop** · **Planned**. Never “unavailable” or “broken”.

Hot-edit: write a script, run it, change it without rebuilding the app.

## Honesty

| Claim | Truth on QualiaDB `0.0.38` |
|---|---|
| Catalog size | Live named work grew (~905 → **1121** at Wave 40 / `7928e3d`). Count the tip; do not hardcode a forever number. **104** families named in plain words on What it can do. |
| Math | First-class family (`LinearAlgebra.gemm`, `dot`, `norm`, …). See core §11.5. **Works here.** |
| Public relay | **Planned**, not built. No invented tick. |
| QDNF / public relay | Map, not a finished mesh. A name or login is not the person. |
| Bundled Pages WASM | `docs/pkg/vibe` is `vibe-wasm 0.0.38` built from QualiaDB tag `v0.0.38` (web target). Math and Rules / Shapes run on this page (`honesty: "local"` residual — not a fake product). Volumes, devices, and a model live in Poet / Desktop. |

Summaries in this repo: [`docs/language-contract.md`](docs/language-contract.md), [`docs/diagnose.md`](docs/diagnose.md), [`docs/catalog-honesty.md`](docs/catalog-honesty.md). Qualia operator pages: [`progress-0.0.38.html`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/progress-0.0.38.html), [`qdnf.html`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/qdnf.html).

## Layout

| Path | What |
|---|---|
| `crates/vibe-lsp` | LSP (stdin/stdout JSON-RPC). Path-depends on a local QualiaDB `crates/vibe` checkout. |
| `editors/vscode` | `.vibe` highlighting + language config |
| `grammar/` | Copy of engine EBNF / GBNF / schema for tools |
| `playground/` | Static samples + optional `vibe-play` HTTP host |
| `skills/SKILL.md` | Agent authoring rules |
| `docs/` | Pages site + language / diagnose / catalog honesty |
| `JOBS.md` | Remaining work vs what already landed in QualiaDB |

## Build the LSP

From this directory, with QualiaDB `0.0.38` (or a compatible tip) checked out as the sibling the Cargo path-dep expects (`../qualia-27062026/crates/vibe` on Timothy’s disk):

```
cargo test -p vibe-lsp
cargo run -p vibe-lsp
```

Point VS Code `vibe.lsp.path` at the resulting `vibe-lsp` binary.

## Do not

- Do not copy `crates/vibe` into this repo.
- Do not bump `workspace.package.version` for a docs/catalog alignment.
- Do not invent family ids or a public relay.
- Do not teach `capability.invoke` as the human workshop dialect.
