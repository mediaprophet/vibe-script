# VT01–VT05 / canonical consumption candidate receipt

Date: 2026-11-06. Worker: /root/tooling_implementation. Status: AWAITING_REVIEW.
Parent /root is supervisor; worker does not self-accept. No active writer after this checkpoint.

## Scope and changes

One coherent tooling candidate consumes actual uncommitted Qualia ABI/registry
sources. New `crates/vibe-tooling` exports every descriptor semantic field and actual
ontology content with explicitly specified serialization, full SHA-256 and bounded
failure behavior. Includes CLI, direct external ABI/registry/dispatch checks, limit
exhaustion and digest mutation vectors. LSP custom request and playground HTTP/page
use this shared export; no new executable language names or authority claims.
CI replaces successful missing-engine skip with immutable sibling checkout/preflight
and locked tests; source lock remains intentionally unqualified.
README/JOBS, source inventory, schema, bridge and CI docs record actual behavior.
Existing setup links/policy and all five plan files preserved.

Baseline tooling HEAD cae21724cfb6b3a1032c8ff1f0b819a656be1c33;
Qualia observed HEAD 3910f89697fbcf1d829a073644af9ec6ea9ab1c9, dirty/uncommitted
ABI/registry source. Parent captured 434 actual files at
C:/Projects/qualia-27062026/target/vs03-tooling-review-20261106/qualia-27062026.
Worker measured identical hashes for 434 files before/after its workspace tests;
worker hash list: vibe-script/target/vs03-source-before.txt. No engine writes.

## Commands and results

Environment: Windows PowerShell; cargo from C:/Users/Admin/.cargo/bin/cargo.exe,
default Rust 1.95.0; CARGO_BUILD_JOBS=1;
CARGO_TARGET_DIR=C:/Projects/vibe-script/target/vs03.

- Initial `cargo test --workspace --offline`: failed because new LSP dependency
  accidentally landed in [[bin]]; corrected [dependencies] placement. No source
  semantic failure was hidden. Subsequent tests use corrected manifest.
- `cargo test --workspace --offline`: passed after bounded-export/tests edits.
- Final code `cargo test --workspace --offline --locked`: exit 0; 37 passed,
  0 failed, 1 ignored. LSP 19; playground 8; tooling unit 2, external registry/ABI 4,
  dispatch 2, export limits/mutation 2. Docs generator and doc-tests contain 0 tests.
- `cargo test -p vibe-tooling --test conformance --offline --locked
  upstream_duplicate_stable_id_must_be_rejected -- --ignored --exact`: failed as
  intended owner regression probe, 0 passed/1 failed; registry returned Ok(0) where
  Err(DuplicateStableFuncId) required. Normal tooling tests explicitly ignore this
  upstream test; it is NOT a passing ABI qualification result.
- `tools/ci/verify-engine.ps1 -EngineRoot ./target/vs03-nonexistent-source`:
  rejected missing engine source.
- Same helper with temporary source lock revision of 40 zeroes: rejected mismatch
  against observed HEAD 3910f89697fbcf1d829a073644af9ec6ea9ab1c9.
- Default helper: rejected uncommitted required
  crates/vibe/src/extension_registry/mod.rs. Parent independently checked same cases;
  evidence C:/Projects/qualia-27062026/target/vs03-tooling-review-20261106/ci-negative-results.json.
- Engine/ABI source-hash compare: 434 files unchanged after workspace tests.

No full Qualia suite, hosted CI, commits, pushes, deployment or generated-site/WASM
refresh. Cargo.lock adds tooling/ABI/SHA-256 dependencies and resolves vibe 0.0.40;
no broad upgrade. Snapshot lock qualification remains parent's independent check.

## Unresolved owner actions and next steps

1. Parent independently reviews this stable candidate and runs final locked tooling
   tests against captured source; worker is stopped with no active process.
2. Qualia must publish retrievable ABI/registry source commit and fix duplicate
   StableFuncId registration; contract digest placeholders need owner serialization
   and real hashes. Tooling export hash deliberately does not claim equivalence.
3. Replace tools/ci/engine-source.json revision with published full commit; verify
   committed/clean sibling source and tests before marking qualified true.
4. Production host/governance and source-language extension dispatch remain owner
   work. Tooling metadata discovery does not qualify those paths.

All candidate additions are focused modules below 500 lines. Existing LSP server
ownership review is in inventory.md. Detailed schema/limits in
../../../../extensions/tooling-manifest.md (repository path docs/extensions/tooling-manifest.md).
