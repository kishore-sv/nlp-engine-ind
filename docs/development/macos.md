# macOS development setup for nlp-engine-ind

This guide sets up a clean Rust development environment on macOS for **nlp-engine-ind**.

## 1. Check Homebrew

Homebrew is optional but useful for supporting tools.

```bash
brew --version
```

If Homebrew is not installed:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

## 2. Install Rust with rustup

Rust is **required**. Do not assume it is already installed.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Choose the default installation (stable toolchain).

Load Rust into your current shell:

```bash
source "$HOME/.cargo/env"
```

## 3. Install toolchain components

The project pins stable Rust via `rust-toolchain.toml` and expects:

```bash
rustup component add rustfmt clippy
```

Optional IDE support:

```bash
rustup component add rust-analyzer
```

## 4. Verify installation

```bash
rustc --version
cargo --version
rustup --version
```

You should see stable versions for all three commands.

## 5. Clone and enter the project

```bash
cd /path/to/nlp-engine-ind
```

## 6. Build

```bash
cargo build --workspace
```

Release build:

```bash
cargo build --workspace --release
```

## 7. Test

```bash
cargo test --workspace --all-features
```

## 8. Format and lint

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Or use Makefile shortcuts:

```bash
make fmt
make lint
make test
```

## 9. Run the CLI

Single conversion:

```bash
cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"
```

JSON output:

```bash
cargo run -p nlp-engine-ind-cli -- --language kn --json "I'll call you later"
```

Interactive mode:

```bash
cargo run -p nlp-engine-ind-cli -- interactive
```

Inside interactive mode:

```text
:lang hi
:quit
```

## 10. Run examples

```bash
cargo run -p nlp-engine-ind-core --example basic
cargo run -p nlp-engine-ind-core --example multilingual
cargo run -p nlp-engine-ind-core --example code_mixing
cargo run -p nlp-engine-ind-core --example performance
```

## 11. Benchmarks

```bash
cargo bench --workspace
```

## 12. Troubleshooting

### `rustc` or `cargo` not found

```bash
source "$HOME/.cargo/env"
```

Add to `~/.zshrc`:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### Wrong Rust version

```bash
rustup update stable
rustup default stable
```

### Clippy warnings fail CI

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Fix reported issues, then re-run tests.

### Build fails on first run

The first `cargo build` downloads crates and may take a few minutes. Ensure network access is available for the initial build only — the **engine itself runs fully offline**.

## Notes

- This project does **not** require Python, Node.js, or cloud APIs for the Rust engine.
- Platform apps (Android, iOS, Web) will consume the Rust core via FFI/WASM later.
