#!/usr/bin/env python3
"""Qualified-profile candidate: existing SwiftUI getter to canonical layout.

No app launch or environment changes. This candidate is not installed/released.
In-flight failure retains debugger ownership. Only a separately authorized,
matching owner command can terminate the held process; Observe never writes that command and never automatically kills/restarts.
"""
import sys
sys.dont_write_bytecode = True
import os
import hashlib
import math
from pathlib import Path
import platform
import plistlib
import shlex
import signal
import stat
import subprocess
import tempfile
import time
import uuid

import native_prepare
from swiftui_debug_fields import GETTER, SWIFT_SYMBOLS
from swiftui_ag_cache import collect_fields, is_host
from swiftui_recovery_control import retain_debugger
from swiftui_debug_snapshot import Refusal, MAX_BYTES, decode, encode, request, snapshot


# Filled only from the selected consumer owner's fresh identity handoff. A caller
# cannot expand this gate with a request flag, filename or bundle prefix.
# One S13 owner-run only, after sequential fixture/recovery qualification.
CONSUMER_ADMISSION = ('s13-random-favorite-one-1', 13623,
    'me.playphraseme.reference.random.s13.w01a20844', 1795452212.070178, 1969,
    dict(type='PlayphrasemeApp.RandomCompletionDialog',
         native_view_type='PlayphrasemeApp.PlayPhrasePointingHandCursorView',
         native_match='ancestor_owner', selection='geometry_records'))
CONSUMER_BACKEND_SHA256 = '3d835dfcab1ee93d2b2eb08d610df02db8b2fa1716889cb3f0b29722d3b3cf48'


def require_admitted_target(req):
    if req['expected_bundle'].startswith('me.uiblueprint.external-layout.canary.'):
        return True
    identity = (req['request_id'], req['pid'], req['expected_bundle'],
                req['expected_launch_time'], req['window_id'], req.get('scope'))
    if identity == CONSUMER_ADMISSION:
        return False
    raise Refusal('product_capture_withdrawn')


def run_lldb(lldb_path, commands):
    # LLDB2103 blocks before its interpreter when inline startup input fills
    # the Darwin pipe. A short source command keeps the payload out of that pipe.
    with tempfile.TemporaryDirectory(prefix='uib-lldb-commands-') as directory:
        path = Path(directory)/'capture.lldb'
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(fd, 'w') as stream:
            stream.write(commands)
        args = [lldb_path, '--no-lldbinit', '--batch', '--source', str(path)]
        signals = (signal.SIGINT, signal.SIGTERM, signal.SIGHUP)
        previous = {signum: signal.getsignal(signum) for signum in signals}
        # Embedded hosts can own native handlers that Python represents as None;
        # refuse before spawn rather than replace a handler we cannot restore.
        if any(handler is None for handler in previous.values()):
            raise Refusal('unsupported_launcher_signal_owner')
        handlers = {}
        try:
            # Cancellation cannot tear down a debugger that may own an uncertain
            # native call. Install before spawn; isolate terminal group signals.
            for signum in signals:
                signal.signal(signum, lambda *_: None)
                handlers[signum] = previous[signum]
            child = subprocess.Popen(args, stdin=subprocess.DEVNULL,
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
            # No subprocess.run or Popen context manager: their exception cleanup
            # can kill/wait for the child outside the fixture owner's decision.
            while True:
                try:
                    time.sleep(.1)
                    returncode = child.wait()
                    break
                except BaseException:
                    # The delay is inside the protected body on the next iteration.
                    continue
            return subprocess.CompletedProcess(args, returncode)
        finally:
            for signum, handler in handlers.items():
                signal.signal(signum, handler)


def read_request(path):
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    with os.fdopen(fd, 'rb') as stream:
        info = os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode) or info.st_size > 4096:
            raise Refusal('invalid_request_file', 2)
        return request(stream.read(4097))


