#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$HOME/.cargo/env"

"$ROOT/scripts/build-native.sh"
"$ROOT/scripts/build-wasm.sh"

if [[ -n "${ANDROID_HOME:-}" ]]; then
  "$ROOT/scripts/build-android.sh" || echo "Android build skipped/failed"
else
  echo "Skipping Android build (ANDROID_HOME not set)"
fi

if xcodebuild -version >/dev/null 2>&1; then
  "$ROOT/scripts/build-ios.sh" || echo "iOS build skipped/failed"
else
  echo "Skipping iOS build (Xcode not available)"
fi

echo "All local tests/build steps attempted."
