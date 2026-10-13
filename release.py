#!/usr/bin/env python3
"""Prepare a committed Mac release locally. Never tags, pushes or publishes."""
import sys
sys.dont_write_bytecode = True

import argparse
import gzip
import io
import json
import os
from pathlib import Path
import re
import stat
import tarfile
import tempfile
from types import SimpleNamespace

import distribution as dist

ROOT = Path(__file__).resolve().parent
REPOSITORY = 'https://github.com/potapenko/ui-blueprint'
BINARIES = ('uiblueprint', 'uiblueprint-validate', 'session-worker', 'native-host-helper')
VERSION = re.compile(r'[0-9]+\.[0-9]+\.[0-9]+(?:-[A-Za-z0-9]+(?:[.-][A-Za-z0-9]+)*)?\Z')


def committed(revision, path):
    return dist.run(['git', 'show', f'{revision}:{path}'], cwd=ROOT)


def resolve_revision(revision):
    resolved = dist.run(['git', 'rev-parse', '--verify', revision + '^{commit}'], cwd=ROOT).decode().strip()
    # Prevent a release recipe different from the recipe recorded in its source pin.
    for name in ('release.py', 'distribution.py'):
        if committed(resolved, name) != (ROOT / name).read_bytes():
            raise ValueError(f'{name} differs from selected commit; checkpoint it before preparation')
    return resolved


def archive_name(version):
    if not VERSION.fullmatch(version):
        raise ValueError('expected a SemVer release version without a v prefix')
    return f'uiblueprint-v{version}-{dist.TARGET}.tar.gz'


def make_archive(bundle, output):
    with dist.destination(bundle) as fd:
        manifest, _ = dist.verify(fd)
    names = sorted(set(manifest['files']) | {dist.MANIFEST})
    with output.open('xb') as stream, gzip.GzipFile(fileobj=stream, mode='wb', mtime=0) as compressed:
        with tarfile.open(fileobj=compressed, mode='w', format=tarfile.USTAR_FORMAT) as archive:
            for name in names:
                path = bundle / name
                data = path.read_bytes()
                entry = tarfile.TarInfo(name)
                entry.size = len(data)
                entry.mode = stat.S_IMODE(path.stat().st_mode)
                archive.addfile(entry, io.BytesIO(data))
    return {name: dist.digest((bundle / name).read_bytes()) for name in names}


def render(template, replacements):
    for key, value in replacements.items():
        template = template.replace(f'@{key}@', value)
    if re.search(r'@[A-Z_]+@', template):
        raise ValueError('unresolved release template field')
    return template.encode()


def write_candidates(bundle, output, *, version, revision, minimum, signing,
                     installer_template, formula_template):
    archive = output / archive_name(version)
    hashes = make_archive(bundle, archive)
    archive_hash = dist.digest(archive.read_bytes())
    replacements = {'VERSION': version, 'ARCHIVE_SHA256': archive_hash, 'MIN_MACOS': minimum,
                    'FILES': '\n'.join(sorted(hashes)),
                    'FILE_HASHES': '\n'.join(f'{hashes[name]}  {name}' for name in sorted(hashes))}
    for name, template in [('install.sh', installer_template), ('uiblueprint.rb', formula_template)]:
        with (output / name).open('xb') as stream:
            stream.write(render(template, replacements))
    names = [archive.name, 'install.sh', 'uiblueprint.rb']
    assets = {name: dist.digest((output / name).read_bytes()) for name in names}
    with (output / 'SHA256SUMS').open('x') as stream:
        stream.write(''.join(f'{assets[name]}  {name}\n' for name in names))
    record = {'format': 'ui-blueprint-release-candidate-1', 'version': version,
              'source_revision': revision, 'target': dist.TARGET, 'minimum_macos': minimum,
              'modules': 'combined', 'signing': signing, 'notarization': 'not_submitted',
              'publication': 'not_published', 'project_license': 'pending_owner_decision',
              'assets': assets}
    with (output / 'release.json').open('xb') as stream:
        stream.write(dist.encoded(record))
    return record


