#!/usr/bin/env python3
import argparse,json,subprocess,tempfile
from pathlib import Path
p=argparse.ArgumentParser()
for name in ('checks','validator','output'):p.add_argument('--'+name,type=Path,required=True)
a=p.parse_args();r=Path(__file__).resolve().parents[4];d=Path(tempfile.mkdtemp(prefix='popup-',dir=a.output));out=d/'responses';out.mkdir()
s=subprocess.run([str(a.checks),str(d),str(r/'tests/bridges/native/acquisition/profile.json'),str(out)],capture_output=True,timeout=20)
assert s.returncode==0 and not s.stderr,(s.returncode,s.stderr[-2000:]);report=json.loads(s.stdout)
for f in out.glob('*.json'):
 v=subprocess.run([str(a.validator),'--max-bytes','524288',str(f)],capture_output=True,timeout=3);assert v.returncode==0,(f.name,v.stdout)
report['canonical_documents']=len(list(out.glob('*.json')));print(json.dumps(report))
