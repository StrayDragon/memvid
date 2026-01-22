# Fork Notes

## Rebase: upstream/main (7851bf2..ee1bebd)
Upstream summary:
- Added SIMD acceleration for vector distance calculations (`simd` feature + `simd_benchmark`).
- Added Japanese README translation and linked it from the main README.
- Search parser now uses implicit AND for multi-word queries to improve precision.

Fork summary (7851bf2..7755eb6):
- Added memvid MCP server crate, workspace wiring, and README.
- Fixed MCP `memvid_put` input schema.
- Added OpenSpec scaffolding.
- Chinese tokenizer experiments/investigation and Tantivy search adjustments.
- QA/clippy/fmt/doc/test cleanups and example tweaks.
- Ignored nvim log noise in `.gitignore`; updated fork notes.

Conflict resolution:
- `Cargo.toml`: kept upstream `search_precision_benchmark` + `criterion` while retaining the `crates/memvid-mcp` workspace membership.

## Rebase: upstream/main (7851bf2..428f1c4)
Upstream summary:
- Added SIMD acceleration for vector distance calculations, including the `simd` feature, new module, and `simd_benchmark` example.
- Added Japanese README translation and linked it from the main README.

Fork summary (7851bf2..970e318):
- Added memvid MCP server crate, workspace wiring, and README.
- Fixed MCP `memvid_put` input schema.
- Added OpenSpec scaffolding.
- Chinese tokenizer experiments/investigation and Tantivy search adjustments.
- QA/clippy/fmt/doc updates and example tweaks.
- Ignored nvim log noise in `.gitignore`; updated fork notes.

Conflict resolution:
- `Cargo.toml`: kept upstream `simd_benchmark` + `simd` feature while retaining the `crates/memvid-mcp` workspace membership.

## Rebase: upstream/main (8b9cd22..7851bf2)
Upstream summary:
- Hardened replay input/output handling with size limits, sanitization, and error sentinels for large payloads (V-002 fix).
- Added doctor recovery tests for dry-run planning, bounds checking, vacuum ordering, and footer offset invariants.
- Disabled Dependabot config.

Fork summary (8b9cd22..a3b2c42):
- Added memvid MCP server crate, workspace setup, and schema fix.
- Added OpenSpec scaffolding and commands.
- Chinese tokenizer experiments + example; Tantivy search adjustments.
- QA/clippy/fmt/doc/test cleanups and example tweaks.
- Ignore nvim log noise in .gitignore.

Conflict resolution:
- None (rebase applied cleanly).

## Rebase: upstream/main (8ad4126..8b9cd22)
Upstream summary:
- Bumped crate version to 2.0.134 and adjusted optional deps (ndarray 0.16, rubato 0.15, rand 0.8).
- Lex search now deduplicates matches by frame_id; added coverage for dedupe behavior.
- Parallel segment indexing prefers search_text when available (no_raw support).
- Implicit multi-word queries now default to OR for better recall.
- Ignored legacy encryption capsule tests due to missing fixture.

Fork summary (8ad4126..450a205):
- Added memvid MCP server crate, workspace setup, and schema fix.
- Added OpenSpec scaffolding and commands.
- Chinese tokenizer experiments + example; Tantivy search adjustments.
- QA/clippy/fmt/doc/test cleanups and example tweaks.
- Ignore nvim log noise in .gitignore.

Conflict resolution:
- None (rebase applied cleanly).

## Rebase: upstream/main (0d469204..8ad4126)
Upstream summary:
- Added install script.
- Updated dependencies (Cargo.toml).
- Stabilized CI across Ubuntu/macOS/Windows.
- README updates and new translations (nl, cs, ko).
- Fixed dependabot cooldown configuration.
- Added example `text_embed_cache_bench` for `vec`.

Fork summary (0d469204..74d6582):
- Added memvid MCP crate and workspace support; MCP server schema fix.
- QA/test/doc updates.
- Chinese tokenizer experiments and investigations.
- Clippy and fmt fixes.
- OpenSpec initialization.

Conflict resolution:
- `Cargo.toml`: kept upstream dependency bumps + new example, retained workspace members for `crates/memvid-mcp`.
- `Cargo.toml`: kept upstream `tantivy` 0.25.0 and retained `tantivy-jieba` for the Chinese tokenizer.
- `Cargo.toml`: kept a single workspace section at the top (removed duplicate at bottom).
- `src/search/tantivy/schema.rs`: instantiated `JiebaTokenizer` with `new()` and kept ordinal position mode to match updated crate API.
