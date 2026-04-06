# Homebrew cask for settl
# This is a template -- the CI workflow (update-homebrew.yml) auto-generates
# the real cask in mozilla-ai/homebrew-tap on each release.
#
# To bootstrap the tap manually:
#   1. Clone git@github.com:mozilla-ai/homebrew-tap.git
#   2. Copy this file to Casks/settl.rb
#   3. Replace VERSION and SHA256 placeholders with real values from a release

cask "settl" do
  name "settl"
  desc "Terminal hex-based settlement game with LLM players"
  homepage "https://github.com/mozilla-ai/settl"
  version "VERSION"

  livecheck do
    skip "Auto-generated on release."
  end

  on_macos do
    on_intel do
      url "https://github.com/mozilla-ai/settl/releases/download/vVERSION/settl-darwin-amd64.tar.gz"
      sha256 "SHA256_DARWIN_AMD64"
      binary "settl-darwin-amd64", target: "settl"
    end
    on_arm do
      url "https://github.com/mozilla-ai/settl/releases/download/vVERSION/settl-darwin-arm64.tar.gz"
      sha256 "SHA256_DARWIN_ARM64"
      binary "settl-darwin-arm64", target: "settl"
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/mozilla-ai/settl/releases/download/vVERSION/settl-linux-amd64.tar.gz"
      sha256 "SHA256_LINUX_AMD64"
      binary "settl-linux-amd64", target: "settl"
    end
    on_arm do
      url "https://github.com/mozilla-ai/settl/releases/download/vVERSION/settl-linux-arm64.tar.gz"
      sha256 "SHA256_LINUX_ARM64"
      binary "settl-linux-arm64", target: "settl"
    end
  end

  postflight do
    if OS.mac?
      system_command "/usr/bin/xattr", args: ["-dr", "com.apple.quarantine", staged_path.to_s], sudo: false
    end
  end
end
