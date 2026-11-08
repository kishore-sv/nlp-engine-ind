# Android keyboard — fresh clone build

Reproducible CLI build for the sample IME at `integrations/android-keyboard/`.

## Prerequisites

| Tool | Purpose |
|------|---------|
| [Rust](https://rustup.rs) | Host UniFFI build + `cargo-ndk` |
| Android SDK (API **34**) | `compileSdk = 34` |
| Android NDK | Native `.so` via `cargo-ndk` |
| JDK **17** | Gradle / AGP 8.5.2 |

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"   # macOS default
# Optional: export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/<version>"

cargo install cargo-ndk --locked
rustup target add aarch64-linux-android
# For x86_64 emulator images only:
# rustup target add x86_64-linux-android
```

Install **Android SDK Platform 34** if missing (Android Studio SDK Manager or `sdkmanager "platforms;android-34"`).

## Step 1 — Native Rust artifacts (required)

From the repository root:

```bash
./scripts/build-android.sh
```

This generates (gitignored, local only):

- `bindings/uniffi/generated/kotlin/` — UniFFI Kotlin API for `:nlp-engine-ind`
- `bindings/android/nlp-engine-ind/src/main/jniLibs/arm64-v8a/libnlp_engine_ind_uniffi.so`

Optional x86_64 emulator libraries:

```bash
BUILD_ANDROID_X86=1 ./scripts/build-android.sh
```

The script also writes `integrations/android-keyboard/local.properties` from `ANDROID_HOME` when that file is missing.

## Step 2 — Gradle APK

The project includes a **Gradle Wrapper** (Gradle **8.9**, minimum **8.7** for AGP **8.5.2**).

```bash
cd integrations/android-keyboard
./gradlew :nlp-engine-ind:assembleRelease   # library AAR (optional)
./gradlew :app:assembleDebug                 # debug APK
```

Debug APK output:

`integrations/android-keyboard/app/build/outputs/apk/debug/app-debug.apk`

## Android Studio

Open `integrations/android-keyboard`, sync Gradle, run **app** on an **arm64-v8a** device or an x86_64 emulator (only if you built with `BUILD_ANDROID_X86=1`).

## Full Rust validation (optional)

```bash
./scripts/build-native.sh   # fmt, clippy, tests, release, UniFFI bindings
```

## Troubleshooting

| Issue | Fix |
|-------|-----|
| Missing Kotlin under `bindings/uniffi/generated/kotlin` | Run `./scripts/build-android.sh` |
| `UnsatisfiedLinkError` at runtime | Re-run `./scripts/build-android.sh`; match device ABI |
| `sdk.dir` not set | Export `ANDROID_HOME` and run `./scripts/ensure-android-local-properties.sh` |
| Platform android-34 not found | Install SDK Platform 34 |

See also [docs/android/setup.md](../../docs/android/setup.md).
