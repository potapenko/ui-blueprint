#!/usr/bin/env python3
"""Finite native wiring around the committed common host, not another lifecycle."""
import argparse
import copy
import json
import pathlib
import queue
import signal
import subprocess
import threading
import time
import uuid

FRAME = 65_536
PENDING = 131_072
DEADLINE_MS = 3_000
AX_SECONDS = 1.0
CAPTURE_SECONDS = 2.0
CLEANUP_SECONDS = 1.0


def encoded(value):
    result = json.dumps(value, separators=(',', ':'), ensure_ascii=False).encode() + b'\n'
    if len(result) > FRAME:
        raise ValueError('frame exceeds explicit proof cap')
    return result


class Reader:
    """One bounded byte reader; producer must be reaped before join."""
    def __init__(self, stream):
        self.stream = stream
        self.items = queue.Queue(maxsize=1)
        self.stop = threading.Event()
        self.thread = threading.Thread(target=self.pump, daemon=True)
        self.thread.start()

    def pump(self):
        while not self.stop.is_set():
            frame = self.stream.readline(FRAME + 1)
            item = frame if len(frame) <= FRAME else ValueError('oversize producer frame')
            while not self.stop.is_set():
                try:
                    self.items.put(item, timeout=0.05)
                    break
                except queue.Full:
                    pass
            if not frame or isinstance(item, Exception):
                return

    def get(self, seconds):
        item = self.items.get(timeout=seconds)
        if isinstance(item, Exception):
            raise item
        if not item:
            raise EOFError('producer closed before expected frame')
        return item

    def finish(self):
        self.stop.set()
        self.thread.join(timeout=CLEANUP_SECONDS)
        if self.thread.is_alive():
            raise RuntimeError('owned reader failed to join')
        self.stream.close()


def reap(process):
    forced = process.poll() is None
    if forced:
        process.terminate()
    try:
        code = process.wait(timeout=CLEANUP_SECONDS)
    except subprocess.TimeoutExpired:
        process.kill()
        code = process.wait(timeout=CLEANUP_SECONDS)
    return {'pid': process.pid, 'exit': code, 'terminated_by_owner': forced, 'reaped': True}


def validate(validator, path):
    p = subprocess.run([str(validator), '--max-bytes', str(FRAME), str(path)],
                       capture_output=True, timeout=3)
    if p.returncode != 0:
        raise RuntimeError(f'canonical validator rejected {path.name}, exit {p.returncode}')


