# Research: macOS Security CLI for Keychain Access

**Date**: 2026-01-29
**Feature**: 002-security-cli-keychain

## Research Questions

### Q1: How does the macOS `security` CLI work for password retrieval?

**Decision**: Use `security find-generic-password -s <service> -w`

**Rationale**:
- The `-s` flag specifies the service name (matches the keychain item's "Where" field)
- The `-w` flag outputs only the password data (not the full keychain item attributes)
- This matches the existing keychain item created by Claude Code: service name is "Claude Code-credentials"

**Alternatives Considered**:
- `security find-generic-password -a <account> -w`: Requires knowing the account name, which is empty for Claude Code credentials
- Using `-g` flag: Outputs password in a different format (with "password:" prefix), harder to parse

### Q2: Why does the `security` CLI avoid per-binary permission prompts?

**Decision**: CLI inherits terminal application's keychain ACL (Access Control List)

**Rationale**:
- When you run a CLI tool from Terminal.app/iTerm, the keychain access is checked against the terminal's code signature, not the binary's
- Terminal apps have stable code signatures (they don't change on every rebuild)
- Once "Always Allow" is granted to Terminal.app, all CLI tools invoked from it inherit that permission
- This is different from `security-framework` which links directly to the binary, creating a per-binary ACL check

**Alternatives Considered**:
- Code-signing the binary: Requires Apple Developer account; doesn't help during development
- Environment variable for API key: Works but less secure; credential visible in process listing

### Q3: What errors can the `security` command return?

**Decision**: Map exit codes and stderr to existing error types

**Error Cases**:
| Scenario | Exit Code | Stderr | Mapped Error |
|----------|-----------|--------|--------------|
| Item not found | 44 | "The specified item could not be found" | `CredentialsNotFound` |
| Access denied (user clicked Deny) | 36 | "User interaction is not allowed" or similar | `KeychainAccess` |
| Keychain locked | 36 | "The user name or passphrase you entered is not correct" | `KeychainAccess` |
| Command not found | N/A (spawn fails) | N/A | `KeychainAccess` |

**Rationale**: The existing error variants already cover these cases appropriately.

### Q4: Is the `security` CLI available on all macOS versions?

**Decision**: Yes, safe to rely on

**Rationale**:
- `security` is a core macOS system utility, present since Mac OS X 10.3 Panther (2003)
- It's part of the Security framework's command-line interface
- Located at `/usr/bin/security`, which is in the default PATH
- No additional installation required

## Conclusion

The implementation approach is straightforward:
1. Use `std::process::Command` to invoke `security find-generic-password -s "Claude Code-credentials" -w`
2. Parse the JSON output and extract `claudeAiOauth.accessToken`
3. Map errors to existing `StatusLineError` variants
4. Remove `security-framework` from Cargo.toml

No additional research needed. Ready for implementation.
