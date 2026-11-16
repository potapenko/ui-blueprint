import sys
sys.dont_write_bytecode = True
import os
from pathlib import Path
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2]/'plugins/macos'))
from swiftui_recovery_control import termination_requested


class RecoveryInput(unittest.TestCase):
    def test_partial_invalid_foreign_commands_do_not_release_hold(self):
        with tempfile.TemporaryDirectory(prefix='uib-recovery-control-') as directory:
            root = Path(directory); path = root/'owner.json'
            fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
            try:
                self.assertFalse(termination_requested(fd, path.name, 123))
                for raw in (b'{', b'null', b'[]', b'x'*257,
                            b'{"pid":123,"pid":123,"action":"terminate-owned-fixture"}',
                            b'{"pid":124,"action":"terminate-owned-fixture"}',
                            b'{"pid":true,"action":"terminate-owned-fixture"}',
                            b'{"pid":123,"action":"resume"}'):
                    path.write_bytes(raw); path.chmod(0o600)
                    self.assertFalse(termination_requested(fd, path.name, 123))
                path.write_bytes(b'{"pid":123,"action":"terminate-owned-fixture"}')
                self.assertTrue(termination_requested(fd, path.name, 123))
                path.chmod(0o644)
                self.assertFalse(termination_requested(fd, path.name, 123))
                path.chmod(0o600)
                link = root/'link'; link.symlink_to(path)
                self.assertFalse(termination_requested(fd, link.name, 123))
                self.assertFalse(termination_requested(fd, '../owner.json', 123))
            finally:
                os.close(fd)


if __name__ == '__main__':
    unittest.main()
