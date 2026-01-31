# Claude Code Status Line

A fast, native status line for Claude Code that shows your API usage, session size, and git status at a glance.

**For:** Claude Code users on macOS who want to monitor their usage limits without leaving the terminal.

## Quick Start

```bash
git clone https://github.com/mannerism/awesome-claude-code-status-line.git
cd awesome-claude-code-status-line
./install.sh
```

Then restart Claude Code.

Install a specific version:

```bash
./install.sh --version v0.1.0
```

Check installed vs latest:

```bash
./install.sh --check
```

## Security & Privacy

- Reads Claude Code OAuth credentials from macOS Keychain (`Claude Code-credentials`) to query usage.
- Sends the bearer token only to Anthropic’s usage endpoint: `https://api.anthropic.com/api/oauth/usage`.
- Does **not** transmit prompts, files, or repository data.
- The installer downloads prebuilt binaries from GitHub Releases and verifies SHA256 checksums. No compilation or Rust toolchain required.

## Uninstall

```bash
./uninstall.sh
```

This removes the binary and settings but preserves your Claude Code login.

For complete cleanup (including keychain credentials and backups):

```bash
./uninstall.sh --purge
```

**Note:** `--purge` deletes your `Claude Code-credentials` entry from macOS Keychain.

## Supported Platforms

| Platform | Architecture          | Status           |
| -------- | --------------------- | ---------------- |
| macOS    | Apple Silicon (arm64) | ✅ Supported     |
| macOS    | Intel (x86_64)        | ✅ Supported     |
| Linux    | —                     | ❌ Not supported |
| Windows  | —                     | ❌ Not supported |

**Requirements:**

- macOS 12+
- curl
- jq
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
