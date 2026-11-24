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
    manifest = {'format': 'ui-blueprint-local-8', 'source_revision': revision,
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
    # Failure at the final activation removes only the newly registered public links.
    mv = stage / 'shims/mv'
    mv.write_text('''#!/bin/sh
for destination do :; done
case "$destination" in
  */current) [ "${FAIL_ACTIVATION:-}" != yes ] || exit 1 ;;
esac
exec /bin/mv "$@"
''')
    mv.chmod(0o755)
    run(command, env=dict(env, FAIL_ACTIVATION='yes'), ok=False)
    assert not current.exists() and not skill.is_symlink()
    assert not (root / '.skill-link').exists()
    assert not (prefix / 'bin/uiblueprint').is_symlink()
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


def local_install_checks(stage, versions):
    prefix, skill = stage / 'local candidate prefix', stage / 'local skills/ui-blueprint'
    first, second = versions / '0.1.0', versions / '0.2.0'
    env = dict(os.environ, PATH=f'{stage / "shims"}:/usr/bin:/bin:/usr/sbin:/sbin',
               TMPDIR=str(stage), DOWNLOAD_FAILURE='yes')
    command = ['sh', first / 'install.sh', '--prefix', prefix, '--skill-dir', skill]
    run([*command, '--local-release', 'relative'], env=env, ok=False)
    run([*command, '--local-release', first, '--version', '0.2.0'], env=env, ok=False)
    assert not prefix.exists()
    run([*command, '--local-release', first], env=env)
    current = prefix / 'share/uiblueprint/current'
    assert os.readlink(current) == 'versions/0.1.0' and (skill / 'SKILL.md').is_file()
    run([*command, '--local-release', first], env=env)
    archive = second / release.skill_archive_name('0.2.0')
    original = archive.read_bytes()
    update = ['sh', second / 'install.sh', '--prefix', prefix, '--local-release', second]
    archive.write_bytes(original + b'corrupt')
    run(update, env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    archive.unlink()
    outside = stage / 'skill-archive-outside'
    outside.write_bytes(original)
    archive.symlink_to(outside)
    run(update, env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    archive.unlink()
    archive.write_bytes(original)
    run(update, env=env)
    assert os.readlink(current) == 'versions/0.2.0' and '0.2.0' in str(skill.resolve())
    run([*command, '--local-release', first], env=env)
    run([*command, '--uninstall'], env=env)
    assert not current.exists() and not skill.is_symlink()
    print('PASS local candidate: no network, repeat/update/rollback, invalid path/version, corrupt/symlink asset refusal, installed skill and removal')


def recipe_checks(stage, bundle, versions):
    record = release.read_candidate(versions / '0.1.0')
    assert record['publication'] == 'not_published' and record['notarization'] == 'not_submitted'
    mismatch = stage / 'wrong-cli-version'
    mismatch.mkdir()
    try:
        release.extract_candidate(versions / '0.1.0', mismatch, dict(record, version_source='VERSION'))
        raise AssertionError('CLI/release version mismatch accepted')
    except ValueError as error:
        assert str(error) == 'release CLI/version mismatch'
    unpacked = stage / 'unpacked'
    unpacked.mkdir()
    release.extract_candidate(versions / '0.1.0', unpacked, record)
    for name in dist.files_for('combined') | {dist.MANIFEST}:
        assert (unpacked / name).read_bytes() == (bundle / name).read_bytes()
    for name in release.SKILL_FILES:
        assert (unpacked / 'skill' / name).read_bytes() == (ROOT / 'skills/ui-blueprint' / name).read_bytes()
    broken_skill = {name: (ROOT / 'skills/ui-blueprint' / name).read_bytes()
                    for name in release.SKILL_FILES}
    broken_skill['SKILL.md'] += b'\n[Missing recipe](references/not-packaged.md#use)\n'
    broken_archive = stage / 'broken-skill.tar.gz'
    try:
        release.make_skill_archive(broken_archive, broken_skill)
        raise AssertionError('unpackaged skill reference accepted')
    except ValueError as error:
        assert str(error).startswith('skill link is not packaged:')
    assert not broken_archive.exists()
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


def native_preparer_smoke(bundle, stage):
    """The real shipped preparer must accept its own manifest before target refusal."""
    if not (bundle / 'native-prepare.py').is_file():
        return
    output = Path(tempfile.mkdtemp(prefix='prepare-', dir=stage))
    result = subprocess.run([sys.executable, '-B', str(bundle / 'native-prepare.py'),
        '--pid', '2147483647', '--bundle-id', 'example.nonexistent.app', '--out', str(output)],
        stdin=subprocess.DEVNULL, capture_output=True, timeout=5)
    assert result.returncode == 4 and result.stderr == b'target_unresolved\n' and not result.stdout, (
        'shipped native preparer must accept its distribution manifest', result.returncode, result.stderr)
    assert not list(output.iterdir())
    output.rmdir()
    print('PASS native preparer: actual bundled manifest accepted; missing process refused before output')


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
            manifest = json.loads((bundle / dist.MANIFEST).read_bytes())
            skill_names = release.skill_inventory(manifest['format'])
            for name in skill_names:
                assert (bundle / 'skill' / name).read_bytes() == release.committed(
                    record['source_revision'], f'skills/ui-blueprint/{name}')
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
        if record.get('version_source') == 'VERSION':
            assert run([prefix / 'bin/uiblueprint', '--version'], env=env).stdout == f'uiblueprint {record["version"]}\n'.encode()
        installed = prefix / 'share/uiblueprint/versions' / record['version']
        if with_skill:
            assert skill.resolve().parent == installed.resolve()
            for name in skill_names:
                assert (skill / name).read_bytes() == (bundle / 'skill' / name).read_bytes()
            # All Markdown references stay inside the installed skill, outside checkout.
            import re
            for path in skill.resolve().rglob('*.md'):
                for link in re.findall(r'\]\(([^)]+)\)', path.read_text()):
                    if re.match(r'[A-Za-z][A-Za-z0-9+.-]*:', link) or link.startswith('#'):
                        continue
                    resolved = (path.parent / link.split('#', 1)[0]).resolve()
                    assert resolved.is_relative_to(skill.resolve()) and resolved.is_file(), link
        checks.smoke(installed)
        if json.loads((installed / dist.MANIFEST).read_bytes())['format'] in ('ui-blueprint-local-7', 'ui-blueprint-local-8'):
            # Isolated Python cannot accidentally resolve modules from the source checkout.
            modules = sorted(Path(name).stem for name in dist.CACHE_FILES)
            run([sys.executable, '-I', '-B', '-c',
                 'import importlib,sys;sys.path.insert(0,sys.argv[1]);'
                 '[importlib.import_module(name) for name in sys.argv[2:]];'
                 'from pathlib import Path;import swiftui_debug_capture as capture;'
                 'assert capture.verified_helper(sys.argv[1])==Path(sys.argv[1])/"native-host-helper"',
                 installed, *modules], env=env)
            print('PASS installed capture modules import outside checkout without attachment')
        if json.loads((installed / dist.MANIFEST).read_bytes())['format'] == 'ui-blueprint-local-8':
            caller = installed / 'swiftui_runtime.py'
            assert caller.read_bytes() == release.committed(record['source_revision'], 'plugins/macos/swiftui_runtime.py')
            help_result = run([sys.executable, '-I', '-B', caller, '--help'], env=env)
            assert all(word in help_result.stdout for word in (b'prepare', b'launch', b'windows', b'capture', b'measure'))
            print('PASS installed standalone runtime caller provenance/help outside checkout')
        native_preparer_smoke(installed, stage)
        run([*command, '--uninstall'], env=env)
        assert not (prefix / 'share/uiblueprint').exists()
        assert not skill.is_symlink()
        if with_skill:
            # Homebrew receives the unchanged CLI archive, not the separate skill asset.
            for name in skill_names:
                (bundle / 'skill' / name).unlink()
            for name in ('agents', 'references', ''):
                (bundle / 'skill' / name).rmdir()
        homebrew_formula(folder, bundle, stage)
        print(f'PASS actual candidate: provenance, signing={record["signing"]}, skill={with_skill}, saved-data smoke, real install/run/remove; transport is local')
    finally:
        dist.clean_stage(stage)
        assert not stage.exists()


def cross_inventory_checks(stage, versions, manifest_version):
    """Actual historical installer and receipt formats; synthetic executables, no network."""
    old_version = f'0.0.{manifest_version}'
    old_folder = stage / f'legacy{manifest_version} inventory release'
    old_bundle = stage / f'legacy{manifest_version} inventory bundle'
    old_folder.mkdir()
    old_bundle.mkdir()
    names = dist.files_for('combined') - dist.RUNTIME_FILES - dist.CACHE_FILES - dist.COMPOSE_FILES - dist.NATIVE_CALLER_FILES
    if manifest_version == 1:
        names -= dist.WEB_FILES
    products = {name: ((name + '\n').encode(), 0o644) for name in names}
    for name in release.BINARIES:
        products[name] = (b'#!/bin/sh\nprintf "legacy synthetic executable\\n"\n', 0o755)
    manifest = {'format': f'ui-blueprint-local-{manifest_version}', 'source_revision': 'a' * 40,
                'modules': 'combined', 'files': {name: {'sha256': dist.digest(data), 'mode': mode}
                                                for name, (data, mode) in products.items()}}
    with dist.destination(old_bundle) as fd:
        dist.publish(fd, products, manifest)
    # Pinned historical template: do not regenerate the old installer from current code.
    historical = '907b9b2c87fcb0475a64da939b4eaf8ec9358beb^'
    old_skill_files = tuple(name for name in release.SKILL_FILES
                            if name not in ('references/compose.md', 'references/gpui.md') and name not in release.RUNTIME_SKILL_FILES)
    with patch.object(release, 'SKILL_FILES', old_skill_files):
        release.write_candidates(old_bundle, old_folder, version=old_version, revision='a' * 40,
            minimum='27.0.1', signing='unsigned',
            installer_template=release.committed(historical, 'install.sh').decode(),
            formula_template=(ROOT / 'uiblueprint.rb').read_text(),
            skill_files={name: release.committed(historical, f'skills/ui-blueprint/{name}') for name in old_skill_files})
    prefix = stage / f'cross inventory{manifest_version} prefix'
    skill = stage / f'cross inventory{manifest_version} host/ui-blueprint'
    env = dict(os.environ, PATH=f'{stage / "shims"}:/usr/bin:/bin:/usr/sbin:/sbin',
               DOWNLOAD_FAILURE='yes', TMPDIR=str(stage))
    old = ['sh', old_folder / 'install.sh', '--prefix', prefix, '--skill-dir', skill,
           '--local-release', old_folder]
    new = ['sh', versions / '0.1.0/install.sh', '--prefix', prefix]
    install = [*new, '--local-release', versions / '0.1.0']
    run(old, env=env)
    root = prefix / 'share/uiblueprint'
    current = root / 'current'
    legacy = root / 'versions' / old_version
    assert len((legacy / '.install-sha256').read_text().splitlines()) == len(names) + 1
    assert len((legacy / 'skill/.skill-sha256').read_text().splitlines()) == 6
    receipt = legacy / '.install-sha256'
    original = receipt.read_bytes()
    lines = original.splitlines(keepends=True)
    for mutation in (b''.join(lines[:-1]), lines[0] + b''.join(lines[:-1])):
        receipt.write_bytes(mutation)
        run(install, env=env, ok=False)
        assert os.readlink(current) == f'versions/{old_version}'
    receipt.write_bytes(original)
    # Even a correct hash cannot make a foreign receipt path part of the closed inventory.
    foreign = legacy / 'foreign.txt'
    foreign.write_text('keep')
    receipt.write_bytes(original + f'{dist.digest(foreign.read_bytes())}  foreign.txt\n'.encode())
    run(install, env=env, ok=False)
    assert foreign.read_text() == 'keep'
    receipt.write_bytes(original)
    foreign.unlink()
    skill_receipt = legacy / 'skill/.skill-sha256'
    skill_original = skill_receipt.read_bytes()
    extra = legacy / 'skill/references/compose.md'
    extra.write_text('foreign')
    skill_receipt.write_bytes(skill_original + f'{dist.digest(extra.read_bytes())}  references/compose.md\n'.encode())
    run(install, env=env, ok=False)
    assert extra.read_text() == 'foreign'
    skill_receipt.write_bytes(skill_original)
    extra.unlink()
    run(install, env=env)
    assert os.readlink(current) == 'versions/0.1.0'
    assert len((current / 'skill/.skill-sha256').read_text().splitlines()) == len(release.SKILL_FILES)
    # New code verifies both inventories; the old shipped script cannot be upgraded in place.
    rollback = [*new, '--version', old_version]
    inactive_source = legacy / 'RUN.md'
    inactive_bytes = inactive_source.read_bytes()
    inactive_source.write_bytes(inactive_bytes + b'local edit')
    run(rollback, env=env, ok=False)
    assert os.readlink(current) == 'versions/0.1.0'
    inactive_source.write_bytes(inactive_bytes)
    run(rollback, env=env)
    assert os.readlink(current) == f'versions/{old_version}'
    assert not (skill / 'references/compose.md').exists()
    run(install, env=env)
    assert os.readlink(current) == 'versions/0.1.0'
    run([*new, '--uninstall'], env=env)
    assert not root.exists() and not skill.is_symlink()
    print(f'PASS cross-inventory: manifest{manifest_version}/six-file skill install → manifest5/eight-file update, verified offline rollback/re-update/removal; tampered and foreign inventories refused')



def compose_inventory_upgrade(stage, versions):
    """The installed Compose manifest3 upgrades without losing the matching skill."""
    bundle = stage / 'compose3-bundle'
    bundle.mkdir()
    names = dist.files_for('combined') - dist.RUNTIME_FILES - dist.CACHE_FILES - dist.NATIVE_CALLER_FILES
    products = {name: ((name + '\n').encode(), 0o644) for name in names}
    for name in release.BINARIES:
        products[name] = (b'#!/bin/sh\nexit 0\n', 0o755)
    manifest = {'format': 'ui-blueprint-local-3', 'source_revision': 'a'*40,
        'modules': 'combined', 'files': {n: {'sha256': dist.digest(d), 'mode': m}
                                         for n, (d, m) in products.items()}}
    with dist.destination(bundle) as fd:
        dist.publish(fd, products, manifest)
    candidate = stage / 'compose3-candidate'; candidate.mkdir()
    old_skill_files = tuple(name for name in release.SKILL_FILES if name != 'references/gpui.md' and name not in release.RUNTIME_SKILL_FILES)
    with patch.object(release, 'SKILL_FILES', old_skill_files):
        release.write_candidates(bundle, candidate, version='0.0.9', revision='a'*40,
            minimum='27.0.1', signing='unsigned',
            installer_template=release.committed('beeab158', 'install.sh').decode(),
            formula_template=(ROOT/'uiblueprint.rb').read_text(),
            skill_files={n: release.committed('beeab158', 'skills/ui-blueprint/'+n) for n in old_skill_files})
    prefix = stage / 'compose3-prefix'; skill = stage / 'compose3-skill'
    run(['sh',candidate/'install.sh','--prefix',prefix,'--skill-dir',skill,'--local-release',candidate])
    new = ['sh',versions/'0.1.0/install.sh','--prefix',prefix]
    run([*new,'--local-release',versions/'0.1.0'])
    current = prefix/'share/uiblueprint/current'
    assert (current/'native-prepare.py').is_file()
    assert (skill/'references/gpui.md').is_file()
    run([*new,'--version','0.0.9'])
    assert not (current/'native-prepare.py').exists()
    assert (skill/'references/compose.md').is_file()
    assert not (skill/'references/gpui.md').exists()
    run([*new,'--uninstall'])
    assert not current.exists() and not skill.exists()
    print('PASS manifest3 Compose installation → manifest5 eight-file skill update/rollback/removal')


def native_inventory_upgrade(stage, versions):
    """The actual installed manifest4/skill7 format updates without replacing old bytes."""
    bundle = stage / 'native4-bundle'
    bundle_at(bundle)
    manifest = json.loads((bundle / dist.MANIFEST).read_bytes())
    for name in dist.CACHE_FILES | dist.RUNTIME_FILES:
        del manifest['files'][name]
        (bundle/name).unlink()
    manifest['format'] = 'ui-blueprint-local-4'
    (bundle / dist.MANIFEST).write_bytes(dist.encoded(manifest))
    candidate = stage / 'native4-candidate'
    candidate.mkdir()
    historical = '2e30f7fbc7d136f48a45bc194c4ae2fea6eb086a'
    old_skill_files = tuple(name for name in release.SKILL_FILES if name != 'references/gpui.md' and name not in release.RUNTIME_SKILL_FILES)
    with patch.object(release, 'SKILL_FILES', old_skill_files):
        release.write_candidates(bundle, candidate, version='0.0.10', revision='a'*40,
            minimum='27.0.1', signing='unsigned',
            installer_template=release.committed(historical, 'install.sh').decode(),
            formula_template=(ROOT/'uiblueprint.rb').read_text(),
            skill_files={n: release.committed(historical, 'skills/ui-blueprint/'+n) for n in old_skill_files})
    extracted = stage / 'native4-extracted'
    extracted.mkdir()
    release.extract_candidate(candidate, extracted, release.read_candidate(candidate))
    assert {str(p.relative_to(extracted / 'skill')) for p in (extracted / 'skill').rglob('*')
            if p.is_file()} == set(old_skill_files)
    prefix, skill = stage / 'native4-prefix', stage / 'native4-skill'
    run(['sh', candidate/'install.sh', '--prefix', prefix, '--skill-dir', skill,
         '--local-release', candidate])
    current = prefix/'share/uiblueprint/current'
    original = (current/'skill/.skill-sha256').read_bytes()
    assert len(original.splitlines()) == 7
    new = ['sh', versions/'0.1.0/install.sh', '--prefix', prefix]
    update = [*new, '--local-release', versions/'0.1.0']
    missing = current/'skill/references/macos.md'
    saved = missing.read_bytes()
    missing.unlink()
    run(update, ok=False)
    assert os.readlink(current) == 'versions/0.0.10'
    missing.write_bytes(saved)
    foreign = current/'skill/references/gpui.md'
    foreign.write_text('keep this foreign file')
    run(update, ok=False)
    assert foreign.read_text() == 'keep this foreign file'
    foreign.unlink()
    run(update)
    assert len((current/'skill/.skill-sha256').read_text().splitlines()) == len(release.SKILL_FILES)
    assert (skill/'references/gpui.md').is_file()
    run([*new, '--version', '0.0.10'])
    assert (current/'skill/.skill-sha256').read_bytes() == original
    assert not (skill/'references/gpui.md').exists()
    run(update)
    run([*new, '--uninstall'])
    assert not current.exists() and not skill.exists()
    print('PASS historical manifest4/skill7 → manifest5/skill8 update, offline rollback, re-update/removal; missing/foreign skill files preserved')


def cache_inventory_upgrade(stage, versions):
    """Pinned manifest6 installer upgrades to7 without changing old receipts."""
    historical = '223cf16808ffa9962ecc28018a8d69e372992c94'
    bundle, candidate = stage / 'cache6-bundle', stage / 'cache6-candidate'
    bundle_at(bundle)
    manifest = json.loads((bundle / dist.MANIFEST).read_bytes())
    for name in dist.CACHE_FILES_V7 | dist.RUNTIME_FILES:
        del manifest['files'][name]
        (bundle / name).unlink()
    manifest['format'] = 'ui-blueprint-local-6'
    (bundle / dist.MANIFEST).write_bytes(dist.encoded(manifest))
    candidate.mkdir()
    release.write_candidates(bundle, candidate, version='0.0.11', revision='a'*40,
        minimum='27.0.1', signing='unsigned',
        installer_template=release.committed(historical, 'install.sh').decode(),
        formula_template=(ROOT / 'uiblueprint.rb').read_text(),
        skill_files={n: release.committed(historical, 'skills/ui-blueprint/'+n)
                     for n in release.skill_inventory('ui-blueprint-local-6')})
    extracted = stage / 'cache6-extracted'
    extracted.mkdir()
    release.extract_candidate(candidate, extracted, release.read_candidate(candidate))
    assert not any((extracted / name).exists() for name in dist.CACHE_FILES_V7)
    prefix, skill = stage / 'cache6-prefix', stage / 'cache6-skill'
    run(['sh', candidate/'install.sh', '--prefix', prefix, '--skill-dir', skill,
         '--local-release', candidate])
    current = prefix / 'share/uiblueprint/current'
    old_receipt = (current / '.install-sha256').read_bytes()
    new = ['sh', versions/'0.1.0/install.sh', '--prefix', prefix]
    update = [*new, '--local-release', versions/'0.1.0']
    # A foreign new-module filename in the old format is not legitimized by upgrade.
    foreign = current / 'swiftui_cache_transform.py'
    foreign.write_text('keep foreign file')
    run(update, ok=False)
    assert foreign.read_text() == 'keep foreign file'
    foreign.unlink()
    run(update)
    assert json.loads((current / dist.MANIFEST).read_bytes())['format'] in ('ui-blueprint-local-7', 'ui-blueprint-local-8')
    assert all((current / name).is_file() for name in dist.CACHE_FILES_V7)
    missing = current / 'swiftui_native_parts.py'
    original = missing.read_bytes()
    missing.unlink()
    rollback = [*new, '--version', '0.0.11']
    run(rollback, ok=False)
    assert current.resolve().name == '0.1.0'
    missing.write_bytes(original)
    run(rollback)
    assert (current / '.install-sha256').read_bytes() == old_receipt
    assert not any((current / name).exists() for name in dist.CACHE_FILES_V7)
    run(update)
    run([*new, '--uninstall'])
    assert not current.exists() and not skill.exists()
    print('PASS manifest6 ->7 ->6 rollback/re-update/removal; foreign/missing modules refused')


def runtime_inventory_upgrade(stage, versions):
    """Use the released manifest7 installer and eight-file skill as the baseline."""
    historical = 'd36d18bd8fc07620d449f02c752aebd11c90c408'
    bundle, candidate = stage/'runtime7-bundle', stage/'runtime7-candidate'
    bundle_at(bundle)
    manifest = json.loads((bundle/dist.MANIFEST).read_bytes())
    for name in dist.RUNTIME_FILES:
        del manifest['files'][name]
        (bundle/name).unlink()
    manifest['format'] = 'ui-blueprint-local-7'
    (bundle/dist.MANIFEST).write_bytes(dist.encoded(manifest))
    candidate.mkdir()
    release.write_candidates(bundle, candidate, version='0.0.12', revision='a'*40,
        minimum='27.0.1', signing='unsigned',
        installer_template=release.committed(historical, 'install.sh').decode(),
        formula_template=(ROOT/'uiblueprint.rb').read_text(),
        skill_files={n: release.committed(historical, 'skills/ui-blueprint/'+n)
                     for n in release.skill_inventory('ui-blueprint-local-7')})
    extracted = stage/'runtime7-extracted'; extracted.mkdir()
    release.extract_candidate(candidate, extracted, release.read_candidate(candidate))
    assert not (extracted/'swiftui_runtime.py').exists()
    prefix, skill = stage/'runtime7-prefix', stage/'runtime7-skill'
    run(['sh',candidate/'install.sh','--prefix',prefix,'--skill-dir',skill,'--local-release',candidate])
    current = prefix/'share/uiblueprint/current'
    old_receipt = (current/'.install-sha256').read_bytes()
    old_skill = (skill/'.skill-sha256').read_bytes()
    new = ['sh',versions/'0.1.0/install.sh','--prefix',prefix]
    update = [*new,'--local-release',versions/'0.1.0']
    foreign = skill/'references/swiftui-setup.md'; foreign.write_text('preserve foreign recipe')
    run(update,ok=False); assert foreign.read_text()=='preserve foreign recipe'; foreign.unlink()
    run(update)
    assert json.loads((current/dist.MANIFEST).read_bytes())['format']=='ui-blueprint-local-8'
    assert (current/'swiftui_runtime.py').is_file()
    for name in release.RUNTIME_SKILL_FILES: assert (skill/name).is_file()
    rollback = [*new,'--version','0.0.12']
    missing = skill/'references/swiftui-runtime.md'; original = missing.read_bytes(); missing.unlink()
    run(rollback,ok=False); assert current.resolve().name=='0.1.0'; missing.write_bytes(original)
    run(rollback)
    assert (current/'.install-sha256').read_bytes()==old_receipt
    assert (skill/'.skill-sha256').read_bytes()==old_skill
    assert not (current/'swiftui_runtime.py').exists()
    run(update); run([*new,'--uninstall'])
    assert not current.exists() and not skill.exists()
    print('PASS actual historical7/skill8 ->8/skill10 update, rollback, reactivation, removal and foreign/missing recipe refusal')


def source_release_checks():
    license_text = (ROOT / 'LICENSE').read_text()
    assert license_text.startswith('MIT License\n')
    assert license_text in (ROOT / 'THIRD_PARTY_NOTICES.md').read_text()
    assert license_text in (ROOT / 'skills/ui-blueprint/SKILL.md').read_text()
    workflow = json.loads(run(['ruby', '-rjson', '-ryaml', '-e',
        'puts JSON.generate(YAML.load_file(ARGV[0]))', ROOT / '.github/workflows/release.yml']).stdout)
    trigger = workflow.get('on', workflow.get('true'))
    assert set(trigger) == {'workflow_dispatch'}
    assert trigger['workflow_dispatch']['inputs']['create_draft']['default'] is False
    assert workflow['permissions'] == {'contents': 'read'}
    assert workflow['jobs']['draft']['needs'] == 'build'
    assert workflow['jobs']['draft']['permissions'] == {'contents': 'write'}
    for job in workflow['jobs'].values():
        for step in job['steps']:
            assert not step.get('uses', '').startswith(('actions/upload-artifact@', 'actions/download-artifact@'))
            assert 'gh release upload' not in step.get('run', '')
    command = workflow['jobs']['draft']['steps'][-1]['run']
    assert 'gh release create' in command and '--draft' in command
    assert not any(name in command for name in ('*.tar.gz', 'install.sh', 'SHA256SUMS', 'release.json', 'uiblueprint.rb'))
    guide = (ROOT / 'docs/install.md').read_text()
    assert '--branch "$UIB_TAG" --no-checkout' in guide and '--local-release' in guide
    assert '/releases/latest/download/install.sh' not in guide
    print('PASS MIT in both local packages; source-only draft has no uploaded assets; prompt installation pins release source')


def numeric_version_checks():
    revision = 'a' * 40
    notes = b'# Changes\n\n## 0.1.0\n\nFirst release.\n\n## 0.0.9\nOld release.\n'
    def source(version=b'0.1.0\n', changelog=notes):
        return lambda rev, name: {'VERSION': version, 'CHANGELOG.md': changelog}[name]
    with patch.object(release, 'committed', side_effect=source()):
        assert release.source_version(revision) == '0.1.0'
        for version, body, excluded in [('0.1.0', 'First release.', 'Old release.'),
                                        ('0.0.9', 'Old release.', 'First release.')]:
            generated = release.release_notes(revision, version)
            assert generated.startswith(f'# UI Blueprint {version}\n\n{body}\n')
            assert excluded not in generated
            assert f'Install UI Blueprint v{version} and its matching skill' in generated
            assert f'https://github.com/potapenko/ui-blueprint/blob/v{version}/docs/install.md' in generated
    for bad in (b'v0.1.0\n', b'0.1\n', b'01.1.0\n', b'0.1.0-web.browser2\n',
                b'0.1.0', b'0.1.0\n\n', b' 0.1.0\n'):
        with patch.object(release, 'committed', side_effect=source(bad)):
            try:
                release.source_version(revision)
                raise AssertionError('noncanonical version accepted')
            except ValueError:
                pass
    for bad in (b'## 0.2.0\nOther\n', b'## 0.1.0\n', b'## 0.1.0\nA\n## 0.1.0\nB\n'):
        with patch.object(release, 'committed', side_effect=source(changelog=bad)):
            try:
                release.release_notes(revision, '0.1.0')
                raise AssertionError('missing/empty/duplicate notes accepted')
            except ValueError:
                pass
    with patch.object(release, 'resolve_revision', return_value=revision), \
         patch.object(release, 'committed', side_effect=source()), \
         patch.object(release.dist, 'build') as build:
        try:
            release.prepare(SimpleNamespace(revision=revision, version='0.2.0'))
            raise AssertionError('mismatched version accepted')
        except ValueError:
            pass
        build.assert_not_called()
    assert release.archive_name('0.1.0-web.browser2').endswith('.tar.gz')
    print('PASS numeric release version, committed notes, pre-build mismatch refusal and legacy candidate naming')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--candidate', type=Path)
    args = parser.parse_args()
    source_release_checks()
    numeric_version_checks()
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
        for manifest_version in (1, 2):
            cross_inventory_checks(stage, versions, manifest_version)
        compose_inventory_upgrade(stage, versions)
        native_inventory_upgrade(stage, versions)
        cache_inventory_upgrade(stage, versions)
        runtime_inventory_upgrade(stage, versions)
        hostile_skill_checks(stage, versions)
        local_install_checks(stage, versions)
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
