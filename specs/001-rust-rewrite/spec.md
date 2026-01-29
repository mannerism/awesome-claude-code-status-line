# Feature Specification: Claude Code Status Line - Rust Rewrite

**Feature Branch**: `001-rust-rewrite`
**Created**: 2026-01-29
**Status**: Draft
**Input**: Rewrite the Claude Code usage tracking status line from Python to Rust, with new session size monitoring feature

## Clarifications

### Session 2026-01-29

- Q: What platforms should be supported? → A: macOS only (arm64 + x86_64), Linux/Windows explicitly excluded
- Q: Should Anthropic API integration be included? → A: API integration required, fail if credentials unavailable
- Q: How should errors be displayed? → A: Both status line (brief) and stderr (detailed)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - View Usage Status Line (Priority: P1)

As a Claude Code user, I want to see my current usage statistics in the status line so I can monitor my quota consumption at a glance.

**Why this priority**: This is the core functionality that the entire tool exists to provide. Without the status line display, there is no product.

**Independent Test**: Run the binary with JSON input containing project path and model info, verify it outputs a properly formatted status line with usage percentages.

**Acceptance Scenarios**:

1. **Given** Claude Code is running with the status line configured, **When** the user performs any action, **Then** the status line displays current project name, model, 5-hour cycle percentage, and weekly usage percentage
2. **Given** JSON input contains model and cwd fields, **When** the binary processes the input, **Then** it correctly identifies the current model and project name
3. **Given** the user is approaching their limits, **When** usage exceeds 75%, **Then** the status line shows red color coding to warn the user

---

### User Story 2 - Session Size Monitoring (Priority: P1)

As a Claude Code user, I want to see my current session file size in the status line so I can avoid session crashes due to excessive context accumulation.

**Why this priority**: Session crashes cause loss of work and are frustrating. Proactive warnings allow users to start a new session before problems occur. This is a new feature that adds significant value.

**Independent Test**: Run the binary with JSON input containing a transcript_path, verify it displays the file size with appropriate color coding based on thresholds.

**Acceptance Scenarios**:

1. **Given** JSON input contains a valid transcript_path, **When** the binary reads the file, **Then** the status line displays the file size in KB or MB format
2. **Given** session file size is less than 5MB, **When** displaying the status, **Then** the size indicator shows green color
3. **Given** session file size is between 5MB and 15MB, **When** displaying the status, **Then** the size indicator shows yellow color with a warning indicator
4. **Given** session file size exceeds 15MB, **When** displaying the status, **Then** the size indicator shows red color with a critical indicator

---

### User Story 3 - Fast Startup Performance (Priority: P2)

As a Claude Code user, I want the status line to update instantly so my workflow is not interrupted by waiting for usage data.

**Why this priority**: The status line is invoked on every Claude Code action. Slow startup would degrade the overall Claude Code experience noticeably.

**Independent Test**: Measure startup time from binary execution to output completion, verify it completes in under 50 milliseconds.

**Acceptance Scenarios**:

1. **Given** the binary is invoked with valid JSON input, **When** processing completes, **Then** total execution time is under 50 milliseconds
2. **Given** the system has many project directories with conversation files, **When** scanning for usage data, **Then** the binary efficiently processes files without noticeable delay

---

### User Story 4 - Cross-Platform Distribution (Priority: P2)

As a user on macOS (either Intel or Apple Silicon), I want to install the binary easily so I can start using the tool without complex setup.

**Why this priority**: Users should not need to compile the tool themselves. Pre-built binaries ensure easy adoption and consistent behavior.

**Independent Test**: Download the appropriate binary for the architecture, run it, verify it executes without any runtime dependencies.

**Acceptance Scenarios**:

1. **Given** a user on macOS with Apple Silicon (arm64), **When** they download the arm64 binary, **Then** it runs without any additional dependencies
2. **Given** a user on macOS with Intel (x86_64), **When** they download the x86_64 binary, **Then** it runs without any additional dependencies
3. **Given** the binary is placed in ~/.local/bin/, **When** the user runs it, **Then** it executes successfully from the PATH

---

### User Story 5 - Git Integration Display (Priority: P3)

As a developer using Git, I want to see my current branch and repository status in the status line so I have context about my working environment.

**Why this priority**: Git status is useful context but not core to quota tracking functionality.

**Independent Test**: Run the binary in a Git repository, verify the branch name and status indicators (modified, untracked, ahead/behind) appear in the output.

**Acceptance Scenarios**:

1. **Given** the current directory is a Git repository, **When** displaying the status line, **Then** the current branch name is shown
2. **Given** the repository has modified files, **When** displaying the status line, **Then** a modified indicator (*) appears
3. **Given** the repository has untracked files, **When** displaying the status line, **Then** an untracked indicator (?) appears
4. **Given** the current directory is not a Git repository, **When** displaying the status line, **Then** no Git information is shown

---

### User Story 6 - Display Preferences (Priority: P3)

As a user, I want to optionally configure display preferences so the status line shows information in my preferred format.

**Why this priority**: Display preferences are nice-to-have but not essential since sensible defaults work for most users.

**Independent Test**: Run the binary with --configure flag, set a preference (e.g., timezone), verify subsequent runs use the saved preference.

**Acceptance Scenarios**:

1. **Given** the user runs the binary with --configure, **When** they set display preferences, **Then** the configuration is saved for future use
2. **Given** no configuration exists, **When** the binary runs, **Then** it uses sensible defaults (local timezone for reset times)

