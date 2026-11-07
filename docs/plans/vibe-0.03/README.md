# VibeScript tooling and extension preparation

Date: 2026-11-06. Status: implementation plan ready; implementation not started.

QualiaDB owns the engine, including VibeScript, canonical capability contracts,
execution semantics and human-centric outcomes. This repository owns tooling.
This programme prepares tooling and extension descriptions without moving the
engine, implementing a competing ABI or modifying concurrent Qualia/Etalon work.

## Start or resume

Read [STATE.md](STATE.md), then the selected packet in [WORK-PACKETS.md](WORK-PACKETS.md)
and [AGENT-RUNBOOK.md](AGENT-RUNBOOK.md). The next packet is VT01. Start only one
implementation packet at a time. The setup request authorizes this plan; the next
execution session must have an implementation instruction before changing code.

The canonical architecture programme is in QualiaDB at
`docs/plans/vibe-0.03/implementation-plan.md`. Its ownership clarification controls:
the lightweight contract is owned in QualiaDB, tooling consumes it, and domain
libraries own their algorithms/adapters. That plan directory is currently ignored
by Qualia's Git rules; this companion plan states the necessary constraints itself
so resumption does not depend on that local document being published.

## Prerelease design priority

User clarification, 2026-11-06: the ecosystem may change as needed for the best
architecture and intended outcomes. Backward compatibility is not the focus.
Breaking API, descriptor, syntax and package changes are permitted when justified.
Do not add legacy shims or retain aliases solely to preserve unreleased behavior.
Record the improvement and affected consumers; update those consumers and tests
coherently. QualiaDB engine ownership, human-centric requirements, user data and
unrelated active work remain protected. Cross-repository changes proceed through
coordinated owner packets; this tooling plan is not a ceiling on the target design.
## Work boundaries

| Ready now | Wait for owner input |
|---|---|
| Inventory existing engine interfaces and source paths | Canonical extension ABI/schema approval or implementation |
| CI with immutable engine source and no silent skips | New engine registry, dispatch, effects or grammar |
| Proposed descriptor examples and independent expected results | Runtime registration and provider invocation |
| Completion/hover tests using existing engine catalog APIs | Advertising extension calls absent from the pinned engine |
| Source/version provenance and accurate documentation | Real Sentinel integration, ontology mounting or receipts |

Consent where required, authority/delegation, context, disclosure and provenance
requirements remain Qualia-owned. Tooling must distinguish declarations, target
support, runtime authorization and qualified execution. A descriptor, editor
completion or simulated fixture grants no authority. Preserve existing isolated
playground/native-bridge profiles; do not add a developer fallback to governed use.

## Observed baseline

| Item | Observed value on 2026-11-06 |
|---|---|
| Tooling checkout | `C:/Projects/vibe-script`, branch `0.0.35` |
| Tooling HEAD | `cae21724cfb6b3a1032c8ff1f0b819a656be1c33` |
| Tooling source status before plan | Clean |
| Qualia HEAD at inspection | `3910f89697fbcf1d829a073644af9ec6ea9ab1c9` |
| Qualia status | Active uncommitted work; never use its live working tree as reproducible qualification evidence |
| Versions | Language `vibe-0.1`; host `vibe-host-0.1`; Qualia `vibe` package `0.0.40`; tooling package `0.0.1` |
| CI issue | Missing sibling engine prints a message and succeeds without running workspace tests |
| Path coupling | LSP/playground manifests and docs generator expect sibling `qualia-27062026/crates/vibe` |

The Qualia commit above is an observed candidate, not a qualified integration pin.
VT01/VT02 must verify retrieval and coherent execution before selecting it for CI.
`vibe-0.03` is a programme label, not a released dialect or package version.

## Completion

Complete VT01–VT06 with packet-specific evidence and independent supervisor review.
Keep VT07 waiting until its Qualia-owned inputs exist. Finishing tooling preparation
does not mean the wider engine upgrade or Sentinel integration is complete.
See [REVIEW.md](REVIEW.md) for the planning review and its limits.
