# Fork Notes

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