def sign_bundle(bundle, identity):
    if not re.fullmatch(r'[A-Fa-f0-9]{40}', identity):
        raise ValueError('use an existing Developer ID Application certificate SHA-1 identity')
    identities = dist.run(['security', 'find-identity', '-v', '-p', 'codesigning']).decode()
    if not any(identity.upper() in line.upper() and 'Developer ID Application:' in line
               for line in identities.splitlines()):
        raise ValueError('selected Developer ID Application identity is not available')
    for name in BINARIES:
        dist.run(['codesign', '--force', '--sign', identity, '--options', 'runtime',
                  '--timestamp', str(bundle / name)], timeout=120)
        dist.run(['codesign', '--verify', '--strict', str(bundle / name)], timeout=30)
    # Signing changes bytes. Never leave the pre-signing manifest in the archive.
    manifest = json.loads((bundle / dist.MANIFEST).read_bytes())
    for name in BINARIES:
        manifest['files'][name]['sha256'] = dist.digest((bundle / name).read_bytes())
    (bundle / dist.MANIFEST).write_bytes(dist.encoded(manifest))
    with dist.destination(bundle) as fd:
        dist.verify(fd)


def prepare(args):
    name = archive_name(args.version)
    revision = resolve_revision(args.revision)
    minimum = dist.run(['sw_vers', '-productVersion']).decode().strip()
    if not re.fullmatch(r'[0-9]+\.[0-9]+(?:\.[0-9]+)?', minimum):
        raise ValueError('unrecognized macOS version')
    with dist.destination(args.destination) as destination:
        dist.absent(destination, {name, 'install.sh', 'uiblueprint.rb', 'SHA256SUMS', 'release.json'})
        stage = Path(tempfile.mkdtemp(prefix='uib-release-'))
        try:
            bundle, output = stage / 'bundle', stage / 'output'
            bundle.mkdir()
            output.mkdir()
            with dist.destination(bundle) as fd:
                dist.build(SimpleNamespace(modules='combined', revision=revision), fd)
            signing = 'unsigned'
            if args.sign_identity:
                sign_bundle(bundle, args.sign_identity)
                signing = 'developer_id'
            record = write_candidates(bundle, output, version=args.version, revision=revision,
                                      minimum=minimum, signing=signing,
                                      installer_template=committed(revision, 'install.sh').decode(),
                                      formula_template=committed(revision, 'uiblueprint.rb').decode())
            # Reuse exclusive publication/rollback, with release.json as completion marker.
            products = {p.name: (p.read_bytes(), 0o644) for p in output.iterdir() if p.name != 'release.json'}
            publish_candidates(destination, products, record)
            return record
        finally:
            dist.clean_stage(stage)


def publish_candidates(fd, products, record):
    """Publish locally, exclusively. Never touch a foreign inode on rollback."""
    written = []
    try:
        for name, (data, mode) in [*sorted(products.items()), ('release.json', (dist.encoded(record), 0o644))]:
            out = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, mode, dir_fd=fd)
            identity = os.fstat(out)
            written.append((name, identity.st_dev, identity.st_ino))
            with os.fdopen(out, 'wb') as stream:
                stream.write(data)
                stream.flush()
                os.fsync(stream.fileno())
    except BaseException:
        for name, device, inode in reversed(written):
            info = os.stat(name, dir_fd=fd, follow_symlinks=False)
            if (info.st_dev, info.st_ino) == (device, inode):
                os.unlink(name, dir_fd=fd)
        raise


def read_candidate(folder):
    with dist.destination(folder) as fd:
        raw, _ = dist.read_regular(fd, 'release.json')
        record = json.loads(raw)
        name = archive_name(record['version'])
        if record.get('format') != 'ui-blueprint-release-candidate-1' or set(record['assets']) != {name, 'install.sh', 'uiblueprint.rb'}:
            raise ValueError('invalid release inventory')
        for asset, expected in record['assets'].items():
            data, _ = dist.read_regular(fd, asset)
            if dist.digest(data) != expected:
                raise ValueError(f'changed candidate: {asset}')
    return record


