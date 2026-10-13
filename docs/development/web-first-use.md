# Web first use: existing connection, URL and component

The optional Node.js 24+ caller selects an existing permitted page and component,
then invokes the real Rust Observe → Inspect → Measure path. You do not prepare
connection/request JSON or target/frame/loader/backend IDs. Core and low-level Web
CLI/worker remain Node-free; nothing is installed or configured by this command.

## Run from a verified Web or combined bundle

Reproduce the tested Web binaries with the current recipe from the checkout:

```sh
BUNDLE="$(mktemp -d "${TMPDIR:-/tmp}/uib-web-first-use.XXXXXX")"
python3 distribution.py build --modules web --revision abd88783221412f8e3f9d1da1f3c9a2b4733b665 --destination "$BUNDLE"
```

The helper/guide are versioned recipe assets; the manifest records their hashes
separately from that immutable Rust product pin. Keep the full set together.

```sh
python3 "$BUNDLE/distribution.py" verify --destination "$BUNDLE"
node "$BUNDLE/web-geometry.cjs" --help
node "$BUNDLE/web-geometry.cjs" \
  --connection "$CDP_BROWSER_WS" \
  --url 'http://localhost:3000/#/clip-search?language=en' \
  --component '#SELECTED_EXISTING_COMPONENT'
```

Replace the illustrative URL and component with your explicitly selected existing
page/component. CLI and worker default to siblings of the helper, so these are the
only consumer inputs. For a checkout caller, --cli and --worker may override both
with absolute paths from one matching verified bundle. Commands work from any cwd.
Help/-h alone is offline; it performs no connection, executable check or temporary IO.

## One-time connection prerequisite

CDP_BROWSER_WS must be an **already authorized browser DevTools WebSocket**, for
example the shape `ws://127.0.0.1:PORT/devtools/browser/ID` (numeric IPv6 loopback is
also supported). It is a browser connection supplied by your permitted caller,
not a page URL, page WebSocket or credentials. Do not guess a port. A URL does not
grant browser access. If this connection is unavailable, the helper reports a fixed
connection-required/connection-failure diagnostic: return that prerequisite to the
UI Blueprint owner/caller. This guide does not prescribe browser/profile changes.
No browser registry, profile/process scanning, app integration, launch, navigation,
focus, input or model key is involved. In particular, do not modify PlayPhrase.me
or expose a debug port merely to follow this guide.

Once that connection is supplied, per-observation runtime identities are obtained
and checked by the helper, not by the user. The browser/page stays open after use.

## Exact selection, not a first match

URL comparison uses WHATWG URL.href serialization, including query AND fragment.
Scheme/host case and default ports normalize; path/query/SPA fragment differences
remain meaningful. There is no title, substring or reordered-query match. HTTP(S)
URLs only, without credentials, control characters or known credential query keys.
Zero permitted page matches returns target_unresolved; multiple returns
ambiguous_target. The helper never navigates to create a match.

Component is one compound CSS selector: optional tag or *, plus #id, .class,
[attribute] or [attribute="value"] terms. Examples: `#picker`, `input.search`,
`[role="dialog"][aria-label="Director"]`. Simple ASCII identifiers and quoted
Unicode attribute values are supported, including literal colons in quotes.
One terminal `:nth-of-type(N)` is supported: canonical integer1..2048, no leading
zeros, signs, whitespace, arithmetic or keywords. Examples: `ul:nth-of-type(6)`
and `ul:nth-of-type(3)`. It counts same-type element siblings within their parent,
including siblings that do not match the rest of the compound; text/comments do
not count. It is not a global index. Matching ordinals in multiple parents refuse.
Combinators/whitespace between terms, selector lists, escapes and all other pseudo
selectors (including :has) remain unsupported; a suffix cannot precede more terms.
Selector text is data passed separately from the fixed selection function.
No IDs have to be added to the application; use its existing public selection.

