# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A high-performance Rust-based status line for Claude Code that displays API usage, session size, and git status. It fetches real-time usage data from the Anthropic API and displays color-coded warnings.

## Key Commands

```bash
# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt

# Install
./install.sh
```

## Architecture

```
src/
├── lib.rs              # Library root
├── main.rs             # CLI entry point
├── error.rs            # StatusLineError enum
├── api/
│   ├── client.rs       # Anthropic API client
│   └── keychain.rs     # macOS Keychain access
├── config/
│   └── mod.rs          # User configuration
├── display/
│   ├── colors.rs       # ANSI color support
│   └── status_line.rs  # StatusLineBuilder
├── domain/
│   ├── git.rs          # GitStatus types
│   ├── input.rs        # ClaudeInput, Model
│   ├── session.rs      # SessionSize
│   └── usage.rs        # UsagePercentage, CycleInfo
└── git/
    └── status.rs       # get_git_status()
```

## Key Dependencies

- `serde` / `serde_json`: JSON serialization
- `ureq`: Blocking HTTP client
- `security-framework`: macOS Keychain access
- `chrono`: Timestamp handling
- `clap`: CLI argument parsing
- `thiserror`: Error handling

## Status Line Format

```
📁 project | 🌿 branch*?↑2↓1 | 🤖 O4 | ⚡ 35% @14:30 | 📅 68% | 📄 2.0MB
```

- 📁 = Project name
- 🌿 = Git branch (* modified, ? untracked, ↑/↓ ahead/behind)
- 🤖 = Model (S4=Sonnet, O4=Opus, H=Haiku)
- ⚡ = 5-hour usage with reset time
- 📅 = 7-day usage
- 📄 = Session size (green <5MB, yellow 5-15MB, red >15MB)

## File Locations

- **Binary**: `~/.local/bin/claude-status`
- **Config**: `~/.config/claude-status/config.json` (optional)
- **Claude Settings**: `~/.claude/settings.json`

## Active Technologies
- Rust 1.75+ (MSRV documented in Cargo.toml) + serde, serde_json, ureq, chrono, clap, thiserror (unchanged); removing security-framework (002-security-cli-keychain)
- N/A (reading from macOS Keychain via CLI) (002-security-cli-keychain)

## Recent Changes
- 002-security-cli-keychain: Added Rust 1.75+ (MSRV documented in Cargo.toml) + serde, serde_json, ureq, chrono, clap, thiserror (unchanged); removing security-framework
