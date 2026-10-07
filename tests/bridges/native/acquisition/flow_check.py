#!/usr/bin/env python3
"""The three review gaps only; no numeric-suite replay or live AX/SCK/UI."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser()
    for name in ('checks', 'validator', 'sample', 'output'):
        parser.add_argument('--' + name, type=Path, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[4]
    run = Path(tempfile.mkdtemp(prefix='flow-', dir=args.output))
    result = subprocess.run([str(args.checks), str(root / 'tests/bridges/native/acquisition/profile.json'),
                             str(root / 'fixtures/golden/ENV-REQUEST-VALID.json'), str(args.sample), str(run)],
                            capture_output=True, timeout=30)
    assert result.returncode == 0, (result.returncode, result.stderr.decode(errors='replace')[-2000:])
    assert not result.stderr
    report = json.loads(result.stdout)
    for name in ('construction.ndjson', 'codec.ndjson'):
        checked = subprocess.run([str(args.validator), '--max-bytes', '4096', str(run / name)],
                                 capture_output=True, timeout=3)
        assert checked.returncode == 0, (name, checked.stdout)
    report.update(whole_failures_canonical_validated=2, owned_run=str(run),
                  scope='actual traversal/schedule/Collector terminal path; bounded synthetic CF data and owned socketpairs')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
