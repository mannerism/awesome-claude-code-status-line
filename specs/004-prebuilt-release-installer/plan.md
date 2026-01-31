# Implementation Plan: Prebuilt Release Installer

**Branch**: `004-prebuilt-release-installer` | **Date**: 2026-01-31 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/004-prebuilt-release-installer/spec.md`

## Summary

Replace the build-from-source installer with a prebuilt-release downloader. The installer (`install.sh`) downloads universal macOS binaries from GitHub Releases, verifies SHA256 checksums, and configures Claude Code settings. A GitHub Actions release workflow builds arm64 + x86_64 binaries, creates universal binaries via `lipo`, and publishes them with checksums on tag push. Code review findings (silent unknown flag handling, missing checksum warning, missing CI quality gates) are fixed as part of this feature.

## Technical Context

**Language/Version**: Bash (POSIX-compatible with bash extensions) for `install.sh`; YAML for GitHub Actions workflow; Rust 1.75+ for the binary being built by CI (not modified in this feature)
**Primary Dependencies**: curl (HTTP client), jq (JSON manipulation), security (macOS Keychain CLI), shasum (checksum verification), lipo (universal binary creation in CI)
**Storage**: N/A (installer reads/writes `~/.claude/settings.json` and `~/.local/bin/claude-status`)
**Testing**: `shellcheck` for static analysis of install.sh; `bats-core` or manual integration test scripts for installer behavior; `cargo test --all-features` for Rust binary quality gates in CI
**Benchmarking**: N/A (installer is not performance-critical; Rust binary benchmarks are unchanged)
**Target Platform**: macOS 12+ (Apple Silicon arm64 and Intel x86_64)
**Project Type**: Shell script (installer) + GitHub Actions workflow (CI) -- not a Rust crate modification
**Performance Goals**: Installation completes in <30 seconds on standard broadband (download-only, no compilation)
**Constraints**: No new Rust dependencies; installer must work without Rust toolchain installed on user machine

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                    | Compliance | Notes                                                                                                                                           |
| ---------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| I. Simple, Not Easy          | ☑          | Single-file installer with clear linear flow. Each flag (--version, --check, --help) is an independent code path. Removing a feature = removing a case branch. |
| II. Spec-Driven (SDD)        | ☑          | spec.md complete with 4 user stories, Given/When/Then scenarios, 15 functional requirements, 10 edge cases. Clarify phase confirmed no ambiguities.            |
| III. Test-Driven (TDD)       | ☑          | Test strategy: shellcheck for static analysis, manual integration tests for installer flows. CI workflow adds cargo fmt/clippy/test gates. TDD applies to Rust code; installer tests follow verify-after-write pattern (acceptable for shell scripts per Complexity Tracking). |
| IV. Rust Best Practices      | ☑ (N/A)    | This feature does not modify Rust source code. Rust principles apply to the binary built by CI, which is unchanged. CI workflow enforces Rust quality gates (fmt, clippy, test) before release. |
| V. Correctness Through Types | ☑ (N/A)    | Shell scripts are untyped. Type correctness is enforced at the Rust binary level (unchanged). Installer uses exit codes and string comparisons for state management, which is idiomatic for bash. |

**Blockers**: None. Principles IV and V are not directly applicable to shell/YAML artifacts but are enforced via CI quality gates on the Rust binary.

## Project Structure

### Documentation (this feature)

```text
specs/004-prebuilt-release-installer/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0: technology decisions
├── data-model.md        # Phase 1: entity/flow documentation
├── quickstart.md        # Phase 1: verification steps
├── checklists/
│   └── requirements.md  # Spec quality checklist
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
install.sh                           # Prebuilt release installer (modified)
.github/workflows/release.yml       # CI release workflow (new)
README.md                            # Updated install/usage docs (modified)
CLAUDE.md                            # Synced with architectural changes (modified)
```

**Structure Decision**: This feature modifies root-level shell/YAML/markdown files only. No changes to the `src/` Rust crate structure. The Rust binary is built by CI from the existing codebase without modification.

## Domain Types

This feature does not introduce Rust domain types. The "domain" is expressed through bash exit codes and string conventions:

| Concept          | Representation                              | Rationale                                                        |
| ---------------- | ------------------------------------------- | ---------------------------------------------------------------- |
| Exit status      | `exit 0` (success), `exit 1` (failure)      | POSIX convention; `set -e` propagates failures automatically     |
| Version tag      | String with `v` prefix (e.g., `v0.2.0`)     | Normalized at parse time: bare `0.2.0` → `v0.2.0`               |
| Install path     | `$HOME/.local/bin/claude-status`             | XDG-adjacent convention; created if missing                      |
| Settings path    | `$HOME/.claude/settings.json`               | Claude Code convention; backed up before modification            |
| Temp directory   | `mktemp -d` with `trap` cleanup             | Ensures cleanup on success or failure                            |

## Error Handling Strategy

Bash error handling via `set -e` and explicit exit codes:

```bash
# Pattern: validate prerequisites early, fail fast
set -e

# Unknown arguments → warn and exit (FR-007)
*) echo "❌ Unknown option: $1" >&2; exit 1 ;;

# Checksum unavailable → warn but continue (FR-003)
if ! curl -sSfL "$SHA_URL" -o "$TMP_DIR/sha256.txt" 2>/dev/null; then
    echo "⚠️  Checksum file not available. Skipping integrity verification."
else
    (cd "$TMP_DIR" && shasum -a 256 -c sha256.txt)
fi

# All other failures → set -e propagates non-zero exit from curl, shasum, etc.
```

## Complexity Tracking

| Violation                                    | Why Needed                                                        | Simpler Alternative Rejected Because                              |
| -------------------------------------------- | ----------------------------------------------------------------- | ----------------------------------------------------------------- |
| TDD not strictly applied to install.sh       | Shell scripts lack compile-time guarantees; TDD cycle is write → shellcheck → manual test | bats-core adds a dependency for 1 script; shellcheck + manual integration tests provide sufficient coverage |
| No Rust domain types for this feature        | Feature is entirely bash/YAML                                     | Wrapping installer in Rust would add unnecessary complexity for a download-and-copy script |
