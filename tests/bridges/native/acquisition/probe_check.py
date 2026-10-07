#!/usr/bin/env python3
"""Bounded explicit-probe encoding only; expected oracle stays in the test."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

p = argparse.ArgumentParser()
for name in ('checks', 'validator', 'output'):
    p.add_argument('--' + name, type=Path, required=True)
a = p.parse_args()
root = Path(__file__).resolve().parents[4]
run = Path(tempfile.mkdtemp(prefix='probe-', dir=a.output))
r = subprocess.run([str(a.checks), str(root / 'tests/bridges/native/acquisition/profile.json'),
                    str(run), str(root / 'fixtures/native/expectations.json')], capture_output=True, timeout=15)
assert r.returncode == 0 and not r.stderr, (r.returncode, r.stderr[-1500:])
report = json.loads(r.stdout)
for item in run.glob('*.json'):
    check = subprocess.run([str(a.validator), '--max-bytes', '524288', str(item)], capture_output=True, timeout=3)
    assert check.returncode == 0, (item.name, check.stdout)
report.update(canonical_documents=8)
print(json.dumps(report))