def output_directory(path, req):
    fd = os.open(path, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        info = os.fstat(fd)
        if info.st_uid != os.getuid() or stat.S_IMODE(info.st_mode) & 0o077:
            raise Refusal('private_destination_required', 2)
        for prefix in ('snapshot-', 'manifest-', 'recovery-', 'owner-command-'):
            try:
                os.stat(prefix+req['request_id']+'.json', dir_fd=fd, follow_symlinks=False)
            except FileNotFoundError:
                continue
            raise Refusal('destination_conflict', 2)
        return fd
    except BaseException:
        os.close(fd)
        raise


def write_new(fd, name, data):
    out = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=fd)
    info = os.fstat(out)
    try:
        with os.fdopen(out, 'wb') as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        return info.st_dev, info.st_ino
    except BaseException:
        current = os.stat(name, dir_fd=fd, follow_symlinks=False)
        if (current.st_dev, current.st_ino) == (info.st_dev, info.st_ino):
            os.unlink(name, dir_fd=fd)
        raise


def metadata(helper, req, deadline=None):
    timeout = 3 if deadline is None else min(3, deadline-time.monotonic())
    if timeout <= 0:
        raise Refusal('deadline')
    try:
        result = subprocess.run([str(helper), 'describe-process', str(req['pid'])],
            stdin=subprocess.DEVNULL, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
            timeout=timeout, check=False)
    except subprocess.TimeoutExpired:
        raise Refusal('deadline' if deadline is not None else 'metadata_timeout',
                      4 if deadline is not None else 1) from None
    if deadline is not None and time.monotonic() >= deadline:
        raise Refusal('deadline')
    if result.returncode != 0:
        raise Refusal('target_unresolved' if result.returncode == 4 else 'metadata_failed',
                      4 if result.returncode == 4 else 1)
    if len(result.stdout) > 4096:
        raise Refusal('invalid_metadata', 2)
    value = decode(result.stdout)
    if not isinstance(value, dict) or set(value) != {'metadata_version', 'status', 'process', 'target'}:
        raise Refusal('invalid_metadata', 2)
    process, target = value['process'], value['target']
    if (value['metadata_version'] != '1.0.0' or value['status'] != 'known'
            or not isinstance(process, dict) or set(process) != {'pid', 'bundle_id', 'launch_time'}
            or type(process['pid']) is not int or process['pid'] != req['pid']
            or type(process['launch_time']) not in (int, float)
            or not isinstance(target, dict) or set(target) != {'id', 'generation'}
            or target['id'] != 'macos-pid-'+str(req['pid'])
            or not isinstance(target['generation'], str) or len(target['generation']) > 256
            or not target['generation'].startswith(str(req['pid'])+':')):
        raise Refusal('invalid_metadata', 2)
    try:
        if float(target['generation'].split(':', 1)[1]) != process['launch_time']:
            raise Refusal('invalid_metadata', 2)
    except ValueError:
        raise Refusal('invalid_metadata', 2) from None
    if process['bundle_id'] != req['expected_bundle']:
        raise Refusal('bundle_mismatch')
    if process['launch_time'] != req['expected_launch_time']:
        raise Refusal('stale_target')
    return value


def verified_helper(bundle_path):
    # Embedded Xcode LLDB uses Python3.9, which lacks hashlib.file_digest.
    # Recheck the one executable used inside LLDB with the same manifest contract.
    bundle = Path(bundle_path)
    manifest_path = bundle/'distribution-manifest.json'
    if manifest_path.is_symlink() or manifest_path.stat().st_size > MAX_BYTES:
        raise Refusal('invalid_bundle', 2)
    manifest = decode(manifest_path.read_bytes())
    if manifest['format'] not in ('ui-blueprint-local-4', 'ui-blueprint-local-5', 'ui-blueprint-local-6') or manifest['modules'] not in ('native', 'combined'):
        raise Refusal('invalid_bundle', 2)
    helper = bundle/'native-host-helper'
    expected = manifest['files']['native-host-helper']
    fd = os.open(helper, os.O_RDONLY | os.O_NOFOLLOW)
    with os.fdopen(fd, 'rb') as stream:
        info = os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode) or info.st_size > 268435456:
            raise Refusal('invalid_bundle', 2)
        digest = hashlib.sha256()
        for chunk in iter(lambda: stream.read(65536), b''):
            digest.update(chunk)
    if digest.hexdigest() != expected['sha256'] or stat.S_IMODE(info.st_mode) != expected['mode']:
        raise Refusal('changed_bundle', 2)
    return helper


