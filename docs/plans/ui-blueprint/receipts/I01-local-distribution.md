# I01-L — local distribution

## Authority, scope and traversal

Finite shipping_product task under approved PLAN.UIB@1 P6 and
[the explicit full-cycle packet](../packets/I01-local-distribution.md). The initiating
chat instruction explicitly authorizes own planning followed by implementation,
checks, scoped commit+push without a second approval. Single-chat execution;
no subagents, new chats, branches/worktrees or desktop/browser operation.
Mode Restore: deliver existing installable-selected-modules/model-free/recovery
requirements. Technical packaging choices use delegated ROADMAP authority; they do
not add product behavior. No released baseline or specification delta is claimed.

Traversal before source inspection: applicable AGENTS → implementation governance,
product-truth core/routing/change/evidence/delivery/coordination, QA governance;
`docs/specs/README.md` registry27 → product/README and acceptance/README →
BOUNDARIES, RUST-BOUNDARIES, ROADMAP, CLI@14, COMPLETION → explicit closure.
Decision branch → D01@1, D02@2, D03@3, D04@1, D05@4, D05-MEMORY@2,
D05-WORK@1, Native acquisition@2, D06@1, D07@5, EVIDENCE@1; DEV.RUST@2/RUST.
Operational safety read before corrective install/remove handling.

Full selected leaves read: BOUNDARIES/MODEL/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/
ACTIONS/LIFECYCLE/CACHE/PRIVACY/ROADMAP/RUST-BOUNDARIES@1, EXCHANGE@2, NATIVE@2,
CLI@14 (CONTENT/OBSERVE, preserved command meaning); ANALYSIS@2 with TYPES/
VALIDATION@1; PILOTS/WEB-PILOTS/NATIVE-PILOTS/GOLDEN/PERFORMANCE/COMPLETION@1;
EXPORT/DRAWING-PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/REVIEW/EXAMPLE@1; REUSE@1.
Clauses are the named leaves' CONTENT clauses plus the explicitly named analysis,
D02 publication/lifecycle and D05 resource/ownership clauses. Context is the complete
finite packet and these required leaves, not the original monolithic specifications.
Excluded conditional siblings: new CLI diff/graph/actions/neighbors implementation,
UI design, future/mobile/ML/source research and orchestration coordination routes.

Drift: packet said registry26/CLI@13; working tree held registry27/CLI@14 from the
concurrent G13 owner. That adds opt-in graph diff without changing the distribution
contract. It was read as current intent but not included as uncommitted source in
the build. Tested product pin is **82342f0e67761504bd643fe4d9d027af93481d0e** (saved
master after initiating checkpoint3fa8d3a). Existing non-overlapping dirty files and
`after-title-spacing.png` remained untouched.

Evidence inspected: current manifests and executable owners, locked Cargo tree and
metadata, selected crate license material, CLI saved-data/export documentation and
validator, Native helper documentation/build-geometry reference. Historical
rust-workspace.md's initial one-crate description is stale inventory, not current
build authority. No product source, manifests, lockfile, CLI documentation or spec
registry was edited. No upstream implementation was copied.

## Local plan and delivered choice

Before edits the chat declared exactly: root distribution.py, README.md,
THIRD_PARTY_NOTICES.md; docs/development/distribution.md and dependencies.md;
tests/bridges/distribution_check.py; this receipt. All are task-owned paths.
Plan: committed-source private temp build → selected binaries/schema/examples and
actual notices → exclusive flat publication with manifest last → offline smoke and
failure/recovery tests → documentation → scoped save/push. This is the actual
approved immediate-execution boundary, not retrospective authority from this file.

Existing source supplies CLI/validator, optional matching worker and thin helper.
Agent technical choice: standard-library Python manager with core/web/native/combined
selection, release profile and explicit existing absolute destination. No framework,
new dependency, global install, symlink/PATH shim, signing or release publication.

Build uses rustup run1.96.0, locked/offline Cargo and a committed Git archive.
Private CARGO_TARGET_DIR/module cache/source staging are system-temp. CLI/worker
features are selected together; Native compiles the existing helper owner. All
product artifacts come from one source pin. Invoked recipe/docs hashes are recorded
separately; they need not be at that product pin. This is reproducible source and
procedure, not guaranteed bit-identical binaries across build paths/SDKs.

Flat bundle: CLI+validator; worker except core; helper for native/combined; two
schemas; six synthetic input examples; RUN.md, manager, notice policy, full Cargo
license texts, full pinned Rust standard-library notices and manifest. No images,
fixture app, browser/model, compiler or SDK is shipped. Explicit trusted live
connection/worker/helper paths remain unchanged product behavior.

