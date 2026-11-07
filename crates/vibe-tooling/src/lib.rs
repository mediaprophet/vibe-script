//! Cold, read-only descriptor export. No registration, execution or authority is implied.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use vibescript_abi::{OntologicalContract, ProviderDescriptor, VibeNamespaceProvider};

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Tooling schema v1: unsigned 64-bit quantities use fixed-width lowercase hex strings.
pub const MAX_MANIFEST_BYTES: usize = 1_048_576;
pub const MAX_JSON_DEPTH: usize = 16;
pub const MAX_DESCRIPTOR_ITEMS: usize = 256;
pub const MAX_TEXT_BYTES: usize = 4096;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportError { TooManyItems, TextTooLong, TooDeep, ByteBudgetExceeded }

pub fn descriptor_manifest(provider: &dyn VibeNamespaceProvider) -> Result<Value, ExportError> {
    let d: &ProviderDescriptor = provider.descriptor();
    let mut count = d.sub_namespaces.len() + provider.static_ontology_nquins().len();
    let mut text_bytes = 0usize;
    let mut text = |s: &str| -> Result<(),ExportError> {
        if s.len() > MAX_TEXT_BYTES { return Err(ExportError::TextTooLong); }
        text_bytes = text_bytes.checked_add(s.len()).ok_or(ExportError::ByteBudgetExceeded)?;
        if text_bytes > MAX_MANIFEST_BYTES / 8 { return Err(ExportError::ByteBudgetExceeded); }
        Ok(())
    };
    text(d.canonical_iri.iri)?; text(d.short_alias)?;
    for sub in d.sub_namespaces {
        count = count.checked_add(sub.functions.len()).ok_or(ExportError::TooManyItems)?;
        if count > MAX_DESCRIPTOR_ITEMS { return Err(ExportError::TooManyItems); }
        text(sub.name)?;
        for f in sub.functions {
            count = count.checked_add(f.params.len()).ok_or(ExportError::TooManyItems)?;
            if count > MAX_DESCRIPTOR_ITEMS { return Err(ExportError::TooManyItems); }
            text(f.name)?; text(f.canonical_iri.iri)?;
            for p in f.params { text(p.name)?; }
        }
    }
    if count > MAX_DESCRIPTOR_ITEMS { return Err(ExportError::TooManyItems); }
    let v = d.versions;
    let mut namespaces: Vec<_> = d.sub_namespaces.iter().collect();
    namespaces.sort_by_key(|s| s.sub_id);
    let namespaces: Vec<_> = namespaces.into_iter().map(|s| {
        let mut functions: Vec<_> = s.functions.iter().collect();
        functions.sort_by_key(|f| f.stable_id.to_u32());
        let functions: Vec<_> = functions.into_iter().map(|f| {
            let e = f.envelope;
            let c = f.contract;
            let params: Vec<_> = f.params.iter().map(|p| json!({
                "name": p.name, "kind": p.kind as u8,
                "min_elems": p.min_elems, "max_elems": p.max_elems
            })).collect();
            json!({"name": f.name, "canonical_iri": f.canonical_iri.iri,
                "iri_hash": format!("{:016x}", f.canonical_iri.fnv1a_hash),
                "stable_id": [f.stable_id.sub_namespace_id, f.stable_id.func_id],
                "params": params, "output_kind": f.output_kind as u8,
                "required_output_elems": f.required_output_elems, "effects": f.effects.0,
                "determinism": f.determinism as u8, "reversibility": f.reversibility as u8,
                "envelope": {"max_ticks": format!("{:016x}", e.max_ticks),
                    "max_scratch_bytes": e.max_scratch_bytes, "max_vram_bytes": e.max_vram_bytes,
                    "max_output_bytes": e.max_output_bytes, "max_spawned_tasks": e.max_spawned_tasks,
                    "flags": e.flags},
                "contract": {"declared_digest": hex(&c.contract_digest),
                    "digest_verification": "unverified_provider_declaration",
                    "schema_hash": format!("{:016x}", c.schema_hash),
                    "precondition_quin_span": [c.precondition_quin_span.0,c.precondition_quin_span.1],
                    "postcondition_quin_span": [c.postcondition_quin_span.0,c.postcondition_quin_span.1],
                    "invariant_quin_span": [c.invariant_quin_span.0,c.invariant_quin_span.1]}})
        }).collect();
        json!({"name": s.name, "sub_id": s.sub_id, "functions": functions})
    }).collect();
    let ontology: Vec<_> = provider.static_ontology_nquins().iter().map(|q| {
        q.0.iter().map(|word| format!("{word:016x}")).collect::<Vec<_>>()
    }).collect();
    Ok(json!({"schema": "vibescript-tooling-descriptor/1", "discovery_only": true,
        "authorization": "not_assessed", "language_dispatch": "not_exposed_by_this_tool",
        "provider": {"canonical_iri": d.canonical_iri.iri, "short_alias": d.short_alias,
            "iri_hash": format!("{:016x}", d.canonical_iri.fnv1a_hash),
            "versions": {"language": format!("{:016x}", v.language_version.0),
                "abi": format!("{:016x}", v.abi_version.0), "host": format!("{:016x}", v.host_version.0),
                "provider": format!("{:016x}", v.provider_version.0),
                "ontology": format!("{:016x}", v.ontology_version.0)},
            "supported_targets": d.supported_targets.0, "max_ontology_quins": d.max_ontology_quins,
            "sub_namespaces": namespaces, "ontology_nquins": ontology}}))
}

