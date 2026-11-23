# macOS geometry observations

Use the matching Native or combined bundle selected in [the skill](../SKILL.md).
Its `session-worker` and `native-host-helper` must be the siblings of the selected
CLI. Observe reads an explicitly bound already-running process; installation does
not grant Accessibility or screen-recording permissions.

## First-use boundary

The native/combined bundle includes `native-prepare.py` (optional Python3.11+).
Use the exact PID of the caller-authorized already-running app and its expected
bundle ID. The caller authorizes that process's current AXFocusedWindow; this
route does not select arbitrary windows/components or change focus. The preparer
reads public process metadata only, checks identity twice, then writes inputs.
It never collects UI, launches an app, requests permission or invokes Observe.

The app must expose public NSRunningApplication bundleIdentifier and launchDate.
Apple's NSRunningApplication.h documents that launchDate is unavailable when an
app was launched without LaunchServices. Running Contents/MacOS/executable directly
can therefore give target_unresolved despite a visible window. Do not substitute
ps timestamps or Info.plist values. The application owner may, within its authorized
runtime workflow, close its own preview and launch the same declared .app through
LaunchServices, then obtain a fresh PID. Preparation does not perform that setup,
retry automatically, or grant permission to close/relaunch someone else's process.

```sh
UIB_RUN=$(mktemp -d "${TMPDIR:-/tmp}/uib-native-inputs.XXXXXX")
python3 "$UIB_BUNDLE/native-prepare.py" --pid "$TARGET_PID" \
  --bundle-id "$EXPECTED_BUNDLE_ID" --out "$UIB_RUN"
```

Check exit0 before using `$UIB_RUN/connection.json` and `$UIB_RUN/request.json`.
The directory must already exist, be private and caller-owned. Existing files
are never overwritten. Exit4 means wrong/missing/changed process;2 invalid input,
bundle or destination;1 IO/helper/timeout. No automatic retry/fallback.
If the supplied bundle lacks this file, use a matching updated bundle; do not
invoke repository fixture scripts or fabricate identities. `describe-process`
is the identity source, not proof that an authorized window can be observed.

Check these input facts before calling Observe:

- Connection version is `1.0.0`; target, session, host limits and attach deadline
  are explicit. Provider is `native_ax`, with an absolute `helper_executable` and
  a bounded JSON-string `configuration`. No `channels` field on this provider.
- Configuration names `process` with the actual PID, bundle ID and full reported
  launch time, a scope ID and explicit acquisition limits. A PID or title alone is
  insufficient. Default `collection` is `focused-ax` for that process's held
  AXFocusedWindow. It does not focus or activate the app.
- The canonical core `0.1.0` Request has operation Observe and matches the trusted
  target/session/scope. For request ID R, the sole Surface is `ax-focused-R`,
  generation R. These are per-observation identities, not stable window handles.
- Collection is external_semantics with current_required freshness and explicitly
  selected supported fields, node/depth/byte limits and deadline. Connection
  capabilities and Request authority must agree. Use the supplied canonical schema
  to validate shape; validation alone does not establish caller authority.

An explicitly selected focused popup may use `focused-popover-ax`; it must be an
AXPopover whose held ancestry reaches that same window. Source-visible collections
may use `focused-visible-ax`. Neither is an automatic fallback from failed whole-window
collection, permission refusal or an unresolved target. Keep the existing limits.

## Observe with prepared inputs

`CONNECTION` and `REQUEST` are absolute paths to those reviewed regular files.
The following budgets are an example consistent with the ordinary caller, not
permission to widen the source scope. Run this block in its own shell command:

```sh
(
  umask 077
  UIB_RUN=$(mktemp -d "${TMPDIR:-/tmp}/uib-native-observation.XXXXXX") || exit 1
  printf 'Observation directory: %s\n' "$UIB_RUN" >&2
  "$UIB_BUNDLE/uiblueprint" observe --connection "$CONNECTION" --request "$REQUEST" \
    --worker "$UIB_BUNDLE/session-worker" --max-input-bytes 2097152 \
    --max-output-bytes 524288 > "$UIB_RUN/observation.ndjson"
)
```

Keep the actual exit and read each complete response. Ordinary `native_ax` is AX-only;
other prepared callers can produce multiple channel lines. Select an observed
ChannelResponse explicitly for [analysis](analysis.md) or [export](export.md),
preserving its original bytes and attribution. A Failed response is not a Snapshot.

