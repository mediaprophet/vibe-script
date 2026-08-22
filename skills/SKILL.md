# Skill: VibeScript (`vibe-0.1`)

You are authoring **Vibe**, a typed language for humans and machines (the JS replacement). **Poet** is a host, not the language. Do not invent JavaScript or Python APIs.

Engine: QualiaDB `crates/vibe`. Tools (this repo): LSP, highlighting, playground, docs.

## Read first

1. QualiaDB `docs/manuals/standards/vibescript-core.md` — grammar, types, effects, fixtures.
2. `using Family;` then `Family.method(...)`. Unknown catalog paths fail closed (E100). Missing lease is E300.
3. `vibe::diagnose(src)` → JSON (`error_code`, `span`, `suggested_fix`, `errors` up to 8). Do not execute invalid source.

## Hard rules

- Quin construction is `quin.statement(...)` only. `<<[ s p o g prov ]>>` is illegal.
- RDF 1.2 only: `<<( s p o )>>` and `<< s p o ~ reifier >>`.
- Workshop dialect: `using Animation;` then `Animation.orbit_spin(t)`. Do not teach `capability.invoke` in human samples.
- `graph.query` / `graph?` always need a grant (`using GraphDatabase` or `graph.read`) and `take`.
- Cells (`= …` and default `cell`) are Pure — no pulse, write, or `time.now` unless `effect cell`.
- Diagnostics: `E001` parse, `E100` type, `E200` effect, `E300` capability, `E400` budget, `E500` policy, `E600` eval.
- Modal verbs (`obligate`, `knows`, …) stay **terms** unless the engine family is leased; never stamp `"Active"`.
- Vocab: `prefix snomed: <http://snomed.info/id/>;` plus a vocab chunk. Do not ingest all of SNOMED. Canonical IRIs, no rename layer.
- Non-capturing `|x| expr` is in 0.1. Capturing closures are not.

## Grammar

- `grammar/vibe-0.1.ebnf`
- `grammar/vibe-0.1.gbnf` (in-process constrained decode, not Ollama)
- `grammar/source.schema.json`

## Fixtures (QualiaDB `crates/vibe/fixtures/`)

- `lamp.vibe` — present + orbit + HID (evals on LocalHost and PoetHost)
- `12_1_cell.vibe`, `12_2_clinic.vibe`, `12_3_count.vibe`
- Reject: `n3_quin_overlay.vibe`
