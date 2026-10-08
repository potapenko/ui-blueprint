#!/usr/bin/env python3
"""Prepare a saved H01 consumer; live SDK execution needs explicit runtime activation."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import threading
import tempfile
import uuid

ROOT = Path(__file__).resolve().parents[3]
FRAME = 512 * 1024
FIELDS = ['role', 'accessibility_name', 'enabled', 'accessibility_bounds']
WINDOW_FIELDS = ['role', 'description', 'value', 'placeholder', 'enabled', 'focused', 'actions', 'accessibility_bounds']


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def encode(value):
    data = json.dumps(value, ensure_ascii=False, separators=(',', ':')).encode()
    if len(data) + 1 > FRAME:
        raise ValueError('caller frame limit')
    return data


def run_bounded(command, timeout, *, env=None, data=None, cap=FRAME):
    child = subprocess.Popen(command, stdin=subprocess.PIPE if data is not None else subprocess.DEVNULL,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env)
    parts = [bytearray(), bytearray()]
    overflow = [False, False]
    def read(index, stream):
        while True:
            chunk = stream.read(4096)
            if not chunk:
                break
            remaining = cap - len(parts[index])
            parts[index].extend(chunk[:remaining])
            if len(chunk) > remaining:
                overflow[index] = True
        stream.close()
    readers = [threading.Thread(target=read, args=(i, stream)) for i, stream in enumerate((child.stdout, child.stderr))]
    for reader in readers:
        reader.start()
    try:
        if data is not None:
            child.stdin.write(data)
            child.stdin.close()
        code = child.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        child.terminate()  # exact child created here; fixture never terminated by this launcher
        try:
            child.wait(timeout=1)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=1)
        raise RuntimeError('owned caller outer timeout; cleanup unconfirmed') from None
    finally:
        # Reap this exact child even if an early stdin/IO failure bypassed wait.
        if child.poll() is None:
            child.terminate()
            try:
                child.wait(timeout=1)
            except subprocess.TimeoutExpired:
                child.kill()
                child.wait(timeout=1)
        else:
            child.wait()
        for reader in readers:
            reader.join(timeout=2)
        if any(reader.is_alive() for reader in readers):
            raise RuntimeError('owned reader cleanup unconfirmed')
    if any(overflow):
        raise RuntimeError('bounded caller output exceeded')
    return code, bytes(parts[0]), bytes(parts[1])


def new_output(path):
    if not path.is_absolute():
        raise ValueError('explicit absolute output required')
    path = path.resolve()
    application_state = (Path.home() / 'Library/Application Support/UIBlueprint/development/P2/M01-H01').resolve()
    temporary = Path(tempfile.gettempdir()).resolve()
    if path.is_relative_to(ROOT) or not (path.is_relative_to(temporary) or path.is_relative_to(application_state)):
        raise ValueError('new absolute output outside source repository required')
    path.mkdir(mode=0o700, parents=False, exist_ok=False)
    return path


def prepare(args):
    if not args.output.resolve().is_relative_to(Path(tempfile.gettempdir()).resolve()):
        raise ValueError('preparation products require system task-temp')
    output = new_output(args.output)
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    source = output / 'source'
    source.mkdir(mode=0o700)
    # Immutable source snapshot, no branch/worktree or changing shared-owner inputs.
    archive = subprocess.check_output(['git', 'archive', revision], cwd=ROOT, timeout=30)
    with tarfile.open(fileobj=io.BytesIO(archive)) as saved:
        saved.extractall(source, filter='data')
    caller = ROOT / 'crates/host/tests/native_fixture.rs'
    destination = source / 'crates/host/tests/native_fixture.rs'
    destination.write_bytes(caller.read_bytes())
    common = ['plugins/macos/NativeAcquisition.swift', 'plugins/macos/NativeJSON.swift',
              'plugins/macos/NativeArtifacts.swift', 'fixtures/native/Observe.swift',
              'tests/bridges/native/Collector.swift', 'tests/bridges/native/WindowAX.swift']
    helper = output / 'native-host-helper'
    descriptor = output / 'descriptor-collector'
    for target, flags, extra in [(helper, ['-D', 'HOST_HELPER'], ['plugins/macos/HostProtocol.swift', 'plugins/macos/NativeFocusedAX.swift', 'plugins/macos/HostHelper.swift']),
                                 (descriptor, [], [])]:
        command = ['xcrun', 'swiftc', '-parse-as-library', '-swift-version', '6', '-D', 'CAPTURE_LIBRARY',
                   '-target', 'arm64-apple-macos14.0', '-module-cache-path', str(output / 'module-cache'), *flags, *[str(source / p) for p in common + extra], '-o', str(target)]
        code, _, error = run_bounded(command, 120, cap=2 * 1024 * 1024)
        if code:
            raise RuntimeError('focused Swift build failed: ' + error.decode(errors='replace')[-2000:])
    environment = dict(os.environ, CARGO_TARGET_DIR=str(output / 'target'), CARGO_INCREMENTAL='0', CARGO_PROFILE_DEV_DEBUG='0')
    command = ['cargo', '+1.96.0', 'test', '--locked', '--offline', '--no-run', '--message-format=json',
               '-p', 'uiblueprint-host', '--test', 'native_fixture', '--manifest-path', str(source / 'Cargo.toml')]
    code, compiler, error = run_bounded(command, 180, env=environment, cap=2 * 1024 * 1024)
    if code:
        raise RuntimeError('focused Rust build failed: ' + error.decode(errors='replace')[-2000:])
    executable = None
    for line in compiler.splitlines():
        item = json.loads(line)
        if item.get('reason') == 'compiler-artifact' and item.get('target', {}).get('name') == 'native_fixture':
            executable = item.get('executable')
    if not executable:
        raise RuntimeError('consumer artifact missing')
    # Saved SDK/build closure plus task-owned caller; no mutable neighbor evidence.
    inputs = [source / p for p in common + ['plugins/macos/HostProtocol.swift', 'plugins/macos/NativeFocusedAX.swift', 'plugins/macos/HostHelper.swift']]
    inputs += [source / p for p in ('Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml')]
    for owner in ('host', 'schema', 'engine', 'plugin-api'):
        inputs += [source / f'crates/{owner}/Cargo.toml', *sorted((source / f'crates/{owner}/src').rglob('*.rs'))]
    inputs.append(destination)
    hashes = {str(p.relative_to(source)): digest(p) for p in inputs}
    receipt = {'source_revision': revision, 'caller_sha256': digest(caller),
               'consumer': executable, 'helper': str(helper), 'descriptor_collector': str(descriptor),
               'consumer_sha256': digest(Path(executable)), 'helper_sha256': digest(helper),
               'descriptor_sha256': digest(descriptor), 'source_sha256': hashes,
               'source': str(source), 'runtime_launched': False,
               'profile': str(source / 'tests/bridges/native/acquisition/profile.json'),
               'retention_owner': 'Native/root; retain for M01 qualification or explicit cleanup'}
    (output / 'prepared.json').write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps({'prepared': str(output / 'prepared.json'), 'runtime_launched': False}))


def live(args):
    if not args.allow_live:
        raise ValueError('runtime packet must explicitly activate --allow-live')
    prepared = json.loads(args.prepared.read_text())
    for key, hash_key in [('consumer', 'consumer_sha256'), ('helper', 'helper_sha256'), ('descriptor_collector', 'descriptor_sha256')]:
        if digest(Path(prepared[key])) != prepared[hash_key]:
            raise ValueError('prepared binary identity changed')
    manifest_bytes = args.manifest.read_bytes()
    if len(manifest_bytes) > FRAME:
        raise ValueError('manifest bound')
    manifest = json.loads(manifest_bytes)
    if not (manifest['bundle_id'] in ('local.uiblueprint.f02.off', 'local.uiblueprint.f02.on') and manifest['window_identifier'] == 'a'
            and manifest['role'] == 'a' and manifest['source_state']['stimulus'] == 'normal'
            and manifest['snapshot_request'] > 0 and manifest['collection_mode'] == 'explicit_request_only'
            and manifest['state']['popup'] is False):
        raise ValueError('fresh owned F02 A normal Snapshot required')
    output = new_output(args.output)
    inputs = output / 'input'
    inputs.mkdir(mode=0o700)
    probe = manifest.get('probe_enabled') is True
    fields = ['layout_bounds'] if probe else FIELDS
    channel = 'opt_in_layout_probe' if probe else 'external_semantics'
    nonce = str(uuid.uuid4())
    context = {'schema_version': '0.1.0', 'session_id': f'native-host-{nonce}',
               'target': {'id': f"f02-pid-{manifest['pid']}", 'generation': manifest['target_generation']},
               'surfaces': [{'id': f"window-{manifest['window_id']}", 'generation': manifest['surface_generation']}],
               'scope_id': f"fixture-window-{manifest['window_id']}-sample", 'projection': 'design' if probe else 'interaction', 'fields': fields,
               'plugin': {'id': 'macos', 'version': '0.1.0'}, 'environment_revision': f'fixture-environment-{nonce}'}
    request = {'schema_version': '0.1.0', 'artifact': {'kind': 'request', 'data': {
        'clock_domain': 'metadata_only_not_an_observation_clock', 'request_id': f'host-request-{nonce}', 'context': context,
        'limits': {'max_elements': 160, 'max_depth': 9, 'max_output_bytes': FRAME, 'deadline_ms': 1000},
        'freshness_policy': 'cached_allowed' if probe else 'current_required', 'operation': {'operation': 'observe', 'channels': [channel]}}}}
    descriptor_request = json.loads(encode(request))
    descriptor_request['artifact']['data']['context']['fields'] = WINDOW_FIELDS
    descriptor_request['artifact']['data']['operation']['channels'] = ['external_semantics']
    # Metadata only: no tree/pixels or Ticket. SessionDescriptor has no fields;
    # the real four-field Request gets actual Attached clock inside the Rust caller.
    code, descriptor, _ = run_bounded([prepared['descriptor_collector'], str(args.manifest.resolve()), str(output),
                                       'describe-window', prepared['profile']], 2, data=encode(descriptor_request) + b'\n')
    if code:
        raise RuntimeError('descriptor-only target/permission metadata failed; no raw diagnostics saved')
    if probe:
        session = json.loads(descriptor)
        session['artifact']['data']['capabilities'] = [{'channel':'opt_in_layout_probe','operation':'observe',
            'status':'partial','reason':'explicit_measured_fixture_snapshot_import'}]
        descriptor = encode(session)
    (inputs / 'session.json').write_bytes(descriptor)
    (inputs / 'request.json').write_bytes(encode(request))
    binding = {k: manifest[k] for k in ('pid', 'bundle_id', 'launch_time', 'window_id', 'window_identifier', 'target_generation', 'surface_generation')}
    config = {'binding': binding, 'scope_id': context['scope_id'], 'collection': 'sample',
              'identity_path': manifest['identity_path'],
              'acquisition_limits': json.loads(Path(prepared['profile']).read_text()), 'acquisition_evidence': False}
    if probe:
        config.update(probe_manifest_path=str(args.manifest.resolve()), probe_snapshot_request=manifest['snapshot_request'],
                      probe_source_revision=manifest['source_state']['revision'], probe_uptime=manifest['uptime_seconds'])
        query = json.loads((ROOT / 'fixtures/analysis/query-gap.json').read_text())
        q = query['artifact']['data']; q['scope_id'] = context['scope_id']
        keys = [{'namespace':'macos.swiftui.probe','key':f'f02.sample.a.{name}'} for name in ('icon','text')]
        q['targets'] = keys; q['units'] = 'pt'
        for anchor, key in zip(q['anchors'], keys):
            anchor['element'] = key
            anchor['coordinate_space'] = {'id':'f02-fixture-local','kind':'local','units':'pt','origin':'top_left'}
        (inputs / 'query.json').write_bytes(encode(query))
    config_bytes = encode(config)
    if len(config_bytes) > 4032:
        raise ValueError('Native config envelope')
    (inputs / 'helper.json').write_bytes(config_bytes)
    environment = dict(os.environ, UIB_NATIVE_FIXTURE_LIVE='owned_f02_a', UIB_NATIVE_INPUT_DIR=str(inputs),
                       UIB_NATIVE_OUTPUT_DIR=str(output / 'host'), UIB_NATIVE_HELPER=prepared['helper'])
    code, _, diagnostic = run_bounded([prepared['consumer'], '--ignored', '--exact', 'actual_owned_f02_ax_observation',
                                      '--test-threads=1'], 15, env=environment, cap=16 * 1024)
    host_report_path = output / 'host/host-report.json'
    if not host_report_path.exists():
        raise RuntimeError('bounded host receipt missing; cleanup not inferred')
    host = json.loads(host_report_path.read_text())
    if code or not host['cleanup_confirmed'] or not host['caller_ok']:
        raise RuntimeError('H01 failure/cleanup unconfirmed; inspect fixed host report, no retry')
    response_path = output / 'host/channel-0.json'
    response_bytes = response_path.read_bytes()
    if len(response_bytes) > FRAME:
        raise ValueError('committed response bound')
    valid_code, _, _ = run_bounded([str(args.validator.resolve()), '--max-bytes', str(FRAME), str(response_path)], 3, cap=4096)
    if valid_code:
        raise RuntimeError('committed canonical response invalid')
    response = json.loads(response_bytes)['artifact']['data']
    assert response['request_id'] == request['artifact']['data']['request_id'] and response['session_id'] == context['session_id']
    assert response['target'] == context['target'] and response['channel'] == channel and response['dispatch_sequence'] > 0
    if response['result']['status'] == 'failed':
        code = response['result']['data']['code']
        print(json.dumps({'result': code, 'canonical_ack': True, 'cleanup_confirmed': True, 'retry': False}))
        return 3  # negative permission/partial result never closes the positive gate
    snapshot = response['result']['data']
    assert snapshot['context'] == json.loads((output / 'host/submitted-request.json').read_bytes())['artifact']['data']['context']
    if probe:
        assert snapshot['coverage']['status'] == 'partial' and len(snapshot['nodes']) == 3
        assert snapshot['observations'][0]['freshness'] == 'unverified'
        assert snapshot['observations'][0]['start'] == manifest['uptime_seconds']
        assert snapshot['components'][0]['logical_component_key'] == 'f02.sample.a'
        measurement_path = output / 'host/measurement.json'
        valid, _, _ = run_bounded([str(args.validator.resolve()), '--max-bytes', str(FRAME), str(measurement_path)], 3, cap=4096)
        if valid: raise RuntimeError('guarded measurement contract invalid')
        measured = json.loads(measurement_path.read_bytes())['artifact']['data']['result']
        assert measured['status'] == 'known'
        value = measured['measurement']['value']['value']
        oracle = json.loads((ROOT / 'fixtures/native/expectations.json').read_text())['gap_pt']
        expected = oracle['expanded' if manifest['state']['expanded'] else 'baseline']
        assert value['amount'] == expected and value['source_units'] == 'pt'
        after = json.loads(args.manifest.read_bytes())
        assert after['state'] == manifest['state'] and after['source_state'] == manifest['source_state']
        report = {'result':'acknowledged_measured_probe_and_guarded_gap','gap_pt':value['amount'],
                  'freshness':'unverified','canonical_ack':True,'cleanup_confirmed':True,
                  'fixture_state_unchanged':True,'capture_requested':False,'off_on_invariance':False}
        (output / 'result.json').write_text(json.dumps(report,indent=2)+'\n')
        print(json.dumps(report)); return 0
    assert snapshot['coverage']['status'] == 'partial' and len(snapshot['nodes']) == 1
    node = snapshot['nodes'][0]
    assert node['key'] == {'namespace': 'macos.ax', 'key': 'f02.sample.a'} and node['surface'] == context['surfaces'][0]
    props = {p['field']: p['state'] for p in node['properties']}
    assert set(props) == set(FIELDS)
    assert props['role']['value']['value'] == 'button'
    assert props['enabled']['value']['value'] is True
    assert 'Activate sample' in props['accessibility_name']['value']['value']  # authored combined-label text; no invented AX exact string
    bounds = props['accessibility_bounds']['value']['value']
    assert bounds['frame_kind'] == 'accessibility_bounds' and bounds['coordinate_space']['units'] == 'pt'
    after = json.loads(args.manifest.read_bytes())
    assert after['state'] == manifest['state'] and after['source_state'] == manifest['source_state']
    report = {'result': 'acknowledged_owned_f02_a_ax', 'requested_fields': FIELDS, 'coverage': 'partial',
              'canonical_response': str(response_path), 'cleanup_confirmed': True, 'fixture_state_unchanged': True,
              'raw_diagnostics_persisted': False, 'diagnostic_bytes': len(diagnostic), 'capture_requested': False,
              'retention_owner': 'root/M01; retain selected canonical evidence until acceptance or explicit cleanup'}
    (output / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))
    return 0


def main():
    parser = argparse.ArgumentParser()
    commands = parser.add_subparsers(dest='mode', required=True)
    build = commands.add_parser('prepare')
    build.add_argument('--output', type=Path, required=True)
    runtime = commands.add_parser('run')
    runtime.add_argument('--prepared', type=Path, required=True)
    runtime.add_argument('--manifest', type=Path, required=True)
    runtime.add_argument('--validator', type=Path, required=True)
    runtime.add_argument('--output', type=Path, required=True)
    runtime.add_argument('--allow-live', action='store_true')
    args = parser.parse_args()
    if args.mode == 'prepare':
        prepare(args)
        return 0
    return live(args)


if __name__ == '__main__':
    raise SystemExit(main())
