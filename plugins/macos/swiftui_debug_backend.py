"""Caller-owned fixed peer; no target is named before executable verification."""
import ctypes
import hashlib
import os
from pathlib import Path
import re
import socket
import stat
import subprocess
import time

from swiftui_debug_snapshot import Refusal, decode


def profile(path):
    fd = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    with os.fdopen(fd, 'rb') as stream:
        info = os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode) or info.st_size > 4096 or info.st_mode & 0o022:
            raise Refusal('invalid_backend_profile', 2)
        value = decode(stream.read(4097))
    if (not isinstance(value, dict) or set(value) != {'profile_version', 'executable', 'sha256', 'policy'}
            or value['profile_version'] != '1.0.0' or value['policy'] != 'rw-data-only-v1'
            or not isinstance(value['sha256'], str) or not re.fullmatch('[a-f0-9]{64}', value['sha256'])
            or not isinstance(value['executable'], str) or not Path(value['executable']).is_absolute()):
        raise Refusal('invalid_backend_profile', 2)
    return value


class FixedBackend:
    def __init__(self, value):
        self.value = value
        self.path = Path(value['executable'])
        self.peer = None
        self.listener = None
        self.finish_result = None
        self.attach_attempted = self.attach_pending = self.attach_refused = False
        self.attach_status = None

    def verify_file(self):
        fd = os.open(self.path, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(fd, 'rb') as stream:
            info = os.fstat(stream.fileno())
            if (not stat.S_ISREG(info.st_mode) or info.st_size > 67108864
                    or info.st_mode & 0o022 or not info.st_mode & 0o100):
                raise Refusal('backend_artifact_changed')
            digest = hashlib.sha256()
            for chunk in iter(lambda: stream.read(65536), b''):
                digest.update(chunk)
            if digest.hexdigest() != self.value['sha256']:
                raise Refusal('backend_artifact_changed')
        return info

    def verify_peer(self):
        info = self.verify_file()
        if self.peer is None or self.peer.poll() is not None:
            raise Refusal('backend_peer_exited')
        library = ctypes.CDLL('/usr/lib/libproc.dylib')
        buf = ctypes.create_string_buffer(4096)
        if (library.proc_pidpath(self.peer.pid, buf, len(buf)) <= 0
                or Path(os.fsdecode(buf.value)).resolve() != self.path.resolve()):
            raise Refusal('backend_peer_identity')
        result = subprocess.run(['/usr/sbin/lsof', '-a', '-p', str(self.peer.pid), '-d', 'txt', '-F', 'pDint'],
                                capture_output=True, timeout=2, check=False)
        if result.returncode or len(result.stdout) > 65536:
            raise Refusal('backend_mapped_identity')
        records, record = [], {}
        for row in result.stdout.decode('utf-8', errors='strict').splitlines():
            if row.startswith('p'):
                continue
            if row.startswith('f'):
                records.append(record)
                record = {}
            else:
                record[row[:1]] = row[1:]
        records.append(record)
        if not any(r.get('i') == str(info.st_ino) and int(r.get('D', '0'), 0) == info.st_dev
                   and Path(r.get('n', '/')).resolve() == self.path.resolve() for r in records):
            raise Refusal('backend_mapped_identity')

    def connect(self, debugger, target, lldb):
        self.verify_file()
        left, right = socket.socketpair()
        try:
            self.peer = subprocess.Popen([str(self.path), '--fd='+str(right.fileno())],
                pass_fds=(right.fileno(),), stdin=subprocess.DEVNULL,
                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
            right.close()
            error = lldb.SBError()
            self.listener = lldb.SBListener('uib-fixed-backend-process')
            process = target.ConnectRemote(self.listener, 'fd://'+str(left.detach()), 'gdb-remote', error)
            if error.Fail() or not process.IsValid():
                raise Refusal('backend_connection_failed')
            deadline = time.monotonic()+3
            while process.GetState() != lldb.eStateConnected and time.monotonic() < deadline:
                event = lldb.SBEvent()
                if self.listener.GetNextEvent(event) and lldb.SBProcess.EventIsProcessEvent(event):
                    lldb.SBProcess.GetStateFromEvent(event)
                time.sleep(.01)
            if process.GetState() != lldb.eStateConnected:
                raise Refusal('backend_connection_failed')
            self.verify_peer()
            return process
        finally:
            left.close()
            right.close()

    def attach(self, process, pid, error, lldb):
        self.verify_peer()
        listener = self.listener
        if listener is None:
            raise Refusal('backend_listener_unavailable')
        self.attach_attempted = self.attach_pending = True
        accepted = process.RemoteAttachToProcessWithID(pid, error)
        if not accepted or error.Fail():
            pid_now = process.GetProcessID()
            state = process.GetState()
            unbound = pid_now in (0, lldb.LLDB_INVALID_PROCESS_ID)
            message = (error.GetCString() or '').lower()
            self.attach_status = dict(permission_rejection=any(word in message for word in ('not allowed', 'not permitted', 'permission')),
                                      accepted=bool(accepted), error=error.GetError(),
                                      state=state, target_bound=not unbound)
            # A terminal rejection with an idle connection is not a held target.
            # Accepted-but-unfinished attachment follows the separate hold path.
            if not accepted and error.Fail() and state == lldb.eStateConnected and unbound:
                self.attach_refused = True
                self.attach_pending = False
            return
        deadline = time.monotonic()+4
        while time.monotonic() < deadline:
            event = lldb.SBEvent()
            if listener.GetNextEvent(event) and lldb.SBProcess.EventIsProcessEvent(event):
                lldb.SBProcess.GetStateFromEvent(event)
            if process.GetState() == lldb.eStateStopped and process.GetProcessID() == pid:
                self.attach_pending = False
                self.verify_peer()
                return
            time.sleep(.02)
        raise Refusal('backend_attach_unconfirmed')

    def finish(self, detached):
        if self.finish_result is not None:
            return self.finish_result
        if self.peer is None or self.peer.poll() is not None:
            return True
        if self.attach_attempted and not detached:
            return False
        try:
            self.peer.wait(timeout=2)
        except subprocess.TimeoutExpired:
            self.finish_result = False
            self.peer.terminate()
            try:
                self.peer.wait(timeout=2)
            except subprocess.TimeoutExpired:
                return False
        self.finish_result = True
        return True
