# Vibe 0.1 language contract (ecosystem summary)

**Normative source:** QualiaDB `0.0.38` [`vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md).  
This file is a human-readable contract for tools in *this* repo. It is not a second spec.

## What 0.1 is

Vibe 0.1 is a typed, capability-bounded, non-JIT interpreted REPL language for:

- reactive HCF cell formulas (Pure only);
- signed document / agent modules that query and transact on a graph snapshot;
- host event handlers (`on pulse…`, `on ui…`) that Poet (or any host) dispatches;
- application support — scripts that call Host families so Poet and other hosts can host apps, not only workshop cells.

**Vibe is the language.** Poet is a host (UI / CLI / habitat). Pulse is transport. Aura is ontology / schema.

## Closed grammar, growing catalog

- The **grammar is closed**. Do not add keywords in this repo.
- The **Host catalog grows**. `ALL_BOUND` / `Family.method` MAY gain new ids when they improve the app / REPL surface.
- Adding a Host id MUST NOT change grammar, types, or effect classes.
- `vibe-host-0.1` is the **outcome** of incorporating existing engine libraries — **not** a freeze of `ALL_BOUND`.
- Historical “no Host widen” / “~885 freeze” wording is sprint-of-record only. Current policy is core §11.5 + the refreshed sprint-deltas header (2026-09-11).

## Human dialect

Authors write **human** sayables:

```vibe
using Animation, LinearAlgebra;
cell score := LinearAlgebra.dot({ a: xs, b: ys });
present lamp { color: #ff8800 }
```

`capability.invoke("Family.method", {…})` remains the JNI / catalog spelling. Both are the same language; `using` is the lease. **Do not teach `capability.invoke` as the human workshop dialect.**

English keywords are always legal. Additional keyword locales are opt-in (`locale zh;`) and MUST round-trip on the AST.

## Human, person, handle, tool

From Timothy’s standards sense / NaturalAgent (see Qualia [`human-centric-nomenclature.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/human-centric-nomenclature.md) and `progress-0.0.38.html`):

| Word | Use |
|---|---|
| **human** | Living NaturalAgent. The nucleus. Directory lists humans first. |
| **person** | Law / jural / legal-person who-kind. Keep this word in law text. |
| **handle** | DID, DNS name, session id, locator. Handle ≠ human. Revoking a handle is not who-erase. |
| **chatbot / agent** | A tool. Not the other party. Not a human. |
| **organization** | Legal-person who-kind, after humans, before tools. |

Qualia is **human-centric** (structural nucleus), not “Human-Centered AI” as a methodology label.

Do not model humans / personhood / sacred or living-natural relations as `owl:Thing` subclasses (standing B-OWL-PERSON / B-OWL-NATURAL / B-OWL-LIFE-UPLIFT). Technical artifacts may still use OWL where Thing is apt.

## Teachable loops are not locked IA

**Ask · Keep · Talk** are teachable loops in the Poet habitat. They are **not** a locked top-level information architecture for this language or for vibe-script pages. Do not redesign this site (or invent Host families) around those three words as nav.

## Hot-edit north star

Script changes must not force a host rebuild.

Vibe evaluates `present` blocks, UI records, and reactive cells. The host reconciles furniture in place. That is core §17 — zero-recompilation workflow. Pages ships `vibe-wasm 0.0.38` from QualiaDB tag `v0.0.38`. Script edits must not force a host rebuild.

## Effects, RDF, Quin

- Cells (`= …` and default `cell`) are Pure — no pulse, write, or clock unless `effect cell`.
- `time.unix()` is the 0.1 time binding (External). Structured `Instant` / nanos clocks are post-0.1.
- RDF 1.2 only: `<<( s p o )>>` and `<< s p o ~ reifier >>`.
- Quin construction is `quin.statement(...)` only. `<<[ s p o g prov ]>>` is illegal.
- `graph.query` / `graph?` always need a grant (`using GraphDatabase` or `graph.read`) and `take`.
- Modal verbs (`obligate`, `knows`, …) stay **terms** unless the engine family is leased; never stamp `"Active"`.
- Non-capturing `|x| expr` is in 0.1. Capturing closures are not.

## Bindings vs Host families

§11 of core lists **0.1 library bindings** (`math.*`, `rdf.*`, `quin.statement`, `graph.*`, `aura.validate`, `pulse.publish`, `capability.resolve`). Those are not an invitation to paste every post-0.1 table into this repo.

Post-0.1 Host families that **are** live on `ALL_BOUND` (cite the catalog, do not invent) include `LinearAlgebra.*` (§11.5), `Cosmic.*`, `Econ.*`, `Orchestration.*`, `Asset.persist_*`, `GraphDatabase.*`, `Render.*`, `Inference.*`, `Animation.*`, `HID.*`. If an id is not on live `ALL_BOUND`, it is **held / not yet**.

## Out of this repo

- No Host invent. No dotted `qualia.*` ahead of live `ALL_BOUND`.
- No CSCP ticks. No MASQUE / public `wss://` invent.
- No claim that QDNF public relay or CSCP-08 / CSCP-12 is complete.
- No copy of the QualiaDB engine into this tree.
