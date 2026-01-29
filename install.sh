#!/bin/bash
# Claude Code Status Line - One-step installer

set -e

echo "🚀 Installing Claude Code Status Line..."
echo ""

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust not found. Install it first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check for jq
if ! command -v jq &> /dev/null; then
    echo "❌ jq not found. Install it first:"
    echo "   brew install jq"
    exit 1
fi

# Check for Claude CLI
if ! command -v claude &> /dev/null; then
    echo "❌ Claude Code CLI not found. Install it first:"
    echo "   npm install -g @anthropic-ai/claude-code"
    exit 1
fi

# Check for Claude Code credentials
echo "🔐 Checking Claude Code authentication..."
if ! security find-generic-password -s "Claude Code-credentials" &>/dev/null; then
    echo ""
    echo "⚠️  Not signed in to Claude Code."
    echo ""
    echo "Opening login page..."
    echo ""

    # Run claude login command (this opens OAuth in browser)
    claude /login

    # Wait and verify credentials exist
    echo ""
    echo "Waiting for authentication to complete..."
    for i in {1..30}; do
        if security find-generic-password -s "Claude Code-credentials" &>/dev/null; then
            echo "✅ Authentication successful!"
            break
        fi
        sleep 2
    done

    if ! security find-generic-password -s "Claude Code-credentials" &>/dev/null; then
        echo "❌ Authentication timed out. Please try again."
        exit 1
    fi
else
    echo "✅ Claude Code authenticated"
fi

# Build release binary
echo "📦 Building..."
cargo build --release --quiet

# Create install directory
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Copy binary
cp target/release/claude-status "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/claude-status"
echo "✅ Installed to $INSTALL_DIR/claude-status"

# Update Claude Code settings
CLAUDE_SETTINGS="$HOME/.claude/settings.json"

if [[ -f "$CLAUDE_SETTINGS" ]]; then
    # Backup existing settings
    cp "$CLAUDE_SETTINGS" "$CLAUDE_SETTINGS.backup"

    # Remove old status_line_script if exists, set correct statusLine format
    jq 'del(.status_line_script) | .statusLine = {"type": "command", "command": "'"$INSTALL_DIR/claude-status"'"}' "$CLAUDE_SETTINGS" > "$CLAUDE_SETTINGS.tmp"
    mv "$CLAUDE_SETTINGS.tmp" "$CLAUDE_SETTINGS"
    echo "✅ Updated Claude Code settings"
else
    # Create new settings file
    mkdir -p "$HOME/.claude"
    echo '{"statusLine": {"type": "command", "command": "'"$INSTALL_DIR/claude-status"'"}}' | jq . > "$CLAUDE_SETTINGS"
    echo "✅ Created Claude Code settings"
fi

echo ""
echo "✨ Done! Restart Claude Code to see your status line."
