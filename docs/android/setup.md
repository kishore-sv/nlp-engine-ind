# Android setup

## Prerequisites

1. **Android Studio** (Ladybug or newer recommended)
2. **Android SDK** with API 34
3. **Android NDK** (installed via SDK Manager)
4. **Rust** with `cargo-ndk`

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/<version>"

cargo install cargo-ndk --locked
rustup target add aarch64-linux-android
```

Optional emulator ABI:

```bash
export BUILD_ANDROID_X86=1   # also builds x86_64 for emulator
rustup target add x86_64-linux-android
```

## Build native library

```bash
./scripts/build-android.sh
```

Output: `bindings/android/nlp-engine-ind/src/main/jniLibs/arm64-v8a/libnlp_engine_ind_uniffi.so`

UniFFI Kotlin bindings: `bindings/uniffi/generated/kotlin/`

## Open sample keyboard

```bash
open -a "Android Studio" integrations/android-keyboard
```

1. Sync Gradle
2. Run on arm64 device or emulator
3. Enable **NlpEngineInd** keyboard in Settings → System → Keyboard
4. Type English text, tap **Convert ✨**

## Offline verification

Enable airplane mode. Conversion must still work — no HTTP requests are made.

## Troubleshooting

| Issue | Fix |
|-------|-----|
| `ANDROID_HOME not set` | Export path to SDK |
| NDK not found | Install NDK in SDK Manager |
| `UnsatisfiedLinkError` | Re-run `./scripts/build-android.sh` |
| Wrong ABI | Ensure device is arm64-v8a or rebuild with x86_64 |
