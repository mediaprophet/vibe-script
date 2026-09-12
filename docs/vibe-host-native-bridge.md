# Vibe-host execution targets

VibeScript remains one language. Its host determines where a capability runs.
Poet is a host (UI / CLI / habitat), not the language. A chatbot is a tool.
`vibe-host` is the reusable execution boundary. Engine truth is QualiaDB `0.0.38`
`crates/vibe` — this file is tooling metadata, not a Host invent.

Missing daemon or WASM-only snapshot: **planned**, never “unavailable”
or “broken”. No invented public-relay path.

The LSP accepts either of these initialization settings:

```json
{
  "initializationOptions": {
    "vibeTarget": "wasm-standalone"
  }
}
```

or:

```json
{
  "initializationOptions": {
    "vibe": { "target": "wasm-standalone" }
  }
}
```

`native-hybrid` is the default and preserves existing editor behaviour.
For `wasm-standalone`, the LSP emits `QDB0402` metadata alongside Vibe's normal
diagnostics:

- `native-bridge`: a paired local QualiaDB daemon is required.
- `standalone-snapshot`: the call can run in WASM, but only against its
  isolated in-memory graph. It is not a persistent native transaction.

Clients can probe a paired daemon at `GET /vibe/capabilities`, which reports
the `qualia-vibe-bridge/1` protocol. Browser integrations must make that probe
after a user gesture and include the pairing token for production daemons.
