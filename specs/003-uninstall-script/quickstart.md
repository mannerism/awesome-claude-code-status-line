# Quickstart: Uninstall Script

## Usage

### Standard Uninstall (Safe)

```bash
./uninstall.sh
```

Removes:
- Binary (`~/.local/bin/claude-status`)
- statusLine configuration from `~/.claude/settings.json`
- Config file and directory (`~/.config/claude-status/`)

Preserves:
- `~/.claude/settings.json.backup` (your original settings backup)
- Claude Code keychain credentials (so you stay logged in)

### Complete Purge

```bash
./uninstall.sh --purge
```

Removes everything above PLUS:
- `~/.claude/settings.json.backup`
- Claude Code keychain credentials (you'll need to re-login)

## Verification

After uninstalling, verify with:

```bash
# Check binary is gone
which claude-status
# Should return: "claude-status not found" or empty

# Check settings are clean
cat ~/.claude/settings.json | jq '.statusLine'
# Should return: null

# Check config directory is gone
ls ~/.config/claude-status/
# Should return: "No such file or directory"
```

## Re-installation

To re-install after uninstalling:

```bash
./install.sh
```

If you used `--purge`, you'll be prompted to log in to Claude Code again.

## Troubleshooting

### "jq: command not found"

Install jq first:
```bash
brew install jq
```

### "Permission denied"

Make the script executable:
```bash
chmod +x uninstall.sh
```

### Some files couldn't be removed

Check if another process is using the binary:
```bash
lsof | grep claude-status
```

Close Claude Code and try again.
