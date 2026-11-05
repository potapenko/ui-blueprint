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

The helper supports root-document light DOM only. A compound selector may end with
one `:nth-of-type(N)`, where N is a canonical integer from 1 to 2048; it counts
same-type siblings within their parent. No other pseudo-selectors, combinators,
lists, escapes, frames, template/slot traversal or authored shadow hosts.
Do not add IDs to the app to make a selector work. Missing connection/selection is
a prerequisite to resolve through the authorized caller, not a reason to scan
profiles, guess ports, launch/navigate a browser or change application configuration.

## Chrome permission and browser sessions

Use the existing caller-supplied browser endpoint. Chrome's built-in remote debugging
may return HTTP404 for `/json/version` and HTTP403 for direct page sockets; that does
not mean the skill is missing. The matching helper and Rust worker use browser CDP
sessions instead of constructing page endpoints. Keep their versions together.

`browser_connection_pending` precedes the initial handshake (up to60s).
`observer_connection_pending` precedes the Rust worker's own handshake (up to60s).
Chrome may show **Allow remote debugging?** for each connection. Check its visible UI
while the command runs. Follow the host's permission rules; when the user has already
explicitly authorized this diagnostic access, handle Allow yourself within that scope
instead of asking the user to operate the computer again. Never bypass a denial,
change debugging settings, or assume another tool's connection grants this one.
A timeout alone does not prove a permission denial or an absent browser/skill.

The helper retains exact URL/component binding throughout; permission waits do not
increase the1000ms geometry-collection deadline or any collection quota. The agent
must allow enough shell-command time (at least180s for both possible permission waits)
and keep the command observable while handling prompts. A caller timeout must not be
shorter than this documented workflow. It closes only its own sessions/sockets.

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
depth 8, 128 KiB and 1000 ms, with a 32-node / 109-method collector cap. Selection walks allow
2048 visits/depth 64. `--max-visited-nodes` can lower, not raise, the visit limit.
Large components can remain partial or refuse. Do not automatically retry or enlarge
limits; a newly selected smaller scope must still answer the user's authorized task.

Read exit status, canonical coverage and safe stderr together. 0 means a complete
result within the requested scope; 4 partial/unresolved/ambiguous/stale/boundary/
deadline; 2 invalid arguments/selector/quota; 1 IO or cleanup failure. Preserve any
more specific Rust result. Report only returned diagnostic facts; unknown subcause
stays unknown. Never close the user's page/browser as cleanup. Apply the skill's
temp-file cleanup and image-retention rules after analysis is complete.
