# Descriptor export schema v1

Owner: VibeScript tooling. Canonical ABI, language semantics and runtime governance
remain owned by QualiaDB. `vibe-tooling` imports the actual `vibescript-abi` types;
it defines no alternate evaluator, provider algorithm or authority engine.

`cargo run -p vibe-tooling --locked` emits a JSON envelope containing `manifest`
and `tooling_manifest_sha256`. Consumers can also request LSP
`vibe/extensionManifest` (advertised in `capabilities.experimental`) or HTTP GET
`/api/extensions`. The playground links a separate descriptor preview. Existing
executable catalog completions and validation continue to use the engine catalog.

The manifest schema identifier is `vibescript-tooling-descriptor/1`. This is an
export schema, not a runtime ABI wire format. Unknown schema versions must not be
interpreted as version 1; callers should reject or display them as opaque metadata.
There is currently no descriptor-import or registration endpoint.

## Exact serialization and digest

SHA-256 covers only `manifest`, excluding the enclosing digest envelope and CLI
trailing newline. Bytes are compact UTF-8 JSON without BOM, whitespace or trailing
newline. Object keys are sorted recursively by UTF-8 byte order. Arrays retain
order, except descriptor sub-namespaces sort ascending by sub-ID and functions
ascending by packed stable ID. Parameters and ontology records retain declared
order, because parameter position and ontology spans depend on it. Equal sort keys
retain declared order. String escaping and decimal integer rendering follow
serde_json 1.x; the manifest has no floating-point numbers. All 64-bit quantities
(hashes, version words, ticks and ontology words) are fixed-width 16-digit lowercase
hex strings. The exported ABI enum fields are their numeric discriminants; effect
and target masks retain all bits. Full 32-byte digests are 64 lowercase hex digits.

All descriptor fields are exported: identity, alias, five independent versions,
targets, ontology capacity, namespace and function stable IDs, ordered parameters,
output kind/count, effects, determinism, reversibility, all resource-envelope fields,
provider-declared contract digest, schema hash, pre/post/invariant spans and actual
six-word ontology records (including parity). Tooling metadata explicitly states
`discovery_only: true`, `authorization: not_assessed`, and language dispatch is not
exposed by this tool.

`declared_digest` is copied verbatim with
`digest_verification: unverified_provider_declaration`. The current Qualia reference
provider's repeated 0x11/0x22/0x33 bytes are placeholders. The tooling manifest SHA-256
is an export identity, and makes no claim of equivalence with the engine's canonical
function contract digest. It does not prove authorization, provider trust, provenance,
ontology truth or actual runtime target availability.

## Resource limits and error behavior

Exports are cold tooling work, outside the engine hot ABI and Sentinel arena. Before
constructing descriptor JSON, tooling admits at most 256 combined namespace,
function, parameter and ontology records; each text field is at most 4096 UTF-8
bytes and combined declared text is at most 131072 bytes. Serialization admits at
most 16 nesting levels, 256 entries per container and 1048576 emitted bytes. API
functions return `ExportError`; partial byte buffers are discarded. CLI fails with
nonzero exit, HTTP returns 500, and LSP returns an explicit error object in its
custom discovery result. No partial manifest or digest is published.

## Qualification gaps

The independent duplicate-stable-ID probe reproduces an upstream registration
failure: distinct names/IRIs with the same stable ID are accepted. That test is
explicitly ignored in normal tooling checks and must be run separately; the receipt
records its failure. Qualia must repair it. No full ABI conformance claim is made.
The registry also has no tooling-enforced authority here. Source publication and
canonical digest serialization remain Qualia owner actions.
