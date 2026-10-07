# L01 bounded recorded diff CLI

Authority: [finite packet](../packets/L01-recorded-diff.md), approved CLI/G02 and
ROADMAP technical representation. Existing compare_recorded5476c28 is consumed;
no engine/schema/cache behavior changes. Before source edits, registered CLI@4,
CLI-DIFF@1 and registry14, using existing feature template and spec directory.
Full selected CLI/CACHE/IDENTITY/EXCHANGE/MODEL/PROJECTIONS/PRIVACY closure and
RUST/DEV.RUST reused. Original CONTENT/INSPECT/OBSERVE/core0.1/analysis0.2 preserved.

Exact11 paths: crates/cli/src/{arguments,input,output,main}.rs;
crates/cli/tests/binary.rs; docs/development/cli.md; docs/specs/{README.md,
product/README.md,product/cli.md,product/cli-diff.md}; this receipt.
No engine/schema/host/Cargo/collector/fixture edits or new directories.

## Actual path and output

`diff --before FILE --after FILE --max-input-bytes N --max-output-bytes N
--max-entries N [--json]`. Input reader now factors the exact existing Snapshot/
observed ChannelResponse extraction used by inspect. Both files share one byte
budget. Arguments require positive byte limits, nonnegative entries (including0).
Main calls existing compare_recorded with borrowed records; typed context mismatch
maps to4 with no output. Complete report0 is report completion, not UI requirement
success. Truncated report4 retains exact omitted_entries independently of coverage.

Compact presents original availability/properties/Observation and flags. JSON
streams both full borrowed Snapshots and scalar entry metadata through existing
Bounded writer, without cloned Snapshot/unbounded Value/new graph or parser.
Fields exactly match CLI-DIFF@1: envelope1.0.0, saved/not revalidated, narrow scope,
before/after, entries, omissions. Entries reference exact source key/field and
before/after presence, separate content/evidence flags. No deleted/removed/Delta.
Source immutable full Context/coverage/uncertainty/Evidence stays accessible.
Entire result/newline is prepared before stdout; overflow has no partial output.

## Focused author verification

Rust1.96/aarch64-apple-darwin; Cargo --locked --offline:

- check -p uiblueprint-cli --no-default-features --bin uiblueprint: passed.
- test -p uiblueprint-cli --no-default-features --test binary diff_cli_:2 passed.
  Literal32→48 + partial absent synthetic B yields content change and record
  absence. Known→Unknown has no inherited value; Evidence/Observation-only update
  remains distinct. JSON full originals match exactly; observed ChannelResponse
  produces the same report as unchanged Snapshot. Complete/zero/small entry limits,
  exact input/output byte boundaries, context mismatch and sanitized malformed
  input/argument cases covered. Original files checked unchanged.
- Existing exact inspect_json_envelope_keeps_uncertainty_and_exact_output_boundary
  passed after input extraction reuse; finite_command_and_argument_contract passed.
- clippy -p uiblueprint-cli --no-default-features --bin uiblueprint --test binary
  -- -D warnings passed. Scoped format/diff/local links and spec≤100-line checks.

Actual retained W01 e5b9cc4a sized-before/after pair tested without mutation using
the new command/JSON,65536 input/output, max-entries100 and10s subprocess timeout.
It returned4/context_mismatch, empty stdout: their environment revisions differ.
Both original hashes matched before/after (aa19c4702e428b781b99fb10f4ec4602050aa5a5cbed06b3524a5959f1a57e7a;
19aee76822668c6b993a4a11a4fb206beef43c48824a3f36cb39bf0151c555fc).
No restamping or fake compatible live comparison. Output stayed in process memory;
no runtime collection or output directories. Test Case removes only own system-temp
files and verifies directory absence. No broad suite/framework/extra agents.

Binary SHA2563cb4af04891edd93580efd1d050082bac12e5b0791c7908bee400ff0b55b9736.
Source/test SHA256:
arguments.rsb2bebd10156e04aa4fbcbd358c80837abf7c8a0512fb8dd11d462bfb9a038bb4;
input.rs450361ea512541761da883f4e5f94890222f1e6d09d69b9acc1f4d929861ae69;
output.rs2975296b635107d2c3f3d1a3179dab3a9facb58c8369b80f876ed6e372aa5f5f;
main.rse75738b2976e3ef4b095ec5b6d1f9bd2dac64209b23413d63b04a7638f0f2039;
binary test7b29e711f62a20886ca563340752fa1a1cb44eac3449e9214ded93b19d5e7a53.

This delivers the recorded property/membership CLI report only. Full graph diff,
justified deletion/Delta generation, changes stream and source correspondence/live
acceptance remain goal obligations. Author checks are not independent acceptance;
checkpoint/push returned under root Git lease.

