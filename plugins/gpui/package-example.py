#!/usr/bin/env python3
"""Prepare the already-built diagnostic example in a new system-temp app bundle.

No build, launch, installation, input, network or cleanup is performed.
"""
import argparse
import hashlib
import json
from pathlib import Path
import plistlib
import shlex
import shutil
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--destination', type=Path, required=True)
    parser.add_argument('--off', action='store_true')
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    parent = args.destination.parent.resolve(strict=True)
    system_temp = Path(tempfile.gettempdir()).resolve()
    if not binary.is_file() or not parent.is_relative_to(system_temp):
        parser.error('binary must exist and destination must be inside system temp')
    destination = parent / args.destination.name
    destination.mkdir(exist_ok=False)
    app = destination / 'UIBlueprint GPUI.app'
    macos = app / 'Contents' / 'MacOS'
    macos.mkdir(parents=True)
    output = destination / 'captures'
    output.mkdir()
    shutil.copy2(binary, macos / 'native')
    digest = hashlib.sha256((macos / 'native').read_bytes()).hexdigest()
    info = {
        'CFBundleName': 'UI Blueprint GPUI',
        'CFBundleDisplayName': 'UI Blueprint GPUI',
        'CFBundleIdentifier': 'org.uiblueprint.gpui-example',
        'CFBundleExecutable': 'launch',
        'CFBundlePackageType': 'APPL',
        'CFBundleVersion': '1',
        'NSHighResolutionCapable': True,
    }
    (app / 'Contents' / 'Info.plist').write_bytes(plistlib.dumps(info))
    command = [str(macos / 'native'), str(output)]
    if args.off:
        command.append('off')
    launch = macos / 'launch'
    launch.write_text('#!/bin/sh\nexport UIB_EXAMPLE_BUILD=' + shlex.quote(digest)
                      + '\nexec ' + shlex.join(command) + ' >'
                      + shlex.quote(str(destination / 'native.log')) + ' 2>&1\n')
    launch.chmod(0o755)
    print(json.dumps({'app': str(app), 'captures': str(output), 'binary_sha256': digest,
                      'mode': 'off' if args.off else 'on'}))


if __name__ == '__main__':
    main()
