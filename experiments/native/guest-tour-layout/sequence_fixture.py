"""LLDB qualification of four fixed sites, exclusively on our console fixture.

Load from an owned interactive LLDB with S14_SEQUENCE_RUNTIME set to the prepared
internal runtime. This file has no product-process mode or arbitrary selectors.
"""
import ctypes
import hashlib
import json
import os
import re
import socket
import subprocess
import tempfile
import time
from pathlib import Path

import lldb

D = Path(os.environ["S14_SEQUENCE_RUNTIME"]).resolve()
SOURCE = Path(__file__).with_name("fixture.swift").resolve()
DEBUGGER = Path("/Applications/Xcode.app/Contents/Developer/usr/bin/lldb")
PEER = D / "debugserver"
CONTROL = D / "Control"
PINS = {
    DEBUGGER: "75572933d93dd4dfec94e0c023a84de7d2724afa6a2bd9918facdee3ecbfbeba",
    PEER: "830f585db3fcf5c5e15f580065137c04924e95551d961a8a3cac63113c2cebdd",
}
SITES = {10: ("map", "origin viewport scale"), 13: ("origin", "origin"),
         16: ("group", "card attachment.id"), 19: ("card", "card target rect start")}
peer = child = process = target = None
owned = {}
contacted = detached = False
records = []
MODE = os.environ.get("S14_SEQUENCE_MODE", "map")


def require(value, reason):
    if not value:
        raise RuntimeError(reason)


def command(value):
    result = lldb.SBCommandReturnObject()
    lldb.debugger.GetCommandInterpreter().HandleCommand(value, result)
    require(result.Succeeded(), "fixed_command_refused")
    return result.GetOutput() or ""


def image(pid):
    buffer = ctypes.create_string_buffer(4096)
    require(ctypes.CDLL("/usr/lib/libproc.dylib").proc_pidpath(pid, buffer, len(buffer)) > 0, "image_unavailable")
    return Path(os.fsdecode(buffer.value)).resolve()


def attest(pid, executable):
    require(image(pid) == executable, "image_path_changed")
    require(hashlib.sha256(executable.read_bytes()).hexdigest() == PINS[executable], "image_hash_changed")
    rows = subprocess.check_output(["/usr/sbin/lsof", "-a", "-p", str(pid), "-d", "txt", "-F", "pDint"], text=True, timeout=2).splitlines()
    files, current = [], {}
    for row in rows:
        if row.startswith("p"):
            continue
        if row.startswith("f"):
            files.append(current)
            current = {}
        else:
            current[row[:1]] = row[1:]
    files.append(current)
    stat = executable.stat()
    require(any(x.get("i") == str(stat.st_ino) and int(x.get("D", "0"), 0) == stat.st_dev
                and Path(x.get("n", "/")).resolve() == executable for x in files), "mapped_image_changed")


def live_owner():
    # During attach macOS reparents the inferior to debugserver. Popen.poll()
    # can observe ECHILD and cache a false exit status while the child is alive.
    require(peer.poll() is None, "peer_exited")
    attest(peer.pid, PEER)
    require(image(child.pid) == CONTROL, "fixture_image_changed")


def wait_stopped(deadline, prior):
    while time.monotonic() < deadline:
        if process.GetState() == lldb.eStateStopped and process.GetStopID() > prior:
            return True
        time.sleep(0.005)
    return False


def owner_recover_stopped():
    global detached
    require(contacted and not detached and os.isatty(0), "recovery_not_admitted")
    require(process.GetState() == lldb.eStateStopped and process.GetProcessID() == child.pid, "owned_stop_required")
    live_owner()
    for bid in owned:
        bp = target.FindBreakpointByID(bid)
        require(bp.IsValid() and bp.IsHardware(), "breakpoint_ownership_unconfirmed")
        require(target.BreakpointDelete(bid), "breakpoint_delete_failed")
    detached = True  # A failed detach is never retried automatically or manually.
    require(process.Detach().Success(), "detach_failed")
    deadline = time.monotonic() + 2
    while process.GetState() != lldb.eStateDetached and time.monotonic() < deadline:
        time.sleep(0.01)
    require(process.GetState() == lldb.eStateDetached, "detach_unconfirmed")
    (case / "finish").touch()
    require(child.wait(timeout=4) == 0 and "recovered" in (case / "stdout").read_text(), "fixture_recovery_unconfirmed")
    print("DETACHED_AND_FIXTURE_RECOVERED", flush=True)


def install_site(line, pins):
    kind, fields = SITES[line]
    command(f"breakpoint set --hardware --shlib Control --file fixture.swift --line {line} --move-to-nearest-code false")
    bp = target.GetBreakpointAtIndex(target.GetNumBreakpoints() - 1)
    # Remember ownership before validation so an installation refusal remains recoverable.
    owned[bp.GetID()] = (kind, fields, None)
    require(bp.IsValid() and bp.IsHardware() and bp.GetNumLocations() == 1, "hardware_site_refused")
    location = bp.GetLocationAtIndex(0)
    address = location.GetAddress()
    entry = address.GetLineEntry()
    require(location.IsResolved() and Path(entry.GetFileSpec().fullpath).resolve() == SOURCE and entry.GetLine() == line,
            "source_site_refused")
    require(address.GetModule().GetUUIDString().upper() == pins["uuid"], "module_changed")
    detail = command("breakpoint list --verbose " + str(bp.GetID())).split(str(bp.GetID()) + ".1:", 1)
    require(len(detail) == 2 and re.search(r"(?m)^\s*hardware = true\s*$", detail[1]), "installed_hardware_unconfirmed")
    owned[bp.GetID()] = (kind, fields, address.GetLoadAddress(target))


