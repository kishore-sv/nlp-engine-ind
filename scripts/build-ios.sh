#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$HOME/.cargo/env"

cargo_target_dir() {
  cargo metadata --format-version=1 --no-deps | python3 -c "import json, sys; print(json.load(sys.stdin)['target_directory'])"
}

if ! xcodebuild -version >/dev/null 2>&1; then
  echo "ERROR: Xcode is required for iOS builds."
  echo "Install Xcode and run: xcode-select --install"
  exit 1
fi

echo "==> Generating UniFFI bindings (host)"
"$ROOT/scripts/build-native.sh"

TARGET_DIR="$(cargo_target_dir)"

echo "==> Building iOS device library (aarch64-apple-ios)"
rustup target add aarch64-apple-ios >/dev/null 2>&1 || true
cargo build -p nlp-engine-ind-uniffi --release --target aarch64-apple-ios

echo "==> Building iOS simulator library (aarch64-apple-ios-sim)"
rustup target add aarch64-apple-ios-sim >/dev/null 2>&1 || true
cargo build -p nlp-engine-ind-uniffi --release --target aarch64-apple-ios-sim

OUT="$ROOT/release/ios"
mkdir -p "$OUT/device" "$OUT/sim"

DEVICE_LIB="$TARGET_DIR/aarch64-apple-ios/release/libnlp_engine_ind_uniffi.a"
SIM_LIB="$TARGET_DIR/aarch64-apple-ios-sim/release/libnlp_engine_ind_uniffi.a"

cp "$DEVICE_LIB" "$OUT/device/"
cp "$SIM_LIB" "$OUT/sim/"

xcodebuild -create-xcframework \
  -library "$OUT/device/libnlp_engine_ind_uniffi.a" \
  -library "$OUT/sim/libnlp_engine_ind_uniffi.a" \
  -output "$OUT/NlpEngineInd.xcframework"

mkdir -p "$OUT/swift-bindings"
cp bindings/uniffi/generated/swift/*.swift "$OUT/swift-bindings/" 2>/dev/null || true
cp bindings/uniffi/generated/swift/*.h "$OUT/swift-bindings/" 2>/dev/null || true
cp bindings/uniffi/generated/swift/*.modulemap "$OUT/swift-bindings/" 2>/dev/null || true

echo "XCFramework written to $OUT/NlpEngineInd.xcframework"
echo "Swift bindings copied to $OUT/swift-bindings"
