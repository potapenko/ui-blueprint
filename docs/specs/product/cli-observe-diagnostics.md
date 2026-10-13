# Opt-in Observe diagnostics
- Node type: leaf; domain: `uib.cli.observe-diagnostics`.
- Contract: `UIB.CLI-OBSERVE-DIAGNOSTICS@1`; clauses: `.INPUT`, `.DATA`, `.SOURCE`, `.BOUNDS`, `.FAILURE`, `.ACCEPTANCE`.
- Authority: Active / Stability: Evolving; accepted/released baseline: none.
- Authority source: user's usable-refusal-reasons request, root USE-OBS-DIAGNOSTICS1 shipping implementation packet for the real Web Director consumer; old default output remains protected.
- Read when: opt-in Observe diagnostics, caller/helper refusal interpretation.
- Do not read when: ordinary Observe or other commands need no new report.
- Requires: [CLI OBSERVE](cli.md), [EXCHANGE2](exchange.md), [IDENTITY1](identity.md), [PRIVACY1](privacy.md), [LIFECYCLE1](lifecycle.md), [BOUNDARIES1](boundaries.md), [D02](../development/decisions/d02-boundaries.md), [D05](../development/decisions/d05-limits.md); explicit closure applies.
- Protected: canonical NDJSON/ACK/order/bytes, legacy stdout/exits/stderr, authority/quotas/deadlines/retry/cleanup, Host wire and graph opacity, compare/other commands.

## UIB.CLI-OBSERVE-DIAGNOSTICS.INPUT
```text
observe --connection C --request R --worker ABSOLUTE_PATH --max-input-bytes N --max-output-bytes N --diagnostics-json --max-diagnostic-bytes D
```
Existing arguments retain meaning. Both new options are required together, only
for Observe; unknown/duplicate flags reject2. D is an independent explicit positive
integer1024..4096; bounds outside that interval reject before inputs/dispatch.
The fixed report's worst encoding fits1024 bytes including newline; caller-owned
bounded encoding runs after operation/cleanup, not in Host steady-state framing.
No new file, trace, discovery, collection or request limit. No CLI timeout default.

## UIB.CLI-OBSERVE-DIAGNOSTICS.DATA
After valid opt-in arguments, stderr is exactly one JSON object plus newline on
success/partial/refusal, replacing only opt-in constant error text. Default remains
unchanged. stdout is still unchanged canonical committed NDJSON only.
Exact report fields: output_version="1.0.0", kind="observe_diagnostic", exit_code,
caller_phase, caller_error, operation, terminal, terminal_error, requested_channels,
committed_channels, missing_channels, incomplete_channels, worker_cleanup,
host_diagnostic. Nullable fields use null for unavailable facts, never guesses.
caller_error is the existing fixed caller Failure code, not exception text.
caller_phase is preflight/read_inputs/validate/prepare/attach/configure_helpers/
submit/wait_terminal/shutdown/publish_stdout/complete. operation is null/attach/observe.
terminal is null/completed/failed/cancelled/timed_out. terminal_error is null or the
fixed snake_case HostError name. Channel fields are null or validated mask0..7;
request mask is from typed Request, publication masks only from Observe completion.
worker_cleanup is not_started/confirmed/unconfirmed; remote cleanup is distinct.

host_diagnostic is null when no validated Host record arrived. Otherwise exact
fields: stage,cause,code,selection_visited_nodes,remote_cleanup,send_progress.
stage names decode/begin/permit/capacity/collect/encode/receive/publish/finish.
cause is the explicit snake_case allowlist of existing DiagnosticCause variants:
host, collector_{invalid_input,limit,stale_target,resync_required,malformed,protocol,
timeout,publication_stopped,cleanup_unconfirmed}, selection_{missing,ambiguous,
incomplete,unsupported,timed_out}, cdp_{invalid_limits,invalid_binding,encoding,
envelope,budget,detached,busy,counter_exhausted,wrong_session,unexpected_reply,
uncorrelated_error,cancelled}, transport_{invalid_limits,endpoint_rejected,
logging_boundary,connect,handshake,extensions,timeout,cancelled,byte_limit,work_limit,
io,protocol,capacity,binary,closed,pending_write,no_pending_write}.
code is existing validated i32; interpretation depends on cause (Host fixed code,
protocol number, or trusted malformed-site code). Zero may mean unspecified, not
success. selection_visited_nodes is u32 only for selection causes, otherwise null.
remote_cleanup is not_required/released/unconfirmed; send_progress is
not_queued/queued/possibly_written/flushed. No free text or untrusted identifier.

## UIB.CLI-OBSERVE-DIAGNOSTICS.SOURCE
Copy fixed facts from existing HostCompletion::diagnostic(), terminal/class and
publication getters while the completion remains held; Host already validates the
record. Preserve attach failure metadata before it is reduced to caller Failure.
No Host accessor/wire change or canonical response graph parse. Only worker-owned
validated completion indicates source partial/failed channel; caller flags do not
infer coverage, node count, root cause or successful acquisition.
Missing record means cause/stage/code unavailable. Parent timeout, worker death or
Native outcomes may have no record. A generic limit/timeout category is not proof
of which configured budget or underlying source operation failed. Report actual
observed phase/status separately; no explanation invented from exit4/configuration.

## UIB.CLI-OBSERVE-DIAGNOSTICS.BOUNDS / .FAILURE
Always perform established shutdown/reap and preserve all earlier ACKed channels.
Snapshot bytes/leases stay unchanged. Capture worker_cleanup from actual caller
shutdown result, not worker terminal or remote cleanup. IO/unconfirmed cleanup keep
existing precedence/exits. Opt-in report records that final exit; successful emission
does not establish Observe success or full source/UI coverage.
Malformed arguments or too-small/large D retain fixed invalid_arguments/2 with no
structured report and no dispatch. Preflight errors after valid options get report
with unavailable Host fields. Unexpected encoding/internal or physical stderr write
failure gives fixed bounded diagnostic_output_failed/1; no raw data/partial graph.
Physical write may be partial; no automatic retry. Default paths remain identical.

## UIB.CLI-OBSERVE-DIAGNOSTICS.ACCEPTANCE
Guarded own peer proves default byte/exit preservation, success/partial/refusal/
timeout, earlier channels and cleanup. Validated-record cases prove fixed enums/
counts and unavailable subcause, exact report bounds, malformed/duplicate options,
untrusted canary exclusion and independent stdout/report budgets. No live Director
acceptance claim: Web owner repeats the real popup/list using the combined fix.
