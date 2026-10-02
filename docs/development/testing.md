# Testing

## Rust core

```bash
cargo test --workspace --all-features
```

**38 tests** including:

| Area | Tests |
|------|-------|
| Normalization, tokenization | unit tests in core |
| Analysis, generation | unit tests in core |
| Code mixing, confidence | unit tests in core |
| Golden conversations | `tests/integration.rs` |
| All languages | integration test |
| Concurrent (100 threads) | `lib.rs` |
| FFI create/convert/destroy | `nlp-engine-ind-ffi` |
| UniFFI engine | `nlp-engine-ind-uniffi` |
| WASM internal | `nlp-engine-ind-wasm` |

## Golden fixtures

`tests/fixtures/golden.json` — representative chat sentences with acceptable output alternatives.

## Format & lint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Full validation

```bash
./scripts/test-all.sh
```

Runs native, WASM, and platform builds where SDKs are available.

## Benchmarks

```bash
cargo bench -p nlp-engine-ind-core --bench pipeline
```

Reports p50/p95/p99 via Criterion. Host benchmarks only — validate mobile separately.

## Platform tests

| Platform | How to test |
|----------|-------------|
| Android | Build keyboard in Android Studio, convert offline |
| iOS | Run keyboard extension in Xcode simulator |
| Web | Serve `integrations/web-example`, click Convert |
| Desktop | Run CLI examples |

## Thread safety

Core test spawns 100 concurrent conversions against a shared `Engine` instance.
