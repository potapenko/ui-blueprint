#!/usr/bin/env python3
"""Run only owned synthetic acquisition checks and saved-record preservation."""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--checks', type=Path, required=True)
    parser.add_argument('--validator', type=Path, required=True)
    parser.add_argument('--sample', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[4]
    run = Path(tempfile.mkdtemp(prefix='synthetic-', dir=args.output))
    artifacts = run / 'artifacts'
    artifacts.mkdir(mode=0o700)
    output = run / 'roundtrip.ndjson'
    result = subprocess.run([str(args.checks), str(root / 'tests/bridges/native/acquisition/profile.json'),
                             str(artifacts), str(args.sample), str(output)], capture_output=True, timeout=40)
    assert result.returncode == 0, (result.returncode, result.stderr.decode(errors='replace')[-2000:])
    report = json.loads(result.stdout)
    assert not result.stderr, 'unexpected diagnostics'
    assert json.loads(output.read_bytes()) == json.loads(args.sample.read_bytes()), 'recorded canonical data changed'
    validate = subprocess.run([str(args.validator), '--max-bytes', '524288', str(output)],
                              capture_output=True, timeout=3)
    assert validate.returncode == 0, validate.stdout
    report.update(recorded_canonical_equal=True, canonical_validator=True, owned_run=str(run),
                  evidence='synthetic CF/Foundation/ImageIO and existing recorded data; no live AX/SCK/UI')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
