import sys
sys.dont_write_bytecode = True
import os
import json
from pathlib import Path
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'plugins/macos'))
from swiftui_recovery_control import termination_requested, retain_debugger
from types import SimpleNamespace
from unittest.mock import patch


class RecoveryInput(unittest.TestCase):
    def test_partial_invalid_foreign_commands_do_not_release_hold(self):
        with tempfile.TemporaryDirectory(prefix='uib-recovery-control-') as directory:
            root = Path(directory); path = root/'owner.json'
            fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
            def allowed(name=path.name):
                return termination_requested(fd, name, 123, '123:1794836979.5', 'held-owner-1')
            try:
                self.assertFalse(allowed())
                for raw in (b'{', b'null', b'[]', b'x'*257,
                            b'{"pid":123,"pid":123,"action":"terminate-owned-fixture"}',
                            b'{"pid":124,"action":"terminate-owned-fixture"}',
                            b'{"pid":true,"action":"terminate-owned-fixture"}',
                            b'{"pid":123,"action":"resume"}'):
                    path.write_bytes(raw); path.chmod(0o600)
                    self.assertFalse(allowed())
                valid = dict(pid=123, generation='123:1794836979.5', owner='held-owner-1',
                             action='terminate-owned-fixture')
                for field, replacement in [('pid', 124), ('generation', '123:1794836980.5'),
                                           ('owner', 'another-owner'), ('action', 'resume')]:
                    path.write_text(json.dumps({**valid, field: replacement}))
                    self.assertFalse(allowed())
                path.write_text(json.dumps(valid))
                self.assertTrue(allowed())
                path.chmod(0o644)
                self.assertFalse(allowed())
                path.chmod(0o600)
                link = root/'link'; link.symlink_to(path)
                self.assertFalse(allowed(link.name))
                self.assertFalse(allowed('../owner.json'))
            finally:
                os.close(fd)

    def test_external_exit_and_failed_command_are_not_owner_success(self):
        lldb = SimpleNamespace(eStateExited=10, eStateDetached=9)
        debugger = SimpleNamespace(SetAsync=lambda _: None)
        for command, success, expected in [(False, False, 'external_target_exit'),
                (True, False, 'exit_after_failed_owner_command'),
                (True, True, 'owner_terminated_fixture')]:
            states = iter([5, 10])
            kills = []
            process = SimpleNamespace(GetState=lambda: next(states), IsValid=lambda: True,
                Kill=lambda: (kills.append(1) or SimpleNamespace(Success=lambda: success)))
            with patch('swiftui_recovery_control.termination_requested', return_value=command), \
                    patch('time.sleep', return_value=None):
                value = retain_debugger(debugger, process, lldb, 0, 'r', 123, 'g', 'o', lambda _: None)
            self.assertEqual(value, expected)
            self.assertEqual(len(kills), int(command))

    def test_failed_command_is_never_retried_and_receipt_failure_keeps_hold(self):
        lldb = SimpleNamespace(eStateExited=10, eStateDetached=9)
        debugger = SimpleNamespace(SetAsync=lambda _: None)
        states = iter([5, 5, 5, 10]); kills = []
        process = SimpleNamespace(GetState=lambda: next(states), IsValid=lambda: True,
            Kill=lambda: (kills.append(1) or SimpleNamespace(Success=lambda: False)))
        def unavailable(_): raise OSError()
        with patch('swiftui_recovery_control.termination_requested', return_value=True), \
                patch('time.sleep', return_value=None):
            result = retain_debugger(debugger, process, lldb, 0, 'r', 123, 'g', 'o', unavailable)
        self.assertEqual(result, 'exit_after_failed_owner_command')
        self.assertEqual(kills, [1])

    def test_repeated_interrupts_in_delay_and_backend_keep_hold(self):
        lldb = SimpleNamespace(eStateExited=10, eStateDetached=9)
        from unittest.mock import Mock
        debugger = Mock()
        debugger.SetAsync.side_effect = [KeyboardInterrupt(), None]
        process = Mock()
        process.GetState.side_effect = [KeyboardInterrupt(), 5, 10]
        process.IsValid.return_value = True
        delays = iter([KeyboardInterrupt(), KeyboardInterrupt(), None, OSError()])
        def delay(_):
            effect = next(delays, None)
            if effect is not None: raise effect
        with patch('swiftui_recovery_control.termination_requested', return_value=False), \
                patch('time.sleep', side_effect=delay):
            result = retain_debugger(debugger, process, lldb, 0, 'r', 123, 'g', 'o', lambda _: None)
        self.assertEqual(result, 'external_target_exit')
        process.Kill.assert_not_called(); process.Detach.assert_not_called()
        self.assertEqual(debugger.SetAsync.call_count, 2)


if __name__ == '__main__':
    unittest.main()