def extract_candidate(folder, bundle, record):
    expected = dist.files_for('combined') | {dist.MANIFEST}
    with tarfile.open(folder / archive_name(record['version']), 'r:gz') as archive:
        members = archive.getmembers()
        if len(members) != len(expected) or {m.name for m in members} != expected or any(not m.isfile() for m in members):
            raise ValueError('invalid release archive inventory')
        for member in members:
            with (bundle / member.name).open('xb') as stream:
                stream.write(archive.extractfile(member).read())
            (bundle / member.name).chmod(member.mode & 0o777)
    with dist.destination(bundle) as fd:
        manifest, _ = dist.verify(fd)
    if manifest['source_revision'] != record['source_revision'] or manifest['modules'] != 'combined':
        raise ValueError('release source/module mismatch')


def notarize(args):
    """Explicit future upload step. Not invoked by prepare or tests."""
    record = read_candidate(args.destination)
    if record['signing'] != 'developer_id':
        raise ValueError('notarization requires a Developer ID signed candidate')
    if record['notarization'] != 'not_submitted':
        raise ValueError('submission already recorded; inspect its status instead of resubmitting')
    stage = Path(tempfile.mkdtemp(prefix='uib-notarize-'))
    try:
        bundle = stage / 'bundle'
        bundle.mkdir()
        extract_candidate(args.destination, bundle, record)
        for name in BINARIES:
            dist.run(['codesign', '--verify', '--strict', str(bundle / name)])
        # Apple accepts zip, not tar.gz. These are the exact signed archive contents.
        archive = stage / 'notarization.zip'
        dist.run(['ditto', '-c', '-k', '--keepParent', str(bundle), str(archive)])
        result = json.loads(dist.run(['xcrun', 'notarytool', 'submit', str(archive),
                                    '--keychain-profile', args.keychain_profile,
                                    '--output-format', 'json'], timeout=300))
        record['notarization'] = 'submitted'
        record['notarization_id'] = result['id']
        # Persist the submission ID before a later bounded wait; do not auto-resubmit.
        with dist.destination(args.destination) as fd:
            temporary = '.release-notarization.json'
            out = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o644, dir_fd=fd)
            with os.fdopen(out, 'wb') as stream:
                stream.write(dist.encoded(record))
                stream.flush()
                os.fsync(stream.fileno())
            os.rename(temporary, 'release.json', src_dir_fd=fd, dst_dir_fd=fd)
        return record
    finally:
        dist.clean_stage(stage)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    build = commands.add_parser('prepare', help='build locally; no upload/publication')
    build.add_argument('--version', required=True)
    build.add_argument('--revision', default='HEAD')
    build.add_argument('--destination', type=Path, required=True, help='existing absolute output directory')
    build.add_argument('--sign-identity', help='existing Developer ID Application certificate SHA-1')
    verify = commands.add_parser('verify', help='verify prepared archive and metadata offline')
    verify.add_argument('--destination', type=Path, required=True)
    submit = commands.add_parser('notarize', help='EXPLICIT upload to Apple; requires separate operator authorization')
    submit.add_argument('--destination', type=Path, required=True)
    submit.add_argument('--keychain-profile', required=True)
    args = parser.parse_args()
    try:
        if args.command == 'prepare':
            result = prepare(args)
        elif args.command == 'notarize':
            result = notarize(args)
        else:
            result = read_candidate(args.destination)
            stage = Path(tempfile.mkdtemp(prefix='uib-verify-release-'))
            try:
                extract_candidate(args.destination, stage, result)
                if result['signing'] == 'developer_id':
                    for name in BINARIES:
                        dist.run(['codesign', '--verify', '--strict', str(stage / name)])
            finally:
                dist.clean_stage(stage)
        print(json.dumps(result))
        return 0
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f'release: {error}', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
