# Native input preparation
- Node type: leaf; domain: `uib.native-first-use`; contract: `UIB.NATIVE-FIRST-USE@1`.
- Authority: Active / Evolving; accepted/released baseline: none.
- Authority source: human message `01a17cdd-a517-7221-99fd-a8cc05c830f1` in Kotlin
  chat `01a1691a-c072-7500-b304-a53377be2769`, read directly2026-10-27: use Blueprint
  for SwiftUI/Kotlin geometry and ask Blueprint chats to implement missing support.
- Read when: preparing installed native_ax connection/request from a selected PID.
- Requires: [NATIVE@5](native.md), [CLI@21](cli.md), [IDENTITY@1](identity.md),
  [PRIVACY@1](privacy.md), [D05@4](../development/decisions/d05-limits.md).

## NATIVE-FIRST-USE.PREPARE
Optional Python3.11+ `native-prepare.py --pid PID --bundle-id EXPECTED --out DIR`
uses sibling, manifest-verified native helper `describe-process` to read actual
PID/bundle/full launch time/target generation. Exact expected bundle must match;
repeat the bounded identity read before publishing and refuse incarnation changes.
The caller owns an already-running process and authorizes its focused window;
PID/bundle identity does not select an arbitrary window title or component.
No AX/content collection, activation, permission prompt, launch, retry, input,
network, pixel capture or fixture files. It prepares inputs only; Observe must
still establish and revalidate the actual AXFocusedWindow at acquisition time.

## NATIVE-FIRST-USE.DATA
Output is exclusive0600 connection.json and request.json in an existing private
caller-owned nonsymlink directory. Conflicts refuse without overwriting; failures
remove only files created by this attempt with unchanged inode identity. Success
prints their paths as JSON after both files close. Input metadata is strict bounded
JSON; helper calls each have3s timeout,4096-byte output admission and sanitized errors.
Matching bundle manifest hashes are checked before executable use. Python belongs
only to this optional preparation tool, never the core/native Observe dependency.
Connection1.0.0 and core0.1 Request remain unchanged. New UUID request/session IDs;
Surface ax-focused-REQUEST with generation REQUEST. Source process generation is
retained verbatim. Fields role/accessibility_name/accessibility_bounds only;
external_semantics, design projection, current_required. Fixed existing developer
profile:160nodes/depth9/512KiB/1000ms; attach2000ms; native acquisition profile and
host quotas identical to the prior developer geometry caller, never enlarged.
Failure stderr is a fixed code: exit2 invalid/bundle/conflict,4 unresolved/changed
process or bundle mismatch,1 IO/helper/timeout. Failure never claims observation.
Stored inputs are not current UI proof. Original Observe refusal/exits remain.

## NATIVE-FIRST-USE.ACCEPT
Focused tests: full metadata precision and generated canonical request; expected
bundle mismatch; disappeared/restarted process; malformed/oversize metadata;
output conflict/symlink/private-directory guards and inode-safe rollback;
unchanged limits; legacy1–3 bundle update/rollback/removal preserved by manifest4.
Consumer runs actual preparation→Observe on its own preview and retains attribution;
source/synthetic success alone does not qualify a live window or grant permissions.

## NATIVE-FIRST-USE.COMPARISON
Native AX bounds(pt) and Compose layout bounds(px/dp) remain separate facts.
Measure each independently through Rust with sourced spaces/units; no automatic
unit equality, shared origin or visual equivalence. Existing compare requires its
protected plugin/Target/namespace basis; do not restamp inputs to cross platforms.
No new cross-identity comparison or arbitrary component-selection capability.
