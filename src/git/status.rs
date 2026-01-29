//! Git status detection

use std::path::Path;
use std::process::Command;

use crate::domain::git::{BranchInfo, GitRepoStatus, GitStatus};
use crate::error::StatusLineError;

/// Get git status for the given directory
pub fn get_git_status(cwd: &Path) -> Result<GitStatus, StatusLineError> {
    // Check if inside a git repository
    let is_repo = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .current_dir(cwd)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !is_repo {
        return Ok(GitStatus::NotRepo);
    }

    // Get branch name
    let branch = get_branch(cwd)?;

    // Get status (modified/untracked)
    let (modified, untracked) = get_status_flags(cwd)?;

    // Get ahead/behind counts
    let (ahead, behind) = get_ahead_behind(cwd).unwrap_or((0, 0));

    Ok(GitStatus::Repo(GitRepoStatus {
        branch,
        modified,
        untracked,
        ahead,
        behind,
    }))
}

/// Get current branch name or commit hash
fn get_branch(cwd: &Path) -> Result<BranchInfo, StatusLineError> {
    // Try to get branch name
    let branch_output = Command::new("git")
        .args(["symbolic-ref", "--short", "HEAD"])
        .current_dir(cwd)
        .output()
        .map_err(|e| StatusLineError::Git(e.to_string()))?;

    if branch_output.status.success() {
        let branch = String::from_utf8_lossy(&branch_output.stdout)
            .trim()
            .to_string();
        return Ok(BranchInfo::Branch(branch));
    }

    // Detached HEAD - get short commit hash
    let commit_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .current_dir(cwd)
        .output()
        .map_err(|e| StatusLineError::Git(e.to_string()))?;

    let hash = String::from_utf8_lossy(&commit_output.stdout)
        .trim()
        .to_string();
    Ok(BranchInfo::Detached(hash))
}

/// Get modified and untracked flags
fn get_status_flags(cwd: &Path) -> Result<(bool, bool), StatusLineError> {
    let output = Command::new("git")
        .args(["status", "--porcelain=v1"])
        .current_dir(cwd)
        .output()
        .map_err(|e| StatusLineError::Git(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut modified = false;
    let mut untracked = false;

    for line in stdout.lines() {
        if line.len() >= 2 {
            let chars: Vec<char> = line.chars().collect();
            let staged = chars[0];
            let working = chars[1];

            // Check for modifications
            if staged != ' ' && staged != '?' {
                modified = true;
            }
            if working == 'M' || working == 'D' || working == 'A' {
                modified = true;
            }

            // Check for untracked
            if staged == '?' {
                untracked = true;
            }
        }
    }

    Ok((modified, untracked))
}

/// Get ahead/behind counts from upstream
fn get_ahead_behind(cwd: &Path) -> Result<(u32, u32), StatusLineError> {
    let output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", "@{upstream}...HEAD"])
        .current_dir(cwd)
        .output()
        .map_err(|e| StatusLineError::Git(e.to_string()))?;

    if !output.status.success() {
        // No upstream configured
        return Ok((0, 0));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let parts: Vec<&str> = stdout.trim().split('\t').collect();

    if parts.len() == 2 {
        let behind = parts[0].parse().unwrap_or(0);
        let ahead = parts[1].parse().unwrap_or(0);
        Ok((ahead, behind))
    } else {
        Ok((0, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_git_status_in_repo() {
        // This test runs in the actual repo
        let cwd = env::current_dir().unwrap();
        let result = get_git_status(&cwd);
        assert!(result.is_ok());

        match result.unwrap() {
            GitStatus::Repo(status) => {
                // Should have a branch
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
        // Use a directory that's definitely not a git repo
        let result = get_git_status(Path::new("/tmp"));
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), GitStatus::NotRepo));
    }
}