class Calls:
    """Fixed getter calls, with no expression API or executable scratch storage."""
    def __init__(self, debugger, target, process, thread, deadline, event_listener=None):
        import lldb
        self.lldb = lldb
        self.d, self.t, self.p, self.thread = debugger, target, process, thread
        self.deadline = deadline
        self.names = ([f'x{i}' for i in range(29)] + ['fp', 'lr', 'sp', 'cpsr']
                      + [f'v{i}' for i in range(32)] + ['fpsr', 'fpcr', 'pc'])
        frame = thread.GetFrameAtIndex(0)
        self.saved = {}
        for name in self.names:
            reg = frame.FindRegister(name)
            if not reg.IsValid() or reg.GetByteSize() == 0:
                raise Refusal('unsupported_register_state')
            self.saved[name] = reg.GetData()
        self.pc, self.sp = frame.GetPC(), frame.GetSP()
        entry = target.GetModuleAtIndex(0).GetObjectFileEntryPointAddress()
        self.return_pc = entry.GetLoadAddress(target)
        if self.return_pc in (lldb.LLDB_INVALID_ADDRESS, self.pc):
            raise Refusal('return_site_unavailable')
        self.return_sp = (self.sp-128) & ~15
        self.count = self.read_bytes = 0
        self.symbols, self.selectors = {}, {}
        self.inflight = self.altered = False
        self.bp = self.memory = None
        self.suspended = []
        self.pool = None
        self.code = self.read(self.pc, 16)
        self.return_code = self.read(self.return_pc, 16)
        self.redzone = self.read(self.sp-128, 128)
        self.listener = event_listener or lldb.SBListener('uib-layout-calls')
        if event_listener is None:
            self.listener.StartListeningForEvents(process.GetBroadcaster(), lldb.SBProcess.eBroadcastBitStateChanged)

    def admit(self):
        if time.monotonic() >= self.deadline:
            raise Refusal('deadline')

    def initialize(self):
        self.admit()
        self.bp = self.t.BreakpointCreateByAddress(self.return_pc)
        if not self.bp.IsValid() or not self.bp.IsHardware():
            raise Refusal('hardware_breakpoint_unavailable')
        self.bp.SetThreadID(self.thread.GetThreadID())
        error = self.lldb.SBError()
        self.memory = self.p.AllocateMemory(4096,
            self.lldb.ePermissionsReadable | self.lldb.ePermissionsWritable, error)
        if error.Fail():
            self.memory = None
            raise Refusal('scratch_allocation_failed')
        for th in self.p:
            if th.GetThreadID() != self.thread.GetThreadID() and not th.IsSuspended():
                if not th.Suspend():
                    raise Refusal('thread_suspend_failed')
                self.suspended.append(th)
        self.symbol('objc_autoreleasePoolPop')
        self.pool = self.call('objc_autoreleasePoolPush')[0]

    def read(self, pointer, size):
        self.admit()
        if size < 0 or self.read_bytes+size > 2*MAX_BYTES:
            raise Refusal('read_byte_limit', 2)
        self.read_bytes += size
        error = self.lldb.SBError()
        value = self.p.ReadMemory(pointer, size, error)
        if error.Fail() or len(value) != size:
            raise Refusal('memory_read_failed')
        return value

    def symbol(self, name):
        if name in self.symbols:
            return self.symbols[name]
        self.admit()
        owners = {'objc_msgSend': 'libobjcMsgSend.dylib', 'NSApp': 'AppKit',
                  'malloc_size': 'libsystem_malloc.dylib'}
        owner = ('SwiftUI' if name == GETTER else 'libswiftCore.dylib'
                 if name in SWIFT_SYMBOLS else owners.get(name, 'libobjc.A.dylib'))
        addresses = set()
        for context in self.t.FindSymbols(name):
            if context.GetModule().GetFileSpec().GetFilename() == owner:
                address = context.GetSymbol().GetStartAddress().GetLoadAddress(self.t)
                if address != self.lldb.LLDB_INVALID_ADDRESS:
                    addresses.add(address)
        if len(addresses) != 1:
            raise Refusal('unsupported_symbol_profile')
        self.symbols[name] = addresses.pop()
        return self.symbols[name]

    def setreg(self, name, value):
        error = self.lldb.SBError()
        if not self.thread.GetFrameAtIndex(0).FindRegister(name).SetValueFromCString(hex(value), error):
            raise Refusal('register_write_failed')

    def restore(self):
        for name in self.names:
            error = self.lldb.SBError()
            if not self.thread.GetFrameAtIndex(0).FindRegister(name).SetData(self.saved[name], error):
                raise Refusal('register_restore_failed')
        # Cleanup checks must run even after the acquisition deadline.
        for pointer, expected in ((self.pc, self.code), (self.return_pc, self.return_code),
                                  (self.sp-128, self.redzone)):
            error = self.lldb.SBError()
            if self.p.ReadMemory(pointer, len(expected), error) != expected or error.Fail():
                raise Refusal('return_state_changed')
        self.altered = False

    def call(self, name, *args, fp_inputs=(), fp_count=0):
        if name not in ('objc_autoreleasePoolPush', 'objc_autoreleasePoolPop',
                        'objc_msgSend', 'sel_registerName', 'class_getInstanceSize',
                        'class_getInstanceVariable', 'ivar_getOffset', 'malloc_size'):
            raise Refusal('forbidden_target_call')
        if (fp_count not in (0, 2, 4) or len(fp_inputs) not in (0, 2, 4)
                or ((fp_inputs or fp_count) and name != 'objc_msgSend')
                or any(type(v) not in (int, float) or not math.isfinite(v) for v in fp_inputs)):
            raise Refusal('unsupported_geometry_abi')
        self.admit()
        # Preserve the existing acquisition ceiling; cache reads own no Swift array.
        limit = 256 if name == 'objc_autoreleasePoolPop' else 254
        if self.count >= limit or len(args) > 8:
            raise Refusal('call_limit', 2)
        address = self.symbol(name)
        self.admit()
        self.count += 1
        self.altered = True
        for i, value in enumerate(args):
            self.setreg('x'+str(i), value)
        for i, value in enumerate(fp_inputs):
            error = self.lldb.SBError()
            data = self.lldb.SBData.CreateDataFromDoubleArray(
                self.t.GetByteOrder(), self.t.GetAddressByteSize(), [float(value)])
            reg = self.thread.GetFrameAtIndex(0).FindRegister('d'+str(i))
            if not reg.IsValid() or not reg.SetData(data, error) or error.Fail():
                raise Refusal('geometry_register_write_failed')
        self.setreg('sp', self.return_sp)
        self.setreg('lr', self.return_pc)
        self.setreg('pc', address)
        while self.listener.GetNextEvent(self.lldb.SBEvent()):
            pass
        self.admit()
        self.d.SetAsync(True)
        self.inflight = True
        if self.p.Continue().Fail():
            raise Refusal('call_dispatch_unknown')
        while True:
            remaining = self.deadline-time.monotonic()
            if remaining <= 0:
                break
            event = self.lldb.SBEvent()
            # This LLDB API only accepts whole seconds. Do not round a final
            # fractional budget up; GetNextEvent is its nonblocking counterpart.
            received = (self.listener.WaitForEvent(1, event) if remaining >= 1
                        else self.listener.GetNextEvent(event))
            if received and self.lldb.SBProcess.EventIsProcessEvent(event):
                if self.lldb.SBProcess.GetStateFromEvent(event) in (
                        self.lldb.eStateStopped, self.lldb.eStateExited, self.lldb.eStateCrashed):
                    break
            if not received and remaining < 1:
                time.sleep(min(.005, max(0, self.deadline-time.monotonic())))
        if self.p.GetState() != self.lldb.eStateStopped:
            self.p.Stop()
        self.d.SetAsync(False)
        frame = self.thread.GetFrameAtIndex(0)
        if (self.p.GetState() != self.lldb.eStateStopped or frame.GetPC() != self.return_pc
                or frame.GetSP() != self.return_sp):
            raise Refusal('call_interrupted_owner_recovery')
        if self.thread.GetStopReason() != self.lldb.eStopReasonBreakpoint:
            raise Refusal('unexpected_stop_owner_recovery')
        if self.thread.GetStopReasonDataAtIndex(0) != self.bp.GetID():
            raise Refusal('unexpected_breakpoint_owner_recovery')
        self.inflight = False
        if fp_count:
            values = []
            for i in range(fp_count):
                error = self.lldb.SBError()
                value = frame.FindRegister('d'+str(i)).GetData().GetDouble(error, 0)
                if error.Fail() or not math.isfinite(value):
                    raise Refusal('invalid_native_geometry')
                values.append(value)
            value = tuple(values)
        else:
            value = tuple(frame.FindRegister('x'+str(i)).GetValueAsUnsigned() for i in range(4))
        self.restore()
        return value

    def selector(self, name):
        # All names are fixed call-site literals, never source/UI input.
        if name not in self.selectors:
            self.admit()
            value = name.encode('ascii')+b'\0'
            if len(value) > 128:
                raise Refusal('selector_limit', 2)
            error = self.lldb.SBError()
            if self.p.WriteMemory(self.memory, value, error) != len(value) or error.Fail():
                raise Refusal('scratch_write_failed')
            self.selectors[name] = self.call('sel_registerName', self.memory)[0]
        return self.selectors[name]

    def objc(self, obj, name, *args):
        if not obj:
            raise Refusal('object_unavailable')
        return self.call('objc_msgSend', obj, self.selector(name), *args)[0]

    def rect(self, obj, selector):
        if not obj or selector not in ('frame', 'bounds'):
            raise Refusal('unsupported_geometry_selector')
        value = self.call('objc_msgSend', obj, self.selector(selector), fp_count=4)
        if min(value[2:]) < 0:
            raise Refusal('invalid_native_geometry')
        return value

    def point_to_window(self, view, point):
        if not view:
            raise Refusal('object_unavailable')
        return self.call('objc_msgSend', view, self.selector('convertPoint:toView:'), 0,
                         fp_inputs=point, fp_count=2)

    def close(self):
        """True only when resuming the interrupted app is supported by this run."""
        if self.inflight:
            return False
        try:
            if self.altered:
                self.restore()
            # All completed-call cleanup shares one bounded two-second interval.
            self.deadline = time.monotonic()+2
            if self.pool:
                self.call('objc_autoreleasePoolPop', self.pool)
                self.pool = None
            if self.memory is not None and self.p.DeallocateMemory(self.memory).Fail():
                return False
            if self.bp and not self.t.BreakpointDelete(self.bp.GetID()):
                return False
            for thread in self.suspended:
                if not thread.Resume():
                    return False
            return True
        except BaseException:
            return False


