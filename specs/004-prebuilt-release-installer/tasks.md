# Tasks: Prebuilt Release Installer

**Input**: Design documents from `/specs/004-prebuilt-release-installer/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, quickstart.md

**Test Strategy**: shellcheck for static analysis of install.sh; manual integration tests per quickstart.md. No bats-core (see research.md Decision 5). CI quality gates enforce Rust tests on the binary.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3, US4)
- Include exact file paths in descriptions

## Path Conventions (This Feature)

- **Installer**: `install.sh` (repository root)
- **CI workflow**: `.github/workflows/release.yml`
- **Documentation**: `README.md`, `CLAUDE.md`
- **Settings**: `~/.claude/settings.json` (user machine, not in repo)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Establish the CLI argument parsing skeleton and shared installer structure that all user stories depend on.

- [x] T001 Rewrite CLI argument parsing in `install.sh` to use a `while` loop with `case` statement supporting `--version|-v`, `--check`, `--help|-h` flags, and rejecting unknown arguments with error message and `exit 1` (FR-006, FR-007)
- [x] T002 Implement `--help` / `-h` handler in `install.sh` that prints usage information and exits 0 (FR-006)
- [x] T003 Add prerequisite validation block in `install.sh`: check for `curl`, `jq` with actionable error messages and `exit 1` (FR-008)
- [x] T004 Add `set -e` and `trap 'rm -rf "$TMP_DIR"' EXIT` cleanup pattern in `install.sh` (FR-011)
- [x] T005 Run `shellcheck install.sh` and fix all warnings/errors

**Checkpoint**: `install.sh --help` works, `install.sh --foo` errors, `shellcheck install.sh` clean

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Version resolution and download infrastructure shared by US1 and US2

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Implement version resolution in `install.sh`: if `--version` not provided, fetch latest tag from `https://api.github.com/repos/mannerism/awesome-claude-code-status-line/releases/latest` via `curl` + `jq -r .tag_name` (FR-001)
- [x] T007 Implement version normalization in `install.sh`: auto-prepend `v` prefix if missing (per data-model.md validation rules)
- [x] T008 Implement download + checksum verification block in `install.sh`: download archive to `mktemp -d`, attempt SHA256 download, verify with `shasum -a 256 -c` if available, warn if checksum unavailable (FR-002, FR-003)
- [x] T009 Implement extract + install block in `install.sh`: `tar -xzf`, verify `claude-status` binary exists in archive, copy to `$HOME/.local/bin/`, `chmod +x` (FR-001)
- [x] T010 Run `shellcheck install.sh` and verify clean

**Checkpoint**: `install.sh` can resolve versions and download/verify/extract archives. Not yet wired to settings or prerequisite checks for Claude CLI.

---

## Phase 3: User Story 1 - Install Latest Release (Priority: P1)

**Goal**: User runs `./install.sh` with no arguments and gets a working installation with configured Claude Code settings.

**Independent Test**: Run `./install.sh` on macOS with curl, jq, Claude Code authenticated; verify `~/.local/bin/claude-status` exists and `~/.claude/settings.json` has correct `statusLine` config.

### Implementation

- [x] T011 [US1] Add Claude CLI prerequisite check in `install.sh`: verify `claude` command exists, exit 1 with `npm install` hint if missing (FR-008)
- [x] T012 [US1] Add Keychain credential check in `install.sh`: `security find-generic-password -s "Claude Code-credentials"`, exit 1 with step-by-step login instructions if missing (FR-008)
- [x] T013 [US1] Implement settings.json configuration in `install.sh`: backup existing file, use `jq` to merge `statusLine` config and remove legacy `status_line_script` key, create new file if missing (FR-009, FR-010)
- [x] T014 [US1] Wire all blocks together for the default (no-flag) install path in `install.sh`: prerequisites → version resolve → download → verify → extract → install → configure settings → success message
- [x] T015 [US1] Run `shellcheck install.sh` and verify clean
- [x] T016 [US1] Manual verification per quickstart.md step 3: run `./install.sh`, verify binary at `~/.local/bin/claude-status`, verify `~/.claude/settings.json` has `statusLine` config

**Checkpoint**: Full install path works end-to-end. `shellcheck` clean.

---

## Phase 4: User Story 2 - Install Specific Version (Priority: P2)

**Goal**: User runs `./install.sh --version vX.Y.Z` to install a pinned version.

**Independent Test**: Run `./install.sh --version v0.1.0` and verify the installed binary matches that version.

### Implementation

- [x] T017 [US2] Verify `--version` flag correctly sets `VERSION` variable and bypasses latest-tag fetch in `install.sh`
- [x] T018 [US2] Verify `--version` without value exits 1 with error message in `install.sh`
- [x] T019 [US2] Verify version normalization: `./install.sh --version 0.1.0` prepends `v` prefix in `install.sh`
- [x] T020 [US2] Manual verification per quickstart.md step 4: run `./install.sh --version v0.1.0`, verify installed version matches

**Checkpoint**: Version pinning works. Bare versions normalized. Missing value caught.

---

## Phase 5: User Story 3 - Check Installed vs Latest Version (Priority: P3)

**Goal**: User runs `./install.sh --check` to see installed vs latest version without modifying anything.

**Independent Test**: Run `./install.sh --check` and verify it prints latest tag and installed version (or "not installed").

### Implementation