## Interpret the result

AX-only geometry reports `accessibility_bounds` in `ax-screen`, kind `screen`,
units `pt`, origin `top_left`. Layout, paint, hit regions and pixel transforms can
remain unavailable. This route captures no images, probe or CG-window metadata.
Source-visible children do not prove pixel visibility or a complete collection.
Read-only observation cannot supply action refs or coordinate-click authority.

Choose exact SourceKeys from the returned source. Measure within that observation;
do not force ordinary observations with different Surface generations through a
stable-identity diff. Unknown coverage or a failed channel remains visible in the report.

Observe 4 can carry partial source data; 1 includes IO/unconfirmed cleanup;
2 invalid input/limits; 5 unsupported platform/build. Permission, stale binding and
unresolved target remain distinct source outcomes. Report the actual result without
opening permission dialogs, changing focus, broadening to another window or retrying
automatically. Cleanup may reap owned helpers, never the user's application.

## Partial coverage and AX identifiers

If an SDK coverage manifest lists the requested overlay as unmeasured, first
identify the requested frame kind. A separately authorized ordinary AX observation
can provide accessibility bounds without changing the app. It does not recover
missing SwiftUI layout bounds. Use the existing saved AX response when available;
otherwise use the preparation/Observe recipe above for the owner's current window.
Do not automatically fall back from a permission or identity refusal.

The ordinary preparer selects the named process's `AXFocusedWindow`. It has no
CGWindowID or arbitrary AXIdentifier subtree selector. `inspect --ref` selects a
SourceKey in saved data; it does not narrow live acquisition. Repository fixture
utilities and `native-session` are not interchangeable ordinary-app recipes.

When `accessibility_name` is requested, inspect both the canonical property and
`extensions`: `AXTitle` preserves the source title separately from the canonical
name (which this collector reads from AXDescription). `AXIdentifier` is also an
extension. Each extension's value is in `property.state.value.value` only when
`property.state.availability` is `known`. Unknown, unsupported and empty differ.
SourceKeys are observation-local aliases, not AXIdentifiers or reusable handles.

For example, enumerate the recorded subtree of one exact observed AXIdentifier.
`SNAPSHOT` must be one original observed ChannelResponse, not multiple NDJSON lines;
`WANTED_AXID` is the caller's selected identifier, not a label guessed from pixels.
This reads saved data only and preserves the original document:

```sh
python3 - "$SNAPSHOT" "$WANTED_AXID" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
a = d['artifact']
assert a['kind'] == 'channel_response'
r = a['data']['result']
assert r['status'] == 'observed', 'response has no observed Snapshot'
s = r['data']
def known(state):
    return state['value']['value'] if state['availability'] == 'known' else None
def extension(node, name):
    return next((known(e['property']['state']) for e in node['extensions']
                 if e['name'] == name and e['namespace'] == 'macos.ax'), None)
def key(k):
    return (k['namespace'], k['key'])
nodes = {key(n['key']): n for n in s['nodes']}
roots = [n for n in s['nodes'] if extension(n, 'AXIdentifier') == sys.argv[2]]
assert len(roots) == 1, f'expected one recorded root, found {len(roots)}'
pending, seen = [roots[0]['key']], set()
print(json.dumps({'coverage': s['coverage'], 'recorded_nodes': len(nodes)}))
while pending:
    k = key(pending.pop(0))
    if k in seen:
        continue
    seen.add(k)
    n = nodes[k]
    print(json.dumps({f: n[f] for f in
        ('key', 'native_role', 'properties', 'extensions', 'children')}))
    pending.extend(n['children'])
PY
```

