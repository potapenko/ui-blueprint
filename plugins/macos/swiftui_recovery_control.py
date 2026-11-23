"""Read a fixture owner's recovery command without ending a debugger hold.

This reader never executes recovery, grants permission, or releases a target.
Absent, partial, malformed, foreign, or unreadable input means keep holding.
"""
import json
import os
import stat


def termination_requested(directory_fd, filename, expected_pid, expected_generation, expected_owner,
                          expected_action='terminate-owned-fixture'):
    if expected_action not in ('terminate-owned-fixture', 'terminate-admitted-source'):
        return False
    # Expected values belong to the retained checkpoint, never to this command.
    if (type(expected_pid) is not int or expected_pid <= 0
            or not isinstance(expected_generation, str) or not expected_generation
            or not isinstance(expected_owner, str) or not expected_owner):
        return False
    if '/' in filename or filename in ('', '.', '..'):
        return False
    try:
        fd = os.open(filename, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK,
                     dir_fd=directory_fd)
        with os.fdopen(fd, 'rb') as stream:
            info = os.fstat(stream.fileno())
            if (not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid()
                    or stat.S_IMODE(info.st_mode) != 0o600 or info.st_size > 256):
                return False
            raw = stream.read(257)
        if len(raw) > 256:
            return False
        def unique(pairs):
            result = {}
            for key, value in pairs:
                if key in result:
                    raise ValueError()
                result[key] = value
            return result
        value = json.loads(raw, object_pairs_hook=unique)
        return (isinstance(value, dict) and set(value) == {'pid', 'generation', 'owner', 'action'}
                and type(value['pid']) is int and value['pid'] == expected_pid
                and value['generation'] == expected_generation
                and value['owner'] == expected_owner
                and value['action'] == expected_action)
    except (OSError, ValueError, UnicodeError, RecursionError):
        return False


def retain_debugger(debugger, process, lldb, directory_fd, request_id, pid, generation, owner, publish,
                    termination_action=None, can_terminate=None):
    """Retain ownership; an explicit bound owner command is separate from Observe.

    The SBProcess stays attached to the original process; no PID-based reattach
    or signal is used. publish accepts fixed-code records, not debugger text.
    """
    import time
    command = 'owner-command-'+request_id+'.json'
    attempted = accepted = False
    async_ready = False
    checkpoint = dict(status='waiting_owner', pid=pid, generation=generation,
                      owner=owner, command=command if termination_action else None,
                      termination_action=termination_action,
                      detach_attempted=False)
    published = False
    try:
        bound_at_entry = process.GetProcessID() == pid and (can_terminate is None or can_terminate())
    except BaseException:
        bound_at_entry = False
    while True:
        try:
            time.sleep(.1)
            if not async_ready:
                debugger.SetAsync(True)
                async_ready = True
            if not published:
                try:
                    publish(checkpoint)
                    published = True
                except BaseException:
                    pass  # Receipt IO must not abandon a live interrupted call.
            state = process.GetState()
            if state == lldb.eStateExited:
                if not bound_at_entry:
                    return 'unbound_backend_exit'
                if accepted:
                    return ('owner_terminated_source' if termination_action == 'terminate-admitted-source'
                            else 'owner_terminated_fixture')
                return 'exit_after_failed_owner_command' if attempted else 'external_target_exit'
            if state == lldb.eStateDetached or not process.IsValid():
                return 'debugger_ownership_lost'
            if (termination_action and not attempted and state == lldb.eStateStopped
                    and process.GetProcessID() == pid and termination_requested(
                    directory_fd, command, pid, generation, owner, termination_action)):
                # Observe never writes this command. For a product the owner must
                # first obtain separate human authorization for this exact target.
                # Mark before dispatch: even an exception must not retry termination.
                attempted = True
                if can_terminate is not None and not can_terminate():
                    continue
                bound_at_entry = True
                accepted = process.Kill().Success()
        except BaseException:
            # Partial commands, interrupts, and debugger errors never detach/resume
            # the uncertain frame or authorize a retry. Forced host/debugger death
            # remains outside this process's recovery guarantees.
            continue
