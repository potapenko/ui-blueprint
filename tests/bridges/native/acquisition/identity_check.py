#!/usr/bin/env python3
"""Real identity owner/reader synthetic lifecycle; no UI/window launch."""
import argparse,json,subprocess,tempfile
from pathlib import Path
p=argparse.ArgumentParser()
for name in ('checks','output'):p.add_argument('--'+name,type=Path,required=True)
a=p.parse_args();root=Path(__file__).resolve().parents[4];run=Path(tempfile.mkdtemp(prefix='identity-',dir=a.output))
r=subprocess.run([str(a.checks),str(run),str(root/'tests/bridges/native/acquisition/profile.json')],capture_output=True,timeout=10)
assert r.returncode==0 and not r.stderr,(r.returncode,r.stderr[-1500:]);print(r.stdout.decode().strip())
