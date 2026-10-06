# S01 Stage A — candidate schema and validator

This is a reviewable implementation slice, **not an accepted or frozen contract**.
The five shared findings were repaired in2828cd8 and independently
[accepted at9ca645a](../plans/ui-blueprint/receipts/S01-recheck.md). That bounded
acceptance does not freeze the schema or accept all S01/runtime behavior. D05
retained policy is selected; working-memory enforcement and later live gates remain open.
Authority: [S01 packet](../plans/ui-blueprint/packets/S01.md), C01 D03/D07 and
[GOLDEN01](../specs/acceptance/golden.md). S01 extends the T01 schema owner in place.

## Canonical owners and validation layers

- [Rust records](../../crates/schema/src/model.rs) own wire types. `SchemaVersion`
  remains the T01 exact `0.1.0` boundary. Boxed payloads change Rust storage only.
- [Generated JSON Schema](../../schemas/uiblueprint-0.1.0.schema.json) describes
  structural shape. It is generated from those Rust records, never from examples.
- [Semantic validator](../../crates/schema/src/validation.rs), with graph/outcome
  modules, checks contextual constraints. The accepted repairs remain protected.
- [Observation plugin API](plugin-interface.md) uses the same records; no adapter DTO
  fork, OS SDK, runtime collector, geometry engine or graph cache is added.

`Document { schema_version, artifact }` carries a tagged artifact. Artifacts cover
request/session/capability, properties, observations, snapshots, geometry, delta,
actions/transitions, expectations/findings, errors, channel replies and GOLDEN01.
Validation bundles (`DeltaCase`, `ActionCase`, `FindingCase`, etc.) supply context
explicitly. They are validator inputs; transport can carry the same inner records
without embedding an entire history. No ID, payload_ref, file or URL inside a
document is dereferenced by the validator.

Properties distinguish selection from availability; known false and empty text
remain values. Unknown/unsupported/redacted carry no value. Tag-only records now
use strict empty struct variants, rejecting extra members instead of discarding
them. Focus has separate known target, known none, unknown and not_requested
states. Native roles, namespaced extensions and source declarations are preserved
separately. Sensitive declarations/properties require redacted state. This checks
declared sensitivity, not universal discovery of secrets in arbitrary UI text.

Identity IDs are nonempty strings bounded to256 Unicode scalar values; UI text is
not an ID. Context retains target/surface generations, scope/fields/projection,
plugin/schema versions and environment revision. Geometry keeps frame kind,
space/unit/origin and explicit transforms; length units stay px/css_px/pt/dp.
Declared result quantities distinguish length, area and ratio separately from
geometry units. Source clocks and parent deadlines are not silently compared.

Serde is the only JSON parser. Map-only record visitors reject array-shaped
records, duplicate/unknown members and invalid IDs without adding another parser.
Serde errors are discarded at the public `Document::from_json` boundary; returned
errors retain no raw input. JSON Schema is a structural checker, not a substitute
for semantic reference/evidence checks. Semantic-negative examples may therefore
be structurally valid; the manifest labels these two expectations separately.

## Independent oracle mapping and scope of proof

[Independent oracle corpus](../../fixtures/golden-oracles/README.md), checkpoint
`d539a0ec2bac243375256652fb75da7065b27350`, remains read-only to Integration.
[Wire manifest](../../fixtures/golden/manifest.json) maps97 case IDs to126 concrete
fixtures, expanding independent omission/variant cases. Expected exits are checked
against the author's expected.json, not inferred from our validator.
[build.py](../../fixtures/golden/build.py) translates those logical facts into the
single wire model; it is fixture tooling, not an analytics engine.

GOLDEN01 supplies S10, resolved checkbox action, delivery, S11 verification,
compatible delta, finding and limited compare export record. Error outcomes are
valid records: validator exit0 does not mean the requested operation succeeded.
Geometry amounts are declared authored oracle records; S01 checks representation,
evidence and result consistency. G01 must independently compute arithmetic and
K01 must implement replay/cache. Synthetic records do not prove real delivery,
platform capture, privacy across every channel or runtime acceptance.

