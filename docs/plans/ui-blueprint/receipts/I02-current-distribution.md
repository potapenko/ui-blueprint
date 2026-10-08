# I02 — current local distribution qualification

## Authority and plan

Finite single-chat full-cycle task under approved PLAN.UIB@1 P6 and the
[I02 packet](../packets/I02-current-distribution.md), dispatched from root chat
01a11088-e608-7801-bdfb-db5c9383af9d. The initiating instruction expressly permits
own plan followed by execution, documentation, scoped commit+push without further
root approval. No subagents, other chats, UI operation, branch/worktree creation,
signing, notarization or release publication. Mode Restore; qualification of
existing delivery, not a new product contract or implementation acceptance.

The plan was stated before edits: build four selections from committed94724df;
extend the owned offline check for Native session and current compare; verify
model-free saved-data paths and safe remove/reinstall; update exact owned docs;
clean own non-image temp resources; checkpoint and push under the shared Git lock.
Write set: README.md, THIRD_PARTY_NOTICES.md, docs/development/distribution.md,
docs/development/dependencies.md, tests/bridges/distribution_check.py and this
receipt. distribution.py was conditionally allowed only for a demonstrated defect;
none was found; the recipe is unchanged. Product source/manifests/specs and Q01 harnesses are protected.

## Spec Basis and evidence boundary

Traversal: applicable AGENTS → implementation governance, product-truth
core/routing/change/evidence/delivery/coordination and QA/operational safety →
[registry](../../../specs/README.md) revision29 → product/README and acceptance/README.
Selected CLI@16, NATIVE-SESSION@1, BOUNDARIES/RUST-BOUNDARIES/ROADMAP@1,
COMPLETION@1 and their full explicit closure; decisions branch → D01@1, D02@2,
D07@5 and DEV.RUST@2/RUST. Read dependencies: EXCHANGE@2, NATIVE@2,
MODEL/IDENTITY/GEOMETRY/PROJECTIONS/FORMS/ACTIONS/LIFECYCLE/CACHE/PRIVACY@1;
CLI-ACTIONS@3, CLI-EXPORT@2, CLI-DIFF@2, CLI-GRAPH-DIFF/CLI-GEOMETRY-DIFF@1;
ANALYSIS@2 with TYPES/VALIDATION@1; D03@3, D04@1, D05@4, MEMORY@2, WORK@1,
Native acquisition@2, D06@1, EVIDENCE@1; PILOTS/NATIVE-PILOTS/WEB-PILOTS/GOLDEN/
PERFORMANCE@1; EXPORT/DRAWING-PACKAGE/STYLE/GEOMETRY/PROMPT-A/PROMPT-B/REVIEW/
EXAMPLE and REUSE@1. Named CONTENT clauses plus CLI entry/export, D02/D05 lifetime,
publication, compatibility, privacy and limits remain unchanged.

Context is the finite packet, selected branch nodes and complete linked contracts;
no monolithic spec preload. Excluded: UI design, mobile/future features, independent
runtime/privacy acceptance and root coordinator-only routes. Cross-domain clauses
are protection/qualification requirements, not authority to edit their owners.
Old revision labels in Requires links resolve to the current files above; no new
contract delta or authority conflict was introduced.

Requirement: coherent installed modules, isolation from unnecessary tooling,
model-free analysis/export and safe recovery. Observed evidence: recipe, build and
feature manifests, CLI session dispatch, export output shapes, bundled fixtures,
I01 receipt and accepted source/safety review of3075239. I01's tested82342f0 is
historical; it does not prove current Native session/export integration.
Agent choice: reuse the sufficient recipe and add only affected smoke assertions
and operator documentation. Existing product/public formats are not redesigned.

## Exact inputs

- Initial checked-out master/task checkpoint: e0ce277388a6decde99a31355d8908ed0571b08b.
- Product source for every build: 94724dfd412f966d3d7a90db29aec8be7e35d650.
- Recipe last changed: 6a5bec23b8d15e4cc825aaff6105ac001a9a17bb; already includes
  NativeFormSession.swift. Recipe SHA-256:
  62fcf2f486d2b1ce8e69d87272089be20ab4bb52e53a7faaa077006c45027cd5.
- Cargo.lock SHA-256: 8084aeea8b0606cb11ac17a3293dc686f86c84f18809ac4f828e92c8fb672841.
- Recipe/docs provenance is distinct from product source; final checkpoint holds
  the documentation and verification changes, not new product binaries/source.

All Cargo manifests, lockfile and toolchain file are byte-unchanged between I01's
82342f0 and94724df. Selected features remain explicit/defaults-off. The existing
I01 license review and [independent source/safety acceptance](export-popup-distribution-review.md)
are reused within their stated scope; no universal license/security audit is claimed.
Concurrent commits and non-overlapping WIP are excluded by git archive of the exact
product pin. Existing after-title-spacing.png is untouched and never cleaned.

## Verification and completion

Builds ran with Rust1.96.0/ac68faa20, release/locked/offline,
aarch64-apple-darwin, macOS27.0.1 build26A434; Native helper used Apple Swift6.4
(swiftlang-6.4.0.34.1), SDK27.0, deployment target14.0. No older runtime is qualified.
Each install destination, source archive, Cargo target and Swift cache is owned
system temp. Two independent builds ran concurrently, without shared target dirs.

```sh
python3 distribution.py build --modules MODULE \
  --revision 94724dfd412f966d3d7a90db29aec8be7e35d650 --destination EXISTING_TASK_TEMP
PYTHONDONTWRITEBYTECODE=1 python3 tests/bridges/distribution_check.py --bundle ABSOLUTE_BUNDLE
python3 ABSOLUTE_BUNDLE/distribution.py verify --destination ABSOLUTE_BUNDLE
python3 ABSOLUTE_BUNDLE/distribution.py remove --destination ABSOLUTE_BUNDLE
```

