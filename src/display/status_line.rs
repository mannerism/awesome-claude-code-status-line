//! Status line builder and formatter

use crate::domain::context::ContextUsageInfo;
use crate::domain::git::GitStatus;
use crate::domain::input::Model;
use crate::domain::session::SessionSize;
use crate::domain::usage::CycleInfo;

/// Builder for status line output
#[derive(Debug, Default)]
pub struct StatusLineBuilder {
    project_name: Option<String>,
    git_status: Option<GitStatus>,
    model: Option<Model>,
    context_usage: Option<ContextUsageInfo>,
    five_hour: Option<CycleInfo>,
    seven_day: Option<CycleInfo>,
    session_size: Option<SessionSize>,
    error_message: Option<String>,
}

impl StatusLineBuilder {
    /// Create a new status line builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the project name
    pub fn project_name(mut self, name: impl Into<String>) -> Self {
        self.project_name = Some(name.into());
        self
    }

    /// Set the git status
    pub fn git_status(mut self, status: GitStatus) -> Self {
        self.git_status = Some(status);
        self
    }

    /// Set the model
    pub fn model(mut self, model: Model) -> Self {
        self.model = Some(model);
        self
    }

    /// Set the context window usage
    pub fn context_usage(mut self, usage: ContextUsageInfo) -> Self {
        self.context_usage = Some(usage);
        self
    }

    /// Set the 5-hour cycle info
    pub fn five_hour(mut self, cycle: CycleInfo) -> Self {
        self.five_hour = Some(cycle);
        self
    }

    /// Set the 7-day cycle info
    pub fn seven_day(mut self, cycle: CycleInfo) -> Self {
        self.seven_day = Some(cycle);
        self
    }

    /// Set the session size
    pub fn session_size(mut self, size: SessionSize) -> Self {
        self.session_size = Some(size);
        self
    }

    /// Set an error message (displayed instead of usage data)
    pub fn error(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    /// Build the status line string
    pub fn build(&self) -> String {
        let mut parts = Vec::new();

        // Model (first)
        if let Some(ref model) = self.model {
            parts.push(format!("🤖 {}", model.display_name()));
        }

        // Context window usage
        if let Some(ref ctx) = self.context_usage {
            let color = ctx.threshold().color();
            let indicator = ctx.threshold().indicator();
            let display = ctx.format_display();
            let colored_display = color.colorize(&display);
            if indicator.is_empty() {
                parts.push(format!("📊 {}", colored_display));
            } else {
                parts.push(format!("📊 {}{}", colored_display, indicator));
            }
        }

        // Error or usage data
        if let Some(ref error) = self.error_message {
            parts.push(format!("⚠️ {}", error));
        } else {
            // 5-hour cycle
            if let Some(ref cycle) = self.five_hour {
                let pct = cycle.utilization.value();
                let color = cycle.utilization.threshold().color();
                let colored_pct = color.colorize(&format!("{}%", pct));
                let reset = cycle.format_reset_local();
                parts.push(format!("⚡ {} @{}", colored_pct, reset));
            }

            // 7-day cycle (with reset time)
            if let Some(ref cycle) = self.seven_day {
                let pct = cycle.utilization.value();
                let color = cycle.utilization.threshold().color();
                let colored_pct = color.colorize(&format!("{}%", pct));
                let reset = cycle.format_reset_local();
                parts.push(format!("📅 {} @{}", colored_pct, reset));
            }
        }

        // Project name
        if let Some(ref name) = self.project_name {
            parts.push(format!("📁 {}", name));
        }

        // Git status
        if let Some(GitStatus::Repo(ref status)) = self.git_status {
            parts.push(format!("🌿 {}", status.format_full()));
        }

        // Session size
        if let Some(ref size) = self.session_size {
            let threshold = size.threshold();
            let color = threshold.color();
            let indicator = threshold.indicator();
            let formatted = size.format_display();
            let colored_size = color.colorize(&formatted);
            if indicator.is_empty() {
                parts.push(format!("📄 {}", colored_size));
            } else {
                parts.push(format!("📄 {}{}", colored_size, indicator));
            }
        }

        parts.join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::git::{BranchInfo, GitRepoStatus};
    use crate::domain::usage::UsagePercentage;
    use chrono::{Duration, Utc};

    #[test]
    fn test_status_line_builder_project_only() {
        let line = StatusLineBuilder::new().project_name("my-project").build();
        assert!(line.contains("📁 my-project"));
    }

    #[test]
    fn test_status_line_builder_with_model() {
        let line = StatusLineBuilder::new()
            .project_name("test")
            .model(Model::from_display_name("Opus 4.5"))
            .build();
        assert!(line.contains("🤖 Opus 4.5"));
    }

    #[test]
    fn test_status_line_builder_with_error() {
        let line = StatusLineBuilder::new()
            .project_name("test")
            .model(Model::from_display_name("Sonnet 4"))
            .error("No creds")
            .build();
        assert!(line.contains("⚠️ No creds"));
        // Should not contain usage data
        assert!(!line.contains("⚡"));
    }

    #[test]
    fn test_status_line_builder_with_usage() {
        let five_hour = CycleInfo::new(UsagePercentage::new(35), Utc::now() + Duration::hours(2));
        let seven_day = CycleInfo::new(UsagePercentage::new(68), Utc::now() + Duration::days(3));

        let line = StatusLineBuilder::new()
            .project_name("test")
            .model(Model::from_display_name("Opus 4.5"))
            .five_hour(five_hour)
            .seven_day(seven_day)
            .build();

        assert!(line.contains("⚡"));
        assert!(line.contains("35%"));
        assert!(line.contains("📅"));
        assert!(line.contains("68%"));
    }

    #[test]
    fn test_status_line_builder_with_git() {
        let git_status = GitStatus::Repo(GitRepoStatus {
            branch: BranchInfo::Branch("main".to_string()),
            modified: true,
            untracked: false,
            ahead: 2,
            behind: 0,
        });

        let line = StatusLineBuilder::new()
            .project_name("test")
            .git_status(git_status)
            .build();

        assert!(line.contains("🌿 main*↑2"));
    }

    #[test]
    fn test_status_line_builder_with_session_size() {
        let size = SessionSize::new(2 * 1024 * 1024); // 2MB

        let line = StatusLineBuilder::new()
            .project_name("test")
            .session_size(size)
            .build();

        assert!(line.contains("📄"));
        assert!(line.contains("2.0MB"));
    }

    #[test]
    fn test_status_line_builder_session_size_warning() {
        let size = SessionSize::new(8 * 1024 * 1024); // 8MB - warning

        let line = StatusLineBuilder::new()
            .project_name("test")
            .session_size(size)
            .build();

        assert!(line.contains("📄"));
        assert!(line.contains("⚠️")); // Warning indicator
    }

    #[test]
    fn test_status_line_builder_session_size_critical() {
        let size = SessionSize::new(20 * 1024 * 1024); // 20MB - critical

        let line = StatusLineBuilder::new()
            .project_name("test")
            .session_size(size)
            .build();

        assert!(line.contains("📄"));
        assert!(line.contains("🔴")); // Critical indicator
    }
}
