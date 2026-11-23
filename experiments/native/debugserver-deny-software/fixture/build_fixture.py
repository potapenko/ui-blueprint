"""Build and smoke-test only the self-owned arm64 geometry fixture on macOS."""
import hashlib
import os
from pathlib import Path
import plistlib
import subprocess
import tempfile


def main():
    scratch = Path("/Volumes/Develop-2TB/Develop/tmp")
    volume = Path("/Volumes/Develop-2TB")
    if not volume.is_mount() or not scratch.is_dir() or not os.access(scratch, os.W_OK):
        raise SystemExit("External temporary volume is unavailable")
    root = Path(tempfile.mkdtemp(prefix="uib-geometry-control-", dir=scratch))
    env = dict(os.environ, TMPDIR=str(root), TMP=str(root), TEMP=str(root))
    (root / "OWNERSHIP.md").write_text(
        "UI Blueprint hardware-breakpoint experiment, owned console fixture. "
        "Retain this generated Debug binary, dSYM and qualification directories "
        "until the operator ends the experiment. The compiler module cache is "
        "disposable after verification.\n"
    )
    src = Path(__file__).resolve().parent
    executable = root / "Control"
    subprocess.run([
        "xcrun", "swiftc", "-g", "-Onone", "-module-name", "Control",
        "-module-cache-path", str(root / "module-cache"), src / "main.swift",
        "-o", executable,
    ], cwd=root, env=env, check=True)
    subprocess.run([
        "codesign", "--force", "--sign", "-", "--entitlements",
        src / "fixture.entitlements", executable,
    ], cwd=root, env=env, check=True)
    subprocess.run(["codesign", "--verify", "--strict", executable], env=env, check=True)
    entitlements = subprocess.check_output([
        "codesign", "-d", "--entitlements", "-", "--xml", executable,
    ], env=env, stderr=subprocess.DEVNULL)
    if plistlib.loads(entitlements) != {"com.apple.security.get-task-allow": True}:
        raise RuntimeError("Unexpected fixture signing entitlements")
    smoke = root / "smoke"
    smoke.mkdir()
    (smoke / "finish").touch()
    output = subprocess.check_output([executable], cwd=smoke, env=env, text=True, timeout=3)
    if output.splitlines()[-1:] != ["recovered"]:
        raise RuntimeError("Fixture did not complete its geometry check")
    print(executable)
    print("sha256", hashlib.sha256(executable.read_bytes()).hexdigest())
    subprocess.run(["xcrun", "dwarfdump", "--uuid", executable], env=env, check=True)


if __name__ == "__main__":
    main()
