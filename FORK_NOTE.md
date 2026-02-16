# Fork Notes

## Rebase: upstream/main (8d8addb..7be69c6)
Upstream summary:
- Released `v2.0.157` (`7be69c6`) and bumped `memvid-core` to `2.0.137`.
- Added structured XLSX extraction (table detection, OOXML metadata parsing, semantic chunking) and wired it into `XlsxReader` (`97f68b2`, `7511889`), with new reader modules and `tests/xlsx_structured.rs`.
- Follow-up fmt/lint fixes (`64bbd32`, `223b93d`) and made structured XLSX tests skip when the `arden.xlsx` fixture is absent (`7be69c6`, CI robustness).

Fork summary (8d8addb..ffbb3ae):
- Added fork-only MCP server crate (`crates/memvid-mcp`) for CRUD workflows against `.mv2` files via stdio transport.
- Added Chinese recall experiments: `examples/chinese_recall.rs`, Tantivy Jieba tokenizer wiring (`tantivy-jieba`), and query/search-path adjustments (e.g. disable sketch pre-filter for CJK).
- Added OpenSpec scaffolding and editor commands (`openspec/`, `.claude/commands/`, `.cursor/commands/`), plus ongoing QA/fmt/clippy maintenance.
- Optimized local build disk usage by disabling dependency debuginfo in dev/test profiles (`Cargo.toml`), and ignored `.nvimlog`.

Potential conflict hotspots identified before rebase:
- `Cargo.toml`: upstream version bump + fork workspace + `tantivy-jieba` / `lex` feature wiring + profile tweaks.
- `.gitignore`: upstream doc ignores vs fork `.nvimlog`.

Conflict resolution:
- None (rebase applied cleanly without manual conflict stops).
- Verified post-rebase merge intent on anticipated hotspots:
  - `Cargo.toml`: kept upstream `2.0.137` while retaining fork workspace + `tantivy-jieba` lex wiring + debug-size profiles.
  - `.gitignore`: includes both upstream `plan_model_robust.md` and fork `.nvimlog` ignore.

## Rebase: upstream/main (38cdd32..d7a2657)
Upstream summary:
- Released `v2.0.136` (`4e04ed8`) with frame-level ACL plumbing across search/ask/replay, plus robustness fixes and expanded test/example coverage.
- Follow-up lint fixes (`c84da0b`, `f807534`) adjusted ACL/mutation internals and added a missing `VecIndexManifest` model field.
- Added Simplified Chinese translation doc `docs/i18n/README.zh-CN.md` (`a40dbf3`, merged by `d7a2657`).

Fork summary (38cdd32..ebd4b8f):
- Added fork-only MCP server crate (`crates/memvid-mcp`) and workspace wiring in `Cargo.toml`, with MCP schema fix for `memvid_put`.
- Added OpenSpec scaffolding and local automation docs/commands (`openspec/`, `.claude/commands/`, `.cursor/commands/`).
- Added Chinese recall experiments: `examples/chinese_recall.rs`, Tantivy CJK tokenizer wiring (`tantivy-jieba`), and query path adjustments in `src/search/tantivy/*`.
- Continued maintenance-only commits: QA/fmt/clippy cleanups, post-rebase maintenance, and iterative `FORK_NOTE.md` updates.

Potential conflict hotspots identified before rebase:
- `Cargo.toml`: upstream version bump to `2.0.136` vs fork workspace + `tantivy-jieba` + `lex` feature wiring.
- `examples/basic_usage.rs` and `examples/pdf_ingestion.rs`: upstream ACL request fields vs fork clippy-allow annotations.
- `src/memvid/search/mod.rs`: upstream ACL enforcement in search path vs fork CJK sketch-bypass condition.
- `.gitignore`: upstream ignores `feature_100x_memvid.md` while fork ignores `.nvimlog`.

Conflict resolution:
- None (rebase applied cleanly without manual conflict stops).
- Verified post-rebase merge intent on anticipated hotspots:
  - `Cargo.toml`: kept upstream `2.0.136` while retaining fork workspace + `tantivy-jieba` lex wiring.
  - `examples/basic_usage.rs` / `examples/pdf_ingestion.rs`: retained fork clippy allow and upstream ACL request fields.
  - `src/memvid/search/mod.rs`: preserved both upstream ACL enforcement and fork CJK sketch bypass.
  - `.gitignore`: includes both upstream `feature_100x_memvid.md` and fork `.nvimlog` ignore.

## Rebase: upstream/main (3864ee8..38cdd32)
Upstream summary:
- Added a Memvid v1 deprecation warning to the README (with a docs link).
- Moved the deprecation warning to the bottom of the README and refreshed formatting.
- Minor README tweak.

