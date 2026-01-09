use std::path::PathBuf;

use memvid_core::{Memvid, PutOptions, Result, SearchRequest};

const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const DIM: &str = "\x1b[2m";

struct Case {
    query: &'static str,
    expected_titles: &'static [&'static str],
    expectation: Expectation,
    note: &'static str,
}

#[derive(Clone, Copy)]
enum Expectation {
    MustMatch,
    KnownHard,
    ExpectNone,
}

#[derive(Clone, Copy)]
enum Outcome {
    Ok,
    Miss,
    KnownMiss,
    UnexpectedHits,
}

impl Expectation {
    fn label(self) -> &'static str {
        match self {
            Expectation::MustMatch => "must_match",
            Expectation::KnownHard => "known_hard",
            Expectation::ExpectNone => "expect_none",
        }
    }
}

struct Doc {
    title: &'static str,
    uri: &'static str,
    text: &'static str,
    tags: &'static [&'static str],
    labels: &'static [&'static str],
    track: Option<&'static str>,
}

#[derive(Clone, Copy)]
enum QueryKind {
    CjkOnly,
    LatinOnly,
    Mixed,
    Other,
}

impl QueryKind {
    fn label(self) -> &'static str {
        match self {
            QueryKind::CjkOnly => "cjk_only",
            QueryKind::LatinOnly => "latin_only",
            QueryKind::Mixed => "mixed",
            QueryKind::Other => "other",
        }
    }
}

#[derive(Default)]
struct Summary {
    total: usize,
    ok: usize,
    miss: usize,
    known_miss: usize,
    unexpected_hits: usize,
    mixed_total: usize,
    mixed_ok: usize,
    cjk_total: usize,
    cjk_ok: usize,
    latin_total: usize,
    latin_ok: usize,
    other_total: usize,
    other_ok: usize,
    elapsed_total_ms: u128,
    mixed_elapsed_ms: u128,
    cjk_elapsed_ms: u128,
    latin_elapsed_ms: u128,
    other_elapsed_ms: u128,
}

impl Summary {
    fn record(&mut self, outcome: Outcome, kind: QueryKind, elapsed_ms: u128) {
        self.total += 1;
        match outcome {
            Outcome::Ok => self.ok += 1,
            Outcome::Miss => self.miss += 1,
            Outcome::KnownMiss => self.known_miss += 1,
            Outcome::UnexpectedHits => self.unexpected_hits += 1,
        }
        self.elapsed_total_ms += elapsed_ms;
        match kind {
            QueryKind::Mixed => {
                self.mixed_total += 1;
                self.mixed_elapsed_ms += elapsed_ms;
                if matches!(outcome, Outcome::Ok) {
                    self.mixed_ok += 1;
                }
            }
            QueryKind::CjkOnly => {
                self.cjk_total += 1;
                self.cjk_elapsed_ms += elapsed_ms;
                if matches!(outcome, Outcome::Ok) {
                    self.cjk_ok += 1;
                }
            }
            QueryKind::LatinOnly => {
                self.latin_total += 1;
                self.latin_elapsed_ms += elapsed_ms;
                if matches!(outcome, Outcome::Ok) {
                    self.latin_ok += 1;
                }
            }
            QueryKind::Other => {
                self.other_total += 1;
                self.other_elapsed_ms += elapsed_ms;
                if matches!(outcome, Outcome::Ok) {
                    self.other_ok += 1;
                }
            }
        }
    }
}

fn classify_query(query: &str) -> QueryKind {
    let mut has_cjk = false;
    let mut has_latin = false;
    for ch in query.chars() {
        if is_cjk_char(ch) {
            has_cjk = true;
        }
        if ch.is_ascii_alphabetic() {
            has_latin = true;
        }
    }
    match (has_cjk, has_latin) {
        (true, true) => QueryKind::Mixed,
        (true, false) => QueryKind::CjkOnly,
        (false, true) => QueryKind::LatinOnly,
        (false, false) => QueryKind::Other,
    }
}

