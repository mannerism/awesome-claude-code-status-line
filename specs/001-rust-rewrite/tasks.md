# Tasks: Claude Code Status Line - Rust Rewrite

**Input**: Design documents from `/specs/001-rust-rewrite/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**TDD Requirement**: Tests are MANDATORY per Constitution Principle III. Every implementation task MUST have a corresponding test task that runs FIRST.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions (Rust - Single Binary Crate)

- **Binary**: `src/main.rs`
- **Library**: `src/lib.rs`
- **Modules**: `src/domain/`, `src/api/`, `src/display/`, `src/git/`, `src/config/`
- **Error Types**: `src/error.rs`
- **Tests**: `tests/integration/`, `tests/contract/`
- **Benchmarks**: `benches/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and Rust tooling setup

- [x] T001 Initialize Cargo project with `cargo init --name claude-status` at repository root
- [x] T002 Configure Cargo.toml with dependencies per research.md: serde, serde_json, ureq, security-framework, chrono, clap, thiserror
- [x] T003 Configure Cargo.toml [profile.release] with optimization settings: opt-level="z", lto="fat", codegen-units=1, strip="symbols", panic="abort"
- [x] T004 [P] Create .cargo/config.toml with macOS cross-compilation settings for arm64 and x86_64 targets
- [x] T005 [P] Create rustfmt.toml with project formatting rules (max_width=100, edition=2021)
- [x] T006 [P] Create .github/workflows/ci.yml with fmt, clippy, test, doc steps

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types and infrastructure that MUST be complete before ANY user story

**CRITICAL**: No user story work can begin until this phase is complete

### Tests First (Red Phase)

- [x] T007 [P] Write unit tests for RgbColor type (new, constants, to_ansi) in src/display/colors.rs
- [x] T008 [P] Write unit tests for UsagePercentage type (new, from_float, threshold) in src/domain/usage.rs
- [x] T009 [P] Write unit tests for UsageThreshold enum (color mapping) in src/domain/usage.rs
- [x] T010 [P] Write unit tests for SessionSize type (new, from_file mock, threshold, format_display) in src/domain/session.rs
- [x] T011 [P] Write unit tests for SizeThreshold enum (color, indicator) in src/domain/session.rs
- [x] T012 [P] Write unit tests for Model enum (from_display_name, short_name) in src/domain/input.rs
- [x] T013 [P] Write unit tests for ClaudeInput struct (deserialize with defaults) in src/domain/input.rs
- [x] T014 [P] Write unit tests for StatusLineError (brief method) in src/error.rs
- [x] T015 Verify tests fail: `cargo test` should show compilation errors or failures

### Implementation (Green Phase)

- [x] T016 Create src/lib.rs with module declarations: domain, api, display, git, config, error
- [x] T017 Create src/domain/mod.rs exporting submodules: usage, session, input, git
- [x] T018 [P] Implement RgbColor struct in src/display/colors.rs per data-model.md
- [x] T019 [P] Implement UsagePercentage and UsageThreshold types in src/domain/usage.rs per data-model.md
- [x] T020 [P] Implement CycleInfo struct in src/domain/usage.rs per data-model.md
- [x] T021 [P] Implement SessionSize and SizeThreshold types in src/domain/session.rs per data-model.md
- [x] T022 [P] Implement Model enum in src/domain/input.rs per data-model.md
- [x] T023 [P] Implement ClaudeInput, ModelInfo, ContextWindow structs in src/domain/input.rs per data-model.md
- [x] T024 [P] Implement StatusLineError enum with thiserror in src/error.rs per plan.md
- [x] T025 Create src/display/mod.rs exporting submodules: colors, status_line
- [x] T026 Create src/api/mod.rs exporting submodules: client, keychain
- [x] T027 Create src/git/mod.rs exporting submodules: status
- [x] T028 Create src/config/mod.rs with placeholder for configuration loading
- [x] T029 Verify tests pass: `cargo test` should be green
- [x] T030 Verify lints pass: `cargo clippy -- -D warnings` should be clean

