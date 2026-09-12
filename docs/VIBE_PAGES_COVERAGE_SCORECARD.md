# Vibe Pages coverage scorecard

**Prior UAT tip:** vibe-script `d7774789b` (PR #5 sole-push) · **SoT:** QualiaDB `v0.0.38`/`cdd7ae728`
**This fold:** complete family map kept; customer-voice rewrite on `reality.html` / `atlas.html` (showcase, not essay)
**Owner:** Capt.

## Scores

| Gate | Score | Evidence |
|------|-------|----------|
| Pages WASM stamp | **PASS** | `docs/pkg/vibe/package.json` → `vibe-wasm` **0.0.38** |
| Logic false-held cleared | **PASS** | N3 / SHACL / Deontic / Epistemic cards = **Works here** |
| Status chips | **PASS** | User-facing chips only: **Works here** · **Works in Poet / Desktop** · **Planned** (not built). No held/not yet lead copy. |
| Complete family presentation | **MAP READY · Capt re-UAT** | SoT **104** families named on `reality.html` `#family-roster` and `atlas.html` `#family-roster`. Prior tip `d7774789b` was **FAIL** (56 named / 48 absent). This fold: **missing = 0** on page text. Capt re-UAT toward COMPLETE FAMILY MAP. |
| Live GH Pages deploy | **PENDING** | Confirm mediaprophet.github.io after Neo sole-push |

## Prior FAIL (tip `d7774789b`)

`reality.html` named **56** families. **48** absent from page text:

Bioinformatics, Calculus, CalculusWorkbench, CapabilityDiscovery, ChatGraph, ComputationalGeometry, ComputerVision, Constructibility, CooperativeWork, Document, EngineeringAnalysis, FinancialModeling, GeometricAlgebra, GraphMatch, InfraExtLogic, InfraLogic, IntegralTransforms, Interpolation, MachineLearning, Manifold, Medical, MedicalComputing, MedicalImaging, Net, NumberTheory, NumericalCalculus, Ode, Optimization, OrganicChemistry, PhysicalUnits, PhysicsAndODE, PhysicsWorkbench, PolynomialAlgebra, Privacy, QuantumAndCryptographic, Sheet, Social, SpatialLogic, SpecialFunctions, SpecialFunctionsAndTransforms, Spectral, SymbolicAlgebra, SymbolicODE, VectorCalculus, biosignal, hash, nlp, sampler.

## This fold (page text)

- Searchable compact roster on **What it can do** (`docs/reality.html`) and the **Capability atlas** (`docs/atlas.html`).
- Every Capt SoT family is a visible `<code>Family</code>` tile with method count.
- Already-partial families stay named: ClinicalRisk, Statistics, Econ, Research, LinearAlgebra, Animation, logic stack.
- Lowercase `agent` (`agent.dag.*`) is also named — catalog prefix on `ALL_INVOKE_IDS`, omitted from Capt’s 104-row table. Do not hide SoT.
- Native-only binds stay on the map with **see Poet/Desktop** / held. Live LocalHost where this WASM evals.
- Title stays **What it can do**. URL stays `reality.html`. vibe-script version stays **0.0.1**. Engine not forked.

Machine list: [`docs/sot-families.json`](sot-families.json) (105 prefixes / 1107 `ALL_INVOKE_IDS` rows; Capt table cites 104 / 1101 because TADL was counted as 1 and `agent.dag.*` was omitted).

## Next

1. Capt: re-UAT — FAIL until `missing` = 0 on the live Pages tip (or explicit “see Poet/Desktop” for native-only with the family still named).
2. Neo sole-pushes. Do not merge this PR from the agent.
3. Confirm GitHub Pages Actions deployed the sole-push tip.