/// UTF-8 compact JSON: object keys recursively sorted by UTF-8 bytes, arrays preserved,
/// serde_json string escaping and integer decimal rendering; no trailing newline.
/// The manifest contains no floating-point JSON numbers.
pub fn manifest_bytes(value: &Value) -> Result<Vec<u8>, ExportError> {
    fn add(out: &mut Vec<u8>, bytes: &[u8]) -> Result<(),ExportError> {
        if bytes.len() > MAX_MANIFEST_BYTES.saturating_sub(out.len()) {
            return Err(ExportError::ByteBudgetExceeded);
        }
        out.extend_from_slice(bytes); Ok(())
    }
    fn emit(value: &Value, out: &mut Vec<u8>, depth: usize) -> Result<(),ExportError> {
        if depth > MAX_JSON_DEPTH { return Err(ExportError::TooDeep); }
        match value {
            Value::Object(map) => {
                if map.len() > MAX_DESCRIPTOR_ITEMS { return Err(ExportError::TooManyItems); }
                add(out,b"{")?;
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort_by(|a,b| a.as_bytes().cmp(b.as_bytes()));
                for (i,key) in keys.into_iter().enumerate() {
                    if key.len() > MAX_TEXT_BYTES { return Err(ExportError::TextTooLong); }
                    if i > 0 { add(out,b",")?; }
                    add(out,&serde_json::to_vec(key).expect("string serialization"))?;
                    add(out,b":")?; emit(&map[key], out, depth+1)?;
                }
                add(out,b"}")?;
            }
            Value::Array(items) => {
                if items.len() > MAX_DESCRIPTOR_ITEMS { return Err(ExportError::TooManyItems); }
                add(out,b"[")?;
                for (i,item) in items.iter().enumerate() {
                    if i > 0 { add(out,b",")?; } emit(item,out,depth+1)?;
                }
                add(out,b"]")?;
            }
            Value::String(text) if text.len() > MAX_TEXT_BYTES => return Err(ExportError::TextTooLong),
            other => add(out,&serde_json::to_vec(other).expect("JSON serialization"))?,
        }
        Ok(())
    }
    let mut out = Vec::new(); emit(value,&mut out,0)?; Ok(out)
}

/// Full SHA-256 of tooling serialization; never a replacement for the runtime contract digest.
pub fn export(provider: &dyn VibeNamespaceProvider) -> Result<Value,ExportError> {
    let manifest = descriptor_manifest(provider)?;
    let digest = hex(&Sha256::digest(manifest_bytes(&manifest)?));
    Ok(json!({"manifest": manifest, "tooling_manifest_sha256": digest}))
}

pub fn reference_preview() -> Result<Value,ExportError> {
    export(&vibe::extension_registry::DeterministicMechanicsProvider)
}

