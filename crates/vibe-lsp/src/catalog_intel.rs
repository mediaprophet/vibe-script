//! Workshop-dialect completion and hover from the Vibe catalog.

use serde_json::{json, Value};
use vibe::catalog::{
    canonical_id, describe, families, family_of, methods_for_family, ALL_INVOKE_IDS,
};
use vibe::vocab::{parse_chunk, VocabChunk};

const KEYWORDS: &[(&str, &str)] = &[
    ("cell", "Reactive cell declaration (`cell name := expr;`)"),
    ("fn", "Function declaration"),
    ("pure", "Pure effect modifier"),
    ("effect", "External effect modifier"),
    ("hot", "Hot zero-heap performance modifier"),
    ("cold", "Cold construction modifier"),
    ("using", "Lease a catalog family (`using Animation;`)"),
    ("requires", "Capability requirements clause"),
    ("import", "import \"vibe:0.1/math\" as math;"),
    ("prefix", "prefix p: <iri>;"),
    ("locale", "Opt-in keyword locale (`locale zh;`)"),
    ("present", "Presentation sheaf surface"),
    ("graph", "Embedded graph pattern"),
    ("graph?", "Embedded SPARQL ASK (fail-closed without GraphDatabase)"),
    ("law", "law Name when expr => consequence;"),
    ("material", "material Name { ... }"),
    ("field", "field name: Type { ... }"),
    ("obligate", "Deontic obligation — lowers to DeonticLogic.evaluate when leased"),
    ("permit", "Deontic permission — lowers to DeonticLogic.evaluate when leased"),
    ("forbid", "Deontic prohibition — lowers to DeonticLogic.evaluate when leased"),
    ("knows", "Epistemic knowledge — lowers to EpistemicLogic.evaluate when leased"),
    ("believes", "Epistemic belief — lowers to EpistemicLogic.evaluate when leased"),
    ("always", "LTL G(φ) — lowers to TemporalAndDescriptionLogic.ltl.globally when leased"),
    ("eventually", "LTL F(φ) — lowers to TemporalAndDescriptionLogic.ltl.finally when leased"),
    ("until", "LTL until"),
    ("bind", "Two-way bind: `bind a <-> b using Clamp[lo, hi] resolve latest;`"),
    ("ease", "Tween easing name after `over duration ease name`"),
    ("spring", "Tween spring(stiffness, damping) after `over duration`"),
    ("enum", "Algebraic enum declaration"),
    ("match", "Pattern match"),
    ("when", "Cell or law guard"),
];

fn clinic_chunk() -> &'static VocabChunk {
    use std::sync::OnceLock;
    static CHUNK: OnceLock<VocabChunk> = OnceLock::new();
    CHUNK.get_or_init(|| {
        parse_chunk(include_bytes!(
            "../../../../qualia-27062026/crates/vibe/fixtures/vocab/clinic.n3"
        ))
        .expect("bundled clinic vocab")
    })
}

