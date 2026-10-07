# W01 collector repair receipt

Status: R1/R2 implementation and focused proof saved by the root-granted checkpoint; not independently accepted. Original source5bce7c6 was rejected by the retained
[reviewer](W01-collector-review.md). Repair authority: [packet](../packets/W01-collector-repair.md)
f499bec, plus explicit R2 accessor amendment a5cc9a4 under approved PLAN.UIB@1.
Consumer: accepted source collector for guarded H01 integration, never a live claim.

## Reused basis and protected scope

Mode Restore, no semantic/spec delta. Reuse full registry10/product/decision/
acceptance closure: D02@2 publication/lifecycle, D03@2, D04@1.CONTENT, D05@3.CONTENT/
MEMORY@2/WORK@1, D06@1/D07@5/D01/RUST/DEV.RUST@2 and complete identity/model/
privacy/geometry/projection/forms/actions/cache/golden/pilot/reuse dependencies from
[original packet](../packets/W01-collector.md). Repair/review/amendment read fully.
Contract requirements govern continuity and acquisition; old source/checks do not. No bootstrap, browser/UI, mobile/export, dependencies or host/Native changes.

Original collector/API and evidence remain in history5bce7c6 (24 Rust/11 mock cases,
fingerprint834b7fe6). Its acceptance status is reject; these old passes do not close
R1/R2. Original R01/S01/F01 context and source supplement are reused, not rerun. Primary source fingerprint da538b9cf02030ffc0a38911c958bd2162671437c6b8141e1419d53b9d032a3e
remains unchanged; additionally inspected pinned inspector_dom_agent.cc::resolveNode:
backend lookup/ResolveNode does not establish frontend parent bindings. Therefore
missing removal events cannot prove selected-node connectivity.

## R1 — original-node publication check

The collector retains bounded original document/node remote handles for the request.
After all DOM/AX reads, final target/frame/loader/document checks precede one isolated
verifyNodes call over those ORIGINAL handles. It checks Element/isConnected and exact
ownerDocument, with no field/name/layout reread or locator/re-resolve fallback.
False/malformed/exception refuses publication; group release follows the check.
Method admission reserves the added verification call and cleanup. Empty selection
requires no node check. Original observation value intervals/last_verified are not
restamped; consistency remains unknown. This is a finite pre-publication check,
not atomic capture or an actionable freshness guarantee after publication.
Regression removes node11 silently while later node12 is read. The final checker
receives original node-11/node-12 handles, returns false, and StaleTarget occurs with
zero callbacks and confirmed group cleanup; no additional resolve/read repairs it.
Compatible positives verify before publication. Mock JS checks late disconnect,
replacement/foreign document and no additional value/rect getter reads.
Final document mismatch now occurs before cleanup; invalidation prohibits another
RPC, so it reports Unconfirmed and closes the owned connection instead of claiming
release. The previous test's post-release sequence was updated to assert this outcome.

## R2 — actual codec admission before dispatch

Root-authorized shared edits are ONLY:

- plugins/web/src/transport/mod.rs: Transport::limits(&self) -> transport::Limits,
  a copy of actual immutable codec configuration, not a liveness assertion.
- plugins/web/src/cdp/session.rs: Client::transport_limits(&self) -> Option<transport::Limits>,
  forwarding the owned config; detached/cancelled states return None, never defaults.

Collector attach refuses frame OR message caps above max_reply_bytes BEFORE first
prepare/command. Each send also refuses when either fixed codec cap exceeds remaining
reply allowance. This covers read-ahead without silently raising caller bounds.
Wire IO/work limits stay separate and include framing/control; no guessed header
allowance is substituted for codec payload bounds. Existing post-receive checks
remain defense in depth, not admission. Transport/CDP IO/state/log behavior unchanged.

Tests set caller1024 below IO16384/CDP8192 and independently mismatch frame/message
caps: zero CDP commands. A compatible1024/1024 codec still completes normalization.
Getter-copy mutation cannot change configuration; detach/cancel return None.
The canonical-output-cap case uses a compatible1024 codec so it still exercises serialization refusal after cleanup; smaller incompatible request allowances refuse
before acquisition. No caller cap or quality expectation was weakened.

## Exact affected proof and inputs

28 collector Rust tests pass; 11 existing mock-reader +3 continuity JS scenarios pass.
Final locked/offline Clippy compiles all Web targets with -D warnings; fmt passes.
Commands: cargo +1.96.0 test --locked --offline -p uiblueprint-web --test collector;
cargo +1.96.0 clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings;
cargo +1.96.0 fmt -p uiblueprint-web -- --check; node --check collector/verify-nodes.js
(Web src prefix); node plugins/web/tests/fixtures/collector/script-check.cjs.
All Cargo runs use the existing own-temp/build, not the shared workspace target.
No unchanged transport/CDP/core/full-workspace runtime suites or live browser tests.
A new JS strict/rest-parameter syntax error was caught by mock execution and fixed
using a simple parameter list plus bounded arguments loop. Test-helper Clippy
let-and-return was repaired; final checks pass without suppressions.

Repair input fingerprint `4223d94d65a5a51c958ca291263d0062115c578349f240b5c5f063268c92f37d`:
SHA256 ordered path+NUL+bytes: rust-toolchain.toml, Cargo.toml, Cargo.lock,
plugins/web/Cargo.toml, src/lib.rs, src/transport/mod.rs, src/cdp/session.rs;
sorted src/collector/* then src/normalize/*; tests/collector.rs; sorted
tests/fixtures/collector/* (src/tests are Web-relative; other paths literal). Docs excluded.
Manifest299c8a44d63290ecf126fda4c0bfc6170883a2c5164d0657b1f198c0448758f0;
lock af468c06f884e8fa07996bdcbc39785d0c005004fb59f1a9f7e74a40cb6c8623;
Web manifest cae7689cc6e75e706cbb675629ea57523b7504cdbd90a5defb65410eac0e1273 remain stable.
Schema tree e97dc2f16fd8fe4b0634edba1c88ed76671402e2 and non-getter shared behavior protected.

## Checkpoint and residuals

11 own paths: the two getter files; collector/{acquire,io,observe,wire}.rs and new
verify-nodes.js; tests/collector.rs; tests/fixtures/collector/script-check.cjs;
web-collector.md handoff and this receipt. Root granted this exact11-path checkpoint;
commit/push result returns to root before the same reviewer's recheck. No self-acceptance.
Initial refs, root-frame/flat scope, one ExternalSemantics DOM+AX completion, opaque
browser work, H01/allocator/ACK integration, live invariance/cleanup and B01–B06/D06
remain open as documented in the [handoff](../../../development/web-collector.md).
Own retained temp /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-collector-cfh50npl:
source+scoped builds, cleanup by this worker after root acceptance. No new raw logs.
