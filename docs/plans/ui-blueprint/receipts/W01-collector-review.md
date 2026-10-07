# W01 collector source review

Reviewer: retained collaboration `/root/web_collector_review`, fresh context,
read-only, no agents/runtime/network/mutations. Artifact: saved
`5bce7c6b71d3f2a495dab5826460a600e83533d4`. Verdict: **reject**.
Consumer: repair before collector acceptance and later H01 integration.

Source and pinned contracts were reviewed before author handoff/receipt. Initial
observations identified node continuity, per-reply admission and frame-tree cost.
Receipt reconciliation retains two actionable findings; opaque frame-tree cost
is disclosed as a limitation, not a third defect. All reviewed source/manifest
hashes match the candidate and receipt; no staged, unstaged or untracked drift in
the reviewed set. Reported 24 Rust and 11 mock-script passes are author evidence;
reviewer ran no tests and did not qualify live Chromium.

1. **W01-COLLECTOR-R1 / P1**, `plugins/web/src/collector/acquire.rs:114–115`:
   final verification covers target/loader/root document but not selected nodes.
   A node removed after its read during collection of later nodes can publish
   Current. Pinned Chromium evidence shows resolveNode does not establish frontend
   bindings, and removal events may be suppressed under unmapped parents. Restore
   current-node continuity before publication or return appropriate unavailability.
   Governing requirement: UIB.D04.CONTENT, current source/document/ref continuity.
2. **W01-COLLECTOR-R2 / P2**, `plugins/web/src/collector/io.rs:184–188`:
   transport admission omits max_reply_bytes. A reply above that cap but below
   io_read_bytes and CDP message limit can be received/retained before rejection
   at lines 205–209. Enforce or validate an effective acquisition/message bound
   before dispatch, accounting for framing. Governing requirement: UIB.D05.CONTENT,
   acquisition bounds rather than post-hoc truncation/rejection.

Canonical channel ownership, cumulative output accounting, source separation,
redaction and cleanup distinctions are present. Bootstrap, H01 integration, live
Chromium/read-only qualification, broader projections/frames and D06 remain open.
Same reviewer owns affected repair recheck; no replacement or broader review.
