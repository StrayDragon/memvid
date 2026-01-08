use std::collections::BTreeMap;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use memvid_core::{
    CanonicalEncoding, DocMetadata, Frame, FrameRole, FrameStatus, Memvid, PutOptions,
    SearchEngineKind, SearchRequest, TimelineQuery,
};
use rmcp::{
    Json, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, JsonSchema)]
struct CreateRequest {
    path: Option<String>,
    overwrite: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct CreateResponse {
    path: String,
    version: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PutOptionsInput {
    timestamp: Option<i64>,
    track: Option<String>,
    kind: Option<String>,
    uri: Option<String>,
    title: Option<String>,
    search_text: Option<String>,
    tags: Option<Vec<String>>,
    labels: Option<Vec<String>>,
    extra_metadata: Option<BTreeMap<String, String>>,
    enable_embedding: Option<bool>,
    auto_tag: Option<bool>,
    extract_dates: Option<bool>,
    extract_triplets: Option<bool>,
    no_raw: Option<bool>,
    source_path: Option<String>,
    dedup: Option<bool>,
    instant_index: Option<bool>,
    extraction_budget_ms: Option<u64>,
}

impl Default for PutOptionsInput {
    fn default() -> Self {
        Self {
            timestamp: None,
            track: None,
            kind: None,
            uri: None,
            title: None,
            search_text: None,
            tags: None,
            labels: None,
            extra_metadata: None,
            enable_embedding: None,
            auto_tag: None,
            extract_dates: None,
            extract_triplets: None,
            no_raw: None,
            source_path: None,
            dedup: None,
            instant_index: None,
            extraction_budget_ms: None,
        }
    }
}

impl PutOptionsInput {
    fn into_put_options(self) -> PutOptions {
        let mut options = PutOptions::default();
        if let Some(timestamp) = self.timestamp {
            options.timestamp = Some(timestamp);
        }
        if let Some(track) = self.track {
            options.track = Some(track);
        }
        if let Some(kind) = self.kind {
            options.kind = Some(kind);
        }
        if let Some(uri) = self.uri {
            options.uri = Some(uri);
        }
        if let Some(title) = self.title {
            options.title = Some(title);
        }
        if let Some(search_text) = self.search_text {
            options.search_text = Some(search_text);
        }
        if let Some(tags) = self.tags {
            options.tags = tags;
        }
        if let Some(labels) = self.labels {
            options.labels = labels;
        }
        if let Some(extra_metadata) = self.extra_metadata {
            options.extra_metadata = extra_metadata;
        }
        if let Some(enable_embedding) = self.enable_embedding {
            options.enable_embedding = enable_embedding;
        }
        if let Some(auto_tag) = self.auto_tag {
            options.auto_tag = auto_tag;
        }
        if let Some(extract_dates) = self.extract_dates {
            options.extract_dates = extract_dates;
        }
        if let Some(extract_triplets) = self.extract_triplets {
            options.extract_triplets = extract_triplets;
        }
        if let Some(no_raw) = self.no_raw {
            options.no_raw = no_raw;
        }
        if let Some(source_path) = self.source_path {
            options.source_path = Some(source_path);
        }
        if let Some(dedup) = self.dedup {
            options.dedup = dedup;
        }
        if let Some(instant_index) = self.instant_index {
            options.instant_index = instant_index;
        }
        if let Some(extraction_budget_ms) = self.extraction_budget_ms {
            options.extraction_budget_ms = extraction_budget_ms;
        }
        options
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum PutContentKind {
    Text,
    Base64,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PutContent {
    kind: PutContentKind,
    text: Option<String>,
    data_base64: Option<String>,
    mime: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct PutRequest {
    path: Option<String>,
    content: PutContent,
    options: Option<PutOptionsInput>,
    commit: Option<bool>,
    create_if_missing: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct PutResponse {
    seq_no: u64,
    committed: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetFrameRequest {
    path: Option<String>,
    frame_id: Option<u64>,
    uri: Option<String>,
    include_payload_base64: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct FrameOutput {
    frame_id: u64,
    timestamp: i64,
    kind: Option<String>,
    track: Option<String>,
    uri: Option<String>,
    title: Option<String>,
    role: String,
    status: String,
    canonical_encoding: String,
    canonical_length: Option<u64>,
    payload_length: u64,
    checksum_hex: String,
    tags: Vec<String>,
    labels: Vec<String>,
    extra_metadata: BTreeMap<String, String>,
    content_dates: Vec<String>,
    parent_id: Option<u64>,
    chunk_index: Option<u32>,
    chunk_count: Option<u32>,
    search_text: Option<String>,
    metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct GetFrameResponse {
    frame: FrameOutput,
    payload_base64: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct UpdateFrameRequest {
    path: Option<String>,
    frame_id: u64,
    text: Option<String>,
    data_base64: Option<String>,
    options: Option<PutOptionsInput>,
    commit: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct UpdateFrameResponse {
    frame_id: u64,
    seq_no: u64,
    committed: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct DeleteFrameRequest {
    path: Option<String>,
    frame_id: u64,
    commit: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct DeleteFrameResponse {
    frame_id: u64,
    seq_no: u64,
    committed: bool,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct SearchRequestInput {
    path: Option<String>,
    query: String,
    top_k: Option<usize>,
    snippet_chars: Option<usize>,
    uri: Option<String>,
    scope: Option<String>,
    cursor: Option<String>,
    no_sketch: Option<bool>,
    as_of_frame: Option<u64>,
    as_of_ts: Option<i64>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct SearchParamsOutput {
    top_k: usize,
    snippet_chars: usize,
    cursor: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct SearchHitEntityOutput {
    name: String,
    kind: String,
    confidence: Option<f32>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct SearchHitMetadataOutput {
    matches: usize,
    tags: Vec<String>,
    labels: Vec<String>,
    track: Option<String>,
    created_at: Option<String>,
    content_dates: Vec<String>,
    entities: Vec<SearchHitEntityOutput>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct SearchHitOutput {
    rank: usize,
    frame_id: u64,
    uri: String,
    title: Option<String>,
    range: (usize, usize),
    text: String,
    matches: usize,
    chunk_range: Option<(usize, usize)>,
    chunk_text: Option<String>,
    score: Option<f32>,
    metadata: Option<SearchHitMetadataOutput>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct SearchResponseOutput {
    query: String,
    elapsed_ms: u64,
    total_hits: usize,
    params: SearchParamsOutput,
    hits: Vec<SearchHitOutput>,
    context: String,
    next_cursor: Option<String>,
    engine: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TimelineRequest {
    path: Option<String>,
    limit: Option<u64>,
    since: Option<i64>,
    until: Option<i64>,
    reverse: Option<bool>,
    cursor: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct TimelineEntryOutput {
    frame_id: u64,
    timestamp: i64,
    preview: String,
    uri: Option<String>,
    child_frames: Vec<u64>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct TimelineResponse {
    entries: Vec<TimelineEntryOutput>,
    next_cursor: Option<String>,
}

const DEFAULT_TIMELINE_LIMIT: u64 = 100;
const MAX_TIMELINE_SCAN: u64 = 2_000;
const DEFAULT_PATH_ENV: &str = "MEMVID_DEFAULT_PATH";

#[derive(Debug, Clone)]
struct MemvidMcp {
    tool_router: ToolRouter<Self>,
}

impl MemvidMcp {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router(router = tool_router)]
impl MemvidMcp {
    #[tool(
        name = "memvid_create",
        description = "Create a new .mv2 memory file. Set overwrite=true to truncate an existing file."
    )]
    async fn memvid_create(
        &self,
        params: Parameters<CreateRequest>,
    ) -> Result<Json<CreateResponse>, String> {
        let request = params.0;
        run_blocking(move || {
            let path = resolve_path(request.path)?;
            let overwrite = request.overwrite.unwrap_or(false);
            if path.exists() && !overwrite {
                return Err(format!(
                    "memvid file already exists at {}",
                    path.display()
                ));
            }
            Memvid::create(&path).map_err(|err| err.to_string())?;
            Ok(CreateResponse {
                path: path.to_string_lossy().to_string(),
                version: memvid_core::MEMVID_CORE_VERSION.to_string(),
            })
        })
        .await
        .map(Json)
    }

    #[tool(
        name = "memvid_put",
        description = "Store content into a memory. Supports content.kind=\"text\" or \"base64\" with optional mime."
    )]
    async fn memvid_put(
        &self,
        params: Parameters<PutRequest>,
    ) -> Result<Json<PutResponse>, String> {
        let request = params.0;
        run_blocking(move || {
            let commit = request.commit.unwrap_or(true);
            let create_if_missing = request.create_if_missing.unwrap_or(false);
            let path = resolve_path(request.path)?;
            let mut mem = open_for_write(path.as_path(), create_if_missing)?;
            let mut options = request
                .options
                .map(PutOptionsInput::into_put_options)
                .unwrap_or_default();
            if let Some(mime) = request.content.mime {
                match options.metadata.as_mut() {
                    Some(metadata) => {
                        if metadata.mime.is_none() {
                            metadata.mime = Some(mime);
                        }
                    }
                    None => {
                        options.metadata = Some(DocMetadata {
                            mime: Some(mime),
                            ..DocMetadata::default()
                        });
                    }
                }
            }
            let payload = match request.content.kind {
                PutContentKind::Text => {
                    if request.content.data_base64.is_some() {
                        return Err("content.data_base64 must be empty when kind=text".to_string());
                    }
                    let text = request
                        .content
                        .text
                        .ok_or_else(|| "content.text is required when kind=text".to_string())?;
                    text.into_bytes()
                }
                PutContentKind::Base64 => {
                    if request.content.text.is_some() {
                        return Err("content.text must be empty when kind=base64".to_string());
                    }
                    let encoded = request.content.data_base64.ok_or_else(|| {
                        "content.data_base64 is required when kind=base64".to_string()
                    })?;
                    STANDARD
                        .decode(encoded.as_bytes())
                        .map_err(|err| format!("invalid base64 payload: {err}"))?
                }
            };
            let seq_no = mem
                .put_bytes_with_options(&payload, options)
                .map_err(|err| err.to_string())?;
            if commit {
                mem.commit().map_err(|err| err.to_string())?;
            }
            Ok(PutResponse {
                seq_no,
                committed: commit,
            })
        })
        .await
        .map(Json)
    }

    #[tool(
        name = "memvid_get_frame",
        description = "Fetch a frame by id or uri. Optionally include payload as base64."
    )]
    async fn memvid_get_frame(
        &self,
        params: Parameters<GetFrameRequest>,
    ) -> Result<Json<GetFrameResponse>, String> {
        let request = params.0;
        run_blocking(move || {
            let include_payload = request.include_payload_base64.unwrap_or(false);
            let has_frame_id = request.frame_id.is_some();
            let has_uri = request.uri.is_some();
            if has_frame_id == has_uri {
                return Err("provide exactly one of frame_id or uri".to_string());
            }
            let path = resolve_path(request.path)?;
            let mut mem = Memvid::open_read_only(path).map_err(|err| err.to_string())?;
            let frame = if let Some(frame_id) = request.frame_id {
                mem.frame_by_id(frame_id).map_err(|err| err.to_string())?
            } else {
                let uri = request.uri.unwrap_or_default();
                mem.frame_by_uri(&uri).map_err(|err| err.to_string())?
            };
            let payload_base64 = if include_payload {
                let payload = mem
                    .frame_canonical_payload(frame.id)
                    .map_err(|err| err.to_string())?;
                Some(STANDARD.encode(payload))
            } else {
                None
            };
            let frame_output = FrameOutput::try_from_frame(&frame)?;
            Ok(GetFrameResponse {
                frame: frame_output,
                payload_base64,
            })
        })
        .await
        .map(Json)
    }

    #[tool(
        name = "memvid_update_frame",
        description = "Update an existing frame payload/metadata by id."
    )]
    async fn memvid_update_frame(
        &self,
        params: Parameters<UpdateFrameRequest>,
    ) -> Result<Json<UpdateFrameResponse>, String> {
        let request = params.0;
        run_blocking(move || {
            let commit = request.commit.unwrap_or(true);
            let payload = match (request.text, request.data_base64) {
                (Some(_), Some(_)) => {
                    return Err("provide only one of text or data_base64".to_string())
                }
                (Some(text), None) => Some(text.into_bytes()),
                (None, Some(encoded)) => Some(
                    STANDARD
                        .decode(encoded.as_bytes())
                        .map_err(|err| format!("invalid base64 payload: {err}"))?,
                ),
                (None, None) => None,
            };
            let options = request
                .options
                .map(PutOptionsInput::into_put_options)
                .unwrap_or_default();
            let path = resolve_path(request.path)?;
            let mut mem = open_for_write(path.as_path(), false)?;
            let seq_no = mem
                .update_frame(request.frame_id, payload, options, None)
                .map_err(|err| err.to_string())?;
            if commit {
                mem.commit().map_err(|err| err.to_string())?;
            }
            Ok(UpdateFrameResponse {
                frame_id: request.frame_id,
                seq_no,
                committed: commit,
            })
        })
        .await
        .map(Json)
    }

    #[tool(
        name = "memvid_delete_frame",
        description = "Delete a frame by id (tombstone)."
    )]
    async fn memvid_delete_frame(
        &self,
        params: Parameters<DeleteFrameRequest>,
    ) -> Result<Json<DeleteFrameResponse>, String> {
        let request = params.0;
        run_blocking(move || {
            let commit = request.commit.unwrap_or(true);
            let path = resolve_path(request.path)?;
            let mut mem = open_for_write(path.as_path(), false)?;
            let seq_no = mem
                .delete_frame(request.frame_id)
                .map_err(|err| err.to_string())?;
            if commit {
                mem.commit().map_err(|err| err.to_string())?;
            }
            Ok(DeleteFrameResponse {
                frame_id: request.frame_id,
                seq_no,
                committed: commit,
            })
        })
        .await
        .map(Json)
    }

    #[tool(
        name = "memvid_search",
        description = "Search a memory using lexical query syntax. Returns ranked hits and context."
    )]
    async fn memvid_search(
        &self,
        params: Parameters<SearchRequestInput>,
    ) -> Result<Json<SearchResponseOutput>, String> {
        let request = params.0;
        run_blocking(move || {
            let top_k = request.top_k.unwrap_or(10);
            if top_k == 0 {
                return Err("top_k must be greater than 0".to_string());
            }
            let snippet_chars = request.snippet_chars.unwrap_or(200);
            let path = resolve_path(request.path)?;
            let search_request = SearchRequest {
                query: request.query,
                top_k,
                snippet_chars,
                uri: request.uri,
                scope: request.scope,
                cursor: request.cursor,
                as_of_frame: request.as_of_frame,
                as_of_ts: request.as_of_ts,
                no_sketch: request.no_sketch.unwrap_or(false),
                #[cfg(feature = "temporal_track")]
                temporal: None,
            };
            let mut mem = Memvid::open_read_only(path).map_err(|err| err.to_string())?;
            let response = mem.search(search_request).map_err(|err| err.to_string())?;
            Ok(SearchResponseOutput::from_response(response))
        })
        .await
        .map(Json)
    }

    #[tool(
        name = "memvid_timeline",
        description = "Scan a memory in chronological order and return timeline entries."
    )]
    async fn memvid_timeline(
        &self,
        params: Parameters<TimelineRequest>,
    ) -> Result<Json<TimelineResponse>, String> {
        let request = params.0;
        run_blocking(move || {
            let limit = request.limit.unwrap_or(DEFAULT_TIMELINE_LIMIT);
            let limit = NonZeroU64::new(limit)
                .ok_or_else(|| "limit must be greater than 0".to_string())?;
            let reverse = request.reverse.unwrap_or(false);
            let cursor = request
                .cursor
                .as_deref()
                .map(parse_timeline_cursor)
                .transpose()?;
            let path = resolve_path(request.path)?;
            let mut since = request.since;
            let mut until = request.until;
            if let Some((cursor_ts, _)) = cursor {
                if reverse {
                    if until.map_or(true, |value| value > cursor_ts) {
                        until = Some(cursor_ts);
                    }
                } else if since.map_or(true, |value| value < cursor_ts) {
                    since = Some(cursor_ts);
                }
            }

            let mut mem = Memvid::open_read_only(path).map_err(|err| err.to_string())?;
            let mut fetch_limit = limit.get().min(MAX_TIMELINE_SCAN);
            loop {
                let query = TimelineQuery {
                    limit: NonZeroU64::new(fetch_limit),
                    since,
                    until,
                    reverse,
                    #[cfg(feature = "temporal_track")]
                    temporal: None,
                };
                let entries = mem.timeline(query).map_err(|err| err.to_string())?;
                let entries_len = entries.len();
                let filtered = entries.into_iter().filter(|entry| match cursor {
                    None => true,
                    Some((cursor_ts, cursor_id)) => {
                        if reverse {
                            entry.timestamp < cursor_ts
                                || (entry.timestamp == cursor_ts && entry.frame_id < cursor_id)
                        } else {
                            entry.timestamp > cursor_ts
                                || (entry.timestamp == cursor_ts && entry.frame_id > cursor_id)
                        }
                    }
                });
                let mut outputs: Vec<TimelineEntryOutput> = filtered
                    .map(|entry| TimelineEntryOutput {
                        frame_id: entry.frame_id,
                        timestamp: entry.timestamp,
                        preview: entry.preview,
                        uri: entry.uri,
                        child_frames: entry.child_frames,
                    })
                    .collect();
                if outputs.len() >= limit.get() as usize
                    || entries_len < fetch_limit as usize
                    || fetch_limit >= MAX_TIMELINE_SCAN
                {
                    outputs.truncate(limit.get() as usize);
                    let next_cursor = if outputs.len() == limit.get() as usize {
                        outputs.last().map(|entry| timeline_cursor_from_entry(entry))
                    } else {
                        None
                    };
                    return Ok(TimelineResponse {
                        entries: outputs,
                        next_cursor,
                    });
                }
                fetch_limit = fetch_limit.saturating_mul(2).min(MAX_TIMELINE_SCAN);
            }
        })
        .await
        .map(Json)
    }

}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for MemvidMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Memvid MCP server exposes tools to create, ingest, search, and inspect .mv2 memories. If path is omitted, MEMVID_DEFAULT_PATH is used."
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

impl SearchResponseOutput {
    fn from_response(response: memvid_core::SearchResponse) -> Self {
        let hits = response
            .hits
            .into_iter()
            .map(|hit| SearchHitOutput {
                rank: hit.rank,
                frame_id: hit.frame_id,
                uri: hit.uri,
                title: hit.title,
                range: hit.range,
                text: hit.text,
                matches: hit.matches,
                chunk_range: hit.chunk_range,
                chunk_text: hit.chunk_text,
                score: hit.score,
                metadata: hit.metadata.map(SearchHitMetadataOutput::from_metadata),
            })
            .collect();
        let elapsed_ms = response.elapsed_ms.min(u128::from(u64::MAX)) as u64;
        let engine = search_engine_label(response.engine).to_string();
        SearchResponseOutput {
            query: response.query,
            elapsed_ms,
            total_hits: response.total_hits,
            params: SearchParamsOutput {
                top_k: response.params.top_k,
                snippet_chars: response.params.snippet_chars,
                cursor: response.params.cursor,
            },
            hits,
            context: response.context,
            next_cursor: response.next_cursor,
            engine,
        }
    }
}

impl SearchHitMetadataOutput {
    fn from_metadata(metadata: memvid_core::SearchHitMetadata) -> Self {
        let entities = metadata
            .entities
            .into_iter()
            .map(|entity| SearchHitEntityOutput {
                name: entity.name,
                kind: entity.kind,
                confidence: entity.confidence,
            })
            .collect();
        SearchHitMetadataOutput {
            matches: metadata.matches,
            tags: metadata.tags,
            labels: metadata.labels,
            track: metadata.track,
            created_at: metadata.created_at,
            content_dates: metadata.content_dates,
            entities,
        }
    }
}

impl FrameOutput {
    fn try_from_frame(frame: &Frame) -> Result<Self, String> {
        let metadata = match &frame.metadata {
            Some(metadata) => Some(
                serde_json::to_value(metadata)
                    .map_err(|err| format!("failed to serialize metadata: {err}"))?,
            ),
            None => None,
        };
        Ok(FrameOutput {
            frame_id: frame.id,
            timestamp: frame.timestamp,
            kind: frame.kind.clone(),
            track: frame.track.clone(),
            uri: frame.uri.clone(),
            title: frame.title.clone(),
            role: frame_role_label(frame.role).to_string(),
            status: frame_status_label(frame.status).to_string(),
            canonical_encoding: canonical_encoding_label(frame.canonical_encoding).to_string(),
            canonical_length: frame.canonical_length,
            payload_length: frame.payload_length,
            checksum_hex: hex::encode(frame.checksum),
            tags: frame.tags.clone(),
            labels: frame.labels.clone(),
            extra_metadata: frame.extra_metadata.clone(),
            content_dates: frame.content_dates.clone(),
            parent_id: frame.parent_id,
            chunk_index: frame.chunk_index,
            chunk_count: frame.chunk_count,
            search_text: frame.search_text.clone(),
            metadata,
        })
    }
}

fn search_engine_label(engine: SearchEngineKind) -> &'static str {
    match engine {
        SearchEngineKind::Tantivy => "tantivy",
        SearchEngineKind::LexFallback => "lex_fallback",
        SearchEngineKind::Hybrid => "hybrid",
    }
}

fn parse_timeline_cursor(cursor: &str) -> Result<(i64, u64), String> {
    let mut parts = cursor.splitn(2, ':');
    let ts = parts
        .next()
        .ok_or_else(|| "invalid cursor format".to_string())?
        .parse::<i64>()
        .map_err(|_| "invalid cursor timestamp".to_string())?;
    let frame_id = parts
        .next()
        .ok_or_else(|| "invalid cursor format".to_string())?
        .parse::<u64>()
        .map_err(|_| "invalid cursor frame_id".to_string())?;
    Ok((ts, frame_id))
}

fn timeline_cursor_from_entry(entry: &TimelineEntryOutput) -> String {
    format!("{}:{}", entry.timestamp, entry.frame_id)
}

fn resolve_path(path: Option<String>) -> Result<PathBuf, String> {
    if let Some(path) = path {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed));
        }
    }
    let fallback = std::env::var(DEFAULT_PATH_ENV)
        .map_err(|_| format!("path is required or set {DEFAULT_PATH_ENV}"))?;
    let trimmed = fallback.trim();
    if trimmed.is_empty() {
        return Err(format!("path is required or set {DEFAULT_PATH_ENV}"));
    }
    Ok(PathBuf::from(trimmed))
}

fn canonical_encoding_label(encoding: CanonicalEncoding) -> &'static str {
    match encoding {
        CanonicalEncoding::Plain => "plain",
        CanonicalEncoding::Zstd => "zstd",
    }
}

fn frame_role_label(role: FrameRole) -> &'static str {
    match role {
        FrameRole::Document => "document",
        FrameRole::DocumentChunk => "document_chunk",
        FrameRole::ExtractedImage => "extracted_image",
    }
}

fn frame_status_label(status: FrameStatus) -> &'static str {
    match status {
        FrameStatus::Active => "active",
        FrameStatus::Superseded => "superseded",
        FrameStatus::Deleted => "deleted",
    }
}

fn open_for_write(path: &Path, create_if_missing: bool) -> Result<Memvid, String> {
    if path.exists() {
        Memvid::open(path).map_err(|err| err.to_string())
    } else if create_if_missing {
        Memvid::create(path).map_err(|err| err.to_string())
    } else {
        Err(format!("memvid file not found at {}", path.display()))
    }
}

async fn run_blocking<R, F>(f: F) -> Result<R, String>
where
    R: Send + 'static,
    F: FnOnce() -> Result<R, String> + Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|err| format!("blocking task failed: {err}"))?
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};
    use tempfile::tempdir;
    use tokio::runtime::Builder;

    fn env_mutex() -> &'static Mutex<()> {
        static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_MUTEX.get_or_init(|| Mutex::new(()))
    }

    fn with_env_default_path<F, R>(value: Option<&str>, f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = env_mutex().lock().expect("env mutex");
        let previous = std::env::var(DEFAULT_PATH_ENV).ok();
        unsafe {
            match value {
                Some(path) => std::env::set_var(DEFAULT_PATH_ENV, path),
                None => std::env::remove_var(DEFAULT_PATH_ENV),
            }
        }
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        unsafe {
            match previous {
                Some(path) => std::env::set_var(DEFAULT_PATH_ENV, path),
                None => std::env::remove_var(DEFAULT_PATH_ENV),
            }
        }
        match result {
            Ok(value) => value,
            Err(payload) => std::panic::resume_unwind(payload),
        }
    }

    fn current_thread_runtime() -> tokio::runtime::Runtime {
        Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime")
    }

    #[tokio::test]
    async fn test_memvid_crud_tools() {
        let temp = tempdir().expect("tempdir");
        let path = temp.path().join("crud.mv2");
        let path_string = path.to_string_lossy().to_string();
        let server = MemvidMcp::new();

        let create = server
            .memvid_create(Parameters(CreateRequest {
                path: Some(path_string.clone()),
                overwrite: Some(true),
            }))
            .await
            .expect("create")
            .0;
        assert_eq!(create.path, path_string);

        let put = server
            .memvid_put(Parameters(PutRequest {
                path: Some(path_string.clone()),
                content: PutContent {
                    kind: PutContentKind::Text,
                    text: Some("hello world".to_string()),
                    data_base64: None,
                    mime: Some("text/plain".to_string()),
                },
                options: Some(PutOptionsInput {
                    uri: Some("mv2://tests/crud".to_string()),
                    ..PutOptionsInput::default()
                }),
                commit: Some(true),
                create_if_missing: Some(false),
            }))
            .await
            .expect("put")
            .0;
        assert!(put.seq_no > 0);
        let timeline = server
            .memvid_timeline(Parameters(TimelineRequest {
                path: Some(path_string.clone()),
                limit: Some(10),
                since: None,
                until: None,
                reverse: None,
                cursor: None,
            }))
            .await
            .expect("timeline")
            .0;
        assert!(!timeline.entries.is_empty());
        let frame_id = timeline.entries[0].frame_id;

        let get = server
            .memvid_get_frame(Parameters(GetFrameRequest {
                path: Some(path_string.clone()),
                frame_id: Some(frame_id),
                uri: None,
                include_payload_base64: Some(true),
            }))
            .await
            .expect("get")
            .0;
        assert_eq!(get.frame.status, "active");
        let payload = STANDARD
            .decode(get.payload_base64.expect("payload"))
            .expect("decode payload");
        assert_eq!(payload, b"hello world");

        let search = server
            .memvid_search(Parameters(SearchRequestInput {
                path: Some(path_string.clone()),
                query: "hello".to_string(),
                top_k: Some(5),
                snippet_chars: None,
                uri: None,
                scope: None,
                cursor: None,
                no_sketch: None,
                as_of_frame: None,
                as_of_ts: None,
            }))
            .await
            .expect("search")
            .0;
        assert!(!search.hits.is_empty());
        assert!(search.hits.iter().any(|hit| hit.frame_id == frame_id));

        let update = server
            .memvid_update_frame(Parameters(UpdateFrameRequest {
                path: Some(path_string.clone()),
                frame_id,
                text: Some("updated".to_string()),
                data_base64: None,
                options: None,
                commit: Some(true),
            }))
            .await
            .expect("update")
            .0;
        assert_eq!(update.frame_id, frame_id);

        let get_updated = server
            .memvid_get_frame(Parameters(GetFrameRequest {
                path: Some(path_string.clone()),
                frame_id: None,
                uri: Some("mv2://tests/crud".to_string()),
                include_payload_base64: Some(true),
            }))
            .await
            .expect("get updated")
            .0;
        let updated_frame_id = get_updated.frame.frame_id;
        let updated_payload = STANDARD
            .decode(get_updated.payload_base64.expect("updated payload"))
            .expect("decode updated payload");
        assert_eq!(updated_payload, b"updated");

        let delete = server
            .memvid_delete_frame(Parameters(DeleteFrameRequest {
                path: Some(path_string.clone()),
                frame_id: updated_frame_id,
                commit: Some(true),
            }))
            .await
            .expect("delete")
            .0;
        assert_eq!(delete.frame_id, updated_frame_id);

        let get_deleted = server
            .memvid_get_frame(Parameters(GetFrameRequest {
                path: Some(path_string.clone()),
                frame_id: Some(updated_frame_id),
                uri: None,
                include_payload_base64: Some(false),
            }))
            .await
            .expect("get deleted")
            .0;
        assert_eq!(get_deleted.frame.status, "deleted");
    }

    #[test]
    fn test_default_path_env_fallback() {
        let temp = tempdir().expect("tempdir");
        let path = temp.path().join("default-path.mv2");
        let path_string = path.to_string_lossy().to_string();
        let server = MemvidMcp::new();
        let rt = current_thread_runtime();

        with_env_default_path(Some(&path_string), || {
            rt.block_on(async {
                server
                    .memvid_create(Parameters(CreateRequest {
                        path: Some(path_string.clone()),
                        overwrite: Some(true),
                    }))
                    .await
                    .expect("create");
                server
                    .memvid_put(Parameters(PutRequest {
                        path: Some(path_string.clone()),
                        content: PutContent {
                            kind: PutContentKind::Text,
                            text: Some("env fallback".to_string()),
                            data_base64: None,
                            mime: None,
                        },
                        options: None,
                        commit: Some(true),
                        create_if_missing: Some(false),
                    }))
                    .await
                    .expect("put");

                let timeline = server
                    .memvid_timeline(Parameters(TimelineRequest {
                        path: None,
                        limit: Some(10),
                        since: None,
                        until: None,
                        reverse: None,
                        cursor: None,
                    }))
                    .await
                    .expect("timeline")
                    .0;
                assert!(!timeline.entries.is_empty());
            });
        });
    }

    #[test]
    fn test_missing_default_path_errors() {
        let server = MemvidMcp::new();
        let rt = current_thread_runtime();

        let result = with_env_default_path(None, || {
            rt.block_on(async {
                server
                    .memvid_timeline(Parameters(TimelineRequest {
                        path: None,
                        limit: Some(10),
                        since: None,
                        until: None,
                        reverse: None,
                        cursor: None,
                    }))
                    .await
            })
        });
        let err = match result {
            Ok(_) => panic!("missing path error"),
            Err(err) => err,
        };
        assert!(err.contains("path is required"));
    }

    #[tokio::test]
    async fn test_timeline_invalid_cursor() {
        let temp = tempdir().expect("tempdir");
        let path = temp.path().join("cursor.mv2");
        let path_string = path.to_string_lossy().to_string();
        let server = MemvidMcp::new();

        server
            .memvid_create(Parameters(CreateRequest {
                path: Some(path_string.clone()),
                overwrite: Some(true),
            }))
            .await
            .expect("create");

        let result = server
            .memvid_timeline(Parameters(TimelineRequest {
                path: Some(path_string),
                limit: Some(10),
                since: None,
                until: None,
                reverse: None,
                cursor: Some("bad-cursor".to_string()),
            }))
            .await;
        let err = match result {
            Ok(_) => panic!("invalid cursor"),
            Err(err) => err,
        };
        assert!(err.contains("invalid cursor"));
    }

    #[tokio::test]
    async fn test_put_invalid_base64() {
        let temp = tempdir().expect("tempdir");
        let path = temp.path().join("base64.mv2");
        let path_string = path.to_string_lossy().to_string();
        let server = MemvidMcp::new();

        server
            .memvid_create(Parameters(CreateRequest {
                path: Some(path_string.clone()),
                overwrite: Some(true),
            }))
            .await
            .expect("create");

        let result = server
            .memvid_put(Parameters(PutRequest {
                path: Some(path_string),
                content: PutContent {
                    kind: PutContentKind::Base64,
                    text: None,
                    data_base64: Some("not_base64??".to_string()),
                    mime: None,
                },
                options: None,
                commit: Some(true),
                create_if_missing: Some(false),
            }))
            .await;
        let err = match result {
            Ok(_) => panic!("invalid base64"),
            Err(err) => err,
        };
        assert!(err.contains("invalid base64 payload"));
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = MemvidMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
