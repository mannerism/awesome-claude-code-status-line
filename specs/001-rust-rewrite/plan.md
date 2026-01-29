# Implementation Plan: Claude Code Status Line - Rust Rewrite

**Branch**: `001-rust-rewrite` | **Date**: 2026-01-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-rust-rewrite/spec.md`

## Summary

Rewrite the Claude Code usage tracking status line from Python to Rust as a single static binary. The tool reads JSON from stdin (provided by Claude Code), fetches usage data from Anthropic's OAuth API via macOS Keychain credentials, monitors session file size, integrates with Git, and outputs a formatted status line with color-coded usage indicators. New feature: session size monitoring with threshold warnings (green <5MB, yellow 5-15MB, red >15MB).

## Technical Context

**Language/Version**: Rust 1.75+ (MSRV to be determined, targeting stable)
**Primary Dependencies**:
- `serde` + `serde_json`: JSON parsing (stdin input, API response)
- `ureq`: Blocking HTTP client (simpler than async for CLI, no tokio needed)
- `security-framework`: macOS Keychain access
- `chrono`: Timestamp parsing and timezone handling
- `clap`: CLI argument parsing (--configure flag)

**Storage**: Local config file at `~/.config/claude-status/config.json` (optional, for display preferences)
**Testing**: `cargo test` + `cargo test --all-features` (integration tests in `tests/`)
**Benchmarking**: criterion (for status line generation hot path)
**Target Platform**: macOS only (arm64 Apple Silicon + x86_64 Intel)
**Project Type**: Binary crate (single executable)
**Performance Goals**: <50ms total execution including API call, <5MB binary size
**Constraints**: No async runtime (blocking IO sufficient for single API call), static linking preferred

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                    | Compliance | Notes                                                                |
| ---------------------------- | ---------- | -------------------------------------------------------------------- |
| I. Simple, Not Easy          | ✓          | Modular: input → api → display. Each component deletable.            |
| II. Spec-Driven (SDD)        | ✓          | spec.md complete with Given/When/Then for all 6 user stories         |
| III. Test-Driven (TDD)       | ✓          | Test strategy: unit tests per module, integration for CLI, contract for output format |
| IV. Rust Best Practices      | ✓          | Custom error types planned, no .unwrap() in lib, Result propagation  |
| V. Correctness Through Types | ✓          | Domain types: UsagePercentage, SessionSize, GitStatus as enums       |

**Blockers**: None - all principles satisfied by design

## Project Structure

### Documentation (this feature)

```text
specs/001-rust-rewrite/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output
```

### Source Code (repository root)

```text
Cargo.toml
src/
├── main.rs              # Entry point, CLI parsing, error display to stderr
├── lib.rs               # Library root, public API
├── domain/              # Domain types (newtypes, enums)
│   ├── mod.rs
│   ├── usage.rs         # UsagePercentage, CycleInfo, WeeklyInfo
│   ├── session.rs       # SessionSize, SizeThreshold
│   ├── git.rs           # GitStatus, BranchInfo
│   └── input.rs         # ClaudeInput (stdin JSON)
├── api/                 # Anthropic API integration
│   ├── mod.rs
│   ├── client.rs        # HTTP client, request/response
│   └── keychain.rs      # macOS Keychain credential retrieval
├── display/             # Output formatting (status line, colors)
│   ├── mod.rs
│   ├── status_line.rs   # Main status line builder
│   └── colors.rs        # RGB color codes, threshold logic
├── git/                 # Git integration
│   ├── mod.rs
│   └── status.rs        # Branch, modified, ahead/behind detection
├── config/              # Configuration loading (optional preferences)
│   └── mod.rs
└── error.rs             # Custom error types

tests/
├── integration/         # End-to-end tests
│   ├── mod.rs
│   ├── happy_path.rs    # Valid input → formatted output
│   └── error_cases.rs   # Missing creds, API failures
└── contract/            # CLI contract tests
    ├── mod.rs
    └── output_format.rs # Status line format verification

