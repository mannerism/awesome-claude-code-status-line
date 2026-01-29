# Tasks: Uninstall Script

**Input**: Design documents from `/specs/003-uninstall-script/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, quickstart.md

**Note**: This is a bash script, not Rust code. TDD principles are adapted: verification commands serve as tests.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

---

## Phase 1: Setup

**Purpose**: Create script skeleton and helper functions

- [x] T001 Create uninstall.sh script skeleton with shebang and header comments in `uninstall.sh`
- [x] T002 Add argument parsing for --purge flag in `uninstall.sh`
- [x] T003 Add jq dependency check with helpful error message in `uninstall.sh`
- [x] T004 Create tracking arrays (removed[], failed[], skipped[]) for summary in `uninstall.sh`
- [x] T005 Create helper function `remove_if_exists()` for safe file removal in `uninstall.sh`
- [x] T006 Create helper function `remove_json_key()` for safe JSON key removal in `uninstall.sh`
- [x] T007 Make script executable: `chmod +x uninstall.sh`

---

## Phase 2: User Story 1 - Complete Uninstallation (Priority: P1) 🎯 MVP

**Goal**: Remove binary, settings, and config when all components exist

**Independent Test**: Run `./install.sh` then `./uninstall.sh` and verify all traces removed

### Implementation

- [x] T008 [US1] Implement binary removal: `rm -f ~/.local/bin/claude-status` in `uninstall.sh`
- [x] T009 [US1] Implement settings.json cleanup: remove `statusLine` and `status_line_script` keys via jq in `uninstall.sh`
- [x] T010 [US1] Implement config file removal: `rm -f ~/.config/claude-status/config.json` in `uninstall.sh`
- [x] T011 [US1] Implement config directory removal: `rmdir ~/.config/claude-status/` if empty in `uninstall.sh`
- [x] T012 [US1] Implement success summary: print list of removed items in `uninstall.sh`
- [x] T013 [US1] Add exit code 0 on success in `uninstall.sh`

### Verification

- [x] T014 [US1] Verify: Run install.sh, then uninstall.sh, check `which claude-status` returns empty
- [x] T015 [US1] Verify: Check `jq '.statusLine' ~/.claude/settings.json` returns null
- [x] T016 [US1] Verify: Check `ls ~/.config/claude-status/` fails (directory removed)

**Checkpoint**: MVP complete - basic uninstall works for fresh installations

---

## Phase 3: User Story 2 - Safe Partial Uninstallation (Priority: P2)

**Goal**: Handle missing components gracefully without errors

**Independent Test**: Manually delete some components, run uninstall, verify no errors

### Implementation

- [x] T017 [US2] Add existence check before binary removal in `uninstall.sh`
- [x] T018 [US2] Add existence check before settings.json modification in `uninstall.sh`
- [x] T019 [US2] Add check for statusLine key presence before removal in `uninstall.sh`
- [x] T020 [US2] Add existence check before config file removal in `uninstall.sh`
- [x] T021 [US2] Implement "Nothing to uninstall" message when no components found in `uninstall.sh`
- [x] T022 [US2] Track skipped items (not found) separately from removed items in `uninstall.sh`

### Verification

- [x] T023 [US2] Verify: Run uninstall.sh twice in a row - second run reports "Nothing to uninstall"
- [x] T024 [US2] Verify: Delete binary manually, run uninstall.sh - completes without error
- [x] T025 [US2] Verify: Create settings.json without statusLine key, run uninstall.sh - completes without error

**Checkpoint**: Robust uninstall - handles any partial state

---

## Phase 4: User Story 3 - Keychain Credential Cleanup (Priority: P2)

**Goal**: Remove keychain entry with --purge flag

**Independent Test**: Run `./uninstall.sh --purge` and verify keychain entry removed

### Implementation

- [x] T026 [US3] Add keychain removal only when --purge flag is set in `uninstall.sh`
- [x] T027 [US3] Implement keychain deletion: `security delete-generic-password -s "Claude Code-credentials"` in `uninstall.sh`
- [x] T028 [US3] Add existence check for keychain entry before removal in `uninstall.sh`
- [x] T029 [US3] Handle keychain removal failure gracefully (may require auth) in `uninstall.sh`

### Verification

- [x] T030 [US3] Verify: Run uninstall.sh without --purge, keychain entry preserved
- [x] T031 [US3] Verify: Run uninstall.sh --purge, `security find-generic-password -s "Claude Code-credentials"` fails

**Checkpoint**: Keychain cleanup works with --purge

---

## Phase 5: User Story 4 - Backup Preservation (Priority: P3)

**Goal**: Preserve backup by default, remove with --purge

**Independent Test**: Run uninstall, verify backup exists; run with --purge, verify backup removed

### Implementation

- [x] T032 [US4] Add backup file removal only when --purge flag is set in `uninstall.sh`
- [x] T033 [US4] Implement backup deletion: `rm -f ~/.claude/settings.json.backup` in `uninstall.sh`
- [x] T034 [US4] Update summary to show backup status (preserved vs removed) in `uninstall.sh`

### Verification

- [x] T035 [US4] Verify: Run install.sh then uninstall.sh, backup file still exists
- [x] T036 [US4] Verify: Run uninstall.sh --purge, backup file removed

**Checkpoint**: All user stories complete

---

## Phase 6: Polish & Error Handling

**Purpose**: Edge cases and final quality checks

- [x] T037 Handle permission denied errors with clear message in `uninstall.sh`
- [x] T038 Handle malformed settings.json gracefully (jq error handling) in `uninstall.sh`
- [x] T039 Add --help flag with usage information in `uninstall.sh`
- [x] T040 Add colored output (emoji indicators) matching install.sh style in `uninstall.sh`
- [x] T041 Final verification: Run full cycle (install → uninstall → install) and verify fresh state
- [x] T042 Update README.md with uninstall instructions (reference quickstart.md)

---

## Dependencies & Execution Order

### Phase Dependencies

```text
Phase 1 (Setup) ─────────────────────────────┐
                                             ▼
