#!/usr/bin/env python3
"""Offline bundle smoke plus focused publication/removal safety checks; no UI."""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
from unittest.mock import patch

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('distribution', ROOT / 'distribution.py')
dist = importlib.util.module_from_spec(spec)
spec.loader.exec_module(dist)


def call(command, expected=0, env=None):
    result = subprocess.run([str(x) for x in command], cwd='/', env=env,
                            capture_output=True, timeout=30)
    assert result.returncode == expected, (command, result.returncode, result.stderr)
    return result


def smoke(bundle):
    # Runtime environment deliberately has no model credentials or build-tool PATH.
    env = {'PATH': '/usr/bin:/bin', 'TMPDIR': tempfile.gettempdir()}
    with dist.destination(bundle) as fd:
        manifest, _ = dist.verify(fd)
    names = {p['name'] for p in manifest['dependencies']}
    assert 'jsonschema' not in names
    assert ('tungstenite' in names) == (manifest['modules'] in ('web', 'combined'))
    assert ('libc' in names) == (manifest['modules'] != 'core')
    assert ('native-host-helper' in manifest['files']) == (manifest['modules'] in ('native', 'combined'))
    licenses = (bundle / 'DEPENDENCY_LICENSES.txt').read_text()
    assert 'UNICODE LICENSE V3' in licenses and 'copyright Alexander Huszagh' in licenses
    assert 'Copyright notices for The Rust Standard Library' in (bundle / 'RUST_LIBRARY_NOTICES.html').read_text()
    call([sys.executable, bundle / 'distribution.py', 'verify', '--destination', bundle], env=env)
    cli, validator = bundle / 'uiblueprint', bundle / 'uiblueprint-validate'
    assert b'native-session --connection' in call([cli, '--help'], env=env).stdout
    native = manifest['modules'] in ('native', 'combined')
    entry = call([cli, 'native-session'], 2 if native else 5, env)
    assert not entry.stdout
    assert entry.stderr.strip() == (b'invalid_arguments' if native else b'unsupported_command')
    for name in ('snapshot', 'query', 'evaluation', 'expectation'):
        result = call([validator, '--max-bytes', '131072', bundle / f'example-{name}.json'], env=env)
        assert json.loads(result.stdout)['valid'] is True
    limits = ['--max-input-bytes', '131072', '--max-output-bytes', '131072']
    result = call([cli, 'measure', '--snapshot', bundle / 'example-snapshot.json',
                   '--query', bundle / 'example-query.json', '--evaluation', bundle / 'example-evaluation.json',
                   '--space', 'local-form', *limits, '--json'], env=env)
    measurement = json.loads(result.stdout)['artifact']['data']['result']['measurement']
    assert measurement['value']['value'] == {'amount': 8.0, 'kind': 'length', 'source_units': 'css_px'}
    result = call([cli, 'check', '--snapshot', bundle / 'example-snapshot.json',
                   '--expectation', bundle / 'example-expectation.json', '--space', 'local-form',
                   *limits, '--json', '--result-version', '0.2.0'], env=env)
    assert json.loads(result.stdout)['artifact']['data']['finding']['status'] == 'pass'
    stage = Path(tempfile.mkdtemp(prefix='uib-dist-smoke-'))
    try:
        for purpose, brief in [('document', 'observed'), ('propose', 'proposed')]:
            result = call([cli, 'imagegen-prompt', '--brief', bundle / f'example-{brief}-brief.json',
                           '--purpose', purpose, '--out', stage / purpose,
                           '--max-input-bytes', '1048576', '--max-output-bytes', '1048576',
                           '--max-components', '100', '--max-views', '10',
                           '--components-per-detail', '10', '--json'], env=env)
            receipt = json.loads(result.stdout)
            assert receipt['generated_image'] is False and receipt['status'] == 'package_written'
            assert set(p.name for p in (stage / purpose).iterdir()) == {
                'manifest.json', 'drawing-brief.md', 'scene.json', 'dimensions.json', 'sheets.json', 'prompt.txt'}
            assert receipt['result_version'] == '0.1.0'
            assert json.loads((stage / purpose / 'manifest.json').read_text())['package_version'] == '0.1.0'
        # Two saved synthetic observations, with one literal width change. No UI or model.
        before = json.loads((bundle / 'example-snapshot.json').read_text())
        after = json.loads(json.dumps(before))
        after['artifact']['data']['nodes'][0]['properties'][0]['state']['value']['value']['shape']['value']['width'] = 34
        (stage / 'after.json').write_text(json.dumps(after))
        brief = json.loads((bundle / 'example-observed-brief.json').read_text())
        side = {'title': 'Synthetic local geometry', 'state': 'saved fixture',
                'scope': 'two authored rectangles', 'environment': 'synthetic fixture; no live collection',
                'safe_source_reference': 'bundled analysis fixture', 'not_depicted': [],
                'public_text_fields': []}
        metadata = {'metadata': brief['metadata'], 'before': side, 'after': side,
                    'geometry_space': 'local-form', 'different_basis': None}
        (stage / 'compare-metadata.json').write_text(json.dumps(metadata))
        result = call([cli, 'imagegen-prompt', '--before', bundle / 'example-snapshot.json',
                       '--after', stage / 'after.json', '--metadata', stage / 'compare-metadata.json',
                       '--purpose', 'compare', '--out', stage / 'compare',
                       '--max-input-bytes', '1048576', '--max-output-bytes', '1048576',
                       '--max-components', '100', '--max-views', '10',
                       '--components-per-detail', '10', '--json'], env=env)
        receipt = json.loads(result.stdout)
        assert receipt['result_version'] == '0.2.0' and receipt['generated_image'] is False
        assert json.loads((stage / 'compare' / 'manifest.json').read_text())['package_version'] == '0.2.0'
        assert set(p.name for p in (stage / 'compare').iterdir()) == {
            'manifest.json', 'drawing-brief.md', 'scene.json', 'dimensions.json', 'sheets.json', 'prompt.txt'}
        scene = json.loads((stage / 'compare' / 'scene.json').read_text())
        comparison = scene['comparison_results'][0]
        assert comparison['omitted_entries'] == 0
        assert any(e['field'] == 'layout_bounds' and e['content_changed'] for e in comparison['entries'])
        geometry = next(g for g in comparison['geometry']
                        if g['before_index'] == 0 and g['field'] == 'layout_bounds')
        assert geometry['displacement'] == {'dx': 0, 'dy': 0, 'dwidth': 4, 'dheight': 0}
        missing = call([cli, 'measure', '--snapshot', stage / 'absent.json',
                        '--query', bundle / 'example-query.json', '--space', 'local-form', *limits], 1, env)
        assert not missing.stdout
    finally:
        dist.clean_stage(stage)
        assert not stage.exists()
    print(f"PASS {manifest['modules']}: hashes/inventory, four validators, Native entry/gate, gap8, check-pass, document/propose/compare, missing input")


