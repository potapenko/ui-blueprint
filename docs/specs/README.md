# Specification registry

- Node type: root
- Status: Active
- Revision: 1
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

## Imported product material

| Material | Recorded state | Read when |
| --- | --- | --- |
| [UI Blueprint general specification](../ui-blueprint-spec.md) | Draft 1.3, 2026-10-06; distinguishes user requirements from engineering proposals | Establishing product scope and selecting a product domain |
| [Engineering visualization and ImageGen guide](../engineering-blueprint-guide.md) | Draft 1.0, 2026-10-06 | DrawingBrief, engineering drawings, or ImageGen preparation |

These documents retain their original content and authority distinctions.
Their presence does not make every proposed API, crate, dependency, or phase an
Active contract. No accepted implementation or release baseline is claimed here.
For a product task, read the applicable source material and dependencies fully,
reconcile its explicit user requirements and proposals, then establish the
authorized domain contract before implementation. Do not silently promote Drafts.

## Routes and ownership

- Rust source: [RUST.md](../../RUST.md); product changes additionally follow
  their selected product contract.
- Development environment: [Rust development](development/rust.md).
- Product specification: applicable imported material or future registered
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
