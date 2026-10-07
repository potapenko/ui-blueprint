# L01 compact inspect — author handoff

Authority: [finite packet](../packets/L01-inspect-compact.md), PLAN.UIB@1.
Spec route: registry11 → CLI/PROJECTIONS/FORMS/MODEL/IDENTITY/EXCHANGE/PRIVACY/
GEOMETRY@1 and their current closure; existing ANALYSIS/TYPES/VALIDATION@1,
core0.1 and RUST.md/DEV.RUST@2 remain protected. Restore, no semantic/wire delta.
Root selected canonical SourceKey JSON for --ref before source mutation; no new
selector grammar or BackendRef authority was inferred from an implementation.

Exact7 paths: crates/cli/src/{arguments,input,output,main}.rs,
crates/cli/tests/binary.rs, docs/development/cli.md and this receipt. No schema,
engine, host, Cargo, collector, fixture/oracle or configuration changes.

## Actual path

```text
uiblueprint inspect --snapshot FILE --ref '{"namespace":"web.dom","key":"33"}' --view interaction|design --max-input-bytes 65536 --max-output-bytes 65536
```

Strict SourceKey decoding includes selector bytes in the aggregate input bound.
Canonical Snapshot and observed ChannelResponse are accepted, preserving the
complete source. Existing engine::scope::relation_neighbors selects the exact key
and original incident relations. No name/rectangle matching, synthetic one-node
Snapshot, inferred edge, graph mutation or action ref is introduced.
Existing output::Bounded prepares the full escaped compact result before stdout.
Found partial/unknown node returns0; absent key target_unresolved/4; invalid/limit2,
IO1, unsupported5. --json explicitly refuses with unsupported_result_version/5.

Both presentations retain Context/projection/coverage, recorded Observations,
native role, properties/availability/Evidence, separate Snapshot focus, declarations,
children/component membership and sourced relations. Interaction places semantic
facts first; design places geometry first, retaining each kind/Space/provenance.
They do not upgrade consistency, freshness, unknown/not_requested or AX geometry.
The first line explicitly identifies saved observation/no live revalidation.

## Focused verification

Rust1.96.0/aarch64-apple-darwin, --locked --offline:

- `cargo check -p uiblueprint-cli --bin uiblueprint`: passed.
- `cargo test -p uiblueprint-cli --test binary inspect_`:3 passed.
  Exact selection with duplicate labels; false/empty/unknown/unsupported/redacted/
  not_requested; escaped hostile newline; geometry kinds and view ordering;
  observed channel equality; malformed/missing ref/input; aggregate byte boundary;
  unsupported JSON/view; duplicate flag; overflow with no partial stdout.
- Existing binary cases finite_command_and_argument_contract,
  measure_returns_fact_without_inventing_success_for_failed_expectation and
  compact_is_default_saved_analysis_with_source_and_limits: each passed via --exact.
- `cargo clippy -p uiblueprint-cli --bin uiblueprint --test binary -- -D warnings`:
  passed. Scoped formatting/whitespace and local documentation links checked.

Binary SHA256 `bcbbfca8702925f6747a4e88dd27c570dc8971ba0588048a062e77c15dfa1369`.
Changed source/test SHA256: arguments.rs
7f997d2765ffacdc025524a5a5cc017b2e64b910b25767b28181b601dc7f84d0;
input.rs04f66d6dbb048a7a7b961ea4e76967aa4c9879f68a0516378bea494bb599605e;
output.rs39a3974e96c2130c3aeec4da7b671f2d3c5b619e90cbcd68a4001ae488ee8928;
main.rsa91ae22601d7ba9707a656944ee46b304a023728c39ed88ea1e494304abf8bb2;
binary.rsa667513fa979568b0ed7d73b5262ed4bb868445253113d7668f66328d1e7a5c1.
Actual saved-data calls used the command above, both views, explicit65536 input/
output bounds and10s subprocess timeout. Every call returned0 with empty stderr:

| Input / selector | interaction bytes / output SHA256 | design bytes / output SHA256 |
| --- | --- | --- |
| Web sized-before / web.dom,33 |2869 /751fe3b8865819442a143893135a2d417936c653a7b730224a37939f6727206e|2864 /a5918fb94be68914ed1188a20ef244eaae07e188a4808f62506f45015346b71a|
| Native channel-0 / macos.ax,f02.sample.a |4075 /886ff6072eef5735ed01982ef0e139a45edc0d17827b97abc2be2f2589bca08a|4070 /c069c188016247f694d03a6f735526180ebe8dbe9c4b19c7ace378e4269bfd51|

Web source is W01-guarded-live/e5b9cc4a-9262-4ac8-a508-0636dd750521/sized-before.json,
SHA256 aa19c4702e428b781b99fb10f4ec4602050aa5a5cbed06b3524a5959f1a57e7a.
Native source is M01-H01/68057b99-aebd-4529-ac95-d68ea8d2800a/host/channel-0.json,
SHA2566db1e93bfd8a13ecaf3c0a2c5d7f4963c3e1298a7d0a60e2d631c351f3749f0a.
Both remain under the established application-state development/P2 directory.
Original bytes/hashes were compared before/after and unchanged; no source rewrite.

Web result preserves LayoutBounds [40,480,32,16] css_px, viewport source and
cssom-getBoundingClientRect Evidence; Role/Placeholder/Focused/Enabled remain
not_requested in this geometry-only input. Native result preserves Button,
AccessibilityName="Activate sample", Enabled=true, AccessibilityBounds
[60,178,173.5,48] pt/ax-screen with ax_to_pixels_not_calibrated transform uncertainty;
LayoutBounds/Placeholder/Focused are not_requested. It does not invent missing
Director form fields; future/current richer observations can expose their actual data.

## Ownership and residuals

No new persistent directories, browser, UI, live acquisition or broad suite.
Existing test Case owns system-temp files and removes its directory on Drop;
absence is now asserted. Actual frame runs kept stdout only in process memory,
printed compact proof inline, and created no output files. Retained sources remain
untouched. No independent acceptance is claimed by this author receipt.
Required JSON inspect representation/output is explicitly NOT implemented here;
full interaction/design source qualification and live revalidation/actions remain
outside this compact stored-data increment. Existing measure/check/export routes
and wire versions remain unchanged.
