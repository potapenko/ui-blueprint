# W01 CDP session receipt

Status: implementation and 19 author tests ready for independent review.
Root granted the exact ten-path checkpoint; this commit saves that source. Consumer: scoped W01 collector inside proven H01 worker.
Packet50f3b8f with dependency amendment3231928. Protected transport4106e04,
finaldocsb8b8721/review40875c9 remain unchanged. No full W01/live acceptance claim.

## Authority, recovery and choices

Mode Restore; approved PLAN.UIB@1 packet, no specification delta. Traversal:
AGENTS → specs/README registry10 → product/decision routes → D02@2/D03@2/D04@1,
D05@3/MEMORY@2/WORK@1/D07@5, D01/RUST/DEV.RUST@2 and the packet's explicit closure:
BOUNDARIES/EXCHANGE/IDENTITY/LIFECYCLE/PRIVACY/MODEL/GEOMETRY/PROJECTIONS/FORMS/
ACTIONS/CACHE/GOLDEN/ROADMAP/PERFORMANCE/REUSE/RUST-BOUNDARIES/C01-EVIDENCE.
R01/F01/S01 and accepted transport context reused; recovery reread current D02/D04/
D05 family/D07, identity/lifecycle and applicable global governance. D07@5/registry10
host-only libc change reconciled without importing host scope. Unaffected analysis/
CLI/native implementation and unrelated source surveys excluded. No missing product
choice or conflicting authority discovered; method/host/live acceptance stays external.

Contract requirements: exact correlation/generations, no late revival or repeated
uncertain send, real bounded ownership, truthful privacy/deadlines and event loss.
Implementation choices within packet: sequential exclusive Pending; non-reused
process IDs1..=i32::MAX plus private connection epoch; shallow serde visitors;
opaque wire moved to result/event; finite shared ownership permits and fixed queue.
These are private library choices, not canonical wire/product defaults.

Primary-source supplement: Chromium145 revision
47e20adcc15fc15f01825aa17e570c8f5492ac0f,
[dispatch.h](https://raw.githubusercontent.com/chromium/chromium/47e20adcc15fc15f01825aa17e570c8f5492ac0f/third_party/inspector_protocol/crdtp/dispatch.h) and
[dispatch.cc](https://raw.githubusercontent.com/chromium/chromium/47e20adcc15fc15f01825aa17e570c8f5492ac0f/third_party/inspector_protocol/crdtp/dispatch.cc).
CallId is int32; duplicate command ID refuses; replies serialize ID plus result/error.
Notification serializer emits method plus params object, including `{}` when its
optional params pointer is absent. This corrects the earlier ambiguous receipt wording
“optional params”: the observed serialized field is present. Client requires object
params/results and exact non-coerced ID subset. Source inspection, not upstream tests
or live browser qualification; no copied code/framework/parser.
Source SHA256: h `5b73835110f48dc841c99c2d5155b9cae89ba838caeb89139836506de14c9e75`;
cc `3609220caa3a1699568fff717c28a2a6fa66bc6d6fc162dba35436609f03a3b2`.

## Dependency and input handoff

Core added exactly serde.workspace=true, serde_json.workspace=true and
uiblueprint-schema path ../../crates/schema. No plugin-api/raw_value/new library,
version or feature change. Core saved manifest/lock; this worker did not edit them.
Stable before/after checks, including existing runtime float_roundtrip:

- Cargo.toml SHA256 `299c8a44d63290ecf126fda4c0bfc6170883a2c5164d0657b1f198c0448758f0`.
- Cargo.lock SHA256 `af468c06f884e8fa07996bdcbc39785d0c005004fb59f1a9f7e74a40cb6c8623`.
- plugins/web/Cargo.toml SHA256 `cae7689cc6e75e706cbb675629ea57523b7504cdbd90a5defb65410eac0e1273`.
- CDP input fingerprint `e11ea502829a8920266b5d0f9be2e9cbf924686125537bdf0237f701300db96c`.

Fingerprint: SHA256 over ordered path+NUL+bytes: root Cargo.toml, Cargo.lock,
plugins/web/Cargo.toml, src/lib.rs, sorted src/cdp/*.rs, then tests/cdp.rs under
plugins/web. Docs excluded. Protected transport source/tests equal4106e04 exactly.

## Actual scoped verification

Rust1.96.0/aarch64-apple-darwin; all following scopes passed:

```sh
cargo +1.96.0 check --locked --offline -p uiblueprint-web --all-targets
cargo +1.96.0 clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings
cargo +1.96.0 test --locked --offline -p uiblueprint-web --lib cdp::
cargo +1.96.0 test --locked --offline -p uiblueprint-web --test cdp
cargo +1.96.0 fmt -p uiblueprint-web -- --check
```

Initial check passed; final Clippy compiles all affected targets. One initial
collapsible_if lint was repaired, then Clippy/tests/fmt passed. Seven unit/IO-fake
plus12 loopback tests pass; no unchanged transport/core/whole-workspace suite rerun.

| Packet proof | Executed evidence |
| --- | --- |
| Correlation | Ordinary reply/event, exact raw frame, wrong/missing session/ID, duplicate reply against next ticket, cancelled/inactive ID and old epoch, separate connection/generation |
| Ownership | Caller-held reply/event reservations, result quota refusal before dispatch, actual drop release, completion survives detach/drop/later failure, bounded queue/byte loss |
| IO/lifecycle | Pending flush fake: one send/three resumes; uncertain failure no retry/read; cancel/drop before send, cancel blocked wait, delayed-dispatch original deadline, timeout closes, peer close, independent connection progress |
| Decoder/admission | Duplicate/ambiguous/unknown/missing/mistyped envelope, null params/results, lossy/overflow IDs, bounded writer, capacity not length, invalid binding/limits, counter/quota/loss overflow |
| Privacy/resources | Raw payload/error/session/method canaries absent from Debug/Failure/logs; finite owned peers and joined cancellation thread, socket close after invalid construction/cancel/timeout |

Test limits are explicit fixture parameters:2048-byte message,1024 request,128 metadata,
1–2 result/event slots,16384 operation IO bytes/1024 work steps; deadlines10/50/700/
1200ms by case. Not production defaults or D06 performance samples. Finite loopback
and private IO fakes only; no browser/UI, automatic resync, subscription or polling.

## Delivery and residuals

[API handoff](../../../development/web-cdp.md) records usage, raw-data ownership and
limits. Root granted the exact ten source/test/doc paths after reading this receipt.
Commit/push result is returned to root; independent review follows. No self-acceptance.
Total allocations/RSS, guarded host publication/ACK, action permits and complete
method-decoder/collector/live browser identity/coverage/D06 remain separate gates.
Own temp `/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-cdp-session-dmup4k5o`
contains only the two inspected upstream files; remove after source evidence acceptance.
No command logs/evidence stored in repo or config; no other owners' files touched.
