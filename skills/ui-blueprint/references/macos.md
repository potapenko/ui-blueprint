# Ordinary macOS Accessibility observation

Use the matching Native or combined bundle selected in [the skill](../SKILL.md).
Its `session-worker` and `native-host-helper` must be the siblings of the selected
CLI. Observe reads an explicitly bound already-running process; installation does
not grant Accessibility or screen-recording permissions.

## First-use boundary

This installed path requires **prepared trusted connection and Request JSON** from
the authorized caller. The skill does not ship a PID-to-connection generator.
If those inputs are absent, report exactly that prerequisite and continue any
independent saved-data work. Do not invoke repository fixture/build scripts as if
they were installed commands, invent process identity, or collect other applications.

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
