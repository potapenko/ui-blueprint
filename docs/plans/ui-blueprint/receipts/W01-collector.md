# W01 collector source-stage receipt

Status: callable source and offline proof ready; root granted this exact checkpoint; independent review pending. Packet47b2a95 under approved PLAN.UIB@1. Consumer: real scoped observe
inside H01, then immutable inspect/measure through existing engine. No live grant.
Transport4106e04, CDP2f9ea77/review1be4ec0, schema/Core/host/Cargo remain protected.

## Basis, authority and actual boundary

Mode Restore; no product/spec delta. AGENTS → registry10 → product/decisions/
acceptance → D02@2/D03@2/D04@1/D05@3/MEMORY@2/WORK@1/D06@1/D07@5/D01/RUST/
DEV.RUST@2 plus MODEL/EXCHANGE/IDENTITY/PROJECTIONS/GEOMETRY/PRIVACY/LIFECYCLE/
BOUNDARIES/FORMS/ACTIONS/CACHE/GOLDEN/ROADMAP/PERFORMANCE/PILOTS/WEB-PILOTS/
REUSE/RUST-BOUNDARIES/C01-EVIDENCE closure. Current applicable CDP basis reused;
missing normalization/source/pilot branches/leaves read. No semantic revision drift.
Excluded drawing/mobile/real apps, host implementation and a broad upstream survey.

Contract: exact refs/document context, explicit unknown/redaction/coverage, bounded
acquisition and real completed-channel ownership. Supporting observation: R01/S01-Web
addressed DOM/CSSOM and partial AX mechanisms, F01 literals; neither accepts this code.
Implementation choices: canonical BackendRef inputs; isolated fixed function with
serialized data and exact document handle; root-frame/loader/document checks; typed
method DTOs; Rust normalization and owned canonical Document callback. No second graph,
JSON parser/framework, new dependency, fake host Ticket or guessed accessible name.

The concrete shared-channel question was returned before API finalization: schema
DOM/AX namespaces share ExternalSemantics; two same-channel replies would violate
DuplicateChannel. Current API publishes that channel once; valid AX method failure
preserves DOM with partial coverage. Root acknowledged/forwarded the compiling
callback shape to Core. No shared API edit. Parent-ACKed DOM before AX is NOT claimed; that stronger boundary would need separate admitted work or a shared contract.
[API handoff](../../../development/web-collector.md) gives signatures, fields,
clock/cancel/error/cleanup semantics, initial-ref bootstrap and live integration gaps.

## Narrow primary-source supplement

