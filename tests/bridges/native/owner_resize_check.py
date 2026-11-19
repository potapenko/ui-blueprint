"""Exercise the pinned helper only against a new isolated macOS fixture."""
import json
import os
import plistlib
import signal
import subprocess
import sys
import tempfile
import time
from pathlib import Path

builder = Path(__file__).resolve().parents[3] / "plugins/macos/build-owner-resize.py"
with tempfile.TemporaryDirectory(prefix="uib-owner-resize-check-") as task_dir:
    root = Path(task_dir)
    app = root / "Owned.app"
    binary = app / "Contents/MacOS/Owned"
    binary.parent.mkdir(parents=True)
    bundle = "me.uiblueprint.external-layout.canary." + root.name
    (app / "Contents/Info.plist").write_bytes(plistlib.dumps({
        "CFBundleIdentifier": bundle, "CFBundleExecutable": "Owned", "LSUIElement": True,
        "CFBundleName": "Owned", "CFBundlePackageType": "APPL",
    }))
    state = root / "state"
    source = root / "main.swift"
    source.write_text("""import SwiftUI
let app = NSApplication.shared
app.setActivationPolicy(.accessory)
let window = NSWindow(contentRect: CGRect(x:150,y:150,width:640,height:370),
                      styleMask:[.titled,.resizable],backing:.buffered,defer:false)
window.contentView = NSHostingView(rootView: Text("UI Blueprint resize fixture").frame(maxWidth:.infinity,maxHeight:.infinity))
window.orderFront(nil)
let timer = Timer.scheduledTimer(withTimeInterval:0.1,repeats:true) { _ in
    let id = CGWindowID(window.windowNumber)
    guard let info = CGWindowListCopyWindowInfo(.optionIncludingWindow,id) as? [[String:Any]],
          let bounds = info.first?[kCGWindowBounds as String] as? NSDictionary,
          let rect = CGRect(dictionaryRepresentation:bounds as CFDictionary),
          let birth = NSRunningApplication.current.launchDate?.timeIntervalSince1970 else { return }
    let observed: [String:Any] = ["pid": getpid(), "birth": String(birth), "cg": id,
        "frame":[rect.minX,rect.minY,rect.width,rect.height],
        "size":[window.frame.width,window.frame.height]]
    try! JSONSerialization.data(withJSONObject: observed).write(
        to:URL(fileURLWithPath:CommandLine.arguments[1]),options:.atomic)
}
let stop = Timer.scheduledTimer(withTimeInterval:30,repeats:false) { _ in app.terminate(nil) }
app.run()
""")
    subprocess.run(["xcrun", "swiftc", source, "-o", binary], check=True)
    subprocess.run(["codesign", "--force", "--sign", "-", app], check=True)
    subprocess.run(["open", "-n", "-g", app, "--args", state], check=True)
    for _ in range(70):
        if state.exists():
            break
        time.sleep(0.1)
    time.sleep(0.7)  # Let the hosting view complete its initial window sizing.

    initial = json.loads(state.read_text())
    pid = initial["pid"]
    identity = [str(pid), bundle, initial["birth"], str(initial["cg"])]
    start = [str(int(i)) for i in initial["frame"]]
    requested = ["777", "480"]
    helper = root / "resize"

    def build(path, frame):
        subprocess.run([sys.executable, builder, helper, path, *identity, *frame, *requested], check=True)

    def refuses(arguments, expected):
        result = subprocess.run([helper, *arguments], capture_output=True, text=True)
        assert (result.returncode, result.stderr) == (4, expected), result
        assert json.loads(state.read_text())["frame"] == initial["frame"]

    try:
        arguments = identity + requested
        build(root / "Other.app", start)
        refuses(arguments, "owner_bundle_path_mismatch")
        build(app, [*start[:2], str(int(start[2]) + 1), start[3]])
        refuses(arguments, "owner_starting_frame_mismatch")
        build(app, start)
        for index in range(len(arguments)):
            changed = arguments.copy()
            changed[index] = "wrong.bundle" if index == 1 else str(int(float(arguments[index])) + 1)
            refuses(changed, "owner_arguments_mismatch")
        subprocess.run([helper, *arguments], check=True)
        deadline = time.monotonic() + 2
        while time.monotonic() < deadline:
            evidence = json.loads(state.read_text())
            if evidence["frame"][2:] == evidence["size"] == [777, 480]:
                break
            time.sleep(0.1)
        else:
            raise AssertionError(evidence)
        assert all(evidence[k] == initial[k] for k in ("pid", "cg", "birth"))
        print("Independent fixture readback", evidence)
    finally:
        # Fixture reports its own identity and exits automatically after 30s.
        # Signal only while its bounded lifetime and PID still belong to this binary.
        command = subprocess.run(["ps", "-p", str(pid), "-o", "comm="], capture_output=True, text=True)
        if Path(command.stdout.strip()).resolve() == binary.resolve():
            os.kill(pid, signal.SIGTERM)
