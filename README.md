# Claude Code Status Line

A fast, native status line for Claude Code that shows your API usage, session size, and git status at a glance.

**For:** Claude Code users on macOS who want to monitor their usage limits without leaving the terminal.

## Quick Start

```bash
git clone <repo-url>
cd claude-code-status-line
./install.sh
```

Then restart Claude Code.

## Uninstall

```bash
./uninstall.sh
```

This removes the binary and settings but preserves your Claude Code login.

For complete cleanup (including keychain credentials and backups):

```bash
./uninstall.sh --purge
```

## Supported Platforms

| Platform | Architecture          | Status           |
| -------- | --------------------- | ---------------- |
| macOS    | Apple Silicon (arm64) | ✅ Supported     |
| macOS    | Intel (x86_64)        | ✅ Supported     |
| Linux    | —                     | ❌ Not supported |
| Windows  | —                     | ❌ Not supported |

**Requirements:**

- macOS 12+
- Rust 1.75+ (for building from source)
- Claude Code installed and signed in

## How It Works

Once installed, your Claude Code status line will show:

```
📁 my-project | 🌿 main*↑2 | 🤖 O4 | ⚡ 35% @14:30 | 📅 68% | 📄 2.0MB
```

| Icon | Meaning                                                           |
| ---- | ----------------------------------------------------------------- |
| 📁   | Current project name                                              |
| 🌿   | Git branch (`*` = modified, `?` = untracked, `↑↓` = ahead/behind) |
| 🤖   | Model (S4 = Sonnet, O4 = Opus, H = Haiku)                         |
| ⚡   | 5-hour usage cycle with reset time                                |
| 📅   | 7-day usage cycle                                                 |
| 📄   | Session size (green < 5MB, yellow 5-15MB, red > 15MB)             |

## License

MIT