pub fn completions_at(src: &str, line: usize, character: usize) -> Vec<Value> {
    let offset = position_to_offset(src, line, character);
    let prefix = &src[..offset.min(src.len())];
    if let Some(pfx) = prefix_before_colon(prefix) {
        let chunk = clinic_chunk();
        if chunk.prefixes.contains_key(pfx) {
            let base = chunk.prefix_iri(pfx).unwrap_or("");
            return chunk
                .terms
                .iter()
                .filter_map(|t| t.iri.strip_prefix(base).map(|local| (t, local)))
                .map(|(t, local)| {
                    json!({
                        "label": local,
                        "kind": 12,
                        "detail": t.label.clone().unwrap_or_else(|| t.iri.clone()),
                        "documentation": format!("`{}:{}`\n{}", pfx, local, t.iri),
                        "insertText": local,
                    })
                })
                .collect();
        }
    }
    if let Some(family) = family_before_dot(prefix) {
        let methods = methods_for_family(family);
        if !methods.is_empty() {
            return methods
                .into_iter()
                .map(|id| {
                    let method = id.rsplit('.').next().unwrap_or(id);
                    json!({
                        "label": method,
                        "kind": 3,
                        "detail": describe(id),
                        "insertText": method,
                        "documentation": format!("`using {family};` leases this invoke"),
                    })
                })
                .collect();
        }
    }
    let mut out = Vec::new();
    for (label, detail) in KEYWORDS {
        out.push(json!({
            "label": label,
            "kind": 14,
            "detail": detail,
        }));
    }
    for fam in families() {
        out.push(json!({
            "label": fam,
            "kind": 9,
            "detail": format!("Catalog family — `using {fam};` then `{fam}.method(...)`"),
            "insertText": fam,
        }));
    }
    for id in ALL_INVOKE_IDS.iter().copied().take(32) {
        out.push(json!({
            "label": id,
            "kind": 3,
            "detail": describe(id),
        }));
    }
    out
}

pub fn hover_at(src: &str, line: usize, character: usize) -> String {
    let offset = position_to_offset(src, line, character);
    let token = ident_path_at(src, offset);
    if token.is_empty() {
        return "**VibeScript** (`vibe-0.1`)\n\nWorkshop dialect: `using Family;` then `Family.method(...)`. Modal verbs stay terms unless the engine family is leased.".into();
    }
    if let Some(id) = canonical_id(&token) {
        let lease = family_of(id)
            .map(|f| format!("\n\nLease: `using {f};`"))
            .unwrap_or_default();
        return format!("**`{id}`**\n\n{}\n\nType: host invoke (fail-closed).{lease}", describe(&token));
    }
    if let Some(hover) = vocab_hover(&token) {
        return hover;
    }
    for (kw, detail) in KEYWORDS {
        if *kw == token {
            return format!("**`{kw}`**\n\n{detail}");
        }
    }
    format!("**`{token}`**\n\n{}", describe(&token))
}

pub fn workspace_edit_for_fix(uri: &str, diagnostic: &Value, src: &str) -> Option<Value> {
    let fix = diagnostic
        .pointer("/data/suggested_fix")
        .and_then(|v| v.as_str())?;
    let range = diagnostic.get("range").cloned().unwrap_or_else(|| {
        json!({
            "start": { "line": 0, "character": 0 },
            "end": { "line": 0, "character": 0 }
        })
    });
    let insert = if fix.starts_with("add `using") || fix.contains("using Family") {
        let fam = family_from_source_diagnostic(src).unwrap_or("Animation");
        format!("using {fam};\n")
    } else if fix.starts_with("add requires") {
        format!("{fix}\n")
    } else {
        format!("/* {fix} */\n")
    };
    let insert_range = if insert.starts_with("using ") || insert.starts_with("add requires") || insert.starts_with("requires") {
        json!({ "start": { "line": 0, "character": 0 }, "end": { "line": 0, "character": 0 } })
    } else {
        range
    };
    Some(json!({
        "changes": {
            uri: [{
                "range": insert_range,
                "newText": insert,
            }]
        }
    }))
}

fn family_from_source_diagnostic(src: &str) -> Option<&'static str> {
    for fam in families() {
        if src.contains(fam) {
            return Some(fam);
        }
    }
    None
}

fn vocab_hover(token: &str) -> Option<String> {
    let (pfx, local) = token.split_once(':')?;
    if local.is_empty() {
        return None;
    }
    let chunk = clinic_chunk();
    if !chunk.has_local(pfx, local) {
        return None;
    }
    let iri = chunk.expand(token)?;
    let term = chunk.terms.iter().find(|t| t.iri == iri)?;
    let label = term.label.as_deref().unwrap_or(local);
    let parents = if term.parents.is_empty() {
        String::new()
    } else {
        format!("\n\nParents: {}", term.parents.join(", "))
    };
    Some(format!(
        "**{label}**\n\n`{token}`\n\nIRI: `{iri}`{parents}\n\nVocab chunk (tree-shaken terms only; not the whole ontology)."
    ))
}

