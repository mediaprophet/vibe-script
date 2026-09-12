# Vibe 0.1 language contract (ecosystem summary)

**Normative source:** QualiaDB `0.0.38` [`vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md).  
This file is a human-readable contract for tools in *this* repo. It is not a second spec.

**Customer voice:** **live** (runs now) · **planned** (not built). Pages chips: **Works here** · **Works in Poet / Desktop** · **Planned**.

## What 0.1 is

`vibe-0.1` is the **current dialect** — a typed, capability-bounded, non-JIT interpreted REPL language for:

- reactive HCF cell formulas (Pure only);
- signed document / agent modules that query and transact on a graph snapshot;
- event handlers (`on pulse…`, `on ui…`) that Poet (or any place that runs Vibe) dispatches;
- application support — scripts that call named families so Poet and Desktop can host apps, not only workshop cells.

**Vibe is the language.** Poet is a place that runs it (UI / CLI / habitat). Pulse is transport. Aura is ontology / schema.

Workspace / crate version stays **`0.0.1`**. Engine truth is QualiaDB `v0.0.38` `crates/vibe`. Do not fork the engine here.

## Grammar may grow

`vibe-0.1` is the current dialect. The grammar is **not** locked, closed, or frozen.

- Grammar **may grow** when humans need a better form.
- Growth is **versioned** and **documented**. This repo copies grammar from the engine; do not invent a private dialect here.
- Adding a catalog id MUST NOT silently change types or effect classes.
- Do not invent families or dotted `qualia.*` ids.

The catalog of named work also grows. `vibe-host-0.1` is an outcome of incorporating live libraries — not a freeze of the catalog. Historical “~885 freeze” wording is sprint-of-record only.

## Human dialect

Authors write **human** sayables:

```vibe
using Animation, LinearAlgebra;
cell score := LinearAlgebra.dot({ a: xs, b: ys });
present lamp { color: #ff8800 }
```

Workshop first: `using LinearAlgebra;` then `LinearAlgebra.dot({…})`. Do not teach `capability.invoke` as the human dialect.

English keywords are always legal. Additional keyword locales are opt-in (`locale zh;`) and MUST round-trip on the AST.

## Human, person, handle, tool

**Subject is the human.** Law keeps the word **person**. A name or login is not the person.

| Word | Use |
|---|---|
| **human** | The living subject. Directory lists humans first. |
| **person** | Law / jural who-kind. Keep this word in law text. Not the subject of this language. |
| **handle** | DID, DNS name, session id, locator. A name or login is not the person. Revoking a handle is not who-erase. |
| **chatbot / agent** | A tool. Not the other party. Not a human. |
| **organization** | Legal-person who-kind, after humans, before tools. |

Qualia is **human-centric** (structural nucleus), not “Human-Centered AI” as a methodology label.

Do not model humans / personhood / sacred or living-natural relations as `owl:Thing` subclasses. Technical artifacts may still use OWL where Thing is apt.

## 104 families, in plain words

What it can do names all **104** families (plus lowercase `agent`) on a searchable map. Lead labels are human words. Catalog ids stay on the tile for search:

Rules · Shapes · Ought · Knowing · Math · Graphs · Motion · and the rest.

Status on Pages:

- **Works here** — this browser snapshot runs it.
- **Works in Poet / Desktop** — live on the catalog; the place that runs it is Poet / Desktop.
- **Planned** — not built. Do not invent a tick.

Implemented-but-dead is a bind to finish, not a fourth chip.

## Teachable loops are not site nav

**Ask · Keep · Talk** are teachable loops in Poet. They are **not** top-level information architecture for this language or for vibe-script pages. Do not invent families or site nav around those three words.

## Hot-edit north star

Write a script. Run it. Change it without rebuilding the app.

Vibe evaluates `present` blocks, UI records, and reactive cells. The place that runs it reconciles furniture in place. That is core §17. Pages ships `vibe-wasm 0.0.38` from QualiaDB tag `v0.0.38`.

## Effects, RDF, Quin

- Cells (`= …` and default `cell`) are Pure — no pulse, write, or clock unless `effect cell`.
- `time.unix()` is the 0.1 time binding (External). Structured `Instant` / nanos clocks are planned.
- RDF 1.2 only: `<<( s p o )>>` and `<< s p o ~ reifier >>`.
- Quin construction is `quin.statement(...)` only. `<<[ s p o g prov ]>>` is illegal.
- `graph.query` / `graph?` always need a grant (`using GraphDatabase` or `graph.read`) and `take`.
- Modal verbs (`obligate`, `knows`, …) stay **terms** unless the family is leased; never stamp `"Active"`.
- Non-capturing `|x| expr` is in 0.1. Capturing closures are planned.

## Live named work (cite the catalog)

§11 of core lists **0.1 library bindings** (`math.*`, `rdf.*`, `quin.statement`, `graph.*`, `aura.validate`, `pulse.publish`, `capability.resolve`). Those are not an invitation to paste every later table into this repo.

Named families that are **live** on QualiaDB `v0.0.38` (cite the catalog, do not invent) include Math (`LinearAlgebra.*`, §11.5), Cosmic, Econ, Orchestration, Graphs, Render, Inference, Motion (`Animation.*`), Hands & gaze (`HID.*`). If an id is not on the live catalog, it is **planned**.

## Out of this repo

- No invented families. No dotted `qualia.*` ahead of the live catalog.
- No public-relay invent. A public relay is **planned**, not built.
- No copy of the QualiaDB engine into this tree.
