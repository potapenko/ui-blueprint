#!/usr/bin/env python3
"""Isolated release/installer checks. Fake HTTPS transport; never installs globally."""
import sys
sys.dont_write_bytecode = True

import argparse
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
import release
import distribution as dist


def run(command, *, env=None, ok=True):
    result = subprocess.run([str(x) for x in command], capture_output=True, env=env,
                            cwd='/', timeout=90)
    if (result.returncode == 0) != ok:
        raise AssertionError((command, result.returncode, result.stdout.decode(), result.stderr.decode()))
    return result


def bundle_at(folder, revision='a' * 40):
    folder.mkdir()
    products = {name: ((name + '\n').encode(), 0o644) for name in dist.files_for('combined')}
    for name in release.BINARIES:
        products[name] = (b'#!/bin/sh\nprintf "Usage: synthetic candidate\\n"\n', 0o755)
    manifest = {'format': 'ui-blueprint-local-2', 'source_revision': revision,
                'modules': 'combined', 'files': {name: {'sha256': dist.digest(data), 'mode': mode}
                                                for name, (data, mode) in products.items()}}
    with dist.destination(folder) as fd:
        dist.publish(fd, products, manifest)


def candidate_at(folder, bundle, version):
    folder.mkdir()
    return release.write_candidates(bundle, folder, version=version, revision='a' * 40,
                                    minimum='27.0.1', signing='unsigned',
                                    installer_template=(ROOT / 'install.sh').read_text(),
                                    formula_template=(ROOT / 'uiblueprint.rb').read_text())


