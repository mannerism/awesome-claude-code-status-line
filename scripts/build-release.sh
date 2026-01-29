#!/bin/bash
# Build release binaries for macOS arm64 and x86_64

set -e

echo "Building release binaries..."

# Build for current architecture
cargo build --release

# Check if cross-compilation targets are available
if rustup target list | grep -q "aarch64-apple-darwin (installed)"; then
    echo "Building for aarch64-apple-darwin..."
    cargo build --release --target aarch64-apple-darwin
fi

if rustup target list | grep -q "x86_64-apple-darwin (installed)"; then
    echo "Building for x86_64-apple-darwin..."
    cargo build --release --target x86_64-apple-darwin
fi

echo "Build complete."
echo ""
echo "Binary locations:"
ls -lh target/release/claude-status 2>/dev/null || true
ls -lh target/aarch64-apple-darwin/release/claude-status 2>/dev/null || true
ls -lh target/x86_64-apple-darwin/release/claude-status 2>/dev/null || true