**Checkpoint**: Foundation ready - all domain types compile and pass tests

---

## Phase 3: User Story 1 - View Usage Status Line (Priority: P1) MVP

**Goal**: Display usage statistics (5-hour and weekly percentages) from Anthropic API with color coding

**Independent Test**: `cargo test us1` should pass independently

### Tests First (Red Phase) MANDATORY

- [x] T031 [P] [US1] Write unit tests for AccessToken type (redacted debug) in src/api/keychain.rs
- [ ] T032 [P] [US1] Write unit tests for keychain credential retrieval (mock security framework) in src/api/keychain.rs
- [x] T033 [P] [US1] Write unit tests for UsageResponse and ApiCycleInfo deserialization in src/api/client.rs
- [ ] T034 [P] [US1] Write unit tests for API client fetch_usage function (mock HTTP) in src/api/client.rs
- [x] T035 [P] [US1] Write unit tests for status line builder (format output with colors) in src/display/status_line.rs
- [x] T036 [P] [US1] Write integration test for happy path: valid input → formatted output in tests/integration/happy_path.rs
- [x] T037 [P] [US1] Write integration test for error cases: no creds, API error in tests/integration/error_cases.rs
- [x] T038 [US1] Verify tests fail: `cargo test us1` should have failures (feature flag: `--features us1`)

### Implementation (Green Phase)

- [x] T039 [P] [US1] Implement AccessToken struct with zeroize in src/api/keychain.rs per data-model.md
- [x] T040 [P] [US1] Implement get_access_token() using security-framework in src/api/keychain.rs per research.md
- [x] T041 [P] [US1] Implement UsageResponse, ApiCycleInfo structs in src/api/client.rs per contracts/anthropic-usage-api.json
- [x] T042 [P] [US1] Implement fetch_usage() HTTP client using ureq in src/api/client.rs per research.md
- [x] T043 [P] [US1] Implement StatusLineBuilder struct in src/display/status_line.rs with project_name, model, usage fields
- [x] T044 [US1] Implement StatusLineBuilder::build() method to format output string with ANSI colors in src/display/status_line.rs
- [x] T045 [US1] Create src/main.rs: read stdin JSON, call API, build status line, handle errors to stderr
- [x] T046 [US1] Wire error handling: brief() to stdout, detailed to stderr in src/main.rs
- [x] T047 [US1] Verify tests pass: `cargo test us1` should be green

### Refactor Phase

- [x] T048 [US1] Refactor: extract common formatting logic, improve error messages
- [x] T049 [US1] Verify: tests still green, clippy clean after refactor

**Checkpoint**: User Story 1 complete - can display API usage with colors

---

## Phase 4: User Story 2 - Session Size Monitoring (Priority: P1)

**Goal**: Display session file size with color-coded warnings (green <5MB, yellow 5-15MB, red >15MB)

**Independent Test**: `cargo test us2` should pass independently

### Tests First (Red Phase) MANDATORY

- [x] T050 [P] [US2] Write unit tests for SessionSize::from_file() in src/domain/session.rs
- [x] T051 [P] [US2] Write unit tests for SizeThreshold indicator() method in src/domain/session.rs
- [x] T052 [P] [US2] Write unit tests for status line with session size in src/display/status_line.rs
- [x] T053 [P] [US2] Write integration test: valid transcript_path → size displayed in tests/integration/session_size.rs
- [x] T054 [P] [US2] Write integration test: missing transcript_path → size omitted in tests/integration/session_size.rs
- [x] T055 [US2] Verify tests fail: `cargo test us2` should have failures

### Implementation (Green Phase)

