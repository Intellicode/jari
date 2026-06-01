class Jari < Formula
  desc "Jira Cloud CLI for LLM coding agents"
  homepage "https://github.com/Intellicode/jari"
  version "0.1.0"
  license "MIT"

  if OS.mac?
    if Hardware::CPU.arm?
      url "https://github.com/Intellicode/jari/releases/download/v0.1.0/jari-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64_MAC"
    else
      url "https://github.com/Intellicode/jari/releases/download/v0.1.0/jari-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_MAC"
    end
  elsif OS.linux?
    if Hardware::CPU.arm?
      url "https://github.com/Intellicode/jari/releases/download/v0.1.0/jari-aarch64-unknown-linux-musl.tar.gz"
      sha256 "PLACEHOLDER_SHA256_ARM64_LINUX"
    else
      url "https://github.com/Intellicode/jari/releases/download/v0.1.0/jari-x86_64-unknown-linux-musl.tar.gz"
      sha256 "PLACEHOLDER_SHA256_X86_LINUX"
    end
  end

  def install
    bin.install "jari"
    bash_completion.install "completions/jari.bash" => "jari"
    zsh_completion.install "completions/jari.zsh" => "_jari"
    fish_completion.install "completions/jari.fish" => "jari.fish"
  end

  test do
    system "#{bin}/jari", "--version"
  end
end
