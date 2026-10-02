# Desktop integration

The Rust core compiles natively on macOS, Linux, and Windows.

## CLI (all platforms)

```bash
cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"
cargo run -p nlp-engine-ind-cli -- --language hi --json "Bro send the file"
cargo run -p nlp-engine-ind-cli -- interactive
```

## C FFI

Header: `bindings/ffi/nlp_engine_ind.h`

```c
NlpEngineIndHandle engine = nlp_engine_ind_create("kn");
NlpEngineIndResult* result = nlp_engine_ind_convert(engine, "I'll call you later");
nlp_engine_ind_free_result(result);
nlp_engine_ind_destroy(engine);
```

## Platform examples

| Platform | Script |
|----------|--------|
| macOS | `./integrations/macos-example/run.sh` |
| Linux | `./integrations/linux-example/run.sh` |
| Windows | `.\integrations\windows-example\run.ps1` |

All examples call the same Rust CLI — identical logical output across platforms.

## Rust library

```rust
use nlp_engine_ind_core::{Converter, Language};

let converter = Converter::new();
let result = converter.convert_text("I'll call you later", Language::Kannada)?;
```

## Building for other targets

```bash
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
```

See [building.md](../development/building.md) for cross-compilation details.
