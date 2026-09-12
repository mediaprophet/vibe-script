# Skill: VibeScript (`vibe-0.1`)

You are authoring **Vibe**, a typed language for **humans** and machines (the JS replacement). **Poet** is a place that runs Vibe, not the language. A chatbot is a **tool**, not a human. Do not invent JavaScript, Python, or family APIs.

Engine: QualiaDB tree **`0.0.38`** [`crates/vibe`](https://github.com/mediaprophet/qualiaDB/tree/0.0.38/crates/vibe).  
Tools (this repo): LSP, highlighting, playground, docs. Workspace version **`0.0.1`** — do not bump it.

**Customer voice:** **live** (runs now) · **planned** (not built). Pages chips: **Works here** · **Works in Poet / Desktop** · **Planned**.

## Read first

1. QualiaDB [`vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md) — grammar, types, effects, fixtures. `vibe-0.1` is the current dialect. Grammar **may grow** when humans need a better form (versioned, documented). Catalog of named work also grows. §11.5 LinearAlgebra / Math is live.
2. This repo: `docs/language-contract.md`, `docs/diagnose.md`, `docs/catalog-honesty.md`.
3. `using LinearAlgebra;` then `LinearAlgebra.dot(...)`. Unknown names fail (`E100`). Missing lease is `E300`.
4. `vibe::diagnose(src)` → JSON (`error_code`, `span`, `suggested_fix`, `errors` up to 8). Do not execute invalid source.
5. Live ids: QualiaDB catalog on `0.0.38` (Wave 40 / `7928e3d` counted **1121** — count the tip, do not hardcode forever). What it can do names **104** families in plain words (Rules, Shapes, Math, Graphs, Motion…) with the real id for search.

## Human-alone rules

- **Subject is the human.** Law keeps **person**. Person is not the subject of this language.
- A name or login is not the person. A DID, DNS name, or session id is a handle — not who.
- Chatbot / agent / model is a tool. Do not write samples where a bot is the other party.
- Directory / contacts stories list humans first, organizations (legal person) next, tools last.
- Ask · Keep · Talk are teachable loops. They are **not** site nav. Do not invent families around them.
- Qualia is human-**centric** (nucleus), not “Human-Centered AI” branding.

## Hard rules

- **No invented families.** If it is not on the live catalog, it is **planned**. Do not mint dotted `qualia.*` IRIs.
- **No public-relay invent.** A public relay is planned, not built.
- Quin construction is `quin.statement(...)` only. `<<[ s p o g prov ]>>` is illegal.
- RDF 1.2 only: `<<( s p o )>>` and `<< s p o ~ reifier >>`.
- Workshop dialect: `using Animation;` then `Animation.orbit_spin(t)`. `using LinearAlgebra;` then `LinearAlgebra.dot({ a, b })`. Do not teach `capability.invoke` in human samples.
- `graph.query` / `graph?` always need a grant (`using GraphDatabase` or `graph.read`) and `take`.
- Cells (`= …` and default `cell`) are Pure — no pulse, write, or `time.unix` unless `effect cell`.
- Diagnostics: `E001` parse, `E100` type, `E200` effect, `E300` capability, `E400` budget, `E500` policy, `E600` eval. Voice: **live** or **planned** — never “unavailable” or “broken”.
- Modal verbs (`obligate`, `knows`, …) stay **terms** unless the family is leased; never stamp `"Active"`.
- Vocab: `prefix snomed: <http://snomed.info/id/>;` plus a vocab chunk. Do not ingest all of SNOMED. Canonical IRIs, no rename layer.
- Non-capturing `|x| expr` is in 0.1. Capturing closures are planned.
- Hot-edit: write a script, run it, change it without rebuilding the app. Do not tell the human to recompile Rust/WASM to try a `.vibe` edit.
- Grammar is **not** locked. Growth is versioned and documented. Do not invent a private dialect in this repo.

## Grammar

- `grammar/vibe-0.1.ebnf`
- `grammar/vibe-0.1.gbnf` (in-process constrained decode, not Ollama)
- `grammar/source.schema.json`

Copies of QualiaDB `crates/vibe/grammar/` on `0.0.38`.

## Fixtures (QualiaDB `crates/vibe/fixtures/`)

- `lamp.vibe` — present + orbit + HID (evals on this page and in Poet / Desktop)
- `12_1_cell.vibe`, `12_2_clinic.vibe`, `12_3_count.vibe`
- Reject: `n3_quin_overlay.vibe`

Do not replace those with invented family demos.
