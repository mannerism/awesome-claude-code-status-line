//! Git status integration tests

use claude_status::display::status_line::StatusLineBuilder;
use claude_status::domain::git::{BranchInfo, GitRepoStatus, GitStatus};
use claude_status::domain::input::Model;
use claude_status::git::status::get_git_status;
use std::path::Path;

#[test]
fn test_git_repo_branch_displayed() {
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Branch("main".to_string()),
        modified: false,
        untracked: false,
        ahead: 0,
        behind: 0,
    });

    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(git_status)
        .model(Model::from_display_name("Sonnet 4"))
        .build();

    assert!(line.contains("🌿 main"), "Should contain git branch");
}

#[test]
fn test_git_info_omitted_when_not_repo() {
    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(GitStatus::NotRepo)
        .model(Model::from_display_name("Sonnet 4"))
        .build();

    assert!(
        !line.contains("🌿"),
        "Should not contain git indicator when not a repo"
    );
}

#[test]
fn test_git_modified_indicator() {
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Branch("feature".to_string()),
        modified: true,
        untracked: false,
        ahead: 0,
        behind: 0,
    });

    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(git_status)
        .build();

    assert!(
        line.contains("feature*"),
        "Should contain modified indicator"
    );
}

#[test]
fn test_git_untracked_indicator() {
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Branch("develop".to_string()),
        modified: false,
        untracked: true,
        ahead: 0,
        behind: 0,
    });

    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(git_status)
        .build();

    assert!(
        line.contains("develop?"),
        "Should contain untracked indicator"
    );
}

#[test]
fn test_git_ahead_behind_indicators() {
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Branch("main".to_string()),
        modified: false,
        untracked: false,
        ahead: 3,
        behind: 2,
    });

    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(git_status)
        .build();

    assert!(line.contains("↑3"), "Should contain ahead indicator");
    assert!(line.contains("↓2"), "Should contain behind indicator");
}

#[test]
fn test_git_detached_head() {
    let git_status = GitStatus::Repo(GitRepoStatus {
        branch: BranchInfo::Detached("abc1234".to_string()),
        modified: false,
        untracked: false,
        ahead: 0,
        behind: 0,
    });

    let line = StatusLineBuilder::new()
        .project_name("test")
        .git_status(git_status)
        .build();

    // Detached heads are formatted with parentheses
    assert!(
        line.contains("(abc1234)"),
        "Should contain detached head hash in parentheses"
    );
}

#[test]
fn test_get_git_status_in_actual_repo() {
    // This test runs in the actual repo directory
    let cwd = std::env::current_dir().unwrap();
    let result = get_git_status(&cwd);

    assert!(result.is_ok(), "Should succeed in a git repo");
    match result.unwrap() {
        GitStatus::Repo(status) => {
            // Should have a branch name
            match &status.branch {
                BranchInfo::Branch(name) => assert!(!name.is_empty()),
                BranchInfo::Detached(hash) => assert!(!hash.is_empty()),
            }
        }
        GitStatus::NotRepo => panic!("Expected to be in a git repo"),
    }
}

#[test]
fn test_get_git_status_not_repo() {
    let result = get_git_status(Path::new("/tmp"));
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), GitStatus::NotRepo));
}