## CLI and reproduction

```sh
cargo +1.96.0 run --locked -p uiblueprint-schema --bin uiblueprint-validate -- --max-bytes 1048576 fixtures/golden/GOLDEN01.json
cargo +1.96.0 run --locked -p uiblueprint-schema --example generate_schema
cargo +1.96.0 test --locked -p uiblueprint-schema -p uiblueprint-plugin-api
```

The byte limit is explicit, not a production default. `-` reads stdin. The CLI
reads at most limit+1 bytes, emits one payload-free JSON result on stdout, and
does not mix diagnostics with it. Exits:0 valid record,2 invalid/bounded-input
failure,1 IO/internal failure. No implicit URL fetching or schema loading occurs.
Saved-schema equality,126 CLI exits, round-trip/parity, IO/bounds and private-error
checks are covered by focused tests. Regenerate the schema only after changing
the canonical types; its equality test detects drift.

## Remaining acceptance boundaries

The five shared findings are closed by the independent recheck above. Existing
[Web](../plans/ui-blueprint/receipts/S01-web-proof.md) and
[Native](../plans/ui-blueprint/receipts/S01-native-proof.md) D02 receipts provide
bounded actual wire evidence, not full adapter/P1 acceptance. D05@2 selects
[retained limits](../specs/development/decisions/d05-memory.md); decoder/replay/host
working-memory enforcement remains open. New cache source or a passed schema
suite does not close that live gate. Historical fixtures/evidence stay unchanged.

<a id="l01-analysis-result-contract"></a>

## L01 analysis result decision — implementation-ready, not yet registered

This replaces the earlier proposal. The finite
[L01-result-contract packet](../plans/ui-blueprint/packets/L01-result-contract.md)
under PLAN.UIB@1 and ROADMAP's delegated D03 choices authorizes this engineering
decision. Root must pin its spec delta and source packet before implementation.
No source, wire fixture, product-spec revision or released behavior changes here.
Existing requirement: measurements are facts; checks add an actual sourced
Expectation; selected Space, units, details, evidence and unknowns must survive.
Observed: G01 already computes these locally, but L01 cannot serialize standalone
measurement/converted check or load additional transforms/observed conditions.
The representation below resolves that gap without changing geometric meaning.

### Version boundary and canonical owner

Keep core `Document`, `Artifact`, `SchemaVersion::CURRENT=0.1.0`, all core DTOs and
`schemas/uiblueprint-0.1.0.schema.json` unchanged. Add `schema::analysis` with
`AnalysisDocument { schema_version: AnalysisVersion, artifact: AnalysisArtifact }`.
AnalysisVersion accepts exactly 0.2.0. Crate package versions are independent;
this requires no Cargo/dependency update or published Rust API compatibility claim.
This is the D03 minor-version boundary for
new incompatible artifact shapes, not permission to reinterpret0.1 documents.
Analysis artifacts reuse the SAME Snapshot/Context/Expectation/Finding/geometry/
evidence Rust types; there is no new normalized graph or copied model family.
Outer0.2 identifies the analysis artifact; embedded Snapshot.context.schema_version
remains0.1 and describes its unchanged source data. Never relabel it0.2.

Strict artifact tags are `geometry_query`, `evaluation_input`, `measurement` and
`geometry_check`, using the existing `{kind,data}` convention. Unknown tags,
versions/members, duplicates, array-shaped records and value-bearing tag-only
records reject. Use the existing map-only record mechanism, shared crate-locally
if needed; do not broaden the old parser or introduce a second JSON parser.
Generate a separate `schemas/uiblueprint-analysis-0.2.0.schema.json` from these
canonical types. The existing0.1 generated schema must remain structurally equal
and byte-identical unless a separately authorized0.1 change occurs.

