# Building

## Native (host)

```bash
./scripts/build-native.sh
```

Runs: fmt, clippy, test, release build, UniFFI binding generation.

## WASM

```bash
./scripts/build-wasm.sh
```

Requires `wasm-pack` (installed by `setup-macos.sh`).

## Android

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
./scripts/build-android.sh
```

Requires: Android NDK, `cargo-ndk`, `aarch64-linux-android` target.

Set `BUILD_ANDROID_X86=1` to also build x86_64 for emulators.

## iOS

```bash
./scripts/build-ios.sh
```

Requires: Xcode, `aarch64-apple-ios`, `aarch64-apple-ios-sim` targets.

Output: `release/ios/NlpEngineInd.xcframework`

## Rust targets

| Target | Platform |
|--------|----------|
| `aarch64-apple-darwin` | macOS Apple Silicon |
| `x86_64-apple-darwin` | macOS Intel |
| `aarch64-linux-android` | Android arm64 |
| `aarch64-apple-ios` | iOS device |
| `aarch64-apple-ios-sim` | iOS simulator |
| `wasm32-unknown-unknown` | Web |
| `x86_64-pc-windows-msvc` | Windows |
| `x86_64-unknown-linux-gnu` | Linux |

Install targets:

```bash
rustup target add <target>
```

## Release artifacts

Generated binaries are gitignored. Reproduce with build scripts:

```text
release/ios/NlpEngineInd.xcframework
bindings/android/nlp-engine-ind/src/main/jniLibs/
integrations/web-example/pkg/
bindings/uniffi/generated/
```

## Makefile shortcuts

```bash
make build
make test
make lint
make fmt
make run
```
