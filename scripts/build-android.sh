#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

if [[ -z "${ANDROID_HOME:-}" || ! -d "$ANDROID_HOME" ]]; then
  echo "ERROR: ANDROID_HOME is not set or invalid."
  echo "Install Android Studio and export ANDROID_HOME=~/Library/Android/sdk"
  exit 1
fi

NDK_VERSION="$(ls -1 "$ANDROID_HOME/ndk" 2>/dev/null | sort -V | tail -1 || true)"
NDK="${ANDROID_NDK_HOME:-${NDK_VERSION:+"$ANDROID_HOME/ndk/$NDK_VERSION"}}"
if [[ -z "${NDK:-}" || ! -d "$NDK" ]]; then
  echo "ERROR: Android NDK not found under $ANDROID_HOME/ndk"
  echo "Install NDK via Android Studio SDK Manager."
  exit 1
fi
export ANDROID_NDK_HOME="$NDK"

if ! command -v cargo >/dev/null 2>&1; then
  echo "ERROR: Rust (cargo) is not installed. Install from https://rustup.rs" >&2
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  echo "Installing cargo-ndk..."
  cargo install cargo-ndk --locked
fi

echo "==> Generating UniFFI Kotlin bindings (host)"
"$ROOT/scripts/generate-uniffi-bindings.sh"

rustup target add aarch64-linux-android >/dev/null 2>&1 || true

NDK_ARGS=(-t arm64-v8a)
if [[ "${BUILD_ANDROID_X86:-0}" == "1" ]]; then
  rustup target add x86_64-linux-android >/dev/null 2>&1 || true
  NDK_ARGS+=(-t x86_64)
fi

echo "==> Building Android native libraries (${NDK_ARGS[*]})"
cargo ndk -o bindings/android/nlp-engine-ind/src/main/jniLibs \
  "${NDK_ARGS[@]}" \
  build --release -p nlp-engine-ind-uniffi

echo "==> Android native libraries written to bindings/android/nlp-engine-ind/src/main/jniLibs"
for abi_dir in bindings/android/nlp-engine-ind/src/main/jniLibs/*/; do
  if [[ -d "$abi_dir" ]]; then
    ls -la "$abi_dir"
  fi
done

"$ROOT/scripts/ensure-android-local-properties.sh"

echo ""
echo "Next: build the keyboard APK from integrations/android-keyboard"
echo "  cd integrations/android-keyboard"
echo "  ./gradlew :app:assembleDebug"