`Document::from_json` remains strict0.1; `AnalysisDocument::from_json` is strict0.2.
A bounded schema-owned `validate_input_document(bytes,max_bytes)` dispatches exact
version to those decoders for `uiblueprint-validate`; it is not plugin negotiation.
Its version probe skips the body without retaining untyped UI values and detects
duplicate version fields; the selected strict decoder still validates the whole
object. It never coerces versions, loads references or uses serde errors as output.
The validator keeps its existing stdout/exits0/2/1 for both formats. Exit0 means
contract-valid declarations, not an independently recomputed measurement.

### Exact records and JSON shapes

All fields below are canonical, map-only and serialized in snake_case. `Option`
fields use explicit JSON null for missing optional metadata, not property unknown.
All reused IDs/numbers/evidence retain their existing bounds and validity rules.

```rust
GeometryQuery {
    id: Id, scope_id: Id, targets: Vec<SourceKey>,
    operation: GeometryRelation, anchors: Vec<Anchor>,
    quantity_kind: QuantityKind, units: Unit,
    applies_when: ContextConditions,
}
ObservedConditions { values: ContextConditions, evidence: Evidence }
EvaluationInput {
    snapshot_id: Id, revision: u64, context: Context,
    result_space: Space, transforms: Vec<Transform>,
    conditions: Option<ObservedConditions>,
}
Measurement { value: Value, space: Space, details: MeasurementDetails, evidence: Vec<Evidence> }
MeasurementCase { snapshot: Snapshot, query: GeometryQuery, evaluation: EvaluationInput, result: MeasurementResult }
GeometryCheckCase { snapshot: Snapshot, expectation: Expectation, evaluation: EvaluationInput, measurement: MeasurementResult, finding: Finding }
```

MeasurementResult is internally tagged by `status`: known has exactly
`{status:"known",measurement:Measurement}`; unknown has exactly
`{status:"unknown",reason:MeasurementUnknownReason,evidence:[Evidence]}`.
Known `value` must be the existing Value::Quantity, including known zero;
unknown has no value/details/fabricated numeric field. Selected destination is
available even for unknown through EvaluationInput.result_space.

MeasurementDetails is internally tagged by `kind`: `scalar` has no payload;
`insets` has finite left/top/right/bottom; `intersection` has `rect:Option<Rect>`;
`gaps` has `values:Vec<f64>`. The scalar variant is strict empty-struct syntax.
Intersection null means known empty intersection/area0, not unavailable geometry.
Inside/overflow use insets; intersects uses intersection; equal_spacing uses gaps
of length anchors.len()-1; other supported operations use scalar. Preserve signed
insets/gaps and exact supplied fractions; no padding/occlusion interpretation.
Area and ratio retain QuantityKind; Value::Quantity.source_units identifies the
selected coordinate units (area is their square, ratio is dimensionless). No new
length-unit vocabulary or hidden accuracy tolerance is introduced.

MeasurementUnknownReason preserves the current engine's closed wire strings:
`not_requested`, `unknown_property`, `unsupported_property`, `redacted_property`,
`target_unresolved`, `missing_transform`, `frame_kind_mismatch`, `unsupported_shape`,
`incomplete_scope`, `unstable_state`, `applicability_unknown`, `not_applicable`,
`undefined_ratio`. Unknown retains attempted evidence in actual evaluation order;
known retains all contributing property/condition/transform evidence, deduplicated
by full Evidence equality in first-use order. Do not require unused input evidence
in the result, discard secondary observations or invent one aggregate Observation.

### Input binding, transforms and applicability

EvaluationInput must match the supplied Snapshot ID/revision and compatible full
Context, including source schema/plugin versions, generations, scope/fields/
projection and environment revision. The Snapshot is immutable. Query scope must
match it; anchors must belong to query.targets, but a missing node remains a
supported unknown outcome rather than a fabricated element. Reuse existing
G01 relation arities, axes and QuantityKind rules in one shared shape-validation
owner; this moves validation vocabulary, not arithmetic, into schema.

