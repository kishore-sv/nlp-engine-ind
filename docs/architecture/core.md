# Core engine

The core crate (`nlp-engine-ind-core`) exposes the stable public API.

## Main types

```rust
Converter          // Stateless default converter
Engine             // Configured, reusable engine instance
KeyboardConverter  // Simplified keyboard API
ConversionRequest  // Full conversion input
ConversionResult   // output, confidence, processing_time_us, diagnostics
EngineConfig       // Language, style, code-mix, enabled languages
```

## Pipeline

The `Pipeline` struct runs all stages in order. It is internal but accessible for testing via `Converter.pipeline`.

## Configuration

```rust
EngineConfig {
    language: Language::Kannada,
    style: Style::Casual,
    code_mixing: CodeMixMode::Natural,
    romanization: RomanizationStyle::CasualChat,
    enabled_languages: vec![Language::Kannada],  // language pack
    confidence_threshold: 0.5,
    strictness: true,
}
```

Use `EngineConfig::for_language(Language::Kannada)` to load a single language pack.

## Error handling

All errors are `EngineError` variants — no panics across public API boundaries. FFI and UniFFI layers map these to safe return codes / thrown exceptions.

## Performance targets

Designed for keyboard use:

| Input size | Target |
|------------|--------|
| Simple sentence | < 10 ms |
| Normal chat | < 50 ms |
| Complex chat | < 200 ms |

Host benchmarks show ~15–30 µs per conversion in release mode.

## Analyzer abstraction (future ML)

```rust
pub trait Analyzer {
    fn analyze(&self, tokens: &[Token]) -> Analysis;
}

pub struct RuleBasedAnalyzer;  // current default
// ModelAnalyzer, HybridAnalyzer — future
```

No ML model is bundled today.
