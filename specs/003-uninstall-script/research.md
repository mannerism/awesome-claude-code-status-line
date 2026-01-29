# Research: Uninstall Script

**Feature**: 003-uninstall-script
**Date**: 2026-01-29

## Research Summary

This feature is a bash script that undoes install.sh. All technical decisions are straightforward based on the existing install.sh implementation.

## Decisions

### D1: Keychain Entry Handling

**Decision**: Only remove keychain entry with `--purge` flag, not by default

**Rationale**:
- The keychain entry "Claude Code-credentials" is created by Claude Code itself, not by claude-status
- install.sh only *checks* for its existence; it doesn't create it
- Removing it would require the user to re-authenticate with Claude Code
- Users who want a completely clean slate can use `--purge`

**Alternatives considered**:
- Always remove keychain → Rejected: would break Claude Code authentication
- Never remove keychain → Rejected: doesn't satisfy "no traces" requirement for users who want it

### D2: Backup File Handling

**Decision**: Preserve settings.json.backup by default, remove only with `--purge`

**Rationale**:
- Backup file is a safety net for user's original Claude Code settings
- Safe default: preserve user data
- Power users can use `--purge` for complete cleanup

**Alternatives considered**:
- Always remove backup → Rejected: could lose user's original settings
- Interactive prompt → Rejected: adds complexity, script should be non-interactive

### D3: jq Dependency

**Decision**: Require jq for JSON manipulation (same as install.sh)

**Rationale**:
- install.sh already requires jq, so users who installed have it
- jq provides safe, correct JSON manipulation
- Alternative approaches (sed, awk) are error-prone for JSON

**Alternatives considered**:
- Use sed/awk → Rejected: fragile, could corrupt JSON
- Use Python → Rejected: unnecessary dependency
- Skip settings.json cleanup if jq missing → Rejected: leaves partial uninstall

### D4: Exit Behavior

**Decision**: Don't use `set -e`; continue on errors and report at end

**Rationale**:
- Uninstall should be resilient - remove what we can
- User should see complete picture of what succeeded/failed
- Partial uninstall is better than no uninstall

**Alternatives considered**:
- Use `set -e` → Rejected: would stop at first error
- Silent failures → Rejected: user needs to know what happened

### D5: Empty Directory Cleanup

**Decision**: Use `rmdir` for config directory (only removes if empty)

**Rationale**:
- If user added custom files to ~/.config/claude-status/, don't delete them
- Only remove directory if it's empty (contains only our files)

**Alternatives considered**:
- `rm -rf` → Rejected: would delete user's custom files
- Leave directory → Rejected: leaves traces

## Findings from install.sh Analysis

| What install.sh does | What uninstall.sh must undo |
|---------------------|----------------------------|
| Creates `~/.local/bin/claude-status` | Remove binary |
| Creates/modifies `~/.claude/settings.json` (statusLine key) | Remove statusLine key |
| Creates `~/.claude/settings.json.backup` | Optionally remove with --purge |
| Checks for keychain entry (doesn't create) | Optionally remove with --purge |

## Additional Discovery: status_line_script

install.sh also removes legacy `status_line_script` key when updating settings. uninstall.sh should also clean this if present (belt and suspenders).

## No Further Research Needed

All technical questions resolved. Ready for Phase 1.
