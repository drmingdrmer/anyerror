all: test lint fmt

test: lint fmt
	cargo test
	cargo test --features anyhow
	cargo test --features backtrace
	cargo test --features backtrace,anyhow
	cargo test --features rkyv
	cargo +stable test --features anyhow

fmt:
	cargo fmt

lint:
	cargo fmt
	cargo clippy --all-targets -- -D warnings
	RUSTFLAGS=-Wunused-crate-dependencies cargo clippy --lib -- -D warnings

clean:
	cargo clean

.PHONY: test fmt lint clean
