#!/bin/bash
# Install claude-status binary and configure Claude Code settings

set -e

BINARY_NAME="claude-status"
INSTALL_DIR="${HOME}/.local/bin"
CLAUDE_SETTINGS="${HOME}/.claude/settings.json"

# Determine which binary to install
if [[ -f "target/release/${BINARY_NAME}-universal" ]]; then
    SOURCE_BINARY="target/release/${BINARY_NAME}-universal"
elif [[ -f "target/release/${BINARY_NAME}" ]]; then
    SOURCE_BINARY="target/release/${BINARY_NAME}"
else
    echo "Error: No binary found. Run ./scripts/build-release.sh first."
    exit 1
fi

# Create install directory if needed
mkdir -p "$INSTALL_DIR"

# Install binary
echo "Installing ${BINARY_NAME} to ${INSTALL_DIR}..."
cp "$SOURCE_BINARY" "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

echo "Binary installed: ${INSTALL_DIR}/${BINARY_NAME}"

# Check if Claude settings exist
if [[ ! -f "$CLAUDE_SETTINGS" ]]; then
    echo ""
    echo "Claude settings file not found at ${CLAUDE_SETTINGS}"
    echo "Please manually configure the status line in Claude Code settings."
    exit 0
fi

# Backup existing settings
BACKUP="${CLAUDE_SETTINGS}.backup.$(date +%Y%m%d_%H%M%S)"
cp "$CLAUDE_SETTINGS" "$BACKUP"
echo "Backed up existing settings to ${BACKUP}"

# Update settings (simple approach - sets status_line_script)
SCRIPT_PATH="${INSTALL_DIR}/${BINARY_NAME}"

# Check if jq is available for safer JSON manipulation
if command -v jq &> /dev/null; then
    jq --arg script "$SCRIPT_PATH" '.status_line_script = $script' "$CLAUDE_SETTINGS" > "${CLAUDE_SETTINGS}.tmp"
    mv "${CLAUDE_SETTINGS}.tmp" "$CLAUDE_SETTINGS"
    echo "Updated Claude Code settings to use ${BINARY_NAME}"
else
    echo ""
    echo "jq not found - please manually add to ${CLAUDE_SETTINGS}:"
    echo '  "status_line_script": "'"${SCRIPT_PATH}"'"'
fi

echo ""
echo "Installation complete!"
echo "Restart Claude Code to see the status line."
