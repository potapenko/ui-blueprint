# E02 proposal geometry repair candidate

## Authority, basis and write boundary

Finite [E02-repair packet](../packets/E02-repair.md) at `36f2089`, under the
existing PLAN.UIB@1 approval and direct user clarification «Какой ответ от меня
нужен? Уже всё же обсудили уже.» The execution runbook records removal of root's
mistaken authority wait. Independent reviewer remains read-only. Mode Restore;
no specification delta, new feature, delegation, runtime or model invocation.

Reused the fully read E01/E01-cli closure: registry/product route → CLI/EXPORT →
all DRAWING leaves/example and required GEOMETRY/MODEL/PROJECTIONS/PRIVACY and
identity/exchange boundaries; RUST/DEV.RUST@2 and applicable D03/D05/D07 closure.
Read the new packet, [review reconciliation](E02-candidate-review.md), execution
runbook authority delta and changed D05 leaf. D05@1→@2 changes retained-storage
policy; geometry precision/tolerance requirements are unchanged. D05-MEMORY is
not applicable to this arithmetic-only repair. Protected source/schema/engine/
manifest/fixture/product-spec paths are untouched by this owner.

Exact writes: `crates/export/src/proposal.rs`, narrow
`crates/export/src/proposal_arithmetic.rs`, new
`crates/export/tests/proposal_regressions.rs`, appended regression cases in
`crates/cli/tests/export_binary.rs`, and this receipt. Existing source snapshots
and authored fixture values are unchanged. New synthetic regression inputs state
their own independent arithmetic expectations.

## Repair and numerical meaning

E02-R1: vertical anchors follow their declared Space origin. TopLeft keeps
top=y/bottom=y+height; BottomLeft uses top=y+height/bottom=y. Centers retain the
same coordinate formula. Both anchor-to-geometry and anchor-to-anchor space
checks still reject mixed origins. Unequal-height rectangles are tested in both
origins and both dimension directions; the review's correct20 passes and wrong50
fails for BottomLeft A.top→B.bottom.

E02-R2: coordinates retain a base and an edge offset. Subtracting bases/offsets
before combining prevents `(x + width) - x` from erasing fractional width or
sub-ULP width at large x. For the actual finite additions/subtractions, TwoSum
recovers the signed rounding residual. An exact operation leaves a point value;
an inexact operation bounds only the direction toward the exact sum using the
adjacent representable f64 endpoint. Interval propagation and absolute distance
handle the remaining arithmetic; no coordinate-scaled epsilon, decimal truncation,
fixed UI tolerance or uncertainty is introduced. Center halving separately
bounds subnormal rounding. If base subtraction overflows, finite endpoint
arithmetic is tried; a known nonfinite dimension still cannot validate.

Source geometry and dimension values are not rewritten. In particular0.1 remains
0.1 at x0.2 and width1 remains1 at x1e16. Adjacent incorrect values fail when the
underlying arithmetic is exact; values outside a nonexact operation's directional
rounding interval fail. Dimension check_tolerance cannot make wrong geometry
valid. Existing explicit DimensionChain arithmetic_tolerance semantics are
unchanged. Unknown dimensions remain unknown and do not require a finite computed
distance; source/status/approval/unknown fields retain their meaning.

## Checks and shared-input barrier

Preliminary focused loops passed5 proposal tests and2 actual-public-CLI tests.
A sixth proposal case was added for finite-endpoint fallback/unknown overflow;
its next compile hit Integration's in-progress schema repair:
`validation.rs:154` referenced `graph::validate_delta_source` before that function
was present in the working graph module. No export failure was inferred and no
shared source was edited to make this compile. This attempt is not final proof.

Root confirmed the missing shared function is now implemented and selected
Integration as the single runner for the coordinated affected-workspace
check/fmt/Clippy/tests, including all6 proposal cases and2 CLI regressions.
E02 source/tests are frozen; this owner does not duplicate the shared suite.
Integration's [final barrier receipt](S01-review-repair.md#final-coordinated-barrier)
reports all four commands passed once on Rust1.96.0:
`cargo +1.96.0 check --locked --workspace --all-targets`,
`cargo +1.96.0 fmt --all -- --check`,
`cargo +1.96.0 clippy --locked --workspace --all-targets -- -D warnings`, and
`cargo +1.96.0 test --locked --workspace`.

Runner inspection HEAD was `3a98c32bd2b7afc0acca0204ec3c855b89785007`, with all
three owners' frozen working changes. Total101 tests passed, zero failed/ignored:
schema22, plugin-api14, engine25, export18 (12 compiler +6 proposal regressions),
CLI22 (including2 E02 actual-command regressions). This is delegated runner
proof; E02 did not repeat the workspace suite or claim independent acceptance.

Exact211-file input-set hash before/after, equal:
`db7792faeea43667960657a53560c90fd47aa4f947cf1b62daec896dc5d35afe`.
Definition: sorted compact JSON path→file-SHA256 map, then SHA256 of that JSON;
root Cargo.toml/Cargo.lock/rust-toolchain.toml plus all regular files recursively
under crates/, fixtures/golden/, fixtures/golden-oracles/, fixtures/export/,
schemas/, tests/bridges/common/, tests/bridges/resources/ and .cargo/ if present.
Added/removed files participate through map membership; target, raw run output
and runtime captures are excluded. Integration confirmed the E02 frozen hash
below matched before running. Final saved-input reconciliation remains root's
responsibility after Integration → Core → E02 scoped checkpoints.

Frozen own set: proposal.rs, proposal_arithmetic.rs, proposal_regressions.rs and
CLI export_binary.rs. SHA256 of the sorted compact JSON path→file-SHA256 map:
`444236409a5905687272c574217a629e731075d2439a35842326022bd0a27162`.
Only this receipt may be updated with the runner's supplied proof before Git.

Root granted the five-path checkpoint after Integration `2828cd8` and Core
`ec0f35c927e9b34fb06313cc0f6c9e4ecaf36f4f` were saved/pushed. Current master,
empty index and unchanged frozen E02 source/test hash were verified before staging.
The terminal receipt returns the exact package SHA/push and full211-file saved-HEAD
reconciliation against the checked hash above, without another suite run.
Same E02 reviewer performs the affected independent recheck after the saved
candidate; author tests do not accept E02/P6/P7 or unrelated shared repairs.
