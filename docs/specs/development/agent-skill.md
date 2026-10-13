# Agent skill and prompt installation

- Node type: leaf; domain: `uib.agent-skill`; contract: `UIB.AGENT-SKILL@1`.
- Authority: Active; stability: Evolving; released baseline: none.
- Authority source: the operator approved this chat's six-step skill/installer/README
  plan on 2026-10-13, then directed: “Начни со skill и документации, существующие
  незакоммиченные изменения установщика пока не трогай. Без публикации и установки
  в мои реальные каталоги.” This narrower execution boundary governs this stage.
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

Current execution covers the skill and documentation only. Existing uncommitted
installer, release builder, distribution manager, formula and their tests remain
untouched. Integrating them is deferred, not accepted or cancelled. Publication,
license selection, pushes and installation into real user directories are excluded.
The original plan and actual approval remain in this chat; this record does not
grant new authority or resume the older coordinated development goal.

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

## AGENT-SKILL.INSTALL — Integration contract, pending implementation

One reviewed release revision supplies compatible executable and skill resources.
The installation prompt names the trusted source and explicitly authorizes the
binary prefix and skill directory. Check platform, inventory, integrity and
compatibility before reporting readiness. Do not treat schema versions as a CLI
release identifier or assume that an existing command on PATH is compatible.
Repeated installation of the same unchanged set is harmless; modified or foreign
files refuse replacement. Updates preserve user edits and keep a matching set.
No automatic shell-profile edit, privilege escalation, UI access or permission grant.
First target is Codex on qualified Apple Silicon macOS; other hosts are unqualified.
Until release integration/publication is complete, README labels the prompt as
preparation and directs the agent to stop when no matching published release exists.

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
| Installation, deferred | Temp-prefix fresh/repeat/update/conflict checks, damaged archive/platform/path failures, matching installed CLI known-result example and skill resources outside checkout |
| Publication, excluded | Real published URL installation and actual Codex discovery in a new chat; never inferred from local file checks |

No new scripts, Rust changes, image generation, live collection or independent-agent
evaluation are needed for the current documentation stage. Static checks are not
behavioral skill acceptance, installed-host discovery or release acceptance.

## Change record

Traversal: registry → product routes → CLI@20, ANALYSIS@2, CLI-DIFF@2,
WEB-FIRST-USE@4, NATIVE@5, CLI-EXPORT@3 → explicit data/identity/privacy/geometry,
lifecycle/cache, D01/D02/D03/D05/D07 and drawing dependencies, including
DRAWING-PACKAGE@3, SHEET-PROMPTS@1 and DRAWING@1.2 human presentation precedence.
Existing CLI help source, packaged examples and consumer guides supply syntax
evidence. Engine changes, live QA, action recipes and release execution are excluded.

AGENT-SKILL-001: user-approved additive Evolve from developer-facing instructions
to a portable agent workflow and a planned prompt-installation contract. Existing
CLI/wire/analysis/export semantics, release ownership and qualifications are protected.
