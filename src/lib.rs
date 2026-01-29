//! # Claude Code Status Line Library
//!
//! A high-performance Rust library for generating Claude Code status line output
//! with usage tracking, session size monitoring, and git integration.
//!
//! ## Features
//!
//! - **API Usage Tracking**: Fetches and displays 5-hour and 7-day usage cycles
//! - **Session Size Monitoring**: Color-coded warnings for session file sizes
//! - **Git Integration**: Branch name, modified/untracked indicators, ahead/behind counts
//! - **Fast Startup**: Sub-millisecond status line generation
//!
//! ## Example
//!
//! ```rust,no_run
//! use claude_status::display::status_line::StatusLineBuilder;
//! use claude_status::domain::input::Model;
//!
//! let line = StatusLineBuilder::new()
//!     .project_name("my-project")
//!     .model(Model::from_display_name("Opus 4.5"))
//!     .build();
//!
//! println!("{}", line);
//! ```
//!
//! ## Modules
//!
//! - [`api`]: Anthropic API client and keychain credential retrieval
//! - [`config`]: User configuration management
//! - [`display`]: Status line formatting and ANSI colors
//! - [`domain`]: Core domain types (usage, session, input, git)
//! - [`error`]: Error types with brief/detailed messages
//! - [`git`]: Git repository status detection

/// Anthropic API client and macOS Keychain credential retrieval
pub mod api;

/// User configuration for display preferences
pub mod config;

/// Status line formatting and ANSI color support
pub mod display;

/// Core domain types for usage tracking and session monitoring
pub mod domain;

/// Error types with brief messages for status line display
pub mod error;

/// Git repository status detection
pub mod git;

pub use error::StatusLineError;