def proof_case(args, manifest, mode):
    folder = args.output / mode
    folder.mkdir()
    nonce = str(uuid.uuid4())
    context = {'schema_version': '0.1.0', 'session_id': f'native-session-{nonce}',
               'target': {'id': f"f02-pid-{manifest['pid']}", 'generation': manifest['target_generation']},
               'surfaces': [{'id': f"window-{manifest['window_id']}", 'generation': manifest['surface_generation']}],
               'scope_id': f"fixture-window-{manifest['window_id']}-sample", 'projection': 'interaction',
               'fields': ['role', 'accessibility_name', 'enabled', 'accessibility_bounds'],
               'plugin': {'id': 'macos', 'version': '0.1.0'}, 'environment_revision': f'fixture-environment-{nonce}'}
    request = {'schema_version': '0.1.0', 'artifact': {'kind': 'request', 'data': {
        'clock_domain': f'native-parent-{nonce}', 'request_id': f'native-request-{nonce}', 'context': context,
        'limits': {'max_elements': 160, 'max_depth': 9, 'max_output_bytes': FRAME, 'deadline_ms': DEADLINE_MS},
        'freshness_policy': 'current_required',
        'operation': {'operation': 'observe', 'channels': ['external_semantics', 'rendered_capture']}}}}
    request_file = folder / 'request.json'
    request_file.write_bytes(encoded(request))
    validate(args.validator, request_file)
    # Descriptor-only call reads public identity/permission metadata, not requested UI fields.
    descriptor = subprocess.run([str(args.collector), str(args.manifest), str(folder), 'describe'],
                                input=encoded(request), capture_output=True, timeout=2)
    if descriptor.returncode != 0 or len(descriptor.stdout) > FRAME:
        raise RuntimeError('descriptor acquisition failed')
    session = json.loads(descriptor.stdout)
    assert session['artifact']['kind'] == 'session'
    session_file = folder / 'session.json'
    session_file.write_bytes(encoded(session))
    validate(args.validator, session_file)
    host_mode = {'live': 'complete', 'capture-timeout': 'expire-after-first',
                 'cancel': 'cancel-after-first', 'detach': 'detach-after-first'}[mode]
    processes, readers, receipts, cleanup = [], [], [], []
    submitted = {}
    try:
        host = subprocess.Popen([str(args.host), str(session_file), str(request_file), str(FRAME), str(PENDING),
                                 host_mode, '1000', '8', str(folder / 'retained')],
                                stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        processes.append(host)
        hr = Reader(host.stdout)
        readers.append(hr)
        ticket = json.loads(hr.get(2))
        receipts.append(ticket)
        assert ticket['event'] == 'ticket'
        assert ticket['request_id'] == request['artifact']['data']['request_id']
        assert ticket['session_id'] == context['session_id']
        assert ticket['parent_clock_domain'] == request['artifact']['data']['clock_domain']
        # Sequence comes only from the actual common ObservationSession::begin.
        collector = subprocess.Popen([str(args.collector), str(args.manifest), str(folder),
                                      'live' if mode == 'live' else 'injected-capture-wait', str(ticket['sequence'])],
                                     stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
        processes.append(collector)
        cr = Reader(collector.stdout)
        readers.append(cr)
        collector.stdin.write(encoded(request))
        collector.stdin.close()
        ax_bytes = cr.get(AX_SECONDS)
        ax = json.loads(ax_bytes)
        assert ax['artifact']['kind'] == 'channel_response'
        assert ax['artifact']['data']['channel'] == 'external_semantics'
        assert ax['artifact']['data']['result']['status'] == 'observed'
        ax_file = folder / 'submitted-ax.json'
        ax_file.write_bytes(ax_bytes)
        validate(args.validator, ax_file)
        submitted['external_semantics'] = ax
        host.stdin.write(ax_bytes)
        host.stdin.flush()
        accepted = json.loads(hr.get(1))
        receipts.append(accepted)
        assert accepted['event'] == 'frame_accepted'
        if mode == 'live':
            cap_bytes = cr.get(CAPTURE_SECONDS)
            cap = json.loads(cap_bytes)
            assert cap['artifact']['data']['channel'] == 'rendered_capture'
            assert cap['artifact']['data']['result']['status'] == 'observed'
            cap_file = folder / 'submitted-capture.json'
            cap_file.write_bytes(cap_bytes)
            validate(args.validator, cap_file)
            submitted['rendered_capture'] = cap
            host.stdin.write(cap_bytes)
            host.stdin.flush()
            accepted = json.loads(hr.get(1))
            receipts.append(accepted)
            assert accepted['event'] == 'frame_accepted'
        elif mode == 'capture-timeout':
            try:
                collector.wait(timeout=CAPTURE_SECONDS)
                raise RuntimeError('injected pending capture returned unexpectedly')
            except subprocess.TimeoutExpired:
                cleanup.append(reap(collector))
        terminal = json.loads(hr.get(DEADLINE_MS / 1000 + 1))
        receipts.append(terminal)
        assert terminal['event'] == 'terminal'
        expected = {'live': 'Completed', 'capture-timeout': 'TimedOut', 'cancel': 'Cancelled', 'detach': 'Detached'}[mode]
        assert terminal['terminal'] == expected
        if mode != 'live':
            assert terminal['missing_channels'] == ['rendered_capture']
            cleanup.append(reap(collector)) if not any(x['pid'] == collector.pid for x in cleanup) else None
            # Inject a valid, correctly correlated replay of this case's live AX success.
            # It is not an actual late OS callback or fabricated capture success.
            host.stdin.write(ax_bytes)
            host.stdin.flush()
            late = json.loads(hr.get(1))
            receipts.append(late)
            assert late['event'] == 'late_probe' and late['rejected'] is True
        host.stdin.close()
        assert host.wait(timeout=2) == 0
        if mode == 'live':
            assert collector.wait(timeout=1) == 0
        retained = [json.loads(p.read_text()) for p in sorted((folder / 'retained').glob('channel-*.json'))]
        assert len(retained) == len(submitted)
        for document in retained:
            channel = document['artifact']['data']['channel']
            assert document == submitted[channel], 'canonical completion changed the observed channel'
        result = {'mode': mode, 'actual_source': 'live_F02_AX_and_capture' if mode == 'live' else 'live_F02_AX_only',
                  'injected_control': mode != 'live', 'late_frame': None if mode == 'live' else 'injected_replay_of_same_live_AX_success',
                  'retained_equals_submitted': True, 'terminal': terminal, 'host_receipts': receipts}
    finally:
        for process in reversed(processes):
            if not any(x['pid'] == process.pid for x in cleanup):
                cleanup.append(reap(process))
        for reader in reversed(readers):
            reader.finish()
    result['cleanup'] = cleanup
    result['all_reader_threads_joined'] = True
    (folder / 'proof.json').write_text(json.dumps(result, indent=2)+'\n')
    return {key: result[key] for key in ('mode', 'actual_source', 'injected_control', 'retained_equals_submitted', 'all_reader_threads_joined')}


def main():
    parser = argparse.ArgumentParser()
    for name in ('host', 'validator', 'collector', 'manifest', 'output'):
        parser.add_argument(f'--{name}', required=True, type=pathlib.Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    manifest = json.loads(args.manifest.read_text())
    (args.output / 'fixture-manifest.json').write_text(json.dumps(manifest, indent=2)+'\n')
    def watchdog(_signal, _frame):
        raise TimeoutError('whole-proof process watchdog')
    signal.signal(signal.SIGALRM, watchdog)
    signal.alarm(45)
    try:
        results = [proof_case(args, manifest, mode) for mode in ('live', 'capture-timeout', 'cancel', 'detach')]
        report = {'support_revision': '73d772e97efcf550ea4a4d3e8480b56509ebc548', 'results': results,
                  'limits': {'frame_bytes_including_newline': FRAME, 'pending_encoded_bytes': PENDING,
                             'max_in_flight': 1, 'max_frames': 8, 'request_deadline_ms': DEADLINE_MS,
                             'ax_wait_seconds': AX_SECONDS, 'capture_wait_seconds': CAPTURE_SECONDS,
                             'cleanup_wait_seconds': CLEANUP_SECONDS, 'whole_proof_watchdog_seconds': 45},
                  'generic_support_checks': 'reused committed 3 Rust/8 synthetic results; not repeated',
                  'production_concurrency_or_probe_acceptance': False}
        (args.output / 'report.json').write_text(json.dumps(report, indent=2)+'\n')
        print(json.dumps(report, indent=2))
    finally:
        signal.alarm(0)


if __name__ == '__main__':
    main()
