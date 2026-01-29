# Status Line Output Contract

## Overview

The status line binary outputs a single line to stdout containing formatted usage information with ANSI color codes.

## Output Format

### Full Format (all components present)

```
📁 {project} | 🌿 {branch}{indicators} | 🤖 {model} | 📊 {context}% | ⚡ {5h}% @{reset} | 📅 {weekly}% | 📄 {size}{size_indicator}
```

### Component Definitions

| Component | Format | Example | Condition |
|-----------|--------|---------|-----------|
| Project | `📁 {name}` | `📁 my-project` | Always shown |
| Git Branch | `🌿 {branch}{indicators}` | `🌿 main*?↑2` | Only if in git repo |
| Model | `🤖 {short_name}` | `🤖 O4` | Always shown |
| Context | `📊 {percent}%` | `📊 75%` | Only if context_window in input |
| 5-Hour | `⚡ {percent}% @{reset}` | `⚡ 35% @01/29 14:30` | Always shown (or error) |
| Weekly | `📅 {percent}%` | `📅 68%` | Always shown (or error) |
| Session Size | `📄 {size}{indicator}` | `📄 12.5MB⚠️` | Only if transcript_path valid |

### Git Status Indicators

| Indicator | Meaning |
|-----------|---------|
| `*` | Working tree has modifications |
| `?` | Untracked files exist |
| `↑N` | N commits ahead of upstream |
| `↓N` | N commits behind upstream |

### Model Short Names

| Full Name | Short |
|-----------|-------|
| Sonnet 4 / Sonnet 4.5 | S4 |
| Opus 4 / Opus 4.5 | O4 |
| Haiku / Haiku 4.5 | H |
| Unknown | First 8 chars |

### Session Size Indicators

| Range | Color | Indicator |
|-------|-------|-----------|
| < 5 MB | Green | (none) |
| 5-15 MB | Yellow | ⚠️ |
| > 15 MB | Red | 🔴 |

## Color Coding

All percentage values are color-coded using ANSI 24-bit color escape sequences.

### Usage Thresholds

| Range | RGB | ANSI Escape |
|-------|-----|-------------|
| 0-49% | (0, 255, 0) | `\x1b[38;2;0;255;0m` |
| 50-74% | (255, 255, 0) | `\x1b[38;2;255;255;0m` |
| 75-100% | (255, 100, 100) | `\x1b[38;2;255;100;100m` |

### Color Application

```
{ANSI_COLOR}{value}%{ANSI_RESET}
```

Where `{ANSI_RESET}` is `\x1b[0m`

## Error States

### Credential Error

When Keychain credentials are missing or invalid:

**stdout**: `📁 {project} | 🤖 {model} | ⚠️ No creds`
**stderr**: `Error: Keychain credentials not found. Authenticate with Claude Code first.`

### API Error

When API request fails:

**stdout**: `📁 {project} | 🤖 {model} | ⚠️ API error`
**stderr**: `Error: API request failed: {detailed_reason}`

### Input Error

When stdin JSON is malformed:

**stdout**: `⚠️ Bad input`
**stderr**: `Error: Invalid JSON input: {parse_error}`

## Examples

### Happy Path - Full Output

**Input** (stdin):
```json
{
  "cwd": "/Users/dev/my-project",
  "model": {"display_name": "Opus 4.5"},
  "transcript_path": "/Users/dev/.claude/session.jsonl"
}
```

**Output** (stdout):
```
📁 my-project | 🌿 main*↑2 | 🤖 O4 | ⚡ [GREEN]35%[RESET] @01/29 14:30 | 📅 [YELLOW]68%[RESET] | 📄 2.3MB
```

### No Git, High Usage

**Input** (stdin):
```json
{
  "cwd": "/tmp/test",
  "model": {"display_name": "Sonnet 4"}
}
```

**Output** (stdout):
```
📁 test | 🤖 S4 | ⚡ [RED]85%[RESET] @01/29 14:30 | 📅 [RED]92%[RESET]
```

### Session Size Warning

**Input** (stdin):
```json
{
  "cwd": "/Users/dev/project",
  "model": {"display_name": "Opus 4.5"},
  "transcript_path": "/path/to/large-session.jsonl"
}
```

**Output** (stdout, session file is 8MB):
```
📁 project | 🤖 O4 | ⚡ [GREEN]25%[RESET] @01/29 15:00 | 📅 [GREEN]40%[RESET] | 📄 [YELLOW]8.0MB⚠️[RESET]
```

### Session Size Critical

**Output** (stdout, session file is 18MB):
```
📁 project | 🤖 O4 | ⚡ [GREEN]25%[RESET] @01/29 15:00 | 📅 [GREEN]40%[RESET] | 📄 [RED]18.0MB🔴[RESET]
```

### Missing Credentials

**Output** (stdout):
```
📁 project | 🤖 O4 | ⚠️ No creds
```

**Output** (stderr):
```
Error: Keychain credentials not found.
Please authenticate with Claude Code first by running a Claude Code session.
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (output written to stdout) |
| 1 | Credential error (partial output + stderr message) |
| 2 | API error (partial output + stderr message) |
| 3 | Input error (minimal output + stderr message) |

## Performance Requirements

- Total execution time: < 50ms
- Stdout must be written atomically (single write)
- Stderr written only on errors
