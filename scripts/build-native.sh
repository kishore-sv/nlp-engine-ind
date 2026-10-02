#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$HOME/.cargo/env"

cargo_target_dir() {
  cargo metadata --format-version=1 --no-deps | python3 -c "import json, sys; print(json.load(sys.stdin)['target_directory'])"
}

uniffi_lib_path() {
  local target_dir="$1"
  local profile="$2"
  local lib_name="libnlp_engine_ind_uniffi"
  case "$(uname -s)" in
    Darwin) echo "${target_dir}/${profile}/${lib_name}.dylib" ;;
    Linux) echo "${target_dir}/${profile}/${lib_name}.so" ;;
    MINGW*|MSYS*|CYGWIN*) echo "${target_dir}/${profile}/${lib_name}.dll" ;;
    *) echo "ERROR: unsupported host OS for UniFFI library-mode binding generation" >&2; exit 1 ;;
  esac
}

echo "==> Building native workspace"
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release

echo "==> Generating UniFFI bindings"
cargo build -p nlp-engine-ind-uniffi --release
TARGET_DIR="$(cargo_target_dir)"
LIB="$(uniffi_lib_path "$TARGET_DIR" release)"
if [[ ! -f "$LIB" ]]; then
  echo "ERROR: UniFFI library not found at $LIB"
  exit 1
fi

mkdir -p bindings/uniffi/generated/kotlin bindings/uniffi/generated/swift
cargo run -p nlp-engine-ind-uniffi --bin uniffi-bindgen -- generate \
  --library "$LIB" \
  --language kotlin \
  --out-dir bindings/uniffi/generated/kotlin \
  --crate nlp_engine_ind_uniffi \
  --no-format
cargo run -p nlp-engine-ind-uniffi --bin uniffi-bindgen -- generate \
  --library "$LIB" \
  --language swift \
  --out-dir bindings/uniffi/generated/swift \
  --crate nlp_engine_ind_uniffi \
  --no-format

echo "Native build complete."
echo "UniFFI library: $LIB"
echo "Kotlin bindings: bindings/uniffi/generated/kotlin"
echo "Swift bindings: bindings/uniffi/generated/swift"
