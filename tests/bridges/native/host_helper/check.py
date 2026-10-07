#!/usr/bin/env python3
"""Bounded offline FD tests; no AX, capture, permissions or UI acquisition."""
import argparse
import copy
import fcntl
import json
import os
from pathlib import Path
import socket
import struct
import subprocess
import time

ROOT = Path(__file__).resolve().parents[4]


def encoded(value):
    return json.dumps(value, ensure_ascii=False, separators=(',', ':')).encode()


def control(kind, channel, length, ticket=7, auxiliary=0, epoch=11, operation=19):
    # Existing c0abcff Rust Control::encode layout, not a new test protocol.
    return b'UIBHST01' + bytes([kind, 8, channel, 0]) + bytes(4) + struct.pack(
        '<QQQQQ', epoch, operation, length, ticket, auxiliary) + bytes(8)


def inputs(channel=0, cap=65_536, duration=1000, request=None, config=None):
    if request is None:
        request = json.loads((ROOT / 'fixtures/golden/ENV-REQUEST-VALID.json').read_text())
        r = request['artifact']['data']
        r['clock_domain'] = 'uib-worker-11-clock'
        r['context']['target'] = {'id': 'f02-pid-123', 'generation': 'g1'}
        r['context']['surfaces'] = [{'id': 'window-456', 'generation': 'w1'}]
        r['context']['fields'] = ['role', 'accessibility_name', 'enabled', 'accessibility_bounds']
        r['operation']['channels'] = ['external_semantics', 'rendered_capture']
        r['limits']['deadline_ms'] = 1000
    if config is None:
        config = {'binding': {'pid': 123, 'bundle_id': 'local.uiblueprint.f02.off',
                             'launch_time': 100.0, 'window_id': 456, 'window_identifier': 'a',
                             'target_generation': 'g1', 'surface_generation': 'w1'},
                  'scope_id': 'form-1', 'collection': 'sample',
                  'artifact_directory': '/tmp/unused-offline-native-fixture',
                  'pixel_policy': 'owned_synthetic_fixture',
                  'acquisition_limits': json.loads((ROOT / 'tests/bridges/native/acquisition/profile.json').read_text()),
                  'acquisition_evidence': False}
    # Pretty input proves length framing accepts whitespace/newlines unchanged.
    body = json.dumps(request, indent=2).encode()
    setup = encoded(config)
    prefix = control(1, channel, len(setup), auxiliary=cap) + setup
    return prefix + control(3, channel, len(body), auxiliary=duration) + body, request, config


