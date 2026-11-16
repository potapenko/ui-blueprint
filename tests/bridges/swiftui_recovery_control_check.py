import sys
sys.dont_write_bytecode = True
import os
import json
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


if __name__ == '__main__':
    unittest.main()
