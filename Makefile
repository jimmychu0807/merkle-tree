build-check: build check

build:
	cargo build

run-example:
	cargo run --example main

test:
	cargo test

clean:
	cargo clean

check:
	cargo fmt
	cargo clippy