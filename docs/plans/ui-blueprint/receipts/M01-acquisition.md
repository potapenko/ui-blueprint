# M01 admitted acquisition implementation receipt

Packet M01-acquisition@69d1e63; Restore registered a0281dff profile. Reuse full
registry11 → D05@4/Native acquisition@1 and explicit M01 closure; current leaf and
helper review read. No spec delta, SDK/live/UI or new permission. Source baseline
7c6e078; parent5f52cb5, unchanged Control/config boundary.

Declared actual write subset before edits: plugins/macos/NativeAcquisition.swift,
NativeJSON.swift, NativeArtifacts.swift, HostProtocol.swift, HostHelper.swift,
README.md; tests/bridges/native/Collector.swift, WindowAX.swift, prove.py, README.md;
fixtures/native/Observe.swift (shared collection/capture); host_helper/ProtocolPeer.swift
and check.py; new acquisition/Checks.swift and check.py under tests/bridges/native;
docs/development/native-helper.md; this receipt. No other source owners are open.

Concrete external caller dependency returned to root: sizing.py and capture_lifecycle.py
must pass explicit limits; fixtures/native/script/build.sh must link the shared
source, and standalone Observe main needs its explicit-limit caller adaptation.
Root granted these exact three supporting paths in saved d4b0c92; standalone
Observe caller adaptation is included. Only mandatory limit args/build wiring,
no expected/stimulus/fault/UI changes. Test input acquisition/profile.json is also
part of the actual synthetic-check subset, containing explicit caller values.

Status: checkpoint_ready; exact21 paths below await root's short Git lease.
Registered source slice and assigned nonvisual checks completed; independent
review and actual SDK/H01/live acceptance remain open. Existing helper7c6e078
review evidence/temp is retained unchanged.

## Actual implementation and reuse

NativeAcquisition validates every mandatory explicit profile integer, admits
ranged arrays/CF types/text/action/batches before additional copying, and checks
image dimensions/area/nominal and returned row-byte footprint. Returned image
size must equal the admitted requested size. UTF-16 count precedes UTF-8 sizing;
the conversion buffer plus owned String are both charged, including batch overlap.
Requested/admitted/actual copied byte counters remain distinct. No string prefixes,
lossy conversion or image downscaling satisfy a cap.

WindowAX's fixed field schedule, per-attribute availability and secure-value
exclusion are reused; unknown identity classification cannot request AXValue.
Sample and window collection share ranged-array/value admission. Exact window
count is checked before and after enumeration; over-limit/unresolved binding
returns target_unresolved without a prefix match or other-window content. Sample
traversal also counts duplicate/cyclic entries while retaining distinct handles.

NativeJSON charges existing canonical object/string construction, including
wrappers and repeated Context, before constructing/inserting output. It uses the
existing Foundation codec with a reserved LF-inclusive sink, not a second parser/
schema. Complete failure envelopes are reserved before risky collection; rejected
graph/encoder state is discarded and the same output buffer reused. No partial
frame is sent, and a failed FD write is not followed by an attempted second reply.

NativeArtifacts caps ImageIO callback bytes before writes, uses exclusive/no-follow
own temporary files, and publishes via no-overwrite linkat only after successful
Finalize and unfailed writer. Short writes/deadline/refusal latch failure; only its
own partial path is cleaned. Parent still owns leftovers after abrupt child exit.
CaptureLifecycle keeps its existing callback/lease behavior and parent-owned H01
admission. All helpers remain directly parent-owned; Core APIs/Control unchanged.

Private configuration now requires acquisition_limits and optionally requests
acquisition_evidence. The concrete protocol-test configuration is748 bytes, within
unchanged4032; no Core config/build dependency remains. To support AX evidence and
capture in one operation without conflicting creation/overwrite, the trusted private
operation container holds fresh0700 ax/ and capture/ subdirectories. H01 payload_ref
is capture/capture.png relative to that container; legacy capture.png/window.png
names remain unchanged. Caller owns container association/retention/cleanup and
uses a fresh container for a later operation. No path derives from UI data.
Ordinary AX creates no artifacts; optional proof sidecars contain bounded metrics/
existing capture metadata and never imply parent ACK. Both real capture permission
and the synthetic-only pixel policy remain required; no real-user export policy.

The legacy Collector and H01 share these owners. The historical standalone Observe
keeps its noncanonical AX diagnostic reporting; its shared capture/PNG path receives
the explicit profile. It is not relabelled a fully guarded canonical H01 adapter.
Current legacy drivers only gain required limit arguments/build wiring; no expected,
stimulus, UI, permission or injected fault semantics changed. Their live branches
were not run, and historical prepare.py products remain historical.

## Actual checks and identity

Apple Swift6.4, Swift6 language mode, arm64 deployment macOS14.0. Final actual helper,
legacy Collector, standalone Observe, ProtocolPeer and AcquisitionChecks all compiled
without warnings/errors. Initial typecheck exposed two CGDataConsumer Swift-label/
optionality mistakes and unsafe-cast warnings; those were corrected before checks.
No SDK/live failure was inferred from compilation. No Cargo rebuild was needed.

