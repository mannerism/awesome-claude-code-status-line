# Feature Specification: Claude Code Status Line - Rust Rewrite

**Feature Branch**: `001-rust-rewrite`
**Created**: 2026-01-29
**Status**: Draft
**Input**: Rewrite the Claude Code usage tracking status line from Python to Rust, with new session size monitoring feature

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

### User Story 6 - Configuration Management (Priority: P3)

As a user with a specific Claude subscription tier, I want to configure my limits so the usage percentages are calculated accurately for my plan.

**Why this priority**: Different subscription tiers have different limits. Without configuration, the tool cannot provide accurate usage percentages.

**Independent Test**: Run the binary with --configure flag, select a tier, verify the configuration is persisted and used for subsequent calculations.

**Acceptance Scenarios**:

1. **Given** the user runs the binary with --configure, **When** they select their subscription tier, **Then** the configuration is saved for future use
2. **Given** the user has Max 5x subscription, **When** the status line calculates percentages, **Then** it uses Max 5x limits (50-200 prompts per 5h, separate Sonnet/Opus weekly hours)
3. **Given** no configuration exists, **When** the binary runs, **Then** it uses reasonable defaults (Pro tier limits)

---

### Edge Cases

- What happens when the transcript_path file does not exist? Display session size as "N/A" or omit the field.
- What happens when JSON input is malformed or missing fields? Use sensible defaults (cwd from current directory, default model).
- What happens when ~/.claude/projects directory does not exist? Display 0% usage.
- What happens when the user's subscription tier is not configured? Default to Pro tier limits.
- What happens when Git operations fail (timeout, not a repo)? Omit Git information from the status line gracefully.
- What happens when the 5-hour cycle or week boundary is crossed during execution? Correctly reset counters and display new cycle/week data.

## Requirements *(mandatory)*

### Functional Requirements

#### Input Processing
- **FR-001**: System MUST accept JSON input via stdin containing optional fields: cwd, model, context_window, transcript_path
- **FR-002**: System MUST parse the model display_name field to identify the current model (Sonnet 4, Opus 4, Haiku)
- **FR-003**: System MUST use the current working directory as fallback when cwd is not provided in JSON input
- **FR-004**: System MUST read the transcript_path field to determine session file location for size monitoring

#### Usage Tracking
- **FR-005**: System MUST scan all JSONL conversation files in ~/.claude/projects/ subdirectories
- **FR-006**: System MUST count actual user prompts, excluding meta messages and local command output (messages containing `<command-name>` or `<local-command-stdout>` tags)
- **FR-007**: System MUST calculate 5-hour cycle boundaries using epoch-based formula: cycle_number = floor(seconds_since_epoch / 18000)
- **FR-008**: System MUST calculate weekly boundaries starting from Monday midnight UTC
- **FR-009**: System MUST track session hours separately per model (Sonnet 4 and Opus 4) for Max tier users
- **FR-010**: System MUST detect which model was used in each session by analyzing assistant message model fields

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
- **FR-023**: System MUST support configuration of subscription tier (Free, Pro, Max 5x, Max 20x)
- **FR-024**: System MUST persist configuration to a local config file
- **FR-025**: System MUST load subscription tier limits from configuration for percentage calculations

#### Performance
- **FR-026**: System MUST complete execution and produce output in under 50 milliseconds
- **FR-027**: System MUST compile to a single static binary with no runtime dependencies

#### Distribution
- **FR-028**: System MUST support cross-compilation for macOS arm64 (Apple Silicon)
- **FR-029**: System MUST support cross-compilation for macOS x86_64 (Intel)
- **FR-030**: System MUST install to ~/.local/bin/ directory

### Key Entities

- **JSON Input**: Contains cwd (project path), model (with display_name), context_window (usage info), transcript_path (session file location)
- **JSONL Conversation**: Individual lines with type, timestamp, message.role, message.model, message.content, isMeta, userType fields
- **Usage Data**: Current 5-hour cycle (start_time, total_prompts), current week (start_time, sonnet_hours, opus_hours)
- **User Configuration**: Subscription tier selection determining limit thresholds
- **Subscription Tier**: Defines 5-hour cycle limits and weekly limits per model

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Status line displays complete usage information within 50 milliseconds of invocation
- **SC-002**: Binary file size is under 5MB for each target architecture
- **SC-003**: Binary runs successfully on macOS arm64 and x86_64 without any runtime dependencies or additional libraries
- **SC-004**: Session size monitoring correctly identifies file sizes with appropriate thresholds (green < 5MB, yellow 5-15MB, red > 15MB)
- **SC-005**: Usage percentages match the Python implementation's calculations within 1% accuracy
- **SC-006**: All unit tests pass covering prompt counting, cycle calculations, and session size determination
- **SC-007**: Installation script successfully places binary in ~/.local/bin/ and updates Claude Code settings
- **SC-008**: Git status information appears within 100ms for repositories with up to 10,000 files

## Assumptions

- Users have Claude Code installed and configured with the status line feature enabled
- The ~/.claude/projects/ directory structure follows Claude Code's standard format
- JSONL files use UTF-8 encoding with one JSON object per line
- Git is installed on the system for Git integration features (graceful degradation if not present)
- macOS users have standard Keychain access for API credential retrieval (optional feature)
- Users understand subscription tier terminology (Free, Pro, Max 5x, Max 20x)
