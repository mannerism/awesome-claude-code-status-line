# Data Model: Prebuilt Release Installer

**Feature**: 004-prebuilt-release-installer
**Date**: 2026-01-31

## Entities

### GitHub Release

A versioned distribution point on GitHub containing binary artifacts.

| Attribute     | Type   | Description                                   | Example                                   |
| ------------- | ------ | --------------------------------------------- | ----------------------------------------- |
| tag_name      | string | Semver tag with `v` prefix                    | `v0.2.0`                                  |
| archive_url   | URL    | Download URL for the binary archive           | `https://github.com/.../claude-status-macos-universal.tar.gz` |
| checksum_url  | URL    | Download URL for the SHA256 checksum file     | `https://github.com/.../sha256.txt`       |

**Source**: GitHub Releases API (`/repos/{owner}/{repo}/releases/latest`)

### Installer CLI Interface

The `install.sh` script accepts the following flags:

| Flag              | Type    | Required | Default       | Description                          |
| ----------------- | ------- | -------- | ------------- | ------------------------------------ |
| `--version`, `-v` | string  | No       | latest        | Specific release tag to install      |
| `--check`         | boolean | No       | false         | Compare installed vs latest, no install |
| `--help`, `-h`    | boolean | No       | false         | Print usage and exit                 |

**Validation rules**:
- `--version` requires a non-empty value that does not start with `--`
- Unknown flags cause immediate exit with error
- `--help` takes precedence (checked first, exits before any network calls)

### Claude Code Settings

The `~/.claude/settings.json` file managed by the installer.

| Field               | Type   | Description                               | Value                                      |
| ------------------- | ------ | ----------------------------------------- | ------------------------------------------ |
| `statusLine.type`   | string | Status line provider type                 | `"command"`                                |
| `statusLine.command` | string | Path to status line binary               | `"~/.local/bin/claude-status"`             |

**Lifecycle**:
1. If file exists: backup → read → merge (add `statusLine`, remove `status_line_script`) → write
2. If file missing: create directory → write new file with `statusLine` only

## Flow: Install Latest (US1)

```text
┌─────────────────┐
│ Parse CLI args   │
└────────┬────────┘
         │ no flags
         ▼
┌─────────────────┐     ┌──────────────┐
│ Check curl      │──✗──│ Exit 1       │
└────────┬────────┘     └──────────────┘
         │ ✓
         ▼
┌─────────────────┐     ┌──────────────┐
│ Check jq        │──✗──│ Exit 1       │
└────────┬────────┘     └──────────────┘
         │ ✓
         ▼
┌─────────────────┐
│ Fetch latest tag │
│ (GitHub API)     │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌──────────────┐
│ Check claude CLI│──✗──│ Exit 1       │
└────────┬────────┘     └──────────────┘
         │ ✓
         ▼
┌─────────────────┐     ┌──────────────┐
│ Check Keychain  │──✗──│ Exit 1       │
│ credentials     │     │ (with steps) │
└────────┬────────┘     └──────────────┘
         │ ✓
         ▼
┌─────────────────┐
│ Download archive │
│ to tmp dir       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌──────────────┐
│ Download SHA256 │──✗──│ Warn: skip   │──┐
└────────┬────────┘     └──────────────┘  │
         │ ✓                               │
         ▼                                 │
┌─────────────────┐                        │
│ Verify checksum │                        │
└────────┬────────┘                        │
         │ ✓                               │
         ▼◄────────────────────────────────┘
┌─────────────────┐
│ Extract + copy   │
│ to ~/.local/bin  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Configure        │
│ settings.json    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Done ✨          │
└─────────────────┘
```

## Flow: CI Release (US4)

```text
┌─────────────────┐
│ Tag push (v*)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐     ┌──────────────┐
│ cargo fmt       │──✗──│ Fail workflow │
│ --check         │     └──────────────┘
└────────┬────────┘
         │ ✓
         ▼
┌─────────────────┐     ┌──────────────┐
│ cargo clippy    │──✗──│ Fail workflow │
│ -D warnings     │     └──────────────┘
└────────┬────────┘
         │ ✓
         ▼
┌─────────────────┐     ┌──────────────┐
│ cargo test      │──✗──│ Fail workflow │
│ --all-features  │     └──────────────┘
└────────┬────────┘
         │ ✓
         ▼
┌─────────────────┐
│ Build arm64     │
│ Build x86_64    │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ lipo → universal│
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ tar.gz + sha256 │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Upload to       │
│ GitHub Release  │
└─────────────────┘
```
