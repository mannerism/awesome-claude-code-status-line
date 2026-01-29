# Data Model: macOS Security CLI for Keychain Access

**Date**: 2026-01-29
**Feature**: 002-security-cli-keychain

## Overview

This feature does not introduce new data types. The existing domain types are already well-designed and sufficient for the CLI-based approach.

## Existing Types (Unchanged)

### AccessToken

```rust
/// OAuth access token (cleared from memory on drop in future with zeroize)
#[derive(Clone)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(token: String) -> Self;
    pub fn as_str(&self) -> &str;
}

// Debug is implemented to redact the token value
impl std::fmt::Debug for AccessToken { ... }
```

**Purpose**: Encapsulates the OAuth access token with security-conscious design (redacted debug output).

**Location**: `src/api/keychain.rs`

### StatusLineError (Relevant Variants)

```rust
#[derive(Debug, thiserror::Error)]
pub enum StatusLineError {
    /// Keychain credentials not found
    #[error("Keychain credentials not found: authenticate with Claude Code first")]
    CredentialsNotFound,

    /// Keychain access denied
    #[error("Keychain access denied: {0}")]
    KeychainAccess(String),

    /// API response invalid (used for JSON parsing)
    #[error("API response invalid: {0}")]
    ApiResponse(String),
}
```

**Purpose**: Error types for keychain operations, already support all CLI error scenarios.

**Location**: `src/error.rs`

## Keychain Item Structure (External)

The "Claude Code-credentials" keychain item contains JSON with this structure:

```json
{
  "claudeAiOauth": {
    "accessToken": "<oauth-access-token>",
    "refreshToken": "<refresh-token>",
    "expiresAt": "<timestamp>"
  }
}
```

**Note**: We only read `claudeAiOauth.accessToken`. Other fields are ignored.

## No New Types Required

The change from `security-framework` crate to `security` CLI is purely an implementation detail. The public API (`get_access_token() -> Result<AccessToken, StatusLineError>`) remains unchanged.
