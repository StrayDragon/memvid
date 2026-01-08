# memvid-mcp

An MCP (Model Context Protocol) server for Memvid. It exposes a small, structured tool surface for CRUD-style workflows against `.mv2` files via stdio transport.

## Run

```bash
cargo run -p memvid-mcp --bin memvid-mcp
```

The server speaks MCP over stdio, so it is meant to be launched by an MCP client (Claude Code, Cursor, etc.).

## Design preset (LLM-focused)

Memvid MCP intentionally exposes a compact tool surface that covers the core memory workflow for agents:

- Ingest new memory (text or bytes).
- Retrieve by search and read full payload.
- Update or delete stale facts.
- List a lightweight timeline when you need to scan or pick a frame.

Maintenance-only operations (stats/verify) are excluded to keep the toolset minimal and reduce accidental heavy calls.

## Tools

- `memvid_create` - Create a new `.mv2` file (optionally overwrite).
- `memvid_put` - Store content (text or base64) and optional mime (returns WAL sequence).
- `memvid_get_frame` - Read frame metadata, optionally payload (base64).
- `memvid_update_frame` - Update a frame by id (payload and/or metadata).
- `memvid_delete_frame` - Tombstone a frame by id (delete).
- `memvid_search` - Lexical search with snippets.
- `memvid_timeline` - Chronological scan of frames (lightweight list, cursor pagination).

## CRUD flow example

Create:
```json
{
  "name": "memvid_create",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "overwrite": true
  }
}
```

Create a document:
```json
{
  "name": "memvid_put",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "content": {
      "kind": "text",
      "text": "hello memvid",
      "mime": "text/plain"
    },
    "commit": true
  }
}
```

Resolve a frame id via timeline or search:
```json
{
  "name": "memvid_timeline",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "limit": 10
  }
}
```

If you set `MEMVID_DEFAULT_PATH`, you can omit `path` entirely:
```json
{
  "name": "memvid_timeline",
  "arguments": {
    "limit": 10
  }
}
```

Read by id (with payload):
```json
{
  "name": "memvid_get_frame",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "frame_id": 0,
    "include_payload_base64": true
  }
}
```

Update by id:
```json
{
  "name": "memvid_update_frame",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "frame_id": 0,
    "text": "updated text",
    "commit": true
  }
}
```

Delete by id:
```json
{
  "name": "memvid_delete_frame",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "frame_id": 0,
    "commit": true
  }
}
```

Search:
```json
{
  "name": "memvid_search",
  "arguments": {
    "path": "/tmp/demo.mv2",
    "query": "updated",
    "top_k": 5
  }
}
```

## Notes

- `memvid_get_frame` requires exactly one of `frame_id` or `uri`.
- `memvid_put` responses include a WAL sequence number, not a frame id. Use search, timeline, or a known `uri` to resolve the frame id.
- `memvid_timeline` defaults to `limit=100` and returns `next_cursor` for paging; pass `cursor` to fetch the next page.
- If `path` is omitted or empty, the server uses `MEMVID_DEFAULT_PATH` from the environment.
- `data_base64` fields use standard base64 encoding.
- `commit` defaults to true. If you disable it for batching, finish with a write call that commits.
- Lexical search requires the default `lex` feature on `memvid-core` (enabled by default in this workspace).

## Tests

```bash
cargo test -p memvid-mcp
```

## Client configuration examples

Build the binary first (recommended for production usage):

```bash
cargo build -p memvid-mcp
```

Replace `/ABS/PATH/TO/memvid` with your repo path.

### Claude Code (JSON)

```json
{
  "mcpServers": {
    "memvid": {
      "command": "/ABS/PATH/TO/memvid/target/debug/memvid-mcp",
      "env": {
        "MEMVID_DEFAULT_PATH": "/ABS/PATH/TO/memory.mv2"
      }
    }
  }
}
```

### Codex (`~/.codex/config.toml`)

```toml
[mcp_servers.memvid]
command = "/ABS/PATH/TO/memvid/target/debug/memvid-mcp"
env = { MEMVID_DEFAULT_PATH = "/ABS/PATH/TO/memory.mv2" }
```

### Cursor (`.cursor/mcp.json`)

```json
{
  "mcpServers": {
    "memvid": {
      "command": "/ABS/PATH/TO/memvid/target/debug/memvid-mcp",
      "env": {
        "MEMVID_DEFAULT_PATH": "/ABS/PATH/TO/memory.mv2"
      }
    }
  }
}
```
