# Release template: release.py renders version, digest and supported macOS floor.
# License intentionally pending the owner's decision; do not publish this template.
class Uiblueprint < Formula
  desc "Local UI geometry analysis and bounded Mac and Web observation"
  homepage "https://github.com/potapenko/ui-blueprint"
  url "https://github.com/potapenko/ui-blueprint/releases/download/v@VERSION@/uiblueprint-v@VERSION@-aarch64-apple-darwin.tar.gz"
  version "@VERSION@"
  sha256 "@ARCHIVE_SHA256@"

  depends_on :macos
  depends_on arch: :arm64

  def install
    odie "Requires macOS @MIN_MACOS@ or later" if MacOS.version < Version.new("@MIN_MACOS@")
    # Keep worker/helper, schemas and optional Web caller beside the real CLI.
    libexec.install Dir["*", ".??*"].reject { |name| name == "." || name == ".." }
    bin.install_symlink libexec/"uiblueprint", libexec/"uiblueprint-validate"
  end

  def caveats
    <<~EOS
      Complete bundle: #{opt_libexec}
      Optional Web caller needs separately installed Node.js 24+.
      Native observation needs the user's macOS permissions and explicit connection.
      Stop active sessions before upgrading. Keep trusted helper/worker paths together.
    EOS
  end

  test do
    assert_match "Usage:", shell_output("#{bin}/uiblueprint --help")
    output = shell_output("#{bin}/uiblueprint-validate --max-bytes 131072 #{libexec}/example-snapshot.json")
    assert_match '"valid":true', output.delete(" \n")
  end
end
