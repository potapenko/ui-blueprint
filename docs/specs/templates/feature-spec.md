# <Feature name>

- Node type: leaf
- Domain: `<stable-domain-id>`
- Authority: Draft
- Stability: Evolving
- Contract: `<domain>@1`
- Authority source: <actual user decision or approved contract delta; pending if absent>
- Read when: <task selection conditions>
- Do not read when: <excluded neighboring work>
- Requires: <explicit links and clause revisions, or none>
- Supersedes: none
- Accepted/released baseline: none

This template is not implementation authority. Replace placeholders before
registering a real contract. Keep each node within 100 physical lines; split
larger domains into children with explicit reading routes.

## Goal and scope

Describe the requested outcome, affected users/consumers, and responsibility
boundary. Separate confirmed requirements from unresolved proposals.

## Non-goals and protected behavior

Name adjacent domains, existing behavior, and shared consumers that must remain
unchanged. Record precedence if a general and specific contract overlap.

## <DOMAIN>.BEHAVIOR — Observable contract

Assign stable clause IDs to independently verifiable rules. Define inputs,
preconditions, actions, state transitions, intermediate and final results.
Include visibility and accessibility/input expectations when relevant.

## <DOMAIN>.INVARIANTS — Conditions that must hold

Describe invariants independent of implementation. Keep measured facts,
requirements, and proposals distinct.

## <DOMAIN>.FAILURE — Edge cases and recovery

Define invalid/absent inputs, partial results, limits, errors, cancellation,
timeouts, and recovery only where relevant. Do not invent defaults to complete
the template; leave unresolved decisions explicit.

## <DOMAIN>.DATA — Public and persistent boundaries

Specify applicable API/CLI, schema, identifiers, units, state ownership,
persistence, permissions, lifecycle, and version/compatibility rules.
State “not applicable” for absent surfaces rather than designing new ones.

## Dependencies and evidence ownership

Link the governing shared contracts and implementation owners once known.
Reference the minimum design/runtime/source evidence needed for this domain.
An implementation observation does not silently become a requirement.

## Acceptance mapping

| Scenario | Clause | Preconditions / action | Expected state and result | Evidence |
| --- | --- | --- | --- | --- |
| <ID> | <clause> | <setup and action> | <observable expectation> | <focused check or pending> |

Include relevant failures, recovery, platform constraints, and compatibility.
Keep scenario expectations separate from the record of what actually passed.

## Change record

For semantic changes, record the external authorization, old/new behavior,
affected clauses and revision, compatibility impact, protected domains, and
design/QA impact. Plans and green tests are not independent authority.

## Open decisions

List only unresolved choices affecting implementation or acceptance. State
which work can proceed independently. Do not label an unresolved proposal Active.
