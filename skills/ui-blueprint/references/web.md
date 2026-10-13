# Observe one Web component

Use the matching Web or combined bundle selected in [the skill](../SKILL.md).
This optional helper needs an already-installed Node.js 24+; the Rust saved-data
commands do not. Check `node --version` and the helper's offline help first.

```sh
node "$UIB_BUNDLE/web-geometry.cjs" --help
```

## Required inputs

- `CDP_BROWSER_WS`: an already-authorized browser DevTools WebSocket on numeric
  loopback, with an explicit port and `/devtools/browser/ID` path. It has no
  credentials, query or fragment. A page URL is not this connection or its authority.
- `PAGE_URL`: the exact authorized, already-open HTTP(S) page URL, including query
  and fragment. No credentials or known credential query keys. Multiple matching
  page targets are ambiguous; do not choose the first one.
- `COMPONENT_SELECTOR`: one explicit compound selector such as `#picker`,
  `input.search` or `[role="dialog"][aria-label="Settings"]`, naming exactly one
  component. These are examples, not targets to assume exist.

The helper supports root-document light DOM only. No selector combinators, lists,
pseudo-selectors, escapes, frames, template/slot traversal or authored shadow hosts.
Do not add IDs to the app to make a selector work. Missing connection/selection is
a prerequisite to resolve through the authorized caller, not a reason to scan
profiles, guess ports, launch/navigate a browser or change application configuration.

## Read measurements or save a source

For a concise inspection and sourced width/height:

```sh
node "$UIB_BUNDLE/web-geometry.cjs" \
  --connection "$CDP_BROWSER_WS" --url "$PAGE_URL" --component "$COMPONENT_SELECTOR"
```

To obtain original canonical bytes for further analysis/export, use `--json`.
Run this shell block in its own command; its exit is the helper's actual status:

```sh
(
  umask 077
  UIB_RUN=$(mktemp -d "${TMPDIR:-/tmp}/uib-observation.XXXXXX") || exit 1
  printf 'Observation directory: %s\n' "$UIB_RUN" >&2
  node "$UIB_BUNDLE/web-geometry.cjs" \
    --connection "$CDP_BROWSER_WS" --url "$PAGE_URL" \
    --component "$COMPONENT_SELECTOR" --json > "$UIB_RUN/observation.json"
)
```

Record the printed temp path and exit. Exit 4 may include a useful partial observed
response, or no response at all. Check nonempty complete JSON, observed result and
coverage before using it. On IO failure, stdout may be physically incomplete;
do not feed it to export. Do not merge diagnostics into the saved JSON or extract
and reconstruct its geometry. Continue with [analysis](analysis.md) or
[export](export.md) using the original single observed ChannelResponse document.

The helper revalidates the same target/document/component and releases its own
resources before publishing. This establishes its observation binding, not ongoing
freshness or atomic UI capture. Names/roles are not requested by this geometry route;
do not infer labels from selectors or promise text inventory from these fields.

## Limits and failures

The caller is deliberately bounded: URL-mode collection requests 32 output nodes,
depth 8, 128 KiB and 1000 ms, with a 16-node collector cap. Selection walks allow
2048 visits/depth 64. `--max-visited-nodes` can lower, not raise, the visit limit.
Large components can remain partial or refuse. Do not automatically retry or enlarge
limits; a newly selected smaller scope must still answer the user's authorized task.

Read exit status, canonical coverage and safe stderr together. 0 means a complete
result within the requested scope; 4 partial/unresolved/ambiguous/stale/boundary/
deadline; 2 invalid arguments/selector/quota; 1 IO or cleanup failure. Preserve any
more specific Rust result. Report only returned diagnostic facts; unknown subcause
stays unknown. Never close the user's page/browser as cleanup. Apply the skill's
temp-file cleanup and image-retention rules after analysis is complete.
