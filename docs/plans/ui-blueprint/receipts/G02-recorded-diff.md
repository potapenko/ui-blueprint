# G02 borrowed recorded difference slice

Authority: [finite packet](../packets/G02-recorded-diff.md), approved G02/K01/CLI
under PLAN.UIB@1. Selected before implementation: content/value/availability and
Evidence/Observation-only differences are independent; absence is not removal.
Basis reused: CACHE/IDENTITY/EXCHANGE/MODEL/PROJECTIONS/GEOMETRY/LIFECYCLE/PRIVACY
and current D03/analysis closure, RUST/DEV.RUST. No new normative/wire choice.

Exact5 paths: crates/engine/src/diff.rs; crates/engine/src/lib.rs export only;
crates/engine/tests/diff.rs; docs/development/diff.md; this receipt. No edits to
schema/host/CLI/Cargo/fixtures/registry/accepted replay or CacheStore. New files
are inside the already-existing authorized directories; no directories created.

API compare_recorded(&Snapshot,&Snapshot,DiffLimits)->Result<RecordedDiff,DiffError>
reuses validate_snapshot and contexts_compatible. Borrowed node/property entries
carry explicit missing side, content_changed and evidence_changed. Full originals
retain all source context/coverage. Matching is exact SourceKey/Field, not labels
or geometry. Known→Unknown cannot inherit the old value. Even complete coverage
does not manufacture Removal/evidence. Existing geometry Transform Evidence is
separated from its numeric/Space/binding content; no second geometry calculator.
Output cap is explicit; exact omitted count is selection-only. Storage is a fallible
Vec reserved from actual differences, with no cloned graph/unbounded auxiliary map.

Focused independent expected cases passed (5 tests):

- A.width32→48, partial after without B: one content change plus BeforeOnly B;
  original objects are pointer-borrowed and immutable, with partial source intact.
- Known→Unknown has no replacement numeric value; absent versus explicit
  NotRequested property is represented as missing-side difference.
- Same value/new Evidence or same Evidence/new Observation is evidence-only;
  content plus source update reports both dimensions.
- Geometry transform Evidence-only changes do not become numeric/content changes.
- Different namespace yields two membership absences, never merge; environment/
  generation/projection mismatch refuses; invalid duplicate identity refuses;
  zero/small/large caps give exact returned/omitted counts.

Commands: cargo test --locked --offline -p uiblueprint-engine --test diff (5/5);
cargo check --locked --offline -p uiblueprint-engine --lib --test diff;
cargo clippy --locked --offline -p uiblueprint-engine --lib --test diff -- -D warnings.
All passed on existing Rust1.96/aarch64-apple-darwin. Scoped formatting/diff/local
links checked. First run's fixture helper assumed two nodes in single-width source;
corrected test setup to add explicit synthetic B, preserving read-only golden input.
No implementation/expected-value relaxation. Final Clippy was rerun only because
that test input changed. No broad tests/runtime, processes, temporary/output dirs,
extra agents or new framework. Self-review distinguishes property slice from full
graph diff; complete records remain accessible but unexamined graph metadata is not
reported as unchanged. Independent acceptance is not claimed here.

SHA256: diff.rs01f7e2f9c314985def1131ba29bda260f71f322d62c4951b00fea1a4da4f7b62;
lib.rsfd04f09142c64dcff8456f35055458b6613cbf10a9ab612f6a1050570ea20197;
tests/diff.rs2ab35b87ac496c8b4177f55b724fbcd5bd0baccb6acbfeff0d40f02dcd617045.
Saved checkpoint/push reported separately under root Git lease. Required CLI
diff/changes, graph-relationship comparison, justified deletion generation and
live source continuity remain open; no full K01/K02/G02 claim is made.

## G12 explicit selected-space geometry comparison

Root selected the bounded source-backed proposal in this packet's G12 section,
approved PLAN.UIB@1 P3/P6/GEOMETRY/BOUNDARIES geometry requirement. Reused full
CLI@12/CLI-DIFF@2/ANALYSIS@2/TYPES/VALIDATION@1 and explicit GEOMETRY/MODEL/
IDENTITY/EXCHANGE/PRIVACY/BOUNDARIES/CACHE/PROJECTIONS closure, RUST/DEV.RUST.
Registered CLI-GEOMETRY-DIFF@1/CLI@13/registry25 BEFORE source. Additive local
representation, not new canonical0.1/analysis0.2 shape/arithmetic/authority.

