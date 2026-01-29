//! Benchmark for JSON input parsing
//!
//! Run with: cargo bench --bench parsing

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use claude_status::domain::input::ClaudeInput;

fn bench_input_parsing(c: &mut Criterion) {
    let minimal_json = r#"{"cwd": "/test"}"#;

    let full_json = r#"{
        "cwd": "/Users/dev/my-project",
        "model": {"display_name": "Opus 4.5"},
        "transcript_path": "/Users/dev/.claude/session.jsonl",
        "session_size_bytes": 2097152,
        "context_window": {
            "current_usage": {
                "input_tokens": 50000,
                "cache_creation_input_tokens": 10000,
                "cache_read_input_tokens": 5000
            },
            "context_window_size": 200000
        }
    }"#;

    c.bench_function("parse_minimal_input", |b| {
        b.iter(|| serde_json::from_str::<ClaudeInput>(black_box(minimal_json)).unwrap())
    });

    c.bench_function("parse_full_input", |b| {
        b.iter(|| serde_json::from_str::<ClaudeInput>(black_box(full_json)).unwrap())
    });
}

criterion_group!(benches, bench_input_parsing);
criterion_main!(benches);
