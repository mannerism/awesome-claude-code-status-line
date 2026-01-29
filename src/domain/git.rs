//! Git status types for repository information display

/// Git repository status
#[derive(Debug, Clone)]
pub enum GitStatus {
    /// Not inside a git repository
    NotRepo,
    /// Inside a repo with status information
    Repo(GitRepoStatus),
}

/// Status of a git repository
#[derive(Debug, Clone)]
pub struct GitRepoStatus {
    /// Current branch name or commit hash
    pub branch: BranchInfo,
    /// Working tree has modifications
    pub modified: bool,
    /// Untracked files exist
    pub untracked: bool,
    /// Commits ahead of upstream
    pub ahead: u32,
    /// Commits behind upstream
    pub behind: u32,
}

impl GitRepoStatus {
    /// Format status indicators for display
    pub fn format_indicators(&self) -> String {
        let mut indicators = String::new();
        if self.modified {
            indicators.push('*');
        }
        if self.untracked {
            indicators.push('?');
        }
        if self.ahead > 0 {
            indicators.push_str(&format!("↑{}", self.ahead));
        }
        if self.behind > 0 {
            indicators.push_str(&format!("↓{}", self.behind));
        }
        indicators
    }

    /// Format branch name for display
    pub fn format_branch(&self) -> String {
        match &self.branch {
            BranchInfo::Branch(name) => name.clone(),
            BranchInfo::Detached(hash) => format!("({})", hash),
        }
    }

    /// Format full git status for status line
    pub fn format_full(&self) -> String {
        let branch = self.format_branch();
        let indicators = self.format_indicators();
        if indicators.is_empty() {
            branch
        } else {
            format!("{}{}", branch, indicators)
        }
    }
}

/// Branch identification
#[derive(Debug, Clone)]
pub enum BranchInfo {
    /// On a named branch
    Branch(String),
    /// Detached HEAD at commit
    Detached(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_repo_status_format_indicators_empty() {
        let status = GitRepoStatus {
            branch: BranchInfo::Branch("main".to_string()),
            modified: false,
            untracked: false,
            ahead: 0,
            behind: 0,
        };
        assert_eq!(status.format_indicators(), "");
    }

    #[test]
    fn test_git_repo_status_format_indicators_modified() {
        let status = GitRepoStatus {
            branch: BranchInfo::Branch("main".to_string()),
            modified: true,
            untracked: false,
            ahead: 0,
            behind: 0,
        };
        assert_eq!(status.format_indicators(), "*");
    }

    #[test]
    fn test_git_repo_status_format_indicators_all() {
        let status = GitRepoStatus {
            branch: BranchInfo::Branch("main".to_string()),
            modified: true,
            untracked: true,
            ahead: 2,
            behind: 1,
        };
        assert_eq!(status.format_indicators(), "*?↑2↓1");
    }

    #[test]
    fn test_git_repo_status_format_branch_named() {
        let status = GitRepoStatus {
            branch: BranchInfo::Branch("feature-branch".to_string()),
            modified: false,
            untracked: false,
            ahead: 0,
            behind: 0,
        };
        assert_eq!(status.format_branch(), "feature-branch");
    }

    #[test]
    fn test_git_repo_status_format_branch_detached() {
        let status = GitRepoStatus {
            branch: BranchInfo::Detached("abc1234".to_string()),
            modified: false,
            untracked: false,
            ahead: 0,
            behind: 0,
        };
        assert_eq!(status.format_branch(), "(abc1234)");
    }

    #[test]
    fn test_git_repo_status_format_full() {
        let status = GitRepoStatus {
            branch: BranchInfo::Branch("main".to_string()),
            modified: true,
            untracked: true,
            ahead: 0,
            behind: 0,
        };
        assert_eq!(status.format_full(), "main*?");
    }

    #[test]
    fn test_git_repo_status_format_full_clean() {
        let status = GitRepoStatus {
            branch: BranchInfo::Branch("main".to_string()),
            modified: false,
            untracked: false,
            ahead: 0,
            behind: 0,
        };
        assert_eq!(status.format_full(), "main");
    }
}
