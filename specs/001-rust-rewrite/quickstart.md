# Quickstart: Claude Code Status Line (Rust)

## Prerequisites

- macOS (arm64 or x86_64)
- Rust 1.75+ installed (`rustup` recommended)
- Claude Code installed and authenticated
- Git (for development)

## Building from Source

### 1. Clone and Setup

```bash
# Clone the repository
git clone https://github.com/your-org/claude-code-status-line.git
cd claude-code-status-line

# Switch to the rust rewrite branch
git checkout 001-rust-rewrite
```

### 2. Build

```bash
# Debug build (faster compile, slower runtime)
cargo build

# Release build (optimized, use for installation)
cargo build --release
```

### 3. Test

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

### 4. Install

**Option A: Using the install script (recommended)**

```bash
./scripts/install.sh
```

This will:
- Copy the binary to `~/.local/bin/`
- Update Claude Code settings automatically
- Create a backup of existing settings

**Option B: Manual installation**

```bash
# Install to ~/.local/bin/
mkdir -p ~/.local/bin
cp target/release/claude-status ~/.local/bin/

# Ensure ~/.local/bin is in PATH
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Quick Verification

```bash
# Test with sample input
echo '{"cwd":"/tmp/test","model":{"display_name":"Opus 4.5"}}' | claude-status

# Expected output (colors will vary):
# 📁 test | 🤖 O4 | ⚡ 35% @01/29 14:30 | 📅 68%
```

## Integrating with Claude Code

### 1. Update Claude Code Settings

Edit `~/.claude/settings.json`:

```json
{
  "status_line": {
    "command": ["~/.local/bin/claude-status"]
  }
}
```

### 2. Restart Claude Code

Close and reopen Claude Code to apply changes.

## Development Workflow

### Running During Development

```bash
# Build and run with test input
cargo run -- < test-input.json

# Watch mode (rebuild on changes)
cargo watch -x run
```

### Running Tests

```bash
# Unit tests only
cargo test --lib

# Integration tests
cargo test --test '*'

# With coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Check docs compile
cargo doc --no-deps
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Profile specific benchmark
cargo bench --bench status_line
```

## Troubleshooting

### "No creds" Error

**Cause**: Keychain credentials not found.

**Solution**:
1. Open Claude Code application
2. Authenticate (sign in)
3. Verify credentials exist: `security find-generic-password -s "Claude Code-credentials"`

### "API error"

**Cause**: Network issue or invalid token.

**Solution**:
1. Check internet connection
2. Try re-authenticating in Claude Code
3. Check stderr for detailed error message

### Build Fails on macOS

**Cause**: Missing Xcode command line tools.

**Solution**:
```bash
xcode-select --install
```

### Binary Not Found

**Cause**: `~/.local/bin` not in PATH.

**Solution**:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Cross-Compilation (Universal Binary)

Build for both architectures:

```bash
# Add targets
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Build both
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create universal binary
lipo -create \
    target/aarch64-apple-darwin/release/claude-status \
    target/x86_64-apple-darwin/release/claude-status \
    -output target/release/claude-status-universal
```

## Next Steps

1. Review [spec.md](./spec.md) for full requirements
2. Review [data-model.md](./data-model.md) for type definitions
3. Check [contracts/](./contracts/) for API schemas
4. Run `/speckit.tasks` to generate implementation tasks
