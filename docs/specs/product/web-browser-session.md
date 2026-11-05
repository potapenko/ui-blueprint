# Web browser-session connection

- Node type: leaf; domain: `uib.web.first-use`; contract: `UIB.WEB-BROWSER-SESSION@1`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: the user requested repair of UI Blueprint access in the chat
  “Спланировать перенос админки на Rust” on 2026-11-05, then explicitly directed
  immediate completion and authorized pressing the Chrome Allow button in the
  diagnostic chat. The accepted scope is browser-CDP compatibility, connection
  permission handling, skill/bundle update and real component measurements.
- Read when: URL/component first use through a browser DevTools connection.
- Requires: [WEB-FIRST-USE@7](web-first-use.md), [CLI@21](cli.md).
- Precedence: replaces only WEB-FIRST-USE.BINDING's direct page socket and its
  connection-wait bounds; all collection, identity, privacy and output rules remain.

## Connection and scope

Chrome approval-only debugging returns 404 for HTTP discovery and rejects direct
page WebSockets with 403. A working connection held by another tool does not
authorize or establish a new connection. Source: Chromium
[DevTools HTTP handler](https://chromium.googlesource.com/chromium/src/+/main/content/browser/devtools/devtools_http_handler.cc).
Local evidence: existing Chrome9222, native Allow sheet, HTTP404 and page upgrade403.

URL mode uses the supplied browser WebSocket and Target.attachToTarget(flatten=true)
for the uniquely resolved page. All page commands and replies carry that connection's
actual sessionId. Root control replies and other sessions must not satisfy a page
request. No synthesized /devtools/page endpoint, fallback, browser reconfiguration,
automatic permission grant, profile scan or application mutation.

The Rust worker opens its own connection to the same authorized browser endpoint,
attaches to the exact trusted target ID and uses the session ID returned on THAT
connection. Never reuse a session ID from the Node socket. Existing explicit-session
and direct-page connections retain their meaning. Collector binding still verifies
target/frame/loader/document before reading the selected component.

## Permission wait and bounded work

Initial browser handshake has a separate maximum60s wait and fixed pending/timeout
diagnostics. A timeout identifies no underlying cause; the skill checks for a native
permission prompt through the caller's authorized UI workflow. Declining permission
is not bypassed. No automatic connection retry or permission-setting change.
After handshake, URL preparation/revalidation has a75s overall bracket including
the worker's at-most60s attach. Per-CDP call2s,32 bootstrap calls,128 targets,
64KiB message/256KiB total and16 events remain. Worker attachment allows the existing
second Chrome permission prompt; the helper names this phase before spawning.
Collection remains32 nodes/depth8/128KiB/1000ms and109 methods; waiting for permission
does not increase measurement, traversal, memory or output limits. CLI child wait
is capped at65s and the remaining bracket. Legacy exact-ID timing stays unchanged.

## Cleanup and proof

Release the owned object group and detach the Node-owned target session, then close
its browser socket. Worker shutdown closes its owned socket/session. Never close
the user's tab/browser or disconnect another tool. No persistent bridge or service.
No output before the original identity/cleanup/publication checks.

Proof: flat-session correlation and wrong-session rejection; malformed/error attach;
old direct-page behavior; permission timeout with no geometry; helper cleanup and
stale-target negatives; actual local component through matching installed CLI/worker.
Facts must distinguish successful measurements from partial coverage or a refusal.
No public release, remote deployment or changes to the admin application are included.
