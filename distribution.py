#!/usr/bin/env python3
"""Build a selected, committed UI Blueprint bundle without global installation."""
import argparse
import contextlib
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import re
import shutil
import signal
import stat
import subprocess
import sys
import tarfile
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parent
TARGET = 'aarch64-apple-darwin'
TOOLCHAIN = '1.96.0'
MANIFEST = 'distribution-manifest.json'
COMMON = {'uiblueprint', 'uiblueprint-validate', 'distribution.py', 'RUN.md',
          'THIRD_PARTY_NOTICES.md', 'DEPENDENCY_LICENSES.txt', 'RUST_LIBRARY_NOTICES.html',
          'uiblueprint-0.1.0.schema.json', 'uiblueprint-analysis-0.2.0.schema.json',
          'example-snapshot.json', 'example-query.json', 'example-evaluation.json',
          'example-expectation.json', 'example-observed-brief.json', 'example-proposed-brief.json'}
IMAGE_SUFFIXES = {'.png', '.jpg', '.jpeg', '.gif', '.webp', '.svg', '.ico', '.icns',
                  '.bmp', '.tif', '.tiff', '.heic', '.heif', '.avif', '.pdf', '.partial'}
SWIFT = ['plugins/macos/NativeAcquisition.swift', 'plugins/macos/NativeJSON.swift',
         'plugins/macos/NativeArtifacts.swift', 'fixtures/native/Observe.swift',
         'tests/bridges/native/Collector.swift', 'tests/bridges/native/WindowAX.swift',
         'plugins/macos/HostProtocol.swift', 'plugins/macos/NativeFocusedAX.swift',
         'plugins/macos/NativeFormSession.swift', 'plugins/macos/HostHelper.swift']


def files_for(modules):
    names = COMMON.copy()
    if modules != 'core':
        names.add('session-worker')
    if modules in ('native', 'combined'):
        names.add('native-host-helper')
    return names


def encoded(value):
    return (json.dumps(value, indent=2, sort_keys=True) + '\n').encode()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def run(command, *, cwd=None, env=None, timeout=300):
    """A fresh process group owns compilers too; output stays in memory, never logs."""
    child = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.DEVNULL,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                             start_new_session=True)
    try:
        stdout, stderr = child.communicate(timeout=timeout)
        if child.returncode:
            raise ValueError(f'{command[0]} failed ({child.returncode}): '
                             + stderr.decode(errors='replace')[-3000:])
        return stdout
    finally:
        # Only the process group created above, never a target app/browser.
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        child.wait()


def clean_stage(folder):
    """Remove own non-image products only; never recursively delete images/dirs."""
    for parent, dirs, files in os.walk(folder, topdown=False, followlinks=False):
        for name in files:
            path = Path(parent) / name
            if path.suffix.lower() not in IMAGE_SUFFIXES:
                path.unlink()
        for name in dirs:
            path = Path(parent) / name
            if path.is_symlink():
                path.unlink()
            else:
                with contextlib.suppress(OSError):
                    path.rmdir()
    with contextlib.suppress(OSError):
        folder.rmdir()
    if folder.exists():
        print(f'Retained image-containing temporary directory: {folder}', file=sys.stderr)


@contextlib.contextmanager
def destination(path):
    if not path.is_absolute():
        raise ValueError('destination must be an explicit existing absolute directory')
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        yield fd
    finally:
        os.close(fd)


def absent(fd, names):
    for name in names:
        try:
            os.stat(name, dir_fd=fd, follow_symlinks=False)
        except FileNotFoundError:
            continue
        raise ValueError(f'destination conflict: {name}; nothing overwritten')