def native_mapping(calls, host, window, req):
    if calls.objc(host, 'window') != window:
        raise Refusal('stale_window')
    bounds = calls.rect(host, 'bounds')
    frame = calls.rect(window, 'frame')
    flipped = bool(calls.objc(host, 'isFlipped'))
    if bounds[:2] != (0.0, 0.0) or not flipped:
        return None
    basis = [calls.point_to_window(host, point) for point in ((0, 0), (1, 0), (0, 1))]
    repeated_basis = [calls.point_to_window(host, point) for point in ((0, 0), (1, 0), (0, 1))]
    if (repeated_basis != basis or bool(calls.objc(host, 'isFlipped')) != flipped
            or calls.objc(host, 'window') != window or calls.rect(host, 'bounds') != bounds
            or calls.rect(window, 'frame') != frame
            or calls.objc(window, 'windowNumber') != req['window_id']):
        raise Refusal('stale_window')
    return dict(window_id=req['window_id'], host_bounds=list(bounds),
                window_frame=list(frame), host_is_flipped=flipped,
                basis_window_points=basis, source='NSView.convertPoint.to-window-base')


def collect(calls, req):
    c = calls
    app = int.from_bytes(c.read(c.symbol('NSApp'), 8), 'little')
    windows = c.objc(app, 'windows')
    count = c.objc(windows, 'count')
    if count > 32:
        raise Refusal('window_limit', 2)
    selected = []
    for index in range(count):
        window = c.objc(windows, 'objectAtIndex:', index)
        if c.objc(window, 'windowNumber') == req['window_id']:
            selected.append(window)
    if len(selected) != 1:
        raise Refusal('window_unresolved')
    window = selected[0]
    c.selected_window = window
    root = c.objc(window, 'contentView')
    pending, hosts, seen = [root], [], set()
    while pending:
        view = pending.pop()
        if not view or view in seen or len(seen) >= 256:
            raise Refusal('view_tree_limit', 2)
        seen.add(view)
        if is_host(c, view, req):
            hosts.append(view)
            # This getter owns its SwiftUI graph. Its native descendants may be
            # numerous controls or nested hosts, not additional outer roots.
            continue
        children = c.objc(view, 'subviews')
        length = c.objc(children, 'count')
        if len(seen)+len(pending)+length > 256:
            raise Refusal('view_tree_limit', 2)
        pending.extend(c.objc(children, 'objectAtIndex:', i) for i in range(length))
    # Multiple hosted surfaces must never be merged into one invented origin.
    if len(hosts) != 1:
        raise Refusal('hosting_surface_unresolved')
    raw = collect_fields(c, hosts[0], req)
    c.native_layout_mapping = native_mapping(c, hosts[0], window, req)
    if c.objc(window, 'windowNumber') != req['window_id'] or c.objc(window, 'contentView') != root:
        raise Refusal('stale_window')
    flags = dict(is_key_window=bool(c.objc(window, 'isKeyWindow')),
                 is_main_window=bool(c.objc(window, 'isMainWindow')),
                 app_is_active=bool(c.objc(app, 'isActive')))
    return raw, dict(source='NSWindow-and-NSApplication-getters',
                     consistency='unknown', interval='selected-stopped-process', values=flags)


