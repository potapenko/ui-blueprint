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
import time
import platform
import shutil
import signal
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


def run_bounded(command, timeout, *, env=None, data=None, cap=FRAME, build_group=False):
    child = subprocess.Popen(command, stdin=subprocess.PIPE if data is not None else subprocess.DEVNULL,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env, start_new_session=build_group)
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
        if build_group:
            os.killpg(child.pid, signal.SIGTERM)  # isolated compiler group created here
        else:
            child.terminate()  # exact child created here; target app never terminated
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
        if build_group:
            # Cargo/Swift can spawn compiler descendants. Stop only this newly
            # isolated build group, including descendants after its leader exits.
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
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


def sample_frame(args):
    """Own P01 test input only: existing exact-fixture AX sample read, no event."""
    run = args.run_dir.resolve()
    if not args.run_dir.is_absolute() or not run.is_relative_to(Path(tempfile.gettempdir()).resolve()):
        raise ValueError('explicit own system-temp run directory required')
    for executable in (args.cli,args.worker,args.helper):
        if not executable.is_absolute() or not executable.is_file():
            raise ValueError('absolute executable required')
    manifest = json.loads((run/'a.json').read_bytes())
    frame_request = json.loads((run/'sample-frame-request.json').read_bytes())
    keys = ('pid','bundle_id','launch_time','window_id','window_identifier','target_generation','surface_generation')
    binding = {key:manifest[key] for key in keys}
    if (binding != frame_request['binding'] or manifest['window_identifier']!='a'
            or manifest['bundle_id'] not in ('local.uiblueprint.f02.off','local.uiblueprint.f02.on')):
        raise ValueError('exact own A frame request mismatch')
    current = json.loads(Path(manifest['identity_path']).read_bytes())
    if current['state']!='open' or any(current[key]!=binding[key] for key in keys):
        raise ValueError('current own surface mismatch')
    nonce = uuid.uuid4().hex
    context = {'schema_version':'0.1.0','session_id':'sample-frame-'+nonce,
        'target':{'id':'f02-pid-'+str(binding['pid']),'generation':binding['target_generation']},
        'surfaces':[{'id':'window-'+str(binding['window_id']),'generation':binding['surface_generation']}],
        'scope_id':'own-sample-frame','projection':'interaction','fields':FIELDS,
        'plugin':{'id':'macos','version':'0.1.0'},'environment_revision':'own-p01-frame'}
    session = {'allowed_scopes':[context['scope_id']],'session_id':context['session_id'],
        'plugin':context['plugin'],'supported_versions':['0.1.0'],'target':context['target'],
        'surfaces':context['surfaces'],'capabilities':[{'channel':'external_semantics','operation':'observe','status':'partial','reason':'own_sample_scope'}]}
    limits = {'workers':1,'worker_bytes':67108864,'publication_reserve':1048576,'bootstrap_bytes':1048576,
        'parent_bytes':33554432,'input_bytes':2097152,'ingress_bytes':FRAME,'output_bytes':FRAME,
        'request_output_bytes':2097152,'completion_groups':2,'control_bytes':4096,'cleanup_ms':1000,
        'retained_domain_bytes':67108864,'retained_per_worker':15728640,'main_stack_bytes':8388608,'watchdog_stack_bytes':1048576}
    configuration = {'binding':binding,'identity_path':manifest['identity_path'],'scope_id':context['scope_id'],
        'collection':'sample','acquisition_limits':json.loads((ROOT/'tests/bridges/native/acquisition/profile.json').read_bytes()),'acquisition_evidence':False}
    connection = {'connection_version':'1.0.0','target':context['target'],'session':session,'host_limits':limits,
        'attach_deadline_ms':2000,'provider':{'backend':'native_fixture','helper_executable':str(args.helper),'configuration':encode(configuration).decode(),'channels':1}}
    request = {'schema_version':'0.1.0','artifact':{'kind':'request','data':{'clock_domain':'rebound_after_attach',
        'request_id':nonce,'context':context,'limits':{'max_elements':160,'max_depth':9,'max_output_bytes':FRAME,'deadline_ms':1000},
        'freshness_policy':'current_required','operation':{'operation':'observe','channels':['external_semantics']}}}}
    connection_path = run/'frame-connection.json';request_path = run/'frame-request.json'
    try:
        connection_path.write_bytes(encode(connection));request_path.write_bytes(encode(request))
        code,output,_ = run_bounded([str(args.cli),'observe','--connection',str(connection_path),
            '--request',str(request_path),'--worker',str(args.worker),'--max-input-bytes','2097152','--max-output-bytes',str(FRAME)],15)
        if code not in (0,4) or not output:
            raise RuntimeError('bounded own AX frame read failed')
        response = json.loads(output)['artifact']['data']['result']
        if response['status']!='observed':
            raise RuntimeError('AX frame unavailable: '+response['data']['code'])
        snapshot = response['data']
        if snapshot['context'] != context or len(snapshot['nodes']) != 1:
            raise ValueError('AX sample scope mismatch')
        node = snapshot['nodes'][0]
        if node['key']!={'namespace':'macos.ax','key':'f02.sample.a'} or node['surface']!=context['surfaces'][0]:
            raise ValueError('AX sample identity mismatch')
        prop = next(value for value in node['properties'] if value['field']=='accessibility_bounds')
        geometry = prop['state']['value']['value']
        if (prop['state']['availability']!='known' or geometry['frame_kind']!='accessibility_bounds'
                or geometry['coordinate_space']!={'id':'ax-screen','kind':'screen','origin':'top_left','units':'pt'}):
            raise ValueError('AX sample frame space unavailable')
        after = json.loads(Path(manifest['identity_path']).read_bytes())
        if after!=current or frame_request!=json.loads((run/'sample-frame-request.json').read_bytes()):
            raise ValueError('own binding changed during AX frame read')
        value = {'binding':binding,'environment':frame_request['environment'],'ax_rect':geometry['shape']['value']}
        data = encode(value)
        if len(data)>4096:
            raise ValueError('test frame input cap')
        destination = run/'sample-frame-input.json'
        temporary = run/'sample-frame-input.json.tmp'
        with temporary.open('xb') as file:
            file.write(data)
        try:
            os.link(temporary,destination)  # complete JSON, exclusive publication
        finally:
            temporary.unlink()
        print('own AX frame input ready; no events dispatched by runner')
        return 0
    finally:
        connection_path.unlink(missing_ok=True);request_path.unlink(missing_ok=True)


