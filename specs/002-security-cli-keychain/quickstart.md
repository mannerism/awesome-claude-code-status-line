# Quickstart: macOS Security CLI for Keychain Access

**Date**: 2026-01-29
**Feature**: 002-security-cli-keychain

## Prerequisites

- macOS (any modern version)
- Rust 1.75+ installed
- Claude Code authenticated (so keychain item exists)

## First-Time Terminal Authorization

When running `claude-status` for the first time from a terminal, macOS may prompt for keychain access. This prompt will be for your **terminal application** (Terminal.app, iTerm, etc.), not for the `claude-status` binary itself.

Click "Always Allow" to grant permanent access. After this one-time authorization:
- No prompts will appear on subsequent runs
- No prompts will appear after rebuilding the binary
- The authorization persists across terminal sessions

## Changes Summary

This feature modifies two files:

| File | Change |
|------|--------|
| `src/api/keychain.rs` | Use `security` CLI instead of `security-framework` crate |
| `Cargo.toml` | Remove `security-framework` dependency |

## Implementation Steps

### Step 1: Update keychain.rs

Replace the `security-framework` import and usage with `std::process::Command`:

```rust
use std::process::Command;

pub fn get_access_token() -> Result<AccessToken, StatusLineError> {
    let output = Command::new("security")
        .args(["find-generic-password", "-s", "Claude Code-credentials", "-w"])
        .output()
        .map_err(|e| StatusLineError::KeychainAccess(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(StatusLineError::KeychainAccess(stderr.to_string()));
    }

    let password = String::from_utf8(output.stdout)
        .map_err(|e| StatusLineError::KeychainAccess(e.to_string()))?;

    let creds: serde_json::Value = serde_json::from_str(password.trim())
        .map_err(|e| StatusLineError::ApiResponse(e.to_string()))?;

    let token = creds
        .get("claudeAiOauth")
        .and_then(|oauth| oauth.get("accessToken"))
        .and_then(|t| t.as_str())
        .ok_or(StatusLineError::CredentialsNotFound)?;

    Ok(AccessToken::new(token.to_string()))
}
```

### Step 2: Update Cargo.toml

Remove the `security-framework` dependency:

```diff
 # HTTP client (blocking, minimal)
 ureq = { version = "2.9", features = ["json"] }

-# macOS Keychain access
-security-framework = "3.0"
-
 # Timestamp parsing and timezone handling
 chrono = { version = "0.4", features = ["serde"] }
```

### Step 3: Verify

```bash
# Run tests
cargo test

# Run clippy
cargo clippy -- -D warnings

# Build release
cargo build --release

# Manual test - rebuild and run, should not prompt for keychain access
./target/release/claude-status
```

## Verification Checklist

- [ ] `cargo test` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Binary runs without keychain permission dialog
- [ ] Credentials are retrieved successfully
- [ ] Error messages are clear when credentials are missing

## Rollback

If issues occur, revert both files:
- Restore `security-framework = "3.0"` to Cargo.toml
- Restore original `get_access_token()` implementation using `find_generic_password`
