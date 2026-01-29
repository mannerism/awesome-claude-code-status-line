//! Benchmark for status line generation
//!
//! Run with: cargo bench --bench status_line

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chrono::{Duration, Utc};
use claude_status::display::status_line::StatusLineBuilder;
use claude_status::domain::git::{BranchInfo, GitRepoStatus, GitStatus};
use claude_status::domain::input::Model;
use claude_status::domain::session::SessionSize;
use claude_status::domain::usage::{CycleInfo, UsagePercentage};

fn bench_status_line_build(c: &mut Criterion) {
    let five_hour = CycleInfo::new(UsagePercentage::new(35), Utc::now() + Duration::hours(2));
    let seven_day = CycleInfo::new(UsagePercentage::new(68), Utc::now() + Duration::days(3));
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Branch("main".to_string()),
        modified: true,
        untracked: false,
        ahead: 2,
        behind: 0,
    });
    let session_size = SessionSize::new(2 * 1024 * 1024);

    c.bench_function("status_line_build_full", |b| {
        b.iter(|| {
            StatusLineBuilder::new()
                .project_name(black_box("my-project"))
                .git_status(black_box(git_status.clone()))
                .model(black_box(Model::Opus4))
                .five_hour(black_box(five_hour.clone()))
                .seven_day(black_box(seven_day.clone()))
                .session_size(black_box(session_size))
                .build()
        })
    });

    c.bench_function("status_line_build_minimal", |b| {
        b.iter(|| {
            StatusLineBuilder::new()
                .project_name(black_box("test"))
                .model(black_box(Model::Sonnet4))
                .build()
        })
    });
}

criterion_group!(benches, bench_status_line_build);
criterion_main!(benches);