fn prefix_before_colon(prefix: &str) -> Option<&str> {
    let trimmed = prefix.trim_end();
    let trimmed = trimmed.strip_suffix(':')?;
    let start = trimmed
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .map(|i| i + 1)
        .unwrap_or(0);
    let pfx = &trimmed[start..];
    if pfx.is_empty() {
        return None;
    }
    Some(pfx)
}

fn family_before_dot(prefix: &str) -> Option<&str> {
    let trimmed = prefix.trim_end();
    let trimmed = trimmed.strip_suffix('.')?;
    let start = trimmed
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .map(|i| i + 1)
        .unwrap_or(0);
    let fam = &trimmed[start..];
    if fam.is_empty() {
        return None;
    }
    Some(fam)
}

fn ident_path_at(src: &str, offset: usize) -> String {
    let bytes = src.as_bytes();
    if bytes.is_empty() {
        return String::new();
    }
    let mut i = offset.min(bytes.len().saturating_sub(1));
    while i > 0 {
        let c = bytes[i] as char;
        if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == ':' {
            break;
        }
        i = i.saturating_sub(1);
    }
    let mut start = i;
    while start > 0 {
        let c = bytes[start - 1] as char;
        if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == ':' {
            start -= 1;
        } else {
            break;
        }
    }
    let mut end = i;
    while end < bytes.len() {
        let c = bytes[end] as char;
        if c.is_ascii_alphanumeric() || c == '_' || c == '.' || c == ':' {
            end += 1;
        } else {
            break;
        }
    }
    src[start..end].to_string()
}

pub fn position_to_offset(src: &str, line: usize, character: usize) -> usize {
    let mut cur_line = 0;
    let mut cur_char = 0;
    for (i, ch) in src.char_indices() {
        if cur_line == line && cur_char >= character {
            return i;
        }
        if ch == '\n' {
            cur_line += 1;
            cur_char = 0;
            if cur_line > line {
                return i;
            }
        } else {
            cur_char += ch.len_utf16();
        }
    }
    src.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn family_dot_completes_methods() {
        let items = completions_at("using Render;\nRender.", 1, 7);
        let labels: Vec<String> = items
            .iter()
            .filter_map(|v| v.get("label").and_then(|l| l.as_str()).map(str::to_string))
            .collect();
        assert!(
            labels.iter().any(|l| l.contains("gpu_init") || l == "gpu_init"),
            "expected Render.gpu_* methods, got {labels:?}"
        );
        assert!(!labels.iter().any(|l| l == "cell"));
    }

    #[test]
    fn hover_catalog_path() {
        let src = "using Animation;\nAnimation.orbit_spin(t)";
        let text = hover_at(src, 1, 12);
        assert!(text.contains("orbit_spin") || text.contains("evaluate_preset"));
        assert!(text.contains("using"));
    }

    #[test]
    fn hover_snomed_term_shows_iri_and_label() {
        let src = "prefix snomed: <http://snomed.info/id/>;\npure fn f() { return snomed:386661006; }\n";
        let text = hover_at(src, 1, 32);
        assert!(text.contains("386661006"), "{text}");
        assert!(text.contains("http://snomed.info/id/"), "{text}");
        assert!(text.contains("Fever") || text.contains("IRI"), "{text}");
    }

    #[test]
    fn snomed_colon_completes_locals() {
        let items = completions_at("prefix snomed: <http://snomed.info/id/>;\nsnomed:", 1, 7);
        let labels: Vec<String> = items
            .iter()
            .filter_map(|v| v.get("label").and_then(|l| l.as_str()).map(str::to_string))
            .collect();
        assert!(
            labels.iter().any(|l| l == "386661006"),
            "expected SNOMED locals, got {labels:?}"
        );
    }
}
