# Research: Prebuilt Release Installer

**Feature**: 004-prebuilt-release-installer
**Date**: 2026-01-31

## Decision 1: Installer Distribution Strategy

**Decision**: Download prebuilt universal macOS binaries from GitHub Releases instead of compiling from source.

**Rationale**:
- Eliminates Rust toolchain as a user prerequisite (was the #1 install friction point)
- Universal binaries via `lipo` support both arm64 and x86_64 from a single archive
- GitHub Releases provides reliable CDN-backed hosting at no cost for open-source projects
- SHA256 checksums enable integrity verification without GPG key management complexity

**Alternatives considered**:
- **Build from source (current)**: Requires Rust 1.75+, ~60s compile time, cargo dependency resolution. Rejected: too much friction for end users.
- **Homebrew tap**: Would provide automatic updates and dependency management. Rejected: adds maintenance overhead for a single binary; can be added later as enhancement.
- **cargo install from crates.io**: Still requires Rust toolchain. Rejected: same friction as build-from-source.

## Decision 2: CI Quality Gates in Release Workflow

**Decision**: Add `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test --all-features` as mandatory steps before building release artifacts.

**Rationale**:
- Constitution requires "CI MUST enforce: formatting, linting, tests, documentation"
- Running quality gates in the release workflow ensures no release ships with lint warnings or test failures
- Placing gates before the build step means artifact creation is blocked on quality

**Alternatives considered**:
- **Separate CI workflow for quality gates**: Would require a separate `ci.yml` triggered on push/PR. Rejected for now: adds complexity when the release workflow can embed the same gates. A separate CI workflow for PRs is recommended as a future enhancement.
- **Post-build quality gates**: Run tests after building. Rejected: wastes CI time on a build that may not ship.

## Decision 3: Unknown Argument Handling

**Decision**: Warn on unrecognized CLI flags and exit with non-zero status.

**Rationale**:
- Silent swallowing of unknown flags is a common source of user confusion (typos go undetected)
- Strict argument validation follows the principle of least surprise
- Consistent with GNU/POSIX convention: unknown flags are errors

**Alternatives considered**:
- **Silent ignore (current behavior)**: `*) shift ;;` discards unknowns. Rejected: violates FR-007 and hides user mistakes.
- **Warn but continue**: Print warning but proceed with installation. Rejected: if the user intended a different flag, proceeding may produce unexpected results.

## Decision 4: Checksum Unavailability Handling

**Decision**: Print a warning when SHA256 checksum file is unavailable but continue with installation.

**Rationale**:
- Early releases may not have checksum files (retroactive compliance)
- Blocking installation on missing checksums would break `--version` for older releases
- Warning makes the user aware they are accepting unverified integrity

**Alternatives considered**:
- **Hard fail on missing checksum**: Exit non-zero if sha256.txt is unavailable. Rejected: would break installs for any release that lacks checksums.
- **Silent skip (current behavior)**: No output when checksum is unavailable. Rejected: user has no way to know integrity was not verified.

## Decision 5: Shell Script Testing Strategy

**Decision**: Use `shellcheck` for static analysis. Manual integration testing for installer behavior. No `bats-core` dependency.

**Rationale**:
- `shellcheck` catches the majority of bash anti-patterns, quoting issues, and portability problems
- The installer is a single file with a linear flow; integration tests are most valuable at the end-to-end level
- Adding `bats-core` as a test dependency for one script adds overhead disproportionate to the value

**Alternatives considered**:
- **bats-core test suite**: Full bash testing framework with assertions. Rejected: adds a dependency (npm or brew) for testing a single script; the installer's correctness is primarily validated by running it.
- **No testing**: Ship without any validation. Rejected: shellcheck is zero-cost and catches real bugs.
