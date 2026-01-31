# Quickstart Verification: Prebuilt Release Installer

**Feature**: 004-prebuilt-release-installer
**Date**: 2026-01-31

## Prerequisites

- macOS 12+
- `curl` installed
- `jq` installed (`brew install jq`)
- Claude Code installed and authenticated (`claude` → `/login`)

## Verification Steps

### 1. Verify install.sh argument handling

```bash
# Should print usage and exit 0
./install.sh --help

# Should error on unknown flag
./install.sh --foo
echo $?  # Expected: 1

# Should error on missing version value
./install.sh --version
echo $?  # Expected: 1
```

### 2. Verify version check (requires GitHub access)

```bash
./install.sh --check
# Expected output:
# Latest:    vX.Y.Z
# Installed: <version or "not installed">
```

### 3. Verify full installation (requires GitHub release + Claude Code auth)

```bash
./install.sh
# Expected: Downloads, verifies checksum, installs to ~/.local/bin/claude-status
# Configures ~/.claude/settings.json

# Verify binary exists and runs
~/.local/bin/claude-status --version

# Verify settings configured
cat ~/.claude/settings.json | jq .statusLine
# Expected: {"type": "command", "command": "~/.local/bin/claude-status"}
```

### 4. Verify specific version install

```bash
./install.sh --version v0.1.0
~/.local/bin/claude-status --version
# Expected: version matches v0.1.0
```

### 5. Verify CI workflow (requires tag push)

```bash
# On a fork or the main repo:
git tag v0.0.1-test
git push origin v0.0.1-test

# Check GitHub Actions:
# - Quality gates (fmt, clippy, test) should run first
# - Build produces claude-status-macos-universal.tar.gz
# - sha256.txt is published alongside

# Cleanup:
git tag -d v0.0.1-test
git push origin :refs/tags/v0.0.1-test
```

### 6. Verify shellcheck passes

```bash
shellcheck install.sh
# Expected: no warnings or errors
```

## Quality Gates

```bash
# Rust binary quality (unchanged but enforced by CI)
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all-features

# Shell script quality
shellcheck install.sh
```