Reuse devtools-protocol revision d209a9a38897d2935a078a0bf00ca821811d21ed:
[pdl/domains](https://github.com/ChromeDevTools/devtools-protocol/tree/d209a9a38897d2935a078a0bf00ca821811d21ed/pdl/domains)
DOM/Page/Accessibility/Target, and [js_protocol.pdl](https://raw.githubusercontent.com/ChromeDevTools/devtools-protocol/d209a9a38897d2935a078a0bf00ca821811d21ed/pdl/js_protocol.pdl).
Read selected method/type blocks: document/resolve, isolated world/context/group,
current target/frame metadata, addressed AX/AXValue types, callFunctionOn and release.
Runtime is in js_protocol.pdl here; an initial domains/Runtime.pdl404 was corrected
through a bounded directory lookup. No upstream tests or browser APIs executed.

Also read getDocument/BuildObjectForNode/BuildArrayForContainerChildren in Chromium145
[inspector_dom_agent.cc](https://raw.githubusercontent.com/chromium/chromium/47e20adcc15fc15f01825aa17e570c8f5492ac0f/third_party/blink/renderer/core/inspector/inspector_dom_agent.cc), revision47e20adcc15fc15f01825aa17e570c8f5492ac0f.
Depth0 is passed through; Document avoids element shadow/pseudo forced-child rules.
Root child counts/adopted sheets and frame metadata still have opaque browser cost;
this is not universal CPU/heap qualification. No source copied.
Six-source fingerprint `da538b9cf02030ffc0a38911c958bd2162671437c6b8141e1419d53b9d032a3e`:
SHA256 over sorted basename+NUL+bytes for the five PDLs and inspector_dom_agent.cc.

## Checked inputs and actual proof

Collector input SHA256 `834b7fe6920791fd90cfc1bc1a6a25cb758ed18ac02cebf5fd91f54a9fd1eda5`:
ordered path+NUL+bytes for rust-toolchain.toml, Cargo.toml, Cargo.lock,
plugins/web/Cargo.toml, src/lib.rs; sorted collector/* then normalize/*; tests/collector.rs;
sorted tests/fixtures/collector/* (Web-relative paths except first three). Docs excluded.
Schema tree at3abd8e5 `e97dc2f16fd8fe4b0634edba1c88ed76671402e2` remained unchanged.
Manifest SHA256 `299c8a44d63290ecf126fda4c0bfc6170883a2c5164d0657b1f198c0448758f0`;
lock `af468c06f884e8fa07996bdcbc39785d0c005004fb59f1a9f7e74a40cb6c8623`;
Web manifest `cae7689cc6e75e706cbb675629ea57523b7504cdbd90a5defb65410eac0e1273`.

Rust1.96.0/ac68faa20, aarch64-apple-darwin; Node24.15.0. Final checks passed:

```sh
cargo +1.96.0 check --locked --offline -p uiblueprint-web --all-targets
cargo +1.96.0 test --locked --offline -p uiblueprint-web --test collector
cargo +1.96.0 clippy --locked --offline -p uiblueprint-web --all-targets -- -D warnings
cargo +1.96.0 fmt -p uiblueprint-web -- --check
node --check plugins/web/src/collector/read-node.js
node plugins/web/tests/fixtures/collector/script-check.cjs
```

24 Rust owned-peer tests +11 mock-getter script scenarios pass. Final Cargo runs use
CARGO_TARGET_DIR=<own-temp>/build with180s per-command bounds. Early checks used the
existing ignored workspace target; no shared artifacts deleted. No command logs
persisted. One test initially compared JSON integer/f64 representations; corrected
to exact canonical Rect equality, without tolerance change. Clippy's8-argument
observation builder was changed to an interval pair; subsequent checks pass.

| Proof | Executed evidence |
| --- | --- |
| Identity | Wrong actual target stops before frame read; session/target/surface/source ref refusal before IO; loader/root replacement, disconnected/foreign document, post-read navigation, event loss; no fallback/recollection |
| Canonical data | Independent literal DOM/AX inputs/expectations, same labels/distinct keys, real backend relation, rect units/origin/local-only, false/empty, typed numeric/bool/mixed states, canonical Document roundtrip/validator, shared worker clock origin and distinct snapshot IDs |
| Bounds/privacy | Node/method/request-wire and cumulative canonical-output caps; fields gate AX/string reads; empty scope distinguished; malformed records, wrong AX backend/frame; caller/password/OTP redaction, canaries absent from output/logs/errors |
| Lifecycle | AX protocol failure preserves DOM; first owned canonical callback survives later cancel/output failure; late ACK/deadline refusal, mid-AX cancel stops methods; acknowledged group cleanup vs explicit Unconfirmed; owned socket/thread cleanup |

Test caps are explicit synthetic parameters, not production/D06 defaults:16 DOM refs,
100 methods,8192-byte reply,65536 cumulative bytes,600 text bytes,256 handle bytes,
32 AX properties,16384 IO bytes/2048 work steps per method; smaller per-case bounds.
Mock script tests cannot establish Chromium side-effect checks, native getter/layout
behavior or live redaction. Unchanged transport/CDP/core/workspace suites not rerun.

## Delivery/retention and open gates

Root granted exactly15 own source/test/fixture/doc paths for this checkpoint.
Commit/push result is returned to root; independent review follows. No self-acceptance. H01 admission/allocator/publication/ACK,
initial current refs, real browser/cleanup/read-only invariance, complete projection/
portal/frame/pixel behavior and B01–B06/D06 stay open; no full W01 claim.
Own temp: /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-collector-cfh50npl
contains inspected source and scoped builds; owner=this worker, cleanup after root accepts this source evidence. No browser, user app, new branch/worktree or agents.