def build_geometry(args):
    """Local selected committed-source build; no install, browser, model or UI."""
    if platform.system() != 'Darwin' or platform.machine() != 'arm64':
        raise ValueError('selected build requires the supported arm64 Mac')
    if not args.output.is_absolute() or not args.output.is_dir():
        raise ValueError('output must be an explicit existing absolute directory')
    output = args.output.resolve()
    names = ('uiblueprint', 'session-worker', 'native-host-helper')
    if any(os.path.lexists(output/name) for name in names):
        raise ValueError('conflicting product file; nothing overwritten')
    stage = Path(tempfile.mkdtemp(prefix='uib-geometry-build-'))
    published = []
    try:
        revision = subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True,timeout=10).strip()
        archive = subprocess.check_output(['git','archive',revision,'Cargo.toml','Cargo.lock','rust-toolchain.toml',
            'crates','plugins/web','plugins/macos','fixtures/native/Observe.swift',
            'tests/bridges/native/Collector.swift','tests/bridges/native/WindowAX.swift'],cwd=ROOT,timeout=30)
        source = stage/'source'
        source.mkdir()
        with tarfile.open(fileobj=io.BytesIO(archive)) as saved:
            saved.extractall(source,filter='data')
        environment = dict(os.environ,CARGO_TARGET_DIR=str(stage/'target'),CARGO_INCREMENTAL='0',CARGO_PROFILE_DEV_DEBUG='0')
        for package, binary, features in [('uiblueprint-cli','uiblueprint','web,macos'),
                                           ('uiblueprint-host','session-worker','web')]:
            code, _, error = run_bounded(['rustup','run','1.96.0','cargo','build','--locked','--offline',
                '--manifest-path',str(source/'Cargo.toml'),'-p',package,'--bin',binary,'--features',features],
                180,env=environment,cap=2*1024*1024,build_group=True)
            if code:
                raise RuntimeError('selected Rust build failed: '+error.decode(errors='replace')[-2000:])
        swift_sources = ['plugins/macos/NativeAcquisition.swift','plugins/macos/NativeJSON.swift',
            'plugins/macos/NativeArtifacts.swift','fixtures/native/Observe.swift','tests/bridges/native/Collector.swift',
            'tests/bridges/native/WindowAX.swift','plugins/macos/HostProtocol.swift',
            'plugins/macos/NativeFocusedAX.swift','plugins/macos/HostHelper.swift']
        code, _, error = run_bounded(['xcrun','swiftc','-parse-as-library','-swift-version','6',
            '-D','HOST_HELPER','-D','CAPTURE_LIBRARY','-target','arm64-apple-macos14.0',
            '-module-cache-path',str(stage/'module-cache'),*[str(source/path) for path in swift_sources],
            '-o',str(stage/'native-host-helper')],120,cap=2*1024*1024,build_group=True)
        if code:
            raise RuntimeError('selected Swift build failed: '+error.decode(errors='replace')[-2000:])
        products = {'uiblueprint':stage/'target/debug/uiblueprint',
            'session-worker':stage/'target/debug/session-worker','native-host-helper':stage/'native-host-helper'}
        for name, product in products.items():
            destination = output/name
            # Exclusive create also protects a conflict appearing after preflight.
            fd = os.open(destination,os.O_WRONLY|os.O_CREAT|os.O_EXCL,0o700)
            identity = os.fstat(fd)
            published.append((destination,identity.st_dev,identity.st_ino))
            with os.fdopen(fd,'wb') as target, product.open('rb') as source_file:
                shutil.copyfileobj(source_file,target)
                os.fchmod(target.fileno(),0o755)
        print(json.dumps({'source_revision':revision,'features':['web','macos'],
            'products':{name:str(output/name) for name in names},
            'sha256':{name:digest(output/name) for name in names},
            'runtime_launched':False,'installed':False},indent=2))
        return 0
    except Exception:
        for path, device, inode in published:
            try:
                info = path.lstat()
                if info.st_dev == device and info.st_ino == inode:
                    path.unlink()
            except FileNotFoundError:
                pass
        raise
    finally:
        # Build-owned non-image files only. Keep any source/SDK image and its
        # containing directory; no recursive removal of retained images.
        image_extensions = {'.png','.jpg','.jpeg','.gif','.webp','.svg','.ico','.icns',
                            '.bmp','.tif','.tiff','.heic','.partial'}
        for path in stage.rglob('*'):
            if (path.is_file() or path.is_symlink()) and path.suffix.lower() not in image_extensions:
                path.unlink()
        for path in sorted((path for path in stage.rglob('*') if path.is_dir()),
                           key=lambda path:len(path.parts),reverse=True):
            if not any(path.iterdir()):
                path.rmdir()
        if not any(stage.iterdir()):
            stage.rmdir()
        else:
            print('retained build image artifacts: '+str(stage))


