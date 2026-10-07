# Native host-helper source review

Packet: [M01-host-helper-review](../packets/M01-host-helper-review.md).
Fresh reviewer `/root/m01_host_helper_review`, no forked conversation or nested
agents. Exact candidate7c6e0780ac085c2a024281c2f8b0196ce536bfa0 against parent
74ea8e1ad9ea95eae6d0e491cae674f472095717. Root accepts the bounded source
connection with the mandatory runtime gaps below; no actionable introduced defects.

Source observations preceded author receipts/docs. Reviewer inspected all six
changed source/test files, unchanged WindowAX and concrete c0abcff Core protocol,
binding/broker/helpers/receive/cleanup dependencies. It confirmed one shared
collector/encoder, real Ticket correlation, admitted input lengths, partial IO,
LF-inclusive cap, independent channels, fixture-specific process/Surface binding,
sanitized failures and parent-owned capture cleanup. Legacy entrypoints retain
the same shared implementation. Source does not establish general application
identity or a universal SDK/memory/pixel privacy guarantee.

After initial observations, reviewer reconciled the saved author receipt and
consumer docs. Independently checked all seven source-manifest entries against
7c6e078, all19 shared entries againstc0abcff, before/after shared manifest byte
equality and the three reported binary hashes. No mismatch. Manifest hashes:

- own7: f359a0c37c0cbbd6bf1167c5bb6958efe0a4585c8f6fbb19d10b9be865b60f3d;
- shared19: 54dab07d5e95718c3c3729ca7647a1acf3046c96445c914f84d68121eab84a03.

Four compiled targets and31 offline cases remain author execution evidence;
reviewer ran none. Initial reviewer count32 was corrected after source accounting:
31 is accurate, including exactly three actual-helper refusals before SDK access.
Positive replies are synthetic ProtocolPeer permission failures using the shared
canonical encoder, not actual acquisition. The receipt's checkpoint_ready wording
is historical; saved/pushed identity is established separately by7c6e078.

Actual Swift-through-H01 acquisition/receive/ACK, cancellation/parent-loss/EOF/reap,
positive Native pilots, copied-string/pixel/helper acquisition bounds, production
pixel privacy, broader identity and D06 remain open. Build/offline proof cannot
close them. Reuse the same reviewer for an affected repair; do not repeat unchanged
source review merely to obtain a broader verdict.

Read-only throughout: no edits, builds/tests/helpers/SDK/UI/external messages.
Unrelated Core/Integration working changes excluded; final observed HEADda6330e
and no staged files. Named Native task-temp remains retained for immediate consumers.
