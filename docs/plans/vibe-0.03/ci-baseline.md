# CI baseline and source publication blocker

CI checks out tooling at its event revision under vibe-script and Qualia at the full
commit in tools/ci/engine-source.json under qualia-27062026. Preflight requires that
HEAD matches, required paths exist and are tracked, dependency sources are clean,
and the lock is explicitly qualified. Missing source never yields success or skip.
Then CI runs `cargo test --workspace --locked` with one job and an isolated target.

The current source lock is deliberately `qualified: false`. Its observed HEAD lacks
tracked canonical ABI/registry sources used by this candidate. The same helper
rejects missing directories, wrong revisions, and current uncommitted required files.
No hosted CI run is claimed. After Qualia publishes a commit containing the ABI and
registry, update the full revision, verify a clean sibling checkout, run the locked
workspace test, and only then set qualified true with review evidence.

Cargo.lock changed only to resolve the actual vibe version (0.0.35 to 0.0.40), the new
canonical vibescript-abi dependency, the tooling crate and SHA-256 dependencies.
No broad dependency upgrade command was run. sha2 0.10.9 was already cached; local
checks ran offline. Parent final qualification uses a captured source snapshot,
which proves local candidate behavior but cannot replace a retrievable source commit.
