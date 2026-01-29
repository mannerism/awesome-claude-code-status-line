//! macOS Keychain credential retrieval

use crate::error::StatusLineError;
use std::process::Command;

/// OAuth access token (cleared from memory on drop in future with zeroize)
#[derive(Clone)]
pub struct AccessToken(String);

impl AccessToken {
    /// Create a new access token
    pub fn new(token: String) -> Self {
        Self(token)
    }

    /// Get token as string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Prevent accidental logging
impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessToken([REDACTED])")
    }
}

/// Retrieve access token from macOS Keychain using the `security` CLI.
/// This avoids per-binary permission prompts - once allowed for Terminal/iTerm,
/// it works forever regardless of binary rebuilds.
pub fn get_access_token() -> Result<AccessToken, StatusLineError> {
    // Use the security CLI which inherits terminal permissions
    let output = Command::new("security")
        .args([
            "find-generic-password",
            "-s",
            "Claude Code-credentials",
            "-w",
        ])
        .output()
        .map_err(|e| StatusLineError::KeychainAccess(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(StatusLineError::KeychainAccess(stderr.to_string()));
    }

    let password = String::from_utf8(output.stdout)
        .map_err(|e| StatusLineError::KeychainAccess(e.to_string()))?;

    // Parse the JSON credentials
    let creds: serde_json::Value = serde_json::from_str(password.trim())
        .map_err(|e| StatusLineError::ApiResponse(e.to_string()))?;

    // Extract the access token
    let token = creds
        .get("claudeAiOauth")
        .and_then(|oauth| oauth.get("accessToken"))
        .and_then(|t| t.as_str())
        .ok_or(StatusLineError::CredentialsNotFound)?;

    Ok(AccessToken::new(token.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_token_debug_redacted() {
        let token = AccessToken::new("secret-token-123".to_string());
        let debug = format!("{:?}", token);
        assert_eq!(debug, "AccessToken([REDACTED])");
        assert!(!debug.contains("secret"));
    }

    #[test]
    fn test_access_token_as_str() {
        let token = AccessToken::new("test-token".to_string());
        assert_eq!(token.as_str(), "test-token");
    }
}
