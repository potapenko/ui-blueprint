# H01 Native reaping repair recheck

Reviewer: retained `/root/host_process_review`; read-only, source first and author
evidence second, no runtime/tests/mutations. Verdict: **accept_with_residual for
the finite Native R1 repair only**. Full H01-PROCESS-R1 remains open for Core.

Saved artifacts: Native54da088807e08b977562842b9095cf164accc310 and Core shared
declarationb22b05af72c29bae4bb3c72785f831e0dbfed8a2, both pushed. Scoped source/API
matches those revisions; no staged or related dirty drift. Unconnected Core and
Web changes were excluded. Same pinned contracts apply without semantic delta.

Initial observations, before new author documentation/receipts: Native reads
SIGCHLD policy, refuses incompatible disposition/flags before spawn, and retains
an ownership-lost wrapper after detected post-spawn drift. Loss is permanent:
restoring signal policy does not re-enable PID wait/signal authority. The shared
trait contract requires continuously compatible policy and exclusive reaping;
point checks do not claim atomic protection against same-process native code.
No actionable issue remains in that mechanism under its supported invariant.

New source tests cover incompatible-policy refusal, unchanged policy and resource
counts, no child creation, actual automatic reaping and irreversible loss after
policy restore. Source inspection does not establish execution by the reviewer.

Stage2 reconciled saved host-process documentation and H01-process/H01-host API
receipts. All38 saved shared files equal the provider manifest; before/after
manifest hashes match da4837e06106e88d21f41f761d5d28dca49e12c64a30ac3d72da4f2ed7f4270a.
Only the agreed API changed between phases. Four initial focused tests and two
affected forwarding tests remain attributed Native execution evidence; source
checks support their stated scenarios without a new reviewer run.

Remaining mandatory acceptance consumer: connected HostDomain/RuntimeHost must
establish the lifetime invariant, stop dispatch after loss and retain quarantine/
grants while preserving ACKed results under D02.LIFECYCLE. This is a concrete Core
integration dependency, not an unchanged Native defect. Full R1 and H01 are not
closed by this receipt; the same reviewer remains available for affected consumer
recheck after saved Core implementation and evidence exist.

After acceptance, Native confirmed cleanup of its two explicit run-owned temporary
trees uib-h01-process-v2hfxco_ and uib-h01-reaping-kghf73oo under the recorded system
temporary directory. Before removal it found no active consumers/open files; both
trees are absent afterward. No source/docs/Git/other evidence or user process was
changed. Saved-source identity reconciliation above remains the durable evidence;
temporary manifest/log/binary paths in historical author receipts are now cleaned.
