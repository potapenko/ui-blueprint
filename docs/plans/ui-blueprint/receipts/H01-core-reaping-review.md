# H01 R1 Core consumer review

Reviewer: same `/root/host_process_review`; source first then author evidence.
Artifact50c0c95f273c51120a088f68d556b27f57bd4195; verdict **reject**, one remaining
R1 consumer defect. Accepted Native54da088/APIb22b05a repair remains closed.

P1 at saved crates/host/src/supervisor.rs:147–149 (150–152 after stage-D shutdown
guard): attach accepts every successful spawn result and queues ConfigHeader
without checking the returned child's ownership. Native intentionally returns
Ok with permanently Lost ownership when its post-spawn policy check fails. If
current SIGCHLD policy is restored before next_event, the domain predicate passes
and Core can dispatch configuration/input and accept readiness despite lost cleanup
authority. This is a detected and latched loss, not an invisible transient.

Required Restore: check returned ownership before dispatch; on CleanupPending
retain/quarantine child, reservation and backing rather than ordinary spawn-error
release. Add a focused returned-Lost/current-policy-valid case. Governing clauses:
the ProcessPlatform fail-closed contract at process_api.rs:90 and D02.LIFECYCLE,
including ownership/quarantine. Core owns the repair; no product/spec delta.

Otherwise source confirmed unique parent claim/provider binding, sticky poison,
stopped admission, retained grant/backing/claim on uncertain destruction, no PID
operations after domain-detected loss and ACKed-byte preservation. Existing isolated
proof covers post-ACK domain detection, not this returned-owner path.

Stage2 documentation does not close that gap. Reviewer independently reproduced
all63 saved inputs and digest7c192040f86531c1bba2117fe860039b3fdfcf892b0f8013c1a82a2bad17bdcb.
The11 regular tests and isolated R1 peer remain attributed author execution.
Concurrent stage-D lifecycle work was excluded except locating its line shift;
it does not implement this remedy. No reviewer tests/runtime/mutations. Full H01
allocator/helper/nonce/live acceptance remains outside scope.

## Repair recheck — R1 accepted

Same reviewer inspected bf67c98b72ab73e3abb7babc20f8adfb98110391 source before new
author proof and accepted H01-PROCESS-R1 under its supported embedding contract.
Returned-Lost children cannot reach configuration, input or readiness; uncertain
owners retain quarantine, grants/backing and the parent claim. Confirmed exits
permit normal release, and ACKed-result preservation remains intact. No remaining
actionable R1 finding. Native's accepted boundary was not reopened.

Stage2 independently reproduced all64 saved inputs and digest
9b5a0e03f919b7b2288026675eb95c2e81ee140e7b1bbfb14929eebc14e9a414.
Seven runtime tests and two explicitly executed isolated peer scenarios remain
author execution evidence; the focused source regression uses actual Native lost
state followed by restored policy. Reviewer ran no tests or runtime operations.
The caller must continuously preserve compatible SIGCHLD and exclusive reaping;
this is not protection from arbitrary same-process native interference. Full H01,
allocator/helper/nonce and live acceptance remain separate unfinished gates.