Require the correct recorded ancestor as well as the label/identifier. A form's
Close button and a full-window dismiss backdrop may both be named Close; a window
close control is another object. Likewise, a named inner scroll area and its outer
container can report different rectangles. Keep both identities and frame meanings.
Copy the chosen node's exact `{namespace,key}` into `inspect --ref` and use the
[factual query recipe](analysis.md#construct-a-factual-query) for Rust `measure`.

The 0.6.1 preparer requests at most 160 nodes, depth 9, 512 KiB and 1000 ms. The collector
walks breadth-first and charges discovered handles before collecting their children.
An exhausted node budget can therefore leave a recorded group with `children: []`
even when the runtime has children. A partial response with 160 nodes is evidence of
reaching that ceiling, not proof of the missing nodes' runtime existence or absence.
Check requested limits, depth, source outcome and available coverage before assigning
a cause. Exit4 can accompany useful partial data; preserve the Observe exit immediately
before another shell command overwrites it. Validation alone does not establish coverage.

A successful observation with a smaller background list qualifies that state only.
Do not switch to empty Favorites, hide rows, enlarge quotas or retry automatically to
make the original case pass. If children remain unmeasured, report the available parent
and controls separately, and request a qualified scoped route or an explicitly agreed
new scenario. Do not describe a missing selector capability as an implemented feature.

## SwiftUI versus Compose

Use native `accessibility_bounds` in their reported pt space and Compose
`layout_bounds` in their sourced px/dp space as separate measurements. AX may
merge/omit internal pieces; its rectangle is not a SwiftUI layout probe. Invoke
Rust measure separately on each original record, with independent source keys
and spaces; report both values and qualifications. Numerical equality does not
establish equivalent layout/paint/hit areas or pt=dp mapping.
Existing `compare` requires compatible plugin/Target/namespace; `diff` also needs
its strict identity basis. Do not rewrite snapshots or force cross-platform pairs.
Screenshots/interaction remain separate caller-owned visual evidence.

## Opt-in SwiftUI layout observations

An application owner can explicitly integrate the separately supplied
`UIBlueprintSwiftUI` Debug SDK and provide a canonical core0.1 Snapshot from its
real SwiftUI component. This is a different source from `native_ax`; the ordinary
preparer does not install, inject or activate it. Require the SDK's supplied
revision/integration instructions and the app owner's authorized capture path.
Do not assume the installed CLI archive contains this source package or modify
an application merely because the user requested a read-only observation.

For an integrated app, follow its documented capture ingress for the exact selected
window. If no smaller component is requested, use its window-content root with
`selection: .mountedSubtree`: required markers are a minimum, and all current
marked descendants are exported. Do not supply a fixed sample list as the entire
window. The legacy `.requiredMarkers` mode retains explicit subset capture.
Unmarked SwiftUI internals are not automatically discovered. Source integration
is separate from using an already integrated app; a missing capture route is an
explicit prerequisite, not a reason to substitute Accessibility bounds.

One window may have separate content, toolbar and presentation roots. Use the
app's coverage result and each original canonical record independently, preserving
surface IDs and local spaces. Equal space names in different records do not prove
a common origin. Account for unavailable toolbar/overlays and unmaterialized rows;
do not merge their coordinates or call mounted layout pixel visibility.
When the app publishes a coverage manifest last, require that completion record
before treating the multi-file window request as finished. Earlier individual
Snapshot files alone do not prove that every requested surface was accounted for.

The producer measures selected bounds anchors inside the real app, with semantic
marker IDs, separate mount UUIDs and declared parent/component membership.
Use its original `swiftui.layout` SourceKeys and `swiftui-root-pt` local point
space through the existing [saved analysis](analysis.md) recipe. No new importer
or handwritten geometry is needed. Inspect the record before choosing anchors.
Do not substitute the old F02 fixture manifest or its hard-coded component names.

Capture sequence, measured-layout revision and displayed frame are distinct.
The owner checks selected state and `isCurrentMounts(snapshot)` before publishing
the SDK bytes. Saved output remains cache/unverified; equal observations do not
prove an unchanged animation interval. Source display scale and SwiftUI-global
root coordinates alone do not establish a screen/pixel transform.

Layout, hit, visible and paint coverage stay separate. In particular, a native
button's Accessibility rectangle may be smaller than its SwiftUI layout frame.
Never coerce those rectangles to equality. A screenshot pairing needs its own
source-bound window/capture metadata and a qualified transform. Actual app
interaction and probe-on/off invariance remain separate evidence.

## Independent native PNG from the selected window

Use the matching installed CLI/helper in `UIB_BUNDLE` (0.3.0 or a qualified later
version). This path requires existing Screen Recording permission; it never opens
a permission prompt. It performs no app launch, focus, input or resize. The app
owner supplies an authorized live PID and expected bundle ID.

For an operator-designated mounted volume, set `UIB_VOLUME` to its mount path
and `UIB_TMP_BASE` to its approved existing directory named `tmp` on that volume.
In a task shell verify the mount before creating private temporary storage:

```sh
test -n "${UIB_VOLUME:-}" && test -n "${UIB_TMP_BASE:-}" &&
test "$(stat -f %d "$UIB_VOLUME")" != "$(stat -f %d "$(dirname "$UIB_VOLUME")")" &&
test "$(stat -f %d "$UIB_TMP_BASE")" = "$(stat -f %d "$UIB_VOLUME")" &&
test -d "$UIB_TMP_BASE" && test -w "$UIB_TMP_BASE" || exit 1
UIB_RUN=$(mktemp -d "$UIB_TMP_BASE/uib-window.XXXXXXXX") || exit 1
export TMPDIR="$UIB_RUN" TMP="$UIB_RUN" TEMP="$UIB_RUN"
```

Record the task owner, purpose and retention condition in this directory.
The capture helper admits destinations below this caller-owned 0700 task root;
intervening directories must remain on the same device, owned and free of symlinks,
with write access restricted to their owner.
Where system temp is the approved storage, the existing system-temp recipe applies.

First obtain actual CGWindowIDs for that PID, without capturing anything:

```sh
"$UIB_BUNDLE/uiblueprint" capture-window --list-windows \
  --pid "$APP_PID" --expected-bundle "$APP_BUNDLE_ID"
```

Use the returned process launch_time and the explicitly selected window_id. CUA,
AX, SDK and CGWindow IDs are not interchangeable. Multiple candidates require a
real window choice; do not guess by title or capture another window as fallback.
Only numeric metadata from the selected PID is returned, never foreign titles.

```sh
"$UIB_BUNDLE/uiblueprint" capture-window \
  --pid "$APP_PID" --window-id "$CG_WINDOW_ID" \
  --expected-bundle "$APP_BUNDLE_ID" --expected-launch-time "$LAUNCH_TIME" \
  --out "$UIB_RUN/native-window" --request-id "window-reference"
```

`UIB_RUN` is the verified caller-owned task directory; `native-window` must
not yet exist. The command returns bounded JSON and writes `window.png` plus
`capture.json`. PNG is native lossless full-window SDR, at source scale, without
shadow/cursor/audio/child windows and without resizing/upscaling. Keep images and staging files according to the task ownership and retention policy.
The default deadline is5seconds; `--deadline-ms` accepts100–10000. Pixel ceilings
4096 per side and8,388,608 total refuse larger output rather than downscale it.

Metadata binds PID/bundle/launch and CGID, records actual window/filter rectangles,
point_pixel_scale and PNG dimensions/hash, and provides verified screen-to-image
and window-local-to-image affines when the source APIs establish them. A changed
process/window/frame/display configuration refuses. This is an independent external
observation, not an atomic GPU frame or a proof of visibility/occlusion.

Associate with existing SDK geometry without modifying its JSON. Confirm the same
process/window and settled state. A source-proven root offset `(rx, ry)` in the
window maps local `(x,y)` through the window-local affine after adding that offset;
for natural scale `s`, this is `(s*(x+rx), s*(y+ry))`. Use exact per-state metadata,
not a prior window height. GPUI coordinates already in window-content space still
need the actual content-to-full-window inset. SwiftUI-global is not screen space.
When an offset/space is uncertain, verify it against native bounds and selected
image edges; do not invent it. A Snapshot's `captures: []` does not prohibit this
external association. Never require new donor-side bridges or GPU-frame tokens
merely to use independently observed screenshot/geometry evidence.

Show the retained native PNG inline. Distinguish derived association from reported
source facts and preserve unknown layout/hit/paint properties. No PNG payload or
private UI content is sent to another service merely by successful capture.

## External SwiftUI cached layout

A native or combined bundle from 0.4.0 onward includes `swiftui_debug_capture.py`
and the matching read-only cache decoder. The 0.5.0 option for matching repeated
native classes within a SwiftUI owner is fixture-only. The earlier Reels
one-shot admission has been consumed. Source-owner qualification supplies the exact
pinned request and acquisition procedure. Keep the immutable request, consumer
identity, chosen window and approved source-type scope together. Use the
consumer's qualified procedure for live collection. Installation alone does
not authorize attaching to a running app.

For a successful saved acquisition, require the terminal manifest to identify
the expected request, a complete snapshot filename and restored detach. Validate
the snapshot with the matching uiblueprint-validate. Inspect boundary_records
in cache_audit to find the canonical node index for each source-qualified
component. A type may produce several nested records. Read field availability
before measuring a particular node. Layout of a covering ancestor is a separate
observation. Each local pt space has its own identity and an unknown transform
until independent source evidence establishes a mapping.
