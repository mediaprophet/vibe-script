//! Workshop-dialect completion and hover from the Vibe catalog.

use serde_json::{json, Value};
use vibe::catalog::{
    canonical_id, describe, families, family_of, methods_for_family, ALL_INVOKE_IDS,
};
use vibe::extension_registry::ExtensionRegistry;
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
    (
        "graph?",
        "Embedded SPARQL ASK (fail-closed without GraphDatabase)",
    ),
    ("law", "law Name when expr => consequence;"),
    ("material", "material Name { ... }"),
    ("field", "field name: Type { ... }"),
    (
        "obligate",
        "Deontic obligation — lowers to DeonticLogic.evaluate when leased",
    ),
    (
        "permit",
        "Deontic permission — lowers to DeonticLogic.evaluate when leased",
    ),
    (
        "forbid",
        "Deontic prohibition — lowers to DeonticLogic.evaluate when leased",
    ),
    (
        "knows",
        "Epistemic knowledge — lowers to EpistemicLogic.evaluate when leased",
    ),
    (
        "believes",
        "Epistemic belief — lowers to EpistemicLogic.evaluate when leased",
    ),
    (
        "always",
        "LTL G(φ) — lowers to TemporalAndDescriptionLogic.ltl.globally when leased",
    ),
    (
        "eventually",
        "LTL F(φ) — lowers to TemporalAndDescriptionLogic.ltl.finally when leased",
    ),
    ("until", "LTL until"),
    (
        "bind",
        "Two-way bind: `bind a <-> b using Clamp[lo, hi] resolve latest;`",
    ),
    ("ease", "Tween easing name after `over duration ease name`"),
    (
        "spring",
        "Tween spring(stiffness, damping) after `over duration`",
    ),
    ("enum", "Algebraic enum declaration"),
    ("match", "Pattern match"),
    ("when", "Cell or law guard"),
];

fn clinic_chunk() -> &'static VocabChunk {
    use std::sync::OnceLock;
    static CHUNK: OnceLock<VocabChunk> = OnceLock::new();
    CHUNK.get_or_init(|| {
        parse_chunk(include_bytes!(
            "../../../../qualiaDB/crates/vibe/fixtures/vocab/clinic.n3"
        ))
        .expect("bundled clinic vocab")
    })
}

const WEB_RUNTIME_AND_CBOR_LD_BINDINGS: &[&str] = &[
    "cbor_ld.compress_iri",
    "cbor_ld.decode",
    "cbor_ld.encode",
    "cbor_ld.expand_term",
    "cbor_ld.ingest_json",
    "cbor_ld.provider_namespace",
    "cbor_ld.verify_deterministic",
    "form.can_redo",
    "form.can_undo",
    "form.checkpoint",
    "form.commit_defaults",
    "form.crdt_merge",
    "form.create",
    "form.diff",
    "form.field_is_dirty",
    "form.filter_disclosure",
    "form.from_cbor_ld",
    "form.from_provider_json",
    "form.get_group_value",
    "form.get_value",
    "form.is_dirty",
    "form.redo",
    "form.request_submit",
    "form.reset",
    "form.rollback_to",
    "form.set_value",
    "form.sign_submission",
    "form.to_cbor_ld",
    "form.to_quins",
    "form.to_rdf_turtle",
    "form.to_record",
    "form.to_url_params",
    "form.undo",
    "form.zk_prove_field",
    "input.apply_mask",
    "input.parse_coordinate",
    "input.parse_currency",
    "input.parse_duration",
    "input.parse_instant",
    "input.parse_quantity",
    "input.parse_uncertainty",
    "input.step",
    "layout.solve_flex",
    "layout.virtual_slice",
    "selection.replace_range",
    "selection.set_range",
    "ui.accordion_toggle",
    "ui.anchor_position",
    "ui.blur",
    "ui.combobox",
    "ui.control",
    "ui.dag_canvas",
    "ui.data_grid",
    "ui.dialog_close",
    "ui.dialog_open",
    "ui.focus",
    "ui.focus_next",
    "ui.hit_test",
    "ui.ink_pad",
    "ui.otp_input",
    "ui.patch",
    "ui.patch_batch",
    "ui.patch_to_cbor_ld",
    "ui.popover_toggle",
    "ui.projectional_editor",
    "ui.range_interval",
    "ui.rating",
    "ui.segmented_control",
    "ui.select_get_selected",
    "ui.select_persona",
    "ui.select_set_options",
    "ui.set_tri_state",
    "ui.show_picker",
    "ui.spectral_picker",
    "ui.splitter",
    "ui.stepper",
    "ui.tag_input",
    "ui.transfer_list",
    "ui.tree_select",
    "validate.check_field",
    "validate.check_form",
    "validate.cross_field",
    "validate.paraconsistent",
    "web.check_permit",
    "web.range_response",
    "web.response",
    "web.route_mail",
];

