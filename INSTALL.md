# Install & Run Guide

Complete setup for **nlp-engine-ind** from a fresh clone. The engine runs **fully offline** after the initial build (internet is only needed to download Rust crates and SDKs once).

---

## Quick start (macOS, ~10 minutes)

If you just cloned the repo and want to run the CLI:

```bash
git clone https://github.com/YOUR_USER/nlp-engine-ind.git
cd nlp-engine-ind

# 1. Install Rust, wasm-pack, and required targets
./scripts/setup-macos.sh
source "$HOME/.cargo/env"

# 2. Build and test
./scripts/build-native.sh

# 3. Run a conversion
cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"
```

Expected output (example):

```text
Language: Kannada
Input: I'll call you later
Output: Later call madtini
Confidence: 0.90
```

---

## What gets installed

| Component | Required for | How it is installed |
|-----------|--------------|---------------------|
| **Rust (stable)** | Everything | `./scripts/setup-macos.sh` or [rustup.rs](https://rustup.rs) |
| **rustfmt, clippy** | Lint / CI | `setup-macos.sh` |
| **wasm32 target** | Web / Node WASM | `setup-macos.sh` |
| **wasm-pack** | Web / Node WASM | `setup-macos.sh` |
| **cargo-ndk** | Android native libs | `./scripts/build-android.sh` (auto-installs) |
| **Xcode** | iOS builds | [Mac App Store](https://apps.apple.com/app/xcode/id497799835) |
| **Android Studio + NDK** | Android builds | [developer.android.com/studio](https://developer.android.com/studio) |
| **XcodeGen** | iOS keyboard Xcode project | `brew install xcodegen` |
| **Python 3** | Local web demo server | Usually pre-installed on macOS |

---

## Step-by-step: macOS

### 1. Clone the repository

```bash
git clone https://github.com/YOUR_USER/nlp-engine-ind.git
cd nlp-engine-ind
```

### 2. Optional: Homebrew

Useful for XcodeGen and other tools:

```bash
brew --version
# If missing:
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 3. Install Rust and core tooling

Automated (recommended):

```bash
./scripts/setup-macos.sh
source "$HOME/.cargo/env"
```

Or manual:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup component add rustfmt clippy rust-analyzer
rustup target add wasm32-unknown-unknown
cargo install wasm-pack --locked
```

Verify:

```bash
rustc --version
cargo --version
rustup --version
```

Add to `~/.zshrc` if commands are not found after reopening the terminal:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

### 4. Build the project

Full native build (format, lint, test, release, UniFFI bindings):

```bash
./scripts/build-native.sh
```

Or minimal build:

```bash
cargo build --workspace --release
cargo test --workspace --all-features
```

### 5. Run the CLI

Single sentence (all 5 languages):

```bash
cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"
cargo run -p nlp-engine-ind-cli -- --language hi "Bro send the file"
cargo run -p nlp-engine-ind-cli -- --language te "Are you coming?"
cargo run -p nlp-engine-ind-cli -- --language ta "Can you call me?"
cargo run -p nlp-engine-ind-cli -- --language ml "I'll finish this work"
```

JSON output:

```bash
cargo run -p nlp-engine-ind-cli -- --language kn --json "I'll call you later"
```

Interactive mode:

```bash
cargo run -p nlp-engine-ind-cli -- interactive
```

Commands inside interactive mode: `:lang hi`, `:quit`

### 6. Run examples

```bash
cargo run -p nlp-engine-ind-core --example basic
cargo run -p nlp-engine-ind-core --example multilingual
cargo run -p nlp-engine-ind-core --example code_mixing
cargo run -p nlp-engine-ind-core --example performance
```

### 7. Run benchmarks

```bash
cargo bench -p nlp-engine-ind-core --bench pipeline
```

### 8. Makefile shortcuts

```bash
make build    # release build
make test     # all tests
make lint     # clippy with -D warnings
make fmt      # format check
make run      # sample CLI conversion
```

---

## Platform integrations (optional)

Generated binaries are **not** committed to Git. Build them locally after clone.

### Web (browser)

```bash
./scripts/build-wasm.sh
cd integrations/web-example
python3 -m http.server 8080
```

Open [http://localhost:8080](http://localhost:8080), type English text, pick a language, click **Convert**. No server-side NLP — WASM runs in the browser.

### Node.js

```bash
./scripts/build-wasm.sh
node integrations/node-example/convert.mjs "I'll call you later" kn
```

### Desktop (macOS / Linux / Windows)

```bash
# macOS
./integrations/macos-example/run.sh

# Linux
./integrations/linux-example/run.sh

# Windows (PowerShell)
.\integrations\windows-example\run.ps1
```

### Android keyboard

**Prerequisites**

1. Install [Android Studio](https://developer.android.com/studio)
2. Install Android SDK (API 34) and NDK via SDK Manager
3. Set environment variable:

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/$(ls "$ANDROID_HOME/ndk" | tail -1)"
```

Add to `~/.zshrc` to persist.

**Build & run**

```bash
./scripts/build-android.sh
open -a "Android Studio" integrations/android-keyboard
```

In Android Studio: sync Gradle → run on arm64 device/emulator → enable **NlpEngineInd** keyboard in Settings → System → Keyboard.

Optional x86_64 emulator support:

```bash
BUILD_ANDROID_X86=1 ./scripts/build-android.sh
```

### iOS keyboard

**Prerequisites**

1. Install **Xcode** from the Mac App Store
2. Accept license and install CLI tools:

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
xcodebuild -version
```

3. Install XcodeGen:

```bash
brew install xcodegen
```

**Build & run**

```bash
./scripts/build-ios.sh
cd integrations/ios-keyboard
xcodegen generate
open NlpEngineIndKeyboard.xcodeproj
```

Run the host app on simulator or device → Settings → Keyboard → add **NlpEngineInd** → type English → tap **Convert ✨**.

---

## Run everything (where SDKs are available)

```bash
./scripts/test-all.sh
```

Runs native build, WASM build, and Android/iOS builds if `ANDROID_HOME` and Xcode are configured.

---

## Clean reinstall

Remove all build artifacts and globally installed Rust tools:

```bash
./scripts/cleanup-macos.sh
```

Then clone again and repeat from **Quick start**.

---

## Troubleshooting

### `rustc` or `cargo` not found

```bash
source "$HOME/.cargo/env"
export PATH="$HOME/.cargo/bin:$PATH"
```

### First build is slow

The first `cargo build` downloads crates from crates.io. Later builds are incremental and offline-capable.

### Clippy fails

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Fix reported issues, then re-run `./scripts/build-native.sh`.

### Android: `ANDROID_HOME not set`

Install Android Studio, then:

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
```

### Android: NDK not found

Open Android Studio → SDK Manager → SDK Tools → check **NDK (Side by side)**.

### iOS: Xcode required

iOS builds cannot be done with Cargo alone. Install Xcode from the App Store.

### Web: blank page or WASM error

Rebuild WASM:

```bash
./scripts/build-wasm.sh
```

Serve from `integrations/web-example/` (not the repo root).

### Disk space

Full platform builds need several GB free. If builds fail with "No space left on device":

```bash
cargo clean
./scripts/cleanup-macos.sh
```

---

## Verify installation checklist

After setup, these should all succeed:

```bash
source "$HOME/.cargo/env"

cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo build --workspace --release
cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"
```

---

## More documentation

| Topic | File |
|-------|------|
| macOS details | [docs/development/macos.md](docs/development/macos.md) |
| Building all targets | [docs/development/building.md](docs/development/building.md) |
| Testing | [docs/development/testing.md](docs/development/testing.md) |
| Android | [docs/android/setup.md](docs/android/setup.md) |
| iOS | [docs/ios/setup.md](docs/ios/setup.md) |
| Web / WASM | [docs/web/wasm.md](docs/web/wasm.md) |
| Architecture | [docs/architecture/overview.md](docs/architecture/overview.md) |

---

## Privacy reminder

The engine does not send text anywhere, log messages, or require internet at runtime. Network is only used during **initial tool and dependency installation**.
