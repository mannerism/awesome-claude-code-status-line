---
description: "Task list template for Rust feature implementation with mandatory TDD"
---

# Tasks: [FEATURE NAME]

**Input**: Design documents from `/specs/[###-feature-name]/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**TDD Requirement**: Tests are MANDATORY per Constitution Principle III. Every implementation task MUST have a corresponding test task that runs FIRST.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions (Rust)

- **Binary crate**: `src/main.rs`, `src/lib.rs`, `src/*/mod.rs`
- **Library crate**: `src/lib.rs`, `src/*.rs`
- **Workspace**: `crates/*/src/`
- **Tests**: `tests/` for integration, `#[cfg(test)]` for unit tests
- **Benchmarks**: `benches/`
- Paths shown below assume single binary crate - adjust based on plan.md structure

<!--
  ============================================================================
  IMPORTANT: The tasks below are SAMPLE TASKS for illustration purposes only.

  The /speckit.tasks command MUST replace these with actual tasks based on:
  - User stories from spec.md (with their priorities P1, P2, P3...)
  - Feature requirements from plan.md
  - Domain types from plan.md
  - Error types from plan.md

  Tasks MUST follow TDD: test tasks BEFORE implementation tasks.
  Tasks MUST be organized by user story so each story can be:
  - Implemented independently
  - Tested independently
  - Delivered as an MVP increment

  DO NOT keep these sample tasks in the generated tasks.md file.
  ============================================================================
-->

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and Rust tooling setup

- [ ] T001 Initialize Cargo project: `cargo init` with correct crate type
- [ ] T002 Configure Cargo.toml: dependencies, features, metadata, MSRV
- [ ] T003 [P] Setup rustfmt.toml with project formatting rules
- [ ] T004 [P] Setup clippy.toml or .cargo/config.toml for lint rules
- [ ] T005 [P] Create CI workflow: fmt, clippy, test, doc

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and infrastructure that MUST be complete before ANY user story

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Tests First (Red Phase)

- [ ] T006 [P] Unit tests for domain types in src/domain.rs (`#[cfg(test)]` module)
- [ ] T007 [P] Unit tests for error types in src/error.rs (`#[cfg(test)]` module)

### Implementation (Green Phase)

- [ ] T008 Create domain types per plan.md Domain Types table in src/domain.rs
- [ ] T009 Create error types per plan.md Error Handling Strategy in src/error.rs
- [ ] T010 [P] Setup configuration loading in src/config.rs (if needed)
- [ ] T011 Verify: `cargo test` passes, `cargo clippy -- -D warnings` passes

**Checkpoint**: Foundation ready - `cargo test` green, all domain types compile

---

## Phase 3: User Story 1 - [Title] (Priority: P1) 🎯 MVP

**Goal**: [Brief description of what this story delivers]

**Independent Test**: `cargo test us1` should pass independently

### Tests First (Red Phase) ⚠️ MANDATORY

> **TDD: Write these tests FIRST. They MUST fail before implementation.**

- [ ] T012 [P] [US1] Unit test: [function] behavior in src/[module].rs
- [ ] T013 [P] [US1] Integration test: [user journey] in tests/integration/us1.rs
- [ ] T014 [US1] Verify tests fail: `cargo test us1` should have failures

### Implementation (Green Phase)

- [ ] T015 [P] [US1] Implement [Module1] in src/[module1].rs
- [ ] T016 [P] [US1] Implement [Module2] in src/[module2].rs
- [ ] T017 [US1] Wire modules together in src/lib.rs
- [ ] T018 [US1] Verify tests pass: `cargo test us1` should be green

### Refactor Phase

- [ ] T019 [US1] Refactor: simplify, remove duplication, improve naming
- [ ] T020 [US1] Verify: tests still green after refactor

**Checkpoint**: User Story 1 complete - `cargo test us1` green, `cargo clippy` clean

---

## Phase 4: User Story 2 - [Title] (Priority: P2)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: `cargo test us2` should pass independently

### Tests First (Red Phase) ⚠️ MANDATORY

- [ ] T021 [P] [US2] Unit test: [function] behavior in src/[module].rs
- [ ] T022 [P] [US2] Integration test: [user journey] in tests/integration/us2.rs
- [ ] T023 [US2] Verify tests fail: `cargo test us2` should have failures

### Implementation (Green Phase)

- [ ] T024 [P] [US2] Implement [Module] in src/[module].rs
- [ ] T025 [US2] Integrate with US1 components (if needed)
- [ ] T026 [US2] Verify tests pass: `cargo test us2` should be green

