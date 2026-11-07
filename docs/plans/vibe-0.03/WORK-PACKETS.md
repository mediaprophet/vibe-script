# Tooling work packets

2026-11-06 candidate update: actual canonical ABI/registry inputs now exist locally.
The authorized coherent VT01–VT05/VT07 candidate is checkpointed in STATE.md and
receipts/VT-IMPLEMENTATION.md; immutable source publication remains blocked.
Historical proposal-only/wait instructions below describe the initial plan, not the
current source inventory. Scoped local implementation is supervisor-accepted; full ABI and immutable CI gates remain open. See receipts/VT06-SUPERVISOR.md.

Original sequencing: execute in order except that VT03 can follow
VT01 independently of VT02. Allowlisted paths below are relative to `vibe-script`.
Every packet also updates its receipt and STATE.md. No packet writes QualiaDB,
Etalon or Poet. Proposals for those owners are recorded as external dependencies.

## VT01 Source and change-impact inventory

Owner: GPT-6.1 Sol worker, medium reasoning. Size: one focused session.
Read: root README.md/JOBS.md/Cargo.toml, crate manifests, CI, docs generator,
LSP catalog/target adapters and existing packaging scripts.
Write: `docs/plans/vibe-0.03/inventory.md` and packet receipt only.

Record exact engine API imports, source-path assumptions, generated/copied assets,
runtime profiles, current tests, file sizes and existing edits. Separate code facts
from historical JOBS completion claims. Record engine candidate commit, repository
URL and availability; no moving-branch pin. Define a caller-selected isolated build
root with sibling `vibe-script/` and `qualia-27062026/` snapshots, preserving workspace
metadata required by path dependencies. Never create a second implementation.

Acceptance: every affected consumer has a source and validation command; output
identifies a reproducible baseline, or an explicit retrieval/integration blocker.
Do not run the full Qualia core suite. Supervisor verifies path calculations and
checks the proposed source pin excludes uncommitted work.

## VT02 CI that executes tests

Depends: VT01. Owner: Sol medium. Size: one workflow packet; split if baseline fails.
Write: `.github/workflows/ci.yml`, a small pin/verification helper under `tools/ci/`,
`docs/plans/vibe-0.03/ci-baseline.md`; Cargo.lock only for a separately explained,
reviewed resolution change. Do not broadly update dependencies.

Check out exact tooling and Qualia sources into the sibling layout from VT01.
Set working directories explicitly. Pin the engine by full commit and record the
actual checkout revision. Missing/unavailable/mismatched engine is an error, never
a successful skip. Validate inherited workspace dependencies and docs-generator
paths. Use a packet-owned CARGO_TARGET_DIR; do not share or clean active targets.

Candidate validation: run `cargo test --workspace --locked` from the isolated
tooling snapshot with one build job. If the existing lock is incompatible, preserve
the failure and diagnose before proposing its minimum repair. Exercise missing
engine and mismatched revision as negative cases using the same preflight CI calls.
Keep generated docs/assets out of the commit unless separately validated.

Acceptance: actual tests execute against the recorded sources; both negative cases
fail; workflow paths and status propagation are reviewed. Hosted CI remains pending
until a real run occurs. Do not push/deploy merely to obtain evidence without an
instruction authorizing publication. A locally checked workflow is not hosted success.

## VT03 Proposed extension descriptors and fixtures

Depends: VT01. Owner: Sol medium. Size: one proposal/fixture packet.
Write: `docs/extensions/` and `fixtures/extensions/proposed/` only.

Define a small proposal using existing catalog facts: canonical identity, display
path/alias, provider version, signature, effect, target profile, requirements,
resource bounds and provenance/evidence status. Link semantic expectations to
Qualia-owned requirements. Include independently authored expected outcomes for
duplicate IDs, alias collisions, unavailable targets, missing required metadata,
unsupported versions, invalid bounds and declared-versus-authorized distinctions.

