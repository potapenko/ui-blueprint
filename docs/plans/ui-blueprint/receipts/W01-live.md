# W01 actual Chromium preparation receipt

Status: preparation ready; runtime waiting for saved preparation/explicit activation. No browser/server/fixture/test execution. Authority [W01-live](../packets/W01-live.md)
e02eeff plus durable-evidence amendmentc3c28a3 under approved PLAN.UIB@1.
Publication gap accepted ea4f64a; this does NOT itself activate browser/runtime.
Exact4 paths only: guarded-live.cjs, web_live.rs, host-web.md and this receipt.

## Basis and reuse

Reuse registry11 and complete current WEB-PILOTS/PILOTS/GEOMETRY/PROJECTIONS/FORMS/
CACHE/ACTIONS, D02@2/D03@2/D04@1/D05@4/MEMORY@2/WORK@1/D06@1/D07@5, RUST/DEV.RUST@2
and QA/operational closure. D05@4 Native-only delta leaves common Web/Rust unchanged.
No production/fixture/oracle/Cargo/lock/spec changes, dependencies or nested agents.
fixture-host.cjs prepare(), server.cjs start(), frozen R01/F01 assets and independent
expected.json oracles are reused. Rust uses actual RuntimeHost attach_web/
submit_web_observe, Attached clock, real Ticket/ACK/leases and current Collector.
No Node collector/d02_host/combined historical run.cjs or alternate product graph. Pinned Playwright1.58.2 launchServer/process/default-args/readiness/cleanup source was
read narrowly; no source copied. Thin driver adds loopback remote-debugging port0,
uses ONLY its owned generated profile and matches direct PAGE CDP URL to actual
Target.getTargetInfo ID. Playwright protocol URL is never sent to Rust. Actual
DevToolsActivePort/runtime endpoint availability remains UNEXECUTED, bounded refusal.

Rust is #[ignore] and requires explicit activation plus pinned worker path. Node
requires --run-authorized, UIB_WEB_LIVE_ALLOW=1 and both executable hashes before
launch. Source preparation/syntax/no-run cannot implicitly execute the live path.

## Prepared finite cases and fixed limits

Initial #left: real refs, AX Apply/button, DOM[40,60,120,40], distinct sources and
unknown AX geometry. #sized then fixture-only textLarge uses existing ref to prove
current [40,480,32,16]→[40,480,48,24], not a cached substitute. Second owned tab,
remount and navigation reject wrong/stale refs; explicit new attach/selection binds
actual backend/loader identities. Existing #draft receives only synthetic fixture
setup canary/type/autocomplete, without typing/events/submit. Every observe compares
focus/scroll/f01.checkpoint in memory. Earlier bytes survive refusals/reaps; fixture
browser must remain alive until its separate outer owner closes it.

32 output nodes/depth8/64KiB/250ms EVERY observe; literal ε0.01css_px. Initial traversal
cap256 declared before runtime. Existing explicit setup:16 refs/100 methods/8192
reply/65536 total reply/600 text/256 handle/32 AX properties;16384 read/32768 write
bytes and2048 IO steps. No tuning after failure or false complete coverage.
Launch8s (outer9s), endpoint/navigation3s, action2s/phase8s, host attach/cleanup5s,
consumer90s/whole120s, owned close/kill3–4s. No B02/input/Q02/pixels/real PlayPhrase.me.

## Minimal durable evidence — no writes during preparation

Exact planned exclusive UUID directory (currently DOES NOT EXIST):
/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P2/W01-guarded-live/4fc1c266-67b4-4b2a-85f7-60c959468c6b
Root retention: through P7 acceptance or explicit discard/replacement. Consumers:
W01 review, later G02/P7 canonical-data reuse. Runtime mkdir refuses existing runs;
fixed filenames use exclusive create and never overwrite another file.

Only report.json plus initial-left.json, sized-before.json and sized-after.json. Rust sends the exact ACKed UTF-8 bytes for those three positive cases. Node verifies
agreement with the validated Document/oracle, byte cap and absence of the canary,
then writes bytes WITHOUT JSON reserialization. Partial own failed writes are
removed; complete frame entries include byte length/hash. No expected record stands
in for observation. Frames are unchanged; report marks records historical/non-live
and separately records whether worker-session closure was confirmed.

Compact report includes source/build/fixture hashes, actual environment, fixed limits,
case outcomes/invariance and cleanup states. Failure codes/counts are sanitized;
unconfirmed cleanup is never reported successful. Report/file-write failure can emit
only that same compact metadata to stdout. No privacy body, private checkpoint,
raw stdout/stderr/CDP traffic, canary value or endpoint credentials are persisted.
Only report-listed complete frames with matching hashes are consumer inputs.

## Actually checked preparation and identities

Node syntax and owned Rust fmt: PASS. Changed Rust sink compiled --no-run on saved
f2af5a30226943412ae6c41dc7bb5c250a2f70c0: PASS, no warnings. Production5f52cb5; shared before/after equality passed, compile window released. Runtime NOT executed.

~~~sh
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<own-prep>/build cargo +1.96.0 test --locked --offline -p uiblueprint-host --features web --test web_live --no-run --message-format=json
~~~

Own prep /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-live-prep-4qyhni9l.
Test build/debug/deps/web_live-f02c419e3beec6bc SHA256
e4d0c58f1d095f4d5eeb1a4b9b3751a28fe38c800618689ee3cb1b5c3b8a4ad4;
worker build/debug/session-worker SHA256
e0ffd4b5993ff47b0ee0e0479f1daa8d581eec25cc689955100178352b5b5fba (unchanged).
Source96 digest10a407e42be854e4ec9a2e1852462b59da8e31a5e2a0ed9e7626e8613c641a4f:
compact sorted JSON path→SHA256: tracked f2af5a3 files under crates/{host,schema,engine,
plugin-api}/src and plugins/web/src; root Cargo/lock/toolchain + five crate Cargo;
web_worker_data.rs, fixture-host.cjs, six F01/R01 fixture/oracle files, two new code
files. Docs excluded. Previous prep digest/binary were superseded only by this sink.

## Concrete activation command — PREPARED, NOT RUN

~~~sh
S01_WEB_PLAYWRIGHT_CORE=/Users/eugenepotapenko/.npm/_npx/f88013d20c39cb98/node_modules/playwright-core \
UIB_WEB_LIVE_TEST=/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-live-prep-4qyhni9l/build/debug/deps/web_live-f02c419e3beec6bc \
UIB_WEB_LIVE_TEST_SHA256=e4d0c58f1d095f4d5eeb1a4b9b3751a28fe38c800618689ee3cb1b5c3b8a4ad4 \
UIB_WEB_LIVE_WORKER=/var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-web-live-prep-4qyhni9l/build/debug/session-worker \
UIB_WEB_LIVE_WORKER_SHA256=e0ffd4b5993ff47b0ee0e0479f1daa8d581eec25cc689955100178352b5b5fba \
UIB_WEB_LIVE_EVIDENCE='/Users/eugenepotapenko/Library/Application Support/UIBlueprint/development/P2/W01-guarded-live/4fc1c266-67b4-4b2a-85f7-60c959468c6b' \
UIB_WEB_LIVE_ALLOW=1 node tests/bridges/web/guarded-live.cjs --run-authorized
~~~

Checkpoint/root activation still required. No successful acquisition was repeated. Preserve all earlier named primary/build temps and this prep for the immediate run.
