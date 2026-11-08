#!/usr/bin/env bash
# Writes integrations/android-keyboard/local.properties from ANDROID_HOME when missing.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROPS="$ROOT/integrations/android-keyboard/local.properties"

if [[ -f "$PROPS" ]]; then
  exit 0
fi

SDK="${ANDROID_HOME:-${ANDROID_SDK_ROOT:-}}"
if [[ -z "$SDK" || ! -d "$SDK" ]]; then
  echo "ERROR: Set ANDROID_HOME to your Android SDK before building Gradle projects." >&2
  echo "Example: export ANDROID_HOME=\"\$HOME/Library/Android/sdk\"" >&2
  exit 1
fi

mkdir -p "$(dirname "$PROPS")"
printf 'sdk.dir=%s\n' "$SDK" >"$PROPS"
echo "Wrote $PROPS (sdk.dir=$SDK)"