- [x] T056 [P] [US2] Implement SessionSize::from_file() with std::fs::metadata in src/domain/session.rs
- [x] T057 [P] [US2] Implement SizeThreshold::indicator() returning emoji strings in src/domain/session.rs
- [x] T058 [US2] Extend StatusLineBuilder to accept optional session_size field in src/display/status_line.rs
- [x] T059 [US2] Update StatusLineBuilder::build() to include session size with color and indicator in src/display/status_line.rs
- [x] T060 [US2] Update src/main.rs to read transcript_path from input and display session size
- [x] T061 [US2] Verify tests pass: `cargo test us2` should be green

### Refactor Phase

- [x] T062 [US2] Refactor: consolidate threshold color logic between UsageThreshold and SizeThreshold
- [x] T063 [US2] Verify: tests still green after refactor

**Checkpoint**: User Stories 1 AND 2 complete - API usage + session size monitoring

---

## Phase 5: User Story 3 - Fast Startup Performance (Priority: P2)

**Goal**: Ensure total execution time <50ms including API call

**Independent Test**: `cargo bench` and manual timing verification

### Tests First (Red Phase) MANDATORY

- [x] T064 [P] [US3] Create benchmark for status line generation in benches/status_line.rs using criterion
- [x] T065 [P] [US3] Create benchmark for JSON parsing in benches/parsing.rs
- [x] T066 [US3] Establish baseline: run `cargo bench` to measure current performance

### Implementation (Green Phase)

- [ ] T067 [P] [US3] Optimize JSON parsing: use serde's zero-copy where possible in src/domain/input.rs
- [x] T068 [P] [US3] Set HTTP timeout to 5 seconds in src/api/client.rs to fail fast
- [x] T069 [US3] Verify benchmark: `cargo bench` should show <50ms p99 latency
- [ ] T070 [US3] Add timing instrumentation in debug builds for profiling in src/main.rs

### Refactor Phase

- [x] T071 [US3] Profile and optimize hot paths identified by benchmarks
- [x] T072 [US3] Verify: benchmarks still meet target after refactor

**Checkpoint**: Performance validated - <50ms total execution

---

## Phase 6: User Story 4 - Cross-Platform Distribution (Priority: P2)

**Goal**: Pre-built binaries for macOS arm64 and x86_64 that run without dependencies

**Independent Test**: Binary runs on target architecture without errors

### Tests First (Red Phase) MANDATORY

- [x] T073 [P] [US4] Write contract test verifying binary accepts stdin JSON in tests/contract/cli_input.rs
- [x] T074 [P] [US4] Write contract test verifying binary outputs valid status line format in tests/contract/output_format.rs
- [x] T075 [US4] Verify tests pass with release build: `cargo test --release`

### Implementation (Green Phase)

- [x] T076 [P] [US4] Create scripts/build-release.sh for building arm64 and x86_64 binaries
- [x] T077 [P] [US4] Create scripts/create-universal.sh using lipo to combine architectures
- [x] T078 [P] [US4] Create scripts/install.sh to copy binary to ~/.local/bin/ and update Claude Code settings
- [x] T079 [US4] Test arm64 binary on Apple Silicon: build and run with sample input
- [ ] T080 [US4] Test x86_64 binary on Intel (or Rosetta): build and run with sample input
- [x] T081 [US4] Verify binary size: `ls -lh target/release/claude-status` should be <5MB per arch

### Refactor Phase

- [x] T082 [US4] Document build and install process in README.md
- [x] T083 [US4] Verify: all contract tests pass on both architectures

**Checkpoint**: Distribution ready - universal binary <5MB, runs without dependencies

---

## Phase 7: User Story 5 - Git Integration Display (Priority: P3)

**Goal**: Display current branch and status indicators (modified, untracked, ahead/behind)

**Independent Test**: `cargo test us5` should pass independently

### Tests First (Red Phase) MANDATORY

- [x] T084 [P] [US5] Write unit tests for GitStatus and GitRepoStatus types in src/domain/git.rs
- [x] T085 [P] [US5] Write unit tests for BranchInfo enum in src/domain/git.rs
- [x] T086 [P] [US5] Write unit tests for git status detection (mock git commands) in src/git/status.rs
- [x] T087 [P] [US5] Write integration test: in git repo → branch displayed in tests/integration/git_status.rs
- [x] T088 [P] [US5] Write integration test: not in git repo → git info omitted in tests/integration/git_status.rs
- [x] T089 [US5] Verify tests fail: `cargo test us5` should have failures