def safety():
    stage = Path(tempfile.mkdtemp(prefix='uib-dist-safety-'))
    try:
        folder = stage / 'bundle'; folder.mkdir()
        (folder / 'unrelated.txt').write_text('leave me')
        products = {name: (name.encode(), 0o644) for name in dist.files_for('core')}
        manifest = {'format': 'ui-blueprint-local-1', 'modules': 'core', 'source_revision': 'synthetic',
                    'files': {name: {'sha256': dist.digest(data), 'mode': mode}
                              for name, (data, mode) in products.items()}}
        with dist.destination(folder) as fd:
            # A late collision must roll back only newly-created files.
            (folder / 'uiblueprint').write_text('foreign')
            try:
                dist.publish(fd, products, manifest)
                raise AssertionError('collision accepted')
            except FileExistsError:
                pass
            assert {p.name for p in folder.iterdir()} == {'unrelated.txt', 'uiblueprint'}
            assert (folder / 'uiblueprint').read_text() == 'foreign'
            (folder / 'uiblueprint').unlink()
            # Fault injected after files have been created: no completion marker/success.
            real_fsync = os.fsync
            count = [0]
            def fail_sync(value):
                count[0] += 1
                if count[0] == 3:
                    raise OSError('injected disk failure')
                real_fsync(value)
            with patch.object(dist.os, 'fsync', side_effect=fail_sync):
                try:
                    dist.publish(fd, products, manifest)
                    raise AssertionError('write failure accepted')
                except OSError:
                    pass
            assert {p.name for p in folder.iterdir()} == {'unrelated.txt'}
            dist.publish(fd, products, manifest)
            (folder / 'uiblueprint').write_text('modified')
            try:
                dist.remove(fd)
                raise AssertionError('changed file removed')
            except ValueError:
                pass
            assert len(list(folder.iterdir())) == len(products) + 2
            (folder / 'uiblueprint').write_bytes(products['uiblueprint'][0])
            (folder / 'uiblueprint').unlink()
            (folder / 'uiblueprint').symlink_to(folder / 'unrelated.txt')
            try:
                dist.remove(fd)
                raise AssertionError('symlink accepted')
            except OSError:
                pass
            (folder / 'uiblueprint').unlink()
            (folder / 'uiblueprint').write_bytes(products['uiblueprint'][0])
            bad = dict(manifest, files=dict(manifest['files']))
            bad['files']['../outside'] = bad['files'].pop('RUN.md')
            (folder / dist.MANIFEST).write_bytes(dist.encoded(bad))
            try:
                dist.remove(fd)
                raise AssertionError('path traversal accepted')
            except ValueError:
                pass
            (folder / dist.MANIFEST).write_bytes(dist.encoded(manifest))
            dist.remove(fd)
            assert {p.name for p in folder.iterdir()} == {'unrelated.txt'}
            assert (folder / 'unrelated.txt').read_text() == 'leave me'
            # Same destination can be reinstalled after owned removal.
            dist.publish(fd, products, manifest)
            dist.verify(fd)
            dist.remove(fd)
        manager = ROOT / 'distribution.py'
        call([sys.executable, manager, 'build', '--modules', 'core', '--destination', 'relative'], 1)
        call([sys.executable, manager, 'build', '--modules', 'core', '--destination', stage / 'missing'], 1)
        result = call([sys.executable, manager, 'build', '--modules', 'core', '--destination', folder], 1,
                      dict(os.environ, PATH=''))
        assert b'missing required tool: git' in result.stderr
        call([sys.executable, manager, 'build', '--modules', 'core', '--destination', folder,
              '--revision', 'not-an-existing-revision'], 1)
        assert {p.name for p in folder.iterdir()} == {'unrelated.txt'}
        print('PASS safety: collision, write failure rollback, modified file, symlink, path traversal, scoped remove/reinstall, missing tool/revision/destination')
    finally:
        dist.clean_stage(stage)
        assert not stage.exists()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bundle', type=Path, action='append', default=[])
    args = parser.parse_args()
    safety()
    for bundle in args.bundle:
        smoke(bundle.resolve())