Publication uses exclusive/no-follow file creation, records own inodes for rollback,
and writes the manifest last. Verify checks the fixed complete inventory and every
hash/mode/type. Remove preflights the whole set and removes only its exact files;
foreign files and the directory remain. Modified/missing/symlink files refuse.
Crash/power-loss remnants require a new destination or manual inspection; no force
repair/recursive cleanup. Concurrent edits/removal while running are unsupported.

## Verification

All build/install operations ran only in owned system-temp folders on arm64
macOS27.0.1 (26A434), Rust1.96.0/ac68faa20, Swift6.4/SDK27.0.
No live UI collection, permission request or browser/app launch occurred.

Build command shape:

```sh
python3 distribution.py build --modules MODULE \
  --revision 82342f0e67761504bd643fe4d9d027af93481d0e --destination EXISTING_TASK_TEMP
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/distribution_check.py --bundle ABSOLUTE_BUNDLE
```

| Selection | Actual configuration / result |
| --- | --- |
| core | CLI defaults off + validator;17 external normal/build crates; no host/Web/Swift; PASS |
| web | CLI web + host web + validator;39 external crates; xcrun/swift/swiftc/xcodebuild refusal sentinels in PATH; PASS |
| native | CLI macos + host defaults off + helper + validator;18 external crates; PATH contains Rust/system tools only, no Node/browser tooling; PASS |
| combined | CLI web,macos + host web + helper + validator;39 external crates; PASS |

The final packaging adds pinned Rust COPYRIGHT-library.html beyond the Cargo graph;
the common inventory/notice change was checked by rebuilding all four selections.
Both core and Web final builds use the Apple-build-tool refusal sentinels. Mac native
linking still uses the system linker/SDK, not Swift collector compilation.

Each bundle passes verify and --help from cwd `/` with no model credentials/build
PATH. All four supplied canonical inputs validate; Rust measure returns literal
8 css_px/local-form, check returns pass. Observed document and proposed document
exports each create exactly six files, generated_image=false, without model calls.
Missing input refuses with empty stdout. `otool -L` on every shipped executable
resolves only system `/usr/lib` and `/System/Library` dependencies, with no build-temp
or checkout dylib paths. These are deterministic saved-data smoke
checks, not live/E2E, calibration, performance or independent acceptance evidence.

Focused safety checks pass: early destination errors (relative/missing); missing
tool and revision; late foreign-file conflict; injected fsync/write failure after
partial publication with rollback/no manifest; modified-file refusal before deletion;
symlink refusal; malicious manifest path rejected; preservation of unrelated data;
scoped removal and reinstall. All four actual first-pass bundles were removed with
their own copied managers, retaining an unrelated sentinel and destination, then
rebuilt. Tests never delete images or image-containing directories.

Notices: actual selected graph17/18/39, exact locked versions/checksums, complete
LICENSE/NOTICE/COPYING texts and hashes; selected MIT + unicode-ident Unicode-3.0;
serde_json lexical/Alexander Huszagh and sha1 attributions retained. No NOTICE-named
file in this graph. Dev jsonschema is absent. Pinned Rust standard-library copyright
HTML is retained in full, including its own dependencies/license texts, independently
of Cargo packages. No blanket license/security audit or project license claim.

Documentation checks: changed local Markdown links and route consistency;
`git diff --check`. Rust business logic was unchanged; no unrelated logic suite,
full workspace test or visual QA was run or claimed. Required broad V01/Q01/Q02
acceptance remains separate.

## Limits and completion boundary

The local delivery outcome is complete once the scoped checkpoint/push succeeds;
final chat reports its exact SHA. Builds are qualified only on the recorded arm64
Mac. No Linux/Windows/Intel/macOS14–26 runtime, mobile/TV, signed/notarized release,
live Mac/Web/M05/P7, privacy/isolation V01 or D06 performance qualification follows.
No blocker remains for this finite I01-L slice. Product source revision is explicit;
concurrent G13/Web/Native changes require a newly selected source build for their
acceptance. A coherent bundle does not independently prove protocol/runtime behavior.

After successful final checks, all four final bundles were removed using their own
copied manager; each preserved the unrelated test sentinel. Then only run-owned
non-image sentinels, tool guards and metadata were removed, empty task directories
removed and their absence verified. No image paths were created or touched. No raw logs, generated
hash receipt files or run archives are committed. This authored receipt is the
packet-required durable handoff for the I01/Q01 consumer.
