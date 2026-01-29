#!/bin/bash
# Claude Code Status Line - Uninstaller
#
# Usage: ./uninstall.sh [--purge] [--help]
#
# Options:
#   --purge  Also remove keychain credentials and backup files
#   --help   Show this help message
#
# This script removes all traces of the claude-status installation.
# By default, it preserves:
#   - ~/.claude/settings.json.backup (your original settings backup)
#   - Claude Code keychain credentials (so you stay logged in)
#
# Use --purge to remove everything including backups and keychain entries.

# Don't use set -e - we want to continue even if some removals fail

# === Argument Parsing ===
PURGE=false
SHOW_HELP=false

for arg in "$@"; do
    case $arg in
        --purge)
            PURGE=true
            ;;
        --help|-h)
            SHOW_HELP=true
            ;;
        *)
            echo "Unknown option: $arg"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

if [[ "$SHOW_HELP" == true ]]; then
    echo "Claude Code Status Line - Uninstaller"
    echo ""
    echo "Usage: ./uninstall.sh [--purge] [--help]"
    echo ""
    echo "Options:"
    echo "  --purge  Also remove keychain credentials and backup files"
    echo "  --help   Show this help message"
    echo ""
    echo "Components removed by default:"
    echo "  - Binary: ~/.local/bin/claude-status"
    echo "  - Settings: statusLine config from ~/.claude/settings.json"
    echo "  - Config: ~/.config/claude-status/"
    echo ""
    echo "Additional components removed with --purge:"
    echo "  - Backup: ~/.claude/settings.json.backup"
    echo "  - Keychain: Claude Code credentials"
    exit 0
fi

# === Dependency Check ===
if ! command -v jq &> /dev/null; then
    echo "❌ jq not found. Install it first:"
    echo "   brew install jq"
    exit 1
fi

# === Tracking Arrays ===
removed=()
failed=()
skipped=()

# === Helper Functions ===

# Remove a file if it exists, track result
remove_if_exists() {
    local path="$1"
    local desc="$2"

    if [[ -e "$path" ]]; then
        if rm -f "$path" 2>/dev/null; then
            removed+=("$desc")
            return 0
        else
            failed+=("$desc: permission denied")
            return 1
        fi
    else
        skipped+=("$desc: not found")
        return 0
    fi
}

# Remove a JSON key from settings.json if it exists
remove_json_key() {
    local file="$1"
    local key="$2"
    local desc="$3"

    if [[ ! -f "$file" ]]; then
        skipped+=("$desc: settings.json not found")
        return 0
    fi

    # Check if key exists
    if jq -e ".$key" "$file" &>/dev/null; then
        # Key exists, remove it
        if jq "del(.$key)" "$file" > "$file.tmp" 2>/dev/null; then
            mv "$file.tmp" "$file"
            removed+=("$desc")
            return 0
        else
            rm -f "$file.tmp" 2>/dev/null
            failed+=("$desc: failed to modify settings.json")
            return 1
        fi
    else
        skipped+=("$desc: key not present")
        return 0
    fi
}

echo "🗑️  Uninstalling Claude Code Status Line..."
echo ""

# === Phase 2: User Story 1 - Core Uninstallation ===

# Remove binary
BINARY_PATH="$HOME/.local/bin/claude-status"
remove_if_exists "$BINARY_PATH" "Binary (~/.local/bin/claude-status)"

# Remove statusLine from settings.json
CLAUDE_SETTINGS="$HOME/.claude/settings.json"
remove_json_key "$CLAUDE_SETTINGS" "statusLine" "Settings (statusLine config)"

# Also remove legacy status_line_script if present
remove_json_key "$CLAUDE_SETTINGS" "status_line_script" "Settings (legacy status_line_script)"

# Remove config file
CONFIG_FILE="$HOME/.config/claude-status/config.json"
remove_if_exists "$CONFIG_FILE" "Config file (~/.config/claude-status/config.json)"

# Remove config directory if empty
CONFIG_DIR="$HOME/.config/claude-status"
if [[ -d "$CONFIG_DIR" ]]; then
    if rmdir "$CONFIG_DIR" 2>/dev/null; then
        removed+=("Config directory (~/.config/claude-status/)")
    else
        # Directory not empty - might have user files
        skipped+=("Config directory: not empty (preserved)")
    fi
fi

# === Phase 4 & 5: --purge options ===

if [[ "$PURGE" == true ]]; then
    echo "🔥 Purge mode: removing backup and keychain..."
    echo ""

    # Remove backup file
    BACKUP_FILE="$HOME/.claude/settings.json.backup"
    remove_if_exists "$BACKUP_FILE" "Backup (~/.claude/settings.json.backup)"

    # Remove keychain entry
    if security find-generic-password -s "Claude Code-credentials" &>/dev/null; then
        if security delete-generic-password -s "Claude Code-credentials" &>/dev/null; then
            removed+=("Keychain (Claude Code credentials)")
        else
            failed+=("Keychain: failed to remove (may require authentication)")
        fi
    else
        skipped+=("Keychain: no credentials found")
    fi
else
    # Inform user about preserved items
    if [[ -f "$HOME/.claude/settings.json.backup" ]]; then
        echo "ℹ️  Backup preserved: ~/.claude/settings.json.backup"
        echo "   (use --purge to remove)"
        echo ""
    fi
fi

# === Summary ===

echo "─────────────────────────────────────"

if [[ ${#removed[@]} -gt 0 ]]; then
    echo "✅ Removed:"
    for item in "${removed[@]}"; do
        echo "   • $item"
    done
fi

if [[ ${#failed[@]} -gt 0 ]]; then
    echo ""
    echo "❌ Failed:"
    for item in "${failed[@]}"; do
        echo "   • $item"
    done
fi

if [[ ${#skipped[@]} -gt 0 ]]; then
    echo ""
    echo "⏭️  Skipped:"
    for item in "${skipped[@]}"; do
        echo "   • $item"
    done
fi

echo "─────────────────────────────────────"

# === Exit Status ===

if [[ ${#removed[@]} -eq 0 && ${#failed[@]} -eq 0 ]]; then
    echo ""
    echo "ℹ️  Nothing to uninstall - claude-status was not installed."
    exit 0
fi

if [[ ${#failed[@]} -gt 0 ]]; then
    echo ""
    echo "⚠️  Some items could not be removed. Check permissions and try again."
    exit 1
fi

echo ""
echo "✨ Done! Claude Code Status Line has been uninstalled."

if [[ "$PURGE" != true ]]; then
    echo ""
    echo "Note: Run with --purge to also remove keychain credentials and backups."
fi

exit 0
