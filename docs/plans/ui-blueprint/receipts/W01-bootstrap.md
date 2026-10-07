# W01 first-request bootstrap receipt

Status: functional source/offline candidate ready for checkpoint and review; not accepted/live-qualified. Authority [W01-bootstrap packet](../packets/W01-bootstrap.md)
e11ca17 under approved PLAN.UIB@1, plus root's explicit early API acknowledgement.
Consumer: H01 first scoped Web observe without fabricated prior refs. Accepted collector4a45400/rechecke11ca17 supplies protected R1/R2 behavior.

## Basis, implementation and API

Mode Restore, no spec delta. Reuse current registry10/product/decisions → IDENTITY/
PROJECTIONS/EXCHANGE/MODEL/PRIVACY/LIFECYCLE@1, D02@2/D04@1/D05@3/MEMORY@2/WORK@1,
full original-collector closure, WEB-PILOTS/PILOTS@1, RUST/DEV.RUST@2. Bootstrap
packet/recheck, R01 ledger and exact source methods were read before implementation.
Contract permits explicit bounded locator selection and new source-bound refs; source history/tests do not authorize fake prior identity or stale-ref search repair.
No shared contract, host, Cargo, old fixtures, transport/CDP or normalize owner changed.

Actual public API (collector/bootstrap.rs), compiled and exercised through peers:

```rust,ignore
Collector::observe_initial(&mut self, request: &Request, scope: &InitialScope,
    dispatch_sequence: u64, deadline: Instant,
    publish: impl FnMut(Document) -> Publication) -> Result<BootstrapReport, Failure>
```

InitialScope carries scope_id, Vec<DomId { id: Id, sensitivity: Sensitivity }>, and explicit max_visited_nodes. Request max_depth bounds lookup too. No prior BackendRef, Snapshot or Observation input exists. Root acknowledged this additive partial mode;
[handoff](../../../development/web-collector.md) gives the exact Core composition.

One bounded light-DOM walk compares exact case-sensitive id attributes. All nodes, including document/text nodes, count toward visits. IDs are data to a fixed script;
no CSS/XPath/eval, frame/shadow traversal, descendant snapshot or second graph.
Unique resolution requires a completed scan, never just the first found node.
Missing/ambiguous/incomplete/detected-boundary/local-timeout produce explicit
Selection status and visited count in Failure. Selected open-shadow/frame/template/
slot boundaries refuse; absence is only within this explicitly limited search domain.

The script retains only counts and matched original objects in a flat owned result.
Runtime.getProperties inspects that null-prototype container, ownProperties=true,
preview=false. DOM.describeNode(objectId,depth0,pierce=false) obtains actual backend
identity. Original selected objects enter the SAME collector/normalizer/R1 check;
no backend re-resolve substitutes a new same-label node. One method/byte/deadline/
output budget spans selection and all later work. All group objects release together.

BootstrapReport returns ordinary Report, SelectionReport and canonical DOM BackendRefs
with the REAL emitted Snapshot ID, DOM Observation ID, source key and exact context.
They return only after publication ACK/success; error/Stop yields no ref report. The prior existing-ref API is unchanged and never starts a locator fallback. This does not establish mutation authority, atomicity or freshness after return.

## Primary-source decision, no broad survey

Existing R01/S01/F01 mechanisms and retained source supplement reused. At protocol
revision d209a9a38897d2935a078a0bf00ca821811d21ed read Runtime.callFunctionOn,
getProperties, PropertyDescriptor/RemoteObject and DOM.describeNode definitions.
getProperties inherits the target object's group; app properties are never inspected.
At Chromium47e20adcc15fc15f01825aa17e570c8f5492ac0f read describeNode's exact call to
BuildObjectForNode with depth0 and NULL frontend maps; it does not request a frontend
subtree. Attribute/shadow/pseudo metadata and native string/layout work remain opaque
browser cost under response caps, not a CPU/RSS bound. No upstream code copied. Source supplement fingerprint remains da538b9cf02030ffc0a38911c958bd2162671437c6b8141e1419d53b9d032a3e; no downloads, live browser or upstream test runs in this packet.

## Actual focused evidence

37 collector Rust tests pass, including all28 R1/R2 cases; 24 mock-JS scenario groups pass (11 reader,3 continuity,10 bootstrap). Existing canonical Document validation and
independent literal expectations are reused. Checks on Rust1.96.0/Node24.15.0:

```sh
cargo +1.96.0 check --locked --offline -p uiblueprint-web --all-targets
cargo +1.96.0 test --locked --offline -p uiblueprint-web --test collector
cargo +1.96.0 clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings
cargo +1.96.0 fmt -p uiblueprint-web -- --check
node --check plugins/web/src/collector/select-ids.js
node plugins/web/tests/fixtures/collector/script-check.cjs
```

| Proof | Executed case |
| --- | --- |
| First request | No prior refs; two actual selected identities/observations, callback-owned canonical data; returned refs drive later existing-ref request without search |
| Selection | Unique/missing/ambiguous/depth-or-visit-incomplete/unsupported/timeout, exact traversal ceiling, hostile identifier-as-data; selection performs no value/layout reads |
| Continuity | Original-object remount and final document drift; R1 removal barrier on initial path; no selector retry or convenient same-label replacement |
| Admission/privacy | Method budget before lookup, shared cumulative reply budget, malformed/accessor/overshoot result refusal; sensitive values/descriptive attributes absent from canonical data/debug |
| Publication | Stop cannot return refs; canonical identity matches emitted source data and confirmed callback; original R1/R2 cleanup/cancel/output tests retained |

All Cargo artifacts remain in own-temp/build. No unchanged transport/CDP/core or workspace runtime suites repeated. Mock JS/peers do not qualify actual Chromium,
side-effect checks, browser work bounds, runtime cleanup or H01 commit/ACK.

Input fingerprint `93d06a9fbbc44f61e0f7668309ed1939e1f0a263636001f54a80a510cfff7404`:
SHA256 ordered path+NUL+bytes: rust-toolchain.toml, Cargo.toml, Cargo.lock,
plugins/web/Cargo.toml, src/lib.rs, src/transport/mod.rs, src/cdp/session.rs;
sorted src/collector/* then src/normalize/*; tests/collector.rs; sorted
tests/fixtures/collector/* (src/tests Web-relative; other paths literal). Docs excluded.
Root manifest299c8a44…758f0, lockaf468c06…f1a9f7e74a40cb6c8623 and Web manifestcae7689c…0e1273
match accepted4a45400; shared/protected source remains unchanged.

## Delivery and residuals

11 own paths: collector/{acquire,bootstrap,io,mod,observe,wire}.rs and select-ids.js;
tests/collector.rs, fixtures/collector/script-check.cjs, web-collector.md and this receipt.
Await root Git grant, exact-path commit/push and source review. No self-acceptance.
H01 actual composition/ACK, live invariance/cleanup, broader locator/projection/frame/
shadow/pixel modes and B01–B06/D06 remain open; initial-ref API gap alone is addressed.
Retained own source/build temp: /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-collector-cfh50npl;
cleanup by this worker after root accepts this consuming handoff. No new raw logs.
