#!/usr/bin/env python3
"""Export a pinned, self-contained GPUI SDK source directory into system temp.

Copies only this project's unchanged library sources/license. Generated manifests
replace repository-relative workspace inheritance; no app edits or installation.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[2]


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args], timeout=10)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--revision', required=True)
    parser.add_argument('--out', type=Path, required=True, help='new directory under system temp')
    args = parser.parse_args()
    revision = git('rev-parse', '--verify', args.revision + '^{commit}').decode().strip()
    parent = args.out.parent.resolve(strict=True)
    if not parent.is_relative_to(Path(tempfile.gettempdir()).resolve()):
        parser.error('destination must be under system temp')
    output = parent / args.out.name
    output.mkdir(mode=0o700, exist_ok=False)
    version = git('show', f'{revision}:VERSION').decode().strip()
    inputs = {}
    for prefix, destination in [('plugins/gpui/src/', 'src/'), ('crates/schema/src/', 'schema/src/')]:
        for name in git('ls-tree', '-r', '--name-only', revision, prefix).decode().splitlines():
            if not name.endswith('.rs') or '/src/bin/' in name:
                continue
            data = git('show', f'{revision}:{name}')
            path = output / (destination + name.removeprefix(prefix))
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            inputs[str(path.relative_to(output))] = {'source': name, 'sha256': hashlib.sha256(data).hexdigest()}
    for source, target in [('LICENSE', 'LICENSE'), ('plugins/gpui/Cargo.lock', 'Cargo.lock'), ('plugins/gpui/rust-toolchain.toml', 'rust-toolchain.toml')]:
        (output / target).write_bytes(git('show', f'{revision}:{source}'))
    manifest = git('show', f'{revision}:plugins/gpui/Cargo.toml').decode()
    manifest = manifest.replace('path = "../../crates/schema"', 'path = "schema"')
    manifest = manifest[:manifest.index('[dev-dependencies]')]
    manifest = manifest.replace('[workspace]', '[workspace]\nmembers = ["schema"]')
    manifest = manifest.replace('publish = false', 'publish = false\nautobins = false\nautoexamples = false\nautotests = false')
    manifest += '\n[lints.rust]\nunsafe_op_in_unsafe_fn = "deny"\nunused_must_use = "deny"\n'
    (output / 'Cargo.toml').write_text(manifest)
    root_manifest = tomllib.loads(git('show', f'{revision}:Cargo.toml').decode())
    dependencies = root_manifest['workspace']['dependencies']
    schema = '[package]\nname = "uiblueprint-schema"\nversion = "0.1.0"\nedition = "2024"\nrust-version = "1.100"\npublish = false\nautobins = false\n\n[dependencies]\n'
    for name in ('serde', 'serde_json', 'schemars'):
        value = dependencies[name]
        if isinstance(value, str):
            schema += f'{name} = {json.dumps(value)}\n'
        else:
            schema += f'{name} = {{ version = {json.dumps(value["version"])}, features = {json.dumps(value.get("features", []))} }}\n'
    (output / 'schema/Cargo.toml').write_text(schema)
    (output / 'README.md').write_text('# UI Blueprint GPUI SDK source\n\nThis is a standalone source distribution, not an injected runtime plugin.\nSee sdk-source.json for the exact repository revision, file hashes and generated\nmanifest adaptation. Library Rust sources are unchanged; LICENSE is MIT.\nThird-party crates retain their licenses and resolve from the locked registry;\nno GPUI or other third-party source is copied here.\n\nCopy this directory into the consumer\'s authorized vendor directory. Add an\noptional path dependency to its own vendored copy, enabled only by a Debug\nfeature. No reference to the original UI Blueprint checkout is needed.\n\nThe qualified baseline is Rust1.100.0 and gpui-kit0.9.1/gpui-pre0.3.12.\nAfter gpui_kit::init and other Root plugins, before opening windows, call\nregister(Configuration::application(public_app_id, exact_build_id, unique_run)\n.required_anchors(&["content"]), cx). Resolve Session::for_window(window,cx)\ninside the existing view/event callback. mark(public_label, existing_element)\nrequires a real native ElementId; it adds no layout node. Use structural labels,\nnot private file/account/query values. Repeated rows get separate opaque mounts.\n\nRequest::capture(id,state,deadline,max_bytes) without a component selector captures\nthe registered window root and all current marked descendants. The app owns the\nexplicit trigger and private system-temp output. After completed draw plus\nretirement, Session::take and Capture::to_json produce canonical core0.1 JSON.\nNative on_next_frame callbacks run before drawing in the pinned framework: use\nthe second event after the requested redraw, or a qualified completion callback.\nKeep captures bounded; no polling loop, logging text or production inclusion.\n\nKeep layout/AX/paint/hit/baseline distinct. This collector reports layout only;\nunmarked/virtualized-out nodes are unknown. TableDelegate::render_tr returns a\nfixed Stateful<Div>, so a generic wrapper cannot replace that type; mark actual\ncell/header content roots where the return type permits it and retain separate\nAX row geometry. Never add a layout box or infer unmeasured frame/baseline values.\n\nThe consumer workspace owns its Cargo.lock. The included lock qualifies this\nstandalone package; preserve exact runtime dependency versions on integration.\n')
    original_lock = tomllib.loads((output / 'Cargo.lock').read_text())
    original_packages = {(p['name'], p['version'], p.get('source'), p.get('checksum')) for p in original_lock['package']}
    # Cargo may prune unused example/test packages, but must not upgrade anything.
    subprocess.run(['cargo', '+1.100.0', 'metadata', '--offline', '--format-version', '1'], cwd=output,
                   check=True, stdout=subprocess.DEVNULL, timeout=120)
    actual = tomllib.loads((output / 'Cargo.lock').read_text())
    assert {(p['name'], p['version'], p.get('source'), p.get('checksum')) for p in actual['package']} <= original_packages
    provenance = {'format': 'uiblueprint-gpui-source-1', 'product_version': version,
        'source_revision': revision, 'sdk_version': '0.1.0', 'license': 'MIT',
        'manifest_adaptation': 'standalone local schema dependency; runtime sources unchanged; unused dev packages pruned only',
        'source_files': inputs,
        'files': {str(p.relative_to(output)): hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in output.rglob('*') if p.is_file()}}
    (output / 'sdk-source.json').write_text(json.dumps(provenance, indent=2) + '\n')
    print(json.dumps({'directory': str(output), 'revision': revision, 'product_version': version,
                      'manifest': str(output / 'sdk-source.json')}))


if __name__ == '__main__':
    main()
