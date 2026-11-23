"""Launch one self-owned R11 fixture for one capture, then request normal exit.

One process generation per case avoids reusing an uncertain debugger state. A
held capture does not return here: recovery remains with its bound owner command.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time

from capture_fixture import capture


def run(task_path, case, phase, no_oracle=False):
    task=json.loads(Path(task_path).read_text());runtime=Path(task["internal"])
    control=runtime/("control-off" if no_oracle else "control")
    app=runtime/("R11PreviewNoOracle.app" if no_oracle else "R11PreviewFixture.app")
    oracle=control/"oracle.json"
    if oracle.exists():
        previous=json.loads(oracle.read_text())["pid"]
        if subprocess.run(["kill","-0",str(previous)],capture_output=True).returncode==0:
            raise RuntimeError("previous fixture generation still exists")
        oracle.unlink()
    if (control/"finish").exists(): (control/"finish").unlink()
    (control/"phase").write_text(str(phase))
    subprocess.run(["open","-n","-g","--env","SWIFTUI_VIEW_DEBUG=29","--env","UIB_R11_CONTROL="+str(control),
        "--env","TMPDIR="+str(runtime/"tmp"),"--env","TMP="+str(runtime/"tmp"),
        "--env","TEMP="+str(runtime/"tmp"),str(app)],check=True)
    end=time.monotonic()+5
    while not oracle.exists() and time.monotonic()<end:time.sleep(.05)
    if not oracle.exists():raise RuntimeError("fixture launch has no oracle readiness")
    code=capture(task_path,case,phase,control.name,boundaries=True)
    evidence=Path(task["external"])/"evidence"/case
    request=json.loads((evidence/"request.json").read_text())
    manifest=json.loads((evidence/("manifest-"+request["request_id"]+".json")).read_text())
    if manifest["cleanup"]!="detached_restored":raise RuntimeError("capture ownership unresolved")
    if code == 0 and no_oracle:
        bundle=Path(task["bundle"])
        env=dict(os.environ,TMPDIR=task["external"],TMP=task["external"],TEMP=task["external"])
        image=subprocess.run([str(bundle/"uiblueprint"),"capture-window","--pid",str(request["pid"]),
            "--window-id",str(request["window_id"]),"--expected-bundle",request["expected_bundle"],
            "--expected-launch-time",str(request["expected_launch_time"]),"--out",str(evidence/"native"),
            "--request-id","r11-no-oracle"],env=env,capture_output=True,text=True)
        if image.returncode != 0:raise RuntimeError("fixture image failed: "+image.stderr)
    pid=json.loads(oracle.read_text())["pid"]
    (control/"finish").touch()
    end=time.monotonic()+5
    while subprocess.run(["kill","-0",str(pid)],capture_output=True).returncode==0 and time.monotonic()<end:time.sleep(.1)
    if subprocess.run(["kill","-0",str(pid)],capture_output=True).returncode==0:
        raise RuntimeError("normal fixture termination not confirmed")
    (evidence/"fixture-exit.json").write_text(json.dumps(dict(pid=pid,normal_exit=True)))
    return code


if __name__=="__main__":
    p=argparse.ArgumentParser(description=__doc__);p.add_argument("task");p.add_argument("case");p.add_argument("phase",type=int)
    p.add_argument("--no-oracle",action="store_true");a=p.parse_args()
    raise SystemExit(run(a.task,a.case,a.phase,a.no_oracle))
