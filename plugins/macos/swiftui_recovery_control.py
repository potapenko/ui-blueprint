"""Read a fixture owner's recovery command without ending a debugger hold.

This reader never executes recovery, grants permission, or releases a target.
Absent, partial, malformed, foreign, or unreadable input means keep holding.
"""
import json
import os
import stat


def termination_requested(directory_fd, filename, expected_pid, expected_generation, expected_owner):
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
                and value['action'] == 'terminate-owned-fixture')
    except (OSError, ValueError, UnicodeError, RecursionError):
        return False


def retain_debugger(debugger, process, lldb, directory_fd, request_id, pid, generation, owner, publish):
    """Hold an owned fixture until an authenticated decision or observed exit.

    The SBProcess stays attached to the original process; no PID-based reattach
    or signal is used. publish accepts fixed-code records, not debugger text.
    """
    import time
    command = 'owner-command-'+request_id+'.json'
    attempted = accepted = False
    debugger.SetAsync(True)
    checkpoint = dict(status='waiting_owner', pid=pid, generation=generation,
                      owner=owner, command=command, detach_attempted=False)
    published = False
    while True:
        try:
            if not published:
                try:
                    publish(checkpoint)
                    published = True
                except BaseException:
                    pass  # Receipt IO must not abandon a live interrupted call.
            state = process.GetState()
            if state == lldb.eStateExited:
                return ('owner_terminated_fixture' if accepted else
                        'exit_after_failed_owner_command' if attempted else 'external_target_exit')
            if state == lldb.eStateDetached or not process.IsValid():
                return 'debugger_ownership_lost'
            if not attempted and termination_requested(directory_fd, command, pid, generation, owner):
                # This function is reached only through the synthetic-fixture gate.
                # Mark before dispatch: even an exception must not retry termination.
                attempted = True
                accepted = process.Kill().Success()
            time.sleep(.1)
        except BaseException:
            # Partial commands, interrupts, and debugger errors never detach/resume
            # the uncertain frame or authorize a retry. Forced host/debugger death
            # remains outside this process's recovery guarantees.
            time.sleep(.1)
