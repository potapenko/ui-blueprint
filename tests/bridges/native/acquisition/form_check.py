#!/usr/bin/env python3
"""Only focused requested form properties; no live app or value collection."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

parser = argparse.ArgumentParser()
for name in ('checks', 'validator', 'output'):
    parser.add_argument('--' + name, type=Path, required=True)
args = parser.parse_args()
root = Path(__file__).resolve().parents[4]
run = Path(tempfile.mkdtemp(prefix='form-', dir=args.output))
result = subprocess.run([str(args.checks), str(root / 'tests/bridges/native/acquisition/profile.json'), str(run)],
                        capture_output=True, timeout=15)
assert result.returncode == 0 and not result.stderr, (result.returncode, result.stderr[-1000:])
report = json.loads(result.stdout)
for p in sorted(run.glob('*.json')):
    valid = subprocess.run([str(args.validator), '--max-bytes', '524288', str(p)], capture_output=True, timeout=3)
    assert valid.returncode == 0, (p.name, valid.stdout)
report.update(canonical_validated=5, owned_run=str(run))
print(json.dumps(report, indent=2))
