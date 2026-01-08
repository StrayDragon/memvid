use std::path::PathBuf;

use memvid_core::{Memvid, PutOptions, Result, SearchRequest};

struct Case {
    query: &'static str,
    expected_titles: &'static [&'static str],
}

fn main() -> Result<()> {
    let path = PathBuf::from("tmp.mv2");
    if path.exists() {
        std::fs::remove_file(&path)?;
    }

    let mut mem = Memvid::create(&path)?;

    let docs = [
        (
            "中文搜索段落",
            "中文搜索通常依赖分词和分字策略但本项目使用英文分词器因此召回率可能下降",
        ),
        (
            "向量检索段落",
            "向量检索和语义搜索是现代检索系统的核心能力也影响中文召回",
        ),
        (
            "英文说明",
            "WAL and append-only storage keep the file consistent and crash safe.",
        ),
    ];

    for (idx, (title, text)) in docs.iter().enumerate() {
        let options = PutOptions::builder()
            .title(*title)
            .uri(format!("mv2://cn/doc{}", idx + 1))
            .build();
        mem.put_bytes_with_options(text.as_bytes(), options)?;
    }
    mem.commit()?;

    let cases = [
        Case {
            query: "中文搜索",
            expected_titles: &["中文搜索段落"],
        },
        Case {
            query: "分词",
            expected_titles: &["中文搜索段落", "向量检索段落"],
        },
        Case {
            query: "向量检索",
            expected_titles: &["向量检索段落"],
        },
        Case {
            query: "语义搜索",
            expected_titles: &["向量检索段落"],
        },
        Case {
            query: "WAL",
            expected_titles: &["英文说明"],
        },
    ];

    for case in cases {
        let request = SearchRequest {
            query: case.query.to_string(),
            top_k: 5,
            snippet_chars: 80,
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
        let matched = case
            .expected_titles
            .iter()
            .filter(|title| hit_titles.iter().any(|hit| *hit == **title))
            .count();
        let expected = case.expected_titles.len().max(1);
        let recall = matched as f32 / expected as f32;

        println!("query: {}", case.query);
        println!(
            "engine: {:?}, hits: {}, recall: {}/{} ({:.2})",
            response.engine,
            response.total_hits,
            matched,
            expected,
            recall
        );
        if response.hits.is_empty() {
            println!("hits: []\n");
            continue;
        }
        for hit in response.hits.iter().take(5) {
            let title = hit.title.as_deref().unwrap_or("Untitled");
            let score = hit.score.unwrap_or(0.0);
            println!(
                "- [{}] {} score={:.3} snippet={}",
                hit.frame_id, title, score, hit.text
            );
        }
        println!();
    }

    Ok(())
}