def run():
    global peer, child, process, target, contacted, case
    require(MODE in ("map", "groups"), "fixture_mode_refused")
    require(os.isatty(0) and os.isatty(1), "owned_interactive_pty_required")
    require(D.parent == Path("/private/tmp") and D.name.startswith("uib-s14-map-"), "runtime_refused")
    pins = json.loads((D / "fixture-pins.json").read_text())
    require(set(pins) == {"control", "source", "uuid"}, "fixture_pin_shape")
    require(hashlib.sha256(CONTROL.read_bytes()).hexdigest() == pins["control"] and hashlib.sha256(SOURCE.read_bytes()).hexdigest() == pins["source"], "fixture_pin_changed")
    attest(os.getpid(), DEBUGGER)
    for setting in ("plugin.process.gdb-remote.packet-timeout 3", "stop-disassembly-display never", "frame-format stopped-frame",
                    "stop-line-count-before 0", "stop-line-count-after 0", "target.require-hardware-breakpoint true",
                    "target.load-script-from-symbol-file false", "target.process.stop-on-sharedlibrary-events false"):
        command("settings set " + setting)
    dbg = lldb.debugger
    dbg.SetAsync(False)
    target = dbg.CreateTarget(str(CONTROL))
    require(target.IsValid(), "target_creation_failed")
    left, right = socket.socketpair()
    peer = subprocess.Popen([PEER, "--fd=" + str(right.fileno())], pass_fds=(right.fileno(),),
                            stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    right.close()
    attest(peer.pid, PEER)  # Before a fixture PID even exists.
    error = lldb.SBError()
    process = target.ConnectRemote(dbg.GetListener(), "fd://" + str(left.detach()), "gdb-remote", error)
    require(error.Success() and process.GetState() == lldb.eStateConnected, "idle_connection_refused")
    case = Path(tempfile.mkdtemp(prefix="sequence-", dir=D / "tmp"))
    child = subprocess.Popen([CONTROL], cwd=case, stdout=(case / "stdout").open("wb"), stderr=subprocess.DEVNULL)
    live_owner()
    contacted = True
    require(process.RemoteAttachToProcessWithID(child.pid, error) and error.Success(), "fixture_attach_refused")
    require(wait_stopped(time.monotonic() + 4, -1) and process.GetProcessID() == child.pid, "fixture_attach_unconfirmed")
    require(target.GetNumBreakpoints() == 0 and target.GetNumWatchpoints() == 0, "unexpected_debug_points")
    dbg.SetAsync(True)
    deadline = time.monotonic() + 10
    for phase, card_count in (("baseline", 3), ("expanded", 2)):
        for bid in list(owned):
            require(target.BreakpointDelete(bid), "phase_breakpoint_delete_failed")
            del owned[bid]
        install_site(10 if MODE == "map" else 16, pins)
        (case / "trigger").write_text(phase)
        phase_start = len(records)
        prior = process.GetStopID()
        require(process.Continue().Success(), "continue_refused")
        expected = ["map", "origin"] + ["card"] * card_count if MODE == "map" else ["group"] * card_count
        for count in range(len(expected)):
            require(wait_stopped(deadline, prior), "sequence_deadline_retained")
            thread = next((t for t in process if t.GetStopReason() == lldb.eStopReasonBreakpoint
                           and t.GetStopReasonDataCount() >= 2 and t.GetStopReasonDataAtIndex(0) in owned), None)
            require(thread is not None, "unexpected_stop_retained")
            bid = thread.GetStopReasonDataAtIndex(0)
            kind, fields, pc = owned[bid]
            frame = thread.GetFrameAtIndex(0)
            require(frame.GetPC() == pc, "different_site")
            process.SetSelectedThread(thread)
            thread.SetSelectedFrame(0)
            start = time.monotonic_ns()
            raw = command("frame variable -d no-dynamic-values -R -S false -- " + fields)
            end = time.monotonic_ns()
            require(len(raw) < 8192, "local_text_limit")
            records.append({"kind": kind, "start_ns": start, "end_ns": end, "raw": raw, "phase": phase})
            if kind == "group":
                value = frame.FindVariable("attachment", lldb.eNoDynamicValues).GetNonSyntheticValue().GetChildMemberWithName("id", lldb.eNoDynamicValues).GetNonSyntheticValue()
                require(value.IsValid() and value.GetError().Success() and 0 < value.GetByteSize() <= 8, "fixture_enum_unavailable")
                error = lldb.SBError()
                data = value.GetData().ReadRawData(error, 0, value.GetByteSize())
                require(error.Success(), "fixture_enum_read_failed")
                records[-1]["fixture_enum_bytes"] = list(data)
            if kind == "map":
                require(target.BreakpointDelete(bid), "map_breakpoint_delete_failed")
                del owned[bid]
                install_site(13, pins)
            elif kind == "origin":
                require(target.BreakpointDelete(bid), "origin_breakpoint_delete_failed")
                del owned[bid]
                install_site(19, pins)
            if count < len(expected) - 1:
                prior = process.GetStopID()
                require(process.Continue().Success(), "continue_refused")
        require([r["kind"] for r in records[phase_start:]] == expected, "sequence_differs")

    owner_recover_stopped()
    print("FIXTURE_SEQUENCE " + json.dumps(records), flush=True)


try:
    run()
except BaseException as error:
    print("SEQUENCE_REFUSED", str(error) if isinstance(error, RuntimeError) else type(error).__name__, flush=True)
    if contacted:
        print("OWNER_CONTROL_RETAINED; no retry or debugger teardown", flush=True)
    elif peer is not None and peer.poll() is None:
        peer.terminate()
        peer.wait(timeout=2)
