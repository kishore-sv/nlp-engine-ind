# iOS library

## Artifacts

After `./scripts/build-ios.sh`:

| File | Purpose |
|------|---------|
| `release/ios/NlpEngineInd.xcframework` | Static library for device + simulator |
| `release/ios/swift-bindings/*.swift` | UniFFI Swift API |
| `release/ios/swift-bindings/*.h` | C FFI header |
| `bindings/uniffi/generated/swift/` | Source bindings (regenerated on build) |

## Swift API

```swift
import nlp_engine_ind_uniffi

let engine = try NlpEngine(
    config: EngineConfig(
        language: .kannada,
        style: .casual,
        codeMix: .natural,
        romanization: .casualChat,
        enabledLanguages: [.kannada]
    )
)

let result = try engine.convertText(text: "I'll call you later")
print(result.output)
```

## Keyboard helper

```swift
let result = try engine.convertForKeyboard(text: source)
// result.output, result.confidence, result.processingTimeUs
```

## Xcode integration

1. Drag `NlpEngineInd.xcframework` into your project
2. Add Swift binding files to your target
3. Link the XCFramework in **Frameworks, Libraries, and Embedded Content**

## Thread safety

`NlpEngine` uses internal mutexes. Reuse one instance per keyboard extension — do not reinitialize on every keystroke.

## No duplicate NLP logic

All conversion is in Rust. Swift is a thin wrapper only.
