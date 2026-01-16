BIN := gogolf-cli
all: build-local

build-local:
	cargo fmt
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	cargo build
	./target/debug/gogolf-cli

build-ci:
	cargo fmt
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	cargo build -- release

run:
	cargo run

test:
	cargo test

lint: 
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

clean: 
	cargo clean

.PHONY: all build run test lint fmt clean 