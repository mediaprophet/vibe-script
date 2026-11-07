# Tooling source inventory — 2026-11-06

Tooling baseline HEAD: cae21724cfb6b3a1032c8ff1f0b819a656be1c33.
Observed Qualia HEAD: 3910f89697fbcf1d829a073644af9ec6ea9ab1c9.
Qualia remote: https://github.com/mediaprophet/qualiaDB.
The actual ABI/registry files are uncommitted; HEAD is not a reproducible source pin.

| Consumer | Actual canonical source/API | Check |
|---|---|---|
| LSP | vibe diagnose/parse/projectional, catalog canonical_id/describe/families/methods, vocab clinic.n3 include, shared descriptor export | cargo test -p vibe-lsp --locked |
| Playground | vibe check/parse/diagnose/Engine/LocalHost/project_program; shared export | cargo test -p vibe-play --locked |
| Descriptor tools | vibescript-abi ProviderDescriptor/VibeNamespaceProvider, actual DeterministicMechanicsProvider; canonical registry/prepare/dispatch in tests | cargo test -p vibe-tooling --locked |
| Docs generator | scans sibling crates/vibe/src using CARGO_MANIFEST_DIR | cargo test -p vibe-docs --locked (no generator tests; assets not regenerated) |
| CI | full sibling Qualia checkout; root manifest inherited workspace deps; immutable source preflight | tools/ci/verify-engine.ps1 |

Path contract: caller-selected isolated parent containing sibling `vibe-script/`
and `qualia-27062026/`. Preserve the Qualia workspace root manifest and inherited
bytemuck/vibescript-abi dependencies; copying crates/vibe alone is insufficient.
The parent captured an isolated source snapshot under Qualia target for final review.
Worker builds used vibe-script/target/vs03, one job; no Qualia target build or core suite.
The worker hashed 434 engine+ABI files before/after tests and found no change.

Existing generated/copied `grammar/`, docs/dev-docs.json and WASM assets have unknown
immutable source provenance; this change does not regenerate or relabel them. Docs
site source extraction retains its current schema. README now corrects E3 wiring and
JOBS corrects prior CI completion claims. Package version 0.0.1, language axis 0.1.0,
ABI axis 0.3.0 and reference provider 0.4.2 are independent, taken from their owners.

Maintainability review: LSP server was already about 600 lines, owns one JSON-RPC
lifecycle, and receives a small discovery dispatch arm only. New exporter, conformance,
HTTP and LSP tests live in separate focused files below 500 lines. No decomposition
of unrelated server behavior was needed for this bounded integration.

Initial user edits preserved: README/JOBS plan links and prerelease policy; five
plan documents. No engine/Etalon edits, commits, pushes, deployment or cleanup.
