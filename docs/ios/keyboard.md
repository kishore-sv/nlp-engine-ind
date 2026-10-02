# iOS keyboard integration

Sample extension at `integrations/ios-keyboard/`.

## Architecture

```text
KeyboardViewController (UIInputViewController)
        │
        ▼
NlpEngine (UniFFI Swift)
        │
        ▼
NlpEngineInd.xcframework (Rust static lib)
```

## UX

- Language segmented control (KN, HI, TE, TA, ML)
- **Convert ✨** button
- Converts selected text or current sentence via `textDocumentProxy`

## Security & privacy

Configured in `Info.plist`:

- `RequestsOpenAccess`: **false** — no network, no full access
- No logging of user text
- No disk writes of conversion inputs
- Conversion runs on background queue

## Performance

- Engine initialized once in `viewDidLoad`
- Conversion on `DispatchQueue.global(qos: .userInitiated)`
- Result inserted on main queue

## Generate Xcode project

```bash
./scripts/build-ios.sh
cd integrations/ios-keyboard && xcodegen generate
open NlpEngineIndKeyboard.xcodeproj
```

Requires [XcodeGen](https://github.com/yonaskolb/XcodeGen) (`brew install xcodegen`).

## Testing

Run on arm64 simulator or device. Verify offline conversion with airplane mode enabled.
