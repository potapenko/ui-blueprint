# UI Blueprint third-party notices

This document describes the local bundle's notice policy. It does not grant a
license to UI Blueprint itself. Project crates remain `publish = false`; no project
license has been invented and this task does not publish a release.

Every local build writes **DEPENDENCY_LICENSES.txt** with the full original
LICENSE/NOTICE/COPYING files found in the actual selected locked normal/build graph.
The adjacent distribution-manifest.json records names, versions, registry checksums,
license metadata and notice-content hashes. Those two files travel with the binaries.
The graph is selected with Cargo normal/build edges, explicit target and the same
features as the binaries; dev-only jsonschema and its graph are not included.
Build-time procedural macros are included conservatively; they are not runtime
processes. Cargo metadata locates files but does not define the shipping graph.

The inspected graph at product revision 82342f0 includes:

- Core: serde, serde_core, serde_derive, serde_json, itoa, memchr, zmij,
  proc-macro2, quote, syn, unicode-ident, schemars, schemars_derive,
  serde_derive_internals, dyn-clone, ref-cast, ref-cast-impl.
- Native worker adds libc. No third-party Swift package is linked.
- Web/combined add libc, tungstenite, log, bytes, http, httparse, data-encoding,
  sha1, digest, block-buffer, crypto-common, const-oid, hybrid-array, typenum,
  cpufeatures, cfg-if, rand, rand_core, getrandom, chacha20, thiserror, thiserror-impl.

MIT is the selected alternative for MIT/Apache-2.0 or Unlicense/MIT packages;
all packaged license alternatives are nevertheless retained verbatim.
**unicode-ident requires MIT plus Unicode-3.0**, including its Unicode copyright
and permission text. serde_json's enabled float_roundtrip lexical implementation
is derived from Alexander Huszagh's lexical code; its original module attribution
is appended alongside serde_json's MIT/Apache texts. sha1's MIT notice retains
RustCrypto, Artyom Pavlov, Mozilla Foundation and Graydon Hoare attributions.
No NOTICE-named files were found in this inspected graph. A changed graph still
collects its own actual notice files; an unfamiliar license expression stops build
for review. This is not a universal source/security audit.

Rust standard-library code linked into the executables is outside Cargo.lock.
RUST_LIBRARY_NOTICES.html preserves the pinned toolchain’s complete upstream
COPYRIGHT-library.html, including its dependency notices and full license texts.
This conservatively retains all standard-library target notices, not a claim that
every listed platform-specific component is linked here. Rust/Swift compilers and
SDKs are build prerequisites; no compiler/SDK distribution is copied. Apple runtime/framework availability remains a
host requirement. Node, Playwright and browser executables are fixture/external
runtime tools and are not shipped. No upstream blueprint application, model,
image, font or screenshot is bundled. Earlier research reference repositories were
not copied into this delivery. See the source checkout's
`docs/development/dependencies.md` inventory for provenance and limits.
