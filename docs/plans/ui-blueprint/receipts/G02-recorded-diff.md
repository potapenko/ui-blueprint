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
