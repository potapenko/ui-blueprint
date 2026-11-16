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
