//! Independently authored consumer checks, using the engine registry and ABI directly.
use vibe::extension_registry::{prepare_extension_call, DeterministicMechanicsProvider, ExtensionRegistry};
use vibescript_abi::*;

#[test]
fn independent_abi_vectors() {
    assert_eq!(q_hash(b"hello"), 0xa430d84680aabd0b);
    assert_eq!(core::mem::size_of::<NQuin48>(), 48);
    assert_eq!(core::mem::align_of::<NQuin48>(), 16);
    assert_eq!(VibeOpCode::pack(19, StableFuncId::new(7,42)).stable_id(), StableFuncId::new(7,42));
    assert!(!SemVerReq::Caret(SemVer64::release(0,4,0)).matches(SemVer64::release(0,5,0)));
    assert!(!SemVerReq::Caret(SemVer64::release(0,0,3)).matches(SemVer64::release(0,0,4)));
}

#[test]
fn canonical_alias_target_and_unknown_resolution() {
    let provider = DeterministicMechanicsProvider;
    let mut registry = ExtensionRegistry::new();
    registry.register(&provider).unwrap();
    let resolve = |namespace, name, target| prepare_extension_call(&registry, namespace,
        "kinematics", name, SemVerReq::AnyStable, target);
    let a = resolve("mech", "kinetic_energy", TargetProfileMask::NATIVE_DESKTOP).unwrap();
    let b = resolve(provider.descriptor().canonical_iri.iri, "kinetic_energy", TargetProfileMask::NATIVE_DESKTOP).unwrap();
    assert_eq!(a,b);
    assert_eq!(resolve("missing", "kinetic_energy", TargetProfileMask::NATIVE_DESKTOP), Err(VibeTrap::NamespaceNotFound));
    assert_eq!(resolve("mech", "missing", TargetProfileMask::NATIVE_DESKTOP), Err(VibeTrap::FunctionNotFound));
    assert_eq!(resolve("mech", "kinetic_energy", TargetProfileMask::GPU_COMPUTE), Err(VibeTrap::TargetProfileUnsupported));
}

struct MetadataProvider(&'static ProviderDescriptor);
impl VibeNamespaceProvider for MetadataProvider {
    fn descriptor(&self) -> &'static ProviderDescriptor { self.0 }
    fn static_ontology_nquins(&self) -> &'static [NQuin48] {
        DeterministicMechanicsProvider.static_ontology_nquins()
    }
    fn call(&self, _: StableFuncId, _: &[BorrowedSliceView<'_>], _: &mut CallerOutputSlice<'_>) -> Result<u64,VibeTrap> {
        panic!("discovery must never execute provider")
    }
}
fn modified(change: impl FnOnce(&mut ProviderDescriptor)) -> MetadataProvider {
    let mut d = *DeterministicMechanicsProvider.descriptor(); change(&mut d);
    MetadataProvider(Box::leak(Box::new(d)))
}
#[test]
fn registration_rejects_hash_and_version_and_missing_metadata_atomically() {
    for (provider, expected) in [
        (modified(|d| d.canonical_iri.fnv1a_hash ^= 1), VibeTrap::HashMismatch),
        (modified(|d| d.versions.abi_version = SemVer64::release(99,0,0)), VibeTrap::IncompatibleVersionAxes),
        (modified(|d| d.short_alias = ""), VibeTrap::InvalidCanonicalIri),
        (modified(|d| d.max_ontology_quins = 0), VibeTrap::OntologyBudgetExceeded),
    ] {
        let mut r = ExtensionRegistry::new(); let generation = r.generation();
        assert_eq!(r.register(&provider), Err(expected));
        assert_eq!(r.active_count(),0); assert_eq!(r.generation(), generation);
    }
}
#[test]
fn alias_collision_rejected_and_export_does_not_execute() {
    let first = DeterministicMechanicsProvider;
    let second = modified(|d| d.canonical_iri = CanonicalIri::new("https://example.org/provider"));
    let mut r = ExtensionRegistry::new(); r.register(&first).unwrap();
    assert_eq!(r.register(&second),Err(VibeTrap::DuplicateShortAlias));
    assert_eq!(vibe_tooling::export(&second).unwrap()["manifest"]["discovery_only"],true);
}

#[test]
fn upstream_duplicate_stable_id_must_be_rejected() {
    let provider = modified(|d| {
        let mut functions = d.sub_namespaces[0].functions.to_vec();
        functions[1].stable_id = functions[0].stable_id;
        let mut subs = d.sub_namespaces.to_vec();
        subs[0].functions = Box::leak(functions.into_boxed_slice());
        d.sub_namespaces = Box::leak(subs.into_boxed_slice());
    });
    assert_eq!(ExtensionRegistry::new().register(&provider),Err(VibeTrap::DuplicateStableFuncId));
}