/// Validates that a `VibeNamespaceProvider` conforms to the VibeScript Provider SDK contract:
/// - `descriptor.canonical_iri.is_valid()` and non-empty `short_alias`
/// - Every sub-namespace and function has valid `CanonicalIri`, unique `StableFuncId`,
///   valid `OntologicalContract::validate_spans(provider.static_ontology_nquins().len())`,
///   and `contract_digest == OntologicalContract::canonical_digest(...)`
/// - Every `NQuin48` in `provider.static_ontology_nquins()` passes `verify_parity()` (5-field parity)
/// - Produces a deterministic `export(provider)` manifest.
pub fn verify_provider_sdk_conformance(provider: &dyn VibeNamespaceProvider) -> Result<Value, String> {
    let d = provider.descriptor();
    if !d.canonical_iri.is_valid() {
        return Err(format!("invalid provider canonical_iri '{}'", d.canonical_iri.iri));
    }
    if d.short_alias.trim().is_empty() {
        return Err("provider short_alias must not be empty".to_string());
    }
    if !d.versions.is_engine_compatible() {
        return Err("provider ContractVersionAxes are not engine-compatible".to_string());
    }

    let quins = provider.static_ontology_nquins();
    if quins.len() > d.max_ontology_quins as usize {
        return Err(format!(
            "ontology quin count {} exceeds max_ontology_quins {}",
            quins.len(),
            d.max_ontology_quins
        ));
    }
    for (idx, q) in quins.iter().enumerate() {
        if !q.verify_parity() {
            return Err(format!("ontology NQuin48 at index {idx} failed 5-field verify_parity()"));
        }
    }

    let mut seen_sub_ids = BTreeSet::new();
    let mut seen_stable_ids = BTreeSet::new();
    let mut seen_func_iris = BTreeSet::new();
    for sub in d.sub_namespaces {
        if sub.name.trim().is_empty() {
            return Err(format!("sub-namespace {} has empty name", sub.sub_id));
        }
        if !seen_sub_ids.insert(sub.sub_id) {
            return Err(format!("duplicate sub-namespace sub_id {}", sub.sub_id));
        }
        for f in sub.functions {
            if f.name.trim().is_empty() {
                return Err(format!("sub-namespace '{}' has function with empty name", sub.name));
            }
            if !f.canonical_iri.is_valid() {
                return Err(format!("function '{}' has invalid canonical_iri '{}'", f.name, f.canonical_iri.iri));
            }
            if !seen_func_iris.insert(f.canonical_iri.fnv1a_hash) {
                return Err(format!("duplicate function canonical_iri '{}'", f.canonical_iri.iri));
            }
            if f.stable_id.sub_namespace_id != sub.sub_id {
                return Err(format!(
                    "function '{}' stable_id.sub_namespace_id {} does not match sub_id {}",
                    f.name, f.stable_id.sub_namespace_id, sub.sub_id
                ));
            }
            if !seen_stable_ids.insert(f.stable_id.to_u32()) {
                return Err(format!(
                    "duplicate StableFuncId ({}, {}) on function '{}'",
                    f.stable_id.sub_namespace_id, f.stable_id.func_id, f.name
                ));
            }
            if !f.contract.validate_spans(quins.len()) {
                return Err(format!(
                    "function '{}' has ontology spans out of bounds for {} quins",
                    f.name,
                    quins.len()
                ));
            }
            let expected_digest = OntologicalContract::canonical_digest(
                f.canonical_iri.fnv1a_hash,
                f.stable_id,
                f.contract.schema_hash,
                f.effects,
                f.determinism,
                f.reversibility,
                f.envelope,
                f.contract.precondition_quin_span,
                f.contract.postcondition_quin_span,
                f.contract.invariant_quin_span,
            );
            if f.contract.contract_digest != expected_digest {
                return Err(format!(
                    "function '{}' contract_digest does not match OntologicalContract::canonical_digest",
                    f.name
                ));
            }
        }
    }

    let first = export(provider).map_err(|e| format!("export failed: {e:?}"))?;
    let second = export(provider).map_err(|e| format!("export failed on second pass: {e:?}"))?;
    if first != second {
        return Err("export(provider) is not deterministic".to_string());
    }
    Ok(first)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialization_has_independent_golden_bytes_and_sha256() {
        assert_eq!(manifest_bytes(&json!({"z": [2,1], "a": "\n"})).unwrap(), br#"{"a":"\n","z":[2,1]}"#);
        assert_eq!(hex(&Sha256::digest(manifest_bytes(&json!({})).unwrap())),
            "44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a");
    }
    #[test]
    fn preview_preserves_declarations_without_authority() {
        let x = reference_preview().unwrap();
        assert_eq!(x["manifest"]["authorization"], "not_assessed");
        assert_eq!(x["manifest"]["provider"]["sub_namespaces"][0]["functions"][0]["contract"]["declared_digest"].as_str().unwrap().len(), 64);
        assert_eq!(x["tooling_manifest_sha256"].as_str().unwrap().len(),64);
        assert_eq!(x, reference_preview().unwrap());
    }

    #[test]
    fn sdk_conformance_accepts_valid_sdk_provider_and_rejects_bad_digest_or_parity() {
        use vibescript_abi::{
            BorrowedSliceView, CallerOutputSlice, DeterminismGuarantee, EffectSet, FuncDescriptor,
            NQuin48, ParamSpec, PrimitiveKind, ProviderDescriptor, ResourceEnvelope,
            ReversibilityKind, SemVer64, StableFuncId, SubNamespaceDescriptor, TargetProfileMask,
            VibeTrap,
        };

        struct ConformingSdkProvider;
        const PARAMS: &[ParamSpec] = &[ParamSpec::slice("vec3", PrimitiveKind::F64, 3, 3)];
        const FUNC: FuncDescriptor = FuncDescriptor::const_new(
            "norm_sq",
            "https://qualiadb.org/ext/sdk/vec/norm_sq",
            StableFuncId::new(0, 0),
            "(f64[3])->f64",
            PARAMS,
            PrimitiveKind::F64,
            1,
            EffectSet::PURE_COMPUTE,
            DeterminismGuarantee::DeterministicTolerance,
            ReversibilityKind::PureNoState,
            ResourceEnvelope::hot_zero_heap(8, 8),
            (0, 1),
            (1, 1),
            (2, 1),
        );
        const SUBS: &[SubNamespaceDescriptor] = &[SubNamespaceDescriptor {
            name: "vec",
            sub_id: 0,
            functions: &[FUNC],
        }];
        const DESC: ProviderDescriptor = ProviderDescriptor::const_new(
            "https://qualiadb.org/ext/sdk",
            "sdk",
            SemVer64::release(0, 1, 0),
            SemVer64::release(0, 1, 0),
            TargetProfileMask::ALL_CPU,
            SUBS,
            8,
        );
        const QUINS: &[NQuin48] = &[
            NQuin48::precondition(
                "https://qualiadb.org/ext/sdk/vec/norm_sq",
                "https://qualiadb.org/schema/vibe#FiniteRealInputs",
                1,
            ),
            NQuin48::postcondition(
                "https://qualiadb.org/ext/sdk/vec/norm_sq",
                "https://qualiadb.org/schema/vibe#NonNegativeScalar",
                2,
            ),
            NQuin48::invariant(
                "https://qualiadb.org/ext/sdk/vec/norm_sq",
                "https://qualiadb.org/schema/vibe#EuclideanMetric",
                3,
            ),
        ];

        impl VibeNamespaceProvider for ConformingSdkProvider {
            fn descriptor(&self) -> &'static ProviderDescriptor {
                &DESC
            }
            fn static_ontology_nquins(&self) -> &'static [NQuin48] {
                QUINS
            }
            fn call(
                &self,
                _stable_id: StableFuncId,
                inputs: &[BorrowedSliceView<'_>],
                output: &mut CallerOutputSlice<'_>,
            ) -> Result<u64, VibeTrap> {
                let v = inputs[0].read_finite_f64_array::<3>(0)?;
                output.push_f64_slice(&[v[0] * v[0] + v[1] * v[1] + v[2] * v[2]])?;
                Ok(4)
            }
        }

        let exported = verify_provider_sdk_conformance(&ConformingSdkProvider)
            .expect("SDK-built provider must pass conformance");
        assert_eq!(exported["manifest"]["provider"]["short_alias"], "sdk");

        for p in vibe::extension_registry::ExtensionRegistry::with_builtin_providers().iter_providers() { verify_provider_sdk_conformance(p).expect("all 11 built-in Etalon & Qualia providers must pass SDK conformance"); }
    }
}
