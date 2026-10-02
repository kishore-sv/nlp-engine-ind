#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> nlp-engine-ind macOS setup"

if command -v brew >/dev/null 2>&1; then
  echo "Homebrew: $(brew --version | head -1)"
else
  echo "Homebrew not found (optional)."
fi

if ! command -v rustc >/dev/null 2>&1; then
  echo "Installing Rust via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
fi

# shellcheck disable=SC1091
source "$HOME/.cargo/env"

rustup component add rustfmt clippy rust-analyzer
rustup target add wasm32-unknown-unknown

echo "Rust: $(rustc --version)"
echo "Cargo: $(cargo --version)"
echo "Rustup: $(rustup --version | head -1)"

if command -v xcode-select >/dev/null 2>&1; then
  echo "Xcode CLI: $(xcode-select -p)"
else
  echo "WARNING: Xcode command line tools not found. iOS builds will fail until installed."
fi

if [[ -n "${ANDROID_HOME:-}" && -d "$ANDROID_HOME" ]]; then
  echo "Android SDK: $ANDROID_HOME"
else
  echo "WARNING: ANDROID_HOME not set. Android native builds require Android Studio + NDK."
fi

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "Installing wasm-pack..."
  cargo install wasm-pack --locked
fi

echo "Setup complete."
