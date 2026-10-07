# Native acquisition limits — engineering proposal

Status: **selected45c2667; profile registered as [UIB.D05-NATIVE-ACQUISITION@2](../specs/development/decisions/d05-native-acquisition.md)**
under [root's registration authority](../plans/ui-blueprint/packets/M01-acquisition-registration.md).
Image ownership reconciled by explicit user all-task-images rule/[M03 packet](../plans/ui-blueprint/packets/M03-popup-capture.md); numerical profile unchanged.
This document preserves the selected engineering proposal/provenance; the leaf is
normative. Source implementation, SDK/H01 proof and live acceptance remain open.
Original proposal packet: [M01-acquisition-plan](../plans/ui-blueprint/packets/M01-acquisition-plan.md),
PLAN.UIB@1/ROADMAP delegated D05 choices. Consumer: the next finite Native source
slice on helper7c6e078 and Core c0abcff. Preserve one collector/canonical schema,
D02 ownership, current F02 workload and D06@1; no SDK/live execution is authorized
by this document. Numbers below are proposed engineering ceilings, not measurements
of worst-case SDK memory and not caller defaults. KiB=1024; MiB=1,048,576.

## Evidence and unchanged requirements

Saved [D05 Native sample](../plans/ui-blueprint/receipts/D05-native-samples.md):
76 nodes/75 edges,160-node/depth9 request, eight fields,337500 wire bytes including
LF; partial coverage. Read-only analysis of its existing returned-wire.ndjson
(SHA2569b261ac1966dd53e7f6e8233d1bfc2368fdda4d5d048b700cc38c6475aa8f0aa):
observed text/action/native-role/extension values total2312 UTF-8 bytes, longest19,
maximum93/node; at most4 actions/node,71 bytes/list,19/name. This excludes unreturned
AX values and allocation overhead. Whole canonical document has3747 dictionaries,
390 arrays,9920 scalar values and13086 dictionary-key occurrences:27143 structural
slots. All string values+keys with repetition total257900 UTF-8 bytes. These are
counts of the existing serialized document, not new SDK sampling or heap sizing.

D06's F02 baseline is550×525pt/1100×1050px:1,155,000 pixels,4,620,000 logical BGRA8
bytes before row alignment. Proposed limits must preserve these exact dimensions,
fields/known values and existing node coverage. No downscaling, reduced field set,
shortened strings, synthetic replacement nodes or excluding PNG cost to pass caps.
The larger76-node eight-field sample is a separate preservation case, not a new
D06 latency workload. All unchanged latency gates and positive pilot obligations
remain; no simultaneous-saturation guarantee is inferred from independent ceilings.

## Proposed explicit initial profile

Caller supplies positive values at or below each ceiling in trusted configuration;
missing/out-of-range/overflow refuses before acquisition. The source's fixed field
schedule remains fixed, not arbitrary attribute names. Request node/depth/output/
deadline and parent admission can only tighten the resulting operation. Counters
are per selected helper/channel; both helpers remain within the existing parent
reservation/cleanup rules. Text counters count each owned copy, not unique values.

| Metric / proposed ceiling | Admission and rationale |
| --- | --- |
| `ax_windows`32 | Count AXWindows before range copy; more than32 means unresolved identity, never select a prefix match. Own two-window F02 remains covered. |
| `array_page`32 elements | Max values per AXWindows/AXChildren range call; min with remaining node/entry budget. No whole-array copy then prefix. |
| `child_entries`1280 total | Charge each returned child entry including duplicates before queue retention;8×160 bounds duplicate/DAG scan work without reducing the160 unique-node request. |
| `value_utf8_bytes`4096 | Per scalar text/role/subrole/identifier before Swift String conversion; reject UTF-16 length>4096 first, then lossless UTF-8 sizing. Saved maximum19; this is generous finite headroom, not measured maximum. |
| `action_names`32 and `action_name_utf8_bytes`256 | Check returned CFArray count, then every CFString before bridging. Derived list ceiling8192 bytes (checked32×256); saved4 names/71 bytes. No silently shortened action set. |
| `batch_values`8 and `batch_utf8_bytes`16384 | Fixed identity batch3 and value batch≤7 already fit. Preflight returned raw CF objects/types/lengths before copying any batch strings; reject an oversized batch without partial hidden copies. SDK's initial batch return is opaque. |
| `copied_utf8_bytes`262144 | Cumulative owned copies of all acquired text/action/identity values, including discarded candidates. Reserve before copy; saved retained2312 bytes is a lower bound on total acquisition, not its proof. |
| `response_slots`65536 | Cumulative response object construction: count one per container/scalar/dictionary-key occurrence before constructing/appending. Saved27143; proportional160-node baseline shape≈57143, not a proof for arbitrary payloads. |
| `response_string_utf8_bytes`1048576 | Charge all response key/value string occurrences, including constant/repeated provenance and context; charge borrowed input strings before insertion too. Saved257900;160-node proportional shape≈543000 bytes remains below this. Logical payload/shape bound, not Foundation object heap size. |
| `image_width`4096 / `image_height`4096 pixels | Finite positive scaled dimensions checked before Int conversion and screenshot dispatch; refuse larger natural output, do not resize to fit. |
| `image_pixels`8388608 | Checked width×height before dispatch. Preserves1,155,000px baseline and admits3840×2160 in principle; does not qualify that new workload. |
| `image_bytes`67108864 | Check requested BGRA8 width×4×height before capture; inspect actual CGImage bytesPerRow×height/bits/dimensions before encoding or further retention. Allows row padding beyond nominal≤32MiB pixel payload. Actual SDK allocation has already happened and is not retroactively bounded. |
| `png_bytes`67108864 | Cumulative bytes accepted by ImageIO output callback before file writes. Headroom above nominal maximum32MiB BGRA payload; not a compression-ratio claim or guaranteed PNG fit. |
| `sidecar_bytes`16384 per helper | Aggregate allowed acquisition/capture metadata output; finite counters/codes only. Existing small metadata fits; no UI strings/raw errors in telemetry. |

Unchanged: node≤160/depth≤9, configured request scope/fields, existing AX/capture
local budgets and parent overall/cleanup deadlines, two helper streams, parent
512KiB ingress/channel wire cap including LF, cumulative request allowance and
Rust worker/parent quotas. Native profile numbers do not consume or enlarge the
Rust64MiB guard and must never be presented as a Swift/SDK/RSS cap.

## Mechanisms and exact refusal points

1. **Shared AX access, Collector.attribute / collectWindowAX.** Replace whole
   AXWindows and sample AXChildren reads with GetAttributeValueCount then
   CopyAttributeValues(index,maxValues). Iterate raw CFArray entries with runtime
   type checks; no `as? [AXUIElement]`/`[Any]` bridge before count admission. Recheck
   limits/deadline between calls and before each copy; validate returned count≤asked.
   Array mutation/errors cannot establish uniqueness/completeness: unresolved
   window matching returns target_unresolved with no other-window content. Reuse
   the WindowAX bounded path, keeping duplicate/depth/omission accounting.
2. **Shared value admission, WindowAX.batch/scalar/typed and sample attributes.**
   Keep returned objects as CFTypeRef until type/count/length preflight. CFString
   UTF-16 length precedes UTF-8 sizing; CFStringGetBytes with lossByte0 and a fixed
   buffer supports exact conversion. Its NULL-buffer sizing ignores maxBufLen,
   so never call that sizing pass before the UTF-16 cap. Reserve actual UTF-8 bytes
   against per-value/batch/list/aggregate budgets before making owned strings.
   Inspect one bounded batch at a time, drop raw values promptly; no recursive
   conversion of unexpected containers. CopyActionNames has no ranged public
   equivalent: its initial full CFArray/strings remain opaque; count and admit
   before Swift copies. No prefix returns, lossy conversion or false known empty.
   If role/subrole/identifier admission cannot establish safe value classification,
   do not request AXValue. Preserve explicit unknown/refusal; redacted means a
   known protected field, not a generic resource failure.
3. **Response construction, Collector canonicalBytes/publish and HostProtocol.**
   Charge structural/string counters before every response dictionary/array/string
   insertion, including wrappers and input Context reuse; bound child edges by the
   entry budget. Use Foundation JSONSerialization.writeJSONObject to a bounded
   OutputStream over one pre-reserved frame buffer, leaving one byte for LF. Keep
   the current canonical object fields and codec; no hand-written JSON parser or
   serializer/schema fork. Stream overflow fails before any FD4 publication; discard
   candidate output and emit only a complete bounded existing failure response if
   still admissible, otherwise fail transport. Reuse the buffer after releasing the
   rejected graph; final LF goes in reserved storage without a growing Data copy.
   Foundation does not promise bounded internal serializer scratch: stream API is
   evidence for destination writes only, not proof it never materializes internally.
   The admitted object-shape/string limits constrain our inputs/retention, while
   Foundation internals remain opaque and require runtime observation. No graph
   or sanitizing parser moves into Core parent.
4. **CaptureLifecycle.capture.** Check source width/height/scale for finite positive
   values, checked scaling/rounding and integer representability; check all three
   dimension/area/nominal-byte ceilings before SCScreenshotManager. Preserve SDR
   BGRA semantics; explicitly select SDR where the public availability permits,
   retain the documented SDR default on older supported targets. After callback,
   validate actual width/height/format/row-byte product before encoding; mismatch or
   excess returns failed incomplete_scope, no downscaled substitute. SCShareableContent
   returns a whole SDK inventory with no addressed-window variant in this path:
   avoid making a second Swift array, retain only the matching window/filter and
   release inventory; do not claim pre-acquisition control of its internal objects.
5. **Collector capture encoding.** Keep the trusted new0700 directory and caller's
   ownership/retention policy. Open only a new owned temporary artifact via exclusive
   creation/no-follow; use CGImageDestinationCreateWithDataConsumer and a callback
   that checks `written + count` overflow/cap/deadline BEFORE copying/writing. Return0
   on refusal, latch failure and require successful Finalize plus unfailed writer.
   Promote only the finished owned file to capture.png without replacing an existing
   destination via exclusive link while retaining the original image path. All task
   staging/partial/final images and containing directories stay in system temp without
   agent deletion, including failure/stale/helper death; never advertise incomplete or
   stale payload_ref. Close descriptors/reap helpers; retain non-image own partial
   cleanup. Metadata uses a bounded codec sink within the sidecar cap; scratch is opaque.

Budget-limited attribute/action data becomes unknown with a bounded reason and
partial coverage, not unsupported, redacted, empty or a misleading prefix. Node/
edge limits retain honest omitted/unknown counts; failed identity stops dependent
collection. Whole-channel construction/capture overflow uses existing failed
incomplete_scope with bounded failed_step/recovery_class; timeout/permission keep
existing codes. Earlier ACKed AX remains intact if capture later fails. Missing
required known baseline data is a failed positive case, never accepted degradation.

## Metrics, source ownership and exact integration dependency

Keep fixed numeric counters: windows counted/read, array calls/entries/duplicates,
unique handles/visited/returned/omitted/unknown, maximum raw string units inspected,
UTF-8 bytes requested/admitted/copied/refused, batch/action counts and maxima,
response slots/string charge/encoded bytes, requested/returned image dimensions,
row bytes/nominal and actual image footprint, PNG bytes and refusal stage, terminal/
cleanup status. Separate cumulative work from peak live ownership and unknown SDK
cost. No values/labels/error descriptions; no unbounded trace or new logging system.
Expose proof counters through existing run-owned acquisition/capture sidecars only
when the trusted caller requests evidence; regular machine FD4 stays canonical.
Any sidecar location/retention must be caller-owned, including AX-only evidence.

Minimal edit owners: existing Collector.swift/WindowAX.swift common access+builders,
CaptureLifecycle in Observe.swift, and NativeConfiguration/entrypoint for explicit
limits. A small shared Swift admission/sink helper is justified only to keep both
legacy and H01 consumers on the same implementation; update their existing build
instructions. Legacy driver callers must supply the same explicit limits (never
silently receive new defaults). No Cargo/dependency/framework/Rust graph changes.

Exact integration delta: add required private `acquisition_limits` integer object
to NativeConfiguration with the metric keys/ceilings in the table; action-list8192
is derived from its two limits, not a separate inconsistent knob. This remains
inside Core's opaque4032-byte config capacity; Native caller builds it.
For optional proof sidecars, add a private `acquisition_evidence` boolean, explicitly
false for ordinary operation. True requires the existing trusted artifact_directory
and caller cleanup ownership; it must not enable pixels or broaden scope. Reuse
that destination for bounded AX acquisition.json or capture metadata; no FD5/status
or canonical channel extension. The normal AX path still creates no artifacts. Core control
layout/NativeHelperBinding signatures/worker canonical parsing need no change.
The later source owner verifies final encoded config fits4032 and returns a real
Core dependency only if that concrete check fails. Requests can tighten overlapping
limits; small output caps do not silently reduce fields or node coverage.

## Registration and focused proof before live qualification

Root selects this proposal under ROADMAP D05; then register a proposed
`D05-NATIVE-ACQUISITION@1` leaf (copied-value/shape/pixel metrics, ceilings, admission,
opaque-cost and outcome clauses), link it from D05@4 and the specification registry
before source implementation. Existing D02@2/D04@1/MEMORY@2/WORK@1/core0.1/D06@1
meanings and numbers remain unchanged; this is the missing Native acquisition
engineering profile, not a new permission or reduced acceptance criterion.

Offline source packet: equality/cap±1/overflow for every counter, Unicode UTF-8
expansion and invalid conversion, wrong CF types/huge counts with zero forbidden
copy, batch/action refusal without truncation, unknown secure classification with
zero AXValue dispatch, duplicate/cyclic child budget and no prefix window binding,
bounded stream failure with no emitted prefix, nonfinite/huge dimensions before
capture callback, oversized returned image before encoder, PNG callback failure/
short write/finalize refusal/no partial publication, retained staging/final images,
non-image own-file-only cleanup and descriptor release.
Use existing recorded canonical sample to check zero new unknown/lost properties,
no changes to values/edges/coverage and the same Rust validator. Boundary inputs
are synthetic, not SDK acquisition or new worst-case runtime measurements.

Later separately authorized runtime: unchanged own F02 scope/fields/stimulus and
full-size pixels through actual H01 helper; observed copied/SDK/encoding costs and
counts; AX ACK survives capture refusal, another Target AX progresses, timeout/
cancel/detach/late callback retire exact helpers and retain capture lease to reap.
Prove no remaining handles/callback-owned images or helper work after cleanup.
Keep matched D06 stage+outer latency gates, including PNG cost and current data;
do not mix old pixels, reduced workloads or synthetic limits with live acceptance.
B−3801 remains stopped until a separate authorized runtime condition permits its
positive gate; this proposal authorizes no permission/settings/backend retry.