Phase 2 (US1: Complete Uninstall) ───────────┤ MVP COMPLETE
                                             ▼
Phase 3 (US2: Partial Uninstall) ────────────┤
                                             │
Phase 4 (US3: Keychain) ─────────────────────┤ Can run in parallel
                                             │
Phase 5 (US4: Backup) ───────────────────────┤
                                             ▼
Phase 6 (Polish) ────────────────────────────┘
```

### Parallel Opportunities

**Within Phase 1 (Setup)**:
- T003, T004, T005, T006 can run in parallel after T001-T002

**Across User Stories (Phases 3-5)**:
- US2, US3, US4 are independent once US1 is complete
- However, since they all modify uninstall.sh, sequential execution is safer

**Within Polish Phase**:
- T037, T038, T039, T040 can run in parallel

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T007)
2. Complete Phase 2: User Story 1 (T008-T016)
3. **STOP and VALIDATE**: Test full install/uninstall cycle
4. Deploy MVP if ready

### Incremental Delivery

1. Setup → US1 → **MVP ready** (basic uninstall works)
2. US2 → **Robust** (handles partial states)
3. US3 + US4 → **Complete** (--purge flag works)
4. Polish → **Production ready** (error handling, docs)

---

## Verification Commands Summary

```bash
# After US1 (MVP):
./install.sh && ./uninstall.sh && which claude-status  # Should be empty

# After US2 (Robustness):
./uninstall.sh  # Should say "Nothing to uninstall"

# After US3+US4 (--purge):
./install.sh
./uninstall.sh --purge
security find-generic-password -s "Claude Code-credentials"  # Should fail
ls ~/.claude/settings.json.backup  # Should fail

# Full cycle test:
./install.sh && ./uninstall.sh && ./install.sh  # Should work cleanly
```

---

## Notes

- This is a bash script, not Rust - no cargo test
- Verification is manual (run commands, check results)
- All tasks modify single file: `uninstall.sh`
- --purge is for power users who want complete cleanup
- Default behavior is safe (preserves keychain, backup)
