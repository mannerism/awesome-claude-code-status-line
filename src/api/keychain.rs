//! macOS Keychain credential retrieval

use crate::error::StatusLineError;

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

/// Retrieve access token from macOS Keychain
pub fn get_access_token() -> Result<AccessToken, StatusLineError> {
    use security_framework::os::macos::passwords::find_generic_password;

    // Find the Claude Code credentials in the default keychain
    let (password, _item) = find_generic_password(None, "Claude Code-credentials", "")
        .map_err(|e| StatusLineError::KeychainAccess(e.to_string()))?;

    // Parse the JSON credentials
    let creds: serde_json::Value = serde_json::from_slice(&password)
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
