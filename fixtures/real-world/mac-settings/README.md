# Real PlayPhrase.me Settings example

F03a / RC01; captured 2026-10-06 from an existing canonical Release.
This is offline reference data for an agent/engine/export evaluation, not a
shipping UI Blueprint collector result or a new acceptance run of PlayPhrase.me.

- Inputs: prompt.md, observations.json, ax-excerpt.txt and the PNG paths in JSON.
- Reviewer only: expected-answer.md. Do not include the answer key in agent input.
- AX excerpt preserves selected CUA-reported lines; surrounding unrelated content
  was omitted. Numeric CUA indices are historical observation IDs, not live refs.
- JSON is a selected fixture record, not a competing production wire schema.
- All images remain under the root-owned application-state directory named in
  observations.json, retained for C01/Q03/P7 until acceptance or explicit discard.

## Reproduce after an explicit live packet

Use the current source AGENTS/spec/runtime closure. Acquire macos-product then
desktop; do not displace a live owner. Resolve the canonical Release path and
record PID/start time, binary hash, build/source attribution and actual OS.
In paused learner content, press Command-comma, observe Settings, independently
capture the exact native window to PNG, then close with Escape without changing
any value. Preserve the prior scenario and release leases. No monitoring loop.

The retained asset used /usr/sbin/screencapture -x -o -l1277 -t png and a crop-only
sips command. Window ID 1277 is historical and MUST be rediscovered for a new run.
Image pixel dimensions and native CG bounds were checked; no rescaling was used.
Native PNG and final crop were inspected at original detail and logical 2x size.

## Limits

The pre-existing binary's exact source commit is unknown; metadata records its
canonical path and SHA-256, and separately the source checkout HEAD/dirty path.
No build occurred in F03a. AX/frame calls were separate, not atomic; exact AX call
timestamps were not separately retained. PNG capture file times and the enclosing
run interval are recorded. This data is not a latency benchmark or pixel golden.