---

### Edge Cases

- What happens when the transcript_path file does not exist? Display session size as "N/A" or omit the field.
- What happens when JSON input is malformed or missing fields? Use sensible defaults (cwd from current directory, default model).
- What happens when Keychain credentials are missing? Display "No creds" in status line; stderr shows detailed message instructing user to authenticate with Claude Code first.
- What happens when Anthropic API request fails (network error, timeout, invalid token)? Display "API error" in status line; stderr shows detailed failure reason and troubleshooting steps.
- What happens when Git operations fail (timeout, not a repo)? Omit Git information from the status line gracefully.

## Requirements *(mandatory)*

### Functional Requirements

#### Input Processing
- **FR-001**: System MUST accept JSON input via stdin containing optional fields: cwd, model, context_window, transcript_path
- **FR-002**: System MUST parse the model display_name field to identify the current model (Sonnet 4, Opus 4, Haiku)
- **FR-003**: System MUST use the current working directory as fallback when cwd is not provided in JSON input
- **FR-004**: System MUST read the transcript_path field to determine session file location for size monitoring

#### Usage Tracking (via Anthropic API)
- **FR-005**: System MUST retrieve OAuth access token from macOS Keychain (entry: "Claude Code-credentials")
- **FR-006**: System MUST fetch usage data from Anthropic API endpoint (https://api.anthropic.com/api/oauth/usage)
- **FR-007**: System MUST display 5-hour cycle utilization percentage from API response
- **FR-008**: System MUST display 7-day utilization percentage from API response
- **FR-009**: System MUST display reset timestamps for both 5-hour and weekly cycles from API response
- **FR-010**: System MUST display errors in two places: brief message in status line output (e.g., "No creds" or "API error") and detailed diagnostic to stderr

#### Session Size Monitoring
- **FR-011**: System MUST read the file size of the transcript file when transcript_path is provided
- **FR-012**: System MUST display file size in human-readable format (KB for < 1MB, MB for >= 1MB)
- **FR-013**: System MUST apply color coding to session size: green for < 5MB, yellow for 5-15MB, red for > 15MB
- **FR-014**: System MUST display warning indicators for elevated session sizes (warning emoji for yellow, critical emoji for red)

#### Status Line Output
- **FR-015**: System MUST output a single-line status string suitable for Claude Code's status line feature
- **FR-016**: System MUST include project name, model name, 5-hour cycle percentage, and weekly percentage in output
- **FR-017**: System MUST include session file size with color coding when transcript_path is available
- **FR-018**: System MUST apply RGB color codes to percentage values based on thresholds (green < 50%, yellow 50-75%, red > 75%)
- **FR-019**: System MUST display time until next 5-hour cycle reset

#### Git Integration
- **FR-020**: System MUST detect if current directory is a Git repository
- **FR-021**: System MUST display current branch name (or commit hash for detached HEAD)
- **FR-022**: System MUST display working tree status indicators (modified *, untracked ?, ahead ↑N, behind ↓N)

#### Configuration
- **FR-023**: System MUST support optional configuration for display preferences (e.g., timezone for reset times)
- **FR-024**: System MUST persist configuration to a local config file if user customizes settings

#### Performance
- **FR-026**: System MUST complete execution and produce output in under 50 milliseconds
- **FR-027**: System MUST compile to a single static binary with no runtime dependencies

#### Distribution
- **FR-028**: System MUST support cross-compilation for macOS arm64 (Apple Silicon)
- **FR-029**: System MUST support cross-compilation for macOS x86_64 (Intel)
- **FR-030**: System MUST install to ~/.local/bin/ directory

### Key Entities

- **JSON Input**: Contains cwd (project path), model (with display_name), context_window (usage info), transcript_path (session file location)
- **API Response**: Contains five_hour (utilization percentage, resets_at timestamp), seven_day (utilization percentage, resets_at timestamp)
- **Keychain Credentials**: OAuth access token stored in macOS Keychain under "Claude Code-credentials"
- **User Configuration**: Optional display preferences (e.g., timezone for reset times)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Status line displays complete usage information within 50 milliseconds of invocation
- **SC-002**: Binary file size is under 5MB for each target architecture
- **SC-003**: Binary runs successfully on macOS arm64 and x86_64 without any runtime dependencies or additional libraries
- **SC-004**: Session size monitoring correctly identifies file sizes with appropriate thresholds (green < 5MB, yellow 5-15MB, red > 15MB)
- **SC-005**: Usage percentages displayed match the values returned by Anthropic API
- **SC-006**: All unit tests pass covering API integration, error handling, and session size determination
- **SC-007**: Installation script successfully places binary in ~/.local/bin/ and updates Claude Code settings
- **SC-008**: Git status information appears within 100ms for repositories with up to 10,000 files

## Out of Scope

- Linux support (not planned for this release)
- Windows support (not planned for this release)
- Local JSONL file parsing for usage calculation (API provides accurate data)
- Offline usage tracking (requires API connectivity)

## Assumptions

- Users are running macOS (arm64 Apple Silicon or x86_64 Intel)
- Users have Claude Code installed and configured with the status line feature enabled
- The ~/.claude/projects/ directory structure follows Claude Code's standard format
- JSONL files use UTF-8 encoding with one JSON object per line
- Git is installed on the system for Git integration features (graceful degradation if not present)
- Users have authenticated with Claude Code (OAuth credentials stored in macOS Keychain)
- Users have network connectivity for API requests