| Selection | CLI / worker features | Actual checks |
| --- | --- | --- |
| core | none / absent | Release build, verify, saved-data smoke PASS;17 external crates; no Swift invocation |
| web | web / web | Release build, verify, saved-data smoke PASS;39 external crates; no Swift invocation |
| native | macos / none | Release build including current helper, verify, saved-data smoke PASS;18 external crates |
| combined | web,macos / web | Release build including current helper, verify, saved-data smoke PASS;39 external crates |

Core/Web builds put refusing xcrun/swift/swiftc/xcodebuild sentinels ahead of the
system tools. Every build PATH contains only a temp rustup link and system tools;
no Node/Playwright/browser stack. All shipped executables have only system
/usr/lib or /System/Library dynamic dependencies (otool -L), no checkout/build paths.
The manifest source/features, recipe digest and common notices/docs match in all
four sets. Native helper and worker are built from the same archive as their CLI.
This confirms packaging provenance, not actual live private-protocol acceptance.

Smoke runs from cwd / with PATH=/usr/bin:/bin and no model credentials:
- Four canonical example inputs validate; measure returns literal8 css_px and
  check returns pass using analysis0.2.0.
- Document/propose each write six files, result_version0.1.0, generated_image=false.
- Direct saved-pair compare changes a synthetic width30→34; six files,
  result_version0.2.0, recorded layout change and displacement(0,0,4,0).
  This checks current E03 representation without claiming live observations.
- Help exposes native-session. Empty invocation gives invalid_arguments/2 in
  native/combined (entry present), unsupported_command/5 in core/web (feature absent),
  with empty stdout. No attach, helper invocation, permission prompt or UI input.
- Missing analysis input gives IO1 and no partial stdout.

Focused safety cases pass: late conflict rolls back only own files; injected fsync
failure leaves no manifest/partial publication; modified-file and symlink refusal
before removal; malicious manifest path refusal; foreign-file preservation; scoped
synthetic remove/reinstall; missing tool/revision/destination and relative destination.
Actual installed sets also refuse overwrite before compilation. Every real bundle
is removed by its own copied manager, preserving the foreign-I02.txt sentinel and
folder, then rebuilt into the same destination. All four reinstalled outputs passed the
same smoke, manifest/notice fingerprint and dylib checks before final removal.

### Dependency and notice fingerprints

Graph fingerprints are SHA-256 over each manifest dependencies array serialized
with Python json.dumps(sort_keys=True,separators=(',',':')); they include exact
versions, registry checksums, declared licenses and notice-file content hashes.
Shared package records agree across selections. The same committed manifests,
lock and features justify reusing I01's scoped source/license review; regenerated
fingerprints verify this run's material, not a nonexistent historical hash baseline.

| Selection | Graph fingerprint | DEPENDENCY_LICENSES.txt SHA-256 |
| --- | --- | --- |
| core | 04d150beeafd6219f2890248164dc5d948d08766e66d9c1802064c23727effa2 | 5d1a95506c3ea4d4d33e3f1e33feb62909c637eb3117d87e05906fdaa0a55b07 |
| native | 794e234264001d4c8852233fb9bd280643f98a8fe0902ec1b77acfd667c72401 | 7affb0885047cb61156bd13a1ba60851edd64d30b42c1bae9590606b02d87152 |
| web/combined | 5799e3427e446ab380d522860602ab6aa52b6fae01091145771ce43c374412d4 | 41feaf4df8cee2655d071e9f63ade5a062f6b112d2341bb2e773549278096f09 |

Pinned Rust standard-library notice SHA-256:
78c163fcec50e64bfd85fedb850c273595602fafa2b41f30f75d4e410b80ee83.
Unicode-3.0 and serde_json lexical/Alexander Huszagh attribution remain present;
jsonschema is absent from shipped graphs. No new licensing choice, dependency or
compiler policy. Package graph review remains distinct from a universal audit.

## Qualification and remaining limits

Classification: **verification**, with updated documentation and focused smoke
coverage. Existing recipe is sufficient; I02 makes no distribution.py or product
source change. Working deliverable is the reproducible four-selection procedure
and its checked operator examples, not another packaging framework or new runtime.

No compile/API/packaging blocker was found at94724df. Native form delivery,
Web runtime, privacy/isolation, full pilots, D06/P7 and release acceptance remain
Q01/Q02 or their designated owners' work. No UI test, live private-protocol proof,
independent review rerun, universal binary reproducibility, older macOS/Intel/
Linux/Windows/mobile qualification, signing or notarization is claimed. Existing
source/safety review is reused only for the unchanged manager, not labeled new
independent acceptance of this task's harness.

All eight actual builds (initial plus reinstall for four selections) succeeded.
All four reinstalled sets passed the final offline harness, including package and
receipt versions. Final removal used each bundle's own manager and preserved its
foreign sentinel. Afterwards only run-owned sentinels, tool guards and three
transient JSON result/fingerprint files were removed; the exact task-temp directory
was empty, removed, and its absence verified. Recipe-owned source/build stages and
smoke/safety non-image stages were cleaned by their owners. No images were created,
relocated or deleted; existing unrelated after-title-spacing.png remains untouched.
No raw command logs, builds or generated run evidence are staged/committed.

Python syntax, changed local Markdown file links, route consistency and git diff
--check pass. No Rust source changed, so no unrelated Rust logic/full-suite or UI
checks were run. The final exact-path checkpoint is made in the current master
under /tmp/ui-blueprint-master-git.lock (fcntl.flock), then pushed to the existing
canonical origin/master. The completing chat reports the resulting SHA; this
receipt is its own checkpointed record, avoiding a self-referential commit hash.


