# S01-Web — F01 common interface proof

Read [shared handoff](S01-bridges.md) completely; its restrictions apply.
Owner: existing R01/F01 Web chat. Finite outcome: actual F01 observation emitted
through the committed Stage A request/channel envelope and validated by Rust.

## Inputs and selected route

Use F01 `a8368076cdf0d4a917e7c3c5448ca6b6d919cf64` plus documentation correction
`585bcd90ad455110bcc2432f8f36f58e3c8d7ecf`; current fixture is read-only.
Read docs/development/fixtures-web.md and fixtures/web/README.md for setup/cleanup.
Reuse the previously read F01 WEB-PILOTS/PERFORMANCE closure when still current.
Use the existing approved Node/Playwright fixture toolchain; it is not a shipping
dependency. No new browser/CDP transport dependency or production adapter here.

## Scope and evidence

Own one isolated headless context and localhost fixture, exact target/surface.
Request one bounded semantic/geometry fragment with explicit fields/limits/deadline.
Read only the selected DOM/CSSOM and addressed AX needed by this proof; a whole
DOMSnapshot followed by truncation cannot claim bounded scoped acquisition.
Keep observed native roles, coordinate units/space and independent channel times.
Do not claim transformed frame/zoom precision beyond actual evidence.
Show request, live response, validator result and preserved partial/error evidence.
Use the common framing/lifecycle owner for negatives; no parallel protocol policy.

## Exact write/resource set

- `tests/bridges/web/**`
- `docs/development/interface-proof-web.md`
- `docs/plans/ui-blueprint/receipts/S01-web-proof.md`

No edits to fixtures/web, experiments/web, Cargo, schema/plugin-api, native bridge,
shared docs, global config or real applications. Shared API needs go to Integration.
No shared desktop input; only isolated headless resources. Git lease at checkpoint.
Run scoped syntax/tests and the actual fixture→wire→Rust-validator chain plus
assigned error checks. Report exact commands/build identity and cleanup.
