#!/usr/bin/env python3
"""Focused native input preparation checks; no running application is observed."""
import sys
sys.dont_write_bytecode = True
import importlib.util
import json
import os
from pathlib import Path
import stat
import subprocess
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('native_prepare', ROOT/'plugins/macos/native_prepare.py')
caller = importlib.util.module_from_spec(spec)
spec.loader.exec_module(caller)

META = {'metadata_version': '1.0.0', 'status': 'known',
        'process': {'pid': 123, 'bundle_id': 'example.own.preview', 'launch_time': 1793096703.1234567},
        'target': {'id': 'macos-pid-123', 'generation': '123:1793096703.1234567'}}


class Preparation(unittest.TestCase):
    def setUp(self):
        self.stage = Path(tempfile.mkdtemp(prefix='uib-native-prepare-test-'))
        self.helper = self.stage/'native-host-helper'
        self.result = subprocess.CompletedProcess([], 0, caller.encode(META))

    def tearDown(self):
        # These tests create only named text/JSON files, never image data.
        for path in self.stage.rglob('*'):
            if path.is_file() or path.is_symlink():
                path.unlink()
        for path in sorted(self.stage.rglob('*'), key=lambda p: len(p.parts), reverse=True):
            path.rmdir()
        self.stage.rmdir()
        self.assertFalse(self.stage.exists())

    def prepare(self):
        with patch.object(caller, 'verify_bundle', return_value=self.helper):
            return caller.prepare(self.stage, 123, 'example.own.preview', self.stage)

    def test_generated_request_and_limits(self):
        with patch.object(caller.subprocess, 'run', return_value=self.result) as call:
            output = self.prepare()
            self.assertEqual(call.call_count, 2)
            self.assertTrue(all(c.kwargs['timeout'] == 3 for c in call.call_args_list))
        conn = json.loads(Path(output['connection']).read_bytes())
        req = json.loads(Path(output['request']).read_bytes())
        data = req['artifact']['data']
        self.assertEqual(conn['target'], META['target'])
        config = json.loads(conn['provider']['configuration'])
        self.assertEqual(config['process'], META['process'])
        self.assertEqual(config['collection'], 'focused-ax')
        expected = json.loads((ROOT/'tests/bridges/native/acquisition/profile.json').read_bytes())
        self.assertEqual(config['acquisition_limits'], expected)
        self.assertEqual(data['limits'], dict(max_elements=160, max_depth=9,
                                            max_output_bytes=524288, deadline_ms=1000))
        self.assertEqual(data['context']['surfaces'], conn['session']['surfaces'])
        self.assertEqual(data['operation'], dict(operation='observe', channels=['external_semantics']))
        for path in output.values():
            self.assertEqual(stat.S_IMODE(Path(path).stat().st_mode), 0o600)
        validator = Path('/Users/eugenepotapenko/.local/bin/uiblueprint-validate')
        if validator.exists():
            result = subprocess.run([str(validator), '--max-bytes', '131072', output['request']],
                                    capture_output=True, timeout=5)
            self.assertEqual(result.returncode, 0, result.stdout)

    def test_conflict_precedes_process_access(self):
        (self.stage/'connection.json').write_text('foreign')
        with patch.object(caller.subprocess, 'run') as call, self.assertRaises(caller.Refusal):
            self.prepare()
        call.assert_not_called()
        self.assertEqual((self.stage/'connection.json').read_text(), 'foreign')
        self.assertFalse((self.stage/'request.json').exists())

    def test_bundle_mismatch_restart_and_missing_publish_nothing(self):
        wrong = json.loads(caller.encode(META)); wrong['process']['bundle_id'] = 'other.app'
        restarted = json.loads(caller.encode(META)); restarted['process']['launch_time'] += 1
        restarted['target']['generation'] = '123:'+str(restarted['process']['launch_time'])
        cases = [([subprocess.CompletedProcess([], 0, caller.encode(wrong))], 'bundle_mismatch'),
                 ([self.result, subprocess.CompletedProcess([], 0, caller.encode(restarted))], 'stale_target'),
                 ([subprocess.CompletedProcess([], 4, b'')], 'target_unresolved')]
        for values, code in cases:
            with self.subTest(code=code), patch.object(caller.subprocess, 'run', side_effect=values):
                with self.assertRaises(caller.Refusal) as failure:
                    self.prepare()
                self.assertEqual(failure.exception.code, code)
                self.assertEqual(failure.exception.status, 4)
                self.assertEqual(list(self.stage.iterdir()), [])

    def test_bad_metadata_and_timeout(self):
        for raw in (b'{}', b'x'*4097, b'{"status":"known","status":"known"}', b'[]'):
            with patch.object(caller.subprocess, 'run', return_value=subprocess.CompletedProcess([], 0, raw)):
                with self.assertRaises(caller.Refusal):
                    caller.describe(self.helper, 123, 'example.own.preview')
        with patch.object(caller.subprocess, 'run', side_effect=subprocess.TimeoutExpired([], 3)):
            with self.assertRaises(caller.Refusal) as failure:
                self.prepare()
            self.assertEqual(failure.exception.code, 'metadata_timeout')
        self.assertEqual(list(self.stage.iterdir()), [])

    def test_private_directory_symlink_and_failed_write(self):
        self.stage.chmod(0o755)
        with self.assertRaises(caller.Refusal): self.prepare()
        self.stage.chmod(0o700)
        link = self.stage/'link'; link.symlink_to(self.stage, target_is_directory=True)
        with patch.object(caller, 'verify_bundle', return_value=self.helper), self.assertRaises(OSError):
            caller.prepare(self.stage, 123, 'example.own.preview', link)
        link.unlink()
        with patch.object(caller.subprocess, 'run', return_value=self.result), \
                patch.object(caller.os, 'fsync', side_effect=OSError('synthetic write failure')):
            with self.assertRaises(OSError): self.prepare()
        self.assertEqual(list(self.stage.iterdir()), [])

    def test_matching_bundle_and_tamper(self):
        names = ['native-prepare.py', 'native-host-helper', 'session-worker', 'uiblueprint']
        files = {}
        for name in names:
            data = name.encode(); path = self.stage/name; path.write_bytes(data)
            path.chmod(0o644)
            files[name] = dict(sha256=caller.hashlib.sha256(data).hexdigest(), mode=0o644)
        manifest = self.stage/'distribution-manifest.json'
        manifest.write_bytes(caller.encode(dict(format='ui-blueprint-local-4', modules='combined', files=files)))
        self.assertEqual(caller.verify_bundle(self.stage), self.helper)
        self.helper.write_bytes(b'changed')
        with self.assertRaises(caller.Refusal) as failure: caller.verify_bundle(self.stage)
        self.assertEqual(failure.exception.code, 'changed_bundle')


if __name__ == '__main__':
    unittest.main()
