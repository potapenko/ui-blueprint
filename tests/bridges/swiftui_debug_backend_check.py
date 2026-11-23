"""Fixed-backend admission and ownership checks, without a target process."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch
sys.dont_write_bytecode=True
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'plugins/macos'))
from swiftui_debug_backend import FixedBackend, profile
from swiftui_debug_snapshot import Refusal


class BackendChecks(unittest.TestCase):
    def test_profile_rejects_mutable_unknown_and_symlink_inputs(self):
        with tempfile.TemporaryDirectory(prefix='uib-backend-profile-') as directory:
            p=Path(directory)/'profile.json'
            valid=dict(profile_version='1.0.0',executable='/owned/debugserver',sha256='a'*64,policy='rw-data-only-v1')
            p.write_text(json.dumps(valid));self.assertEqual(profile(p),valid)
            for bad in (dict(valid,extra=1),dict(valid,executable='relative'),dict(valid,sha256='x'*64),dict(valid,policy='anything')):
                p.write_text(json.dumps(bad))
                with self.assertRaises(Refusal):profile(p)
            p.write_text(json.dumps(valid));p.chmod(0o666)
            with self.assertRaises(Refusal):profile(p)
            p.chmod(0o600);link=Path(directory)/'link';link.symlink_to(p)
            with self.assertRaises(OSError):profile(link)

    def test_changed_executable_refuses_before_peer_spawn(self):
        with tempfile.TemporaryDirectory(prefix='uib-backend-file-') as directory:
            path=Path(directory)/'peer';path.write_bytes(b'qualified');path.chmod(0o700)
            backend=FixedBackend(dict(executable=str(path),sha256=hashlib.sha256(b'qualified').hexdigest()))
            backend.verify_file();path.write_bytes(b'changed')
            with patch('swiftui_debug_backend.subprocess.Popen') as spawn:
                with self.assertRaisesRegex(Refusal,'backend_artifact_changed'):
                    backend.connect(None,None,None)
                spawn.assert_not_called()

    def test_peer_revalidation_precedes_target_pid(self):
        b=FixedBackend(dict(executable='/unused',sha256='a'*64))
        b.verify_peer=Mock(side_effect=Refusal('backend_peer_identity'))
        process=SimpleNamespace(RemoteAttachToProcessWithID=Mock())
        with self.assertRaises(Refusal):b.attach(process,123,None,None)
        process.RemoteAttachToProcessWithID.assert_not_called()
        self.assertFalse(b.attach_attempted)

    def test_attach_waits_for_matching_stopped_process(self):
        b=FixedBackend(dict(executable='/unused',sha256='a'*64));b.verify_peer=Mock()
        process=SimpleNamespace(RemoteAttachToProcessWithID=Mock(return_value=True),
            GetState=Mock(side_effect=[2,5]),GetProcessID=lambda:123)
        with patch('swiftui_debug_backend.time.sleep'),patch('swiftui_debug_backend.time.monotonic',side_effect=[0,.1,.2]):
            b.attach(process,123,SimpleNamespace(Fail=lambda:False),SimpleNamespace(eStateStopped=5))
        self.assertFalse(b.attach_pending)
        self.assertEqual(b.verify_peer.call_count,2)
        self.assertTrue(b.attach_attempted)

    def test_unfinished_attach_keeps_ownership_and_never_detaches_or_kills(self):
        b=FixedBackend(dict(executable='/unused',sha256='a'*64));b.verify_peer=Mock()
        process=SimpleNamespace(RemoteAttachToProcessWithID=lambda *args:True,GetState=lambda:2,Detach=Mock(),Kill=Mock())
        with patch('swiftui_debug_backend.time.monotonic',side_effect=[0,5]):
            with self.assertRaisesRegex(Refusal,'backend_attach_unconfirmed'):
                b.attach(process,123,SimpleNamespace(Fail=lambda:False),SimpleNamespace(eStateStopped=5))
        self.assertTrue(b.attach_pending)
        process.Detach.assert_not_called();process.Kill.assert_not_called()
        b.peer=Mock();b.peer.poll.return_value=None
        self.assertFalse(b.finish(False));b.peer.terminate.assert_not_called();b.peer.wait.assert_not_called()

    def test_peer_cleanup_requires_release_and_is_not_retried(self):
        b=FixedBackend(dict(executable='/unused',sha256='a'*64));b.attach_attempted=True
        b.peer=Mock();b.peer.poll.return_value=None
        b.peer.wait.side_effect=subprocess.TimeoutExpired('owned-peer',2)
        self.assertFalse(b.finish(True));self.assertFalse(b.finish(True))
        b.peer.terminate.assert_called_once();self.assertEqual(b.peer.wait.call_count,2)


if __name__=='__main__':unittest.main()