def capture(debugger, request_path, bundle_path, output_path, backend_profile=None):
    """LLDB entry; its caller discards all LLDB streams, including backtraces."""
    import lldb
    req = read_request(request_path)
    fd = output_directory(output_path, req)
    process = calls = backend = None
    owner = uuid.uuid4().hex
    before = None
    safe = True
    fixture = False
    payload = None
    outcome = dict(manifest_version='1.0.0', request_id=req['request_id'], status='failed', code='capture_failed', exit_status=1,
                   cleanup='not_attached', snapshot=None, calls=0, native_window_state=None,
                   native_layout_mapping=None, cache_audit=None)
    try:
        fixture = require_admitted_target(req)
        if backend_profile is not None:
            from swiftui_debug_backend import FixedBackend, profile
            selected_backend = profile(backend_profile)
            if not fixture and selected_backend['sha256'] != CONSUMER_BACKEND_SHA256:
                raise Refusal('backend_not_qualified')
            backend = FixedBackend(selected_backend)
        elif not fixture:
            raise Refusal('qualified_backend_required')
        helper = verified_helper(bundle_path)
        before = metadata(helper, req)
        debugger.SetAsync(False)
        debugger.HandleCommand('settings set target.require-hardware-breakpoint true')
        debugger.HandleCommand('settings set target.load-script-from-symbol-file false')
        debugger.HandleCommand('settings set target.process.disable-memory-cache true')
        target = debugger.CreateTarget('')
        error = lldb.SBError()
        if backend is None:
            process = target.AttachToProcessWithID(debugger.GetListener(), req['pid'], error)
        else:
            debugger.HandleCommand('settings set plugin.process.gdb-remote.packet-timeout 3')
            process = backend.connect(debugger, target, lldb)
            backend.attach(process, req['pid'], error, lldb)
        start = time.monotonic()
        deadline = start+req['limits']['deadline_ms']/1000
        if error.Fail() or not process.IsValid():
            raise Refusal('attach_unavailable')
        if process.GetProcessID() != req['pid'] or metadata(helper, req, deadline) != before:
            raise Refusal('stale_target')
        thread = process.GetThreadAtIndex(0)
        frame = thread.GetFrameAtIndex(0)
        if thread.GetQueueName() != 'com.apple.main-thread' or frame.GetFunctionName() != 'mach_msg2_trap':
            raise Refusal('main_thread_not_quiescent')
        calls = Calls(debugger, target, process, thread, deadline,
                      event_listener=backend.listener if backend is not None else None)
        calls.initialize()
        raw, window_state = collect(calls, req)
        calls.admit()
        # Only already allowlisted fields reach the canonical encoder.
        provenance = {}
        payload = snapshot(raw, req, before, start, time.monotonic(), provenance,
                           getattr(calls, 'native_layout_mapping', None))
        outcome['native_layout_mapping'] = getattr(calls, 'native_layout_mapping', None)
        del raw
        if metadata(helper, req, deadline) != before:
            raise Refusal('stale_target')
        calls.admit()
        outcome.update(status='ready', code='partial_layout', exit_status=0,
                       snapshot='snapshot-'+req['request_id']+'.json', native_window_state=window_state)
    except (Refusal, native_prepare.Refusal) as error:
        payload = None
        outcome['code'] = error.code
        outcome['exit_status'] = error.status
    except BaseException:
        payload = None
        outcome['code'] = 'capture_failed'
    finally:
        if backend is not None and (not backend.attach_attempted or backend.attach_refused):
            if not backend.finish(True):
                outcome.update(code='backend_cleanup_pending', exit_status=1)
            if backend.attach_refused:
                outcome['cleanup'] = 'peer_closed_after_refused_attach'
                outcome['backend_attach'] = backend.attach_status
            process = None
        if backend is not None and backend.attach_pending:
            safe = False
        if calls is not None:
            safe = calls.close() and safe
            outcome['calls'] = calls.count
            outcome['cache_audit'] = getattr(calls, 'cache_audit', None)
        if process is not None and process.IsValid() and process.GetState() != lldb.eStateDetached:
            if safe:
                if process.GetState() != lldb.eStateStopped:
                    process.Stop()
                safe = process.Detach(False).Success()
                outcome['cleanup'] = 'detached_restored' if safe else 'owner_recovery_required'
            if not safe:
                payload = None
                outcome.update(status='failed', code='owner_recovery_required', exit_status=4,
                               snapshot=None, native_window_state=None)
                # Never call unsupported Detach(True). Keep the owning debugger
                # alive until the exact fixture owner decides or an exit is observed.
                generation = before['target']['generation'] if before else str(req['pid'])+':unverified'
                def publish(checkpoint):
                    checkpoint.update(calls=outcome['calls'], process_id=process.GetProcessID(),
                                      process_state=process.GetState())
                    write_new(fd, 'recovery-'+req['request_id']+'.json', encode(checkpoint))
                def bound_target():
                    try:
                        return process.GetProcessID() == req['pid'] and metadata(helper, req) == before
                    except BaseException:
                        return False
                outcome['cleanup'] = retain_debugger(debugger, process, lldb, fd,
                    req['request_id'], req['pid'], generation, owner, publish,
                    termination_action='terminate-owned-fixture' if fixture else 'terminate-admitted-source',
                    can_terminate=bound_target, event_listener=backend.listener if backend is not None else None)
        if not safe:
            if process is None or not process.IsValid():
                outcome['cleanup'] = 'debugger_ownership_lost'
            payload = None
            outcome.update(status='failed', code='owner_recovery_required', exit_status=4,
                           snapshot=None, native_window_state=None)
    if backend is not None:
        detached = outcome['cleanup'] in ('peer_closed_after_refused_attach', 'detached_restored', 'owner_terminated_fixture', 'owner_terminated_source', 'external_target_exit', 'exit_after_failed_owner_command')
        if not backend.finish(detached):
            payload = None
            outcome.update(status='failed', code='backend_cleanup_pending', exit_status=1, snapshot=None)
    if payload is None:
        outcome['native_layout_mapping'] = None
    created = None
    try:
        if payload is not None:
            created = write_new(fd, outcome['snapshot'], payload)
            if provenance:
                outcome['selection_provenance'] = provenance
        write_new(fd, 'manifest-'+req['request_id']+'.json', encode(outcome))
    except BaseException:
        if created is not None:
            info = os.stat(outcome['snapshot'], dir_fd=fd, follow_symlinks=False)
            if (info.st_dev, info.st_ino) == created:
                os.unlink(outcome['snapshot'], dir_fd=fd)
        raise
    finally:
        os.close(fd)


