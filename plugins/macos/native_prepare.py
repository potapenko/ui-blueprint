#!/usr/bin/env python3
"""Prepare bounded native_ax inputs for one explicitly authorized running Mac app."""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import uuid

# Same bounded profile as the developer geometry caller; not configurable expansion.
HOST = dict(workers=1, worker_bytes=67108864, publication_reserve=1048576,
    bootstrap_bytes=1048576, parent_bytes=33554432, input_bytes=2097152,
    ingress_bytes=524288, output_bytes=524288, request_output_bytes=2097152,
    completion_groups=2, control_bytes=4096, cleanup_ms=1000,
    retained_domain_bytes=67108864, retained_per_worker=15728640,
    main_stack_bytes=8388608, watchdog_stack_bytes=1048576)
ACQUISITION = dict(ax_windows=32, array_page=32, child_entries=1280,
    value_utf8_bytes=4096, action_names=32, action_name_utf8_bytes=256,
    batch_values=8, batch_utf8_bytes=16384, copied_utf8_bytes=262144,
    response_slots=65536, response_string_utf8_bytes=1048576,
    image_width=4096, image_height=4096, image_pixels=8388608,
    image_bytes=67108864, png_bytes=67108864, sidecar_bytes=16384)


class Refusal(Exception):
    def __init__(self, code, status=2):
        self.code, self.status = code, status


class ArgumentParser(argparse.ArgumentParser):
    def error(self, message):
        raise Refusal('invalid_arguments')


def strict_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise Refusal('invalid_metadata')
        result[key] = value
    return result


def decode(raw):
    return json.loads(raw, object_pairs_hook=strict_object,
                      parse_constant=lambda _: (_ for _ in ()).throw(Refusal('invalid_metadata')))


def encode(value):
    return (json.dumps(value, ensure_ascii=True, allow_nan=False, separators=(',', ':')) + '\n').encode()


def verify_bundle(bundle):
    manifest_path = bundle / 'distribution-manifest.json'
    if manifest_path.is_symlink() or manifest_path.stat().st_size > 1048576:
        raise Refusal('invalid_bundle')
    manifest = decode(manifest_path.read_bytes())
    if manifest['format'] != 'ui-blueprint-local-4' or manifest['modules'] not in ('native', 'combined'):
        raise Refusal('invalid_bundle')
    for name in ('native-prepare.py', 'native-host-helper', 'session-worker', 'uiblueprint'):
        path = bundle / name
        expected = manifest['files'][name]
        with path.open('rb') as stream:
            info = os.fstat(stream.fileno())
            if path.is_symlink() or not stat.S_ISREG(info.st_mode) or info.st_size > 268435456:
                raise Refusal('invalid_bundle')
            digest = hashlib.file_digest(stream, 'sha256').hexdigest()
        if digest != expected['sha256'] or stat.S_IMODE(info.st_mode) != expected['mode']:
            raise Refusal('changed_bundle')
    return bundle / 'native-host-helper'


def describe(helper, pid, expected_bundle):
    # The manifest-verified sibling's describe-process branch emits <4032 bytes.
    # It collects only public process metadata, not UI. Never fall back to ps/title.
    try:
        result = subprocess.run([str(helper), 'describe-process', str(pid)],
            stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
            timeout=3, check=False)
    except subprocess.TimeoutExpired:
        raise Refusal('metadata_timeout', 1) from None
    if result.returncode == 4:
        raise Refusal('target_unresolved', 4)
    if result.returncode != 0:
        raise Refusal('metadata_failed', 1)
    if len(result.stdout) > 4096:
        raise Refusal('invalid_metadata')
    data = decode(result.stdout)
    if not isinstance(data, dict) or set(data) != {'metadata_version', 'status', 'process', 'target'}:
        raise Refusal('invalid_metadata')
    process, target = data['process'], data['target']
    if (data['metadata_version'] != '1.0.0' or data['status'] != 'known'
            or not isinstance(process, dict) or set(process) != {'pid', 'bundle_id', 'launch_time'}
            or type(process['pid']) is not int or process['pid'] != pid
            or type(process['launch_time']) not in (float, int)
            or not math.isfinite(process['launch_time']) or process['launch_time'] <= 0
            or not isinstance(target, dict) or set(target) != {'id', 'generation'}
            or target['id'] != f'macos-pid-{pid}' or not isinstance(target['generation'], str)
            or not target['generation'].startswith(f'{pid}:') or len(target['generation']) > 256):
        raise Refusal('invalid_metadata')
    if process['bundle_id'] != expected_bundle:
        raise Refusal('bundle_mismatch', 4)
    if float(target['generation'].split(':', 1)[1]) != process['launch_time']:
        raise Refusal('invalid_metadata')
    return data


