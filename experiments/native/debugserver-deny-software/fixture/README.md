# Owned source-local geometry fixture

On arm64 macOS with Xcode installed, run `python3 -B build_fixture.py`. This
builds a fresh Debug executable on the mounted development volume, signs just
that binary with the debugger entitlement, and runs its ordinary completion
path in an isolated directory. It prints the actual binary path, SHA-256 and
Mach-O UUID. The source uses only synthetic values and never opens a window.

For hardware-backend qualification create a new empty working directory for
each run. The diagnostic must select only the child it spawned and retain its
original debugger terminal on an uncertain stop or a failed read. Use the
separately qualified deny-write backend. The build smoke test alone does not
verify the debugger or hardware breakpoint.

| Case | Fixed site in main.swift | Expected observation |
| --- | --- | --- |
| Hit | Return in `check`, line 8 | One hardware stop. Static `card` is `sample`, `target` is (11,22,33,44), `rect` is (55,66,77,88), and `start` is (99,111). |
| No hit | Final `exit(5)`, line 17 | Interrupt before normal expiry, verify zero hits, remove the one owned breakpoint, and detach. |
| Missing local | Return in `check`, line 8 | Request the deliberately absent `intentionally_absent_fixture_local`; observe the refusal and the same stopped child. The terminal owner explicitly performs recovery. |

For each case, after confirmed detach, create a file named `finish` only in
that child's working directory. Require both the line `recovered` and exit
status zero. The unmodified fixture otherwise expires with status five.
A stale PID, a debugger exit or the mere absence of the child is insufficient.

To validate a retained qualification, run
`python3 -B check_cases.py MODE LLDB_TRANSCRIPT CHILD_STDOUT`, with MODE
`hit`, `nohit` or `missing`. Supply the transcript and stdout of the same
child. Separately check the child's actual exit status in the launching process.
The validator reads local files only.
