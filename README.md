# nlp-engine-ind

**Offline Indian conversational NLP engine** — a reusable Rust SDK that converts English chat into casual, code-mixed Indian-language conversational text in **Roman script**.

> **New here?** See **[INSTALL.md](INSTALL.md)** for full install and run instructions from a fresh clone.

This is **not** a traditional translation engine. It targets natural messaging style — the way people text on WhatsApp, Instagram, Telegram, and similar apps.

```text
English message
      ↓
NLP understanding (intent, tense, grammar)
      ↓
Conversational generation + code mixing
      ↓
Roman-script Indian-language chat output
```

## Example

**Input**

```text
I'll finish this work and call you once I'm free.
```

**Kannada (Roman script)**

```text
Ee work mugsi free aadmele call madtini
```

**Input**

```text
Bro, can you send me the project?
```

**Hindi (Roman script)**

```text
Bro, mujhe project send kar do
```

English words like `work`, `call`, `project`, `meeting`, and `free` are intentionally preserved when natural.

## Supported languages

| Language  | Code | Feature flag |
|-----------|------|----------------|
| Kannada   | `kn` | `kannada`      |
| Hindi     | `hi` | `hindi`        |
| Telugu    | `te` | `telugu`       |
| Tamil     | `ta` | `tamil`        |
| Malayalam | `ml` | `malayalam`    |

Load only the languages you need via `EngineConfig.enabled_languages`.

## Architecture

```text
                         nlp-engine-ind
                               │
                    ┌──────────┴──────────┐
                    │                     │
               Rust Core               SDK Layer
                    │                     │
        ┌───────────┼───────────┐         │
        │           │           │         │
        ▼           ▼           ▼         ▼
      Native       WASM        FFI      UniFFI
        │           │           │         │
        ▼           ▼           ▼         ▼
     Desktop       Web        C apps    Android/iOS
                                              │
                                         Keyboards
```

**One NLP implementation.** Platform layers are wrappers only — no duplicate logic in Kotlin, Swift, or TypeScript.

| Crate | Purpose |
|-------|---------|
| `nlp-engine-ind-core` | Public API + NLP pipeline |
| `nlp-engine-ind-languages` | Lexicons + templates (embedded JSON) |
| `nlp-engine-ind-romanize` | Casual Roman normalization |
| `nlp-engine-ind-ffi` | C ABI |
| `nlp-engine-ind-uniffi` | Kotlin + Swift bindings |
| `nlp-engine-ind-wasm` | Browser + Node.js |
| `nlp-engine-ind-cli` | Command-line tool |

See [docs/architecture/overview.md](docs/architecture/overview.md) for details.

## Public API (Rust)

```rust
use nlp_engine_ind_core::{
    ConversionRequest, Converter, Engine, EngineConfig, Language, Style,
    CodeMixMode, RomanizationStyle,
};

let converter = Converter::new();

let request = ConversionRequest {
    text: "I'll finish this work and call you later".into(),
    language: Language::Kannada,
    style: Style::Casual,
    code_mixing: CodeMixMode::Natural,
    romanization: RomanizationStyle::CasualChat,
    ..Default::default()
};

let result = converter.convert(request)?;
println!("{}", result.output);
println!("confidence: {:.2}", result.confidence);
println!("latency: {}µs", result.processing_time_us());
```

### Keyboard API

```rust
let engine = Engine::with_language(Language::Kannada)?;
let result = engine.convert_text("I'll call you later")?;
// result.output, result.confidence, result.processing_time_us
```

## Local / offline behavior

The core never requires:

- Internet connectivity
- API keys or cloud services
- HTTP or network I/O
- Logging or persisting user text

Local mode is genuinely local. Optional external AI belongs in application layers, not this engine.

## Privacy

- No text is sent anywhere
- No analytics or telemetry in the core
- No disk persistence of chat content
- Keyboard samples run conversion entirely on-device

## Platform integration

| Platform | Status | Path |
|----------|--------|------|
| Rust / CLI | ✅ Built & tested | `crates/nlp-engine-ind-cli` |
| C FFI | ✅ Built & tested | `crates/nlp-engine-ind-ffi` |
| UniFFI (Kotlin/Swift) | ✅ Bindings generated | `bindings/uniffi/generated/` |
| Android library | ✅ arm64-v8a `.so` built | `bindings/android/nlp-engine-ind` |
| Android keyboard | ✅ Sample IME | `integrations/android-keyboard` |
| iOS XCFramework | ✅ Built | `release/ios/NlpEngineInd.xcframework` |
| iOS keyboard | ✅ Sample extension | `integrations/ios-keyboard` |
| Web / WASM | ✅ Built | `integrations/web-example` |
| Node.js | ✅ WASM wrapper | `integrations/node-example` |
| Desktop | ✅ CLI examples | `integrations/macos-example` etc. |

### Android keyboard

```bash
./scripts/build-android.sh
# Open integrations/android-keyboard in Android Studio
```

Tap **Convert ✨** to convert composing/selected text offline via the Rust engine.

### iOS keyboard

```bash
./scripts/build-ios.sh
# Generate Xcode project (requires xcodegen: brew install xcodegen)
cd integrations/ios-keyboard && xcodegen generate
open NlpEngineIndKeyboard.xcodeproj
```

### Web

```bash
./scripts/build-wasm.sh
cd integrations/web-example && python3 -m http.server 8080
# Open http://localhost:8080
```

### Desktop

```bash
./integrations/macos-example/run.sh
cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"
```

## Quick setup (macOS)

```bash
./scripts/setup-macos.sh
source "$HOME/.cargo/env"
./scripts/build-native.sh
```

See [docs/development/macos.md](docs/development/macos.md) for full setup including Android NDK and Xcode.

## Build commands

```bash
make build          # cargo build --workspace --release
make test           # cargo test --workspace --all-features
make lint           # cargo clippy with -D warnings
make fmt            # cargo fmt --all --check

./scripts/build-native.sh   # Rust + UniFFI bindings
./scripts/build-wasm.sh     # WASM package
./scripts/build-android.sh  # Android .so (requires ANDROID_HOME + NDK)
./scripts/build-ios.sh      # iOS XCFramework (requires Xcode)
./scripts/test-all.sh       # All of the above where possible
```

## Test

```bash
cargo test --workspace --all-features
```

**38 tests** across core, FFI, UniFFI, WASM, languages, and golden fixtures.

## Benchmarks (macOS, release)

| Sentence | p50 latency |
|----------|-------------|
| `I'll call you later` | ~13 µs |
| `Are you coming?` | ~15 µs |
| `Can you send me the file?` | ~26 µs |
| Complex work sentence | ~30 µs |

```bash
cargo bench -p nlp-engine-ind-core --bench pipeline
```

Mobile targets are sub-millisecond on host benchmarks; validate on device separately.

## Future ML architecture

The analyzer is designed for future extension:

```text
Analyzer
 ├── RuleBasedAnalyzer   (current)
 ├── ModelAnalyzer       (future tiny local model)
 └── HybridAnalyzer      (future)
```

No large ML model is included today — the engine stays small for keyboard use.

## License

MIT — see [LICENSE](LICENSE).
