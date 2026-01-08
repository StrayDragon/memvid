# Repository Guidelines

## Project Structure & Module Organization
`src/` contains the Rust library; key entry points include `src/lib.rs`, core logic in `src/memvid/`, I/O and file format handling in `src/io/`, and feature modules like `src/lex.rs`, `src/vec.rs`, and `src/whisper.rs`. Integration tests live in `tests/` (e.g., `tests/lifecycle.rs`), examples in `examples/`, and Docker tooling in `docker/cli/`. Specs and contributor context are in `MV2_SPEC.md` and `CLAUDE.md`.

## Build, Test, and Development Commands
- `make build`: debug build with default features (`lex,pdf_extract`).
- `make build-release`: optimized build.
- `make test` or `cargo test --features lex,pdf_extract`: run the full test suite.
- `make test-integration`: run integration tests in `tests/`.
- `make fmt` / `make clippy`: format and lint (clippy runs with `-D warnings`).
- `just qa`: run local QA mirroring Linux CI (build/test/fmt-check/clippy).
- `cargo run --example basic_usage` or `make run-example-basic`: run examples.

## Coding Style & Naming Conventions
Use `rustfmt` and `clippy` (`make fmt`, `make clippy`). Follow standard Rust naming: `snake_case` for modules/functions, `PascalCase` for types, `SCREAMING_SNAKE_CASE` for constants. Public APIs should be explicit in types, documented with `///` comments, and errors should use `thiserror`. Logging uses `tracing`.

## Testing Guidelines
Unit tests live beside code in `#[cfg(test)]` modules; integration tests are in `tests/*.rs` (snake_case filenames). Prefer testing edge cases and crash-safety paths. Run doc tests with `make test-doc` when editing public docs or examples. No explicit coverage target is enforced.

## Commit & Pull Request Guidelines
Recent history shows concise, imperative messages like “Update README.md,” with occasional Conventional Commit prefixes (e.g., `fix:`). Keep messages short and scoped. Use branch names like `feature/...` or `fix/...`. PRs should include a clear description, tests run, and any feature flags involved; link related issues when applicable and complete the PR template if present. Add docs/examples for public API changes; screenshots only if user-facing output changes.

## Security & Design Constraints
Do not file public issues for vulnerabilities; email `security@memvid.com` (see `SECURITY.md`). The `.mv2` format is single-file and append-only: avoid sidecar files, route writes through the WAL, and keep the library synchronous.