### Implementation (Green Phase)

- [x] T090 [P] [US5] Implement GitStatus, GitRepoStatus, BranchInfo types in src/domain/git.rs per data-model.md
- [x] T091 [P] [US5] Implement get_git_status() using std::process::Command in src/git/status.rs
- [x] T092 [US5] Implement git branch detection: `git symbolic-ref --short HEAD` or `git rev-parse --short HEAD` in src/git/status.rs
- [x] T093 [US5] Implement git status parsing: `git status --porcelain=v1` for modified/untracked in src/git/status.rs
- [x] T094 [US5] Implement ahead/behind detection: `git rev-list --left-right --count @{upstream}...HEAD` in src/git/status.rs
- [x] T095 [US5] Extend StatusLineBuilder to accept optional git_status field in src/display/status_line.rs
- [x] T096 [US5] Update StatusLineBuilder::build() to include git info with indicators in src/display/status_line.rs
- [x] T097 [US5] Update src/main.rs to detect git status and include in output
- [x] T098 [US5] Verify tests pass: `cargo test us5` should be green

### Refactor Phase

- [x] T099 [US5] Refactor: handle git command timeouts gracefully (1s timeout)
- [x] T100 [US5] Verify: tests still green after refactor

**Checkpoint**: Git integration complete - branch and status indicators displayed

---

## Phase 8: User Story 6 - Display Preferences (Priority: P3)

**Goal**: Optional configuration for display preferences (e.g., timezone for reset times)

**Independent Test**: `cargo test us6` should pass independently

### Tests First (Red Phase) MANDATORY

- [x] T101 [P] [US6] Write unit tests for config file loading/saving in src/config/mod.rs
- [x] T102 [P] [US6] Write unit tests for --configure CLI flag parsing in src/main.rs
- [ ] T103 [P] [US6] Write integration test: --configure saves preferences in tests/integration/config.rs
- [x] T104 [US6] Verify tests fail: `cargo test us6` should have failures

### Implementation (Green Phase)

- [x] T105 [P] [US6] Implement Config struct with display preferences in src/config/mod.rs
- [x] T106 [P] [US6] Implement config file path (~/.config/claude-status/config.json) in src/config/mod.rs
- [x] T107 [US6] Implement Config::load() and Config::save() in src/config/mod.rs
- [x] T108 [US6] Add clap CLI parsing for --configure flag in src/main.rs
- [ ] T109 [US6] Implement interactive configuration prompts for --configure mode in src/main.rs
- [x] T110 [US6] Verify tests pass: `cargo test us6` should be green

### Refactor Phase

- [x] T111 [US6] Refactor: use sensible defaults when config file missing
- [x] T112 [US6] Verify: tests still green after refactor

**Checkpoint**: Configuration complete - preferences persist between runs

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Quality gates and final verification

- [x] T113 Full test suite: `cargo test --all-features`
- [x] T114 Lint check: `cargo clippy -- -D warnings`
- [x] T115 Format check: `cargo fmt --check`
- [x] T116 Documentation: `cargo doc --no-deps` with no warnings
- [x] T117 [P] Add doc comments to all public items in src/lib.rs
- [x] T118 [P] Run benchmarks: `cargo bench` and document results
- [x] T119 [P] Update quickstart.md with final build and install instructions
- [x] T120 Update CLAUDE.md with Rust architecture and build commands
- [x] T121 Final verification: run binary with real Claude Code input and verify output matches spec

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1: Setup ──────────────────────────────────────────┐
                                                          │
