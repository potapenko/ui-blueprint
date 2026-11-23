"""One bounded capture of our R11 fixture, never a product process.

The caller supplies the task.json produced during task setup. Collector/backend
are frozen inside that task's internal runtime directory. No automatic retry.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time


def capture(task_path, case, phase, control_name="control", boundaries=False):
    task = json.loads(Path(task_path).read_text())
    runtime, evidence = Path(task["internal"]), Path(task["external"])/"evidence"
    control, bundle = runtime/control_name, Path(task["bundle"])
    if not case.replace("-", "").isalnum() or not 0 <= phase <= 4:
        raise ValueError("invalid fixture case")
    temporary = control/"phase.new"
    temporary.write_text(str(phase)); temporary.replace(control/"phase")
    end, previous, stable_since = time.monotonic()+5, None, 0
    while time.monotonic() < end:
        raw = (control/"oracle.json").read_bytes()
        oracle = json.loads(raw)
        if raw != previous:
            previous, stable_since = raw, time.monotonic()
        if oracle["phase"] == phase and time.monotonic()-stable_since >= .3:
            break
        time.sleep(.05)
    else:
        raise RuntimeError("fixture oracle not settled")
    metadata = json.loads(subprocess.check_output(
        [str(bundle/"native-host-helper"), "describe-process", str(oracle["pid"])], timeout=5))["process"]
    if metadata["bundle_id"] not in ("me.uiblueprint.external-layout.canary.r11.preview",
                                     "me.uiblueprint.external-layout.canary.r11.preview.no-oracle"):
        raise RuntimeError("R11 fixture identity required")
    out = evidence/case
    out.mkdir(mode=0o700)
    request = dict(request_version="1.0.0", request_id="r11-"+case, pid=metadata["pid"],
        expected_bundle=metadata["bundle_id"], expected_launch_time=metadata["launch_time"],
        window_id=oracle["window_id"],
        scope=dict(type="R11PreviewFixture.FeatureGateSubscribePreview", containing_type="SwiftUI.LazyVGrid"),
        limits=dict(max_nodes=256, max_depth=256, max_output_bytes=1048576, deadline_ms=10000))
    (out/"request.json").write_text(json.dumps(request))
    (out/"oracle-before.json").write_bytes(raw)
    env = dict(os.environ, TMPDIR=str(runtime/"tmp"), TMP=str(runtime/"tmp"), TEMP=str(runtime/"tmp"))
    # The collector retains an uncertain debugger; do not kill it with an outer timeout.
    script = Path(__file__).with_name("capture_boundaries.py") if boundaries else runtime/"collector/swiftui_debug_capture.py"
    command = ["python3", "-B", str(script),
        "--request", str(out/"request.json"), "--bundle", str(bundle), "--out", str(out),
        "--backend-profile", str(runtime/"backend.json")]
    if boundaries: command += ["--collector", str(runtime/"collector")]
    result = subprocess.run(command, env=env)
    after = (control/"oracle.json").read_bytes()
    (out/"oracle-after.json").write_bytes(after)
    if after != raw:
        raise RuntimeError("fixture changed during capture")
    return result.returncode


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("task"); parser.add_argument("case"); parser.add_argument("phase", type=int)
    parser.add_argument("--control", default="control", choices=("control", "control-off"))
    parser.add_argument("--boundaries", action="store_true")
    args = parser.parse_args()
    raise SystemExit(capture(args.task, args.case, args.phase, args.control, args.boundaries))
