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
import shutil
import subprocess
import tarfile
import tempfile
from types import SimpleNamespace
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
                                    formula_template=(ROOT / 'uiblueprint.rb').read_text(),
                                    skill_files={name: (ROOT / 'skills/ui-blueprint' / name).read_bytes()
                                                 for name in release.SKILL_FILES})


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
    run([*args, '--version', '0.2.0'], env=dict(env, DOWNLOAD_FAILURE='yes'), ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    run([*args, '--version', '0.2.0'], env=env)
    assert os.readlink(current) == 'versions/0.2.0' and first.exists()
    run(args, env=env)
    assert os.readlink(current) == 'versions/0.1.0'
    run([*args, '--version', '../escape'], env=env, ok=False)
    run([*args, '--version', '9.9.9'], env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    wrong = versions / '0.3.0'
    wrong.mkdir()
    (wrong / 'install.sh').write_bytes((versions / '0.2.0/install.sh').read_bytes())
    run([*args, '--version', '0.3.0'], env=env, ok=False)
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
    (first / 'RUN.md').chmod(0o600)
    run([*args, '--uninstall'], env=env, ok=False)
    (first / 'RUN.md').chmod(0o644)
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


def skill_checks(stage, versions):
    env = dict(os.environ, PATH=f'{stage / "shims"}:/usr/bin:/bin:/usr/sbin:/sbin',
               FIXTURE_RELEASES=str(versions), TMPDIR=str(stage))
    prefix = stage / 'skill prefix'
    skill = stage / "host skills with 'quote/ui-blueprint"
    installer = versions / '0.1.0/install.sh'
    base = ['sh', installer, '--prefix', prefix]
    command = [*base, '--skill-dir', skill]
    root = prefix / 'share/uiblueprint'
    current = root / 'current'
    run([*base, '--skill-dir', 'relative'], env=env, ok=False)
    run([*base, '--skill-dir', ''], env=env, ok=False)
    run([*base, '--skill-dir', root / 'nested'], env=env, ok=False)
    run([*base, '--skill-dir', root.resolve() / 'nested'], env=env, ok=False)
    assert not prefix.exists()
    # A foreign directory or symlink is not adopted, even if its text is identical.
    skill.mkdir(parents=True)
    (skill / 'keep').write_text('foreign')
    run(command, env=env, ok=False)
    assert (skill / 'keep').read_text() == 'foreign' and not prefix.exists()
    (skill / 'keep').unlink()
    skill.rmdir()
    skill.symlink_to(stage / 'foreign-target')
    run(command, env=env, ok=False)
    assert skill.is_symlink() and not prefix.exists()
    skill.unlink()
    # CLI-only installation remains available; adding a skill needs only its archive.
    run(base, env=env)
    assert not (current / 'skill').exists() and not skill.exists()
    run(command, env=env)
    assert skill.is_symlink() and os.readlink(skill) == str(root / 'current/skill')
    assert (root / '.skill-link').read_text() == str(skill) + '\n'
    first_skill = root / 'versions/0.1.0/skill'
    original = (first_skill / 'SKILL.md').read_bytes()
    for name in release.SKILL_FILES:
        assert (skill / name).read_bytes() == (ROOT / 'skills/ui-blueprint' / name).read_bytes()
    run(command, env=dict(env, DOWNLOAD_FAILURE='yes'))
    run([*base, '--skill-dir', stage / 'another-skill'], env=env, ok=False)
    assert not (stage / 'another-skill').exists()
    # Local edits block activation and removal, including an update without --skill-dir.
    (first_skill / 'SKILL.md').write_bytes(original + b'\nlocal edit\n')
    run([*base, '--version', '0.2.0'], env=env, ok=False)
    run([*base, '--uninstall'], env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0' and skill.is_symlink()
    (first_skill / 'SKILL.md').write_bytes(original)
    (first_skill / 'extra.txt').write_text('keep')
    run([*base, '--version', '0.2.0'], env=env, ok=False)
    (first_skill / 'extra.txt').unlink()
    (first_skill / 'references/web.md').chmod(0o600)
    run(command, env=env, ok=False)
    (first_skill / 'references/web.md').chmod(0o644)
    web = (first_skill / 'references/web.md').read_bytes()
    (first_skill / 'references/web.md').unlink()
    (first_skill / 'references/web.md').symlink_to(stage / 'outside')
    run([*base, '--uninstall'], env=env, ok=False)
    (first_skill / 'references/web.md').unlink()
    (first_skill / 'references/web.md').write_bytes(web)
    # Missing/misbound registration cannot be silently repaired over another owner.
    skill.unlink()
    run(base, env=env, ok=False)
    skill.symlink_to(stage / 'foreign-target')
    run(base, env=env, ok=False)
    skill.unlink()
    skill.symlink_to(root / 'current/skill')
    # Corrupt only the next skill archive: old CLI and skill stay selected.
    archive = versions / '0.2.0' / release.skill_archive_name('0.2.0')
    archive_bytes = archive.read_bytes()
    archive.write_bytes(archive_bytes + b'corrupt')
    run([*base, '--version', '0.2.0'], env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    archive.write_bytes(archive_bytes)
    run([*base, '--version', '0.2.0'], env=env)
    assert os.readlink(current) == 'versions/0.2.0'
    assert skill.resolve() == (root / 'versions/0.2.0/skill').resolve()
    run(base, env=env)  # registered skill survives rollback without repeating its path
    assert skill.resolve() == first_skill.resolve()
    legacy = versions / '0.4.0'
    legacy.mkdir()
    (legacy / 'install.sh').write_text("VERSION='0.4.0'\nexit 0\n")
    run([*base, '--version', '0.4.0'], env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    foreign = skill.parent / 'other-skill'
    foreign.write_text('keep')
    run([*base, '--uninstall'], env=env)
    assert not skill.is_symlink() and not root.exists() and foreign.read_text() == 'keep'
    # First install with a failed skill download never activates a partial set.
    archive = versions / '0.1.0' / release.skill_archive_name('0.1.0')
    original_archive = archive.read_bytes()
    archive.unlink()
    run(command, env=env, ok=False)
    assert not current.exists() and not skill.is_symlink()
    archive.write_bytes(original_archive)
    run(command, env=env)
    run([*base, '--uninstall'], env=env)
    print('PASS skill: opt-in, matching bytes, repeat, registered update/rollback, modified/foreign/symlink/missing paths, partial-download refusal, legacy refusal, owned removal')


def hostile_skill_checks(stage, versions):
    folder = versions / '0.1.0'
    archive = folder / release.skill_archive_name('0.1.0')
    original = archive.read_bytes()
    original_script = (folder / 'install.sh').read_text()
    for kind in ('traversal', 'symlink', 'duplicate'):
        data = io.BytesIO()
        with tarfile.open(fileobj=io.BytesIO(original), mode='r:gz') as src, tarfile.open(fileobj=data, mode='w:gz') as dst:
            for member in src.getmembers():
                if kind == 'symlink' and member.name == 'SKILL.md':
                    member.type, member.linkname, member.size = tarfile.SYMTYPE, '../../escape', 0
                    dst.addfile(member)
                else:
                    dst.addfile(member, src.extractfile(member))
            if kind != 'symlink':
                extra = tarfile.TarInfo('../escape' if kind == 'traversal' else 'SKILL.md')
                extra.size = 1
                dst.addfile(extra, io.BytesIO(b'x'))
        archive.write_bytes(data.getvalue())
        script = stage / f'bad-skill-{kind}.sh'
        script.write_text(original_script.replace(dist.digest(original), dist.digest(data.getvalue())))
        env = dict(os.environ, PATH=f'{stage / "shims"}:/usr/bin:/bin:/usr/sbin:/sbin',
                   FIXTURE_RELEASES=str(versions), TMPDIR=str(stage))
        prefix, skill = stage / f'skill-bad-{kind}', stage / f'skill-link-{kind}'
        run(['sh', script, '--prefix', prefix, '--skill-dir', skill], env=env, ok=False)
        assert not (prefix / 'share/uiblueprint/current').exists() and not skill.is_symlink()
        assert not (stage / 'escape').exists()
    archive.write_bytes(original)
    print('PASS skill archive: traversal, symlink and duplicate members rejected before activation')


def recipe_checks(stage, bundle, versions):
    record = release.read_candidate(versions / '0.1.0')
    assert record['publication'] == 'not_published' and record['notarization'] == 'not_submitted'
    unpacked = stage / 'unpacked'
    unpacked.mkdir()
    release.extract_candidate(versions / '0.1.0', unpacked, record)
    for name in dist.files_for('combined') | {dist.MANIFEST}:
        assert (unpacked / name).read_bytes() == (bundle / name).read_bytes()
    for name in release.SKILL_FILES:
        assert (unpacked / 'skill' / name).read_bytes() == (ROOT / 'skills/ui-blueprint' / name).read_bytes()
    with patch.object(release, 'committed', return_value=b'pinned bytes') as source:
        assert set(release.committed_skill('c' * 40).values()) == {b'pinned bytes'}
        assert [call.args for call in source.call_args_list] == [
            ('c' * 40, f'skills/ui-blueprint/{name}') for name in release.SKILL_FILES]
    # Legacy candidate-1 still verifies/extracts without a skill asset or directory.
    legacy = stage / 'legacy-candidate'
    legacy.mkdir()
    old = dict(record, format='ui-blueprint-release-candidate-1', assets=dict(record['assets']))
    del old['assets'][release.skill_archive_name(old['version'])]
    for name in old['assets']:
        (legacy / name).write_bytes((versions / '0.1.0' / name).read_bytes())
    (legacy / 'release.json').write_bytes(dist.encoded(old))
    (legacy / 'SHA256SUMS').write_text(''.join(f'{old["assets"][name]}  {name}\n'
        for name in (release.archive_name(old['version']), 'install.sh', 'uiblueprint.rb')))
    old_bundle = stage / 'legacy-unpacked'
    old_bundle.mkdir()
    release.extract_candidate(legacy, old_bundle, release.read_candidate(legacy))
    assert not (old_bundle / 'skill').exists()
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
    print('PASS recipe: CLI/skill archive round trip, committed skill source, legacy candidate, template syntax, source-pin guard, local publication rollback, post-signing manifest')


def notarization_checks(folder):
    original = (folder / 'release.json').read_bytes()
    record = json.loads(original)
    record['signing'] = 'developer_id'
    args = SimpleNamespace(destination=folder, keychain_profile='test-profile')
    try:
        for timeout in (False, True):
            (folder / 'release.json').write_bytes(dist.encoded(record))
            def command(argv, **_):
                if argv[:3] == ['xcrun', 'notarytool', 'submit']:
                    assert json.loads((folder / 'release.json').read_bytes())['notarization'] == 'submission_outcome_unknown'
                    if timeout:
                        raise subprocess.TimeoutExpired(argv, 300)
                    return b'{"id":"test-submission"}'
                assert argv[0] in ('codesign', 'ditto')
                return b''
            with patch.object(release.dist, 'run', side_effect=command):
                try:
                    release.notarize(args)
                    assert not timeout
                except subprocess.TimeoutExpired:
                    assert timeout
            status = json.loads((folder / 'release.json').read_bytes())['notarization']
            assert status == ('submission_outcome_unknown' if timeout else 'submitted')
            with patch.object(release.dist, 'run') as external:
                try:
                    release.notarize(args)
                    raise AssertionError('submission repeated')
                except ValueError:
                    pass
                external.assert_not_called()
    finally:
        (folder / 'release.json').write_bytes(original)
    print('PASS notarization control: mocked submission ID/timeout retained; no implicit repeat or real upload')


def homebrew_formula(folder, bundle, stage):
    brew = shutil.which('brew')
    if not brew:
        raise AssertionError('Homebrew is required for the candidate formula API check')
    env = dict(os.environ, HOMEBREW_NO_AUTO_UPDATE='1', HOMEBREW_NO_ANALYTICS='1',
               HOMEBREW_NO_INSTALL_FROM_API='1', HOMEBREW_NO_ENV_HINTS='1',
               HOMEBREW_NO_INSTALL_CLEANUP='1', HOMEBREW_CACHE=str(stage / 'cache'),
               HOMEBREW_LOGS=str(stage / 'logs'), HOMEBREW_TEMP=str(stage))
    # Use the actual Homebrew DSL/install/test APIs, redirecting only the formula's
    # prefix. No tap registration, global linking or change to the operator's Cellar.
    code = '''require "formula"
require "formula_assertions"
load ARGV[0]
formula = Uiblueprint.new("uiblueprint", Pathname(ARGV[0]), :stable)
installed = Pathname(ARGV[1])
formula.define_singleton_method(:prefix) { installed }
formula.define_singleton_method(:opt_prefix) { installed }
formula.extend(Homebrew::Assertions)
Dir.chdir(ARGV[2]) { formula.install }
formula.test
raise "missing public link" unless (installed/"bin/uiblueprint").symlink?
raise "missing worker" unless (installed/"libexec/session-worker").executable?
puts "PASS Homebrew formula: real install/test APIs with temporary prefix"
'''
    print(dist.run([brew, 'ruby', '-e', code, '--', str(folder / 'uiblueprint.rb'),
                    str(stage / 'cellar/uiblueprint/0.1.0'), str(bundle)], env=env, timeout=60).decode().strip())


def real_candidate(folder):
    record = release.read_candidate(folder)
    stage = Path(tempfile.mkdtemp(prefix='uib-installed-check-'))
    try:
        bundle = stage / 'bundle'
        bundle.mkdir()
        release.extract_candidate(folder, bundle, record)
        # No checkout WIP can enter shipped support files.
        for name, source in {'distribution.py': 'distribution.py',
                             'RUN.md': 'docs/development/distribution.md',
                             'THIRD_PARTY_NOTICES.md': 'THIRD_PARTY_NOTICES.md',
                             'web-geometry.cjs': 'tests/bridges/web/geometry.cjs',
                             'WEB-FIRST-USE.md': 'docs/development/web-first-use.md'}.items():
            assert (bundle / name).read_bytes() == release.committed(record['source_revision'], source)
        with_skill = record['format'] == 'ui-blueprint-release-candidate-2'
        if with_skill:
            for name, data in release.committed_skill(record['source_revision']).items():
                assert (bundle / 'skill' / name).read_bytes() == data
        spec = importlib.util.spec_from_file_location('distribution_check', ROOT / 'tests/bridges/distribution_check.py')
        checks = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checks)
        checks.smoke(bundle)
        if record['signing'] == 'developer_id':
            for name in release.BINARIES:
                run(['codesign', '--verify', '--strict', bundle / name])
        # Exercise the actual installer with actual signed payload; substitute only
        # HTTPS transport because the user has explicitly deferred publication.
        shims = stage / 'shims'
        shims.mkdir()
        (shims / 'curl').write_text('''#!/bin/sh
set -eu
out=''
for arg do
  if [ "$out" = next ]; then out=$arg; fi
  if [ "$arg" = --output ]; then out=next; fi
  url=$arg
done
case "$url" in
  "$EXPECTED_URL") cp "$LOCAL_ARCHIVE" "$out" ;;
  "$EXPECTED_SKILL_URL") cp "$LOCAL_SKILL_ARCHIVE" "$out" ;;
  *) exit 2 ;;
esac
''')
        (shims / 'curl').chmod(0o755)
        archive = release.archive_name(record['version'])
        env = dict(os.environ, PATH=f'{shims}:/usr/bin:/bin:/usr/sbin:/sbin', TMPDIR=str(stage),
                   EXPECTED_URL=f'{release.REPOSITORY}/releases/download/v{record["version"]}/{archive}',
                   LOCAL_ARCHIVE=str(folder / archive),
                   EXPECTED_SKILL_URL=f'{release.REPOSITORY}/releases/download/v{record["version"]}/{release.skill_archive_name(record["version"])}',
                   LOCAL_SKILL_ARCHIVE=str(folder / release.skill_archive_name(record['version'])))
        prefix = stage / 'real installed prefix'
        command = ['sh', folder / 'install.sh', '--prefix', prefix]
        skill = stage / 'real host skills/ui-blueprint'
        if with_skill:
            command += ['--skill-dir', skill]
        run(command, env=env)
        run([prefix / 'bin/uiblueprint', '--help'], env=env)
        installed = prefix / 'share/uiblueprint/versions' / record['version']
        if with_skill:
            assert skill.resolve().parent == installed.resolve()
            for name in release.SKILL_FILES:
                assert (skill / name).read_bytes() == (bundle / 'skill' / name).read_bytes()
            # All Markdown references stay inside the installed skill, outside checkout.
            import re
            for path in skill.resolve().rglob('*.md'):
                for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
                    resolved = (path.parent / link).resolve()
                    assert resolved.is_relative_to(skill.resolve()) and resolved.is_file(), link
        checks.smoke(installed)
        run([*command, '--uninstall'], env=env)
        assert not (prefix / 'share/uiblueprint').exists()
        assert not skill.is_symlink()
        if with_skill:
            # Homebrew receives the unchanged CLI archive, not the separate skill asset.
            for name in release.SKILL_FILES:
                (bundle / 'skill' / name).unlink()
            for name in ('agents', 'references', ''):
                (bundle / 'skill' / name).rmdir()
        homebrew_formula(folder, bundle, stage)
        print('PASS actual candidate: provenance, signed bundle smoke, real install/run/remove; transport is local')
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
        skill_checks(stage, versions)
        hostile_skill_checks(stage, versions)
        hostile_archive_checks(stage, bundle, versions)
        recipe_checks(stage, bundle, versions)
        notarization_checks(versions / '0.1.0')
    finally:
        dist.clean_stage(stage)
        assert not stage.exists(), stage
    if args.candidate:
        real_candidate(args.candidate)


if __name__ == '__main__':
    main()
