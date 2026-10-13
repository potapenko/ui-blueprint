# Agent skill and prompt installation

- Node type: leaf; domain: `uib.agent-skill`; contract: `UIB.AGENT-SKILL@2`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: the operator approved this chat's six-step plan on 2026-10-13;
  the later instruction “Теперь давай подключим skill к установщику” authorizes
  integration after inspecting existing work, with temp-only checks and no publication
  or real installation. The earlier documentation-only stage is checkpoint2afd3a01.
- Read when: maintaining the distributed skill or its prompt installation path.
- Do not read when: using unchanged engine or collector internals.
- Requires: [BOUNDARIES@1](../product/boundaries.md), [CLI@20](../product/cli.md).
- Conditional requires: the linked product contracts below for the selected recipe.
  Reconcile the committed Mac distribution contract when integration resumes.

## AGENT-SKILL.SCOPE — Approved plan and current stage

The approved outcome is one `ui-blueprint` skill, installation of the matching CLI
and skill through one prompt, and a README entry. Work stays single-agent on the
current branch. The six steps are: register this contract; author the portable
skill; integrate it into versioned distribution; document the installation prompt;
check installation lifecycle and examples; checkpoint only task-owned changes.

Current execution adds installer/release integration and isolated lifecycle checks.
Unrelated changes remain untouched; publication, licensing, pushes, real installation
and host UI discovery remain excluded. The original approval stays in this chat;
this does not resume the older coordinated goal. README and the canonical agent
installation guide are concurrently owned by the installation-documentation chat.

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
The prompt names the trusted release and authorizes both locations. No source build,
profile edit, privilege escalation, permission grant or UI access. First target is
qualified Apple Silicon macOS. Public URL/discovery acceptance stays unclaimed.

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
| Publication, excluded | Real published URL installation and actual Codex discovery in a new chat; never inferred from local file checks |

No Rust changes, image generation, live collection or independent-agent evaluation
are authorized here. Synthetic lifecycle and real saved-data checks stay separate
from installed-host discovery, downloaded Gatekeeper and public release acceptance.

## Change record

Traversal: registry → AGENT-SKILL@2 / MAC-DISTRIBUTION@2; reuse their unchanged
BOUNDARIES/CLI/development dependency closure. Evidence: committed installer,
release builder and isolated release tests; current unrelated Rust edits excluded.
AGENT-SKILL-001 established the portable workflow. AGENT-SKILL-002 is the explicit
user-approved additive installation integration above; CLI/wire/analysis/export
semantics, ordinary no-skill installs and legacy candidate verification stay protected.
