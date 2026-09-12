# Vibe 0.1 language contract (current dialect)

**Normative source:** QualiaDB `0.0.38` [`vibescript-core.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-core.md).  
This file is the ecosystem contract for tools and pages in *this* repo. It is not a second spec. Word list: [`vocab.md`](vocab.md).

Workspace / crate version stays **`0.0.1`**. QualiaDB `0.0.38` is the engine tree this ecosystem tracks.

## What customers see

- Title: **What it can do** (`docs/reality.html`).
- Lead: **Write a script. Run it. Change it without rebuilding the app.**
- **It already does** names shipped work. A miss is a bug to finish.
- **104 families** in plain words. Status chips only: **Works here** · **Works in Poet / Desktop** · **Planned**.
- **Planned** is only work not in the product (today: a public internet relay).
- Continuity once: you are the **human**; the law still says **person**; a name or login is not the person; a chatbot is a tool.

Do not put `ALL_BOUND`, `held / not yet`, or Capability soup on customer pages.

## vibe-0.1 is current — grammar may grow

`vibe-0.1` is the **current dialect**, not a freeze-as-law.

- Grammar **may grow** when humans need a better form. New forms are **versioned and documented**.
- A **Host does not invent** grammar, keywords, or dotted ids.
- This repo does not mint keywords ahead of the engine.
- The **catalog already grows**. New live `Family.method` ids land when they improve the app / REPL surface. Adding a Host id MUST NOT silently invent grammar. Count the tip; do not hardcode a forever number.
- `vibe-host-0.1` is an **outcome** of incorporating live libraries — not a lock on a historic id count.
- Historical “grammar is closed” / “~885 freeze” / “no Host widen” wording is superseded.

## Human dialect

Authors write **human** sayables:

```vibe
using Animation, LinearAlgebra;
cell score := LinearAlgebra.dot({ a: xs, b: ys });
present lamp { color: #ff8800 }
```

`capability.invoke("Family.method", {…})` remains the JNI / catalog spelling. Both are the same language; `using` is the lease. **Do not teach `capability.invoke` as the human dialect.**

English keywords are always legal. Additional keyword locales are opt-in (`locale zh;`) and MUST round-trip on the AST.

## Human, person, handle, tool

From Timothy’s standards sense / NaturalAgent (see Qualia [`human-centric-nomenclature.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/human-centric-nomenclature.md) and `progress-0.0.38.html`):

| Word | Use |
|---|---|
| **human** | Living NaturalAgent. The nucleus. Directory lists humans first. |
| **person** | Law / jural / legal-person who-kind. Keep this word in law text. |
| **handle** | DID, DNS name, session id, locator. A name or login is not the person. |
| **chatbot / agent** | A tool. Not the other party. Not a human. |
| **organization** | Legal-person who-kind, after humans, before tools. |

Qualia is **human-centric** (structural nucleus), not “Human-Centered AI” as a methodology label.

Do not model humans / personhood / sacred or living-natural relations as `owl:Thing` subclasses (standing B-OWL-PERSON / B-OWL-NATURAL / B-OWL-LIFE-UPLIFT). Technical artifacts may still use OWL where Thing is apt.

## Teachable loops are not locked IA

**Ask · Keep · Talk** are teachable loops in the Poet habitat. They are **not** a locked top-level information architecture for this language or for vibe-script pages. Do not redesign this site (or invent Host families) around those three words as nav.

## Hot-edit north star

Script changes must not force a host rebuild.

Vibe evaluates `present` blocks, UI records, and reactive cells. The host reconciles furniture in place. That is core §17 — zero-recompilation workflow. Pages ships `vibe-wasm 0.0.38` from QualiaDB tag `v0.0.38`.

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

Live Host families that **are** on the catalog (cite `ids.rs`, do not invent) include `LinearAlgebra.*` (§11.5), `Cosmic.*`, `Econ.*`, `Orchestration.*`, `Asset.persist_*`, `GraphDatabase.*`, `Render.*`, `Inference.*`, `Animation.*`, `HID.*`, and the rest of the **104-family map**. If an id is not on the live catalog, pages say **Works in Poet / Desktop** or **Planned** — do not invent it.

## Out of this repo

- No Host invent. No dotted `qualia.*` ahead of the live catalog.
- No CSCP ticks. No MASQUE / public `wss://` invent.
- No claim that QDNF public relay or CSCP-08 / CSCP-12 is complete.
- No copy of the QualiaDB engine into this tree.