fn is_cjk_char(ch: char) -> bool {
    matches!(
        ch,
        '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{3040}'..='\u{309F}'
            | '\u{30A0}'..='\u{30FF}'
            | '\u{AC00}'..='\u{D7AF}'
            | '\u{20000}'..='\u{2A6DF}'
            | '\u{2A700}'..='\u{2B73F}'
            | '\u{2B740}'..='\u{2B81F}'
            | '\u{2B820}'..='\u{2CEAF}'
            | '\u{2F800}'..='\u{2FA1F}'
    )
}

fn format_snippet(text: &str, limit: usize) -> String {
    let mut cleaned = String::with_capacity(text.len());
    let mut last_was_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !last_was_space {
                cleaned.push(' ');
                last_was_space = true;
            }
        } else {
            cleaned.push(ch);
            last_was_space = false;
        }
    }
    let cleaned = cleaned.trim();
    let truncated: String = cleaned.chars().take(limit).collect();
    if cleaned.chars().count() > limit {
        format!("{truncated}...")
    } else {
        truncated
    }
}

struct Palette {
    enabled: bool,
}

impl Palette {
    fn new() -> Self {
        let mut enabled = None;
        if let Ok(value) = std::env::var("MEMVID_COLOR") {
            enabled = parse_color_mode(&value);
        }
        if enabled.is_none() {
            if let Ok(value) = std::env::var("FORCE_COLOR") {
                if value != "0" {
                    enabled = Some(true);
                }
            }
        }
        if enabled.is_none() {
            let no_color = std::env::var_os("NO_COLOR").is_some();
            let dumb_term = std::env::var("TERM")
                .map(|value| value == "dumb")
                .unwrap_or(false);
            enabled = Some(!no_color && !dumb_term);
        }
        Self {
            enabled: enabled.unwrap_or(true),
        }
    }

    fn paint(&self, text: &str, color: &str) -> String {
        if self.enabled {
            format!("{color}{text}{RESET}")
        } else {
            text.to_string()
        }
    }
}

fn parse_color_mode(value: &str) -> Option<bool> {
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "always" => Some(true),
        "0" | "false" | "no" | "off" | "never" => Some(false),
        "auto" => None,
        _ => None,
    }
}

#[derive(Default)]
struct QueryFilters {
    terms: Vec<String>,
    tags: Vec<String>,
    labels: Vec<String>,
    tracks: Vec<String>,
    uris: Vec<String>,
    scopes: Vec<String>,
}

fn clean_token(token: &str) -> String {
    token
        .trim_matches(|c: char| c.is_whitespace() || c == '"' || c == ',' || c == ';')
        .to_string()
}

fn parse_query_for_prediction(query: &str) -> QueryFilters {
    let mut filters = QueryFilters::default();
    for raw in query.split_whitespace() {
        let cleaned = clean_token(raw);
        if cleaned.is_empty() {
            continue;
        }
        let lower = cleaned.to_ascii_lowercase();
        if matches!(lower.as_str(), "and" | "or" | "not") {
            continue;
        }
        let mut parts = lower.splitn(2, ':');
        let field = parts.next().unwrap_or("");
        if let Some(value) = parts.next() {
            if value.is_empty() {
                filters.terms.push(lower);
                continue;
            }
            match field {
                "tag" => filters.tags.push(value.to_string()),
                "label" => filters.labels.push(value.to_string()),
                "track" => filters.tracks.push(value.to_string()),
                "uri" => filters.uris.push(value.to_string()),
                "scope" => filters.scopes.push(value.to_string()),
                _ => filters.terms.push(lower),
            }
        } else {
            filters.terms.push(lower);
        }
    }
    filters
}

fn build_haystack(doc: &Doc) -> String {
    let mut text = String::new();
    text.push_str(doc.title);
    text.push(' ');
    text.push_str(doc.text);
    text.push(' ');
    text.push_str(doc.uri);
    text.push(' ');
    for tag in doc.tags {
        text.push_str(tag);
        text.push(' ');
    }
    for label in doc.labels {
        text.push_str(label);
        text.push(' ');
    }
    if let Some(track) = doc.track {
        text.push_str(track);
        text.push(' ');
    }
    text.to_ascii_lowercase()
}

