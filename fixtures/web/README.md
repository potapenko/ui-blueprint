# Controlled Web fixture F01

Canonical scenario extension for B01–B06 tooling. It composes the unchanged,
accepted [R01 foundation](../../experiments/web/fixture.html) with
[extension.html](extension.html) and [fixture.js](fixture.js). R01 is frozen
historical evidence, not a second evolving application. Do not edit its fixture,
probe, receipt, ledger or archived report when extending F01.

[expected.json](expected.json) contains independently authored states, numeric
geometry and sampling parameters. Neither the fixture nor a collector reads it;
only the verification driver does. [run.cjs](run.cjs) checks browser observations
against that oracle and records which assertions test the fixture, CDP, or driver.
It is not a production adapter, graph, schema or executor.

Run from repository root using the existing tested Playwright Core 1.58.2 runtime
and its Chromium 145.0.7632.6 bundle (1208):

```sh
F01_PLAYWRIGHT_CORE=/absolute/path/to/playwright-core \
F01_OUTPUT=/absolute/non-repository/path/report.json \
node fixtures/web/run.cjs
```

Create the output parent directory first. Omit `F01_OUTPUT` for compact stdout
only. The driver creates a localhost server on a fresh port, new headless browsers
and ephemeral contexts; it blocks requests outside that server origin and closes
only its own resources. No global install, root manifest or logged-in tab needed.
Whole-run watchdog 60 s; browser launch 8 s; CDP/navigation 3 s; ordinary action
2 s; intentional blocked/detached clicks 250 ms. A timed-out run fails, not passes.

For another authorized driver, start only the fixture server:

```sh
node fixtures/web/server.cjs
```

The printed URL serves `/` and `/fixture.js`. Stop with Ctrl-C; automatic expiry
is 30 minutes. Add `?generation=1` to set a deterministic fixture generation.
A fresh navigation resets state, timers, rows and viewport-dependent application
content; the driver separately restores viewport 800×600. Reloading with a new
generation creates a new document. URL/generation metadata does not replace actual
CDP loader or target identity. No endpoint changes an external system.

Controller calls are **synthetic fixture setup/stimuli**, not native input:

- `window.f01.checkpoint()` returns declared generation/revision/source_state and
  application state, solely as a controlled oracle input.
- `window.f01.operate(name)` supports `remount`, `overlay`, `removeOverlay`,
  `textLarge`, `locale`, `parentWide`, `fontLarge`, `rowsChanged`.
- Each operation emits `fixture-change` with a monotonic sequence. The driver
  intentionally drops sequence 2 to model notification loss at its subscriber.
  Actual browser/adapter notification-loss testing remains W03 work.

The form is exercised with Playwright keyboard and pointer APIs. Reading the
controller alone is not evidence of delivery: the driver also checks DOM state,
AX, actual pointer interception/detachment and the fixture's delivery counters.
Unexpected-transition stopping is driver policy, not a shipped executor claim.
Geometry uses CSSOM CSS pixels with explicit viewport/document context. Two hit
samples and rect intersection do not prove arbitrary visibility or occlusion.

See [scenario mapping, baseline and proposed gates](../../docs/development/fixtures-web.md)
and [F01 receipt](../../docs/plans/ui-blueprint/receipts/F01.md).

## Focused fixed 5 Hz supplement

This mode runs only four paired off/on windows of 2 seconds each. It does not
rerun scenarios or the original cold/warm/saturation baseline. On windows schedule
10 whole-fixture captures at 200 ms intervals; a full-period-late slot is skipped
and makes schedule verification fail, with no catch-up burst. Off windows keep
identical AX/Performance/rAF instrumentation and perform no captures.

```sh
F01_PLAYWRIGHT_CORE=/absolute/path/to/playwright-core \
F01_BASELINE_REPORT=/absolute/path/to/frozen/report.json \
F01_OUTPUT=/absolute/path/to/supplement/report.json \
node fixtures/web/run.cjs --fixed-overhead-only
```

The output must be new and distinct from the frozen report. Put the retained
`baseline-run.cjs` historical harness beside the output first; its SHA-256 must
match the original report's harness hash. The F01 receipt locates both retained
artifacts. Repeats can use a new filename beside that same retained harness.
The mode verifies identical runtime metadata and unchanged fixture inputs, records
new harness hashes and the frozen report hash, and preserves the original report.
Five Hz is an experimental demand hypothesis, not a product polling policy.