Working engine compare_geometry(before,after,key,FrameKind,old EvaluationInput,
new EvaluationInput) borrows original sources/evaluations, reuses exact raw source
compatibility and independently validates each input, requiring full result Space
equality. Existing directional/all-corner rect resolver and Evidence/unstable/
availability/shape rules resolve each side. No ID-only Space equality, implicit
inverse/scale/origin/name/Surface matching. Known states contain numeric Rect and
contributing Evidence; unknown states precise reason/reached Evidence. Signed
finite dx/dy/dwidth/dheight only when BOTH known; no unknown→zero. This is factual
chosen-space rect comparison, not global layout/cause/norm/pass-fail or cache delta.

Public `diff --geometry` exact selector/frame-kind/space and optional per-side
canonical evaluations. Extracted existing measure evaluation builder unchanged;
no mandatory new file when Snapshot facts suffice. Aggregate bound includes all
source/evaluation files and selector bytes. Distinct geometry_difference JSON1.0.0
retains full originals/evaluations/result_space/side states and displacement/null.
Default raw diff JSON/flags remain exact. Compact shows source attribution/coverage,
resolved facts and displacement or unavailable. Known0/unknown4 report, incompatible
source/full Space4 empty, invalid/bounds2, IO/internal1, unsupported finite mode5.

Exact14 writes: engine src/diff.rs, tests/diff.rs;
CLI src/arguments.rs, input.rs, main.rs, output.rs, tests/geometry_diff_binary.rs;
specs/product/cli-geometry-diff.md, cli.md, README.md, specs/README.md;
development/diff.md, cli.md; this receipt. No resolver math/schema/generated types/
cache/collector/Native/actions/Cargo/root coordination changes.

Focused locked/offline Rust1.96/macOS checks passed:
- engine diff8 tests, including sourced window screen100→local0 motion, sourced
  viewport scroll-50→document0, resize width30→48 delta18, missing mapping unknown,
  wrong input/Surface/full Space despite same ID refusal, original data/Evidence
  and raw environment-only comparison/cache strictness preserved.
- public geometry_diff_binary2 cases: discovery without evaluations, optional
  evaluations produce identical JSON, original files unchanged, raw JSON9 fields
  remains recorded_difference without displacement. Missing map→unknown/null,
  partial source preserved, exact aggregate/output caps/bad binding/conflicting
  raw flags empty stdout errors. Test temp JSON removed/absence-verified.
- relevant raw public diff2 and factual measure1 regression passed; no broad suite.
- affected engine/CLI check+Clippy passed; first len>0 test lint corrected directly
  to is_empty, no runtime/expected-value change. Final format/link checks follow.
All vectors are synthetic independent literals, not real B04 UI measurement.

Immediate actual consumer: Web supplies saved B04 pair once ready. Retain only
already-built CLI/task-temp target for that immediate use; source save must not
wait for another collection/review. No browser/Native/physical action or image
created/deleted. Task owns nonimage target until one consumer, then exact cleanup;
other owners' source/artifacts remain untouched. Short exact14 commit/push lease next.
Final format/link/anchor/diff checks and <=100-line spec ceilings passed.
Source SHA256: engine diff7a6913247d64d08be171df76a40c0b0bfaf9cec6e09ce86752120f062bb1c4e6;
CLI inputb89196cc9e9b485734a96f6dd96c778176e3b87b89a5149b57c9a908155fdadd;
CLI output014a4837372d8c71546e8bd51a47c8a5e447c51a6e43df99e64f5b5d63995ac0.
Immediate-use system-temp CLI:
/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/ui-blueprint-g12.HEZ8oawEh5/target/debug/uiblueprint
SHA2563456553fd63afec294c801fd367109207ce39965e000f8bf79f69c7f1f713761.
Retention owner: this Core task until one B04 pair consumer, then remove only owned
nonimage target after use/absence verification; all images and other owners' state
excluded. No process/physical/index lease held before save. Integration's separate
M05 validator work is outside this slice; final source identity is pinned above,
new shared compatible registration can be reflected mechanically when supplied.
Integration then supplied root-selected EXCHANGE@2/D03@3 M05 composition leaves.
Applied ONLY reserved root/product route changes (registry26), preserving original
historical D03@2 registration evidence; no Integration file in this commit. Its
actual shared validator code input changed after earlier checks, so reran only
geometry_diff_binary2: both passed, no G12 semantic/type/arithmetic change.
Final retained binary SHA is now
1d667712912191c8344001e9c1a404c355aa51ae55af77235f8695f2a6d65274
(same path). Earlier3456553f pin records the prior build, not this final consumer.
