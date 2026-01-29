# Feature Specification: Uninstall Script

**Feature Branch**: `003-uninstall-script`
**Created**: 2026-01-29
**Status**: Draft
**Input**: User description: "Create uninstall.sh script to completely remove all traces of claude-status installation including binary, config files, and keychain entries"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Complete Uninstallation (Priority: P1)

A user wants to completely remove the claude-status tool from their system to test with a fresh Claude Code instance or simply clean up. They run the uninstall script and expect all installed components to be removed with no manual cleanup required.

**Why this priority**: This is the core purpose of the feature - without complete removal capability, users cannot achieve a clean slate for testing or uninstallation.

**Independent Test**: Can be fully tested by running `./uninstall.sh` after a successful installation and verifying no traces remain.

**Acceptance Scenarios**:

1. **Given** claude-status is installed via install.sh, **When** user runs `./uninstall.sh`, **Then** the binary at `~/.local/bin/claude-status` is removed
2. **Given** claude-status is installed via install.sh, **When** user runs `./uninstall.sh`, **Then** the statusLine configuration is removed from `~/.claude/settings.json`
3. **Given** claude-status is installed via install.sh, **When** user runs `./uninstall.sh`, **Then** the config file at `~/.config/claude-status/config.json` is removed (if it exists)
4. **Given** claude-status is installed via install.sh, **When** user runs `./uninstall.sh`, **Then** the script reports success with a summary of removed items

---

### User Story 2 - Safe Partial Uninstallation (Priority: P2)

A user wants to uninstall claude-status, but some components may not exist (e.g., config file was never created, or binary was manually deleted). The script should handle missing components gracefully and still remove what exists.

**Why this priority**: Real-world uninstallation scenarios often involve partial states; robustness ensures good user experience.

**Independent Test**: Can be tested by manually removing some components first, then running uninstall and verifying no errors occur.

**Acceptance Scenarios**:

1. **Given** only the binary exists (no config file), **When** user runs `./uninstall.sh`, **Then** the script removes the binary and completes successfully
2. **Given** no components exist (already uninstalled), **When** user runs `./uninstall.sh`, **Then** the script reports "Nothing to uninstall" and exits cleanly
3. **Given** the Claude settings.json doesn't contain statusLine config, **When** user runs `./uninstall.sh`, **Then** the script still removes other components without error

---

### User Story 3 - Keychain Credential Cleanup (Priority: P2)

A user wants to remove any keychain access or credentials that were used by claude-status. The script should remove the relevant keychain entries to ensure a completely clean state.

**Why this priority**: Keychain entries can persist even after application removal; cleaning these is essential for "no traces" requirement.

**Independent Test**: Can be tested by checking macOS Keychain Access app before and after uninstall to verify credentials are removed.

**Acceptance Scenarios**:

1. **Given** a keychain entry exists for the claude-status Anthropic API credentials, **When** user runs `./uninstall.sh`, **Then** the keychain entry is removed
2. **Given** no keychain entry exists for claude-status, **When** user runs `./uninstall.sh`, **Then** the script continues without error

---

### User Story 4 - Backup Preservation (Priority: P3)

A user may want to preserve their Claude settings backup file that was created during installation. The script should either preserve the backup or give the user the option.

**Why this priority**: Lower priority as backups are safety nets; most users doing full uninstall don't need them.

**Independent Test**: Can be tested by verifying backup file state after uninstall.

**Acceptance Scenarios**:

1. **Given** a settings.json.backup file exists from installation, **When** user runs `./uninstall.sh`, **Then** the backup file is preserved (not deleted)
2. **Given** user wants full cleanup including backups, **When** user runs `./uninstall.sh --purge`, **Then** backup files are also removed

---

### Edge Cases

- What happens when the user lacks permission to delete the binary? Script should report the error clearly.
- What happens when another process is using the binary? Script should warn and suggest closing Claude Code first.
- What happens when settings.json is malformed? Script should handle JSON parsing errors gracefully.
- What happens when the config directory is empty after removing config.json? Directory should be cleaned up if empty.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Script MUST remove the binary from `~/.local/bin/claude-status`
- **FR-002**: Script MUST remove the `statusLine` configuration from `~/.claude/settings.json` without affecting other settings
- **FR-003**: Script MUST remove the config file at `~/.config/claude-status/config.json` if it exists
- **FR-004**: Script MUST remove the config directory at `~/.config/claude-status/` if it becomes empty
- **FR-005**: Script MUST remove any keychain credentials created by or for claude-status (specifically the Anthropic API key stored in keychain)
- **FR-006**: Script MUST handle missing components gracefully without failing
- **FR-007**: Script MUST display a clear summary of what was removed at completion
- **FR-008**: Script MUST NOT remove the settings.json.backup file by default (preserve user data safety net)
- **FR-009**: Script MUST support a `--purge` flag to also remove backup files
- **FR-010**: Script MUST exit with appropriate exit codes (0 for success, non-zero for errors)
- **FR-011**: Script MUST work on macOS (primary target platform)

### Key Entities

- **Binary**: The compiled claude-status executable installed to user's local bin
- **Claude Settings**: The settings.json file containing Claude Code configuration including statusLine
- **Config File**: Optional user configuration for claude-status display preferences
- **Keychain Entry**: macOS Keychain credential for Anthropic API authentication
- **Backup File**: Safety copy of original settings.json created during installation

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: After running uninstall.sh, no claude-status files or directories remain in user's home directory tree (verifiable via find command)
- **SC-002**: After running uninstall.sh, `which claude-status` returns no results
- **SC-003**: After running uninstall.sh, Claude Code launches without any statusLine configuration active
- **SC-004**: After running uninstall.sh with --purge, Keychain Access shows no claude-status related entries
- **SC-005**: Script completes in under 5 seconds for all scenarios
- **SC-006**: User can successfully run install.sh after uninstall.sh to achieve a fresh installation state

## Assumptions

- The installation was performed using the provided install.sh script
- The user has appropriate permissions to modify files in their home directory
- The system is macOS with standard Keychain available
- Claude Code stores credentials in the macOS Keychain with a predictable service name
- The jq tool is available (same requirement as install.sh)
