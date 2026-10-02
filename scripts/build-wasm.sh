#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$HOME/.cargo/env"

if ! command -v wasm-pack >/dev/null 2>&1; then
  cargo install wasm-pack --locked
fi

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true

WEB_OUT="$ROOT/integrations/web-example/pkg"
NODE_OUT="$ROOT/integrations/node-example/pkg"
mkdir -p "$WEB_OUT" "$NODE_OUT"

echo "==> Building WASM (web target)"
wasm-pack build crates/nlp-engine-ind-wasm \
  --target web \
  --out-dir "$WEB_OUT" \
  --release

echo "==> Building WASM (nodejs target)"
wasm-pack build crates/nlp-engine-ind-wasm \
  --target nodejs \
  --out-dir "$NODE_OUT" \
  --release

echo "Web WASM package: integrations/web-example/pkg"
echo "Node WASM package: integrations/node-example/pkg"
echo "Serve web example: cd integrations/web-example && python3 -m http.server 8080"
