# iOS keyboard sample

Demonstrates offline English → Roman Indian-language conversion via the Rust engine.

## Prerequisites

```bash
./scripts/build-ios.sh
brew install xcodegen   # one-time
```

## Generate & open

```bash
cd integrations/ios-keyboard
xcodegen generate
open NlpEngineIndKeyboard.xcodeproj
```

## Usage

1. Run the host app on simulator or device
2. Settings → Keyboard → Add **NlpEngineInd**
3. Open any text field, switch to this keyboard
4. Type English, select language, tap **Convert ✨**

## Privacy

- No Full Access required
- No network calls
- No text logging or persistence

See [docs/ios/keyboard.md](../../docs/ios/keyboard.md).
