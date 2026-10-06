#!/usr/bin/env python3
"""One actual F02 window AX sizing input, through committed common support."""
import argparse
import hashlib
import json
import pathlib
import os
import signal
import subprocess
import uuid
import prove

FRAME = 524_288
PENDING = 1_048_576
FIELDS = ['role', 'description', 'value', 'placeholder', 'enabled', 'focused', 'actions', 'accessibility_bounds']
prove.FRAME = FRAME


def main():
    parser = argparse.ArgumentParser()
    for name in ('host', 'validator', 'collector', 'manifest', 'output'):
        parser.add_argument(f'--{name}', required=True, type=pathlib.Path)
    parser.add_argument("--check-canary-env", help="Name of a one-use test environment value; removed before child launch")
    args = parser.parse_args()
    canary = os.environ.pop(args.check_canary_env).encode() if args.check_canary_env else None
    diagnostics_bytes = 0
    def private_check(data):
        if canary and canary in data: raise RuntimeError("fixture canary refused before persistence")
    def checked_validate(path):
        nonlocal diagnostics_bytes
        result = subprocess.run([str(args.validator), "--max-bytes", str(FRAME), str(path)], capture_output=True, timeout=3)
        private_check(result.stdout); private_check(result.stderr)
        diagnostics_bytes += len(result.stderr)
        if result.returncode != 0: raise RuntimeError("canonical validator rejected sizing document")
    args.output.mkdir(parents=True, exist_ok=False)
    manifest = json.loads(args.manifest.read_text())
    assert manifest['state']['popup'] is False and manifest['source_state']['stimulus'] == 'normal'
    nonce = str(uuid.uuid4())
    context = {'schema_version': '0.1.0', 'session_id': f'native-sizing-{nonce}',
               'target': {'id': f"f02-pid-{manifest['pid']}", 'generation': manifest['target_generation']},
               'surfaces': [{'id': f"window-{manifest['window_id']}", 'generation': manifest['surface_generation']}],
               'scope_id': f"fixture-window-{manifest['window_id']}-strict-AX", 'projection': 'interaction', 'fields': FIELDS,
               'plugin': {'id': 'macos', 'version': '0.1.0'}, 'environment_revision': f'fixture-environment-{nonce}'}
    request = {'schema_version': '0.1.0', 'artifact': {'kind': 'request', 'data': {
        'clock_domain': f'sizing-parent-{nonce}', 'request_id': f'sizing-request-{nonce}', 'context': context,
        'limits': {'max_elements': 160, 'max_depth': 9, 'max_output_bytes': FRAME, 'deadline_ms': 1000},
        'freshness_policy': 'current_required', 'operation': {'operation': 'observe', 'channels': ['external_semantics']}}}}
    (args.output/'fixture-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    request_file = args.output/'request.json'
    request_file.write_bytes(prove.encoded(request))
    checked_validate(request_file)
    descriptor = subprocess.run([str(args.collector), str(args.manifest), str(args.output), 'describe-window'],
                                input=prove.encoded(request), capture_output=True, timeout=2)
    private_check(descriptor.stdout); private_check(descriptor.stderr)
    diagnostics_bytes += len(descriptor.stderr)
    assert descriptor.returncode == 0 and len(descriptor.stdout) <= FRAME
    session_file = args.output/'session.json'
    session_file.write_bytes(descriptor.stdout)
    checked_validate(session_file)
    owned, readers, cleanup, receipts = [], [], [], []
    try:
        host = subprocess.Popen([str(args.host), str(session_file), str(request_file), str(FRAME), str(PENDING),
                                 'complete', '1000', '4', str(args.output/'retained')],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        owned.append(host)
        hr = prove.Reader(host.stdout); readers.append(hr)
        ticket = json.loads(hr.get(2)); receipts.append(ticket)
        assert ticket['event'] == 'ticket' and ticket['session_id'] == context['session_id']
        collector = subprocess.Popen([str(args.collector), str(args.manifest), str(args.output), 'window-ax', str(ticket['sequence'])],
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        owned.append(collector)
        cr = prove.Reader(collector.stdout); readers.append(cr)
        collector.stdin.write(prove.encoded(request)); collector.stdin.close()
        wire = cr.get(1)
        # Preserve exact returned bytes/member order, not a reconstructed document.
        wire_path = args.output/'returned-wire.ndjson'
        private_check(wire)
        wire_path.write_bytes(wire)
        host.stdin.write(wire); host.stdin.flush()
        accepted = json.loads(hr.get(1)); receipts.append(accepted)
        assert accepted['event'] == 'frame_accepted'
        terminal = json.loads(hr.get(1)); receipts.append(terminal)
        assert terminal['event'] == 'terminal' and terminal['terminal'] == 'Completed'
        host.stdin.close()
        assert host.wait(timeout=1) == 0 and collector.wait(timeout=1) == 0
        for process in (collector, host):
            diagnostic = process.stderr.read(16_385)
            assert len(diagnostic) <= 16_384, "diagnostic cap exceeded"
            private_check(diagnostic); diagnostics_bytes += len(diagnostic)
            process.stderr.close()
        checked_validate(wire_path)
        document = json.loads(wire)
        assert document['artifact']['data']['result']['status'] == 'observed'
        retained = json.loads((args.output/'retained/channel-0.json').read_text())
        private_check((args.output/'retained/channel-0.json').read_bytes())
        assert retained == document
        snapshot = document['artifact']['data']['result']['data']
        acquisition = json.loads((args.output/'acquisition.json').read_text())
        assert len(snapshot['nodes']) == acquisition['returned_nodes'] <= 160
        assert all(n['surface'] == context['surfaces'][0] for n in snapshot['nodes'])
        assert all(len(n['properties']) == len(FIELDS) for n in snapshot['nodes'])
        result = {'support_revision': '73d772e97efcf550ea4a4d3e8480b56509ebc548',
                  'wire_bytes_including_newline': len(wire), 'wire_sha256': hashlib.sha256(wire).hexdigest(),
                  'exact_wire_path': str(wire_path), 'retained_equals_returned': True,
                  'request_scope': context['scope_id'], 'fields': FIELDS, 'extra_runtime_metadata': ['AXIdentifier', 'AXSubrole'],
                  'limits': {'node_ceiling': 160, 'depth_ceiling': 9, 'frame_bytes': FRAME,
                             'pending_encoded_bytes': PENDING, 'deadline_ms': 1000, 'max_in_flight': 1, 'max_frames': 4},
                  'acquisition': acquisition, 'host_receipts': receipts,
                  'new_pixels_collected': False, 'configured_bound_coverage': 'not_claimed',
                  'full_D05_acceptance': False, 'canary_check_requested': canary is not None,
                  'canary_absent': True if canary else None, 'diagnostics_bytes': diagnostics_bytes}
    finally:
        for process in reversed(owned): cleanup.append(prove.reap(process))
        for reader in reversed(readers): reader.finish()
    result['cleanup'] = cleanup
    result['platform_readers_joined'] = True
    (args.output/'report.json').write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps({'returned_nodes': acquisition['returned_nodes'], 'wire_bytes': len(wire),
                      'coverage': 'partial', 'retained_equality': True, 'cleanup': 'reaped_and_joined'}))


if __name__ == '__main__':
    def watchdog(_signum, _frame): raise TimeoutError('finite sizing process watchdog')
    signal.signal(signal.SIGALRM, watchdog)
    signal.alarm(15)
    try: main()
    finally: signal.alarm(0)