def run(binary, wire, chunk=None, delayed_body=False):
    sockets = [socket.socketpair() for _ in range(3)]  # input/output/diagnostics
    parents = [pair[0] for pair in sockets]
    sources = [fcntl.fcntl(pair[1].fileno(), fcntl.F_DUPFD_CLOEXEC, 10) for pair in sockets]
    null = os.open('/dev/null', os.O_RDWR)
    actions = [(os.POSIX_SPAWN_DUP2, null, 0), (os.POSIX_SPAWN_DUP2, null, 1),
               (os.POSIX_SPAWN_DUP2, sources[2], 2),
               (os.POSIX_SPAWN_DUP2, sources[0], 3), (os.POSIX_SPAWN_DUP2, sources[1], 4)]
    actions += [(os.POSIX_SPAWN_CLOSE, fd) for fd in sources]
    pid = os.posix_spawn(str(binary), [str(binary)], {}, file_actions=actions)
    for fd in sources:
        os.close(fd)
    os.close(null)
    for _, child in sockets:
        child.close()
    for parent in parents:
        parent.settimeout(3)
    reaped = False
    try:
        try:
            if delayed_body:
                # Send complete headers/config, hold actual Request beyond 1ms.
                setup_length = struct.unpack_from('<Q', wire, 32)[0]
                boundary = 128 + setup_length
                parents[0].sendall(wire[:boundary])
                time.sleep(0.05)
                parents[0].sendall(wire[boundary:])
            else:
                step = chunk or max(1, len(wire))
                for offset in range(0, len(wire), step):
                    parents[0].sendall(wire[offset:offset + step])
            parents[0].shutdown(socket.SHUT_WR)
        except (BrokenPipeError, ConnectionResetError):
            pass  # expected for early admission refusals
        output = bytearray()
        while True:
            part = parents[1].recv(4096)
            if not part:
                break
            output.extend(part)
            assert len(output) <= 512 * 1024, 'unbounded output'
        diagnostic = parents[2].recv(4096)
        deadline = time.monotonic() + 3
        while time.monotonic() < deadline:
            found, status = os.waitpid(pid, os.WNOHANG)
            if found:
                reaped = True
                return os.waitstatus_to_exitcode(status), bytes(output), diagnostic
            time.sleep(0.005)
        raise AssertionError('owned peer did not exit')
    finally:
        if not reaped:
            # Only the exact unreaped test-created child, never a user application.
            os.kill(pid, 9)
            os.waitpid(pid, 0)
        for parent in parents:
            parent.close()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--peer', type=Path, required=True)
    parser.add_argument('--helper', type=Path, required=True)
    parser.add_argument('--validator', type=Path, required=True)
    args = parser.parse_args()
    passed = []

    def case(name, wire, success=False, binary=None, **kwargs):
        code, output, diagnostic = run(binary or args.peer, wire, **kwargs)
        assert diagnostic == b'', (name, 'unexpected diagnostics')
        if success:
            assert code == 0 and output.endswith(b'\n') and output.count(b'\n') == 1, (name, code)
            result = subprocess.run([str(args.validator), '--max-bytes', '524288', '-'],
                                    input=output, capture_output=True, timeout=3)
            assert result.returncode == 0, (name, result.stdout)
            response = json.loads(output)['artifact']['data']
            assert response['dispatch_sequence'] == 7  # deliberately != host operation19
            assert response['result']['data']['code'] == 'permission_required'
        else:
            assert code == 2 and not output, (name, code, len(output))
        passed.append(name)
        return output

    wire, request, config = inputs()
    result = case('valid AX / real encoder / Ticket differs from operation', wire, True)
    assert json.loads(result)['artifact']['data']['channel'] == 'external_semantics'
    cap = len(result)
    case('cap exactly includes LF', inputs(cap=cap)[0], True)
    case('one-byte-less cap refuses before output', inputs(cap=cap - 1)[0])
    case('partial one-byte transfers', wire, True, chunk=1)
    capture = case('capture selection independently routed', inputs(channel=1)[0], True)
    assert json.loads(capture)['artifact']['data']['channel'] == 'rendered_capture'
    quoted = copy.deepcopy(request)
    quoted['artifact']['data']['request_id'] = 'request-escaped-\n-data'
    case('escaped newline remains data', inputs(request=quoted)[0], True)
    for name, cut in [('empty', 0), ('truncated Configure', 63),
                      ('truncated configuration', 70), ('truncated body', len(wire) - 1)]:
        case(name, wire[:cut])
    for name, offset, value in [('magic', 0, 0), ('class', 9, 9), ('channel', 10, 2),
                                ('flags', 11, 1), ('reserved', 12, 1)]:
        bad = bytearray(wire); bad[offset] = value
        case('invalid ' + name, bad)
    setup_length = struct.unpack_from('<Q', wire, 32)[0]
    submit_offset = 64 + setup_length
    for name, offset in [('channel', 10), ('epoch', 16), ('operation', 24), ('Ticket', 40)]:
        bad = bytearray(wire); bad[submit_offset + offset] ^= 1
        case('mismatched ' + name, bad)
    case('zero duration', inputs(duration=0)[0])
    case('elapsed duration while reading body', inputs(duration=1)[0], delayed_body=True)
    case('oversize config admitted before body', control(1, 0, 4033, auxiliary=65_536))
    case('oversize request admitted before body', wire[:submit_offset] + control(3, 0, 2_097_153, auxiliary=1000))
    bad_config = copy.deepcopy(config); bad_config['scope_id'] = 'other-window'
    denied_wire = inputs(config=bad_config)[0]
    case('trusted scope mismatch', denied_wire)
    # These actual entrypoint cases fail in receive(), before Collector/SDK calls.
    case('actual helper rejects scope before SDK', denied_wire, binary=args.helper)
    case('actual helper rejects header before SDK', wire[:63], binary=args.helper)
    bad_config = copy.deepcopy(config); del bad_config['artifact_directory']; del bad_config['pixel_policy']
    case('actual helper rejects missing pixel policy before SDK', inputs(channel=1, config=bad_config)[0], binary=args.helper)
    bad_config = copy.deepcopy(config); del bad_config['acquisition_limits']
    case('missing mandatory acquisition profile', inputs(config=bad_config)[0], binary=args.helper)
    bad_config = copy.deepcopy(config); bad_config['acquisition_limits']['png_bytes'] = 67_108_865
    case('out-of-profile acquisition cap', inputs(config=bad_config)[0], binary=args.helper)
    bad_config = copy.deepcopy(config); bad_config['binding']['bundle_id'] = 'other.app'
    case('unsupported binding', inputs(config=bad_config)[0])
    bad_config = copy.deepcopy(config); bad_config['artifact_directory'] = '/tmp/../outside'
    case('artifact traversal refused', inputs(config=bad_config)[0])
    bad_request = copy.deepcopy(request); bad_request['artifact']['data']['context']['surfaces'][0]['generation'] = 'recreated'
    case('stale surface generation', inputs(request=bad_request)[0])
    bad_request = copy.deepcopy(request); bad_request['artifact']['data']['limits']['max_elements'] = 161
    case('collector acquisition ceiling', inputs(request=bad_request)[0])
    print(json.dumps({'passed': len(passed), 'cases': passed,
                      'scope': 'offline FD protocol; canonical failure records, no SDK calls; all owned peers reaped'}, indent=2))


if __name__ == '__main__':
    main()
