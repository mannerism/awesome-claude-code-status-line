# Feature Specification: Use macOS Security CLI for Keychain Access

**Feature Branch**: `002-security-cli-keychain`
**Created**: 2026-01-29
**Status**: Draft
**Input**: User description: "Replace security-framework Rust crate with macOS security CLI to eliminate permission popups on binary rebuilds"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Seamless Keychain Access After Binary Rebuild (Priority: P1)

As a developer working on `claude-status`, I want keychain access to work without permission prompts after rebuilding the binary, so that I can iterate on the code without being interrupted by macOS security dialogs.

**Why this priority**: This is the core problem being solved. Every rebuild currently triggers a permission popup, breaking the developer workflow significantly.

**Independent Test**: Can be fully tested by rebuilding the binary multiple times and verifying no keychain permission dialogs appear (after initial terminal authorization).

**Acceptance Scenarios**:

1. **Given** the user has previously authorized their terminal (Terminal.app/iTerm) to access keychain, **When** they rebuild and run `claude-status`, **Then** no permission dialog appears and the API key is retrieved successfully
2. **Given** the user runs `claude-status` from a fresh terminal session, **When** the keychain has been authorized for that terminal app, **Then** no permission dialog appears
3. **Given** the binary is installed to a new location, **When** the user runs it from an authorized terminal, **Then** no permission dialog appears

---

### User Story 2 - First-Time Terminal Authorization (Priority: P2)

As a new user installing `claude-status`, I want to authorize my terminal application once for keychain access, so that all future runs work seamlessly without additional prompts.

**Why this priority**: This is the one-time setup experience that enables the seamless workflow.

**Independent Test**: Can be tested by running `claude-status` from a never-authorized terminal and verifying the authorization flow works correctly.

**Acceptance Scenarios**:

1. **Given** a user's terminal has never accessed "Claude Code-credentials" keychain item, **When** they run `claude-status` for the first time, **Then** macOS prompts for keychain access for the terminal application (not the binary)
2. **Given** the user clicks "Always Allow" on the terminal authorization prompt, **When** they run `claude-status` again, **Then** no prompt appears

---

### User Story 3 - Graceful Error Handling (Priority: P3)

As a user, I want clear error messages when keychain access fails, so that I can understand what went wrong and how to fix it.

**Why this priority**: Error handling ensures a good user experience when things go wrong, but is secondary to the core functionality.

**Independent Test**: Can be tested by simulating various failure conditions and verifying appropriate error messages.

**Acceptance Scenarios**:

1. **Given** the user denies keychain access, **When** `claude-status` tries to retrieve credentials, **Then** a clear error message explains that keychain access was denied
2. **Given** the "Claude Code-credentials" keychain item does not exist, **When** `claude-status` runs, **Then** a clear error message indicates credentials are not found
3. **Given** the keychain item exists but contains malformed data, **When** `claude-status` runs, **Then** a clear error message indicates the credentials format is invalid

---

### Edge Cases

- What happens when the `security` CLI command is not available (non-macOS system)?
- How does the system handle when the keychain item exists but the access token field is missing?
- What happens when the keychain is locked and requires unlock?
- How does the system behave when the JSON in keychain is valid but the OAuth token structure differs?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST retrieve credentials using the macOS `security` CLI command instead of direct framework calls
- **FR-002**: System MUST execute `security find-generic-password -s "Claude Code-credentials" -w` to retrieve the keychain password
- **FR-003**: System MUST parse the retrieved JSON to extract the `claudeAiOauth.accessToken` field
- **FR-004**: System MUST return appropriate errors when the `security` command fails
- **FR-005**: System MUST return appropriate errors when JSON parsing fails
- **FR-006**: System MUST return appropriate errors when the access token field is missing from the JSON structure
- **FR-007**: System MUST NOT require the `security-framework` crate as a dependency after implementation

### Key Entities

- **AccessToken**: Represents the OAuth access token retrieved from keychain, with its value kept secure (redacted in debug output)
- **Keychain Item**: The "Claude Code-credentials" entry containing JSON with OAuth credentials stored by Claude Code

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users experience zero permission dialogs when running rebuilt binaries (after initial terminal authorization)
- **SC-002**: All existing functionality is preserved - credentials are successfully retrieved and API calls work
- **SC-003**: All existing tests pass without modification (or with minimal adaptation)
- **SC-004**: Binary size is reduced due to removal of native framework dependency
- **SC-005**: Error messages clearly indicate the nature of any keychain access failures

## Assumptions

- The `security` CLI command is available on all macOS systems (it is a standard system utility)
- Users will run `claude-status` from a terminal application (Terminal.app, iTerm, etc.) rather than from other contexts
- The terminal application's code signature remains stable, allowing "Always Allow" to persist
- The "Claude Code-credentials" keychain item format remains consistent with current Claude Code implementation

## Out of Scope

- Support for non-macOS platforms (Linux, Windows credential stores)
- Alternative credential storage mechanisms (environment variables, config files)
- Migration tooling for existing users (none needed - transparent change)
