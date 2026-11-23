"""Validate output from the three synthetic Control cases; never run a debugger."""
import argparse
import json
from pathlib import Path
import re


def check_case(mode, transcript, child_stdout):
    lines = child_stdout.splitlines()
    ready = re.fullmatch(r"ready (\d+) (\d+) (\d+)", lines[0]) if lines else None
    if not ready or ready[2] != ready[3] or lines[1:] != ["recovered"]:
        raise ValueError("fixture did not report a normal recovered exit path")
    pid = int(ready[1])
    detach = f"Process {pid} detached"
    if (transcript.count(detach) != 1 or transcript.count("DETACHED_AND_OWNER_ALIVE") != 1
            or transcript.index(detach) > transcript.index("DETACHED_AND_OWNER_ALIVE")):
        raise ValueError("no unique independently corroborated detach")
    hits = transcript.count("stop reason = one-shot breakpoint")
    if mode == "hit":
        records = [json.loads(line.removeprefix("PUBLIC_CARD_GEOMETRY "))
                   for line in transcript.splitlines() if line.startswith("PUBLIC_CARD_GEOMETRY ")]
        expected = {"card": "sample", "space": "fixture-local", "target_xywh": [11, 22, 33, 44],
                    "rect_xywh": [55, 66, 77, 88], "start_xy": [99, 111], "detached_verified": True}
        if (hits != 1 or len(records) != 1 or records[0]["identity_before_after"]["pid"] != pid
                or any(records[0].get(k) != v for k, v in expected.items())):
            raise ValueError("unexpected synthetic hit or geometry")
    elif mode == "nohit":
        if hits or "DEADLINE_NO_HIT_CONFIRMED" not in transcript or "PUBLIC_CARD_GEOMETRY " in transcript:
            raise ValueError("zero-hit deadline did not resolve cleanly")
    else:
        events = ["local_type_or_availability_refused", "OPERATOR_CONTROL_RETAINED",
                  "(lldb) script owner_recover_stopped()", detach]
        offsets = [transcript.index(event) for event in events]
        if hits != 1 or offsets != sorted(offsets) or "PUBLIC_CARD_GEOMETRY " in transcript:
            raise ValueError("missing-local refusal did not retain ownership until explicit recovery")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=["hit", "nohit", "missing"])
    parser.add_argument("transcript", type=Path)
    parser.add_argument("child_stdout", type=Path)
    args = parser.parse_args()
    check_case(args.mode, args.transcript.read_text(), args.child_stdout.read_text())
    print(f"Verified {args.mode} fixture transcript and child output.")
