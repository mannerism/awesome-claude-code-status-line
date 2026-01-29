# Research: Claude Code Status Line - Rust Rewrite

**Date**: 2026-01-29
**Branch**: `001-rust-rewrite`

## Executive Summary

All technical unknowns have been resolved. The Rust rewrite is feasible with the following key decisions:
- **HTTP Client**: `ureq` (blocking, minimal, fast startup)
- **Keychain Access**: `security-framework` crate (native macOS API)
- **Performance**: Achievable <50ms startup, <5MB binary with proper optimization
- **Distribution**: Universal binary via `lipo` (arm64 + x86_64)

---

## 1. macOS Keychain Access

### Decision
Use `security-framework` crate (v3.x) for direct Keychain access.

### Rationale
- Direct low-level control over Apple's Security.framework APIs
- No additional abstraction layers
- Well-maintained, production-grade crate
- Required for reading the "Claude Code-credentials" entry format

### Alternatives Considered
| Crate | Pros | Cons | Verdict |
|-------|------|------|---------|
| `keyring` | Cross-platform, high-level API | Extra abstraction, uses security-framework internally | Not needed for macOS-only |
| `security-framework` | Direct API, minimal overhead | macOS-only | **Selected** |
| `keychain-services` | Secure Enclave support | Experimental, requires code signing | Overkill |

### Implementation Notes

**Credential Entry Format**:
- Service Name: `"Claude Code-credentials"`
- Account: Can be empty or any value (service name is the key lookup)
- Value: JSON containing OAuth tokens

**JSON Structure**:
```json
{
  "claudeAiOauth": {
    "accessToken": "sk-ant-oat01-...",
    "refreshToken": "sk-ant-ort01-...",
    "expiresAt": 1748276587173,
    "scopes": ["user:inference", "user:profile"]
  }
}
```

**Retrieval Code Pattern**:
```rust
use security_framework::os::macos::keychain::SecKeychain;
use security_framework::os::macos::passwords::find_generic_password;

fn get_access_token() -> Result<String, Error> {
    let (password, _item) = find_generic_password(None, "Claude Code-credentials", "")?;
    let json: serde_json::Value = serde_json::from_slice(&password)?;
    let token = json["claudeAiOauth"]["accessToken"]
        .as_str()
        .ok_or(Error::MissingToken)?;
    Ok(token.to_string())
}
```

### Cross-Compilation Considerations
- Must compile natively on macOS (no Linux cross-compile possible)
- Use `lipo` to create universal binary from arm64 + x86_64 builds
- Both architectures can be built from any macOS machine

---

## 2. Anthropic OAuth Usage API

### Decision
Use the existing API endpoint with documented headers and response format.

### API Contract

**Endpoint**: `GET https://api.anthropic.com/api/oauth/usage`

**Request Headers**:
```
Authorization: Bearer {access_token}
anthropic-beta: oauth-2025-04-20
User-Agent: claude-code/2.0.31
Content-Type: application/json
```

**Response Schema**:
```json
{
  "five_hour": {
    "utilization": 35.5,        // 0-100 percentage
    "resets_at": "2025-01-29T15:30:00Z"  // ISO8601 UTC
  },
  "seven_day": {
    "utilization": 68.2,        // 0-100 percentage
    "resets_at": "2025-02-03T00:00:00Z"  // ISO8601 UTC
  }
}
```

### Error Handling Strategy
| Scenario | HTTP Status | Action |
|----------|-------------|--------|
| Success | 200 | Parse response, display usage |
| Invalid token | 401 | Display "No creds", stderr: authenticate |
| Expired token | 401/403 | Display "No creds", stderr: re-authenticate |
| Network error | N/A | Display "API error", stderr: check connection |
| Timeout (>5s) | N/A | Display "API error", stderr: timeout message |
| Invalid JSON | N/A | Display "API error", stderr: parse error |

### Timeout Configuration
- Request timeout: 5 seconds (matches Python implementation)
- Fail fast to avoid blocking status line updates

---

## 3. HTTP Client Selection

### Decision
Use `ureq` crate (v2.9+) for blocking HTTP requests.

### Rationale
- **Startup Time**: Pure blocking I/O, no async runtime initialization (saves 5-15ms)
- **Binary Size**: Minimal dependency tree (~500KB contribution vs ~2MB for reqwest)
- **Simplicity**: Synchronous API matches CLI use case perfectly
- **Reliability**: Well-maintained, widely used for CLI tools