fn contains_any(haystack: &[&'static str], needle: &str) -> bool {
    haystack.iter().any(|value| value.eq_ignore_ascii_case(needle))
}

fn predict_titles<'a>(docs: &'a [Doc], query: &str) -> Vec<&'a str> {
    let filters = parse_query_for_prediction(query);
    let mut predicted = Vec::new();
    for doc in docs {
        if !filters.tags.is_empty() && !filters.tags.iter().all(|tag| contains_any(doc.tags, tag)) {
            continue;
        }
        if !filters.labels.is_empty()
            && !filters
                .labels
                .iter()
                .all(|label| contains_any(doc.labels, label))
        {
            continue;
        }
        if !filters.tracks.is_empty() {
            let track = doc.track.unwrap_or("");
            if !filters
                .tracks
                .iter()
                .all(|value| track.eq_ignore_ascii_case(value))
            {
                continue;
            }
        }
        if !filters.uris.is_empty() {
            let uri = doc.uri.to_ascii_lowercase();
            if !filters.uris.iter().all(|value| uri.contains(value)) {
                continue;
            }
        }
        if !filters.scopes.is_empty() {
            let uri = doc.uri.to_ascii_lowercase();
            if !filters.scopes.iter().all(|value| uri.starts_with(value)) {
                continue;
            }
        }
        if !filters.terms.is_empty() {
            let haystack = build_haystack(doc);
            if !filters.terms.iter().all(|term| haystack.contains(term)) {
                continue;
            }
        }
        predicted.push(doc.title);
    }
    predicted
}

fn avg_ms(total_ms: u128, count: usize) -> f64 {
    if count == 0 {
        0.0
    } else {
        total_ms as f64 / count as f64
    }
}