def inputs(metadata, helper):
    nonce = uuid.uuid4().hex
    context = dict(schema_version='0.1.0', session_id='native-'+nonce,
        target=metadata['target'], surfaces=[dict(id='ax-focused-'+nonce, generation=nonce)],
        scope_id='selected-focused-window', projection='design',
        fields=['role', 'accessibility_name', 'accessibility_bounds'],
        plugin=dict(id='macos', version='0.1.0'), environment_revision='native-'+nonce)
    session = dict(allowed_scopes=[context['scope_id']], session_id=context['session_id'],
        plugin=context['plugin'], supported_versions=['0.1.0'], target=context['target'],
        surfaces=context['surfaces'], capabilities=[dict(channel='external_semantics',
            operation='observe', status='partial', reason='explicit_public_AXFocusedWindow')])
    config = dict(collection='focused-ax', process=metadata['process'],
                  scope_id=context['scope_id'], acquisition_limits=ACQUISITION)
    configuration = encode(config).decode().strip()
    if len(configuration.encode()) > 4032:
        raise Refusal('configuration_limit')
    connection = dict(connection_version='1.0.0', target=context['target'], session=session,
        host_limits=HOST, attach_deadline_ms=2000, provider=dict(backend='native_ax',
            helper_executable=str(helper), configuration=configuration))
    request = dict(schema_version='0.1.0', artifact=dict(kind='request', data=dict(
        clock_domain='rebound_after_attach', request_id=nonce, context=context,
        limits=dict(max_elements=160, max_depth=9, max_output_bytes=524288, deadline_ms=1000),
        freshness_policy='current_required', operation=dict(operation='observe',
                                                           channels=['external_semantics']))))
    return {'connection.json': encode(connection), 'request.json': encode(request)}


def prepare(bundle, pid, expected_bundle, output):
    if not (0 < pid <= 2147483647) or not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9.-]{0,255}', expected_bundle):
        raise Refusal('invalid_target')
    if not output.is_absolute():
        raise Refusal('invalid_destination')
    helper = verify_bundle(bundle)
    fd = os.open(output, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    created = []
    try:
        info = os.fstat(fd)
        if info.st_uid != os.getuid() or stat.S_IMODE(info.st_mode) & 0o077:
            raise Refusal('private_destination_required')
        for name in ('connection.json', 'request.json'):
            try:
                os.stat(name, dir_fd=fd, follow_symlinks=False)
            except FileNotFoundError:
                continue
            raise Refusal('destination_conflict')
        metadata = describe(helper, pid, expected_bundle)
        payloads = inputs(metadata, helper)
        if metadata != describe(helper, pid, expected_bundle):
            raise Refusal('stale_target', 4)
        for name, data in payloads.items():
            out = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=fd)
            identity = os.fstat(out)
            created.append((name, identity.st_dev, identity.st_ino))
            with os.fdopen(out, 'wb') as stream:
                stream.write(data)
                stream.flush()
                os.fsync(stream.fileno())
        return {name.removesuffix('.json'): str(output / name) for name in payloads}
    except BaseException:
        for name, device, inode in reversed(created):
            try:
                info = os.stat(name, dir_fd=fd, follow_symlinks=False)
                if (info.st_dev, info.st_ino) == (device, inode):
                    os.unlink(name, dir_fd=fd)
            except FileNotFoundError:
                pass
        raise
    finally:
        os.close(fd)


def main():
    parser = ArgumentParser(description=__doc__)
    parser.add_argument('--pid', type=int, required=True)
    parser.add_argument('--bundle-id', required=True, help='expected exact app bundle ID')
    parser.add_argument('--out', type=Path, required=True, help='existing private system-temp directory')
    try:
        args = parser.parse_args()
        result = prepare(Path(__file__).resolve().parent, args.pid, args.bundle_id, args.out)
        print(json.dumps(result))
        return 0
    except Refusal as error:
        print(error.code, file=sys.stderr)
        return error.status
    except (ValueError, KeyError, TypeError, OverflowError):
        print('invalid_metadata_or_bundle', file=sys.stderr)
        return 2
    except OSError:
        print('preparation_io', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
