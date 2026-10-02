#!/usr/bin/env bash
# Remove nlp-engine-ind build artifacts and dev tools installed for this project.
# After clone: run ./scripts/setup-macos.sh and the platform build scripts again.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

echo "==> Cleaning project build artifacts"

rm -rf \
  "$ROOT/target" \
  "$ROOT/release" \
  "$ROOT/bindings/uniffi/generated" \
  "$ROOT/bindings/android/nlp-engine-ind/src/main/jniLibs" \
  "$ROOT/integrations/web-example/pkg" \
  "$ROOT/integrations/node-example/pkg" \
  "$ROOT/integrations/android-keyboard/.gradle" \
  "$ROOT/integrations/android-keyboard/build" \
  "$ROOT/integrations/android-keyboard/app/build" \
  "$ROOT/bindings/android/nlp-engine-ind/.gradle" \
  "$ROOT/bindings/android/nlp-engine-ind/build"

find "$ROOT" -name ".DS_Store" -delete 2>/dev/null || true

echo "==> Removing wasm-pack cache"
rm -rf "$HOME/Library/Caches/.wasm-pack" 2>/dev/null || true

echo "==> Removing cargo-installed tools (wasm-pack, cargo-ndk)"
if [[ -x "$HOME/.cargo/bin/cargo" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env" 2>/dev/null || true
  cargo uninstall wasm-pack 2>/dev/null || true
  cargo uninstall cargo-ndk 2>/dev/null || true
fi
rm -f \
  "$HOME/.cargo/bin/wasm-pack" \
  "$HOME/.cargo/bin/wasm-pack.stamp" \
  "$HOME/.cargo/bin/cargo-ndk" \
  "$HOME/.cargo/bin/cargo-ndk-env" \
  "$HOME/.cargo/bin/cargo-ndk-runner" \
  "$HOME/.cargo/bin/cargo-ndk-test" \
  2>/dev/null || true

echo "==> Uninstalling Rust (rustup)"
if command -v rustup >/dev/null 2>&1; then
  rustup self uninstall -y || true
fi

# Remove leftover dirs if rustup uninstall missed anything
rm -rf "$HOME/.rustup" "$HOME/.cargo" 2>/dev/null || true

echo ""
echo "Cleanup complete."
echo ""
echo "Removed:"
echo "  - Project: target/, release/, generated bindings, jniLibs, WASM pkg, Gradle caches"
echo "  - Global:  ~/.rustup, ~/.cargo, wasm-pack cache, cargo-ndk, wasm-pack"
echo ""
echo "NOT removed (reinstall separately if needed):"
echo "  - Xcode / Command Line Tools"
echo "  - Android Studio / ANDROID_HOME SDK & NDK"
echo "  - Homebrew"
echo ""
echo "After cloning from GitHub:"
echo "  ./scripts/setup-macos.sh"
echo "  ./scripts/build-native.sh"