`result_space` is the full explicit Space, not an ID or units-only inference.
Transforms are ordered directional canonical Transform records, bound to the
same target/environment and a declared source Surface. Consumed transforms must
bind the anchor node Surface. Finite nonsingular affine mappings and exact source/
destination Space records are required; no implicit inverse/origin/scale discovery.
G01 retains its deterministic existing path order and shape/baseline restrictions.
An unused transform does not prove it was used; merely attaching a destination
with matching units cannot validate a calculated value.

All supplied transform/condition Evidence must resolve to existing Snapshot
Observations with matching namespace and valid uncertainty/method. This finite
contract adds NO supplemental Observation registry and never appends observations
to the Snapshot to make an external fact pass. New independently collected facts
require an ordinary new canonical Snapshot containing their actual observations;
external observation acquisition/merging is a separate adapter responsibility.
All non-null condition fields share the declared Evidence, as the existing engine
API requires. Facts from different observations cannot be silently merged under
one Evidence; a per-field multi-observation condition input is a named follow-up,
not claimed here. These restrictions are explicit, not an unsupported fallback.

Conditions are supplied observed facts, never copied from Expectation.applies_when
or inferred from OS/backend/name. Missing required facts -> applicability_unknown;
proved mismatch -> not_applicable/unknown; unstable evidence -> unstable_state.
No conditions are required when applies_when is empty. Preserve original stored
freshness/consistency/coverage and timestamps; saved analysis is not a new live
verification. No change to the engine's existing freshness/availability semantics.

### Validation and arithmetic authority

Schema performs strict shape and contextual/evidence validation, including query
arity/dimensions; unchanged canonical Snapshot validation; result-space/unit/kind/
details agreement; valid referenced input evidence; unknown-without-value; Finding
identity and measured/status/reason consistency with its declared measurement and
Expectation. Known results need evidence; unknown may have none when evaluation
stopped before reaching a property. Arbitrary result evidence absent from eligible
input records rejects. These checks do not prove real-world observation or arithmetic.

G01 remains the sole arithmetic owner. Add `measure_query(snapshot,query,context)`
and factor the existing `measure(snapshot,expectation,context)` through a faithful
query extraction. Existing expected/comparison/tolerance/expected_from never enter
a fabricated measurement requirement. Existing caller-supplied applies_when remains
an explicit query condition. `check` retains the actual Expectation and comparison.
Move Measurement/MeasurementResult/Details/UnknownReason ownership into analysis
and re-export engine names; migrate internal enum construction/patterns as needed.
Keep the borrowed engine EvaluationContext facade over the canonical owned input.

Add engine-owned `verify_analysis_result` for untrusted/imported result artifacts:
validate the contract, rerun the exact query/check using immutable source data and
EvaluationInput, and compare quantity, full Space, details, all evidence, unknown
reason and Finding semantic fields. A caller-assigned Finding.id remains allowed;
expectation_id/snapshot_id/status/measured/observation_id/reason must agree. No
schema->engine dependency or arithmetic copied into schema. Numeric verification
uses the same deterministic f64 result/serialization contract, not a new epsilon;
round-trip edge vectors must pass before claiming support. A serialization-fidelity
failure is a concrete serialization dependency, not permission to add tolerance.

For0.2 GeometryCheckCase, do not call the old FindingCase common-source-space gate
as a substitute for evaluation. It stays unchanged for0.1. The new bundle carries
selected space/full measurement/context; structural identity/status checks can
reuse nongeometric helpers without weakening accepted0.1 repairs. Finding keeps
its existing single primary observation (first measurement evidence, or none);
all contributing observations remain in measurement evidence and the source.
Known Finding pass/fail follows the existing explicit comparison/tolerance;
unknown Finding has no measured value and the matching reason. Successful
schema validation alone must never be reported as replay-verified arithmetic.
CLI-generated output comes from actual G01 evaluation; imported/cached external
analysis results require engine verification before reuse as a computed result.
A tampered amount with internally consistent status can remain contract-valid;
it MUST fail recomputation. Tests distinguish those two acceptance layers explicitly.

