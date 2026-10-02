# Language system

Languages are independent compile-time packs backed by embedded JSON lexicons.

## Supported languages

| Language | Code | Data path |
|----------|------|-----------|
| Kannada | `kn` | `data/kannada/` |
| Hindi | `hi` | `data/hindi/` |
| Telugu | `te` | `data/telugu/` |
| Tamil | `ta` | `data/tamil/` |
| Malayalam | `ml` | `data/malayalam/` |

## Feature flags

In `Cargo.toml`:

```toml
[features]
default = ["kannada", "hindi", "telugu", "tamil", "malayalam"]
kannada = []
hindi = []
# ...
```

Build with only Kannada:

```bash
cargo build -p nlp-engine-ind-core --no-default-features --features kannada
```

## Runtime language packs

```rust
EngineConfig {
    enabled_languages: vec![Language::Kannada, Language::Hindi],
    language: Language::Kannada,
    ..Default::default()
}
```

An empty `enabled_languages` list means all compiled languages are available.

## Mobile embedding

For Android/iOS, ship only required language `.so`/`.a` artifacts by building with selective features. Language JSON is embedded at compile time — no runtime file I/O.

## Lexicon structure

Each language directory contains:

- `lexicon.json` — word mappings and grammar slots
- Templates referenced by the generator module

See `crates/nlp-engine-ind-languages/src/` for per-language generators.
