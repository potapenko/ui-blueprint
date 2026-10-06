# Specification registry

- Node type: root
- Status: Active
- Revision: 2
- Read when: selecting contracts for product or development work.
- Do not read when: an already selected, current contract fully governs the task.

This registry routes work; it does not approve product proposals.
Global product-truth governance owns authority, change control, and evidence
rules. [Repository instructions](../../AGENTS.md) and
[Rust rules](../../RUST.md) contain only local routing and engineering practice.

## Development contract

| Domain | Contract | Authority / stability | Read when |
| --- | --- | --- | --- |
| Rust development | [Rust development](development/rust.md), `DEV.RUST@1` | Active / Evolving; documentation setup approved by the user on 2026-10-06 | Toolchain, Cargo, dependencies, targets, features, or Rust verification |

Approval covers the documentation foundation. It does not authorize application
implementation or mean that a Rust workspace has been built and accepted.

## Product contracts

| Material | Recorded state | Read when |
| --- | --- | --- |
| [UI Blueprint general specification](../ui-blueprint-spec.md), `UIB.TZ@1.4` | Active / Evolving; confirmed by the user on 2026-10-06 | Product scope, architecture, behavior, P0–P7 and D01–D07 |
| [Engineering visualization and ImageGen guide](../engineering-blueprint-guide.md), `UIB.DRAWING@1.1` | Active / Evolving; confirmed by the user on 2026-10-06 | DrawingBrief, engineering drawings, or ImageGen preparation |

The user's direct confirmation supersedes their former document-level Draft
status. This is an authority change, not proof of implementation or release.
Required behavior governs; explicitly open decisions, preliminary names, and
future phases retain their stated meaning. D01–D07 resolve within approved
implementation scope, not through unrelated product invention.
The general specification governs product boundaries; the visualization guide
governs export detail within those boundaries. Neither overrides global safety.

Contract delta `UIB-AUTH-001`: prior Draft 1.3 / Draft 1.0 become Active 1.4 / 1.1
on the user's 2026-10-06 confirmation. Product rules and acceptance scenarios are
preserved; no schema wire version, released behavior, or implemented API changes.
Field-level `draft` values and historical source labels retain their meaning.

[Development plan](../plans/ui-blueprint-development.md), `PLAN.UIB@1`, is a
planning deliverable awaiting execution approval. It decomposes these contracts
and cannot weaken them. Its registry records proposed work, not running tasks.

## Routes and ownership

- Rust source: [RUST.md](../../RUST.md); product changes additionally follow
  their selected product contract.
- Development environment: [Rust development](development/rust.md).
- Product specification: applicable product material or future registered
  contract, then its explicit dependencies and acceptance scenarios.
- New domain: [feature template](templates/feature-spec.md). Register the domain,
  contract revision, authority source, stability, selection conditions,
  dependencies, precedence, and accepted/released baseline when one exists.

Keep future specification nodes at most 100 physical lines; split substantial
contracts into routed children. Existing imported documents are preserved source
material, not silently rewritten to fit the new tree.

Do not preload sibling domains, source catalogs, or historical plans merely
because they appear in an imported document. Follow requirements applicable to
the selected task. No product source catalog is needed for a behavior-neutral
edit to these development instructions.

## Provenance

The separation of `AGENTS.md`, `RUST.md`, development contracts, and feature
templates is adapted from `ai-friendly-search-engine`. Routed nodes, revision
metadata, and separation of authority from release evidence also draw on
`swiftui-semantic-audit`. These are structural references, not product
dependencies; their contracts and release history are not imported.
