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
