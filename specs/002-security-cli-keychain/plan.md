# Implementation Plan: Use macOS Security CLI for Keychain Access

**Branch**: `002-security-cli-keychain` | **Date**: 2026-01-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-security-cli-keychain/spec.md`

## Summary

Replace the `security-framework` Rust crate with macOS `security` CLI command for keychain credential retrieval. This eliminates per-binary permission prompts that occur after every rebuild, since the CLI inherits the terminal's existing keychain authorization.

## Technical Context

**Language/Version**: Rust 1.75+ (MSRV documented in Cargo.toml)
**Primary Dependencies**: serde, serde_json, ureq, chrono, clap, thiserror (unchanged); removing security-framework
**Storage**: N/A (reading from macOS Keychain via CLI)
**Testing**: `cargo test` + `cargo test --all-features`
**Benchmarking**: criterion (existing benchmarks unaffected)
**Target Platform**: macOS only (this is a macOS-specific keychain feature)
**Project Type**: Single binary crate (existing structure)
**Performance Goals**: <100ms for keychain retrieval (CLI execution is fast)
**Constraints**: macOS-only; relies on `security` CLI being available (system utility)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                    | Compliance | Notes                                                                      |
| ---------------------------- | ---------- | -------------------------------------------------------------------------- |
| I. Simple, Not Easy          | ✅          | Removes external crate dependency; uses standard system CLI                |
| II. Spec-Driven (SDD)        | ✅          | spec.md complete with Given/When/Then acceptance scenarios                 |
| III. Test-Driven (TDD)       | ✅          | Existing tests preserved; new tests for CLI error cases                    |
| IV. Rust Best Practices      | ✅          | No .unwrap() in lib code; custom error types already exist                 |
| V. Correctness Through Types | ✅          | AccessToken newtype already exists; error variants cover failure modes     |

**Blockers**: None - all principles satisfied.

## Project Structure

### Documentation (this feature)

```text
specs/002-security-cli-keychain/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output (minimal - existing types sufficient)
├── quickstart.md        # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
Cargo.toml               # Remove security-framework dependency
src/
├── main.rs              # Entry point (unchanged)
├── lib.rs               # Library root (unchanged)
├── error.rs             # StatusLineError (unchanged - variants already exist)
├── api/
│   ├── mod.rs           # API module (unchanged)
│   ├── client.rs        # Anthropic API client (unchanged)
│   └── keychain.rs      # TARGET: Use security CLI instead of security-framework
├── config/
│   └── mod.rs           # Configuration (unchanged)
├── display/
│   ├── colors.rs        # ANSI colors (unchanged)
│   └── status_line.rs   # Status line builder (unchanged)
├── domain/
│   ├── mod.rs           # Domain types (unchanged)
│   ├── git.rs           # Git status types (unchanged)
│   ├── input.rs         # Input types (unchanged)
│   ├── session.rs       # Session types (unchanged)
│   └── usage.rs         # Usage types (unchanged)
└── git/
    └── status.rs        # Git status (unchanged)

tests/                   # Existing tests (should pass without changes)
benches/                 # Existing benchmarks (unaffected)
```

**Structure Decision**: Existing single binary crate structure is preserved. Only `src/api/keychain.rs` and `Cargo.toml` are modified.

## Domain Types

| Concept       | Type                      | Rationale                                         |
| ------------- | ------------------------- | ------------------------------------------------- |
| Access Token  | `struct AccessToken(String)` | Already exists - encapsulates secret, redacted debug |
| Error Variant | `StatusLineError::KeychainAccess` | Already exists - covers CLI execution failures    |
| Error Variant | `StatusLineError::CredentialsNotFound` | Already exists - covers missing token in JSON     |

**Note**: No new domain types needed. Existing types are sufficient and well-designed.

## Error Handling Strategy

Existing error types in `src/error.rs` are already appropriate:

```rust
#[derive(Debug, thiserror::Error)]
pub enum StatusLineError {
    #[error("Keychain credentials not found: authenticate with Claude Code first")]
    CredentialsNotFound,  // Used when JSON lacks accessToken field

    #[error("Keychain access denied: {0}")]
    KeychainAccess(String),  // Used for: CLI not found, command failed, UTF-8 error

    #[error("API response invalid: {0}")]
    ApiResponse(String),  // Used for JSON parsing failures
    // ... other variants unchanged
}
```

**Error Mapping**:
- `Command::new("security")` spawn fails → `KeychainAccess`
- `security` command returns non-zero exit → `KeychainAccess` (with stderr)
- Output is not valid UTF-8 → `KeychainAccess`
- JSON parsing fails → `ApiResponse`
- `claudeAiOauth.accessToken` missing → `CredentialsNotFound`

## Complexity Tracking

No constitution violations. This change simplifies the codebase by:
1. Removing a native framework dependency (security-framework)
2. Using a standard system CLI that inherits terminal permissions
3. Reducing binary size (no framework linking)

| Change                        | Impact     | Rationale                              |
| ----------------------------- | ---------- | -------------------------------------- |
| Remove security-framework     | Simplifies | One fewer native dependency to manage  |
| Use std::process::Command     | No change  | Standard library, already used for git |
| Binary size reduction         | Improves   | Smaller binary, faster distribution    |

## Implementation Approach

### Phase 1: Core Change

1. **Update `src/api/keychain.rs`**:
   - Replace `security_framework::os::macos::passwords::find_generic_password` with `std::process::Command`
   - Execute: `security find-generic-password -s "Claude Code-credentials" -w`
   - Parse JSON output, extract `claudeAiOauth.accessToken`
   - Map errors to existing `StatusLineError` variants

2. **Update `Cargo.toml`**:
   - Remove `security-framework = "3.0"` from dependencies

### Phase 2: Verification

1. **Run existing tests**: `cargo test`
2. **Run clippy**: `cargo clippy -- -D warnings`
3. **Build release**: `cargo build --release`
4. **Manual verification**: Run binary, verify no keychain popup on rebuild
