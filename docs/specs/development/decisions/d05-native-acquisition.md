# D05 Native acquisition profile
- Node type: leaf; domain: `uib.development.d05-native-acquisition`; contract: `UIB.D05-NATIVE-ACQUISITION@3`; supersedes @2 baseline reference only; @2 image ownership preserved.
- Clauses: `.METRICS`, `.CEILINGS`, `.ADMISSION`, `.OWNERSHIP`, `.OUTCOMES`, `.PROOF` (prefix `UIB.D05-NATIVE-ACQUISITION`).
- Authority: Active / Evolving; accepted/released implementation: none.
- Authority source: ROADMAP D05/PLAN.UIB@1, root selection of45c2667649c347202dfc64b6d5d978c3e47271f0 through [registration packet](../../../plans/ui-blueprint/packets/M01-acquisition-registration.md); @2 reconciles the explicit user all-task-images rule via [M03 packet](../../../plans/ui-blueprint/packets/M03-popup-capture.md), delta D05-NATIVE-IMAGE-002.
- Read when: Native AX/capture acquisition, copied values, response construction or Native helper qualification.
- Do not read when: pure Rust H01/retained/allocator work without a Native acquisition dependency.
- Requires: [D02@2](d02-boundaries.md), [D04@1](d04-identity.md), [NATIVE-PILOTS@1](../../acceptance/native-pilots.md), [NATIVE@1](../../product/native.md), [PRIVACY@1](../../product/privacy.md), [D06@1](d06-performance.md) and their explicit closure.
- Owner: Native; source implementation and SDK/H01 proof follow registration. [Selected mechanism/evidence](../../../development/native-acquisition.md), not an implemented guarantee.
## UIB.D05-NATIVE-ACQUISITION.METRICS

Bounds apply per selected helper/channel; KiB=1024, MiB=1,048,576. Count every owned
text copy, including discarded candidates, not unique strings. Response slots count
one per container/scalar/dictionary-key occurrence; string charge includes repeated
keys/values, constants and borrowed Context inserted into output. These are logical
payload/shape metrics, not Foundation allocation sizes. Distinguish cumulative work,
peak live ownership, destination bytes and unknown SDK costs. Use checked arithmetic.
Caller supplies explicit positive limits≤profile; absent/invalid/overflow refuses
before acquisition. Request scope/fields/node/depth/output/deadline and parent caps
can only tighten; a small output budget cannot silently remove requested fields.

## UIB.D05-NATIVE-ACQUISITION.CEILINGS

| Metric | Initial maximum | Meaning |
| --- | ---: | --- |
| ax_windows |32 | Complete candidate count for exact window binding; no prefix uniqueness |
| array_page |32 | AXWindows/AXChildren entries per ranged read |
| child_entries |1280 | Total returned child entries including duplicates before retention |
| value_utf8_bytes |4096 | Each text/role/subrole/identifier; UTF-16 length checked first |
| action_names / action_name_utf8_bytes |32 /256 | Names per list / bytes per name; derived list maximum8192 bytes |
| batch_values / batch_utf8_bytes |8 /16384 | Fixed attribute batch count / total admitted copied UTF-8 |
| copied_utf8_bytes |262144 | Cumulative acquired text/action/identity copies |
| response_slots / response_string_utf8_bytes |65536 /1048576 | Cumulative output construction / repeated string charge |
| image_width / image_height |4096 /4096 | Pixel dimensions; retain natural output, no downscale |
| image_pixels |8388608 | Checked width×height before screenshot dispatch |
| image_bytes |67108864 | Nominal BGRA8 before dispatch; actual bytesPerRow×height before encoding |
| png_bytes |67108864 | Aggregate encoded bytes accepted before artifact writes |
| sidecar_bytes |16384 | Aggregate permitted metadata output per helper |

## UIB.D05-NATIVE-ACQUISITION.ADMISSION

