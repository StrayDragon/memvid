# Fork Notes

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
