//! Error types for Claude Code status line

use thiserror::Error;

/// Main error type for status line operations
#[derive(Debug, Error)]
pub enum StatusLineError {
    /// Keychain credentials not found
    #[error("Keychain credentials not found: authenticate with Claude Code first")]
    CredentialsNotFound,

    /// Keychain access denied
    #[error("Keychain access denied: {0}")]
    KeychainAccess(String),

    /// API request failed
    #[error("API request failed: {0}")]
    ApiRequest(String),

    /// API response invalid
    #[error("API response invalid: {0}")]
    ApiResponse(String),

    /// Invalid JSON input
    #[error("Invalid JSON input: {0}")]
    InvalidInput(#[from] serde_json::Error),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Git command failed
    #[error("Git command failed: {0}")]
    Git(String),

    /// Timestamp parsing error
    #[error("Invalid timestamp: {0}")]
    TimestampParse(#[from] chrono::ParseError),
}

impl StatusLineError {
    /// Brief message for status line display
    pub fn brief(&self) -> &'static str {
        match self {
            Self::CredentialsNotFound | Self::KeychainAccess(_) => "No creds",
            Self::ApiRequest(_) | Self::ApiResponse(_) => "API error",
            Self::InvalidInput(_) => "Bad input",
            Self::Io(_) => "IO error",
            Self::Git(_) => "", // Git errors are silent (graceful degradation)
            Self::TimestampParse(_) => "Parse error",
        }
    }

    /// Whether this error should be displayed in status line
    pub fn show_in_status_line(&self) -> bool {
        !self.brief().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brief_credentials_not_found() {
        let err = StatusLineError::CredentialsNotFound;
        assert_eq!(err.brief(), "No creds");
    }

    #[test]
    fn test_brief_keychain_access() {
        let err = StatusLineError::KeychainAccess("permission denied".to_string());
        assert_eq!(err.brief(), "No creds");
    }

    #[test]
    fn test_brief_api_request() {
        let err = StatusLineError::ApiRequest("timeout".to_string());
        assert_eq!(err.brief(), "API error");
    }

    #[test]
    fn test_brief_api_response() {
        let err = StatusLineError::ApiResponse("invalid json".to_string());
        assert_eq!(err.brief(), "API error");
    }

    #[test]
    fn test_brief_git_is_empty() {
        let err = StatusLineError::Git("not a git repo".to_string());
        assert_eq!(err.brief(), "");
    }

    #[test]
    fn test_show_in_status_line_true() {
        let err = StatusLineError::CredentialsNotFound;
        assert!(err.show_in_status_line());
    }

    #[test]
    fn test_show_in_status_line_false_for_git() {
        let err = StatusLineError::Git("error".to_string());
        assert!(!err.show_in_status_line());
    }

    #[test]
    fn test_error_display() {
        let err = StatusLineError::CredentialsNotFound;
        let display = format!("{}", err);
        assert!(display.contains("Keychain credentials not found"));
    }
}