def publish(fd, products, manifest):
    """Exclusive files; manifest is the final completion marker. Roll back own inodes."""
    written = []
    try:
        for name, (data, mode) in [*sorted(products.items()), (MANIFEST, (encoded(manifest), 0o644))]:
            out = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW,
                          0o600, dir_fd=fd)
            identity = os.fstat(out)
            written.append((name, identity.st_dev, identity.st_ino))
            with os.fdopen(out, 'wb') as stream:
                stream.write(data)
                os.fchmod(stream.fileno(), mode)
                stream.flush()
                os.fsync(stream.fileno())
    except BaseException:
        for name, device, inode in reversed(written):
            info = os.stat(name, dir_fd=fd, follow_symlinks=False)
            if (info.st_dev, info.st_ino) == (device, inode):
                os.unlink(name, dir_fd=fd)
        raise


def read_regular(fd, name):
    with os.fdopen(os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
                           dir_fd=fd), 'rb') as stream:
        info = os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode):
            raise ValueError(f'not a regular bundle file: {name}')
        return stream.read(), info


def verify(fd):
    raw, _ = read_regular(fd, MANIFEST)
    manifest = json.loads(raw)
    modules = manifest.get('modules')
    if (manifest.get('format') != 'ui-blueprint-local-1' or
            modules not in ('core', 'web', 'native', 'combined') or
            set(manifest.get('files', {})) != files_for(modules)):
        raise ValueError('invalid distribution manifest; no files removed')
    identities = {}
    for name, expected in manifest['files'].items():
        data, info = read_regular(fd, name)
        if digest(data) != expected['sha256'] or stat.S_IMODE(info.st_mode) != expected['mode']:
            raise ValueError(f'changed bundle file: {name}; no files removed')
        identities[name] = (info.st_dev, info.st_ino)
    return manifest, identities


def remove(fd):
    manifest, identities = verify(fd)  # complete preflight before the first removal
    for name, identity in identities.items():
        info = os.stat(name, dir_fd=fd, follow_symlinks=False)
        if (info.st_dev, info.st_ino) != identity:
            raise ValueError('bundle changed during removal; stopped')
        os.unlink(name, dir_fd=fd)
    os.unlink(MANIFEST, dir_fd=fd)
    return manifest


def dependency_notices(source, env, cargo, selection):
    tree = run([*cargo, 'tree', '--locked', '--offline', '--target', TARGET,
                '-e', 'normal,build', '--prefix', 'none', '--format', '{p}',
                *selection], cwd=source, env=env).decode()
    selected = {tuple(line.split()[:2]) for line in tree.splitlines() if line}
    metadata = json.loads(run([*cargo, 'metadata', '--locked', '--offline',
                               '--format-version', '1', '--filter-platform', TARGET],
                              cwd=source, env=env))
    lock = tomllib.loads((source / 'Cargo.lock').read_text())
    checksums = {(p['name'], p['version']): p.get('checksum') for p in lock['package']}
    records, texts = [], []
    for package in sorted(metadata['packages'], key=lambda p: (p['name'], p['version'])):
        name, version = package['name'], package['version']
        if (name, 'v' + version) not in selected or package['source'] is None:
            continue
        root = Path(package['manifest_path']).parent
        licenses = sorted(p for p in root.rglob('*') if p.is_file()
                          and p.name.upper().startswith(('LICENSE', 'NOTICE', 'COPYING')))
        if not licenses or not checksums[(name, version)]:
            raise ValueError(f'missing locked license material: {name}')
        if package['license'] not in ('MIT', 'MIT OR Apache-2.0', 'Apache-2.0 OR MIT',
                                       'Unlicense OR MIT', '(MIT OR Apache-2.0) AND Unicode-3.0'):
            raise ValueError(f'license requires review: {name}')
        material = {str(p.relative_to(root)): p.read_bytes() for p in licenses}
        if name == 'serde_json':
            header = (root / 'src/lexical/mod.rs').read_text().split('//!')[0]
            if 'copyright Alexander Huszagh' not in header:
                raise ValueError('serde_json lexical attribution needs review')
            material['src/lexical/mod.rs attribution'] = header.encode()
        records.append({'name': name, 'version': version, 'license': package['license'],
                        'source': package['source'], 'checksum': checksums[(name, version)],
                        'notices': {key: digest(value) for key, value in material.items()}})
        for filename, data in material.items():
            texts.append(f'\n=== {name} {version} / {filename} ===\n'.encode() + data + b'\n')
    if len(records) != len([p for p in selected if not p[0].startswith('uiblueprint-')]):
        raise ValueError('dependency inventory incomplete')
    return records, b''.join(texts)