fn main() -> Result<()> {
    let path = PathBuf::from("tmp.mv2");
    if path.exists() {
        std::fs::remove_file(&path)?;
    }

    let mut mem = Memvid::create(&path)?;
    let palette = Palette::new();

    let docs = [
        Doc {
            title: "故障复盘：支付失败",
            uri: "mv2://cn/incidents/2024-11-02-payments",
            text: "2024-11-02 线上支付失败，P0。根因是 rate limit + 重试风暴。修复：降低并发，rollback 到 v1.2.3。",
            tags: &["incident", "payments"],
            labels: &["p0"],
            track: Some("ops"),
        },
        Doc {
            title: "搜索实验：中文召回",
            uri: "mv2://cn/search/chinese-recall",
            text: "中文检索依赖分词器。默认 English + Jieba。短词查询可能不稳定，建议引入 ngram 或双字段索引。",
            tags: &["search", "experiment"],
            labels: &["lex"],
            track: Some("lab"),
        },
        Doc {
            title: "向量检索设计",
            uri: "mv2://cn/search/vector-design",
            text: "向量检索使用 embedding + HNSW，评估 recall@10 与 latency，关注中英文混合查询。",
            tags: &["search", "vector"],
            labels: &["design"],
            track: Some("lab"),
        },
        Doc {
            title: "运维日报 2024-12-01",
            uri: "mv2://cn/ops/daily-2024-12-01",
            text: "Kubernetes 1.29 升级完成，集群重启 3 个节点，Pod 自动恢复。",
            tags: &["ops", "daily"],
            labels: &["report"],
            track: Some("ops"),
        },
        Doc {
            title: "客服工单：退款咨询",
            uri: "mv2://cn/support/refund-2024-12",
            text: "用户咨询退款流程，退款周期 3-5 个工作日，需提交原订单号。",
            tags: &["support", "billing"],
            labels: &["cs"],
            track: Some("support"),
        },
        Doc {
            title: "SLA 约定",
            uri: "mv2://cn/legal/sla",
            text: "服务可用性 SLA 99.95%，包含高峰限流策略与窗口期。",
            tags: &["legal", "sla"],
            labels: &["policy"],
            track: Some("biz"),
        },
        Doc {
            title: "日志规范",
            uri: "mv2://cn/infra/wal",
            text: "日志规范中强调 WAL and append-only storage keep the file consistent and crash safe.",
            tags: &["storage", "doc"],
            labels: &["infra"],
            track: Some("eng"),
        },
    ];

    for doc in &docs {
        let mut options = PutOptions::builder().title(doc.title).uri(doc.uri);
        if let Some(track) = doc.track {
            options = options.track(track);
        }
        for tag in doc.tags {
            options = options.push_tag(*tag);
        }
        for label in doc.labels {
            options = options.label(*label);
        }
        mem.put_bytes_with_options(doc.text.as_bytes(), options.build())?;
    }
    mem.commit()?;

    println!("Chinese recall demo (realistic mini corpus)");
    println!(
        "Indexed {} docs. Some queries are intentionally hard for debugging.\n",
        docs.len()
    );
    println!("Note: default tokenizer is English + Jieba; Chinese recall can vary.\n");
    println!(
        "Colors: OK=green, MISS=red, KNOWN_MISS=yellow, alignment=green/red (MEMVID_COLOR=always|never|auto, NO_COLOR=1, FORCE_COLOR=1)\n"
    );

    let show_docs = true;
    if show_docs {
        println!("Docs:");
        for (idx, doc) in docs.iter().enumerate() {
            let tags = if doc.tags.is_empty() {
                "<none>".to_string()
            } else {
                doc.tags.join(", ")
            };
            let labels = if doc.labels.is_empty() {
                "<none>".to_string()
            } else {
                doc.labels.join(", ")
            };
            let track = doc.track.unwrap_or("<none>");
            println!(
                "  {}. {} (tags: {}; labels: {}; track: {})",
                idx + 1,
                doc.title,
                tags,
                labels,
                track
            );
        }
        println!();
    }

    let cases = [
        Case {
            query: "支付失败",
            expected_titles: &["故障复盘：支付失败"],
            expectation: Expectation::MustMatch,
            note: "基础中文词匹配。",
        },
        Case {
            query: "tag:incident 支付失败",
            expected_titles: &["故障复盘：支付失败"],
            expectation: Expectation::KnownHard,
            note: "tag 查询可能受词干化影响。",
        },
        Case {
            query: "回滚",
            expected_titles: &["故障复盘：支付失败"],
            expectation: Expectation::KnownHard,
            note: "文档只含 rollback 英文。",
        },
        Case {
            query: "中文检索 分词器",
            expected_titles: &["搜索实验：中文召回"],
            expectation: Expectation::MustMatch,
            note: "中文短词 + 组合查询。",
        },
        Case {
            query: "向量检索 embedding",
            expected_titles: &["向量检索设计"],
            expectation: Expectation::MustMatch,
            note: "中英文混合词。",
        },
        Case {
            query: "Kubernetes 升级",
            expected_titles: &["运维日报 2024-12-01"],
            expectation: Expectation::MustMatch,
            note: "英文专有名词 + 中文动词。",
        },
        Case {
            query: "k8s 升级",
            expected_titles: &["运维日报 2024-12-01"],
            expectation: Expectation::KnownHard,
            note: "缩写未出现在文档中。",
        },
        Case {
            query: "退款",
            expected_titles: &["客服工单：退款咨询"],
            expectation: Expectation::MustMatch,
            note: "常见中文业务词。",
        },
        Case {
            query: "support billing 退款",
            expected_titles: &["客服工单：退款咨询"],
            expectation: Expectation::MustMatch,
            note: "英文 tag + 中文内容。",
        },
        Case {
            query: "退费",
            expected_titles: &["客服工单：退款咨询"],
            expectation: Expectation::KnownHard,
            note: "同义词未覆盖。",
        },
        Case {
            query: "SLA 99.95",
            expected_titles: &["SLA 约定"],
            expectation: Expectation::KnownHard,
            note: "数字/小数点分词可能影响命中。",
        },
        Case {
            query: "SLA 可用性",
            expected_titles: &["SLA 约定"],
            expectation: Expectation::MustMatch,
            note: "中文词 + 英文缩写。",
        },
        Case {
            query: "SLA 99.9",
            expected_titles: &[],
            expectation: Expectation::ExpectNone,
            note: "数值不匹配，预期无命中。",
        },
        Case {
            query: "WAL 日志",
            expected_titles: &["日志规范"],
            expectation: Expectation::MustMatch,
            note: "中英文混合词。",
        },
        Case {
            query: "payments 重试",
            expected_titles: &["故障复盘：支付失败"],
            expectation: Expectation::MustMatch,
            note: "英文 tag + 中文动词。",
        },
        Case {
            query: "pod 重启",
            expected_titles: &["运维日报 2024-12-01"],
            expectation: Expectation::MustMatch,
            note: "英文缩写 + 中文动词。",
        },
        Case {
            query: "rate limit 退款",
            expected_titles: &[],
            expectation: Expectation::ExpectNone,
            note: "中英文混用但语义分离。",
        },
    ];

    let top_k = 5;
    let snippet_chars = 80;
    let snippet_preview_chars = 120;
    let mut summary = Summary::default();

    for case in cases {
        let request = SearchRequest {
            query: case.query.to_string(),
            top_k,
            snippet_chars,
            uri: None,
            scope: None,
            cursor: None,
            #[cfg(feature = "temporal_track")]
            temporal: None,
            as_of_frame: None,
            as_of_ts: None,
            no_sketch: false,
        };
        let response = mem.search(request)?;
        let hit_titles: Vec<&str> = response
            .hits
            .iter()
            .filter_map(|hit| hit.title.as_deref())
            .collect();
        let mut found_expected = Vec::new();
        let mut missing_expected = Vec::new();
        for &title in case.expected_titles {
            if hit_titles.iter().any(|hit| *hit == title) {
                found_expected.push(title);
            } else {
                missing_expected.push(title);
            }
        }
        let matched = found_expected.len();
        let expected_total = case.expected_titles.len();
        let recall = if expected_total == 0 {
            None
        } else {
            Some(matched as f32 / expected_total as f32)
        };
        let (outcome, status_label) = match case.expectation {
            Expectation::MustMatch => {
                if missing_expected.is_empty() {
                    (Outcome::Ok, "OK")
                } else {
                    (Outcome::Miss, "MISS")
                }
            }
            Expectation::KnownHard => {
                if missing_expected.is_empty() {
                    (Outcome::Ok, "OK (better than expected)")
                } else {
                    (Outcome::KnownMiss, "KNOWN_MISS")
                }
            }
            Expectation::ExpectNone => {
                if response.hits.is_empty() {
                    (Outcome::Ok, "OK")
                } else {
                    (Outcome::UnexpectedHits, "UNEXPECTED_HITS")
                }
            }
        };
        let kind = classify_query(case.query);
        summary.record(outcome, kind, response.elapsed_ms);
        let matched_ratio = if expected_total == 0 {
            "n/a".to_string()
        } else {
            format!("{matched}/{expected_total}")
        };
        let predicted_titles = predict_titles(&docs, case.query);
        let predicted_label = if predicted_titles.is_empty() {
            "should_miss"
        } else {
            "should_hit"
        };
        let predicted_expected = case
            .expected_titles
            .iter()
            .all(|title| predicted_titles.iter().any(|hit| *hit == *title));
        let prediction_check = if expected_total == 0 {
            if predicted_titles.is_empty() {
                "aligned"
            } else {
                "mismatch"
            }
        } else if predicted_expected {
            "aligned"
        } else {
            "mismatch"
        };

        let query_header = format!("== Query: {} ==", case.query);
        println!("{}", palette.paint(&query_header, BLUE));
        let kind_color = match kind {
            QueryKind::Mixed => CYAN,
            QueryKind::CjkOnly => YELLOW,
            QueryKind::LatinOnly => BLUE,
            QueryKind::Other => DIM,
        };
        let kind_label = palette.paint(kind.label(), kind_color);
        let expected_label = palette.paint(case.expectation.label(), MAGENTA);
        println!(
            "Type: {} | Expectation: {}",
            kind_label,
            expected_label
        );
        if !case.note.is_empty() {
            println!("Note: {}", case.note);
        }
        let prediction_color = if predicted_label == "should_hit" {
            CYAN
        } else {
            DIM
        };
        let prediction_label = palette.paint(predicted_label, prediction_color);
        let alignment_color = if prediction_check == "aligned" {
            GREEN
        } else {
            RED
        };
        let alignment_label = palette.paint(prediction_check, alignment_color);
        println!(
            "Prediction: {} | naive_hits: {} | check: {}",
            prediction_label,
            if predicted_titles.is_empty() {
                "<none>".to_string()
            } else {
                predicted_titles.join(", ")
            },
            alignment_label
        );
        let status_color = match outcome {
            Outcome::Ok => GREEN,
            Outcome::Miss => RED,
            Outcome::KnownMiss => YELLOW,
            Outcome::UnexpectedHits => RED,
        };
        let status_display = palette.paint(status_label, status_color);
        println!(
            "Result: {} | hits: {} | matched: {} | elapsed: {} ms",
            status_display, response.total_hits, matched_ratio, response.elapsed_ms
        );
        println!(
            "Engine: {:?} | top_k: {}",
            response.engine, top_k
        );
        println!(
            "Expected titles: {}",
            if case.expected_titles.is_empty() {
                "<none>".to_string()
            } else {
                case.expected_titles.join(", ")
            }
        );
        println!(
            "Matched expected: {}",
            if found_expected.is_empty() {
                "<none>".to_string()
            } else {
                found_expected.join(", ")
            }
        );
        println!(
            "Missing expected: {}",
            if missing_expected.is_empty() {
                "<none>".to_string()
            } else {
                missing_expected.join(", ")
            }
        );
        match recall {
            Some(value) => println!("Recall: {}/{} ({:.2})", matched, expected_total, value),
            None => println!("Recall: n/a (no expected titles)"),
        }

        if response.hits.is_empty() {
            println!("Top hits: <none>\n");
            continue;
        }

        println!("Top hits:");
        for (rank, hit) in response.hits.iter().take(top_k).enumerate() {
            let title = hit.title.as_deref().unwrap_or("Untitled");
            let score = hit.score.unwrap_or(0.0);
            let tag = if case.expected_titles.iter().any(|t| *t == title) {
                "EXPECTED"
            } else if matches!(case.expectation, Expectation::ExpectNone) {
                "UNEXPECTED"
            } else {
                "OTHER"
            };
            let tag_color = match tag {
                "EXPECTED" => GREEN,
                "UNEXPECTED" => RED,
                _ => DIM,
            };
            let tag_label = palette.paint(tag, tag_color);
            let snippet = format_snippet(&hit.text, snippet_preview_chars);
            println!(
                "  {}. [{}] {} score={:.3} tag={} uri={} snippet=\"{}\"",
                rank + 1,
                hit.frame_id,
                title,
                score,
                tag_label,
                hit.uri,
                snippet
            );
            if let Some(metadata) = &hit.metadata {
                let mut meta_parts = Vec::new();
                if !metadata.tags.is_empty() {
                    meta_parts.push(format!("tags={}", metadata.tags.join(",")));
                }
                if !metadata.labels.is_empty() {
                    meta_parts.push(format!("labels={}", metadata.labels.join(",")));
                }
                if let Some(track) = &metadata.track {
                    meta_parts.push(format!("track={}", track));
                }
                if !meta_parts.is_empty() {
                    println!("     meta: {}", meta_parts.join(" "));
                }
            }
        }
        println!();
    }

    println!("== Summary ==");
    let ok_label = palette.paint("OK", GREEN);
    let miss_label = palette.paint("MISS", RED);
    let known_label = palette.paint("KNOWN_MISS", YELLOW);
    let unexpected_label = palette.paint("UNEXPECTED_HITS", RED);
    println!(
        "Total: {} | {}: {} | {}: {} | {}: {} | {}: {}",
        summary.total,
        ok_label,
        summary.ok,
        miss_label,
        summary.miss,
        known_label,
        summary.known_miss,
        unexpected_label,
        summary.unexpected_hits
    );
    println!(
        "Mixed: {}/{} | CJK: {}/{} | Latin: {}/{} | Other: {}/{}",
        summary.mixed_ok,
        summary.mixed_total,
        summary.cjk_ok,
        summary.cjk_total,
        summary.latin_ok,
        summary.latin_total,
        summary.other_ok,
        summary.other_total
    );
    println!(
        "Latency avg (ms): total={:.2} | mixed={:.2} | cjk={:.2} | latin={:.2} | other={:.2}",
        avg_ms(summary.elapsed_total_ms, summary.total),
        avg_ms(summary.mixed_elapsed_ms, summary.mixed_total),
        avg_ms(summary.cjk_elapsed_ms, summary.cjk_total),
        avg_ms(summary.latin_elapsed_ms, summary.latin_total),
        avg_ms(summary.other_elapsed_ms, summary.other_total)
    );

    Ok(())
}
