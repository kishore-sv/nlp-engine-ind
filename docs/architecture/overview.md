# Architecture overview

`nlp-engine-ind` is a single Rust NLP engine exposed to multiple platforms through thin binding layers.

## Design principles

1. **One implementation** — all platforms call the same Rust core
2. **Offline first** — no network, no cloud APIs in core
3. **Mobile first** — fast init, thread-safe, minimal allocations
4. **Language packs** — compile-time feature flags per language
5. **Keyboard ready** — high-level `convert_text` API with confidence scores

## Layer diagram

```text
Application (keyboard, web app, CLI)
        │
        ▼
Platform binding (UniFFI / WASM / FFI)
        │
        ▼
nlp-engine-ind-core (Converter, Engine, Pipeline)
        │
   ┌────┴────┬──────────────┐
   ▼         ▼              ▼
languages  romanize    rule-based analysis
(JSON)     (spelling)  (intent, tense, grammar)
```

## Crates

| Crate | Role |
|-------|------|
| `nlp-engine-ind-core` | Pipeline orchestration, public API |
| `nlp-engine-ind-languages` | Per-language lexicons and templates |
| `nlp-engine-ind-romanize` | Casual chat Roman normalization |
| `nlp-engine-ind-ffi` | Stable C ABI |
| `nlp-engine-ind-uniffi` | Mobile Kotlin/Swift bindings |
| `nlp-engine-ind-wasm` | Browser and Node.js |
| `nlp-engine-ind-cli` | Developer CLI |

## Data flow

1. **Normalize** — expand contractions, clean Unicode
2. **Tokenize** — words, URLs, mentions, emoji
3. **Protect** — preserve brands, links, handles
4. **Analyze** — intent, tense, grammar slots (rule-based)
5. **Code-mix** — decide English vs local words
6. **Generate** — language templates + fallback
7. **Romanize** — casual spelling
8. **Score** — confidence for downstream routing

## Thread safety

`Engine` and `Converter` are safe for concurrent use. Internal state is immutable shared dictionaries; per-request work uses stack allocations.

## Extension points

- `Analyzer` trait for future ML models
- `EngineConfig.enabled_languages` for language packs
- `ConversionContext` for keyboard hints (selected text, previous sentence)

See also:

- [core.md](core.md)
- [code-mixing.md](code-mixing.md)
- [language-system.md](language-system.md)