Fork summary (3864ee8..87be2ed):
- No functional changes required for this upstream update (README-only).
- Fork still carries: memvid MCP server crate/workspace wiring + schema fix; OpenSpec scaffolding; Chinese tokenizer recall experiments + Tantivy adjustments; QA/clippy/fmt/doc/test maintenance.

Conflict resolution:
- None (rebase applied cleanly).

## Rebase: upstream/main (df8723a..3864ee8)
Upstream summary:
- Refreshed README header layout and spacing, replacing the language flags block with a Trendshift badge.
- Added a "Benchmark Highlights" section with accuracy/latency claims and benchmark context.
- Cleaned up README separators and minor formatting/spacing.

Fork summary (df8723a..e2ad7e6):
- Added memvid MCP server crate + workspace wiring and schema fix; updated docs.
- Added OpenSpec scaffolding/commands and project notes.
- Chinese tokenizer recall experiments and Tantivy search adjustments with example updates.
- QA/clippy/fmt/test cleanups and `.gitignore` noise ignore; updated fork notes.

Conflict resolution:
- None (rebase applied cleanly).

## Rebase check: upstream/main (df8723a..df8723a)
Upstream summary:
- No new upstream commits; `git fetch upstream` and `git rebase upstream/main` reported the branch is up to date.

Fork summary (df8723a..48b4d6b):
- Added memvid MCP server crate + workspace wiring and fixed MCP schema.
- Added OpenSpec scaffolding and Chinese tokenizer experiments with Tantivy adjustments + example.
- QA/clippy/fmt/doc/test cleanups, post-rebase maintenance, and `.gitignore` noise ignores; updated fork notes.
- Updated fork notes after rebase checks.

Conflict resolution:
- None (no rebase performed).

## Rebase: upstream/main (c26911b..df8723a)
Upstream summary:
- Released v2.0.135; committed Cargo.lock and updated CI cache keys for reproducible builds.
- Enforced vector index model consistency with strict binding checks and related fmt/clippy fixes.
- Fixed symspell_cleanup data corruption and added dictionary download tooling.
- Added macOS ONNX Runtime stderr suppression (new libc target dep) and guarded tantivy code behind `lex`.
- Added Windows Tantivy file-locking test delays.

Fork summary (c26911b..1e59794):
- Added memvid MCP server crate and schema fix; documentation updates.
- Added OpenSpec scaffolding and Chinese tokenizer recall experiments with Tantivy adjustments + example.
- QA/clippy/fmt cleanups and post-rebase maintenance; ignored nvim log noise; updated fork notes.

Conflict resolution:
- `Cargo.toml`: kept upstream version bump + macOS libc target deps + target-specific deps ordering; retained `tantivy-jieba` + lex feature wiring for Chinese tokenizer experiments.

## Rebase: upstream/main (37df42f..c26911b)
Upstream summary:
- Implemented HNSW vector search with fixed-point distance metric and benchmarks.
- Added API embedding providers (OpenAI) and the `api_embed` feature with reqwest.
- Added LRU eviction for extraction cache and related fixes.
- Clippy safety overhaul: reduced unwrap/expect usage and tightened linting.
- Added translation tooling and refreshed README translations (Arabic, Czech, Spanish, etc.).

Fork summary (37df42f..cc0193b):
- Added memvid MCP server crate + workspace wiring; fixed MCP schema; doc updates.
- Added OpenSpec scaffolding plus Chinese tokenizer experiments and example tweaks.
- QA/clippy/fmt cleanups, justfile additions, and build unblocks.
- Ignored local nvim log noise in `.gitignore`; updated fork notes.

Conflict resolution:
- `Cargo.toml`: kept upstream HNSW bench additions while retaining `crates/memvid-mcp` workspace (single top-level `[workspace]`).
- `src/lib.rs`: preserved upstream clippy lint configuration and allow list.
- `tests/*.rs`: kept fork clippy formatting cleanups (assert formatting, removed stale comments).

## Rebase: upstream/main (ee1bebd..37df42f)
Upstream summary:
- Added Whisper model quantization support with updated README guidance and `src/whisper.rs` handling.
- Fixed Whisper/rubato 1.0 compatibility in the audio pipeline.

Fork summary (ee1bebd..a9f55fe):
- Added memvid MCP server crate, workspace wiring, and README.
- Fixed MCP `memvid_put` input schema.
- Added OpenSpec scaffolding.
- Chinese tokenizer experiments/investigation and Tantivy search adjustments.
- QA/clippy/fmt/doc/test cleanups and example tweaks.
- Ignored nvim log noise in `.gitignore`; updated fork notes.

Conflict resolution:
- None (rebase applied cleanly).

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
- `Cargo.toml`: kept upstream `search_precision_benchmark` + `criterion` while retaining the `crates/memvid-mcp` workspace membership (single workspace section at top).

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
