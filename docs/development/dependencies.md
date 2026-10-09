# Resolved dependency inventory — T01 and S01 Stage A

## I02 final source reconciliation (2026-10-09)

Current candidate `063e709cce4407aed2a2578620b645cf7b554f29` changes only the
shipping Native WindowAX collector relative to `a7c0416`; manifests, lockfile,
toolchain, selected features and third-party material are unchanged. The fresh
combined build rechecks its39-package graph and notice fingerprints against I02's
existing values. Core17/Native18/Web39 inventory and scoped license review remain
applicable; no new audit or licensing policy is introduced. See the
[current I02 receipt](../plans/ui-blueprint/receipts/I02-current-distribution.md#final-current-source-063e709).

## I02 current candidate continuation (2026-10-08)

Candidate `a7c04164df08441cfbbaa61b501aa64d29290732` keeps the same manifests,
Cargo.lock, toolchain and selected features as `94724df`. Native protected input/
popup and Web Documents add no shipping dependencies. Actual Web/combined39 and
Native18 normal/build graphs and notice fingerprints are rechecked; unchanged
core17 evidence is reused after checking its complete production dependency closure.
The historical license review below remains the basis, not a new universal audit.

## I02 qualification (2026-10-08)

Product pin `94724dfd412f966d3d7a90db29aec8be7e35d650` preserves the I01 Cargo.lock,
all Cargo manifests and toolchain pin. The actual selected graph remains
17 core / 18 Native / 39 Web and combined external normal/build crates.
I02 reuses the scoped I01 license review and rechecks generated inventory/notice
fingerprints; it introduces no dependency, license choice or toolchain update.
See [I02 receipt](../plans/ui-blueprint/receipts/I02-current-distribution.md) for
actual build/verification evidence and the distinct product/recipe revisions.

## I01 local distribution inventory (2026-10-08)

The local delivery procedure is now [distribution.py](../../distribution.py),
documented in [distribution](distribution.md). Its selected **normal/build** graph
supersedes the historical 17-package inventory only for current bundle contents.
Product source tested: `82342f0e67761504bd643fe4d9d027af93481d0e`, locked Rust1.96.0,
release profile, `aarch64-apple-darwin`. No manifest/lock/default policy changed.
Core has17 external crates; Native18; Web/combined39. Build-time proc macros are
included; dev-only jsonschema and its graph are excluded. The CLI and optional
worker are built together with matching features from one committed archive.

Web adds locked transport/hash/random support: tungstenite0.30.0, log0.4.29,
bytes1.12.1, http1.5.0, httparse1.10.1, data-encoding2.11.1, sha1 0.11.0,
digest0.11.3, block-buffer0.12.1, crypto-common0.2.2, const-oid0.10.2,
hybrid-array0.4.15, typenum1.20.1, cpufeatures0.3.1, cfg-if1.0.5,
rand0.10.3, rand_core0.10.1, getrandom0.4.3, chacha20 0.10.2,
thiserror/thiserror-impl2.0.21. Native/Web host includes libc0.2.190.
All other versions remain in the tables below. serde_json now uses runtime
float_roundtrip as registered in D07@5; the old T01 feature row is historical.

Actual cached locked LICENSE/NOTICE/COPYING material was enumerated and selected
MIT texts read for these packages. unicode-ident retains additional Unicode-3.0;
serde_json retains Alexander Huszagh's lexical module attribution; sha1 preserves
its RustCrypto/Mozilla/Graydon Hoare notices. No NOTICE-named files were found.
Each build reassembles the complete actual texts and their hashes, rather than
copying this historical table as a license assertion. Missing material or a license
expression outside the reviewed set stops publication. New source/dependency
adoption still requires D07 review; the assembler is not a universal license audit.

The bundle carries [notice policy](../../THIRD_PARTY_NOTICES.md), full
DEPENDENCY_LICENSES.txt and selected package/version/checksum/notice hashes in its
manifest. No project license is granted. The pinned Rust toolchain’s complete COPYRIGHT-library.html is also shipped
for linked standard-library code outside Cargo.lock. Swift runtime and Apple
frameworks remain system prerequisites, not vendored SDKs. Playwright/Node/Chromium remain
fixture/external tooling; no browser/model stack enters Native/core delivery.

## S01 additions under D07@2

The Stage A normal/build dependency closure contains17 external packages: the
eleven T01 packages below plus the six schema-generation packages in this table.
`cargo tree --locked --offline -p uiblueprint-plugin-api -e normal,build` confirms
that jsonschema and its validation-only dependency graph do not enter that closure.
Cargo metadata may unify dev features in its output; it is not evidence that every
reported feature is active in a normal shipping build.

| Added package | Version / declared MSRV | Actual license files inspected |
| --- | --- | --- |
| schemars | 1.2.2 / 1.74 | LICENSE, MIT; Graham Esau2019 |
| schemars_derive | 1.2.2 / 1.74 | LICENSE, MIT; Graham Esau2019 |
| dyn-clone | 1.0.20 / 1.60 | LICENSE-MIT, LICENSE-APACHE; MIT option |
| ref-cast | 1.0.27 / 1.71 | LICENSE-MIT, LICENSE-APACHE; MIT option |
| ref-cast-impl | 1.0.27 / 1.71 | LICENSE-MIT, LICENSE-APACHE; MIT option |
| serde_derive_internals | 0.30.0 / 1.71 | LICENSE-MIT, LICENSE-APACHE; MIT option |

Direct dev-only jsonschema0.58.5 (MSRV1.85, MIT, Dmitry Dygalo2020–2026) performs
structural parity checks against the generated schema. Default HTTP/file/TLS/IDNA
and async features are disabled. Its actual LICENSE was read; its archive also
contains a test-suite license. No root NOTICE in the selected added packages.
Normal/build declared maximum MSRV is1.74; project policy remains1.96. No source
copying or platform/transport/model framework adoption occurred. Full distribution
notice assembly remains I01; this inventory is not a universal dependency audit.

The lockfile preserves exact checksums for the generation and dev validation graphs;
adding plugin-api added only the local package entry. The validator uses serde
map-only visitors and semantic checks; it never loads an input-selected schema.
The saved JSON Schema is generated from canonical Rust types. Existing T01 direct
versions/checksums are unchanged. The following T01 observations remain historical.

## T01 baseline

Authority: [D07@1](../specs/development/decisions/d07-reuse.md); actual resolution
for [uiblueprint-schema](../../crates/schema/Cargo.toml), Rust1.96.0,
aarch64-apple-darwin. [Cargo.lock](../../Cargo.lock) is the checksum/version source.
No dependency version/feature substitution from C01 was needed.

Resolved once with `cargo +1.96.0 generate-lockfile`; subsequent checks used
`--locked`. Read actual downloaded Cargo package manifests, target entry headers,
all selected MIT license texts and Unicode license, and enumerated packaged
LICENSE/NOTICE/COPYING files. This inventory is scoped to these11 registry packages,
not a universal source/security audit or completed distribution acceptance.

| Package | Version | Declared rust-version | Enabled features | Purpose |
| --- | --- | --- | --- | --- |
| serde | 1.0.229 | 1.56 | default,derive,serde_derive,std | Direct typed serialization and private raw-document decoding |
| serde_json | 1.0.151 | 1.71 | default,std | Direct JSON parser/writer |
| serde_core | 1.0.229 | 1.56 | result,std | Serialization traits/core support |
| serde_derive | 1.0.229 | 1.71 | default | Build-time derive proc macro |
| proc-macro2 | 1.0.107 | 1.71 | proc-macro | Build-time token representation |
| quote | 1.0.47 | 1.71 | proc-macro | Build-time generated tokens |
| syn | 3.0.6 | 1.71 | clone-impls,derive,parsing,printing,proc-macro | Build-time syntax parsing |
| unicode-ident | 1.0.26 | 1.71 | none | Build-time Unicode identifier tables |
| itoa | 1.0.18 | 1.68 | none | JSON integer formatting |
| memchr | 2.8.3 | 1.61 | alloc,std | JSON byte search |
| zmij | 1.0.23 | 1.71 | none | JSON numeric formatting support |

Declared dependency MSRVs all≤1.71; actual package compilation succeeded on1.96.0.
This does not test or change the project's chosen1.96 compilation minimum.
No Apple/browser/transport/async/model framework appears in the resolved graph.
No unbounded_depth, preserve_order, arbitrary_precision or serde rc feature enabled.

## Actual package licenses and required notice material

| Packages | Declared terms | Files present / selected terms |
| --- | --- | --- |
| itoa, proc-macro2, quote, serde, serde_core, serde_derive, serde_json, syn | MIT OR Apache-2.0 | LICENSE-MIT and LICENSE-APACHE; MIT option selected for this inventory |
| memchr | Unlicense OR MIT | COPYING and LICENSE-MIT; MIT option selected, includes Andrew Gallant2015 attribution |
| unicode-ident | (MIT OR Apache-2.0) AND Unicode-3.0 | LICENSE-MIT, LICENSE-APACHE, LICENSE-UNICODE; **MIT plus Unicode-3.0**, not MIT alone |
| zmij | MIT | LICENSE-MIT |

No NOTICE-named file was present in any of the11 downloaded packages. The inspected
target entry headers exposed no additional license declaration. Preserve each
applicable packaged license/copyright notice; many MIT files intentionally do not
name a holder in the text, so do not fabricate one. The MIT text requires retaining
copyright/permission notices for copies or substantial portions. Unicode text
attributes Unicode, Inc.1991–2023, requires its copyright/permission notice with
covered files/software or associated documentation, and restricts endorsement use.

No upstream source was copied into our hand-written code. Linked dependencies and
build-time generation still require an accurate final distribution inventory.
I01 must package applicable notice texts for the actual released artifact and
re-evaluate if the graph/features change. The project's own license is not
invented here; the crate remains non-publishable. This document records observed
terms and obligations, not a claim that a root metadata label licenses every file.

## Reinspect the installed locked graph

```sh
cargo +1.96.0 metadata --locked --offline --format-version 1 --filter-platform aarch64-apple-darwin
cargo +1.96.0 tree --locked --offline -p uiblueprint-schema -e features
```

Metadata `manifest_path` locates each actual package directory and its license
files. Cargo verifies registry package checksums from Cargo.lock. Direct checksums
match D07: serde `4148590afebada386688f18773da617792bf2ef03ffc1e4cbd2b1d45b023e0ba`;
serde_json `c841b55ecdae098c80dcae9cf767f6f8a0c2cdb3416bbef72181df4d0fe73f14`.
Do not regenerate/update the lockfile during routine verification. New dependencies
or changes to features/versions require their concrete consumer and selected-file
review before adoption, as specified by D07 and DEV.RUST@2.
