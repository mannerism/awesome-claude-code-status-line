<!--
Sync Impact Report:
- Version change: 1.0.0 → 2.0.0
- Modified principles:
  - "I. Performance First" → "I. Simple, Not Easy"
  - "II. Accuracy Over Estimation" → "II. Spec-Driven Development (SDD)"
  - "III. User Experience" → "III. Test-Driven Development (TDD)"
  - "IV. Simplicity & Maintainability" → "IV. Rust Best Practices"
  - "V. Cross-Platform Compatibility" → "V. Correctness Through Types"
- Added sections:
  - Philosophy section (Simple vs Easy distinction)
  - SDD/TDD workflow integration
  - Rust-specific quality gates
- Removed sections:
  - Python-specific requirements (numpy, type hints)
  - Python tooling references (ruff, pathlib)
- Templates requiring updates:
  - .specify/templates/plan-template.md ✅ updated (Rust tooling, constitution check table, domain types)
  - .specify/templates/spec-template.md ✅ (no updates needed - SDD compatible)
  - .specify/templates/tasks-template.md ✅ updated (mandatory TDD, Rust paths, Red-Green-Refactor phases)
- Follow-up TODOs: None
-->

# Claude Code Usage Tracker Constitution

## Philosophy: Simple vs Easy

**Simple** and **easy** are fundamentally different concepts. This project chooses **simple**.

**Simple** means:

- Eloquent code with clear, well-structured architecture
- Each component has one purpose and does it well
- Adding, editing, or deleting features becomes easier over time
- The codebase improves with age; new code flows naturally from existing patterns
- Complexity is pushed to compile-time, not runtime

**Easy** means (and we reject this):

- Quick hacks that "just work" today
- Shortcuts that create hidden coupling
- Code that breaks with library updates or new requirements
- Spaghetti architecture where changes ripple unpredictably
- Technical debt that compounds until the codebase becomes unmaintainable

**The test**: If adding a feature today is harder than it was six months ago, we have failed.

## Core Principles

### I. Simple, Not Easy

Every design decision MUST favor long-term simplicity over short-term convenience:

- Architecture MUST be modular: clear boundaries between parsing, tracking, display, and config
- Dependencies MUST be minimal and stable; prefer std library over external crates when feasible
- Abstractions MUST emerge from duplication, never be created speculatively
- Code MUST be deletable: removing a feature should not require understanding the entire codebase

**Rationale**: Sustainable velocity. A simple codebase accelerates development; an easy codebase
decelerates it. We optimize for the 100th feature, not the first.

### II. Spec-Driven Development (SDD)

All features MUST begin with a specification before any code is written:

- User stories MUST be captured in spec.md with acceptance scenarios (Given/When/Then)
- Requirements MUST use precise language (MUST/SHOULD/MAY per RFC 2119)
- Edge cases MUST be enumerated before implementation begins
- The spec is the contract; implementation is the fulfillment

**Workflow**:

1. `/speckit.specify` → Create feature specification
2. `/speckit.clarify` → Resolve ambiguities
3. `/speckit.plan` → Design implementation approach
4. `/speckit.tasks` → Generate task breakdown
5. Only then: write code

**Rationale**: Specifications prevent scope creep, clarify thinking, and create testable contracts.
Code without a spec is a guess; guesses accumulate into chaos.

### III. Test-Driven Development (TDD)

All implementation MUST follow the Red-Green-Refactor cycle:

- Tests MUST be written BEFORE implementation code
- Tests MUST fail before implementation (Red phase proves test validity)
- Implementation MUST be minimal to pass tests (Green phase)
- Refactoring MUST not change behavior (Refactor phase cleans up)
- No code ships without corresponding tests

**Test Categories**:

- **Unit tests**: Pure functions, isolated logic (`#[test]` in same file or `tests/unit/`)
- **Integration tests**: Module boundaries, file I/O (`tests/integration/`)
- **Contract tests**: API stability, CLI behavior (`tests/contract/`)

**Rationale**: TDD produces testable designs by construction. Tests written after implementation
test the code we wrote, not the code we need.

### IV. Rust Best Practices

All code MUST follow idiomatic Rust patterns:

