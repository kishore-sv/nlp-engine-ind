.PHONY: build test lint fmt run check all

build:
	cargo build --workspace

test:
	cargo test --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

run:
	cargo run -p nlp-engine-ind-cli -- --language kn "I'll call you later"

check: fmt-check lint test build

all: check

bench:
	cargo bench --workspace
