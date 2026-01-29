# Implementation Plan: [FEATURE]

**Branch**: `[###-feature-name]` | **Date**: [DATE] | **Spec**: [link]
**Input**: Feature specification from `/specs/[###-feature-name]/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

[Extract from feature spec: primary requirement + technical approach from research]

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.75+ (or specify MSRV)
**Primary Dependencies**: [e.g., serde, clap, tokio, chrono or NEEDS CLARIFICATION]
**Storage**: [if applicable, e.g., SQLite, files, sled or N/A]
**Testing**: `cargo test` + `cargo test --all-features` (integration tests in `tests/`)
**Benchmarking**: criterion (for performance-critical paths)
**Target Platform**: [e.g., Linux/macOS/Windows CLI, or NEEDS CLARIFICATION]
**Project Type**: [binary/library/workspace - determines Cargo.toml structure]
**Performance Goals**: [domain-specific, e.g., <100ms status line, 10k msg/sec or NEEDS CLARIFICATION]
**Constraints**: [domain-specific, e.g., zero-copy parsing, no allocations in hot path or NEEDS CLARIFICATION]

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                    | Compliance | Notes                                                    |
| ---------------------------- | ---------- | -------------------------------------------------------- |
| I. Simple, Not Easy          | ☐          | Modular boundaries defined? Deletable code?              |
| II. Spec-Driven (SDD)        | ☐          | spec.md complete with Given/When/Then?                   |
| III. Test-Driven (TDD)       | ☐          | Test strategy defined? Red-Green-Refactor planned?       |
| IV. Rust Best Practices      | ☐          | No .unwrap() in lib? Custom error types?                 |
| V. Correctness Through Types | ☐          | Domain types identified? Invalid states unrepresentable? |

**Blockers**: [List any principle violations that must be resolved before proceeding]

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths. The delivered plan must not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single binary crate (DEFAULT for CLI tools)
Cargo.toml
src/
├── main.rs              # Entry point, CLI parsing
├── lib.rs               # Library root, public API
├── domain/              # Domain types (newtypes, enums)
│   └── mod.rs
├── parsing/             # Input parsing (JSONL, config files)
│   └── mod.rs
├── tracking/            # Core business logic
│   └── mod.rs
├── display/             # Output formatting (status line, colors)
│   └── mod.rs
└── config/              # Configuration loading
    └── mod.rs

tests/
├── integration/         # End-to-end tests
│   └── mod.rs
└── contract/            # CLI contract tests
    └── mod.rs

benches/
└── tracking.rs          # Criterion benchmarks

# [REMOVE IF UNUSED] Option 2: Library crate (when building reusable library)
Cargo.toml
src/
├── lib.rs               # Library root, public API only
├── domain.rs            # Public domain types
├── error.rs             # Custom error types
└── internal/            # Private implementation
    └── mod.rs

tests/
└── integration.rs       # Library integration tests

# [REMOVE IF UNUSED] Option 3: Cargo workspace (when multiple related crates)
Cargo.toml               # [workspace] members = ["crates/*"]
crates/
├── core/                # Shared domain types and logic
│   ├── Cargo.toml
│   └── src/lib.rs
├── parser/              # Parsing logic
│   ├── Cargo.toml
│   └── src/lib.rs
└── cli/                 # CLI binary
    ├── Cargo.toml
    └── src/main.rs
```

**Structure Decision**: [Document the selected structure and reference the real
directories captured above]

## Domain Types

<!--
  ACTION REQUIRED: List the domain types that will be created following
  "Correctness Through Types" principle. Each domain concept should have
  a dedicated type, not a raw primitive.
-->

| Concept                   | Type                                                                  | Rationale                               |
| ------------------------- | --------------------------------------------------------------------- | --------------------------------------- |
| [e.g., Session duration]  | `struct SessionHours(f64)`                                            | Prevents mixing with other f64 values   |
| [e.g., Usage cycle state] | `enum Cycle { NotStarted, Active { started_at }, Completed { ... } }` | Invalid states unrepresentable          |
| [e.g., Prompt count]      | `struct PromptCount(u32)`                                             | Type-safe, non-negative by construction |

## Error Handling Strategy

<!--
  ACTION REQUIRED: Define custom error types following Rust best practices.
  Use thiserror for library errors, anyhow for application errors.
-->

```rust
// Example error type structure
#[derive(Debug, thiserror::Error)]
pub enum TrackerError {
    #[error("Failed to parse JSONL at line {line}: {source}")]
    ParseError { line: usize, source: serde_json::Error },

    #[error("Configuration file not found: {path}")]
    ConfigNotFound { path: PathBuf },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation                    | Why Needed                | Simpler Alternative Rejected Because         |
| ---------------------------- | ------------------------- | -------------------------------------------- |
| [e.g., .unwrap() in main.rs] | CLI exits on error anyway | N/A - acceptable in binary crate entry point |
| [e.g., External crate X]     | [specific need]           | [why std library insufficient]               |
