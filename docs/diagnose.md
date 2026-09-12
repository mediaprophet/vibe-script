# Diagnose voice (ecosystem)

**Engine API:** `vibe::diagnose(src)` on QualiaDB `crates/vibe`.  
Returns JSON with `error_code`, `span`, `suggested_fix`, and `errors[]` (up to eight) on failure. Do not execute invalid source.

This repo’s LSP (`crates/vibe-lsp`) and playgrounds wrap that report. They must not invent a second diagnostic language.

## Voice: live / planned

Wait-honest chrome and tooling say **live** or **planned** — plus a short why.

Pages chips: **Works here** · **Works in Poet / Desktop** · **Planned**.

Never paint user-visible **unavailable** or **broken**. Those words are theatre. A missing lease, a WASM-only snapshot, or a daemon that is not paired is *planned* (or “Works in Poet / Desktop”), not a smashed product.

Live binds must not be marked planned. If the catalog has the id and this page (or Poet / Desktop) implements it, diagnose the call as live.

Workshop first. Prefer:

```vibe
using LinearAlgebra;
LinearAlgebra.dot({ a: xs, b: ys })
```

over teaching agents to emit `capability.invoke("LinearAlgebra.dot", {…})` in workshop samples.

## Stable codes (engine)

| Code | Class |
|---|---|
| `E001` | parse |
| `E100` | type / unknown binding |
| `E200` | effect |
| `E300` | capability / missing lease |
| `E400` | budget |
| `E500` | policy |
| `E600` | eval |
| `E701` | mut violation |
| `E702` | capability not attached on this target (warning) |

A safe suggested fix MUST NOT add authority.

## Tooling extras (this repo)

`QDB0402` is **tooling metadata**, not Vibe grammar. On `wasm-standalone` the LSP may add:

| Mode | Meaning |
|---|---|
| `native-bridge` | Needs a paired local QualiaDB daemon. **Planned** on standalone WASM. |
| `standalone-snapshot` | Runs against an isolated in-memory graph. Not a persistent native transaction. |

Do not describe either mode as broken. Do not invent a public-relay path to “fix” them.

## Cross-frame spans

Diagnose spans are UTF-8 byte ranges on the **source cell**. Timeline glow maps those bytes per frame. That does not need a new family method (sprint B-007).

## Agent loop

1. Read `docs/language-contract.md` and Qualia core §3 / §9 / §11.5.
2. `diagnose(src)` before eval.
3. If the report is not valid: show codes + spans + suggested fixes. Stop.
4. If a family is unleased: add `using Family;` only when that family is live on the catalog.
5. If the id is not on the live catalog: **planned**. Do not mint `qualia.*`.
