# Rust engineering rules

Read for Rust source changes. This is an engineering contract, not a product
specification. Product work starts at [the registry](docs/specs/README.md);
toolchain and command selection follow [Rust development](docs/specs/development/rust.md).

## Basis and scope

Adapted from `ai-friendly-search-engine/RUST.md` and its Rust development
infrastructure contract, as approved for this repository on 2026-10-06.
The donor's search crates, service transports, runtime choice, database flows,
benchmark tools, and deployment rules are not inherited.

Reference material: [Rust Style Guide](https://doc.rust-lang.org/style-guide/),
[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/), and
[the Rust Book](https://doc.rust-lang.org/book/).
These references explain language practice; they do not define product intent.

## Formatting and readability

- Use default rustfmt formatting; do not hand-format against it.
- Prefer clear expressions, early validation returns, and readable loops.
  Iterator chains are useful when they clarify the transformation.
- Keep functions focused on one domain step. Roughly 10–40 lines is a useful
  target, not a limit requiring mechanical extraction.
- Review functions over about 60 lines and modules over about 300–400 lines
  for distinct responsibilities. Line count alone does not justify a refactor.
- Helpers should add domain meaning, validation, adaptation, error context,
  or reuse; avoid pass-through wrappers with no responsibility.
- Comments explain invariants, rationale, safety, or non-obvious cost.

## Pure logic and explicit effects

- Keep deterministic transformations independent of filesystem, network,
  clocks, process control, platform handles, and runtime setup.
- Pass inputs explicitly and return results. Prefer immutable inputs and
  returned values over shared mutation.
- Local mutation is appropriate for sorting, buffering, builders, or hot
  paths when it improves clarity or measured performance.
- Avoid global mutable state and hidden background work in constructors.
  Make loading, spawning, starting, and stopping explicit operations.
- Keep adapters thin: decode and validate, invoke logic, map results/errors.
- Inject clocks or other nondeterministic inputs where behavior depends on them.

## Ownership, types, and errors

- Use structs for records, enums for closed states, and newtypes for identifiers
  or units that must not be confused. Avoid ambiguous boolean parameters.
- Use `Option` for absence and `Result` for recoverable failures.
- Prefer borrowed strings and slices for local work when lifetimes stay clear;
  use owned values at genuine ownership boundaries.
- Do not clone large values or add `Arc<Mutex<_>>` merely to silence the borrow
  checker. First establish who owns the data and who needs access.
- Keep serialization/transport types distinct from internal representations
  when their responsibilities or compatibility requirements differ.
- Libraries expose typed errors with actionable context. Application boundaries
  may add contextual error reporting without requiring a particular crate.
- Do not panic for malformed external input or ordinary runtime failures.
- Production `unwrap`/`expect` requires a local, obvious, documented invariant.
  Tests may use `expect` with useful failure messages.
- Preserve failure context; do not silently substitute success or empty data.

## Modules, crates, and APIs

- Organize by responsibility and data ownership. A new crate needs a real
  boundary; a roadmap item or file-size target is not sufficient.
- Keep `lib.rs` and `main.rs` focused on module exports and composition.
- Default to private items; use the narrowest visibility the consumer needs.
- Prefer plain data and functions. Methods should protect invariants or express
  a cohesive operation, not collect unrelated utilities.
- Introduce traits at real substitution boundaries, not for every struct.
- Use dynamic dispatch when a boundary benefits from it; measure before adding
  complexity to eliminate or introduce it in a hot path.
- Document stable public APIs, their errors, invariants, and relevant examples.

## Concurrency and blocking work

- Do not introduce an async runtime until an authorized I/O requirement needs it.
- Keep blocking operations off an async executor's worker threads. Use the
  selected runtime's supported mechanism when an async adapter needs them.
- Do not hold shared locks across external calls or `.await` without an explicit,
  justified synchronization design.
- Bound queues and concurrent work; give spawned work a clear lifecycle owner.
- A timeout or cancelled future does not prove an underlying blocking operation
  stopped. Handle completion and cleanup according to the operation's contract.
- Product deadline values, retry behavior, and cancellation results belong in
  the governing feature specification, not implicit runtime defaults.

## Performance and determinism

- Start with clear algorithms and data structures; optimize a measured bottleneck.
- Avoid unnecessary allocation and large clones on measured hot paths.
- Do not expose incidental hash-map iteration order where stable output is
  required. Keep fixture results reproducible.
- Compare performance over the same inputs and configuration. Correctness and
  performance are separate checks; one timing sample is not a regression proof.
- Keep test and benchmark helpers in test modules, `tests/`, or `benches/`.
  Select a benchmark harness only when a concrete measurement needs it.

## Unsafe and FFI

- Prefer safe APIs. Do not introduce `unsafe` as a routine optimization.
- If required by FFI or a demonstrated constraint, isolate it behind a small
  safe interface. Document validity, ownership, lifetime, thread, and cleanup
  invariants beside the unsafe operation.
- A safety comment is not evidence by itself: verify each invariant with the
  relevant API contract and focused checks.
- Do not weaken a crate's existing unsafe policy just to make a change compile.

## Dependencies and diagnostics

- Add a dependency only for a concrete responsibility; inspect its feature and
  target impact. Avoid blanket feature activation.
- For a workspace, centralize shared dependency versions in the root manifest
  unless a member has a documented reason to differ.
- Keep diagnostic context useful and bounded. Do not put secrets or full
  sensitive payloads in ordinary errors or logs.
- Machine output, exit codes, log destinations, and verbosity are product
  contracts when exposed publicly; do not invent them during a style change.

## Compiler repair and verification

- Use `cargo check` for the affected package/targets as the fast edit loop.
- Fix the first actionable compiler error locally before broad refactoring.
- If repeated small patches do not resolve it, inspect the exact ownership,
  types, feature configuration, or generated code involved.
- Do not mix compiler repair with unrelated dependency or behavior changes.
- Do not add broad lint suppressions. A narrow exception needs a local reason.
- Leave no new unexplained compiler/Clippy warnings or known test regressions.
  Report any unresolved check accurately.
- Use the command scopes and acceptance mapping in
  [Rust development](docs/specs/development/rust.md); documentation-only edits
  do not require a Rust build, and local logic does not automatically require
  a full workspace test run.
