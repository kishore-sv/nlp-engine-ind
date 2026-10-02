# iOS setup

## Prerequisites

1. **Xcode 15+** with iOS SDK
2. **Rust** iOS targets
3. **xcodegen** (optional, for project generation)

```bash
xcode-select --install
rustup target add aarch64-apple-ios aarch64-apple-ios-sim

brew install xcodegen   # optional
```

## Build XCFramework

```bash
./scripts/build-ios.sh
```

Output:

```text
release/ios/NlpEngineInd.xcframework
release/ios/swift-bindings/
```

## Open sample keyboard

```bash
cd integrations/ios-keyboard
xcodegen generate    # creates NlpEngineIndKeyboard.xcodeproj
open NlpEngineIndKeyboard.xcodeproj
```

1. Select the host app target, run on simulator or device
2. Settings → General → Keyboard → Add **NlpEngineInd**
3. Type English, tap **Convert ✨**

## Apple keyboard limitations

- Keyboard extensions have limited memory (~30–50 MB)
- `RequestsOpenAccess` is **false** — no network access by default
- Text context may be limited in some host apps
- Extension cannot access host app data

## Offline verification

Enable airplane mode. Conversion uses only the embedded Rust library.

## Troubleshooting

| Issue | Fix |
|-------|-----|
| XCFramework missing | Run `./scripts/build-ios.sh` first |
| Link errors | Ensure XCFramework is linked in extension target |
| Simulator vs device | XCFramework includes both device and sim slices |
