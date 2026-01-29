# Implementation Plan: Uninstall Script

**Branch**: `003-uninstall-script` | **Date**: 2026-01-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/003-uninstall-script/spec.md`

## Summary

Create `uninstall.sh` - a bash script that completely removes all traces of claude-status installation. The script mirrors install.sh but in reverse: removes binary, cleans settings.json, removes config directory, and optionally removes keychain entries and backup files with `--purge` flag.

## Technical Context

**Language/Version**: Bash (POSIX-compatible with bash extensions)
**Primary Dependencies**: jq (for JSON manipulation), security (macOS Keychain CLI)
**Storage**: N/A (removes files, doesn't create them)
**Testing**: Manual verification + integration test script
**Benchmarking**: N/A (single-run script)
**Target Platform**: macOS (primary), with graceful degradation for missing macOS-specific tools
**Project Type**: Shell script (not Rust)
**Performance Goals**: <5 seconds total execution
**Constraints**: Must be idempotent (safe to run multiple times)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                    | Compliance | Notes                                                              |
| ---------------------------- | ---------- | ------------------------------------------------------------------ |
| I. Simple, Not Easy          | ✅          | Single-purpose script, mirrors install.sh structure                |
| II. Spec-Driven (SDD)        | ✅          | spec.md complete with Given/When/Then scenarios                    |
| III. Test-Driven (TDD)       | ✅          | Test scenarios defined; verification commands in spec              |
| IV. Rust Best Practices      | N/A        | This is a bash script, not Rust code                               |
| V. Correctness Through Types | N/A        | Shell scripts don't have static types; using explicit path checks  |

**Blockers**: None. Constitution principles IV and V are Rust-specific and don't apply to shell scripts.

## Project Structure

### Documentation (this feature)

```text
specs/003-uninstall-script/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── quickstart.md        # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
uninstall.sh             # New file - the uninstall script
install.sh               # Existing - reference for what to undo
```

**Structure Decision**: Single shell script at repository root, parallel to install.sh. No Rust code changes required.

## Components to Remove

Based on analysis of install.sh, the uninstall script must handle:

| Component | Location | Removal Method |
|-----------|----------|----------------|
| Binary | `~/.local/bin/claude-status` | `rm -f` |
| Claude Settings | `~/.claude/settings.json` (statusLine key) | `jq 'del(.statusLine)'` |
| Config File | `~/.config/claude-status/config.json` | `rm -f` |
| Config Directory | `~/.config/claude-status/` | `rmdir` (if empty) |
| Backup File | `~/.claude/settings.json.backup` | `rm -f` (only with --purge) |
| Keychain Entry | "Claude Code-credentials" | `security delete-generic-password` (only with --purge) |

## Script Flow

```text
uninstall.sh [--purge]
│
├── Parse arguments (check for --purge flag)
│
├── Initialize removal tracking (for summary)
│
├── Remove binary
│   └── rm -f ~/.local/bin/claude-status
│
├── Clean settings.json
│   ├── Check if file exists
│   ├── Check if statusLine key exists
│   └── jq 'del(.statusLine) | del(.status_line_script)'
│
├── Remove config
│   ├── rm -f ~/.config/claude-status/config.json
│   └── rmdir ~/.config/claude-status/ (if empty)
│
├── If --purge:
│   ├── Remove backup: rm -f ~/.claude/settings.json.backup
│   └── Remove keychain: security delete-generic-password -s "Claude Code-credentials"
│
└── Print summary of removed items
```

## Error Handling Strategy

Since this is a bash script, error handling follows shell conventions:

```bash
# Don't use set -e (we want to continue even if some removals fail)
# Track what was removed vs what failed
removed=()
failed=()

# Safe removal pattern
remove_if_exists() {
    local path="$1"
    local desc="$2"
    if [[ -e "$path" ]]; then
        if rm -f "$path" 2>/dev/null; then
            removed+=("$desc")
        else
            failed+=("$desc: permission denied")
        fi
    fi
}
```

## Complexity Tracking

| Decision | Rationale | Alternative Rejected |
|----------|-----------|---------------------|
| No `set -e` | Script should continue removing other items if one fails | `set -e` would abort on first failure |
| jq dependency | Already required by install.sh; consistent tooling | Manual sed/awk would be error-prone for JSON |
| --purge flag | Keychain is shared with Claude Code; default-safe | Always removing keychain would break Claude Code |

## Verification Commands

After running uninstall.sh, these commands verify complete removal:

```bash
# Binary removed
which claude-status  # Should return nothing

# Settings cleaned
jq '.statusLine' ~/.claude/settings.json  # Should return null

# Config removed
ls ~/.config/claude-status/  # Should fail (directory doesn't exist)

# With --purge: keychain removed
security find-generic-password -s "Claude Code-credentials"  # Should fail
```