def main():
    parser = native_prepare.ArgumentParser(description=__doc__)
    parser.add_argument('--request', type=Path, required=True)
    parser.add_argument('--bundle', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--backend-profile', type=Path)
    try:
        args = parser.parse_args()
        req = read_request(args.request)
        require_admitted_target(req)
        native_prepare.verify_bundle(args.bundle)
        fd = output_directory(args.out, req)
        os.close(fd)
        with open('/System/Library/CoreServices/SystemVersion.plist', 'rb') as stream:
            version = plistlib.load(stream)['ProductVersion']
        if platform.machine() != 'arm64' or version != '27.0.1':
            raise Refusal('unsupported_platform')
        lldb_path = subprocess.check_output(['xcrun', '--find', 'lldb'], timeout=3).decode().strip()
        version = subprocess.check_output([lldb_path, '--version'], timeout=3)
        if b'lldb-2103.0.34.407\n' not in version:
            raise Refusal('unsupported_debugger')
        script = 'swiftui_debug_capture.capture(lldb.debugger, '+', '.join(repr(str(p.resolve()))
            for p in (args.request, args.bundle, args.out))
        if args.backend_profile is not None:
            script += ', '+repr(str(args.backend_profile.absolute()))
        script += ')'
        run_lldb(lldb_path, 'command script import '+shlex.quote(str(Path(__file__).resolve()))
                 +'\nscript '+script+'\n')
        result = decode((args.out/('manifest-'+req['request_id']+'.json')).read_bytes())
        print(encode(result).decode(), end='')
        return result['exit_status']
    except (Refusal, native_prepare.Refusal) as error:
        print(error.code, file=sys.stderr)
        return error.status
    except SystemExit:
        raise
    except BaseException:
        print('capture_io_or_setup_failed', file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())
