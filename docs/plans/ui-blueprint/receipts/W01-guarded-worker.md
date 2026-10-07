# W01 guarded Web composition receipt

Status: real guarded synthetic-peer proof ready for checkpoint/review; not full
H01/live acceptance. Authority [packet](../packets/W01-guarded-worker.md), transfer25bdae2,
dependency amendment9183eb0 and approved PLAN.UIB@1. No browser/SDK/UI grant.
Prepared module/config d9de0a60a22b0551768e96474547548bb04ca4e3 was explicitly uncompiled
then; the evidence below supersedes that pending state, not unrelated acceptance.

## Basis and actual boundary

Reuse registry10 → product/development D02@2/D03@2/D04@1/D05@3/MEMORY@2/WORK@1/
D06@1/D07@5 and full current Web/H01 identity/scope/exchange/model/privacy/lifecycle/
publication closure, WEB-PILOTS/PILOTS, RUST/DEV.RUST@2/QA. Mode Restore; no contract,
wire0.1/analysis0.2, quota or feature-default delta. Guard review64cbec9 and Core
source review support this work but do not replace its actual caller proof.

WebSession uses real begin_observation/ObservationRun Ticket, worker clock/origin,
admit_observation, FixedOutput, receive_channel and publish. Core supplied actual
attach_web/submit_web_observe and parent ObserveReady/Permit; no Native binding,
prerecorded response or fake ACK. Request+selection configuration enters the
producer; immutable trusted setup fixes endpoint/binding. Existing Transport→CDP→
Collector and log filter execute inside the installed GuardedAllocator/watchdog.

Callback encodes into fixed output under publication allowance, drops original
Document before ordinary-quota canonical receive, then waits for actual parent
Frame/Commit/ACK. Acknowledged follows publish success. Callback errors preserve
HostError; unfinished ObservationRun Drop-cancels. No second graph/parser/host,
parent decode, new refs JSON or uncharged ref cache. API in [handoff](../../../development/host-web.md).
Default Web feature remains off; actual source IDs stay in canonical Snapshot data.

## Actual scope — six PASS

Five tests spawn the real guarded worker through RuntimeHost/DarwinPlatform; the
sixth tests the pure private-configuration decoder.

| Scope | Executed result |
| --- | --- |
| Begin/Permit/Ticket/ACK/reuse | Actual Permit delayed by forwarding wrapper: no new CDP commands before forwarding. Prior Validate is host operation1; Observe operation2 uses real Ticket1. Response/permit Ticket match and actual ACK commits. Returned Snapshot IDs drive a later ref request without another search; both held results stay charged/readable through shutdown. |
| Later cancel | First Observe ACK forwarded; second withheld. Parent cancellation retains only channel0 after reap, channel1 missing. Second channel was an unsupported-capture response, not actual capture. |
| Admission/privacy | Wrong request target denied before Permit/source IO; subsequent sensitive first request yields redacted canonical bytes without source/attribute canaries. |
| Ref/document refusal | Wrong ref generation and source-loader drift yield ResyncRequired/no new frame; prior held Snapshot remains unchanged. |
| Deadline | CDP peer stalls after real admission. Original parent120ms bound yields TimedOut/no frame and owned cleanup/reap. |
| Configuration | Selection rejects endpoint injection, excessive byte length and non-object data. |

Wrappers call the real platform/predicate/owned child and only delay forwarding
actual controls; they do not manufacture controls or change signal policy. Known
small output is decoded by the test CLIENT for assertions; production parent stays
opaque. All five runtime cases reach reserved_sessions=0 through actual shutdown/
reap. Every peer socket closes/thread joins under finite accept/IO/cleanup bounds.
No process outside returned owned handles is operated/signalled.

Synthetic parameters:2 workers,64MiB child/1MiB publication,32MiB parent,2MiB input,
512KiB channel,2 completion groups,1s cleanup,8MiB/1MiB stacks. Source16 refs/
100 methods/8192 reply/65536 total reply,600 text/256 handle bytes,32 AX properties;
request32 nodes/depth8/65536 output/2s. These are not D06 performance measurements.

## Checks and pinned input wave

Provider c0abcffed886d0f33b34ddf3008a902b8e024676; Web module/config unchanged from
d9de0a6. Before/after production equality to c0abcff passed. Shared hold RELEASED
after checks/hash, not held for documentation/checkpoint.

~~~sh
cargo +1.96.0 test --locked --offline -p uiblueprint-host --features web --test web_worker -- --test-threads=1
cargo +1.96.0 check --locked --offline -p uiblueprint-host --bin session-worker
cargo +1.96.0 check --locked --offline -p uiblueprint-host --features web --bin session-worker --test web_worker
cargo +1.96.0 clippy --locked --offline -p uiblueprint-host --features web --bin session-worker --test web_worker -- -D warnings
~~~

All pass; owned six .rs files pass rustfmt --check --config skip_children=true.
No unrelated/general guard suites rerun. A shell heredoc hit no-space BEFORE later
Cargo checks; direct-command retry passed when space was available. Nothing was
deleted and no recovery of free disk space is attributed to this worker.

Checked91-input digest: 8156d3ef87a01acf4ba0bef887f72c5d8ac0e0ad35faf1a7e0bb35604221265d.
Recipe: SHA256 of compact sorted JSON {repository-relative path: SHA256(bytes)},
UTF-8/ASCII keys. Paths: every tracked file at c0abcff under
crates/{host,schema,engine,plugin-api}/src and plugins/web/src; Cargo.toml/Cargo.lock/
rust-toolchain.toml; those five crates' Cargo.toml; own tests/web_worker.rs and
support/web_worker_{data,peer,process}.rs under crates/host; fixtures/analysis/query-gap.json.
Docs excluded. Pinned c0abcff production plus saved own tests reconstruct this wave,
even when later Core work advances; no retroactive attribution to newer inputs.
Tested worker SHA256 e14516e7ae576137bfc87585e81058d5220aabfc6f838d59f0b55e6a3e2105cb.
Host Cargo9db7f79f2a67d94fbf9d84d38fa3a3a5535c354c56db306ed6434b5e3d628eef;
lockb2b39be77e62da78baffcb34afb21e9592373a301a5e99ad8bac8965ee788332 match handoff.

## Saving, retention and open gates

Next exact6 paths: four own test/helper files above, host-web.md and this receipt.
Module/config already saved; no Core/Native/Cargo/source edits staged by this save.
Runtime temp: /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-guarded-web-r4nkr9vn.
Primary temp uib-web-collector-cfh50npl stays retained as directed. Owner=this Web
worker; cleanup only own artifacts after accepted handoff. No raw command logs saved.
Independent integration review, actual Chromium/read-only invariance, opaque browser
memory/cleanup, broader projections/pixels and W01/B01–B06/D06 remain open.
