# Agent skill and prompt installation
- Node type: leaf; domain: `uib.agent-skill`; contract: `UIB.AGENT-SKILL@4`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: the operator approved this chat's six-step plan on 2026-10-13;
  the later instruction “Теперь давай подключим skill к установщику” authorizes
  integration after inspecting existing work, with temp-only checks and no publication
  or real installation. The earlier documentation-only stage is checkpoint2afd3a01.
- Read when: maintaining the distributed skill or its prompt installation path.
- Do not read when: using unchanged engine or collector internals.
- Requires: [BOUNDARIES@1](../product/boundaries.md), [CLI@20](../product/cli.md).
- Conditional requires: linked recipe contracts and [Mac distribution](macos-distribution.md).
## AGENT-SKILL.SCOPE — Approved plan and current stage
The approved outcome is one portable `ui-blueprint` skill and matching CLI,
versioned installation, README instructions and verified installation/use.
Current-branch work remains single-agent except the explicit cold-chat test below.
Integration and isolated lifecycle checks are complete. The operator subsequently
authorized isolated Codex/Claude discovery, then recording its results and readiness.
On 2026-10-14 the user authorized push master, real installation under ~/.local
and ~/.agents/skills/ui-blueprint, then testing installation and use in a new chat.
Release publication/licensing and unrelated changes remain excluded. This does not
resume the older goal; the new-chat test must use the pushed consumer instruction.
## AGENT-SKILL.WORKFLOW — Portable operating instructions
The entrypoint is [skills/ui-blueprint/SKILL.md](../../../skills/ui-blueprint/SKILL.md).
Keep it short and automatically discoverable for UI Blueprint observation,
measurement, requirement checks, saved comparisons and drawing preparation.
Bundle conditional references and Codex UI metadata in the same skill directory.
All operational links resolve within that directory; no checkout, personal path,
private Page, fixture script or another installed skill is a hidden prerequisite.
An explicitly selected compatible executable bundle and operation-specific runtime
access remain real prerequisites. Missing access is reported, never manufactured.

Use [ANALYSIS@2](../product/analysis.md) for factual query/check recipes,
[CLI-DIFF@2](../product/cli-diff.md) for compatible recorded comparisons,
[WEB-FIRST-USE@4](../product/web-first-use.md) for Web URL/component acquisition,
[NATIVE@5](../product/native.md) for the prepared ordinary AX caller and
[CLI-EXPORT@3](../product/cli-export.md) plus its drawing closure for export.
Native connection preparation is explicitly caller-owned in this first skill;
do not advertise a shipped PID-to-connection bootstrap or developer test helper.
Observation, coverage, units, frame kinds, identity and unavailable results retain
their product meanings. Examples do not create UI requirements or action authority.

## AGENT-SKILL.INSTALL — Opt-in matching release

One committed revision supplies the flat CLI archive plus a separate versioned
skill archive and both pinned hashes. `--skill-dir ABSOLUTE_PATH` explicitly opts
into installation at that exact host-selected location; there is no Codex/Claude
path default, discovery or duplicate installation in the installer. Host documentation
selects its current discovery directory; this machine's legacy `.codex` path is not
promoted to a universal default. Custom locations remain caller-authorized.
Store skill files inside the corresponding managed version's `skill` directory;
the external skill symlink points through the same `current` link as the CLI.
Register its exact destination, reuse it on updates and forward it in version dispatch.
Refuse an incompatible legacy installer when a skill is registered. Verify existing
managed skill hashes/inventory/modes before activation or removal; preserve edits,
foreign destinations and unrelated files. Uninstall removes only the owned link,
receipt and verified files, never the shared skill parent. Never recursively delete.
An interrupted install may leave a verified inactive version, but no successful
combined-install claim or active switch before both payloads are ready.
The prompt authorizes both locations. Before a published release exists, use one
pinned GitHub master commit and the existing local build recipe with already-installed
prerequisites; --local-release ABS consumes its matching rendered candidate offline.
No implicit toolchain install, profile edit, privilege escalation or UI access.
Source builds are labelled local/unsigned, not public releases. Later release installs
retain their existing path. New-chat invocation is required evidence for this stage.

## AGENT-SKILL.RETENTION

All task-created images live in system temp and are never agent-cleaned. Copy any
tool-managed image there without changing its original; disclose retained originals.
Display local images inline using their absolute temp paths. Use system temp for
transient non-image files; remove only run-owned non-image files after use and verify
removal. Never recursively remove image-containing directories. Persistent outputs
require explicit authorization for their location and purpose.

## AGENT-SKILL.ACCEPT — Evidence by stage

| Stage | Required evidence |
| --- | --- |
| Skill/documentation | Frontmatter/UI metadata valid; internal links portable; examples match current command inputs; changed links/routes and diff whitespace checked |
| Installation | Temp-prefix fresh/repeat/update/conflict checks, damaged archive/platform/path failures, matching installed CLI known-result example and skill resources outside checkout |
| Host discovery, verified | [Recorded versions/modes](../../development/macos-release.md#skill-discovery-verification--2026-10-13): Codex skills/list and Claude initialization list the installed skill; negative controls exclude cached presence |
| Publication, excluded | Published URL/Gatekeeper and real user-session invocation remain separate from isolated discovery |

Earlier installation discovery used control requests, not product proof.
The authorized cold test installs and uses saved data; public release is separate.

## Agent-composed drawing — direct user direction2026-10-14

Keep literal CLI export; the primary human-drawing workflow now teaches a fresh
agent to compose a selective prompt from Rust-verified geometry. Bundle the English
strategy, adaptable template and review criteria in the existing export reference;
labels follow the user's language. Do not embed prior screenshots/prompts or A04
answers. The user explicitly requests fresh-chat installation and forward testing:
one saved-source case and one generation per initial iteration, with no inherited
conversation or successful example. Isolated system-temp installation is authorized
for this test; no application changes, new public release or old goal resume.

AGENT-SKILL-003: authorized pinned-source bootstrap resolves the empty-Releases
onboarding gap. AGENT-SKILL-004 adds the agent-composed drawing/cold-test workflow.
CLI/wire/analysis/export semantics and public release boundaries remain protected.