Search covers the root document's light DOM only, under a bounded complete walk.
It does not enter frame/shadow/template contents. Selected authored open/closed
shadow hosts refuse via both DOM and CDP boundary metadata; user-agent internals
of ordinary input controls are not traversed and do not exclude their public host.
A selected frame/template/slot boundary also refuses. Multiple matches, missing roots or exhausted traversal refuse;
finding one element before reaching a limit does not prove uniqueness.

## What the operation does

The helper derives the actual page target, root frame/loader/document and element
backend ID, retaining the original document/root/parent objects. It builds private
temporary connection/request records and calls the installed Rust collector.
After Observe it checks URL/page uniqueness, document identity and the SAME selected
root again before printing results. Navigation, remount, reparent or changed selector
binding refuses; it never substitutes a new same-label/selector element. This
includes sibling insertion/removal/reorder that changes the held node’s ordinal.
The ordinal is counted during the same bounded walk, with no extra sibling scans.

All geometry comes from the canonical Rust observation/analysis. Output contains
design inspection, sourced width/height and component layout bounds, units/evidence,
coverage and unavailable fields. Normalized names/roles are not requested by this geometry profile. Existing
source-native roles and reported children are shown as attribution, not inferred
logical membership. Layout bounds do not prove hit/visible bounds, padding or atomicity.
Temporary files and remote handles/connections are owned and cleaned; source apps
and their data are not cleanup targets. This caller creates no durable observation
file itself; opt-in --json below lets the caller save the original response.

## Save the original observation for export

Add the valueless `--json` flag to the URL+component invocation. Stdout contains
only the original canonical observed ChannelResponse from Rust Observe, including
its original newline. It skips human Inspect/Measure and preserves IDs, precise
values, units, availability and privacy. Safe diagnostics stay on stderr.

Publication follows final identity revalidation, confirmed worker/remote cleanup,
child reaping and private temporary-file removal. Failed observation, stale binding
or cleanup failure publishes no JSON. Exit4 with a nonempty valid response is useful
partial data; exit4 with no response is a refusal. Physical stdout failure can leave
partial bytes and returns1; never export such a file or automatically retry.

