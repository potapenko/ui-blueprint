#!/usr/bin/env python3
"""Finite offline command/binding/limits checks; no screen capture or app changes."""
import argparse
from pathlib import Path
import subprocess
import tempfile
import sys


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--helper', type=Path, required=True)
    parser.add_argument('--checks', type=Path, required=True)
    args = parser.parse_args()
    with tempfile.TemporaryDirectory(prefix='uib-native-command-') as name:
        root = Path(name)
        result = subprocess.run([str(args.checks), str(root)], capture_output=True, timeout=10)
        assert result.returncode == 0, result.stderr
        print(result.stdout.decode().strip())
        help_result = subprocess.run([str(args.helper), 'capture-window', '--help'], capture_output=True, timeout=3)
        assert help_result.returncode == 0 and b'CG_WINDOW_ID' in help_result.stdout
        for flags in ([], ['--pid', '0'], ['--unknown', 'PRIVATE_CANARY'],
                      ['--help', '--pid', '1']):
            result = subprocess.run([str(args.helper), 'capture-window', *flags], capture_output=True, timeout=3)
            assert result.returncode == 2 and not result.stdout
            assert b'PRIVATE_CANARY' not in result.stderr
        wrong = ['--pid','2147483647','--window-id','1','--expected-bundle','public.app',
                 '--out',str(root/'not-created'),'--request-id','wrong']
        result = subprocess.run([str(args.helper),'capture-window',*wrong],capture_output=True,timeout=3)
        assert result.returncode == 4 and not result.stdout and not (root/'not-created').exists()
        print('Helper help/invalid-input/privacy/wrong-process: 6 passed')
    assert not root.exists()


if __name__ == '__main__':
    sys.exit(main())
