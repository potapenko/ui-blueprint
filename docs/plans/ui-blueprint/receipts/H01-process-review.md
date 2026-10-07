# H01 process review

Verdict: **reject**, one P1 process-ownership defect; no additional scoped findings.
Fresh reviewer `/root/host_process_review`; Nativebb69d4c, connected Core
7b942ab1ddd5af334da18412c6b2d3ac7bd7a21b. Independent source/contract observation
preceded author receipt; same reviewer reconciled author docs/input identities.
All38 saved shared Git blobs match Native's extended input manifest. Native's
phase-specific9 and2 successful test claims remain attributed evidence; reviewer
ran no tests, peers, apps or mutations. Initial disclosed failure log is not hidden
or confused with successful proof; no extra log-retention gate was imposed.

## H01-PROCESS-R1 — P1: automatic reaping invalidates PID reservation

At `crates/host/src/process.rs:209–211`, try_reap returning Running is followed by
kill. With parent SA_NOCLDWAIT, Darwin may automatically reap a naturally exiting
child between those calls and reuse its PID. SIGKILL can then target an unrelated
process without another component ever calling waitpid. The documented exclusive
wait assumption does not cover this kernel behavior. Core confirmed no implemented
SIGCHLD/SA_NOCLDWAIT compatibility or lifetime ownership guarantee.

Basis: review criterion4 and [D02.LIFECYCLE](../../../specs/development/decisions/d02-boundaries.md#uibd02lifecycle--deadline-and-truthful-effects)
(line72: owned-process-only termination); SDK
`/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/share/man/man2/sigaction.2:197–208`
establishes automatic reaping/ECHILD under SA_NOCLDWAIT.

Repair owner: Native rejects incompatible automatic-reaping configuration before
spawn and preserves a stable supported parent signal/reaping lifetime contract;
Core owns its real host integration/guarantee. Recheck in a disposable owned peer,
including SA_NOCLDWAIT, repeated reap and lost ownership. Never mutate operator
signal policy or demonstrate the race by risking an unrelated process.
Same reviewer retained for focused recheck. Full watchdog/allocator/supervisor/live
acceptance remains separate. Source is not accepted while this defect remains.
