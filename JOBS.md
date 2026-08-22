# VibeScript 0.1 — remaining jobs

**Split:** QualiaDB `C:\Projects\qualia-27062026\crates\vibe` is the **language engine**. This repo is **tools** (LSP, editors, playground, agent files, docs). Do not fork the engine here.

0.1 dialect is **frozen**. Do not add grammar in this repo.

---

## A. Language engine (QualiaDB) — remaining polish only

These are **not** blockers for LSP/highlighting. They are host/runtime or principal.

| ID | Job | Status |
|---|---|---|
| A1 | Spec §3 + EBNF match implementation (`bind`, lambda, tween, `using`, numeric prefixed names) | Done this wave |
| A2 | Bytecode compile of lambda/tween/graph/modal | Out of 0.1 authoring path; AST eval is canonical |
| A3 | P15 / P16.4 GPU 512 / HWND / WASM canvas | Host/GPU; fail closed |
| A4 | P16.8 Poet REPL + GPU introspection | Poet product |
| A5 | P17.5 vocab hash-lock vs latest | **Principal** — do not pick |
| A6 | Lorentz frame morphism | Honest stub; not dialect |

---

## B. This repo — ecosystem assets

| ID | Job | Status | Notes |
|---|---|---|---|
| E1 | **LSP** `crates/vibe-lsp` | ✅ Complete | Diagnose up to 8 diagnostics; `Family.` completion; hover; format (`project_program`); code actions; 14 tests pass. |
| E2 | **Syntax highlighting** | ✅ Complete | TextMate `editors/vscode/syntaxes/vibe.tmLanguage.json` with full keyword, operator, and token grammar. |
| E3 | **VS Code extension** | ✅ Complete | `editors/vscode/`: package.json, language-configuration.json, extension.js client wired to `vibe.lsp.path`. |
| E4 | **Playground** | ✅ Complete | `crates/vibe-play`: HTTP server on port 7420 serving playground, docs, `/api/diagnose`, `/api/eval`, `/api/format`, static routes; responsive UI in `playground/index.html`. |
| E5 | **Tooltips / hover** | ✅ Complete | `catalog_intel.rs`: keyword hover, catalog method descriptions, and vocab-chunk hover with IRI & label for `snomed:…`. |
| E6 | **Agent helper** | ✅ Complete | `skills/SKILL.md`: workshop dialect, hard rules, grammar pointers, and fixture guides. |
| E7 | **Dev-docs generator** | ✅ Complete | `crates/vibe-docs`: scans `crates/vibe/src`, extracts 104 modules / 1290 public items into `docs/dev-docs.json`. |
| E8 | **Docs / pages** | ✅ Complete | `docs/`: `index.html`, `showcase.html`, `spec.html`, `dev-docs.html` with real-time search & filter. |
| E9 | **CI** | ✅ Complete | `.github/workflows/ci.yml`: runs `cargo test --workspace` and validates LSP catalog completions. |

---

## C. Do not

- Do not copy `crates/vibe` into this repo.
- Do not add grammar keywords for catalog methods.
- Do not teach `capability.invoke` as the human dialect.
- Do not claim GPU/HWND in the playground until P15 is real.
- Do not put more editor/plugin crates into QualiaDB.

QualiaDB still has in-tree drafts (`crates/vibe-lsp`, `docs/vibe/*`). They stay until Timothy says to delete them; **new work lands here**.
