# D07 — initial dependencies and no-copy disposition

- Domain: `uib.development.d07`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.D07@2`; clause: `UIB.D07.CONTENT`.
- Authority: Active / Stability: Evolving; initial dependency intent, no installation.
- Read when: T01/S01 dependency resolution or later selected code/dependency adoption.
- Do not read when: no dependency/source transfer is proposed.
- Requires: [REUSE@1](../../reference/reuse.md),
  [RUST-BOUNDARIES@1](../../product/rust-boundaries.md), [evidence](evidence.md).
- Owner/deadline: T01/S01 before initial resolution; each adapter owner before
  importing its selected library/code; I01 verifies final distribution notices.

## Requirement and decisions

Use small dependencies for concrete boundaries; analytics remain our Rust code.
R01/R02/R03 source ledgers identify transferable mechanisms and incompatible
assumptions. Decision: **no code copied from those upstreams** in the initial
engine. Reimplement selected identity/update/geometry rules against our contracts.
Use public Apple frameworks only within native helper. Playwright/Node are fixture
tools, not a mandatory product runtime. No model, daemon framework, persistence
engine, async runtime or native SDK dependency in core by default.

Exact initial root workspace dependencies when T01/S01 need serialization:

| Package / version / features | Purpose / evidence |
| --- | --- |
| `serde = =1.0.229`, default std, `derive` | Typed envelope serialization; registry metadata Rust minimum1.56, MIT OR Apache-2.0; [versioned API](https://docs.rs/serde/1.0.229/serde/) |
| `serde_json = =1.0.151`, default std only | JSON parser/writer; registry minimum1.71, MIT OR Apache-2.0; [versioned API](https://docs.rs/serde_json/1.0.151/serde_json/) |
| `schemars = =1.2.2`, std/derive only | S01 schema generation from canonical types; minimum1.74, MIT (Graham Esau2019) |
| dev-only `jsonschema = =0.58.5`, default-features=false | S01 structural validation/parity; minimum1.85, MIT (Dmitry Dygalo2020–2026); HTTP/file/TLS/async resolution disabled |

S01 authorized delta `S01-DEP-001`, revision2: these two dependencies prevent
handwritten schema drift and supply an independent structural checker. Exact crate
archives/manifests/LICENSE and schemars JsonSchema API inspected2026-10-06; no
NOTICE in selected library archives (jsonschema includes its test-suite license).
schemars_derive1.2.2 also MIT/minimum1.74. No source copied. Semantic reference/
generation checks still belong to our validator. A common map-only serde visitor
rejects nested array-shaped records without another JSON parser. jsonschema stays
in tests, avoiding its large transitive runtime graph; generated schemas use local
references only. Its float_roundtrip feature is test-only. No URL/file is loaded
from input documents. Schema/parser parity is independently checked on the goldens.

Crates.io version metadata checked 2026-10-06: both non-yanked stable releases.
serde checksum `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba`;
serde_json checksum `c841b55ecdae098c80dcae9cf767f6f8a0c2cdb3416bbef72181df4d0fe73f14`.
Choose typed parsing rather than untyped `Value` indexing which returns null on
missing/mistyped paths. Semantic validation still required. Do not enable
`unbounded_depth`, arbitrary_precision/preserve_order, rc or unrelated formats.
Stable output comes from explicit deterministic representation, not source-map
insertion order. There is no selected serde dependency on Apple/browser SDKs.

T01 resolves and commits Cargo.lock, records transitive versions/features/license
files, then uses `--locked`. Inspect actual resolved package LICENSE/NOTICE before
distribution; metadata is not a blanket audit of all transitive material.
Serde derive proc-macro dependencies are build-time dependencies, not a model or
runtime service. A failed resolution/MSRV/build requires a concrete reconciliation,
not an unnoticed version substitution. No `cargo update` as routine QA.

## Later dependencies and rejected alternatives

Web CDP transport needs an owned bounded socket/protocol library before W01.
`tungstenite 0.30.0` metadata was inspected (minimum1.85, MIT OR Apache-2.0,
handshake feature), but **not adopted**: no scoped transport/timeout/source audit
or shipping use yet. W01 must inspect exact selected API/source/license/NOTICE and
set version/features before adding it; do not write a second WebSocket protocol
implementation or pull an async framework merely to avoid that finite decision.

AccessKit full nodes are a reference, not our fields/provenance schema; Chromium
notice applies to its schema source in addition to AccessKit licenses. Galen and
Extras hidden tolerances/runner are rejected. Compose drawing padding/density and
Preview survivor cache/reflection are rejected as measurement/cache behavior.
AXorcist private identity API and scope expansion are rejected; Peekaboo/agent
stacks and unsafe fallback action policies are not adopted. [Source ledgers](evidence.md)
retain each exact revision, selected paths, tests and conditions for reconsideration.

Acceptance: T01/S01 locked package builds and semantic parser tests; I01 selected
distribution inventory plus applicable licenses/notices; no claims that an
upstream test was run, a license audit was universal, or a dependency was installed
by this decision. Further adoption is bounded D07 work before transfer, not a
blanket authorization for extra platform/model/transport architecture.
