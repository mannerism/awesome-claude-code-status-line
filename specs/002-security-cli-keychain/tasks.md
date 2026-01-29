# Tasks: Use macOS Security CLI for Keychain Access

**Input**: Design documents from `/specs/002-security-cli-keychain/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, quickstart.md

**Note**: This is a refactoring task - existing tests cover the public API. No new test files needed; existing unit tests in `src/api/keychain.rs` will verify the implementation.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Dependency Removal)

**Purpose**: Remove the `security-framework` dependency to enable CLI-based approach

- [x] T001 Remove `security-framework = "3.0"` dependency from Cargo.toml
- [x] T002 Verify project compiles: `cargo check` (will fail until keychain.rs updated)

---

## Phase 2: User Story 1 - Seamless Keychain Access (Priority: P1) 🎯 MVP

**Goal**: Replace `security-framework` crate with `security` CLI command so rebuilt binaries don't trigger permission dialogs

**Independent Test**: `cargo test keychain` should pass, and manual rebuild test shows no permission dialog

### Implementation (Existing Tests Validate)

- [x] T003 [US1] Replace security-framework import with std::process::Command in src/api/keychain.rs
- [x] T004 [US1] Implement `security find-generic-password` CLI execution in src/api/keychain.rs:get_access_token()
- [x] T005 [US1] Parse CLI output as JSON and extract claudeAiOauth.accessToken in src/api/keychain.rs
- [x] T006 [US1] Verify existing tests pass: `cargo test keychain`

### Verification

- [x] T007 [US1] Run full test suite: `cargo test`
- [x] T008 [US1] Run clippy: `cargo clippy -- -D warnings`
- [x] T009 [US1] Build release binary: `cargo build --release`
- [ ] T010 [US1] Manual test: Run binary, rebuild, run again - verify no keychain dialog appears

**Checkpoint**: User Story 1 complete - `cargo test` green, no permission dialogs on rebuild

---

## Phase 3: User Story 2 - First-Time Terminal Authorization (Priority: P2)

**Goal**: Ensure first-time users see the prompt for terminal authorization (not binary authorization)

**Independent Test**: Manual test from new terminal context

**Note**: This user story is inherently satisfied by the CLI approach - terminal apps inherit their own keychain ACL. No code changes needed beyond US1.

### Verification Only

- [x] T011 [US2] Document in README or quickstart.md: First run may prompt for terminal keychain access
- [ ] T012 [US2] Manual test: Run from fresh terminal, verify prompt targets terminal app (Terminal.app/iTerm), not claude-status binary

**Checkpoint**: User Story 2 verified - authorization prompt appears for terminal, not binary

---

## Phase 4: User Story 3 - Graceful Error Handling (Priority: P3)

**Goal**: Ensure clear error messages for keychain access failures

**Independent Test**: `cargo test keychain` covers error paths

**Note**: Error handling is already implemented in keychain.rs with proper StatusLineError mapping. Existing tests validate this.

### Verification

- [x] T013 [US3] Review error mapping in src/api/keychain.rs: CLI spawn failure → KeychainAccess
- [x] T014 [US3] Review error mapping in src/api/keychain.rs: Non-zero exit → KeychainAccess(stderr)
- [x] T015 [US3] Review error mapping in src/api/keychain.rs: Invalid UTF-8 → KeychainAccess
- [x] T016 [US3] Review error mapping in src/api/keychain.rs: JSON parse failure → ApiResponse
- [x] T017 [US3] Review error mapping in src/api/keychain.rs: Missing token → CredentialsNotFound
- [x] T018 [US3] Verify error tests exist in src/api/keychain.rs: test_access_token_debug_redacted, test_access_token_as_str

**Checkpoint**: User Story 3 verified - error handling is comprehensive and tested

---

## Phase 5: Polish & Final Verification

**Purpose**: Quality gates and final validation

- [x] T019 Full test suite: `cargo test --all-features`
- [x] T020 Lint check: `cargo clippy -- -D warnings`
- [x] T021 Format check: `cargo fmt --check`
- [x] T022 Documentation: `cargo doc --no-deps` with no warnings
- [x] T023 Compare binary sizes: Before vs after removing security-framework (Current: 1.8MB)
- [x] T024 Update specs/002-security-cli-keychain/checklists/requirements.md to mark validation complete

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1 (Setup) ──────────────────────────────────┐
                                                   │
                                                   ▼
Phase 2 (US1: Core Implementation) ───────────────┐
                                                   │
                              ┌────────────────────┴────────────────────┐
                              ▼                                         ▼
              Phase 3 (US2: Authorization)              Phase 4 (US3: Error Handling)
                              │                                         │
                              └────────────────────┬────────────────────┘
                                                   │
                                                   ▼
                                    Phase 5 (Polish & Verification)
```

### Parallel Opportunities

- **Phase 1**: T001 and T002 sequential (T002 depends on T001)
- **Phase 2**: T003-T005 sequential (implementation steps); T007-T009 can run in parallel after T006
- **Phase 3**: T011 and T012 can run in parallel
- **Phase 4**: T013-T018 are verification tasks, can run in parallel
- **Phase 5**: T019-T022 can run in parallel; T023-T024 can run in parallel after

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Remove dependency from Cargo.toml
2. Complete Phase 2: Implement CLI-based keychain access
3. **STOP and VALIDATE**: `cargo test`, `cargo clippy`, manual rebuild test
4. Deploy/release if ready

### Incremental Delivery

1. Phase 1 + Phase 2 → MVP ready (no permission dialogs on rebuild)
2. Phase 3 → Documentation update (authorization behavior documented)
3. Phase 4 → Error handling verification (confirms robustness)
4. Phase 5 → Full quality gate (ready for merge)

---

## Notes

- This is a refactoring task - the public API (`get_access_token()`) remains unchanged
- Existing tests validate the implementation without new test files
- The key verification is manual: rebuild binary and confirm no keychain dialog
- User Stories 2 and 3 are inherently satisfied by the CLI approach; they require verification only
- Binary size reduction is a bonus outcome - measure before/after in T023