### Public CLI result and compatibility

After implementation, these local commands become real supported paths:

```text
uiblueprint measure --snapshot S.json --query Q.json --space SPACE_ID --max-input-bytes N --max-output-bytes N [--evaluation E.json] [--json]
uiblueprint measure --snapshot S.json --expectation X.json --space SPACE_ID --max-input-bytes N --max-output-bytes N [--evaluation E.json] [--json]
uiblueprint check --snapshot S.json --expectation X.json --space SPACE_ID --max-input-bytes N --max-output-bytes N [--evaluation E.json] [--json --result-version 0.2.0]
```

S/X remain core0.1 Snapshot/Expectation Documents. Q/E are analysis0.2
geometry_query/evaluation_input documents. Measure requires exactly one of query
or expectation; check requires expectation and rejects query. The compatibility
measure form extracts the query from the actual supplied expectation without
inventing expected values/tolerance. Pure query measurement needs no Expectation.
When E is omitted, construct only snapshot binding and selected full Space, with
empty extra transforms and no observed conditions. `--space` stays required and
must agree with E's result_space; resolve it against existing spaces plus supplied
transform endpoints, rejecting unknown/ambiguous IDs rather than guessing.
Input budget covers the SUM of all explicitly named files. Output budget includes
the final newline; no reference loading, new files, live collection or partial
stdout on bounded validation/encoding failure. Compact output preserves details,
conditions/provenance and unknown reasons; query mode invents no expectation text.

Measure --json emits analysis0.2 MeasurementCase (previously unsupported).
For check, existing --json WITHOUT result-version keeps core0.1 FindingCase and
its existing representability restrictions. Explicit --result-version 0.2.0 emits
GeometryCheckCase for same-space AND transformed/conditional cases. Explicit0.1.0
is equivalent to the old check mode. Result-version requires --json; measure only
supports0.2.0. Unsupported versions/representations return exit5, bounded
`unsupported_result_version`, rather than silently downgrade or erase context.
Legacy0.1 cannot carry nonempty extra transforms/conditions or a converted result
Space; that finite mode refuses those requests. An explicitly empty E that only
restates the source binding and same result Space remains representable. Compact and explicit0.2 do not retain
the resolved consumer_contract_gap merely because the old envelope cannot encode it.
Exits stay0 known/pass,3 measured fail,4 unknown,2 invalid/limits,1 IO/internal,
5 unsupported finite mode. No pass/fail is added to factual measurement.

| Current consumer | Required protection / migration |
| --- | --- |
|0.1 fixtures, independent oracle and generated schema | Preserve all existing bytes/expectations; new cases live under fixtures/analysis |
| Web/Native D02 bridge and plugin negotiation | Continue strict0.1 Context/Document/supported_versions; never advertise0.2 analysis as transport support |
| Existing schema validator/build paths | Same crate/binary/flags/exits; add exact two-version file validation only; old Document API remains strict |
| G01/CLI | One canonical result owner; actual query/evaluation and output wiring in one coupled migration; preserve old check JSON by explicit mode rules above |
| E01 exporter | Re-exported result types; replace its internal placeholder Expectation used only for width/height measure with GeometryQuery, retaining identical dimensions/evidence/statuses; package_version and DrawingBrief unchanged |
| K01 Snapshot store | Keep Context.schema_version0.1, keys/grants/slots and accounting untouched; analysis results are not silently stored as Snapshots |
| Canonical owned_size | Add exhaustive accounting for new analysis types; existing Snapshot byte accounting must remain unchanged |

### Registration, source order and independent cases

Before code, root pins D03@2's exact dual-format/minor boundary and registers a
small UIB.ANALYSIS@1 leaf in the product/spec/decision routes. It restates these
record/version/validation obligations under delegated representation authority;
GEOMETRY/EXCHANGE/MODEL/PRIVACY meaning and the accepted five fixes stay unchanged.
This document is the implementation handoff, not an Active spec edit by itself.

