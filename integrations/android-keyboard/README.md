# Android keyboard sample

Demonstrates offline English → Roman Indian-language conversion via the Rust engine.

## Fresh clone build

See **[BUILD.md](BUILD.md)** for reproducible CLI steps (Gradle Wrapper, `./scripts/build-android.sh`, `./gradlew`).

Quick path from repo root:

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
./scripts/build-android.sh
cd integrations/android-keyboard
./gradlew :app:assembleDebug
```

## Prerequisites

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
./scripts/build-android.sh
```

## Open in Android Studio

```bash
open -a "Android Studio" integrations/android-keyboard
```

## Usage

1. Run on arm64 device or emulator (x86_64 emulator requires `BUILD_ANDROID_X86=1` when running `build-android.sh`)
2. Settings → System → Keyboard → Enable **NLP Engine Keyboard**
3. Type English in any app
4. Tap **Convert ✨**

## Privacy

- No internet permission
- No text logging or persistence
- Conversion runs on background thread

See [docs/android/keyboard.md](../../docs/android/keyboard.md).
