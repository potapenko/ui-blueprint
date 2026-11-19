"""Build a single-window helper from the exact identity supplied by its owner."""
import argparse
import subprocess
import tempfile
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("output", type=Path)
parser.add_argument("bundle_path", type=Path)
parser.add_argument("identity", nargs=4, metavar="PID/BUNDLE/BIRTH/CGID")
parser.add_argument("frame", nargs=4, type=int, metavar="FRAME_PT")
parser.add_argument("size", nargs=2, type=int, metavar="SIZE_PT")
args = parser.parse_args()
if not args.bundle_path.is_absolute():
    parser.error("bundle_path must be the owner's absolute app path")
# Encode arbitrary paths and bundle identifiers as UTF-8 byte literals. Owner
# input must never become Swift source, including quotes and interpolation.
def literal(text):
    return f"String(decoding: {list(text.encode())} as [UInt8], as: UTF8.self)"

with tempfile.TemporaryDirectory(prefix="uib-owner-build-") as directory:
    root = Path(directory)
    source = Path(__file__).with_name("resize-owned-window.swift")
    (root / "main.swift").write_bytes(source.read_bytes())
    x, y, width, height = args.frame
    arguments = args.identity + list(map(str, args.size))
    (root / "Owner.swift").write_text(
        "import CoreGraphics\n"
        + "let ownerArguments = [" + ", ".join(map(literal, arguments)) + "]\n"
        + f"let ownerBundlePath = {literal(str(args.bundle_path.resolve()))}\n"
        + f"let ownerStartingFrame = CGRect(x:{x}, y:{y}, width:{width}, height:{height})\n"
    )
    subprocess.run(["xcrun", "swiftc", "-D", "PINNED_OWNER_RESIZE", root / "main.swift",
                    root / "Owner.swift", "-o", args.output], check=True)