1. Schema packet: new analysis module/validation; lib export; crate-local reuse of
   model record macros only if necessary (no existing DTO change); owned_size;
   generate_analysis_schema example/new0.2 schema; validator binary dispatch;
   new fixtures/analysis manifest and schema analysis-contract/CLI tests. Keep
   existing generate_schema/0.1 model/schema/version APIs unchanged.
2. Engine packet: lib/re-export and new analysis helper; arithmetic/resolve only
   query/context factoring and canonical result constructors; geometry regression
   and analysis-result verification tests. No cache/replay algorithm changes.
3. CLI packet: arguments/input/main/output for query/evaluation/version paths and
   bounded0.2 serialization; binary tests and docs/development/cli.md. Export owner
   adapts observed.rs to factual query/result types and verifies unchanged packages;
   no E02 arithmetic changes, DrawingBrief version or fixture rewrite.
4. Integration barrier: exact stable source hashes, generated-schema parity and
   affected schema/engine/CLI/export/plugin builds/lints/tests. Run existing0.1
   golden/bridge protocol tests; no live UI rerun solely for unchanged transport.
   Core's active Store source waits only for the shared API barrier, not a migration
   of its0.1 data. Path-limited checkpoints/push precede independent acceptance.

Exact candidate write sets for those separate packets (no permission to edit them
in this documentation task):

| Owner | Paths |
| --- | --- |
| Spec registration | `docs/specs/product/analysis.md`, product/spec route READMEs, `docs/specs/development/decisions/d03-data.md` and decision README |
| Schema | `crates/schema/src/analysis/{mod,types,validation}.rs`, `src/lib.rs`, `src/model.rs` only crate-local record-macro reuse, `src/validation.rs` only reusable helper access/factoring with old behavior preserved, `src/owned_size.rs`, `src/bin/uiblueprint-validate.rs`, `examples/generate_analysis_schema.rs`, `tests/analysis_contract.rs`, `tests/analysis_cli.rs`; new `schemas/uiblueprint-analysis-0.2.0.schema.json`, `fixtures/analysis/**` |
| Engine | `crates/engine/src/{lib,arithmetic,resolve,analysis}.rs`, `tests/geometry.rs`, `tests/analysis.rs`, `docs/development/geometry.md`; no replay/cache source |
| CLI | `crates/cli/src/{arguments,input,output,main}.rs`, `tests/binary.rs`, `tests/analysis_binary.rs`, `docs/development/cli.md` |
| Export adaptation | `crates/export/src/observed.rs`, focused `tests/compiler.rs` only if the caller migration needs assertions, `docs/development/export.md`; no proposal arithmetic or package/brief version change |

Brace paths name concrete files, not permission for sibling-wide edits. Root
assigns one writer to shared files and scoped implementation receipts. Existing
model records, schema version, core Artifact variants and golden/oracle files are
protected even when macro/helper visibility in their owning files is adjusted.

Independent valid/invalid cases must include same-space measurement without an
Expectation; signed gap, known0, area/ratio, empty intersection and ordered gaps;
separate-space same-unit negative; sourced multi-hop/unit/origin/baseline conversion;
unknown/redacted/missing target/unsupported shape without numeric fallback; distinct
contributing observations and unused transforms; supplied/missing/mismatched/unstable
conditions; context/target/surface/revision/evidence mismatch; tampered quantity,
space/details/evidence or check outcome caught by engine verification;0.1/0.2
strict cross-decoder rejection; unchanged126 legacy fixtures/schema; exact input/
output boundaries, output round-trip and sanitized failures. Literal expected
numbers come from existing independent GEO vectors and separately authored cases,
not from serializing the candidate's own answers as expected fixtures.

This delivers local factual measure JSON and explicitly versioned converted check
JSON plus existing-context input support. It does not claim live observation,
per-field supplemental-condition capture, new transform discovery, generic diff,
D05 peak enforcement or full S01/P1/P6 acceptance. No product fork was found;
remaining work is the named spec registration and bounded coupled implementation.
