# Supervisor qualification of tooling implementation

Date: 2026-11-06. Result: scoped local tooling implementation accepted.
Full ABI conformance, immutable-source CI and production governance remain open.

## Independent review and corrections

Implementation: gpt-6.1-sol medium, task tooling_implementation. Independent
read-only reviewer: gpt-6.1-sol medium, task tooling_plan_review. Reviewer inspected
the scoped diff, exporter/tests, LSP/HTTP/HTML consumers, manifests/lock and CI;
no actionable P1/P2 tooling defects found, conditional on supervisor checks.
The parent supervisor performed the following checks, not just a review of reports.

Strict UTF-8 validation found Windows-1252 bytes in the new preview and three
documentation files. Converted only those files to UTF-8 and added the
`descriptor_preview_is_valid_utf8` regression. This adds one test to the worker's
37 passing tests. No runtime semantics or Qualia source changed in this correction.

## Source identity and isolated harness

Tooling committed baseline: cae21724cfb6b3a1032c8ff1f0b819a656be1c33, plus the
uncommitted candidate identified by the source manifest below.
Qualia committed baseline: 3910f89697fbcf1d829a073644af9ec6ea9ab1c9, plus its
already-present uncommitted engine/ABI source. The commit alone is insufficient.

Evidence directory:
C:/Projects/qualia-27062026/target/vs03-tooling-review-20261106

The supervisor captured 434 actual engine/ABI files (3,219,408 bytes), verified
copy hashes, and verified all original source hashes again after final testing.
Tooling code/manifests/assets were separately captured and compared to the final
working files. Source capture was bounded at 16 MiB per source collection.

- engine-source-manifest.json: every copied engine/ABI file SHA-256 and byte count.
- tooling-final-source-manifest.json: final tested tooling file SHA-256 and sizes.
- qualia-original-Cargo.toml: retained original workspace manifest.
- qualia-27062026/Cargo.toml: minimal validation workspace supplying the same two
  inherited engine dependencies (ABI path and bytemuck 1.25.2). This is a test
  harness, not a claim that the full production Qualia workspace was qualified.

## Actual checks

Environment: Windows, Rust/Cargo 1.95.0, CARGO_BUILD_JOBS=1, dedicated build target.
No shared Qualia build target was used.

1. `cargo test --manifest-path <evidence>/vibe-script/Cargo.toml --workspace --offline --locked`
   final exit 0: **38 passed, 0 failed, 1 ignored owner-defect probe**.
   LSP 19, playground 9, tooling unit 2, conformance 4, dispatch 2, limits/digests 2.
   Full output: workspace-tests-final.log. Earlier 37-test run retained separately.
2. `cargo test --manifest-path <evidence>/vibe-script/Cargo.toml -p vibe-tooling --test conformance --offline --locked upstream_duplicate_stable_id_must_be_rejected -- --ignored --exact`
   independently reproduced the defect, exit 101. Registry returned Ok(0) instead
   of Err(DuplicateStableFuncId). See upstream-duplicate-id-probe.log. This is an
   observed failure, not a passing conformance test or a fully green ABI gate.
3. Actual CLI export succeeded. Recomputed its compact manifest JSON digest using
   PowerShell/.NET SHA-256, independently of Rust sha2:
   1c192d29c85f59cf6113990125fc9becaf77432e310205edc347f432abd6a62c.
   Output: descriptor-export.json. This verifies export serialization only.
4. Actual preflight rejected missing source, wrong commit and uncommitted required
   engine files. Exact outcomes: ci-negative-results.json.
5. `git diff --check` passed. Relevant engine/ABI source hashes remained unchanged.

## Acceptance boundary and next action

Accept the local descriptor export, conformance tooling, discovery-only LSP/HTTP
interfaces and CI failure-path implementation. Retain the provider-declared digest
as unverified; tooling export hashing cannot certify it. No executable language
extension calls, Sentinel authorization, hosted CI or production deployment were
established by this work.

Next owner actions: repair duplicate stable-ID registration; replace reference
contract digest placeholders with the owner-defined canonical encoding/digests;
provide a retrievable commit containing the ABI/registry. Then update the engine
source lock, run clean committed-source checks and remove the ignored regression
only after its original rejection assertion passes. Keep qualified=false until
that evidence exists. No commits, pushes or deployment occurred in this task.