benches/
└── status_line.rs       # Criterion benchmarks for output generation
```

**Structure Decision**: Single binary crate. The tool is a focused CLI with no reuse outside this context. Workspace complexity is unnecessary.

## Domain Types

| Concept              | Type                                                                              | Rationale                                    |
| -------------------- | --------------------------------------------------------------------------------- | -------------------------------------------- |
| Usage percentage     | `struct UsagePercentage(u8)` with `impl TryFrom<u8>`                              | Clamps to 0-100, prevents invalid values     |
| Session file size    | `struct SessionSize(u64)` (bytes)                                                 | Type-safe size, display methods for KB/MB    |
| Size threshold       | `enum SizeThreshold { Normal, Warning, Critical }`                                | Color coding states as types                 |
| Git status           | `enum GitStatus { NotRepo, Clean, Modified { branch, indicators } }`              | Invalid states unrepresentable               |
| API response         | `struct UsageResponse { five_hour: CycleInfo, seven_day: CycleInfo }`             | Typed API contract                           |
| Cycle info           | `struct CycleInfo { utilization: UsagePercentage, resets_at: DateTime<Utc> }`     | Groups related data                          |
| Input source         | `struct ClaudeInput { cwd, model, transcript_path, ... }`                         | Validated stdin JSON                         |
| Model name           | `enum Model { Sonnet4, Opus4, Haiku, Unknown(String) }`                           | Known models + fallback                      |
| Display color        | `struct RgbColor(u8, u8, u8)`                                                     | Type-safe RGB, no raw tuples                 |
| Credential           | `struct AccessToken(String)` (zeroize on drop)                                    | Security: clear from memory                  |

## Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum StatusLineError {
    #[error("Keychain credentials not found: authenticate with Claude Code first")]
    CredentialsNotFound,

    #[error("Keychain access denied: {0}")]
    KeychainAccess(String),

    #[error("API request failed: {0}")]
    ApiRequest(#[from] ureq::Error),

    #[error("API response invalid: {0}")]
    ApiResponse(String),

    #[error("Invalid JSON input: {0}")]
    InvalidInput(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git command failed: {0}")]
    Git(String),
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
        }
    }
}
```

## Complexity Tracking

| Violation                     | Why Needed                                      | Simpler Alternative Rejected Because                |
| ----------------------------- | ----------------------------------------------- | --------------------------------------------------- |
| `ureq` external crate         | HTTP client for API calls                       | std has no HTTP client; manual sockets too complex  |
| `security-framework` crate    | macOS Keychain access required                  | No std library for Keychain; FFI too error-prone    |
| `serde` + `serde_json`        | JSON parsing is core requirement                | Manual parsing would be fragile and verbose         |
| `.unwrap()` in main.rs        | CLI exits on fatal errors anyway                | Acceptable in binary entry point per constitution   |

---

## Constitution Re-Check (Post Phase 1 Design)

| Principle                    | Compliance | Verification                                                          |
| ---------------------------- | ---------- | --------------------------------------------------------------------- |
| I. Simple, Not Easy          | ✓          | 6 modules (domain, api, display, git, config, error) with clear boundaries |
| II. Spec-Driven (SDD)        | ✓          | data-model.md maps directly to spec entities                          |
| III. Test-Driven (TDD)       | ✓          | Contract tests defined in contracts/, test structure planned          |
| IV. Rust Best Practices      | ✓          | Error types use thiserror, domain types implement std traits          |
| V. Correctness Through Types | ✓          | 10 domain types defined, invalid states unrepresentable               |

**Status**: All gates passed. Ready for task generation (`/speckit.tasks`).

---

## Generated Artifacts

| Artifact | Path | Description |
|----------|------|-------------|
| Research | [research.md](./research.md) | Technology decisions, API contracts, performance analysis |
| Data Model | [data-model.md](./data-model.md) | Domain types with Rust code examples |
| Quickstart | [quickstart.md](./quickstart.md) | Build and development instructions |
| Input Contract | [contracts/stdin-input.json](./contracts/stdin-input.json) | JSON schema for Claude Code input |
| API Contract | [contracts/anthropic-usage-api.json](./contracts/anthropic-usage-api.json) | Anthropic OAuth API specification |
| Output Contract | [contracts/status-line-output.md](./contracts/status-line-output.md) | Status line format specification |
| Keychain Contract | [contracts/keychain-credentials.json](./contracts/keychain-credentials.json) | Keychain entry format |
