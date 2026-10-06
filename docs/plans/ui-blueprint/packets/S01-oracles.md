# S01-oracles — independent semantic expectations

- Class: tooling; bounded independent oracle author, not implementation reviewer.
- Authority: approved S01/PLAN.UIB@1 and user's explicit request to parallelize.
- Consumer: S01 wire fixtures/validator tests, then G01/K01; no duplicate schema.
- Ready: accepted C01/T01 and fixed GOLDEN01/product clauses; no implementation
  syntax is needed to establish expected semantic outcomes.
- Model/reasoning inherit. Current master only; no branch/worktree/nested delegation.

## Spec Basis

AGENTS → specs/README → decisions/handoff and D03/D04/D05@1 with their explicit
closure, GOLDEN@1 and RUST-independent product CONTENT requirements. C01 decision
commit `42d2e6b`; T01 version boundary `28a08d3`; initial wire candidate0.1.0.
Use the complete current applicable closure. No product changes or API inventions.
Do not read implementation files/tests in crates/schema or plugin-api: the oracle
must derive from contracts, not current code behavior. C01 documents are legitimate
chosen contracts, not an implementation verdict. No actual app/runtime work.

## One finite deliverable

Create a compact schema-neutral semantic corpus with independent expected outcomes:
1. GOLDEN01 checkbox S10→delivery→S11→delta→check→export-record and every named
   negative variant from the governing GOLDEN leaf.
2. Known false/empty, unknown/unsupported/redacted and not_requested distinctions,
   required-field absence, stale-history contamination and incompatible replacement.
3. Identity/generation/clock/coverage/transform integrity cases required by D03/D04,
   including duplicate labels without equating identities and invalid/missing context.
4. Representative explicit geometric inputs/expected values or unknown outcomes
   for schema/rule records, using declared fixture tolerances, not product defaults.

For each case: stable case_id, clause IDs, preconditions/input facts, operation or
declared record, expected semantic result, invalid reason where relevant, evidence
requirements and protected distinction. Include all envelope families D03 names.
Separate inputs from expected answers. Keep it small and complete for these rules;
no broad fuzz campaign, test framework or future platform corpus.

This is test oracle data, NOT the wire schema. Use a clearly labeled oracle format
and logical paths; Integration owns serialization details, executable validator,
JSON Schema and actual wire fixtures. Do not fabricate serialization keys/options
that C01 has not fixed. Integration maps facts losslessly into its single schema.
Do not loosen expected semantics to match a failing implementation. A real contract
ambiguity returns to root with exact clauses; do not silently pick a product fork.

## Allowed writes and validation

- `fixtures/golden-oracles/**`: compact inputs, expected outcomes, source mapping/readme.
- `docs/plans/ui-blueprint/receipts/S01-oracles.md`: short terminal receipt.
No edits to actual `fixtures/golden/`, source, Cargo, specs, other fixtures or root
coordination docs. Do not read agent-evaluation answer keys from real-case packs.

Validate JSON/local links, case ID uniqueness, clause coverage and expected-value
arithmetic where applicable with a small task-temp check. Do not run Rust suites,
native/browser tools or generate persistent raw check logs. Source-controlled
oracle data is the requested deliverable, not auxiliary evidence.
Git stage/commit lease initially absent: finish/check → waiting_resource for
serialized checkpoint grant. Root then provides the accepted corpus to Integration.
Return exact paths, covered clauses, unresolved questions and concise receipt.
No subsequent packet or peer messages without root follow-up.