def install_checks(stage, versions):
    shims = stage / 'shims'
    shims.mkdir()
    # A deterministic downloader receives the production HTTPS URL/flags, but
    # supplies local bytes. No network or undocumented production test flag.
    (shims / 'curl').write_text('''#!/bin/sh
set -eu
out=''
for arg do
  if [ "$out" = next ]; then out=$arg; fi
  if [ "$arg" = --output ]; then out=next; fi
  url=$arg
done
case "$url" in https://github.com/potapenko/ui-blueprint/releases/download/v*/*) ;; *) exit 3;; esac
relative=${url#https://github.com/potapenko/ui-blueprint/releases/download/v}
if [ "${DOWNLOAD_FAILURE:-}" = yes ]; then printf partial > "$out"; exit 22; fi
cp "$FIXTURE_RELEASES/$relative" "$out"
if [ "${CORRUPT_DOWNLOAD:-}" = yes ]; then printf corrupt >> "$out"; fi
''')
    (shims / 'uname').write_text('''#!/bin/sh
case "$1" in -s) printf '%s\\n' "${TEST_OS:-Darwin}";; -m) printf '%s\\n' "${TEST_ARCH:-arm64}";; esac
''')
    (shims / 'sw_vers').write_text('#!/bin/sh\nprintf "%s\\n" "${TEST_MACOS:-27.0.1}"\n')
    for path in shims.iterdir():
        path.chmod(0o755)
    env = dict(os.environ, PATH=f'{shims}:/usr/bin:/bin:/usr/sbin:/sbin',
               FIXTURE_RELEASES=str(versions), TMPDIR=str(stage))
    installer = versions / '0.1.0/install.sh'
    prefix = stage / "prefix with spaces and 'quote"
    args = ['sh', installer, '--prefix', prefix]
    install_root = prefix / 'share/uiblueprint'
    current = install_root / 'current'
    first = install_root / 'versions/0.1.0'
    for failure in [{'TEST_ARCH': 'x86_64'}, {'TEST_OS': 'Linux'}, {'TEST_MACOS': '14.0'}]:
        run(args, env=dict(env, **failure), ok=False)
        assert not prefix.exists()
    run(args, env=dict(env, DOWNLOAD_FAILURE='yes'), ok=False)
    assert not current.exists()
    run(args, env=dict(env, CORRUPT_DOWNLOAD='yes'), ok=False)
    assert not current.exists()
    result = run(args, env=env)
    assert os.readlink(current) == 'versions/0.1.0'
    path_line = next(line.removeprefix('For this shell: ') for line in result.stdout.decode().splitlines()
                     if line.startswith('For this shell: '))
    value = run(['sh', '-c', path_line + '; printf "%s" "$PATH"'], env=env).stdout.decode()
    assert value.startswith(str(prefix / 'bin') + ':')
    run([prefix / 'bin/uiblueprint', '--help'], env=env)
    run(args, env=dict(env, DOWNLOAD_FAILURE='yes'))  # Already installed: no download.
    (install_root / '.lock').mkdir()
    run(args, env=env, ok=False)
    (install_root / '.lock').rmdir()
    # Cross-version dispatch pins the requested remote installer.
    run([*args, '--version', '0.2.0'], env=env)
    assert os.readlink(current) == 'versions/0.2.0' and first.exists()
    run(args, env=env)
    assert os.readlink(current) == 'versions/0.1.0'
    run([*args, '--version', '../escape'], env=env, ok=False)
    run([*args, '--version', '9.9.9'], env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    # Modified/unknown content refuses complete uninstall before unlinking commands.
    (first / 'foreign.txt').write_text('keep')
    run([*args, '--uninstall'], env=env, ok=False)
    assert (prefix / 'bin/uiblueprint').is_symlink()
    (first / 'foreign.txt').unlink()
    original = (first / 'RUN.md').read_bytes()
    (first / 'RUN.md').write_text('changed')
    run([*args, '--uninstall'], env=env, ok=False)
    assert (first / 'uiblueprint').exists()
    (first / 'RUN.md').write_bytes(original)
    (first / 'RUN.md').unlink()
    outside = stage / 'outside'
    outside.write_bytes(original)
    (first / 'RUN.md').symlink_to(outside)
    run([*args, '--uninstall'], env=env, ok=False)
    assert outside.read_bytes() == original
    (first / 'RUN.md').unlink()
    (first / 'RUN.md').write_bytes(original)
    foreign = prefix / 'bin/foreign-command'
    foreign.write_text('keep')
    run([*args, '--uninstall'], env=env)
    assert not install_root.exists() and foreign.read_text() == 'keep'
    # A foreign public command and symlink prefix are never overwritten.
    collision = prefix / 'bin/uiblueprint'
    collision.write_text('foreign')
    run(args, env=env, ok=False)
    assert collision.read_text() == 'foreign'
    collision.unlink()
    linked = stage / 'linked-prefix'
    linked.symlink_to(prefix)
    run(['sh', installer, '--prefix', linked], env=env, ok=False)
    run(args, env=env)
    run([*args, '--uninstall'], env=env)
    assert not list(stage.glob('uiblueprint-install.*'))
    print('PASS installer: platform, download/hash failures, spaces/quotes/PATH, repeat, lock, update, rollback, missing release, modified/foreign/symlink files, removal')


def hostile_archive_checks(stage, bundle, versions):
    original_script = (versions / '0.1.0/install.sh').read_text()
    original_archive = versions / '0.1.0' / release.archive_name('0.1.0')
    original_bytes = original_archive.read_bytes()
    original_hash = dist.digest(original_bytes)
    for index, kind in enumerate(('traversal', 'symlink', 'duplicate')):
        crafted = stage / f'{kind}.tar.gz'
        with tarfile.open(fileobj=io.BytesIO(original_bytes), mode='r:gz') as source, tarfile.open(crafted, 'w:gz') as archive:
            for member in source.getmembers():
                if kind == 'symlink' and member.name == 'RUN.md':
                    member.type = tarfile.SYMTYPE
                    member.linkname = '../../escape'
                    member.size = 0
                    archive.addfile(member)
                else:
                    archive.addfile(member, source.extractfile(member))
            if kind != 'symlink':
                item = tarfile.TarInfo('../escape' if kind == 'traversal' else 'RUN.md')
                item.size = 4
                archive.addfile(item, io.BytesIO(b'evil'))
        data = crafted.read_bytes()
        original_archive.write_bytes(data)
        installer = stage / f'bad-{index}.sh'
        installer.write_text(original_script.replace(original_hash, dist.digest(data)))
        env = dict(os.environ, PATH=f'{stage / "shims"}:/usr/bin:/bin:/usr/sbin:/sbin',
                   FIXTURE_RELEASES=str(versions), TMPDIR=str(stage))
        run(['sh', installer, '--prefix', stage / f'bad-prefix-{index}'], env=env, ok=False)
        assert not (stage / f'bad-prefix-{index}/share/uiblueprint/current').exists()
    original_archive.write_bytes(original_bytes)
    assert not (stage / 'escape').exists()
    print('PASS archive: traversal, link and duplicate entries refused before extraction')


def recipe_checks(stage, bundle, versions):
    record = release.read_candidate(versions / '0.1.0')
    assert record['publication'] == 'not_published' and record['notarization'] == 'not_submitted'
    unpacked = stage / 'unpacked'
    unpacked.mkdir()
    release.extract_candidate(versions / '0.1.0', unpacked, record)
    for name in dist.files_for('combined') | {dist.MANIFEST}:
        assert (unpacked / name).read_bytes() == (bundle / name).read_bytes()
    run(['ruby', '-c', versions / '0.1.0/uiblueprint.rb'])
    with patch.object(release.dist, 'run') as command:
        command.side_effect = [b'b' * 40 + b'\n', b'foreign recipe']
        try:
            release.resolve_revision('HEAD')
            raise AssertionError('uncommitted recipe accepted')
        except ValueError:
            pass
    destination = stage / 'publish'
    destination.mkdir()
    (destination / 'foreign').write_text('keep')
    with dist.destination(destination) as fd:
        with patch.object(release.os, 'fsync', side_effect=OSError('injected failure')):
            try:
                release.publish_candidates(fd, {'asset': (b'hello', 0o644)}, record)
                raise AssertionError('failed publication accepted')
            except OSError:
                pass
    assert {p.name for p in destination.iterdir()} == {'foreign'}
    # Real signing changes bytes; prove refreshed manifest hashes using a signer double.
    identity = 'A' * 40
    def signer(command, **_):
        if command[0] == 'security':
            return f'1) {identity} "Developer ID Application: Test"'.encode()
        if '--sign' in command:
            path = Path(command[-1])
            path.write_bytes(path.read_bytes() + b'# signed\n')
        return b''
    with patch.object(release.dist, 'run', side_effect=signer):
        release.sign_bundle(bundle, identity)
    with dist.destination(bundle) as fd:
        dist.verify(fd)
    print('PASS recipe: archive round trip, template syntax, source-pin guard, local publication rollback, post-signing manifest')


def real_candidate(folder):
    record = release.read_candidate(folder)
    stage = Path(tempfile.mkdtemp(prefix='uib-installed-check-'))
    try:
        bundle = stage / 'bundle'
        bundle.mkdir()
        release.extract_candidate(folder, bundle, record)
        spec = importlib.util.spec_from_file_location('distribution_check', ROOT / 'tests/bridges/distribution_check.py')
        checks = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checks)
        checks.smoke(bundle)
        if record['signing'] == 'developer_id':
            for name in release.BINARIES:
                run(['codesign', '--verify', '--strict', bundle / name])
        print('PASS actual candidate: complete offline bundle smoke and recorded signature checks')
    finally:
        dist.clean_stage(stage)
        assert not stage.exists()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', type=Path)
    args = parser.parse_args()
    stage = Path(tempfile.mkdtemp(prefix='uib-release-check-'))
    try:
        bundle = stage / 'bundle'
        bundle_at(bundle)
        versions = stage / 'releases'
        versions.mkdir()
        for version in ('0.1.0', '0.2.0'):
            candidate_at(versions / version, bundle, version)
        install_checks(stage, versions)
        hostile_archive_checks(stage, bundle, versions)
        recipe_checks(stage, bundle, versions)
    finally:
        dist.clean_stage(stage)
        assert not stage.exists(), stage
    if args.candidate:
        real_candidate(args.candidate)


if __name__ == '__main__':
    main()
