#!/bin/bash
# Claude Code Status Line - One-step installer (prebuilt release)

set -e

VERSION=""
SHOW_HELP=false
CHECK_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --version|-v)
            if [[ -z "${2:-}" ]]; then
                echo "❌ Missing value for --version"
                exit 1
            fi
            VERSION="$2"
            shift 2
            ;;
        --check)
            CHECK_ONLY=true
            shift
            ;;
        --help|-h)
            SHOW_HELP=true
            shift
            ;;
        *)
            echo "❌ Unknown option: $1" >&2
            echo "   Run './install.sh --help' for usage information." >&2
            exit 1
            ;;
    esac
done

if [[ "$SHOW_HELP" == true ]]; then
    echo "Claude Code Status Line - Installer"
    echo ""
    echo "Usage: ./install.sh [--version vX.Y.Z]"
    echo ""
    echo "Options:"
    echo "  --version, -v   Install a specific release tag (e.g., v0.1.0)"
    echo "  --check         Show installed vs latest version and exit"
    echo "  --help, -h      Show this help"
    exit 0
fi

echo "🚀 Installing Claude Code Status Line..."
echo ""

# Create install directory (needed for --check)
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# Check for curl
if ! command -v curl &> /dev/null; then
    echo "❌ curl not found. Install it first."
    exit 1
fi

# Check for jq
if ! command -v jq &> /dev/null; then
    echo "❌ jq not found. Install it first:"
    echo "   brew install jq"
    exit 1
fi

# Resolve version
if [[ -z "$VERSION" ]]; then
    echo "🔎 Fetching latest release..."
    RELEASE_JSON=$(curl -sSfL "https://api.github.com/repos/mannerism/awesome-claude-code-status-line/releases/latest" 2>/dev/null) || {
        echo "❌ Could not fetch latest release from GitHub."
        echo "   This may mean no releases exist yet, or GitHub API is unreachable."
        exit 1
    }
    VERSION=$(echo "$RELEASE_JSON" | jq -r .tag_name)
    if [[ -z "$VERSION" || "$VERSION" == "null" ]]; then
        echo "❌ No release tag found. Push a tag first: git tag v0.1.0 && git push origin v0.1.0"
        exit 1
    fi
fi

if [[ "$VERSION" != v* ]]; then
    VERSION="v$VERSION"
fi

ASSET="claude-status-macos-universal.tar.gz"
BASE_URL="https://github.com/mannerism/awesome-claude-code-status-line/releases/download/$VERSION"
ARCHIVE_URL="$BASE_URL/$ASSET"
SHA_URL="$BASE_URL/sha256.txt"

if [[ "$CHECK_ONLY" == true ]]; then
    INSTALLED_VERSION=$("$INSTALL_DIR/claude-status" --version 2>/dev/null | awk '{print $2}')
    echo "Latest:    $VERSION"
    echo "Installed: ${INSTALLED_VERSION:-not installed}"
    exit 0
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
    echo "Please login first:"
    echo "  1. Run: claude"
    echo "  2. Type: /login"
    echo "  3. Complete the login in your browser"
    echo "  4. Exit Claude Code (Ctrl+C or type 'exit')"
    echo "  5. Re-run: ./install.sh"
    echo ""
    exit 1
else
    echo "✅ Claude Code authenticated"
fi

echo "📦 Downloading $VERSION..."
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT
curl -sSfL "$ARCHIVE_URL" -o "$TMP_DIR/$ASSET"

# Verify checksum if available
if curl -sSfL "$SHA_URL" -o "$TMP_DIR/sha256.txt" 2>/dev/null; then
    (cd "$TMP_DIR" && shasum -a 256 -c sha256.txt)
else
    echo "⚠️  Checksum file not available. Skipping integrity verification."
fi

tar -xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"
if [[ ! -f "$TMP_DIR/claude-status" ]]; then
    echo "❌ Downloaded archive missing claude-status binary"
    exit 1
fi

# Copy binary
cp "$TMP_DIR/claude-status" "$INSTALL_DIR/"
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