Final protocol suite:33/33 pass, including required/malformed acquisition-profile
refusal before SDK access. Canonical failure responses use the actual shared builder
and codec; real helper executes only pre-acquisition refusal cases. Real Ticket7
remains distinct from host operation19, and original correlation/framing/deadline/
cap-including-LF scenarios stay covered. Every owned protocol child was reaped.

Final synthetic suite:137 assertions passed. Owned CF values cover exact/over-limit
text/Unicode/invalid conversion, cumulative/batch/action limits without hidden partial
copies, array paging/count/entry/cycle-budget behavior and unsafe value-classification
gates. Existing native scalar conversion preserves false/empty/redacted and refuses
nonfinite values. Builder refusal precedes construction; bounded codec refuses overflow
without publishing a prefix. Pixel tests cover checked/nonfinite/overflow dimensions,
actual footprint and requested/returned size mismatch. Tiny synthetic CGImages exercise
ImageIO cap equality/refusal, wrong format/oversize refusal, short writes, failed
Finalize, no-overwrite/symlink checks, separate channel directories and own-partial-only
cleanup. These use nonvisual CF/Foundation/CoreGraphics/ImageIO, no live AX/SCK/UI.

The existing76-node D05 sample was charged and re-encoded with exact structural
equality, all original properties/values/edges/coverage retained, then passed the
real canonical validator. Observed sample counts remain27143 logical slots and
257900 repeated string bytes. This proves bounded-codec preservation of saved data;
it is not fresh AX collection or proof of a live WindowAX traversal/SDK timing.
Source sample SHA2569b261ac1966dd53e7f6e8233d1bfc2368fdda4d5d048b700cc38c6475aa8f0aa.
Final synthetic run: synthetic-_d1erbae under the task-temp below. Earlier synthetic
waves passed and were expanded only for affected/missing boundary cases.

Five edited Python drivers parse; build.sh syntax passes. Documentation local links,
scoped whitespace and routes checked. No broad suite, live driver, UI application,
ScreenCaptureKit request, permission change or B−3801 retry was executed.

Reused validator SHA256119842ca9b12b7f2a9b0f2d4adbc1276f2c83e2eae5df983370fb015d14ddf15
from the retained M01-host-helper temp. All16 actual validator source/manifest inputs
still match that saved build; manifest SHA256
fdd17e991e85294c8ebc3921ebfd315e5bf68b52a599eea87b3448ed844be8ae.
Core protocol/native_binding bytes match saved5f52cb5; their hashes are
6f15af73f5d0824aa5a1ef6a326984733c4c83cf9daebc1db03520b8a181df69 and
f93f3a161504ac3495edf8e029ce86bd3f3aac4b7143dbbda0161cd53f56bc6b.
Own17 source/fixture/build-input manifest SHA256
82850f96b34df0a15255522d426099618bcdc7d9e2b231363794ff049ab161dd.

Final binary SHA256:

- native-host-helper: d6c47c45368e86a267f18355c048883b1d1a72ea462b4e4eaad726cb3d199163;
- legacy-collector: c6481027f8be8322a99ba128c773ec3a2c7ae59a78277eb880436d755241a404;
- legacy-observe: baac1eccb81944aadc61efe6baa2f358019b8f470cb8b51cb48103b9599a03cb;
- protocol-peer: 4a03fb411ee2ff4d7980ffdbe93ece663c5e4ffe5199c98bc1cbc2fce7ae0d09;
- acquisition-checks: 53034e70ad06f41401ad6515f56ef12f28d08f164448e8d406b2da7467d4945f.

## Exact checkpoint set, retained evidence and residual

The exact21 paths are the17 entries in own-source-hashes.json plus the four docs:
plugins/macos/README.md; tests/bridges/native/README.md;
docs/development/native-helper.md; this receipt. Source entries are:

- plugins/macos/NativeAcquisition.swift, NativeJSON.swift, NativeArtifacts.swift,
  HostProtocol.swift, HostHelper.swift;
- tests/bridges/native/Collector.swift, WindowAX.swift, prove.py, sizing.py,
  capture_lifecycle.py;
- tests/bridges/native/host_helper/ProtocolPeer.swift, check.py;
- tests/bridges/native/acquisition/Checks.swift, check.py, profile.json;
- fixtures/native/Observe.swift and fixtures/native/script/build.sh.

Task-temp: /var/folders/px/srfnff157mg33175_4y8yrnr0000gn/T/uib-m01-acquisition-eshyh_wk.
Native owns its manifests/binaries and synthetic files for immediate root/reviewer/
M01 consumption. Retain until accepted review/handoff or explicit root cleanup;
no logs/binaries or generated run evidence are staged. No source freeze, active
helper, UI lane or Git lease is held at readiness. Other owners' changes untouched.

Opaque initial AX/SCShareableContent/CGImage and Foundation/ImageIO internal costs
remain explicit; these controls do not establish a global Swift/SDK/RSS cap.
Independent source review, actual helper-through-H01 SDK resource/lifetime/ACK proof,
production privacy/general-app identity and all unchanged positive F02/M01/D05/D06
conditions remain open. Source bounds and synthetic results are not SDK qualification.
No next runtime packet or additional agent was started.