### Alternatives Considered
| Crate | Startup | Binary Size | Verdict |
|-------|---------|-------------|---------|
| `ureq` | ~10ms | ~500KB | **Selected** |
| `reqwest` (blocking) | ~25ms | ~2MB | Overkill |
| `hyper` | ~15ms | ~1MB | Too low-level |
| `attohttpc` | ~10ms | ~400KB | Less maintained |

### Usage Pattern
```rust
let response = ureq::get("https://api.anthropic.com/api/oauth/usage")
    .set("Authorization", &format!("Bearer {}", token))
    .set("anthropic-beta", "oauth-2025-04-20")
    .set("User-Agent", "claude-code/2.0.31")
    .timeout(std::time::Duration::from_secs(5))
    .call()?;

let usage: UsageResponse = response.into_json()?;
```

---

## 4. Cross-Compilation Setup

### Decision
Build separate binaries for each architecture, combine with `lipo`.

### Build Configuration

**.cargo/config.toml**:
```toml
[target.x86_64-apple-darwin]
linker = "clang"

[target.aarch64-apple-darwin]
linker = "clang"
```

**Build Script**:
```bash
#!/bin/bash
set -e

# Ensure targets are installed
rustup target add x86_64-apple-darwin aarch64-apple-darwin

# Build both architectures
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create universal binary
lipo -create \
    target/aarch64-apple-darwin/release/claude-status \
    target/x86_64-apple-darwin/release/claude-status \
    -output target/release/claude-status-universal

echo "Universal binary created: target/release/claude-status-universal"
```

### Binary Size Optimization

**Cargo.toml [profile.release]**:
```toml
[profile.release]
opt-level = "z"       # Optimize for size
codegen-units = 1     # Maximum optimization
lto = "fat"           # Whole-program optimization
strip = "symbols"     # Remove debug symbols
panic = "abort"       # Smaller panic handling
```

**Expected Results**:
- Single-architecture binary: ~3MB
- Universal binary: ~5.5MB (both architectures)
- Cold startup: 15-25ms
- Warm startup: 5-10ms

---

## 5. Performance Validation

### Benchmarking Strategy

**Cold Start Test**:
```bash
hyperfine --warmup 0 --runs 50 './claude-status'
```

**Target**: <50ms (p99)

**Contributing Factors**:
| Component | Expected Time | Notes |
|-----------|---------------|-------|
| Binary load | 5-10ms | Depends on binary size |
| Keychain read | 2-5ms | Single API call |
| HTTP request | 20-40ms | Network latency dominant |
| JSON parse | <1ms | Small responses |
| Output format | <1ms | String formatting |
| **Total** | 30-55ms | Within budget |

### Optimization Opportunities
1. **Parallel operations**: Read keychain while preparing request (save 2-5ms)
2. **Connection reuse**: Not applicable for single-shot CLI
3. **Lazy git status**: Only query if needed (save 10-20ms when disabled)

---

## 6. Dependencies Summary

### Cargo.toml Dependencies

```toml
[dependencies]
# Core
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP client
ureq = { version = "2.9", features = ["json"] }

# macOS Keychain
security-framework = "3.0"

# CLI
clap = { version = "4.4", features = ["derive"] }

# Time handling
chrono = { version = "0.4", features = ["serde"] }

# Error handling
thiserror = "2.0"

[dev-dependencies]
criterion = "0.5"
tempfile = "3.10"

[profile.release]
opt-level = "z"
codegen-units = 1
lto = "fat"
strip = "symbols"
panic = "abort"
```

### Dependency Audit
| Crate | Purpose | Size Impact | Justification |
|-------|---------|-------------|---------------|
| serde | JSON parsing | ~200KB | Required for API/input |
| ureq | HTTP client | ~500KB | Simplest blocking client |
| security-framework | Keychain | ~100KB | Required for credentials |
| clap | CLI args | ~300KB | Standard Rust CLI |
| chrono | Timestamps | ~200KB | ISO8601 parsing |
| thiserror | Errors | ~10KB | Idiomatic error types |

**Total estimated binary size**: 3-4MB (within 5MB target)

---

## Conclusion

All technical questions have been answered. The implementation can proceed with:

1. **Keychain**: `security-framework` for direct macOS API access
2. **HTTP**: `ureq` for minimal, fast blocking requests
3. **Build**: Universal binary via `lipo`, optimized release profile
4. **Performance**: 30-55ms expected execution time (within 50ms target)
5. **Size**: 3-4MB single arch, ~5.5MB universal (within 5MB target)

**Next Phase**: Data Model & API Contracts
