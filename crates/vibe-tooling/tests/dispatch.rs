//! Consumer tests with numeric expectations fixed independently of provider oracle helpers.
use vibe::extension_registry::*;
use vibescript_abi::*;
fn context(mode: ExecutionMode) -> CallExecutionContext {
    CallExecutionContext { actor_did_hash: 7, context_world_hash: 1, lamport_timestamp: 5, mode }
}
#[test]
fn numeric_dispatch_and_lifecycle_failures() {
    let provider = DeterministicMechanicsProvider;
    let mut registry = ExtensionRegistry::new();
    let slot = registry.register(&provider).unwrap();
    let handle = prepare_extension_call(&registry,"mech","kinematics","kinetic_energy",
        SemVerReq::AnyStable,TargetProfileMask::NATIVE_DESKTOP).unwrap();
    let mass = 2.0f64.to_le_bytes();
    let velocity: Vec<_> = [3.0f64,4.0,0.0].into_iter().flat_map(f64::to_le_bytes).collect();
    let inputs = [BorrowedSliceView::from_raw_bytes(&mass,PrimitiveKind::F64,1).unwrap(),
        BorrowedSliceView::from_raw_bytes(&velocity,PrimitiveKind::F64,3).unwrap()];
    let mut bytes = [0;8];
    let mut out = CallerOutputSlice::new(&mut bytes,PrimitiveKind::F64).unwrap();
    let result = dispatch_prepared_call(&registry,&handle,&context(ExecutionMode::StandaloneDeveloper),None,&inputs,&mut out).unwrap();
    assert_eq!(out.written_byte_slice(),25.0f64.to_le_bytes());
    assert_eq!(result.receipt.status,CompletionStatus::StandaloneCompleted);
    assert!(!result.receipt.simulated);
    assert!(result.audit_nquins.iter().all(NQuin48::verify_parity));
    let governed = context(ExecutionMode::SimulatedGovernanceTest);
    let cases = [VibeTrap::SentinelUnavailable,VibeTrap::ExplicitDenyRule,VibeTrap::ResourceEnvelopeExceeded,VibeTrap::ReceiptPersistenceFailed];
    for expected in cases {
        let mut gate = SimulatedPolicyGate::new_for_actor(7);
        match expected {
            VibeTrap::SentinelUnavailable => gate.controller_alive=false,
            VibeTrap::ExplicitDenyRule => gate.explicit_deny_func_hash=handle.func_iri_hash,
            VibeTrap::ResourceEnvelopeExceeded => gate.budget.max_ticks=0,
            VibeTrap::ReceiptPersistenceFailed => gate.fail_receipt_commit=true,
            _ => unreachable!(),
        }
        let mut bytes=[0;8]; let mut out=CallerOutputSlice::new(&mut bytes,PrimitiveKind::F64).unwrap();
        assert_eq!(dispatch_prepared_call(&registry,&handle,&governed,Some(&mut gate),&inputs,&mut out),Err(expected));
        assert_eq!(gate.committed_receipts_count,0);
    }
    let mut bytes=[0;8]; let mut out=CallerOutputSlice::new(&mut bytes,PrimitiveKind::F64).unwrap();
    assert_eq!(dispatch_prepared_call(&registry,&handle,&context(ExecutionMode::GovernedApplication),None,&inputs,&mut out),Err(VibeTrap::SentinelUnavailable));
    let mut empty=[]; let mut out=CallerOutputSlice::new(&mut empty,PrimitiveKind::F64).unwrap();
    let mut deny = SimulatedPolicyGate::new_for_actor(7); deny.explicit_deny_func_hash=handle.func_iri_hash;
    // Capacity failure wins over policy denial, proving pre-admission validation ordering.
    assert_eq!(dispatch_prepared_call(&registry,&handle,&governed,Some(&mut deny),&inputs,&mut out),Err(VibeTrap::OutputBufferTooSmall));
    assert_eq!(out.written_bytes(),0);
    registry.unregister(slot).unwrap();
    assert_eq!(dispatch_prepared_call(&registry,&handle,&governed,None,&inputs,&mut out),Err(VibeTrap::StaleRegistryGeneration));
}
#[test]
fn wrong_input_kind_count_and_arity_fail_before_execution() {
    let provider = DeterministicMechanicsProvider;
    let mut registry=ExtensionRegistry::new();registry.register(&provider).unwrap();
    let handle=prepare_extension_call(&registry,"mech","kinematics","kinetic_energy",SemVerReq::AnyStable,TargetProfileMask::NATIVE_DESKTOP).unwrap();
    let raw=[0u8;24];
    let scalar=BorrowedSliceView::from_raw_bytes(&raw[..8],PrimitiveKind::F64,1).unwrap();
    let vector=BorrowedSliceView::from_raw_bytes(&raw,PrimitiveKind::F64,3).unwrap();
    let wrong=BorrowedSliceView::from_raw_bytes(&raw[..8],PrimitiveKind::U64,1).unwrap();
    for (inputs,expected) in [(vec![],VibeTrap::ArityMismatch),
        (vec![wrong,vector],VibeTrap::TypeMismatch),(vec![scalar,scalar],VibeTrap::SliceLengthMismatch)] {
        let mut bytes=[0;8];let mut out=CallerOutputSlice::new(&mut bytes,PrimitiveKind::F64).unwrap();
        assert_eq!(dispatch_prepared_call(&registry,&handle,&context(ExecutionMode::StandaloneDeveloper),None,&inputs,&mut out),Err(expected));
        assert_eq!(out.written_bytes(),0);
    }
}
