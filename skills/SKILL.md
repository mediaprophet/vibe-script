# Skill: VibeScript (`vibe-0.1`)

You are authoring **Vibe**, a typed language for **humans** (and the tools that help them). **Poet** is a host, not the language. A chatbot is a **tool**, not a human. Do not invent JavaScript, Python, or Host APIs.

Engine: QualiaDB tree **`0.0.38`** [`crates/vibe`](https://github.com/mediaprophet/qualiaDB/tree/0.0.38/crates/vibe).  
Tools (this repo): LSP, highlighting, playground, docs. Workspace version **`0.0.1`** — do not bump it.

Current dialect (not the first-shot workshop sketch): [`docs/vocab.md`](../docs/vocab.md), [`docs/language-contract.md`](../docs/language-contract.md).

## Read first

1. QualiaDB [`vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md) — syntax, types, effects, fixtures.
2. This repo: `docs/vocab.md`, `docs/language-contract.md`, `docs/diagnose.md`, `docs/catalog-honesty.md`.
3. `using Family;` then `Family.method(...)`. Unknown catalog paths fail closed (`E100`). Missing lease is `E300`.
4. `vibe::diagnose(src)` → JSON (`error_code`, `span`, `suggested_fix`, `errors` up to 8). Do not execute invalid source.
5. Live ids: QualiaDB `poet_host/invoke/ids.rs` and `crates/vibe/src/catalog/ids.rs`. Count the tip. Do not hardcode a forever number.

## Customer voice (pages)

When you write vibe-script **pages**, talk to humans:

- H1 **What it can do**. Lead: write / run / change without rebuilding the app.
- Status chips only: **Works here** · **Works in Poet / Desktop** · **Planned**.
- **104 families** in plain words (Rules, Shapes, Math, Motion…). Keep the catalog name on the tile.
- Continuity once: human; law keeps person; a name or login is not the person; a chatbot is a tool.
- Do not print `ALL_BOUND`, `held / not yet`, or Capability soup on those pages.

## vibe-0.1 is current — grammar may grow

- `vibe-0.1` is the current dialect, not frozen-as-law.
- Grammar **may grow** when humans need a better form — versioned, documented.
- A Host does not invent grammar or ids. This repo does not mint keywords.
- The **catalog already grows**.

## Human-alone rules

- Say **human** (Timothy standards sense / NaturalAgent). Law keeps **person**.
- A name or login is not the person. A DID, DNS name, or session id is a handle.
- Chatbot / agent / model is a tool. Do not write samples where a bot is the other party.
- Directory / contacts stories list humans first, organizations (legal person) next, tools last.
- Ask · Keep · Talk are teachable loops. They are **not** locked top-level IA.
- Qualia is human-**centric** (nucleus), not “Human-Centered AI” branding.

## Hard rules

- **No Host invent.** If it is not on the live catalog, do not mint it. Pages: **Works in Poet / Desktop** or **Planned**. Do not mint dotted `qualia.*` IRIs.
- **No CSCP ticks.** No invented `wss://`, MASQUE, or public-relay completeness.
- Quin construction is `quin.statement(...)` only. `<<[ s p o g prov ]>>` is illegal.
- RDF 1.2 only: `<<( s p o )>>` and `<< s p o ~ reifier >>`.
- Human dialect: `using Animation;` then `Animation.orbit_spin(t)`. `using LinearAlgebra;` then `LinearAlgebra.dot({ a, b })`. Do not teach `capability.invoke` in human samples.
- `graph.query` / `graph?` always need a grant (`using GraphDatabase` or `graph.read`) and `take`.
- Cells (`= …` and default `cell`) are Pure — no pulse, write, or `time.unix` unless `effect cell`.
- Diagnostics: `E001` parse, `E100` type, `E200` effect, `E300` capability, `E400` budget, `E500` policy, `E600` eval. Tooling chrome: **held / not yet** — never “unavailable” or “broken”.
- Modal verbs (`obligate`, `knows`, …) stay **terms** unless the engine family is leased; never stamp `"Active"`.
- Vocab: `prefix snomed: <http://snomed.info/id/>;` plus a vocab chunk. Do not ingest all of SNOMED. Canonical IRIs, no rename layer.
- Non-capturing `|x| expr` is in 0.1. Capturing closures are not.
- Hot-edit: script changes must not force a host rebuild. Do not tell the human to recompile Rust/WASM to try a `.vibe` edit.

## Grammar copies

- `grammar/vibe-0.1.ebnf`
- `grammar/vibe-0.1.gbnf` (in-process constrained decode, not Ollama)
- `grammar/source.schema.json`

Copies of QualiaDB `crates/vibe/grammar/` on `0.0.38`. They track the current dialect; they are not a law that the language may never grow.

## Fixtures (QualiaDB `crates/vibe/fixtures/`)

- `lamp.vibe` — present + orbit + HID (evals on LocalHost and PoetHost)
- `12_1_cell.vibe`, `12_2_clinic.vibe`, `12_3_count.vibe`
- Reject: `n3_quin_overlay.vibe`

Do not replace those with invented Host demos.