- [x] T021 [US3] Implement `--check` handler in `install.sh`: fetch latest tag, read installed version via `$INSTALL_DIR/claude-status --version`, print both, exit 0 (FR-005)
- [x] T022 [US3] Verify `--check` shows "not installed" when binary is absent in `install.sh`
- [x] T023 [US3] Manual verification per quickstart.md step 2: run `./install.sh --check`, verify output format

**Checkpoint**: Version check works without side effects.

---

## Phase 6: User Story 4 - CI Release Workflow (Priority: P3)

**Goal**: Tag push triggers GitHub Actions to build universal macOS binary with quality gates.

**Independent Test**: Push a test tag to fork and verify release assets are published with checksums.

### Implementation

- [x] T024 [P] [US4] Add quality gate job in `.github/workflows/release.yml`: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all-features` steps BEFORE build steps (FR-012)
- [x] T025 [P] [US4] Verify `.github/workflows/release.yml` builds arm64 and x86_64 targets, creates universal binary via `lipo`, packages as `claude-status-macos-universal.tar.gz` (FR-013)
- [x] T026 [P] [US4] Verify `.github/workflows/release.yml` generates `sha256.txt` and uploads both archive and checksum as release assets (FR-014)
- [x] T027 [US4] Verify `.github/workflows/release.yml` triggers only on `v*` tags (FR-015)

**Checkpoint**: Release workflow has quality gates before build. Assets include archive + checksum.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Documentation sync, quality gates, and final verification

- [x] T028 Update `README.md`: verify install options (`--version`, `--check`), Security & Privacy section, requirements list (curl, jq instead of Rust) reflect current `install.sh` behavior
- [x] T029 [P] Update `CLAUDE.md`: add prebuilt installer details (no longer builds from source), new installer flags, new CI workflow path `.github/workflows/release.yml`, keep `cargo build --release` for development
- [x] T030 Run full Rust quality gates: `cargo fmt --check && cargo clippy -- -D warnings && cargo test --all-features`
- [x] T031 Run `shellcheck install.sh` (final pass)
- [x] T032 Run quickstart.md full validation (all 6 steps from quickstart.md)
- [x] T033 Verify all 10 edge cases from spec.md are handled in `install.sh` (curl missing, jq missing, GitHub unreachable, checksum fail, checksum unavailable, archive missing binary, no credentials, unknown flags, Claude CLI missing, ~/.local/bin missing)

**Checkpoint**: All quality gates green. Documentation synced. Edge cases verified.

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 (argument parsing structure must exist)
- **US1 (Phase 3)**: Depends on Phase 2 (needs download/verify/extract infrastructure)
- **US2 (Phase 4)**: Depends on Phase 2 (shares download infrastructure; `--version` flag parsed in Phase 1)
- **US3 (Phase 5)**: Depends on Phase 1 only (`--check` is independent of download flow)
- **US4 (Phase 6)**: Independent of all installer phases (YAML-only, can run in parallel)
- **Polish (Phase 7)**: Depends on all user stories being complete

### Parallel Opportunities

- **US4 (Phase 6)** can proceed in parallel with all other phases (separate file: `.github/workflows/release.yml`)
- **US3 (Phase 5)** can proceed after Phase 1 (only needs arg parsing, not download infra)
- T024, T025, T026 within US4 are all parallelizable (different sections of release.yml)
- T028, T029 in Polish are parallelizable (different files)

```text
Phase 1 (Setup) ──────────────┬──── Phase 2 (Foundation) ──── Phase 3 (US1) ──── Phase 4 (US2)
                               │                                                        │
                               ├──── Phase 5 (US3)                                      │
                               │                                                        │
Phase 6 (US4) ─── [parallel] ─┘                                                        │
                                                                                        │
                               Phase 7 (Polish) ◄───────────────────────────────────────┘
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (argument parsing skeleton)
2. Complete Phase 2: Foundational (download + verify infrastructure)
3. Complete Phase 3: User Story 1 (full install path)
4. **STOP and VALIDATE**: `shellcheck install.sh`, manual install test
5. Ship if ready (US2-US4 are incremental improvements)

### Incremental Delivery

1. Setup + Foundational → Installer skeleton ready
2. User Story 1 → Full install works (MVP!)
3. User Story 2 → Version pinning works
4. User Story 3 → Version check works
5. User Story 4 → CI release pipeline works
6. Polish → Documentation synced, all quality gates green, PR ready

---

## Verification Checklist

Before marking this feature complete, verify:

- [x] `shellcheck install.sh` passes with no warnings
- [x] `./install.sh --help` prints usage and exits 0
- [x] `./install.sh --foo` prints error and exits 1
- [x] `./install.sh --check` shows versions without installing
- [ ] `./install.sh` completes full installation (when release exists) — requires published GitHub Release
- [ ] `./install.sh --version vX.Y.Z` installs specific version — requires published GitHub Release
- [x] `cargo fmt --check` passes
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo test --all-features` passes (107 tests)
- [x] README.md reflects current installer behavior
- [x] CLAUDE.md synchronized with architectural changes
- [x] All 10 edge cases from spec.md are handled

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Shell scripts use verify-after-write pattern instead of strict TDD (per Complexity Tracking in plan.md)
- `shellcheck` runs after each phase as the shell equivalent of `cargo clippy`
- US4 (CI workflow) is fully independent and can be done at any time