Acceptance: examples explicitly say proposed/simulated; unknown fields/versions
have defined handling; no native dispatch, authority implementation, copied engine,
authoritative new syntax or production ABI. A validation expectation is not an executed test.
Defer choosing canonical hashing, wire format and runtime semantics to Qualia.

## VT04 Tooling behavior against existing catalogs

Depends: VT02, VT03. Owner: Sol medium. Size: one consumer per packet iteration.
Write: focused LSP modules/tests under `crates/vibe-lsp/`; separately claim playground
modules/tests under `crates/vibe-play/` if needed. Read inventory file sizes first;
decompose large mixed-responsibility files before substantial additions.

Use the selected canonical engine catalog as executable authority. Improve completion,
hover and diagnostics for existing names and existing target profiles. Proposed
extension descriptors may be displayed only in a clearly separate documentation
preview; do not add synthetic calls to executable completion/validation. If a new
engine API is required, prepare a bounded owner packet with rationale, affected callers
and acceptance tests. Coordinate its implementation in QualiaDB; continue independent
tooling work while waiting. Do not build a compatibility workaround by default.

Acceptance: focused LSP tests cover known names, unknown names, aliases, target
availability and no false authorization claim. If playground changes, test its
affected routes separately. No new network probe or provider execution is implicit
in discovery. Reviewer traces each executable name back to the pinned engine.

## VT05 Documentation and asset provenance

Depends: VT02; incorporate VT03/VT04 results when available. Owner: Sol low/medium.
Write: README.md, JOBS.md, `docs/vibe-host-native-bridge.md`, targeted docs-generator
provenance support, and a bounded source manifest. Generated assets require an
explicit path claim; avoid regenerating the entire site for prose corrections.

Resolve README's outdated E3 wiring note and JOBS' overstated CI completion claim
using actual evidence. Document independent language/host/package versions and
Qualia ownership. Record source revision for generated grammar/docs/WASM assets;
unknown existing asset provenance stays unknown until reproduced and checked.

Acceptance: links and commands resolve, claims match evidence, generated output
is reproducible if changed, and no stale binary is relabelled as a new release.

## VT06 Independent tooling milestone qualification

Depends: VT01â€“VT05. Owner: fresh Sol reviewer, medium; supervisor accepts result.
Write: `docs/plans/vibe-0.03/receipts/VT06.md` and STATE.md only.

Inspect the complete scoped diff and receipts. Reuse valid exact-source evidence;
rerun the focused failed/changed paths, not every historical test. Require one
workspace test run on the final code state; a later prose-only change needs no rebuild.
Check ownership, source pins, CI failure behavior, schema honesty, target claims,
test count/exit status and unrelated-diff preservation. Report open findings with
paths and severity. Mark accepted only after actionable findings are resolved.

## VT07 Canonical contract consumption

Status: local canonical consumption implemented and accepted; immutable source and upstream fixes pending. See STATE.md.
Requires: Qualia-owned contract/schema/version, approved semantics, immutable source
revision and owner-supplied conformance fixtures. Until then, no implementation.

When available, map proposals to the canonical contract and update consumers without
duplicating types or semantics. Domain adapters remain in their owner repositories;
real registration/governance qualification belongs to the Qualia programme. Define
a new bounded file claim and acceptance packet before starting this work.

## Coordinated ecosystem changes

Backward compatibility is not an acceptance gate. Existing behavior is a baseline
for identifying impact. Each justified break records the desired outcome, affected
repositories/files, proposed semantic change, data/artifact handling, replacement
tests and coordinated source revisions. Fix the owning layer rather than preserving
a weak interface through tooling shims. Preserve valid safety/correctness assertions;
replace deliberately superseded test expectations with independently justified ones.
VT07's wait means canonical inputs must exist before consumers execute them. It does
not prohibit preparing or implementing the required owner packet once its file claim
is coordinated with ongoing work. No engine ownership transfer is authorized.
