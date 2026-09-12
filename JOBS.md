# VibeScript 0.1 — remaining jobs

**Split:** QualiaDB tree [`0.0.38` `crates/vibe`](https://github.com/mediaprophet/qualiaDB/tree/0.0.38/crates/vibe) is the **language engine**. This repo is **tools** (LSP, editors, playground, agent files, docs). Do not fork the engine here.

0.1 **grammar** is closed. The **Host catalog grows** (core §11.5). Do not add grammar in this repo. Do not treat `vibe-host-0.1` as a freeze of `ALL_BOUND`. Drop stale “~885 freeze” language.

Workspace version stays **`0.0.1`**.

---

## A. Language engine (QualiaDB) — already landed vs remaining

These are **not** blockers for LSP / highlighting. They are host / runtime or principal.

| ID | Job | Status on QualiaDB `0.0.38` |
|---|---|---|
| A1 | Spec §3 + EBNF match implementation (`bind`, lambda, tween, `using`, numeric prefixed names) | Landed (grammar tests assert EBNF ↔ core §3) |
| A2 | Bytecode compile of lambda / tween / graph / modal | Out of 0.1 authoring path; AST eval is canonical |
| A3 | P15 / P16.4 GPU 512 / HWND / WASM canvas | Host / GPU; fail closed. No playground claim until real. |
| A4 | P16.8 Poet REPL + GPU introspection | Poet product |
| A5 | P17.5 vocab hash-lock vs latest | **Principal** — do not pick |
| A6 | Lorentz frame morphism | Honest stub; not dialect |
| A7 | Host catalog grow (Wave 40 / `7928e3d`) | **Landed** — `ALL_BOUND` ~905 → **1121** including `LinearAlgebra.*`. Count the tip. |
| A8 | `GraphDatabase.volume_open` / `volume_commit` | Landed (B-001). Native sanctuary; wasm E300. |
| A9 | Diagnose JSON `errors[]` | Landed |
| A10 | CSCP-08 Internet WSS / MASQUE, CSCP-12 datatracker | **Parked**. No invented ticks. |
| A11 | QDNF public relay / two-host Internet | **Held / not yet**. Handle ≠ human. |

Sprint-B rows B-001–B-009 are done or parked as standing OWL constraints. See Qualia [`vibescript-sprint-deltas.md`](https://github.com/mediaprophet/qualiaDB/blob/0.0.38/docs/manuals/standards/vibescript-sprint-deltas.md).

---

## B. This repo — ecosystem assets

| ID | Job | Status | Notes |
|---|---|---|---|
| E1 | **LSP** `crates/vibe-lsp` | Landed | Diagnose up to 8; `Family.` completion; hover; format; code actions. Reports workspace version `0.0.1`. |
| E2 | **Syntax highlighting** | Landed | TextMate includes live families (`LinearAlgebra`, `Cosmic`, …) as highlight names only — not a Host invent. |
| E3 | **VS Code extension** | Landed | `vibe.lsp.path` client. Version `0.0.1`. |
| E4 | **Playground** | Landed | `crates/vibe-play` + `playground/index.html`. Samples must follow diagnose / catalog honesty. |
| E5 | **Tooltips / hover** | Landed | `catalog_intel.rs` reads the **engine** catalog — do not hardcode a forever id count. |
| E6 | **Agent helper** | Landed | `skills/SKILL.md`: human-alone, no invent ids, held / not yet. |
| E7 | **Dev-docs generator** | Landed | `crates/vibe-docs` scans sibling QualiaDB `crates/vibe/src`. Refresh when Neo updates the sibling. |
| E8 | **Docs / pages** | Landed | Language / diagnose / catalog honesty summaries + Pages site. WASM bundle may lag `0.0.38`. |
| E9 | **CI** | Landed | `.github/workflows/ci.yml` tests when the QualiaDB sibling is present. |
| E10 | **Refresh bundled `docs/pkg/vibe` WASM** | Remaining | Pages ships a snapshot (package stamp currently `0.0.34`). It can E100 `LinearAlgebra.dot` (live on `0.0.38` `ALL_BOUND`) and even suggest a non-catalog id. Rebuild from QualiaDB `vibe-wasm` when Neo publishes; do not vendor the engine source. Do not “fix” samples by inventing the snapshot’s guess. |
| E11 | **Hot-edit in this playground** | Remaining (north star) | Script edits must not imply a host rebuild. Browser WASM eval is local; claiming live Poet furniture here is held / not yet. |

---

## C. Do not

- Do not copy `crates/vibe` into this repo.
- Do not add grammar keywords for catalog methods.
- Do not teach `capability.invoke` as the human dialect.
- Do not invent Host ids, dotted `qualia.*`, CSCP ticks, or MASQUE / `wss://`.
- Do not claim GPU / HWND / public QDNF relay in the playground.
- Do not lock Ask · Keep · Talk as top-level IA on these pages.
- Do not bump `workspace.package.version` for catalog / docs alignment.
- Do not put more editor / plugin crates into QualiaDB.

QualiaDB still has in-tree drafts (`crates/vibe-lsp`, `docs/vibe/*`). They stay until Timothy says to delete them; **new work lands here**.
