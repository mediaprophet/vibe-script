# Agent execution and supervision

## Budget policy

The user reported 11% weekly allowance remaining on 2026-11-06. This is a user
estimate, not a measured token budget. No process here can guarantee continuation
after the account's limit or automatically wake when its allowance resets.
Durable checkpoints allow another authorized agent to resume when available.

Use `gpt-6.1-sol` with medium reasoning for scoped implementation. Use a fresh
`gpt-6.1-sol` reviewer with medium reasoning and read-only scope. Raise only a
specific unresolved contract/authority question to high reasoning; do not escalate
the entire programme. Avoid Astra/max-effort agents by default. A Luna low-effort
worker is optional for mechanical inventories/link checks if actually useful;
running the corresponding script directly is usually cheaper than another agent.

Sol is selected as a balance of coding capability and cost, consistent with the
[official model guidance](https://developers.openai.com/api/docs/models/gpt-6.1-sol).
API prices do not establish Codex weekly usage or remaining account allowance.
These model choices are recommendations, not claims of measured task-level savings.

## Roles and concurrency

| Role | Assignment | Authority |
|---|---|---|
| Supervisor | Current orchestrator; a resumed Sol agent can take this role | Select packet, claim files, check evidence, dispatch independent review, accept or reopen |
| Implementation worker | Sol medium, one packet | Edit only claimed tooling paths and produce receipt; cannot self-accept |
| Independent reviewer | Fresh Sol medium context | Read scoped diff and evidence; run only necessary bounded checks; report findings; no implementation edits |

At most one writer is active. Review starts after the candidate is checkpointed,
so a reviewer sees stable files. Use short explicit briefs and `fork_turns=none`
when available. Do not inherit the full conversation. Do not recursively delegate.
Agent IDs are temporary; durable packet IDs and file state are the source of truth.

Budget heuristics, not enforced tool limits: aim for one 20–40 minute worker pass
and one 10–15 minute review per packet, no more than two substantive repair attempts
before recording the blocker. Use the lowest effort that handles the actual packet.
Read only named files and relevant excerpts; cache command results in the receipt.
Do not run broad repository audits or repeat tests on unchanged source. Do not start
a new packet when insufficient budget remains to checkpoint and review it.

## File claims and conflicts

Before work, record HEAD, relevant dirty paths and hashes in STATE.md. Never stash,
reset, rebase, switch the shared branch, delete artifacts or overwrite another
agent's changes. A dirty overlap requires coordination; non-overlapping work can
continue. Use a dedicated worktree/checkout only when setup is permitted, or a
separate immutable source snapshot for validation. Qualia's active checkout is read-only.
Creating a build snapshot does not transfer engine ownership.

Record claim owner, packet, allowed paths, start time and status in STATE.md before
editing. If another process changes a claimed file, stop edits to that file and
reconcile from the observed contents. Do not claim automatic conflict protection:
this is cooperative coordination, not an OS lock. An interrupted claim is not an
automatic lease expiry; verify the worker is stopped before taking it over.

## Prerelease change decisions

Favor the best design and human-centric outcomes over backward compatibility.
Review proposed breaks for justified benefit, complete consumer updates and new
acceptance evidence. Preserve semantic obligations, data and unrelated work.
Coordinate cross-repository owner packets; avoid both arbitrary scope growth and
compatibility shims that hide a necessary engine correction.

## Review and completion gate

Worker states: READY -> IN_PROGRESS -> AWAITING_REVIEW -> ACCEPTED or NEEDS_FIX.
WAITING_EXTERNAL and BLOCKED preserve their reason and independent work remaining.
Only the supervisor records ACCEPTED after an independent review with no unresolved
actionable findings. The supervisor checks the actual diff/evidence, not merely a
worker's completion message. Same-model review is a second pass, not proof by itself.

Reviewer brief: read the packet, scope claim, candidate diff and receipt; verify
ownership boundaries, negative cases, test/source identity and honest claims.
Report file/line findings, missing evidence and acceptance recommendation in <=500
words. Request a reproduction only when it resolves an uncertainty. Do not rewrite
the implementation or expand scope.

## Durable handoff

After every packet and before interruption/compaction/limit:

1. Save changed files; record exact paths and candidate hashes or commit identity.
2. Write `receipts/VTxx.md`: objective, baseline, changes, commands, exits/counts,
   failure history, unexecuted checks, reviewer findings and next concrete command.
3. Update STATE.md with status, remaining work, live process IDs/target directories,
   claim ownership and blockers. Never mark interrupted work accepted.
4. Stop owned expendable processes when appropriate; preserve any active build's
   artifact ownership and instructions. Do not leave an agent silently writing.
5. A replacement agent reads STATE first, checks Git/diffs/process state, verifies
   claims and source identity, then resumes the single next action. Prior successful
   evidence may be reused only if relevant sources/configuration are unchanged.

Do not use sleep loops to wait for a weekly reset. No unattended scheduler or new
chat is created by this plan. Stop with the handoff saved when work cannot continue.

## Copyable resume instruction

> Work in C:/Projects/vibe-script. Read docs/plans/vibe-0.03/STATE.md,
> docs/plans/vibe-0.03/README.md, docs/plans/vibe-0.03/AGENT-RUNBOOK.md and the
> selected docs/plans/vibe-0.03/WORK-PACKETS.md entry. QualiaDB owns the engine
> and canonical semantics. Implement only the next READY tooling packet, preserving
> unrelated work and respecting its file claim. Use one GPT-6.1 Sol medium worker
> and a separate bounded Sol reviewer; no recursive agents. Save a receipt and
> STATE checkpoint before stopping. Do not implement waiting engine/ABI work, push,
> deploy or claim unexecuted checks. Acknowledge any source changes since baseline.
