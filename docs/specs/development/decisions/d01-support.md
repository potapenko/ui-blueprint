# D01 — toolchain and initial support matrix

- Domain: `uib.development.d01`; accepted/released baseline: none.
- Authority source: [C01-DEC-001](README.md#meaning-and-precedence).
- Node type: leaf; contract: `UIB.D01@1`; clause: `UIB.D01.CONTENT`.
- Authority: Active / Stability: Evolving; no accepted/released product support.
- Read when: T01 setup or W01/M01 support/capability declaration.
- Do not read when: no build/runtime support choice is affected.
- Requires: [BOUNDARIES@1](../../product/boundaries.md),
  [ROADMAP@1](../../product/roadmap.md), [RUST-BOUNDARIES@1](../../product/rust-boundaries.md),
  [evidence](evidence.md), [Rust engineering rules](../../../../RUST.md).
- Owner/deadline: T01 pin before first Cargo build; W01/M01 matrix before P2 claims.

## Requirement, observation, decision

Reproducible stable setup and platform separation are required. Observed R03
compiled on installed Rust 1.96.0; official versioned manifest confirms that stable
release. Choose `rust-toolchain.toml`: channel `1.96.0`, profile `minimal`, components
`rustfmt`, `clippy`; edition `2024`, workspace resolver `3`, rust-version `1.96`.
Initial supported compilation minimum intentionally equals the pinned version:
no evidence supports promising an older compiler. This is policy, not a claim that
the source intrinsically requires every 1.96 feature. A later MSRV change needs
its own compile/lock/dependency evidence. T01 installs/uses the named toolchain
if needed; C01 only observed the existing `stable` alias resolving to 1.96.0.

Reject moving stable/nightly pins and untested older-MSRV promises. Edition 2024
and resolver 3 use the supported Cargo path; they do not select an async framework.
Target initially `aarch64-apple-darwin`. Platform-neutral owners contain no Apple
or browser SDK dependencies. No blanket `--all-features` or all-target-OS build.

| Slice | Selected development configuration | Evidence / qualification |
| --- | --- | --- |
| Core/schema/plugin API/CLI | Rust 1.96.0, edition 2024, aarch64-apple-darwin | R03 standalone diagnostic compiled; Cargo product still T01/S01 work |
| Web | Chromium 145.0.7632.6, CDP 1.3, exact authorized tab; headless fixture 800×600 DPR1 (R01 also DPR2) | R01/F01 on macOS 27.0.1 build 26A434, Apple M4 Pro; no product adapter accepted |
| Native | Public AX + ScreenCaptureKit; thin Swift 6 bridge, SDK27.0, compile deployment target macOS14.0 | R02/F02 run only on macOS27.0.1 arm64; 14.0 is compile minimum, not verified runtime support |
| Opt-in probe | Own debug SwiftUI fixture, explicit one-shot request | Current three-marker evidence; full P01 off/on acceptance open |
| Other OS/arch/browser | No qualification yet | Intel Mac, macOS14–26 runtime, Linux/Windows, Safari/Firefox/WebKit/OOPIF and mobile not claimed |

Node24.15.0/Playwright Core1.58.2 and Chromium bundle1208 are **fixture tooling**,
not mandatory shipping runtime dependencies. A Web-only build must not invoke
Swift/Xcode, and a core-only build must not attach any runtime. Target-scoped
capabilities can still be partial/permission_required independently by channel.

## Concrete feature/owner policy and acceptance

T01 begins only the concrete schema owner named in [handoff](handoff.md); no
empty platform crates. When L01/adapter owners exist, CLI features `web` and
`macos` opt into their corresponding adapter; default features stay empty until
I01 chooses the installable distribution. Core/schema/plugin-api have no platform
feature switch. `macos` is valid only on Apple targets; unsupported combinations
give an explicit build/configuration error, not silent substitution.

T01 records locked core build/fmt/clippy/test results on the pinned host. S01
validates pure owners without runtime permissions. W01/M01 record exact browser,
OS/SDK/caller permission and per-channel capability evidence before advertising
support; M01 must reproduce/fix concurrent capture and W01 must prove bounded
acquisition. Before I01, verify web-only has no Apple SDK build path and native
feature does not require a browser/model stack. Older runtime qualification is
an explicit future matrix entry, never inferred from deployment target.