def geometry(args):
    """Developer example: explicit running PID, public CLI, original Observe input."""
    for executable in (args.cli, args.worker, args.helper):
        if not executable.is_absolute() or not executable.is_file():
            raise ValueError('absolute existing executable required')
    if args.pid <= 0 or args.pid > 2_147_483_647 or len(args.name.encode()) > 4096:
        raise ValueError('explicit positive PID and bounded component name required')
    owned = []
    output = Path(tempfile.mkdtemp(prefix='uib-native-geometry-'))
    def save(name, data):
        path = output / name
        owned.append(path)
        path.write_bytes(data)
        return path
    def call(command, timeout=15, data=None):
        return run_bounded(command, timeout, data=data, cap=FRAME)
    try:
        code, metadata, _ = call([str(args.helper), 'describe-process', str(args.pid)], 3)
        if code:
            print('target_unresolved: supplied process is not a current running Mac app' if code == 4 else 'process metadata lookup failed')
            return code
        metadata = json.loads(metadata)
        if (metadata.get('metadata_version') != '1.0.0' or metadata.get('status') != 'known'
                or metadata.get('process', {}).get('pid') != args.pid):
            raise ValueError('invalid helper process metadata')
        nonce = uuid.uuid4().hex
        fields = ['role', 'accessibility_name', 'accessibility_bounds']
        context = {'schema_version':'0.1.0','session_id':'geometry-'+nonce,
            'target':metadata['target'], 'surfaces':[{'id':'ax-focused-'+nonce,'generation':nonce}],
            'scope_id':'selected-focused-window','projection':'design','fields':fields,
            'plugin':{'id':'macos','version':'0.1.0'},'environment_revision':'geometry-'+nonce}
        session = {'allowed_scopes':[context['scope_id']],'session_id':context['session_id'],
            'plugin':context['plugin'],'supported_versions':['0.1.0'],'target':context['target'],
            'surfaces':context['surfaces'],'capabilities':[{'channel':'external_semantics','operation':'observe',
                'status':'partial','reason':'explicit_public_AXFocusedWindow'}]}
        profile = {'workers':1,'worker_bytes':67108864,'publication_reserve':1048576,'bootstrap_bytes':1048576,
            'parent_bytes':33554432,'input_bytes':2097152,'ingress_bytes':524288,'output_bytes':524288,
            'request_output_bytes':2097152,'completion_groups':2,'control_bytes':4096,'cleanup_ms':1000,
            'retained_domain_bytes':67108864,'retained_per_worker':15728640,'main_stack_bytes':8388608,
            'watchdog_stack_bytes':1048576}
        configuration = {'collection':'focused-ax','process':metadata['process'],'scope_id':context['scope_id'],
            'acquisition_limits':json.loads((ROOT/'tests/bridges/native/acquisition/profile.json').read_bytes())}
        if len(encode(configuration)) > 4032:
            raise ValueError('private configuration bound')
        connection = {'connection_version':'1.0.0','target':context['target'],'session':session,
            'host_limits':profile,'attach_deadline_ms':2000,'provider':{'backend':'native_ax',
                'helper_executable':str(args.helper),'configuration':encode(configuration).decode()}}
        request = {'schema_version':'0.1.0','artifact':{'kind':'request','data':{
            'clock_domain':'rebound_after_attach','request_id':nonce,'context':context,
            'limits':{'max_elements':160,'max_depth':9,'max_output_bytes':FRAME,'deadline_ms':1000},
            'freshness_policy':'current_required','operation':{'operation':'observe','channels':['external_semantics']}}}}
        connection_path = save('connection.json', encode(connection))
        request_path = save('request.json', encode(request))
        start = time.monotonic()
        code, observed, _ = call([str(args.cli),'observe','--connection',str(connection_path),
            '--request',str(request_path),'--worker',str(args.worker),'--max-input-bytes','2097152',
            '--max-output-bytes',str(FRAME)])
        wall_ms = (time.monotonic()-start)*1000
        if code not in (0,4) or not observed:
            print('observe failed; no geometry result')
            return code if code else 1
        response = json.loads(observed)['artifact']['data']['result']
        if response['status'] != 'observed':
            print('observe unavailable: '+response['data']['code'])
            return 4
        snapshot = response['data']
        observed_path = save('observed.json', observed)
        matches = [node for node in snapshot['nodes'] if any(
            prop['field']=='accessibility_name' and prop['state'].get('availability')=='known'
            and prop['state'].get('value',{}).get('value')==args.name for prop in node['properties'])]
        if len(matches) != 1:
            print('component matches in returned record: '+str(len(matches))+'; use an exact SourceKey with public inspect/measure')
            if matches:
                print('candidates='+json.dumps([node['key'] for node in matches], separators=(',',':')))
            return 4
        node = matches[0]
        code, _, _ = call([str(args.cli),'inspect','--snapshot',str(observed_path),'--ref',encode(node['key']).decode(),
            '--view','design','--max-input-bytes','2097152','--max-output-bytes',str(FRAME)], 5)
        if code:
            return code
        bounds = next((prop['state'].get('value',{}).get('value') for prop in node['properties']
            if prop['field']=='accessibility_bounds' and prop['state'].get('availability')=='known'), None)
        if not bounds:
            print('accessibility_bounds unknown for selected component')
            return 4
        values = {}
        for metric in ('width','height'):
            query = {'schema_version':'0.2.0','artifact':{'kind':'geometry_query','data':{
                'id':metric,'scope_id':context['scope_id'],'targets':[node['key']],
                'operation':metric,'quantity_kind':'length','units':'pt',
                'applies_when':{'platform':None,'input_mode':None,'text_scale':None},
                'anchors':[{'element':node['key'],'frame_kind':'accessibility_bounds',
                    'coordinate_space':bounds['coordinate_space'],'fraction':0,'axis':'x' if metric=='width' else 'y'}]}}}
            query_path = save(metric+'-query.json', encode(query))
            code, measured, _ = call([str(args.cli),'measure','--snapshot',str(observed_path),'--query',str(query_path),
                '--space',bounds['coordinate_space']['id'],'--max-input-bytes','2097152',
                '--max-output-bytes',str(FRAME),'--json'],5)
            if code:
                print(metric+' measurement unavailable')
                return code
            case = json.loads(measured)['artifact']['data']
            if case['snapshot'] != snapshot:
                raise ValueError('source Snapshot changed in analysis')
            values[metric] = case['result']['measurement']['value']['value']['amount']
        rect = bounds['shape']['value']
        print('component='+json.dumps(args.name)+' source_key='+encode(node['key']).decode())
        print('accessibility_bounds x='+str(rect['x'])+' y='+str(rect['y'])+' width='+str(values['width'])+
            ' height='+str(values['height'])+' units=pt space='+bounds['coordinate_space']['id'])
        print('coverage='+snapshot['coverage']['status']+' source=saved_observation transform='+
            bounds['transform']['status']+' observe_wall_ms='+format(wall_ms,'.2f'))
        print('hidden layout/paint/hit/clipping unavailable; AX refs are observation-scoped')
        return 0
    finally:
        # Only this example's named non-image inputs/outputs; never recursive
        # deletion of a directory that could contain retained image artifacts.
        for path in owned:
            path.unlink(missing_ok=True)
        try:
            output.rmdir()
        except OSError:
            pass


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
    example = commands.add_parser('geometry', help='developer example: running PID → public AX geometry')
    example.add_argument('--pid', type=int, required=True)
    example.add_argument('--name', required=True, help='exact reported accessibility name in returned Snapshot')
    for name in ('cli', 'worker', 'helper'):
        example.add_argument('--'+name, type=Path, required=True)
    selected = commands.add_parser('build-geometry', help='build selected Web+Mac executables without installation')
    selected.add_argument('--output', type=Path, required=True, help='existing absolute destination; product conflicts refuse')
    frame = commands.add_parser('sample-frame', help='own P01 test-only external AX frame binding')
    frame.add_argument('--run-dir', type=Path, required=True)
    for name in ('cli','worker','helper'):
        frame.add_argument('--'+name,type=Path,required=True)
    args = parser.parse_args()
    if args.mode == 'sample-frame':
        return sample_frame(args)
    if args.mode == 'build-geometry':
        try:
            return build_geometry(args)
        except ValueError as error:
            print(str(error))
            return 2
        except (OSError, RuntimeError, subprocess.SubprocessError) as error:
            print(str(error))
            return 1
    if args.mode == 'geometry':
        try:
            return geometry(args)
        except (ValueError, KeyError, TypeError):
            print('geometry_example_invalid_input')
            return 2
        except (OSError, RuntimeError):
            print('geometry_example_io_or_cleanup_failure')
            return 1
    if args.mode == 'prepare':
        prepare(args)
        return 0
    return live(args)


if __name__ == '__main__':
    raise SystemExit(main())