fn stdlib_v2_all_bindings() -> impl Iterator<Item = &'static str> {
    vibe::stdlib_v2::REFLECT_AND_TYPES_BINDINGS
        .iter()
        .chain(vibe::stdlib_v2::NUM_AND_AUTODIFF_BINDINGS.iter())
        .chain(vibe::stdlib_v2::TEXT_AND_COLLECTIONS_BINDINGS.iter())
        .chain(vibe::stdlib_v2::TOPOLOGY_AND_MANIFOLD_BINDINGS.iter())
        .chain(WEB_RUNTIME_AND_CBOR_LD_BINDINGS.iter())
        .copied()
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
        let ns_prefix = format!("{family}.");
        let stdlib_matches: Vec<Value> = stdlib_v2_all_bindings()
            .filter_map(|id| id.strip_prefix(&ns_prefix).map(|m| (id, m)))
            .map(|(id, method)| {
                json!({
                    "label": method,
                    "kind": 3,
                    "detail": format!("VibeScript v0.02 stdlib `{id}`"),
                    "insertText": method,
                    "documentation": format!("Built-in pure v0.02 stdlib function `{id}(...)`"),
                })
            })
            .collect();
        if !stdlib_matches.is_empty() {
            return stdlib_matches;
        }
        let ext_matches = extension_provider_completions(family);
        if !ext_matches.is_empty() {
            return ext_matches;
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
    let reg = ExtensionRegistry::with_builtin_providers();
    for p in reg.iter_providers() {
        let d = p.descriptor();
        out.push(json!({
            "label": d.short_alias,
            "kind": 9,
            "detail": format!("ABI extension provider `{}` (`{}`)", d.short_alias, d.canonical_iri.iri),
            "insertText": d.short_alias,
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
        return format!(
            "**`{id}`**\n\n{}\n\nType: host invoke (fail-closed).{lease}",
            describe(&token)
        );
    }
    if let Some(hover) = vocab_hover(&token) {
        return hover;
    }
    if stdlib_v2_all_bindings().any(|id| id == token) {
        return format!(
            "**`{token}`**\n\nVibeScript v0.02 built-in standard library / CBOR-LD / web-runtime binding (pure, deterministic, zero-heap hot-path safe)."
        );
    }
    if let Some(ext_hover) = extension_provider_hover(&token) {
        return ext_hover;
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
    let insert_range = if insert.starts_with("using ")
        || insert.starts_with("add requires")
        || insert.starts_with("requires")
    {
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
    families().into_iter().find(|&fam| src.contains(fam))
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
        .rfind(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
        .map(|i| i + 1)
        .unwrap_or(0);
    let fam = &trimmed[start..];
    if fam.is_empty() {
        return None;
    }
    Some(fam)
}

fn extension_provider_completions(prefix_before_dot: &str) -> Vec<Value> {
    let reg = ExtensionRegistry::with_builtin_providers();
    let mut out = Vec::new();
    for p in reg.iter_providers() {
        let d = p.descriptor();
        if d.short_alias == prefix_before_dot {
            for sub in d.sub_namespaces {
                out.push(json!({
                    "label": sub.name,
                    "kind": 9,
                    "detail": format!("ABI sub-namespace `{}.{}` (`{}`)", d.short_alias, sub.name, d.canonical_iri.iri),
                    "insertText": sub.name,
                }));
                for f in sub.functions {
                    let qualified = format!("{}.{}", sub.name, f.name);
                    out.push(json!({
                        "label": qualified,
                        "kind": 3,
                        "detail": format!("ABI function `{}.{}.{}` (`{}`)", d.short_alias, sub.name, f.name, f.canonical_iri.iri),
                        "insertText": format!("{}.{}", sub.name, f.name),
                        "documentation": format!(
                            "Canonical IRI: `{}`\nStable ID: `({}, {})` · Effects: `0x{:x}` · Max ticks: `{}`",
                            f.canonical_iri.iri,
                            f.stable_id.sub_namespace_id,
                            f.stable_id.func_id,
                            f.effects.0,
                            f.envelope.max_ticks
                        ),
                    }));
                }
            }
        } else if let Some((alias, sub_name)) = prefix_before_dot.split_once('.') {
            if d.short_alias == alias {
                for sub in d.sub_namespaces {
                    if sub.name == sub_name {
                        for f in sub.functions {
                            out.push(json!({
                                "label": f.name,
                                "kind": 3,
                                "detail": format!("ABI function `{}.{}.{}` (`{}`)", d.short_alias, sub.name, f.name, f.canonical_iri.iri),
                                "insertText": f.name,
                                "documentation": format!(
                                    "Canonical IRI: `{}`\nStable ID: `({}, {})` · Effects: `0x{:x}` · Max ticks: `{}`",
                                    f.canonical_iri.iri,
                                    f.stable_id.sub_namespace_id,
                                    f.stable_id.func_id,
                                    f.effects.0,
                                    f.envelope.max_ticks
                                ),
                            }));
                        }
                    }
                }
            }
        }
    }
    out
}

fn extension_provider_hover(token: &str) -> Option<String> {
    let reg = ExtensionRegistry::with_builtin_providers();
    for p in reg.iter_providers() {
        let d = p.descriptor();
        if token == d.short_alias || token == d.canonical_iri.iri {
            return Some(format!(
                "**`{}`** (`{}`)\n\nBuilt-in `vibescript-abi` zero-heap extension provider ({} sub-namespace(s), max `{}` ontology `NQuin48` records).",
                d.short_alias,
                d.canonical_iri.iri,
                d.sub_namespaces.len(),
                d.max_ontology_quins
            ));
        }
        for sub in d.sub_namespaces {
            let sub_path = format!("{}.{}", d.short_alias, sub.name);
            if token == sub_path {
                return Some(format!(
                    "**`{sub_path}`**\n\nSub-namespace `{}` (`sub_id = {}`) of ABI provider `{}` (`{}`).",
                    sub.name, sub.sub_id, d.short_alias, d.canonical_iri.iri
                ));
            }
            for f in sub.functions {
                let fn_path = format!("{}.{}.{}", d.short_alias, sub.name, f.name);
                if token == fn_path || token == f.canonical_iri.iri {
                    return Some(format!(
                        "**`{fn_path}`**\n\nCanonical IRI: `{}`\nStable ID: `({}, {})` · Output kind: `{:?}` (`{}` elems)\nEffects: `0x{:x}` · Max ticks: `{}` · Contract SHA-256 prefix: `{:02x}{:02x}{:02x}{:02x}…`",
                        f.canonical_iri.iri,
                        f.stable_id.sub_namespace_id,
                        f.stable_id.func_id,
                        f.output_kind,
                        f.required_output_elems,
                        f.effects.0,
                        f.envelope.max_ticks,
                        f.contract.contract_digest[0],
                        f.contract.contract_digest[1],
                        f.contract.contract_digest[2],
                        f.contract.contract_digest[3],
                    ));
                }
            }
        }
    }
    None
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
            labels
                .iter()
                .any(|l| l.contains("gpu_init") || l == "gpu_init"),
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
        let src =
            "prefix snomed: <http://snomed.info/id/>;\npure fn f() { return snomed:386661006; }\n";
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

    #[test]
    fn stdlib_v2_and_cbor_ld_dot_completes_and_hovers() {
        let dec_items = completions_at("decimal.", 0, 8);
        let dec_labels: Vec<String> = dec_items
            .iter()
            .filter_map(|v| v.get("label").and_then(|l| l.as_str()).map(str::to_string))
            .collect();
        assert!(
            dec_labels.iter().any(|l| l == "from_str") && dec_labels.iter().any(|l| l == "add"),
            "expected decimal.* methods, got {dec_labels:?}"
        );

        let cbor_items = completions_at("cbor_ld.", 0, 8);
        let cbor_labels: Vec<String> = cbor_items
            .iter()
            .filter_map(|v| v.get("label").and_then(|l| l.as_str()).map(str::to_string))
            .collect();
        assert!(
            cbor_labels.iter().any(|l| l == "ingest_json")
                && cbor_labels.iter().any(|l| l == "encode"),
            "expected cbor_ld.* methods, got {cbor_labels:?}"
        );

        let hover = hover_at("cbor_ld.ingest_json(raw, origin)", 0, 10);
        assert!(hover.contains("cbor_ld.ingest_json") && hover.contains("v0.02"), "{hover}");

        let ext_items = completions_at("mech.kinematics.", 0, 16);
        let ext_labels: Vec<String> = ext_items
            .iter()
            .filter_map(|v| v.get("label").and_then(|l| l.as_str()).map(str::to_string))
            .collect();
        assert!(
            ext_labels.iter().any(|l| l == "verlet_step")
                && ext_labels.iter().any(|l| l == "kinetic_energy"),
            "expected mech.kinematics.* ABI functions, got {ext_labels:?}"
        );

        let ext_hover = hover_at("mech.kinematics.verlet_step(pos, vel, acc, dt)", 0, 20);
        assert!(
            ext_hover.contains("https://qualiadb.org/ext/etalon/mechanics")
                && ext_hover.contains("Contract SHA-256"),
            "{ext_hover}"
        );
    }
}
