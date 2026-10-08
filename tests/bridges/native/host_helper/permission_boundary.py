#!/usr/bin/env python3
"""Compile the exact production preflight/issue switch against inert SDK stubs.
No TCC operation, application access or input. The fixture supplies permission and
identity independently; expected outcomes come from NATIVE/EXCHANGE, not output.
"""
import pathlib
import subprocess
import tempfile

root = pathlib.Path(__file__).resolve().parents[4]
source = (root/'plugins/macos/NativeFormSession.swift').read_text()
prefix = source.split('    func current(', 1)[1].split('        do {', 1)[0]
mapper = source.split('            let code: String, recovery: String', 1)[1].split('            try frame.encode', 1)[0]
assert '"code": nextPhase == 2 ? "action_outcome_unknown" : code' in source
failure = next(line for line in source.splitlines() if line.startswith('enum NativeFormFailure:'))
program = '''import Foundation
'''+failure+'''
enum NativeAcquisitionError: Error { case expired, limit }
struct Binding { let pid = 1; let bundle_id = "fixture"; let launch_time = 10.0; let window_id = 2 }
struct Config { let binding = Binding(); let parent_binding: Binding? = nil }
struct NativeAcquisition { func check() throws {} }
var trusted = true
var identityMatches = true
func AXIsProcessTrusted() -> Bool { trusted }
struct NSRunningApplication {
    init?(processIdentifier: Int) {}
    var isTerminated: Bool { !identityMatches }
    let bundleIdentifier = "fixture"
    let launchDate: Date? = Date(timeIntervalSince1970: 10)
}
enum Collector { static func windowOwnedBy(pid: Int, window: Int) -> Bool { identityMatches } }
struct Subject {
    let config = Config()
    func current('''+prefix+'''    }
}
func encodeIssue(_ error: Error, _ nextPhase: Int) -> (String, String) {
    let code: String, recovery: String
'''+mapper+'''
    return (nextPhase == 2 ? "action_outcome_unknown" : code, recovery)
}
var checks = 0
for permission in [false, true] {
    for identity in [false, true] {
        trusted = permission; identityMatches = identity
        do {
            try Subject().current(NativeAcquisition())
            precondition(permission && identity); checks += 1
        } catch {
            let expected = permission ? "stale_target" : "permission_required"
            for phase in [0, 1, 2, 3] {
                precondition(encodeIssue(error, phase).0 == (phase == 2 ? "action_outcome_unknown" : expected))
                checks += 1
            }
        }
    }
}
print("permission/identity phase boundaries: \\(checks) checks; SDK input: none")
'''
with tempfile.TemporaryDirectory(prefix='uib-n03-permission-') as directory:
    run = pathlib.Path(directory)
    (run/'Checks.swift').write_text(program)
    subprocess.run(['xcrun', 'swiftc', '-swift-version', '5', '-module-cache-path', str(run/'cache'), str(run/'Checks.swift'), '-o', str(run/'checks')], check=True, timeout=60)
    subprocess.run([str(run/'checks')], check=True, timeout=5)
assert not run.exists()
