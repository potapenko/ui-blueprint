# Stable allocation guard source review

Fresh reviewer `/root/h01_allocator_review`; source-first observations followed by
author-evidence reconciliation. Verdict: no actionable findings/source-review
blocker within this finite stable-guard scope. Pin64cbec90de0bd1b46c8faea4efe01b2f9b3204c5;
quota_allocator SHA2562f1bf278b9265855ececed61ccb5b3f2e1904f17b1855adb6d659782b241e50e.

Source supports prospective checked charging, unchanged System pointer/layout
forwarding, zeroing, full-old-plus-new realloc reservation/rollback, deallocation
ordering, atomic concurrency, publication-reserve isolation and fixed allocation-
free fatal encoding/send followed by _exit. Private controlled-null seams preserve
fixed shipping System forwarding. Valid GlobalAlloc caller contracts remain assumed.

Stage2 reconciliation:

- 2a712aa's five method-level cases cover bounded actual allocation and injected
  alloc/zeroed/realloc null handling. Relevant guard/counter/limits/fatal/probe source
  matches the reviewed pin. Root independently reproduced its16-input digest
  4d717bc422445f92b9efa482f32c6d85d658817658c22c45eaf35f54b2c21c53.
- bdeb87e's four child-only cases cover named hostile decode families, ACKed-byte
  preservation and normal Retain/Replay; reported execution remains attributed.
- ba3c372's two cases prove fixed-writer/retained-admission refusal, with the explicit
  distinction from allocator exhaustion preserved in source and receipt.

Reviewer inspected source/docs/Git only; no builds, tests, probes, SDK/network,
external posts, file or Git mutation. Author execution is not relabelled independent
runtime verification. No whole-project review was performed.

Remaining mandatory qualification: changed manifests/installation callers and
producer wiring need affected integration review/execution. Forced allocation
failure in rejection, semantic validation and Replay remains unproved; validation
shares Decode marking and completion metadata alone cannot isolate it. Writer/
admission refusal is not dependency-allocation failure; injected null is not OS
exhaustion. Full parent inventory/supervisor, live Web/Native and D06 stay outside
this verdict. Accepted guard mechanism can be reused while its source/invariants
remain unchanged; this receipt does not accept future caller revisions or full H01.

## Rejection and direct-validation supplement

New evidence justified finite same-reviewer reconciliation; unchanged production
guard source was not re-audited. Initial source observations covered rejection
testda6330e and validator/probe5639cba before author results at6230b74. No actionable
findings. Earlier unproved-rejection/direct-validation notes above are superseded
only by this bounded supplement, not a full H01 verdict.

Rejection uses the real guarded worker, borrowed unescaped unknown key and actual
unknown_field → Error::custom rejection-string path. The test demands matching
quota fatal/Decode marker/large layout, no new frame, prior ACK bytes and confirmed
shutdown/reap. Reviewer checked all five dependency hashes. Runtime1/1 and71-input
digestfc155032…48c36 remain attributed author evidence on provider0c1bb44.

Direct validation calls the canonical public validator under the unchanged real
guard in a terminal test-only interception window. Pre-window System pointers and
charged4096-byte ballast retain separate ownership; no charged pointer can escape
globally. Callbacks add no allocation/formatting/log/lock/unwind. Positive validation
and volatile ballast controls precede validate_context → unique → BTreeSet insertion.
Tests require positive live0, negative quota/live4096/probe-phase2 and actual owned
child reap. Runtime1/1 and67-input digestc34fc3b3…067f0 are author evidence on5f52cb5;
prepared test sources stayed unchanged. No reviewer execution is claimed.

D05-WORK.PROOF clarification: these support rejection pressure and direct semantic
validator scratch refusal. Existing fixed-writer and retained-admission cases prove
their bounded failure paths. “Force failures” does not require actual OS exhaustion
or a separate allocator-OOM test in an allocation-free slice writer.

Limits: probe phase2 is not integrated from_json attribution/ACK evidence or whole-
worker accounting. Writer refusal alone does not prove all serializer/dependency
scratch or the complete publication allowance. Replay, changed producers, parent
inventory, integrated cleanup/publication, live and D06 have separate evidence/gates.
No new source-review blocker. Concurrent Core/Native changes were excluded; no
files/runtime/external state changed during review.
