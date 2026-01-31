# Feature Specification: Prebuilt Release Installer

**Feature Branch**: `004-prebuilt-release-installer`
**Created**: 2026-01-31
**Status**: Draft
**Input**: User description: "Retroactively bring the prebuilt-release installer feature into full constitution compliance. Rewrite install.sh from build-from-source to prebuilt GitHub Release download with --version, --check, --help flags. Add CI release workflow. Fix code issues found in review."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Install Latest Release (Priority: P1)

A user clones the repository and runs `./install.sh` to install the Claude Code status line tool. The installer downloads the latest prebuilt universal macOS binary from GitHub Releases, verifies its SHA256 checksum, places it in `~/.local/bin/`, and configures Claude Code settings to use the status line.

**Why this priority**: This is the primary installation path. Without it, no user can install the tool.

**Independent Test**: Can be fully tested by running `./install.sh` on a macOS machine with curl, jq, and Claude Code credentials present. Delivers a working status line installation.

**Acceptance Scenarios**:

1. **Given** a macOS machine with curl, jq, and Claude Code authenticated, **When** the user runs `./install.sh` with no arguments, **Then** the installer fetches the latest release tag from GitHub API, downloads the universal binary archive, verifies its SHA256 checksum, extracts and copies the binary to `~/.local/bin/claude-status`, and configures `~/.claude/settings.json` with the statusLine command.
2. **Given** a macOS machine with curl, jq, and Claude Code authenticated, **When** the user runs `./install.sh` and a `settings.json` already exists, **Then** the installer backs up the existing settings, merges the statusLine configuration (removing any legacy `status_line_script` key), and preserves all other settings.
3. **Given** a macOS machine with curl, jq, and Claude Code authenticated, **When** the user runs `./install.sh` and no `settings.json` exists, **Then** the installer creates `~/.claude/settings.json` with the statusLine configuration.

---

### User Story 2 - Install Specific Version (Priority: P2)

A user wants to pin to a specific release version for stability or compatibility reasons. They run `./install.sh --version vX.Y.Z` to download and install that exact version.

**Why this priority**: Version pinning is essential for reproducible environments and rollback scenarios, but secondary to basic installation.

**Independent Test**: Can be tested by running `./install.sh --version v0.1.0` and verifying the installed binary version matches.

**Acceptance Scenarios**:

1. **Given** a valid release tag exists on GitHub, **When** the user runs `./install.sh --version v0.1.0`, **Then** the installer downloads that specific version's binary and checksum, verifies integrity, and installs it.
2. **Given** a version string without the `v` prefix, **When** the user runs `./install.sh --version 0.1.0`, **Then** the installer automatically prepends `v` and installs `v0.1.0`.
3. **Given** the `--version` flag is provided without a value, **When** the user runs `./install.sh --version`, **Then** the installer exits with a non-zero status and prints an error message explaining that a version value is required.
4. **Given** a non-existent release tag, **When** the user runs `./install.sh --version v99.99.99`, **Then** the installer exits with a non-zero status and prints an error message indicating the release was not found.

---

### User Story 3 - Check Installed vs Latest Version (Priority: P3)

A user wants to know whether they are running the latest version without performing an installation. They run `./install.sh --check` to compare the installed version against the latest GitHub release.

**Why this priority**: Version checking is a convenience feature that supports upgrade decisions but is not required for core functionality.

**Independent Test**: Can be tested by running `./install.sh --check` and verifying it prints both the latest and installed version strings.

**Acceptance Scenarios**:

1. **Given** the tool is installed and a newer version exists on GitHub, **When** the user runs `./install.sh --check`, **Then** the output shows both the latest release tag and the currently installed version.
2. **Given** the tool is not installed, **When** the user runs `./install.sh --check`, **Then** the output shows the latest release tag and "not installed" for the installed version.
3. **Given** the GitHub API is unreachable, **When** the user runs `./install.sh --check`, **Then** the installer exits with a non-zero status and prints an error indicating it could not reach the GitHub API.

---

### User Story 4 - CI Builds and Publishes Release (Priority: P3)

A maintainer pushes a semver tag (e.g., `v0.2.0`) to the repository. GitHub Actions builds universal macOS binaries for both arm64 and x86_64 architectures, creates a release archive with SHA256 checksums, and publishes them as release assets.

**Why this priority**: The CI pipeline is infrastructure that supports all other stories but is not user-facing.

**Independent Test**: Can be tested by pushing a tag to a fork and verifying that the GitHub Actions workflow produces a release with the expected assets (`claude-status-macos-universal.tar.gz` and `sha256.txt`).

**Acceptance Scenarios**:

