use serde_json::json;
use vibe_tooling::*;
#[test]
fn resource_limits_fail_without_publishing_partial_output() {
    let mut nested = json!(0);
    for _ in 0..MAX_JSON_DEPTH+1 { nested = json!([nested]); }
    assert_eq!(manifest_bytes(&nested), Err(ExportError::TooDeep));
    assert_eq!(manifest_bytes(&json!("x".repeat(MAX_TEXT_BYTES+1))),Err(ExportError::TextTooLong));
    assert_eq!(manifest_bytes(&json!(vec![0;MAX_DESCRIPTOR_ITEMS+1])),Err(ExportError::TooManyItems));
    let row = vec!["x".repeat(MAX_TEXT_BYTES);MAX_DESCRIPTOR_ITEMS];
    assert_eq!(manifest_bytes(&json!([row.clone(),row])),Err(ExportError::ByteBudgetExceeded));
}
#[test]
fn semantic_mutations_change_full_digest() {
    use vibe::extension_registry::DeterministicMechanicsProvider;
    let original = descriptor_manifest(&DeterministicMechanicsProvider).unwrap();
    let original_bytes = manifest_bytes(&original).unwrap();
    let paths = [
        "/provider/sub_namespaces/0/functions/0/params/0/max_elems",
        "/provider/sub_namespaces/0/functions/0/effects",
        "/provider/sub_namespaces/0/functions/0/envelope/max_output_bytes",
        "/provider/ontology_nquins/0/0",
    ];
    use sha2::{Digest,Sha256};
    for path in paths {
        let mut changed = original.clone();
        let slot = changed.pointer_mut(path).unwrap();
        *slot = if slot.is_string() { json!("0000000000000000") } else { json!(999) };
        assert_ne!(Sha256::digest(manifest_bytes(&changed).unwrap()),Sha256::digest(&original_bytes),"{path}");
    }
}
