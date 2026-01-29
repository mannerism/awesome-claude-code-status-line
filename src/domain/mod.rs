//! Domain types for Claude Code status line
//!
//! This module contains all the domain types following the "Correctness Through Types" principle.

pub mod context;
pub mod git;
pub mod input;
pub mod session;
pub mod usage;

pub use context::{ContextThreshold, ContextUsageInfo};
pub use git::{BranchInfo, GitRepoStatus, GitStatus};
pub use input::{ClaudeInput, ContextUsage, ContextWindow, Model, ModelInfo};
pub use session::{SessionSize, SizeThreshold};
pub use usage::{CycleInfo, UsagePercentage, UsageThreshold};