## Read-only comparison versus Delta applicability

Root requested this bounded semantic handoff after the selected implementation.
CACHE's exact environment/context compatibility protects applying Delta/replay;
it is not by itself a requirement that recorded comparison reject a resized/font-
changed source. BOUNDARIES.CONTENT explicitly requires observe→change application
→geometric/semantic diff. GEOMETRY.CONTENT preserves text scale/zoom/scroll and
coordinate spaces, invalidating dependent transforms when environment changes.
IDENTITY.CONTENT permits reported-key diff while preserving generation/action
boundaries; PROJECTIONS preserves scope/fields/source identities and unknowns.

Minimum proposed read-only boundary: retain exact session/schema/plugin/Target and
Surface generations plus scope/projection/field set, allow different environment
revision while preserving both original environments and geometry kinds/Spaces/
transforms. Report raw recorded content/Evidence differences only; no coordinate
conversion, implied common transform, normalized geometric delta, pass/fail,
source continuity or replay/deletion applicability follows. Cache contexts_compatible
must remain strict. Engine comparison needs its own narrow applicability condition
and routed diff-contract reconciliation before that separate correction. This
checkpoint3d1c25e3a3d6028f2ef9e933c528106a3356f07e preserved the originally selected
guard. Root/advisor then accepted the bounded correction recorded below.

## Environment applicability reconciliation

Root's amended packet explicitly adopts Core/original Web advisor's conclusion.
Registered CLI-DIFF@2/CLI@5/registry15 BEFORE correction: the prior technical rule
was an overconstraint on BOUNDARIES change→diff and GEOMETRY resize/font flow.
Engine::diff now compares source-binding fields directly, requiring same session,
schema/plugin, Target/Surface generations, scope/projection/field sets while
allowing distinct environment revisions. No cloned/restamped Context or global
schema helper change. CACHE/Delta strict contexts_compatible stays byte-unchanged.
Different units/frame kinds remain original attributed records; no shared transform,
normalization/arithmetic displacement, deletion or action ref is introduced.

Exact correction10 paths: engine/src/diff.rs and tests/diff.rs under crates;
crates/cli/tests/binary.rs; docs/specs/{README.md,product/README.md,product/cli.md,
product/cli-diff.md}; docs/development/{diff,cli}.md; this receipt. No new directory.

Affected Cargo --locked --offline checks passed:
- engine --test diff:6/6, including environment-only acceptance, immutable source
  bytes, separate recorded units/Space and unchanged strict Delta compatibility;
  valid session/Target/Surface generations/plugin/scope/projection/fields mismatches
  all refuse. Existing absence/unknown/evidence/cap behaviors remain covered.
- Exact CLI diff_cli_entry_and_byte_limits_context_and_invalid_input_are_explicit:
  passed; original both Snapshots retained on environment change; target generation
  still gives context_mismatch/4. Entry/byte boundaries remain unchanged.
- Engine/CLI affected lib/bin/diff-test/binary-test Clippy -D warnings passed.
No broad suite or repeated unrelated tests. New behavior is read-only comparison;
the test explicitly proves contexts_compatible still rejects environment mismatch.

Same actual retained Web pair then invoked unchanged in JSON and compact,65536
input/output cap, max-entries100,10s per process: both exit0/empty stderr. JSON5877
bytes SHA256fee648910c6d2b3285fbe50b09a4a2e3c6aba5f58d3f965204671a9ffb37a4e1;
compact3814 bytes SHA2569add5c0c939ecfa6662814698848b943a3b0dae822caf3c9b943f52c995d1250.
One LayoutBounds entry for web.dom:33 reports content_changed=true and
evidence_changed=true; original rects [40,480,32,16]→[40,480,48,24] css_px agree
with authored F01. Full JSON before/after equals original decoded Snapshots.
Both environments f01-800x600-dpr1-sized-before/-after and consistency=unknown
remain exact. Original frame hashes/bytes unchanged; no output files created.
This is reportcomplete0, not a pass/atomic/live/source-correspondence claim.

Corrected binary SHA25608d9b880dcd83d7640857b3f4bd2d8d7f1be2b40d706c515b3055c6ceb4f4cba.
Source hashes: diff.rs278f5fc9f232cef3d4e229647d826a614d25f64e82252f1ff4d0dcaaa6580444;
engine diff test03450abd62ba0d036ed944fd7c230db8d191b15ac945e58b18d7d8156e496ada;
CLI binary test85f4e22c4940504faa371832db776eaed304e92371d8e883c3762bd75e35c0da.
Scoped formatting/whitespace/local links and spec≤100-line requirements checked.
