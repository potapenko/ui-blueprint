"""Synthetic IO checks of the common host; no browser/native collection."""
import argparse
import copy
import json
import queue
import subprocess
import tempfile
import threading
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]


def fixture(name):
    return json.loads((ROOT / 'fixtures/golden' / name).read_text())


def check(host, directory, mode='complete', fault=None):
    session = fixture('ENV-CAPABILITY-VALID.json')
    request = fixture('ENV-REQUEST-VALID.json')
    request['artifact']['data']['operation']['channels'].append('rendered_capture')
    request['artifact']['data']['limits']['deadline_ms'] = 200 if mode == 'expire-after-first' else 3000
    if fault == 'request-version':
        request['schema_version'] = '0.2.0'
    session_path, request_path = directory / 'session.json', directory / 'request.json'
    session_path.write_text(json.dumps(session))
    request_path.write_text(json.dumps(request))
    retained = directory / 'retained'
    proc = subprocess.Popen([str(host), str(session_path), str(request_path), '65536', '131072', mode, '1000', '8', str(retained)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    events = queue.Queue(maxsize=32)

    def pump():
        while line := proc.stdout.readline(16_384):
            events.put(line, timeout=5)

    thread = threading.Thread(target=pump, daemon=True)
    thread.start()

    def event():
        return json.loads(events.get(timeout=5))

    def send(document):
        proc.stdin.write(json.dumps(document, separators=(',', ':')).encode() + b'\n')
        proc.stdin.flush()

    try:
        first = event()
        if fault == 'request-version':
            assert first['event'] == 'host_error'
            assert proc.wait(timeout=5) == 1
            return
        assert first['event'] == 'ticket'
        snapshot = fixture('ENV-SNAPSHOT-VALID.json')['artifact']['data']
        # These are authored records, never represented as real UI observations.
        snapshot['observations'][0]['clock_domain'] = 'authored-test-clock'
        base = dict(request_id=first['request_id'], session_id=first['session_id'], dispatch_sequence=first['sequence'], target=snapshot['context']['target'])
        ax = dict(schema_version='0.1.0', artifact=dict(kind='channel_response', data=dict(**base, channel='external_semantics', result=dict(status='observed', data=snapshot))))
        capture = dict(schema_version='0.1.0', artifact=dict(kind='channel_response', data=dict(**base, channel='rendered_capture', result=dict(status='failed', data=dict(code='permission_required', scope_id='form-1', failed_step='capture', recovery_class='explicit_permission')))))
        if fault in ('malformed', 'response-version'):
            if fault == 'malformed':
                proc.stdin.write(b'{"SYNTHETIC_PRIVATE_TOKEN":true}\n')
                proc.stdin.flush()
            else:
                bad = copy.deepcopy(ax)
                bad['schema_version'] = '0.2.0'
                send(bad)
            rejected = event()
            assert rejected == dict(event='frame_rejected', code='InvalidFrame')
        send(ax)
        assert event()['event'] == 'frame_accepted'
        if fault == 'oversize':
            proc.stdin.write(b'x' * 65537 + b'\n')
            proc.stdin.flush()
            assert event() == dict(event='framing_rejected', code='Oversize')
            terminal = event()
            assert terminal['terminal'] == 'Cancelled' and len(terminal['channels']) == 1
            assert proc.wait(timeout=5) == 2
        elif mode == 'complete':
            send(capture)
            assert event()['event'] == 'frame_accepted'
            terminal = event()
            assert terminal['terminal'] == 'Completed' and len(terminal['channels']) == 2
            assert proc.wait(timeout=5) == 0
        else:
            terminal = event()
            assert terminal['terminal'] == {'cancel-after-first': 'Cancelled', 'detach-after-first': 'Detached', 'expire-after-first': 'TimedOut'}[mode]
            assert terminal['injected_control'] and len(terminal['channels']) == 1
            if mode == 'expire-after-first':
                assert terminal['parent_elapsed_ms'] >= 200
            send(capture)
            late = event()
            assert late['event'] == 'late_probe' and late['rejected'] and late['injected']
            assert proc.wait(timeout=5) == 0
        stored = json.loads((retained / 'channel-0.json').read_text())
        assert stored == ax, 'Core must retain the actual admitted AX data'
        assert proc.stderr.read() == b''
    finally:
        if proc.poll() is None:
            proc.kill()
            proc.wait(timeout=5)
        for stream in (proc.stdin, proc.stdout, proc.stderr):
            stream.close()
        thread.join(timeout=5)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--host', type=Path, required=True)
    host = parser.parse_args().host.resolve()
    cases = [('complete', None), ('cancel-after-first', None), ('detach-after-first', None), ('expire-after-first', None), ('complete', 'request-version'), ('complete', 'malformed'), ('complete', 'response-version'), ('complete', 'oversize')]
    with tempfile.TemporaryDirectory(prefix='uib-s01-common-') as temp:
        for index, (mode, fault) in enumerate(cases):
            directory = Path(temp) / str(index)
            directory.mkdir()
            check(host, directory, mode, fault)
    print('8 synthetic common-host scenarios passed; no live collector used')


if __name__ == '__main__':
    main()