Phase 2: Foundational ◄───────────────────────────────────┘
     │
     ├──► Phase 3: US1 (API Usage) ──► MVP Checkpoint
     │         │
     │         ▼
     ├──► Phase 4: US2 (Session Size) ──► Full P1 Checkpoint
     │
     ├──► Phase 5: US3 (Performance) ◄── Can start after US1
     │
     ├──► Phase 6: US4 (Distribution) ◄── Can start after US1
     │
     ├──► Phase 7: US5 (Git) ◄── Can start after US1
     │
     └──► Phase 8: US6 (Config) ◄── Can start after US1
                                    │
Phase 9: Polish ◄───────────────────┘ (after all desired stories)
```

### Story Independence

| Story | Dependencies | Can Start After |
|-------|--------------|-----------------|
| US1 | Foundational | Phase 2 complete |
| US2 | US1 (extends StatusLineBuilder) | US1 green tests |
| US3 | US1 (benchmarks the full path) | US1 green tests |
| US4 | US1 (distributes working binary) | US1 green tests |
| US5 | US1 (extends StatusLineBuilder) | US1 green tests |
| US6 | US1 (adds config to main) | US1 green tests |

### Parallel Opportunities

**Phase 1 (Setup)**: T004, T005, T006 can run in parallel

**Phase 2 (Foundational)**: T007-T014 tests can run in parallel; T018-T024 implementations can run in parallel

**Phase 3+ (User Stories)**: All test tasks within a story can run in parallel; implementation tasks [P] can run in parallel

**Cross-Story**: US3, US4, US5, US6 can all proceed in parallel after US1 is complete (different files/concerns)

---

## Parallel Example: User Story 1

```bash
# TDD Step 1: Write all tests in parallel (Red phase)
# These 7 tasks can run simultaneously:
T031: Unit tests for AccessToken
T032: Unit tests for keychain retrieval
T033: Unit tests for API response deserialization
T034: Unit tests for HTTP client
T035: Unit tests for status line builder
T036: Integration test happy path
T037: Integration test error cases

# TDD Step 2: Verify tests fail
cargo test us1  # Should show failures

# TDD Step 3: Implementation in parallel (Green phase)
# These tasks can run simultaneously:
T039: AccessToken struct
T040: keychain retrieval
T041: API response types
T042: HTTP client
T043: StatusLineBuilder struct

# TDD Step 4: Sequential wiring
T044: StatusLineBuilder::build()
T045: main.rs integration
T046: Error handling

# TDD Step 5: Verify tests pass
cargo test us1  # Should be green

# TDD Step 6: Refactor (tests must stay green)
T048, T049
```

---

## Implementation Strategy

### MVP First (User Story 1 + 2)

1. Complete Phase 1: Setup (T001-T006)
2. Complete Phase 2: Foundational (T007-T030)
3. Complete Phase 3: User Story 1 - API Usage (T031-T049)
4. **STOP and VALIDATE**: `cargo test`, `cargo clippy`, working binary
5. Complete Phase 4: User Story 2 - Session Size (T050-T063)
6. **MVP COMPLETE**: Both P1 stories done, deploy/demo ready

### Incremental Delivery

| Milestone | Stories Complete | Deliverable |
|-----------|------------------|-------------|
| Foundation | - | Types compile, tests pass |
| MVP | US1 | API usage with colors |
| Full P1 | US1 + US2 | + Session size monitoring |
| Performance | + US3 | Verified <50ms |
| Distribution | + US4 | Universal binary |
| Git | + US5 | Branch/status display |
| Config | + US6 | Persistent preferences |
| 1.0 | All | Production ready |

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
- [ ] Error handling uses StatusLineError type

---

## Notes

- [P] tasks = different files, no dependencies, can run in parallel
- [Story] label maps task to specific user story for traceability
- TDD is MANDATORY per Constitution Principle III
- Verify tests FAIL before implementing (proves test validity)
- Commit after each TDD phase (Red commit, Green commit, Refactor commit)
- US1 + US2 = MVP (both are P1 priority)
- US3-US6 can proceed in parallel after US1 is green