def build(args, fd):
    if platform.system() != 'Darwin' or platform.machine() != 'arm64':
        raise ValueError('this recipe is qualified only for arm64 macOS; no cross-build claim')
    absent(fd, files_for(args.modules) | {MANIFEST})
    for tool in ('git', 'rustup') + (('xcrun',) if args.modules in ('native', 'combined') else ()):
        if shutil.which(tool) is None:
            raise ValueError(f'missing required tool: {tool}')
    revision = run(['git', 'rev-parse', '--verify', args.revision + '^{commit}'], cwd=ROOT).decode().strip()
    stage = Path(tempfile.mkdtemp(prefix='uib-distribution-'))
    try:
        source = stage / 'source'
        source.mkdir()
        paths = ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', 'crates', 'plugins/web',
                 'schemas', 'fixtures/analysis', 'fixtures/export']
        if args.modules in ('native', 'combined'):
            paths += SWIFT
        archive = run(['git', 'archive', revision, *paths], cwd=ROOT)
        with tarfile.open(fileobj=io.BytesIO(archive)) as saved:
            for member in saved:
                path = Path(member.name)
                if path.is_absolute() or '..' in path.parts or not (member.isdir() or member.isfile()):
                    raise ValueError('unsupported source archive entry')
                if path.suffix.lower() in IMAGE_SUFFIXES:
                    continue  # no image assets are needed by this build
                if member.isdir():
                    (source / path).mkdir(parents=True, exist_ok=True)
                else:
                    (source / path).parent.mkdir(parents=True, exist_ok=True)
                    (source / path).write_bytes(saved.extractfile(member).read())
        env = dict(os.environ, CARGO_TARGET_DIR=str(stage / 'target'), CARGO_INCREMENTAL='0')
        cargo = ['rustup', 'run', TOOLCHAIN, 'cargo']
        cli_features = {'core': [], 'web': ['web'], 'native': ['macos'], 'combined': ['web', 'macos']}[args.modules]
        selection = ['-p', 'uiblueprint-cli', '-p', 'uiblueprint-schema']
        features = ['uiblueprint-cli/' + f for f in cli_features]
        if args.modules != 'core':
            selection += ['-p', 'uiblueprint-host']
            if 'web' in cli_features:
                features += ['uiblueprint-host/web']
        if features:
            selection += ['--features', ','.join(features)]
        selection += ['--no-default-features']
        tool_version = run(['rustup', 'run', TOOLCHAIN, 'rustc', '-vV'], cwd=source).decode().strip()
        print(f'Building {args.modules} from {revision}', file=sys.stderr, flush=True)
        run([*cargo, 'build', '--release', '--locked', '--offline', '--target', TARGET,
             '--bins', *selection], cwd=source, env=env)
        records, licenses = dependency_notices(source, env, cargo, selection)
        binaries = ['uiblueprint', 'uiblueprint-validate']
        if args.modules != 'core':
            binaries += ['session-worker']
        products = {name: ((stage / 'target' / TARGET / 'release' / name).read_bytes(), 0o755)
                    for name in binaries}
        swift_version = None
        if args.modules in ('native', 'combined'):
            swift_version = run(['xcrun', 'swiftc', '--version']).decode().strip()
            run(['xcrun', 'swiftc', '-O', '-parse-as-library', '-swift-version', '6',
                 '-D', 'HOST_HELPER', '-D', 'CAPTURE_LIBRARY', '-target', 'arm64-apple-macos14.0',
                 '-module-cache-path', str(stage / 'module-cache'),
                 *[str(source / p) for p in SWIFT], '-o', str(stage / 'native-host-helper')])
            products['native-host-helper'] = ((stage / 'native-host-helper').read_bytes(), 0o755)
        for path in (source / 'schemas').glob('*.json'):
            products[path.name] = (path.read_bytes(), 0o644)
        example = json.loads((source / 'fixtures/analysis/measurement-gap.json').read_bytes())['artifact']['data']
        products['example-snapshot.json'] = (encoded({'schema_version': '0.1.0',
            'artifact': {'kind': 'snapshot', 'data': example['snapshot']}}), 0o644)
        check = json.loads((source / 'fixtures/analysis/check-pass.json').read_bytes())['artifact']['data']
        products['example-expectation.json'] = (encoded({'schema_version': '0.1.0',
            'artifact': {'kind': 'expectation', 'data': check['expectation']}}), 0o644)
        for output, input_path in {'example-query.json': 'fixtures/analysis/query-gap.json',
                'example-evaluation.json': 'fixtures/analysis/evaluation-local.json',
                'example-observed-brief.json': 'fixtures/export/observed-brief.json',
                'example-proposed-brief.json': 'fixtures/export/proposed-brief.json'}.items():
            products[output] = ((source / input_path).read_bytes(), 0o644)
        # Recipe/docs provenance is explicit, independent of the chosen product revision.
        for output, path in {'distribution.py': Path(__file__), 'RUN.md': ROOT / 'docs/development/distribution.md',
                             'THIRD_PARTY_NOTICES.md': ROOT / 'THIRD_PARTY_NOTICES.md'}.items():
            products[output] = (path.read_bytes(), 0o644)
        products['DEPENDENCY_LICENSES.txt'] = (licenses, 0o644)
        sysroot = Path(run(['rustup', 'run', TOOLCHAIN, 'rustc', '--print', 'sysroot'],
                           cwd=source).decode().strip())
        # Rust std is linked into binaries but absent from the workspace Cargo graph.
        products['RUST_LIBRARY_NOTICES.html'] = (
            (sysroot / 'share/doc/rust/COPYRIGHT-library.html').read_bytes(), 0o644)
        if set(products) != files_for(args.modules):
            raise ValueError('unexpected bundle inventory')
        manifest = {'format': 'ui-blueprint-local-1', 'source_revision': revision,
            'modules': args.modules, 'target': TARGET, 'profile': 'release',
            'cli_features': cli_features, 'worker_features': ['web'] if 'web' in cli_features else [],
            'rustc': tool_version, 'swift': swift_version, 'host_os': platform.mac_ver()[0],
            'cargo_lock_sha256': digest((source / 'Cargo.lock').read_bytes()),
            'dependencies': records, 'files': {name: {'sha256': digest(data), 'mode': mode}
                                              for name, (data, mode) in products.items()}}
        publish(fd, products, manifest)
        return manifest
    finally:
        clean_stage(stage)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    for name in ('build', 'verify', 'remove'):
        sub = commands.add_parser(name)
        sub.add_argument('--destination', type=Path, required=True)
        if name == 'build':
            sub.add_argument('--modules', choices=['core', 'web', 'native', 'combined'], required=True)
            sub.add_argument('--revision', default='HEAD', help='committed product source (default HEAD)')
    args = parser.parse_args()
    try:
        with destination(args.destination) as fd:
            if args.command == 'build':
                manifest = build(args, fd)
            elif args.command == 'verify':
                manifest, _ = verify(fd)
            else:
                manifest = remove(fd)
        print(json.dumps({'status': {'build': 'installed', 'verify': 'verified', 'remove': 'removed'}[args.command],
                          'source_revision': manifest['source_revision'], 'modules': manifest['modules'],
                          'destination': str(args.destination)}))
        return 0
    except (OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired) as error:
        print(f'distribution: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
