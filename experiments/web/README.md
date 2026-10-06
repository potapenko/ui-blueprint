# R01 bounded Web diagnostic

Question: can DOMSnapshot/CSSOM/AX observations keep exact node/document/target
identity through repeated names, node remount and child document replacement?
A secondary calibration checks geometry at DPR 1 and 2. This is supporting
research for C01/F01/W01, not a shipping adapter or B01–B06 acceptance.

`fixture.html` owns all content; `expected.json` is a manually written numeric
and transition oracle. `probe.cjs` never derives expected values from its
collector. It launches and closes its own headless Chromium, blocks network,
and uses two fresh contexts sequentially. No existing profile or tab is used.
No package files or root dependencies are changed.

Run with Node and an existing compatible Playwright Core installation:

```sh
R01_PLAYWRIGHT_CORE=/absolute/path/to/playwright-core \
R01_OUTPUT=/absolute/non-repository/path/report.json \
node experiments/web/probe.cjs
```

Without `R01_OUTPUT`, only the compact result prints; no report file is created.
The tested pair is Playwright Core 1.58.2 / Chromium 145.0.7632.6, bundled browser
revision 1208. No automatic installation or browser fallback occurs. Whole run
has a 25 s watchdog, launch 8 s, CDP calls 3 s, ordinary fixture waits 1.5 s.
The watchdog closes only the browser launched by this process.

The prototype compares literal CSS geometry at tolerance 0.01 css_px (fixture
arithmetic tolerance chosen before its first run, not a product accuracy gate).
It records each expected/observed result, source state, observations, timestamps,
source identities, coverage, unavailable geometry, browser/protocol and hashes.
Delivery is an exact ElementHandle pointer click. A fixture event counter and a
separate asynchronously applied counter establish distinct checks. Validation
rejects the old document/node/target; a detached-handle negative proves the
browser does not silently click the replacement. No production dispatch API
or race-free prepare/act guarantee is implemented.

`DOMSnapshot.captureSnapshot` acquires the entire small owned fixture before
selecting four controls. Its post-capture size guard does not bound acquisition
on arbitrary pages. Partial AX reads are target-addressed; no DOM rectangles are
relabeled as AX or visible/hit regions. Source namespaces remain separate even
when CDP supplies an explicit backend-node mapping. Cross-channel consistency is
unknown despite equal frame trees before/after. No screenshot is captured.

Out of scope: OOPIF, zoom/transforms/scroll calibration, shadow DOM, IME, secrets,
pixel redaction, subscriptions/cache/deltas, event loss, cancellation outcome,
Safari/Firefox, latency gates, full B01–B06 and independent acceptance.

See [research and adapter proposal](../../docs/research/R01-web.md) and
[terminal receipt](../../docs/plans/ui-blueprint/receipts/R01.md).
