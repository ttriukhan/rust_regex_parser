.PHONY: build fmt clippy test clean pre-commit

build:
	cargo build

fmt:
	cargo fmt

clippy:
	cargo clippy

test:
	cargo test

run:
	cargo run -- parse "regex.txt"

clean:
	cargo clean

pre-commit: fmt clippy test
	@echo "Pre-commit checks complete."