- **Ownership**: Prefer borrowing over cloning; use `&str` over `String` in function parameters
- **Error handling**: Use `Result<T, E>` with custom error types; no `.unwrap()` in library code
- **Enums over booleans**: Model states explicitly (`enum Status { Active, Paused, Stopped }`)
- **Iterators over loops**: Prefer `.iter().map().filter()` chains over manual iteration
- **Derive what you can**: `#[derive(Debug, Clone, PartialEq)]` for data types
- **Documentation**: `///` doc comments on all public items; examples in doc comments run as tests

**Forbidden Patterns**:

- `unsafe` blocks without documented safety invariants
- `.unwrap()` or `.expect()` in library code (use `?` operator)
- `clone()` to satisfy the borrow checker without understanding why
- Stringly-typed APIs (use newtypes: `struct UserId(u64)`)
- Mutable global state

**Rationale**: Rust's type system encodes correctness. Fighting the borrow checker indicates
a design flaw; the compiler is usually right.

### V. Correctness Through Types

The type system MUST encode business rules and prevent invalid states:

- Invalid states MUST be unrepresentable at compile time
- Domain concepts MUST have dedicated types (not raw primitives)
- State machines MUST use enums with associated data
- Parse, don't validate: transform unstructured input into typed structures at boundaries

**Examples**:

```rust
// BAD: Primitive obsession
fn calculate_usage(hours: f64, limit: f64) -> f64

// GOOD: Domain types
fn calculate_usage(hours: SessionHours, limit: WeeklyLimit) -> UsagePercentage

// BAD: Impossible state representable
struct Cycle { started_at: Option<Timestamp>, ended_at: Option<Timestamp> }

// GOOD: States encoded in types
enum Cycle {
    NotStarted,
    Active { started_at: Timestamp },
    Completed { started_at: Timestamp, ended_at: Timestamp },
}
```

**Rationale**: A bug that cannot compile cannot ship. Move validation from runtime to compile-time.

## Quality Standards

All code contributions MUST meet these standards:

**Testing Requirements**:

- Test coverage MUST include all public functions
- Property-based tests (proptest) SHOULD be used for parsing and calculation logic
- Benchmarks (`criterion`) MUST exist for performance-critical paths
- All tests MUST pass before merge: `cargo test --all-features`

**Code Quality**:

- `cargo fmt` MUST produce no changes (formatting is automated)
- `cargo clippy -- -D warnings` MUST pass (all warnings are errors)
- `cargo doc --no-deps` MUST succeed with no warnings
- MSRV (Minimum Supported Rust Version) MUST be documented and tested in CI

**Documentation**:

- README MUST include build instructions and usage examples
- CLAUDE.md MUST stay synchronized with architectural decisions
- Public API MUST have doc comments with examples
- Architecture decisions MUST be recorded (ADR format recommended)

## Development Workflow

**Branch Strategy**:

- All features MUST be developed on feature branches
- Branch names MUST follow pattern: `<type>/<description>` (e.g., `feat/opus-tracking`)
- Main branch MUST always be in a deployable state

**TDD Workflow**:

1. Write failing test that captures requirement from spec
2. Verify test fails for the right reason
3. Write minimal code to pass test
4. Refactor while keeping tests green
5. Repeat until feature complete

**Review Process**:

- All PRs MUST include a constitution compliance statement
- Tests MUST exist before implementation code in commit history (or same commit)
- Performance-affecting changes MUST include benchmark comparisons
- Breaking changes MUST document migration path

**Release Process**:

- Version numbers follow semantic versioning (MAJOR.MINOR.PATCH)
- Breaking changes trigger MAJOR version bump
- New features trigger MINOR version bump
- Bug fixes trigger PATCH version bump
- Releases MUST pass full test suite including integration tests

## Governance

This constitution is **binding** for all development on the Claude Code Usage Tracker.

**Enforcement**:

- All pull requests MUST verify compliance with each principle before merge
- Violations MUST be documented with explicit justification if accepted
- Code reviewers MUST check constitution compliance as part of review
- CI MUST enforce: formatting, linting, tests, documentation

**Amendment Process**:

1. Propose change via pull request modifying this document
2. Document rationale for the change
3. Update version number according to semantic versioning
4. Update LAST_AMENDED_DATE to current date
5. Propagate changes to dependent templates if principle names or requirements change

**Compliance Review**:

- Monthly review of merged PRs for constitution adherence
- Quarterly review of constitution relevance and principle effectiveness
- Annual review of governance strictness level

**Version**: 2.0.0 | **Ratified**: 2025-01-29 | **Last Amended**: 2025-01-29