### Refactor Phase

- [ ] T027 [US2] Refactor: simplify, remove duplication
- [ ] T028 [US2] Verify: tests still green after refactor

**Checkpoint**: User Stories 1 AND 2 complete - both test suites green

---

## Phase 5: User Story 3 - [Title] (Priority: P3)

**Goal**: [Brief description of what this story delivers]

**Independent Test**: `cargo test us3` should pass independently

### Tests First (Red Phase) ⚠️ MANDATORY

- [ ] T029 [P] [US3] Unit test: [function] behavior in src/[module].rs
- [ ] T030 [P] [US3] Integration test: [user journey] in tests/integration/us3.rs
- [ ] T031 [US3] Verify tests fail: `cargo test us3` should have failures

### Implementation (Green Phase)

- [ ] T032 [P] [US3] Implement [Module] in src/[module].rs
- [ ] T033 [US3] Verify tests pass: `cargo test us3` should be green

### Refactor Phase

- [ ] T034 [US3] Refactor: simplify, remove duplication
- [ ] T035 [US3] Verify: tests still green after refactor

**Checkpoint**: All user stories complete - full test suite green

---

[Add more user story phases as needed, following the same TDD pattern]

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates and final verification

- [ ] TXXX Full test suite: `cargo test --all-features`
- [ ] TXXX Lint check: `cargo clippy -- -D warnings`
- [ ] TXXX Format check: `cargo fmt --check`
- [ ] TXXX Documentation: `cargo doc --no-deps` with no warnings
- [ ] TXXX [P] Benchmarks: `cargo bench` (if performance-critical)
- [ ] TXXX [P] Property tests: proptest for parsing/calculation logic
- [ ] TXXX Run quickstart.md validation
- [ ] TXXX Update CLAUDE.md if architecture changed

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational completion
  - User stories can proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### TDD Order Within Each User Story

```text
┌─────────────────────────────────────────────────────────────┐
│  1. Write failing tests (Red)                                │
│     └─> Tests must compile but FAIL                         │
│  2. Verify failure: `cargo test` shows failures              │
│  3. Write minimal implementation (Green)                     │
│     └─> Only enough code to pass tests                      │
│  4. Verify success: `cargo test` all green                   │
│  5. Refactor (keep tests green)                              │
│     └─> Simplify, extract, rename - tests must stay green   │
└─────────────────────────────────────────────────────────────┘
```

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational test tasks marked [P] can run in parallel
- Once Foundational completes, user stories can start in parallel
- Within a story: test tasks [P] can run in parallel
- Within a story: implementation tasks [P] can run in parallel (after tests written)

---

## Parallel Example: User Story 1

```bash
# TDD Step 1: Write all tests in parallel (Red phase)
Task: "Unit test: [function] in src/[module].rs"
Task: "Integration test: [journey] in tests/integration/us1.rs"

# TDD Step 2: Verify tests fail
cargo test us1  # Should show failures

# TDD Step 3: Implementation in parallel (Green phase)
Task: "Implement [Module1] in src/[module1].rs"
Task: "Implement [Module2] in src/[module2].rs"

# TDD Step 4: Verify tests pass
cargo test us1  # Should be green

# TDD Step 5: Refactor
# Only after tests are green
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1 (full TDD cycle)
4. **STOP and VALIDATE**: `cargo test`, `cargo clippy`, `cargo doc`
5. Deploy/demo if ready

### Incremental Delivery

1. Setup + Foundational → Foundation ready
2. User Story 1 (TDD) → Test green → Deploy/Demo (MVP!)
3. User Story 2 (TDD) → Test green → Deploy/Demo
4. User Story 3 (TDD) → Test green → Deploy/Demo
5. Each story adds value without breaking previous stories

---

## TDD Verification Checklist

Before marking a user story complete, verify:

- [ ] All tests were written BEFORE implementation
- [ ] Tests failed before implementation (Red phase documented)
- [ ] Implementation is minimal - no speculative features
- [ ] `cargo test` passes for this story
- [ ] `cargo clippy -- -D warnings` passes
- [ ] No `.unwrap()` in library code (use `?` operator)
- [ ] Domain types used (no primitive obsession)
- [ ] Error handling uses custom error types

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- TDD is MANDATORY - tests before implementation, always
- Verify tests FAIL before implementing (proves test validity)
- Commit after each TDD phase (Red commit, Green commit, Refactor commit)
- Stop at any checkpoint to validate story independently
- Avoid: implementation before tests, .unwrap() in lib, primitive obsession
