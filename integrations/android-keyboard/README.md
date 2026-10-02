# Android keyboard sample

Demonstrates offline English → Roman Indian-language conversion via the Rust engine.

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

1. Run on arm64 device or emulator
2. Settings → System → Keyboard → Enable **NlpEngineInd**
3. Type English in any app
4. Tap **Convert ✨**

## Privacy

- No internet permission
- No text logging or persistence
- Conversion runs on background thread

See [docs/android/keyboard.md](../../docs/android/keyboard.md).
