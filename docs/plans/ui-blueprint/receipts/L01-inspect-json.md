# L01 inspect JSON — author handoff

Authority: [finite packet](../packets/L01-inspect-json.md), PLAN.UIB@1/ROADMAP
technical representation selection. Existing CLI.CONTENT requires JSON alongside
compact; compact507383d was observed to lack its own machine-readable envelope.
Before code, registered CLI@2 INSPECT and registry12 in existing CLI/product/root
routes. Original CONTENT preserved; core0.1/analysis0.2 untouched. Full selected
CLI/PROJECTIONS/FORMS/MODEL/EXCHANGE/IDENTITY/PRIVACY closure and RUST/DEV.RUST
reused; only local output representation changes, not source product semantics.

Exact9 paths: docs/specs/product/{cli,README}.md, docs/specs/README.md;
crates/cli/src/{arguments,output,main}.rs; crates/cli/tests/binary.rs;
docs/development/cli.md and this receipt. input.rs remains unchanged. No engine,
schema, host, Cargo, fixture/oracle, Web/Native or new directory changes.

## Implemented output

Existing inspect arguments now accept one --json flag; duplicate rejects2.
Unchanged strict SourceKey/view/aggregate byte accounting, input validation and
exact node selection precede output. JSON streams canonical borrowed fields
through output::Bounded: no Snapshot clone, unbounded Value, new wire artifact,
generic DTO/schema framework or import parser. The existing complete-only stdout
path and compact formatting remain; prepublication failure has empty stdout.

One object plus newline has exactly7 fields:
output_version="1.0.0", kind="inspection", source="saved",
live_revalidated=false, selector (SourceKey), requested_view (Projection),
snapshot (full unchanged canonical Snapshot). Embedded Context version stays0.1.0.
No action/ref generation, new freshness, inferred property or computed-result claim.
CLI@2 versions this envelope separately; product does not import it.

## Actual checks

On existing Rust1.96/aarch64-apple-darwin environment:

- `cargo test --locked --offline -p uiblueprint-cli --test binary inspect_`:
  4 passed,0 failed. Snapshot and observed ChannelResponse preserve complete
  canonical source; envelope fields/selector/view exact; unknown consistency,
  redacted/false/empty/unknown/unsupported values survive. Compact tests still pass.
  Exact output byte limit including newline succeeds; one byte less refuses2 with
  no partial stdout. Missing ref4, malformed selector2 and duplicate JSON2 checked.
- `cargo clippy --locked --offline -p uiblueprint-cli --bin uiblueprint
  --test binary -- -D warnings`: passed.
- Scoped format/diff/local links; selected specification files≤100 lines.

No unchanged full suite or extra runtime acquisition. Original retained actual
Web and Native frames used by [compact proof](L01-inspect-compact.md) were read
directly with both requested views and --json,65536 input/output bounds,10s timeout.
Output stayed in memory; every complete parsed envelope equalled the expected7
fields with the original full Snapshot, including all Context/Evidence/coverage.
Input bytes/hashes remained unchanged. All4 calls exit0/empty stderr:

| Input/view | Output bytes | SHA256 |
| --- | ---: | --- |
| Web/interaction |2935|5fd496b9f94eef231c2fbbec3d0a46898d00f7cd313e9aef1acaa3b39e297c51|
| Web/design |2930|203ad3767849b4633d02c2c1298a36ae504efcfd58d9ab5cad6a0fcfb136382a|
| Native/interaction |4388|042ef86ec0311f1c65aeb69171539246382abcd87b33c2c21b7c13fec3f819aa|
| Native/design |4383|2a91b3e2868c46b44e487d084fed7a1ee74bfeddda1bca3d9459f005b3b53ac3|

Example Web selector {namespace:web.dom,key:33}, snapshot web-snapshot:1:2,
layout32×16 css_px with original unknown consistency. Native selector
{namespace:macos.ax,key:f02.sample.a}, original Button/Activate sample/Enabled=true
and AX bounds retained; no fabricated LayoutBounds or focus. Both outputs state
source=saved/live_revalidated=false. Exact original paths/hashes in compact receipt.

Binary SHA2561f2aa5042fcabc985526bcd41f3e454e4c4d2068e1918bef3a4f1e0dbd5b0b4d.
Changed source/test pins:
arguments.rs bb99e1ab00ed80f43a4dd225d200553a86e6f7daa2aa6c83963f15e7d661c17f;
output.rs1e11ee3a0bb8631533e894da7564a07561208ba384519079aac754f372b04236;
main.rse98d60ad7c8be67fc6e3d0673af44aed85243134a3c06dbc451a429230943850;
binary.rsf241e58c2c0d41b8706a6481f887a8512d0e54b08fc916a6aaef06aad3763778.

No new directories/output files were created for actual frame calls. Existing
binary test helper removes only its system-temp directory and asserts absence.
This closes the saved-node inspect JSON output gap. It does not close full G02
projections, live revalidation, platform field qualification or actions. Author
checks are not independent acceptance; checkpoint/push is reported separately.
