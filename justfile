# Local QA tasks matching .github/workflows/ci.yml (Linux runners).
export CARGO_TERM_COLOR := "always"
export RUST_BACKTRACE := "1"

default:
	@just --list

qa: build-ci test-ci fmt-check clippy

build-ci:
	cargo build --verbose

test-ci:
	cargo test --verbose

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy -- -D warnings
