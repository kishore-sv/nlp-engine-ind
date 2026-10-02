#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$HOME/.cargo/env"

if [[ -z "${ANDROID_HOME:-}" || ! -d "$ANDROID_HOME" ]]; then
  echo "ERROR: ANDROID_HOME is not set or invalid."
  echo "Install Android Studio and export ANDROID_HOME=~/Library/Android/sdk"
  exit 1
fi

NDK="${ANDROID_NDK_HOME:-$ANDROID_HOME/ndk/$(ls "$ANDROID_HOME/ndk" 2>/dev/null | tail -1)}"
if [[ ! -d "$NDK" ]]; then
  echo "ERROR: Android NDK not found under $ANDROID_HOME/ndk"
  exit 1
fi

if ! command -v cargo-ndk >/dev/null 2>&1; then
  echo "Installing cargo-ndk..."
  cargo install cargo-ndk --locked
fi

echo "==> Generating UniFFI bindings (host)"
"$ROOT/scripts/build-native.sh"

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
echo "Open integrations/android-keyboard in Android Studio to build the sample keyboard."
