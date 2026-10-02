#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
source "$HOME/.cargo/env"
cargo run -p nlp-engine-ind-cli --release -- --language kn "$@"