1. **Given** the repository has the release workflow configured, **When** a tag matching `v*` is pushed, **Then** GitHub Actions builds arm64 and x86_64 binaries, creates a universal binary via `lipo`, packages it as `claude-status-macos-universal.tar.gz`, generates `sha256.txt`, and uploads both as release assets.
2. **Given** the release workflow is triggered, **When** the build runs, **Then** it MUST pass `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test --all-features` before producing any artifacts.
3. **Given** any quality gate fails (formatting, linting, or tests), **When** the workflow runs, **Then** the release is NOT published and the workflow fails with a clear error.

---

### Edge Cases

- **curl not installed**: Installer MUST exit with non-zero status and print a message indicating curl is required.
- **jq not installed**: Installer MUST exit with non-zero status and print a message indicating jq is required with a `brew install jq` hint.
- **GitHub API unreachable or rate-limited**: Installer MUST exit with non-zero status and print a descriptive error. The `curl -sSfL` flags cause curl to fail on HTTP errors, which `set -e` propagates.
- **Checksum verification fails**: Installer MUST exit with non-zero status. The `shasum -a 256 -c` command returns non-zero on mismatch, which `set -e` propagates.
- **Checksum file unavailable**: Installer MUST print a warning that integrity was not verified but SHOULD continue with installation (user accepts risk).
- **Downloaded archive missing binary**: Installer MUST exit with non-zero status and print a message indicating the archive does not contain the expected binary.
- **No Claude Code credentials in Keychain**: Installer MUST exit with non-zero status and print step-by-step instructions for authenticating Claude Code.
- **Unknown CLI flags**: Installer MUST warn about unrecognized arguments and exit with non-zero status rather than silently ignoring them.
- **Claude Code CLI not installed**: Installer MUST exit with non-zero status and print the `npm install` command for installing Claude Code.
- **`~/.local/bin` does not exist**: Installer MUST create the directory before attempting to copy the binary.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Installer MUST download prebuilt binaries from GitHub Releases instead of compiling from source.
- **FR-002**: Installer MUST verify downloaded archive integrity via SHA256 checksum when the checksum file is available.
- **FR-003**: Installer MUST warn the user when checksum verification is skipped due to unavailable checksum file.
- **FR-004**: Installer MUST support `--version <tag>` flag to install a specific release version.
- **FR-005**: Installer MUST support `--check` flag to display installed vs latest version without installing.
- **FR-006**: Installer MUST support `--help` / `-h` flag to display usage information.
- **FR-007**: Installer MUST reject unrecognized CLI arguments with a warning and non-zero exit.
- **FR-008**: Installer MUST validate all prerequisites (curl, jq, Claude CLI, Keychain credentials) before downloading.
- **FR-009**: Installer MUST configure `~/.claude/settings.json` with the correct `statusLine` format, removing any legacy `status_line_script` key.
- **FR-010**: Installer MUST back up existing `settings.json` before modifying it.
- **FR-011**: Installer MUST clean up temporary files on exit (success or failure) via trap.
- **FR-012**: CI release workflow MUST run quality gates (`cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test --all-features`) before building release artifacts.
- **FR-013**: CI release workflow MUST build universal macOS binary supporting both arm64 and x86_64 architectures.
- **FR-014**: CI release workflow MUST generate and publish SHA256 checksums alongside release archives.
- **FR-015**: CI release workflow MUST trigger only on tags matching `v*`.

### Key Entities

- **Release**: A GitHub Release identified by a semver tag (e.g., `v0.2.0`), containing a universal macOS binary archive and a SHA256 checksum file.
- **Installer**: The `install.sh` bash script responsible for downloading, verifying, and installing the binary and configuring Claude Code.
- **Settings**: The `~/.claude/settings.json` file that Claude Code reads to determine the status line command.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can install the tool in under 30 seconds on a standard broadband connection (no compilation step).
- **SC-002**: Installation succeeds on both Apple Silicon (arm64) and Intel (x86_64) Macs using a single binary.
- **SC-003**: Every released binary has a verifiable SHA256 checksum published alongside it.
- **SC-004**: No release is published without passing formatting, linting, and test quality gates.
- **SC-005**: Users receive clear, actionable error messages for every failure mode (missing prerequisites, network errors, checksum failures, unknown flags).
- **SC-006**: Users can pin to any previously released version and install it reproducibly.
- **SC-007**: Users can check whether an update is available without modifying their installation.

### Assumptions

- Target platform is macOS 12+ only (no Linux/Windows support in this feature).
- GitHub Releases is the sole distribution mechanism for prebuilt binaries.
- The `security` CLI tool is available on all supported macOS versions for Keychain access.
- Users have internet access during installation.
- The `v` prefix convention is used for all release tags (semver with `v` prefix).
