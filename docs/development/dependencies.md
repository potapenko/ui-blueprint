# Resolved T01 dependency inventory

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