Count AXWindows/AXChildren before bounded range copy; inspect raw CFArray count/type
before Swift bridging. Check returned count≤asked, deadline and remaining node/entry
allowance; mutation/error cannot prove complete unique binding. Reuse one collector.
Keep values as CFTypeRef until type/length admission. Check UTF-16 length against the
value ceiling before lossless UTF-8 sizing; CFStringGetBytes NULL-buffer sizing ignores
maxBufLen. Reserve per-value/list/batch/aggregate bytes before owned copies; no lossy
conversion, recursive unknown-container copy or post-copy prefix as acquisition bound.
Preflight an entire batch before copying strings; drop raw values promptly. Failure to
establish safe role/subrole/identifier classification must not dispatch AXValue.
Charge response slots/string occurrences before construction, including wrappers.
Use the existing Foundation codec and a bounded sink over reserved output storage,
leaving LF space. Stream failure publishes no partial canonical frame; release the
rejected graph before a complete bounded failure response. No parallel JSON schema.
Check finite positive width/height/scale, scaling/rounding/integer range, area and
nominal BGRA8 footprint before capture. Preserve SDR semantics with public availability
handling; inspect actual image dimensions/format/row-byte product before encoding.
ImageIO data-consumer callback checks overflow/cap/deadline before copying/writing;
refusal latches failure and returns0. Successful Finalize AND an unfailed writer are
required before publishing an owned completed artifact; never advertise a partial file.

## UIB.D05-NATIVE-ACQUISITION.OWNERSHIP

AX action-name/scalar/batch initial returns, SCShareableContent inventory and returned
CGImage allocation remain opaque; post-return inspection is not pre-acquisition proof.
Avoid duplicate Swift inventory arrays; retain only the required matched objects.
Foundation/ImageIO scratch is opaque even with stream sinks. No global Swift/SDK/RSS
cap follows; Native limits neither enlarge nor substitute for common Rust quotas.
Preserve parent capture lease until confirmed helper reap, independent AX/capture,
local duration/watchdog and authoritative parent deadline/cleanup. One channel/helper.
Artifacts require trusted caller-owned destination, existing synthetic pixel policy, new0700 directory and exclusive/no-follow creation.
All task image staging/partial/final paths and containing directories remain in system temp, never agent-deleted, including failure/stale/helper death.
Close descriptors/reap helpers; exclusively link only complete output without overwrite while retaining its original image path. Never advertise incomplete or stale image payload_ref.
Non-image own partial cleanup remains; no unmasked real-user export.
Private acquisition_limits config remains within the existing4032-byte Core envelope;
no Control/schema/API change. Optional private acquisition_evidence permits bounded
counter/code sidecars only when explicitly requested with trusted artifact_directory
and cleanup owner. Ordinary output gains no telemetry; no UI values/raw errors.

## UIB.D05-NATIVE-ACQUISITION.OUTCOMES

Budget-limited attributes/actions are unknown with bounded reason and partial coverage,
never unsupported/redacted/empty/prefix success. Known protected values remain unacquired
and redacted. Honest omitted/unknown counts persist. Unresolved identity returns
target_unresolved without other-window content; whole-channel resource refusal uses
existing failed incomplete_scope. Timeout/permission keep their existing meanings;
prior ACKed AX survives capture failure. No new public errors, automatic retries or
permission expansion; recorded B−3801 remains stopped.

## UIB.D05-NATIVE-ACQUISITION.PROOF
Offline boundary/overflow/Unicode/type/array/batch/stream/pixel/PNG/cleanup checks must
precede live qualification; source and synthetic checks do not prove SDK lifetime.
Preserve actual F02 known fields/values/coverage,160/depth9 and1100×1050px;
[D06@2 request-only@2](d06-native-request.md) preserves the76-node reference input,
all available facts/extras and same gates; proved variation follows its same-call criterion. Original75 records and failed @1 results remain historical.
Never lower quality/fabricate nodes; all positive pilots stay protected.
Later authorized H01 proof covers actual acquisition/copy/encoding costs, callbacks/
handles/images/helpers after cancel/detach/timeout, AX ACK survival and another Target's
AX progress; parent capture lease ends only on reap. SDK/codec unknowns remain explicit.
No release, live acceptance or source implementation is established by this registration.