This shell example uses a private system-temp directory. Set PAGE_URL and
COMPONENT_SELECTOR to the authorized existing selection, and METADATA_FILE to
reviewed caller metadata in the [export guide](export.md#observed-file-to-package).
Metadata adds document/state/scope/environment annotations, never geometry or a
replacement Snapshot. Use unknown for unavailable state/environment and draft
approval; `public_text_fields: []` exports no unreviewed public text.

```sh
(
  umask 077
  OUT="$(mktemp -d "${TMPDIR:-/tmp}/uib-web-export.XXXXXX")" || exit 1
  observation_exit=0
  node "$BUNDLE/web-geometry.cjs" --connection "$CDP_BROWSER_WS" \
    --url "$PAGE_URL" --component "$COMPONENT_SELECTOR" --json \
    > "$OUT/observation.json" || observation_exit=$?
  case "$observation_exit" in 0|4) ;; *) exit "$observation_exit" ;; esac
  test -s "$OUT/observation.json" || exit 4
  "$BUNDLE/uiblueprint-validate" --max-bytes 131072 "$OUT/observation.json" || exit $?
  "$BUNDLE/uiblueprint" imagegen-prompt \
    --snapshot "$OUT/observation.json" --metadata "$METADATA_FILE" \
    --out "$OUT/package" --max-input-bytes 2000000 --max-output-bytes 4000000 \
    --max-components 32 --max-views 1 --components-per-detail 12 --json || exit $?
  printf 'Observation and package: %s\n' "$OUT"
)
```

The finite export limits are example caller choices, not new defaults. Export
accepts the untouched response directly; no extraction, JSON adapter or manual
geometry entry is needed. It preserves partial/unknown source status. A written
package does not mean an image was generated, checked or accepted. The caller owns
the saved non-image files through their declared review/retention period. Legacy
exact-ID mode does not accept this flag; its existing behavior is unchanged.

## CSSOM and IntersectionObserver disagreement

If initial and callback CSSOM rectangles match exactly but IntersectionObserver
reports different target bounds, layout remains the exact CSSOM measurement.
Requested visible_region and the three existing IO facts are unknown with reason
`intersection-bounds-disagree-with-layout`; nothing is rounded or snapped to IO.
Hit samples retain their own attribution. The observation remains partial and
sequential/non-atomic, and every identity/viewport/cleanup check still applies.
If the two CSSOM rectangles differ, collection still refuses with resync/code1.
These code1 semantics apply to diagnostics contract@3 and its producer revision;
older@2 records meant any rect-triple disagreement. Keep the producer pin with
saved diagnostics. A partial response is not full visibility or paint evidence.

## Limits and outcomes

Preparation:10s overall,2s per CDP call,32 calls,128 targets,64KiB per text reply,
256KiB total per connection and16 events. Both selection walks have at most2048
visited nodes, depth64 and1s each. --max-visited-nodes may lower (not raise) the2048
limit. Browser/WebSocket internals are opaque tooling costs, not an RSS guarantee.
URL-mode collection uses32 output nodes/depth8/128KiB/1000ms, collector32 nodes/109
methods/256 visits and unchanged host/transport bounds. Layout, hit and visible fields are all
retained. The component budget follows measured full replies above64KiB and serial
per-node IntersectionObserver waits, not a retry or a relaxed D06 threshold. Legacy
exact-ID mode keeps collector16/methods100/64KiB/250ms. These limits do not promise whole-form
completeness; a large selected subtree can still return a truthful partial/refusal.
The URL collector now admits up to32 Elements within the existing request32; this
allows a24-element whole component including non-layout elements such as STYLE.
The explicit109-method allowance matches the existing rooted preflight3×32+13;
method counters and guards remain. All other byte/work/deadline limits are unchanged
and can still refuse. No field/node is
removed to fit, and separately observed subcomponents are not merged.

- 0: complete result within requested scope.
- 4: partial, unresolved/ambiguous/stale, unsupported source scope or deadline.
  Useful dimensions can accompany partial output; inspect coverage and unknowns.
- 2: invalid/missing arguments, unsupported selector syntax or a quota refusal.
- 1: IO or unconfirmed cleanup. Other Rust exits retain their meanings.

Preparation/revalidation failures print no result. URL mode uses the matching CLI's
bounded opt-in diagnostic report and prints only validated enums/counts, never raw
stderr, URLs, selectors or UI text. A partial observation can print useful geometry
AND a diagnostic with terminal=completed, committed=1, incomplete=1. A refusal names
the actual terminal and available Host stage/cause; host_detail=unavailable stays
unknown. A generic collector_limit or timeout does not identify a particular budget;
code=0 can mean unspecified, not success. Worker cleanup and remote cleanup differ.
Missing/malformed reports fail as observe_diagnostic_invalid: verify a matching new
bundle, not an old CLI with unsupported flags. There is no silent fallback. Make a new explicit request only under current
authority; there is no automatic retry, relaxed guard or source stabilization.

## Legacy exact-ID mode

The previous invocation remains available and cannot be mixed with the new mode:
--cli/--worker, --endpoint (page WebSocket), --target-id, --frame-id, --loader-id,
--document-backend-id and --root-backend-id. Every identity must come from the SAME
original document/selection. See --help for the complete signature. Legacy mode
also constructs its own temporary connection/request JSON.

## Qualification boundary

The installed URL+component route is checked on the existing controlled F01 fixture,
with independent geometry and failure/state/cleanup checks. That is not real
PlayPhrase.me acceptance. The USE-WEB1 owner performs the independent Director retry
with its already permitted browser connection and actual URL/component.
