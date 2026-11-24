#!/usr/bin/env python3
"""Source-limit diagnosis on the existing owned SubtreeProbe fixture only."""
import argparse
import json
from pathlib import Path
import subprocess
import sys
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('caller', 'legacy-caller', 'identity', 'control', 'evidence-root'):
        parser.add_argument('--' + name, required=True)
    args = parser.parse_args()
    identity = json.loads(Path(args.identity).read_bytes())
    base = [sys.executable, '-B', args.caller]
    windows = json.loads(subprocess.check_output(base + ['windows', '--session', identity['session']], timeout=10))
    window, = windows['windows']
    binding = ['--session', identity['session'], '--window', str(window['window']),
               '--generation', window['generation'], '--timeout', '10']
    root = Path(args.evidence_root)
    assert root.is_dir()
    (Path(args.control) / 'phase').write_text('5')
    time.sleep(.5)  # bounded owned-fixture state transition, no consumer app action
    results = {}
    for name, caller, operation in [('capture', args.caller, 'capture'),
                                     ('discover', args.caller, 'discover'),
                                     ('legacy', args.legacy_caller, 'capture')]:
        output = root / (name + '.json')
        command = [sys.executable, '-B', caller, operation, *binding, '--out', str(output)]
        if operation == 'capture':
            command += ['--subtree-type', 'SwiftUI.SpacingLayout']
        result = subprocess.run(command, capture_output=True, timeout=15)
        assert result.returncode == 4 and not result.stdout and not output.exists(), name
        failure = json.loads(result.stderr)
        assert failure['status'] == 'failed' and failure['reason'] == 'sourceLimit'
        assert b'UIB_RUNTIME_MODEL_CANARY_317cce' not in result.stderr
        if name == 'legacy':
            assert failure == dict(status='failed', reason='sourceLimit'), failure
        else:
            detail = failure['source_limit']
            assert detail == dict(kind='source_nodes', limit=16384, observed=16385), detail
            assert set(failure) == {'status', 'reason', 'source_limit'}
        results[name] = failure
    endpoint = Path(identity['session']).parent
    assert not list(endpoint.glob('*.snapshot.json'))
    assert not list(endpoint.glob('*.discovery.json'))
    assert not list(endpoint.glob('*.request.json'))
    assert not list(endpoint.glob('*.response.json'))
    (root / 'summary.json').write_text(json.dumps(results, indent=2))
    print(json.dumps(results))


if __name__ == '__main__':
    main()
