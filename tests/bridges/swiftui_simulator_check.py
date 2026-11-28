#!/usr/bin/env python3
"""Simulator pair platform and exact-device binding regressions; no live device."""
import importlib.util
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import Mock, patch

spec = importlib.util.spec_from_file_location("simulator", Path(__file__).parents[2] / "plugins/ios/swiftui_simulator.py")
simulator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(simulator)

class PairChecks(unittest.TestCase):
    def run_pair(self, platform, *, swapped=False):
        with tempfile.TemporaryDirectory() as folder:
            container = Path(folder)
            endpoint = container / "app/tmp/session"
            window = dict(window=1, generation="generation", platform=platform, scene_state=0)
            caller = Mock()
            caller.endpoint.return_value = (endpoint, dict(bundle="fixture", pid=123))
            caller.request.side_effect = [(dict(windows=[window]), None), RuntimeError("capture-reached")]
            args = SimpleNamespace(out=str(container / "out"), session="descriptor", simulator="owned-tv",
                                   timeout=30, window=1, source="swiftui")
            with patch.object(simulator, "simctl", return_value=str(container / ("foreign" if swapped else "app"))) as command:
                try:
                    simulator.pair(args, caller)
                except (ValueError, RuntimeError) as error:
                    return str(error), caller.request.call_count, command.call_args.args
            self.fail("test must terminate before capture")

    def test_label_style_rejects_default_source_before_endpoint(self):
        caller = Mock()
        with self.assertRaisesRegex(ValueError, "invalid_label_style_selection"):
            simulator.pair(SimpleNamespace(label_style=True, source="swiftui"), caller)
        caller.endpoint.assert_not_called()

    def test_label_style_is_forwarded_to_uikit_capture(self):
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            caller = Mock()
            caller.endpoint.return_value = (root / "app/tmp/session", dict(bundle="fixture", pid=1))
            caller.request.side_effect = [(dict(windows=[dict(window=1,generation="g",platform="tvos-simulator",scene_state=0)]),None),RuntimeError("capture-reached")]
            args = SimpleNamespace(out=str(root/"out"),session="descriptor",simulator="owned-tv",timeout=30,window=1,source="uikit",label_style=True)
            with patch.object(simulator,"simctl",return_value=str(root/"app")):
                with self.assertRaisesRegex(RuntimeError,"capture-reached"):
                    simulator.pair(args,caller)
            self.assertEqual(caller.request.call_args.kwargs, dict(source="uikit",label_style=True,font_resolution=False))

    def test_font_resolution_rejects_default_source_before_endpoint(self):
        caller = Mock()
        with self.assertRaisesRegex(ValueError,"invalid_font_resolution_selection"):
            simulator.pair(SimpleNamespace(label_style=False,font_resolution=True,source="swiftui"),caller)
        caller.endpoint.assert_not_called()

    def test_explicit_supported_platforms_reach_capture(self):
        for platform in ("ios-simulator", "tvos-simulator"):
            with self.subTest(platform=platform):
                error, calls, _ = self.run_pair(platform)
                self.assertEqual((error, calls), ("capture-reached", 2))

    def test_other_platforms_refuse_before_capture(self):
        for platform in (None, "ios", "tvos", "macos", "visionos-simulator"):
            with self.subTest(platform=platform):
                error, calls, _ = self.run_pair(platform)
                self.assertEqual((error, calls), ("simulator_metadata_required", 1))

    def test_swapped_device_container_refuses_before_request(self):
        error, calls, command = self.run_pair("tvos-simulator", swapped=True)
        self.assertEqual((error, calls), ("simulator_session_mismatch", 0))
        self.assertEqual(command, ("get_app_container", "owned-tv", "fixture", "data"))

if __name__ == "__main__":
    unittest.main()
