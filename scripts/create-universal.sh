#!/bin/bash
# Create a universal binary for macOS (arm64 + x86_64)

set -e

BINARY_NAME="claude-status"
ARM64_BINARY="target/aarch64-apple-darwin/release/${BINARY_NAME}"
X86_64_BINARY="target/x86_64-apple-darwin/release/${BINARY_NAME}"
UNIVERSAL_BINARY="target/release/${BINARY_NAME}-universal"

# Check if both architecture binaries exist
if [[ ! -f "$ARM64_BINARY" ]]; then
    echo "Error: ARM64 binary not found at $ARM64_BINARY"
    echo "Run: rustup target add aarch64-apple-darwin && cargo build --release --target aarch64-apple-darwin"
    exit 1
fi

if [[ ! -f "$X86_64_BINARY" ]]; then
    echo "Error: x86_64 binary not found at $X86_64_BINARY"
    echo "Run: rustup target add x86_64-apple-darwin && cargo build --release --target x86_64-apple-darwin"
    exit 1
fi

echo "Creating universal binary..."
lipo -create -output "$UNIVERSAL_BINARY" "$ARM64_BINARY" "$X86_64_BINARY"

echo "Universal binary created:"
ls -lh "$UNIVERSAL_BINARY"
file "$UNIVERSAL_BINARY"
