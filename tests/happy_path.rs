//! Happy path integration tests

use chrono::{Duration, Utc};
use claude_status::display::status_line::StatusLineBuilder;
use claude_status::domain::git::{BranchInfo, GitRepoStatus, GitStatus};
use claude_status::domain::input::Model;
use claude_status::domain::session::SessionSize;
use claude_status::domain::usage::{CycleInfo, UsagePercentage};

#[test]
fn test_complete_status_line_output() {
    let five_hour = CycleInfo::new(UsagePercentage::new(35), Utc::now() + Duration::hours(2));
    let seven_day = CycleInfo::new(UsagePercentage::new(68), Utc::now() + Duration::days(3));
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Branch("main".to_string()),
        modified: true,
        untracked: false,
        ahead: 2,
        behind: 0,
    });
    let session_size = SessionSize::new(2 * 1024 * 1024); // 2MB

    let line = StatusLineBuilder::new()
        .project_name("my-project")
        .git_status(git_status)
        .model(Model::from_display_name("Opus 4.5"))
        .five_hour(five_hour)
        .seven_day(seven_day)
        .session_size(session_size)
        .build();

    // Verify all expected components are present
    assert!(
        line.contains("📁 my-project"),
        "Should contain project name"
    );
    assert!(
        line.contains("🌿 main*↑2"),
        "Should contain git branch with indicators"
    );
    assert!(line.contains("🤖 Opus 4.5"), "Should contain model");
    assert!(line.contains("⚡"), "Should contain 5-hour cycle indicator");
    assert!(line.contains("35%"), "Should contain 5-hour percentage");
    assert!(line.contains("📅"), "Should contain 7-day cycle indicator");
    assert!(line.contains("68%"), "Should contain 7-day percentage");
    assert!(line.contains("📄"), "Should contain session size indicator");
    assert!(line.contains("2.0MB"), "Should contain session size");
}

#[test]
fn test_minimal_status_line_output() {
    let line = StatusLineBuilder::new()
        .project_name("test")
        .model(Model::from_display_name("Sonnet 4"))
        .build();

    assert!(line.contains("📁 test"));
    assert!(line.contains("🤖 Sonnet 4"));
    // Should not contain optional components
    assert!(!line.contains("🌿")); // No git
    assert!(!line.contains("⚡")); // No usage
    assert!(!line.contains("📄")); // No session size
}

#[test]
fn test_status_line_with_not_repo() {
    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(GitStatus::NotRepo)
        .model(Model::from_display_name("Sonnet 4"))
        .build();

    assert!(line.contains("📁 test"));
    assert!(!line.contains("🌿")); // NotRepo should not show git info
}

#[test]
fn test_status_line_parts_separated_by_pipe() {
    let line = StatusLineBuilder::new()
        .project_name("test")
        .model(Model::from_display_name("Sonnet 4"))
        .build();

    // Parts should be separated by " | "
    assert!(line.contains(" | "), "Parts should be separated by pipe");
}
