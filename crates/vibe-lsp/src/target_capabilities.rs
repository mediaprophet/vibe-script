//! Target-aware diagnostics for the QualiaDB Vibe host boundary.
//!
//! This is tooling metadata, not Vibe grammar. It reports a bridge
//! requirement before execution instead of changing a program or pretending
//! that a persistent native operation has a WASM equivalent.

use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetProfile {
    WasmStandalone,
    #[default]
    NativeHybrid,
}

impl TargetProfile {
    pub fn parse(value: Option<&str>) -> Self {
        match value {
            Some("wasm") | Some("wasm-standalone") | Some("wasm_standalone") => {
                Self::WasmStandalone
            }
            _ => Self::NativeHybrid,
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WasmStandalone => "wasm-standalone",
            Self::NativeHybrid => "native-hybrid",
        }
    }
}

const NATIVE_BRIDGE_IDS: &[&str] = &[
    "GraphDatabase.sparql",
    "Inference.load_model",
    "Inference.run_transformer",
];

const SNAPSHOT_IDS: &[&str] = &[
    "graph.query",
    "graph.snapshot",
    "graph.stage",
    "graph.commit",
    "aura.validate",
    "pulse.publish",
];

/// `QDB0402` is a tooling diagnostic. The Vibe engine keeps its established
/// `E702` code for execution-time capability misses (voice: held / not yet).
pub fn diagnostics(src: &str, target: TargetProfile) -> Vec<Value> {
    if target != TargetProfile::WasmStandalone {
        return Vec::new();
    }

    let mut out = Vec::new();
    for (start, end, path) in invocation_paths(src) {
        if NATIVE_BRIDGE_IDS.contains(&path.as_str()) {
            out.push(diagnostic(
                src,
                start,
                end,
                path,
                "native-bridge",
                2,
                "requires a paired local QualiaDB daemon; held / not yet on a standalone WASM target",
            ));
        } else if SNAPSHOT_IDS.contains(&path.as_str()) {
            out.push(diagnostic(
                src,
                start,
                end,
                path,
                "standalone-snapshot",
                3,
                "runs against an isolated in-memory snapshot in standalone WASM; pair the native daemon for persistent graph semantics",
            ));
        }
    }
    out
}

fn diagnostic(
    src: &str,
    start: usize,
    end: usize,
    capability: String,
    mode: &'static str,
    severity: u8,
    detail: &'static str,
) -> Value {
    let (start_line, start_character) = offset_to_position(src, start);
    let (end_line, end_character) = offset_to_position(src, end);
    json!({
        "range": {
            "start": { "line": start_line, "character": start_character },
            "end": { "line": end_line, "character": end_character },
        },
        "severity": severity,
        "code": "QDB0402",
        "source": "vibe-target",
        "message": format!("{capability}: {detail}"),
        "data": {
            "capability": capability,
            "target": TargetProfile::WasmStandalone.as_str(),
            "mode": mode,
            "bridge_protocol": "qualia-vibe-bridge/1",
        },
    })
}

/// Extract dotted call paths while skipping quoted strings and line comments.
fn invocation_paths(src: &str) -> Vec<(usize, usize, String)> {
    let bytes = src.as_bytes();
    let mut out = Vec::new();
    let mut index = 0;
    let mut in_string = false;
    while index < bytes.len() {
        if in_string {
            if bytes[index] == b'\\' {
                index = index.saturating_add(2);
                continue;
            }
            if bytes[index] == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        if bytes[index] == b'"' {
            in_string = true;
            index += 1;
            continue;
        }
        if bytes[index] == b'/' && bytes.get(index + 1) == Some(&b'/') {
            index += 2;
            while index < bytes.len() && bytes[index] != b'\n' {
                index += 1;
            }
            continue;
        }
        if !is_path_byte(bytes[index]) {
            index += 1;
            continue;
        }
        let start = index;
        while index < bytes.len() && is_path_byte(bytes[index]) {
            index += 1;
        }
        let end = index;
        let path = &src[start..end];
        let mut after = index;
        while after < bytes.len() && bytes[after].is_ascii_whitespace() {
            after += 1;
        }
        if path.contains('.') && bytes.get(after) == Some(&b'(') {
            out.push((start, end, path.to_string()));
        }
    }
    out
}

const fn is_path_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'.'
}

fn offset_to_position(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut character = 0;
    for (index, ch) in src.char_indices() {
        if index >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            character = 0;
        } else {
            character += ch.len_utf16();
        }
    }
    (line, character)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_bridge_call_is_flagged_for_wasm() {
        let diags = diagnostics(
            "using GraphDatabase;\nGraphDatabase.sparql(query);",
            TargetProfile::WasmStandalone,
        );
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0]["code"], "QDB0402");
        assert_eq!(diags[0]["data"]["mode"], "native-bridge");
        assert!(
            diags[0]["message"]
                .as_str()
                .unwrap_or("")
                .contains("held / not yet"),
            "diagnose voice must be held / not yet"
        );
    }

    #[test]
    fn standalone_graph_call_explains_snapshot_semantics() {
        let diags = diagnostics("graph.commit();", TargetProfile::WasmStandalone);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0]["severity"], 3);
        assert_eq!(diags[0]["data"]["mode"], "standalone-snapshot");
    }

    #[test]
    fn strings_and_comments_do_not_create_diagnostics() {
        let src = "// GraphDatabase.sparql(query)\nlet note = \"Inference.load_model(x)\";";
        assert!(diagnostics(src, TargetProfile::WasmStandalone).is_empty());
    }
}
