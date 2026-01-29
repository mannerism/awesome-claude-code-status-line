# Data Model: Claude Code Status Line - Rust Rewrite

**Date**: 2026-01-29
**Branch**: `001-rust-rewrite`

## Overview

This document defines the domain types following the "Correctness Through Types" principle. Invalid states should be unrepresentable at compile time.

---

## 1. Input Types

### ClaudeInput
JSON input received from Claude Code via stdin.

```rust
/// Input JSON from Claude Code status line invocation
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeInput {
    /// Working directory path (optional, falls back to cwd)
    #[serde(default)]
    pub cwd: Option<PathBuf>,

    /// Current model information
    #[serde(default)]
    pub model: Option<ModelInfo>,

    /// Context window usage (optional)
    #[serde(default)]
    pub context_window: Option<ContextWindow>,

    /// Path to current session transcript file
    #[serde(default)]
    pub transcript_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModelInfo {
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContextWindow {
    pub current_usage: Option<ContextUsage>,
    pub context_window_size: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContextUsage {
    pub input_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}
```

**Validation Rules**:
- All fields are optional with sensible defaults
- `cwd` defaults to `std::env::current_dir()`
- `model` defaults to `Model::Unknown("Unknown")`
- Missing `transcript_path` means session size is not displayed

---

## 2. Model Types

### Model
Represents the Claude model in use.

```rust
/// Claude model identifier
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Model {
    Sonnet4,
    Opus4,
    Haiku,
    Unknown(String),
}

impl Model {
    /// Parse from display name string
    pub fn from_display_name(name: &str) -> Self {
        let lower = name.to_lowercase();
        if lower.contains("opus") {
            Model::Opus4
        } else if lower.contains("sonnet") {
            Model::Sonnet4
        } else if lower.contains("haiku") {
            Model::Haiku
        } else {
            Model::Unknown(name.to_string())
        }
    }

    /// Short display name for status line
    pub fn short_name(&self) -> &str {
        match self {
            Model::Sonnet4 => "S4",
            Model::Opus4 => "O4",
            Model::Haiku => "H",
            Model::Unknown(s) => s,
        }
    }
}
```

**Validation Rules**:
- Case-insensitive parsing
- Unknown models preserved for forward compatibility

---

## 3. Usage Types

### UsagePercentage
Type-safe percentage value (0-100).

```rust
/// Usage percentage, guaranteed 0-100
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UsagePercentage(u8);

impl UsagePercentage {
    /// Create from raw value, clamping to 0-100
    pub fn new(value: u8) -> Self {
        Self(value.min(100))
    }

    /// Create from float, rounding and clamping
    pub fn from_float(value: f64) -> Self {
        Self::new(value.round().clamp(0.0, 100.0) as u8)
    }

    /// Get raw percentage value
    pub fn value(&self) -> u8 {
        self.0
    }

    /// Get threshold level for color coding
    pub fn threshold(&self) -> UsageThreshold {
        match self.0 {
            0..=49 => UsageThreshold::Normal,
            50..=74 => UsageThreshold::Warning,
            75..=100 => UsageThreshold::Critical,
            _ => unreachable!(), // Clamped to 0-100
        }
    }
}
```

### UsageThreshold
Color coding threshold states.

```rust
/// Usage threshold levels for color coding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsageThreshold {
    /// < 50%: Green
    Normal,
    /// 50-74%: Yellow
    Warning,
    /// >= 75%: Red
    Critical,
}

impl UsageThreshold {
    /// RGB color for this threshold
    pub fn color(&self) -> RgbColor {
        match self {
            Self::Normal => RgbColor::GREEN,
            Self::Warning => RgbColor::YELLOW,
            Self::Critical => RgbColor::RED,
        }
    }
}
```

### CycleInfo
Information about a usage cycle (5-hour or weekly).

```rust
/// Usage cycle information from API
#[derive(Debug, Clone)]
pub struct CycleInfo {
    /// Current utilization percentage
    pub utilization: UsagePercentage,
    /// When this cycle resets
    pub resets_at: DateTime<Utc>,
}

impl CycleInfo {
    /// Format reset time for display (MM/DD HH:MM in local timezone)
    pub fn format_reset_local(&self) -> String {
        let local = self.resets_at.with_timezone(&Local);
        local.format("%m/%d %H:%M").to_string()
    }

    /// Time remaining until reset
    pub fn time_until_reset(&self) -> Duration {
        let now = Utc::now();
        if self.resets_at > now {
            self.resets_at - now
        } else {
            Duration::zero()
        }
    }

    /// Format time remaining as "Xh Ym"
    pub fn format_time_remaining(&self) -> String {
        let remaining = self.time_until_reset();
        let hours = remaining.num_hours();
        let minutes = remaining.num_minutes() % 60;
        format!("{}h{}m", hours, minutes)
    }
}
```

---

## 4. Session Size Types

### SessionSize
Type-safe file size in bytes.

```rust
/// Session file size in bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionSize(u64);

impl SessionSize {
    /// Size thresholds in bytes
    const WARNING_THRESHOLD: u64 = 5 * 1024 * 1024;  // 5 MB
    const CRITICAL_THRESHOLD: u64 = 15 * 1024 * 1024; // 15 MB

    /// Create from byte count
    pub fn new(bytes: u64) -> Self {
        Self(bytes)
    }

    /// Create from file metadata
    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let metadata = std::fs::metadata(path)?;
        Ok(Self(metadata.len()))
    }

    /// Get raw byte count
    pub fn bytes(&self) -> u64 {
        self.0
    }

    /// Get threshold level for color coding
    pub fn threshold(&self) -> SizeThreshold {
        match self.0 {
            0..=Self::WARNING_THRESHOLD => SizeThreshold::Normal,
            Self::WARNING_THRESHOLD..=Self::CRITICAL_THRESHOLD => SizeThreshold::Warning,
            _ => SizeThreshold::Critical,
        }
    }

    /// Format for display (KB or MB)
    pub fn format_display(&self) -> String {
        const KB: u64 = 1024;
        const MB: u64 = 1024 * 1024;

        if self.0 >= MB {
            format!("{:.1}MB", self.0 as f64 / MB as f64)
        } else {
            format!("{}KB", self.0 / KB)
        }
    }
}
```

### SizeThreshold
Session size threshold states.

```rust
/// Session size threshold levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeThreshold {
    /// < 5 MB: Normal (green)
    Normal,
    /// 5-15 MB: Warning (yellow)
    Warning,
    /// > 15 MB: Critical (red)
    Critical,
}

impl SizeThreshold {
    /// RGB color for this threshold
    pub fn color(&self) -> RgbColor {
        match self {
            Self::Normal => RgbColor::GREEN,
            Self::Warning => RgbColor::YELLOW,
            Self::Critical => RgbColor::RED,
        }
    }

    /// Warning indicator emoji
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Warning => "\u{26A0}\u{FE0F}",  // ⚠️
            Self::Critical => "\u{1F534}",         // 🔴
        }
    }
}
```

---

## 5. Git Types

### GitStatus
Git repository status (invalid states unrepresentable).

```rust
/// Git repository status
#[derive(Debug, Clone)]
pub enum GitStatus {
    /// Not inside a git repository
    NotRepo,
    /// Inside a repo with status information
    Repo(GitRepoStatus),
}

/// Status of a git repository
#[derive(Debug, Clone)]
pub struct GitRepoStatus {
    /// Current branch name or commit hash
    pub branch: BranchInfo,
    /// Working tree has modifications
    pub modified: bool,
    /// Untracked files exist
    pub untracked: bool,
    /// Commits ahead of upstream
    pub ahead: u32,
    /// Commits behind upstream
    pub behind: u32,
}

/// Branch identification
#[derive(Debug, Clone)]
pub enum BranchInfo {
    /// On a named branch
    Branch(String),
    /// Detached HEAD at commit
    Detached(String),  // Short commit hash
}

impl GitRepoStatus {
    /// Format status indicators for display
    pub fn format_indicators(&self) -> String {
        let mut indicators = String::new();
        if self.modified {
            indicators.push('*');
        }
        if self.untracked {
            indicators.push('?');
        }
        if self.ahead > 0 {
            indicators.push_str(&format!("↑{}", self.ahead));
        }
        if self.behind > 0 {
            indicators.push_str(&format!("↓{}", self.behind));
        }
        indicators
    }

    /// Format branch name for display
    pub fn format_branch(&self) -> String {
        match &self.branch {
            BranchInfo::Branch(name) => name.clone(),
            BranchInfo::Detached(hash) => format!("({})", hash),
        }
    }
}
```

---

## 6. Display Types

### RgbColor
Type-safe RGB color.

```rust
/// RGB color for terminal output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    /// Standard colors
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };
    pub const YELLOW: Self = Self { r: 255, g: 255, b: 0 };
    pub const RED: Self = Self { r: 255, g: 100, b: 100 };

    /// Create new color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Format as ANSI escape sequence
    pub fn to_ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// Reset ANSI color
    pub const RESET: &'static str = "\x1b[0m";
}
```

---

## 7. API Response Types

### UsageResponse
Response from Anthropic usage API.

```rust
/// Response from /api/oauth/usage endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct UsageResponse {
    pub five_hour: ApiCycleInfo,
    pub seven_day: ApiCycleInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiCycleInfo {
    /// Utilization percentage (0-100)
    pub utilization: f64,
    /// ISO8601 reset timestamp
    pub resets_at: String,
}

impl UsageResponse {
    /// Convert to domain types
    pub fn to_domain(&self) -> Result<(CycleInfo, CycleInfo), ParseError> {
        Ok((
            CycleInfo {
                utilization: UsagePercentage::from_float(self.five_hour.utilization),
                resets_at: DateTime::parse_from_rfc3339(&self.five_hour.resets_at)?
                    .with_timezone(&Utc),
            },
            CycleInfo {
                utilization: UsagePercentage::from_float(self.seven_day.utilization),
                resets_at: DateTime::parse_from_rfc3339(&self.seven_day.resets_at)?
                    .with_timezone(&Utc),
            },
        ))
    }
}
```

---

## 8. Credential Types

### AccessToken
Secure access token (zeroized on drop).

```rust
use zeroize::Zeroize;

/// OAuth access token (cleared from memory on drop)
#[derive(Clone, Zeroize)]
#[zeroize(drop)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(token: String) -> Self {
        Self(token)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Prevent accidental logging
impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AccessToken([REDACTED])")
    }
}
```

---

## Entity Relationships

```
ClaudeInput
    ├── cwd: PathBuf (project name extraction)
    ├── model: ModelInfo → Model
    ├── context_window: ContextWindow (optional display)
    └── transcript_path: PathBuf → SessionSize → SizeThreshold

AccessToken (from Keychain)
    └── API Request → UsageResponse
                          ├── five_hour → CycleInfo → UsagePercentage → UsageThreshold
                          └── seven_day → CycleInfo → UsagePercentage → UsageThreshold

GitStatus
    └── NotRepo | Repo(GitRepoStatus)
                      ├── branch: BranchInfo
                      ├── modified: bool
                      ├── untracked: bool
                      ├── ahead: u32
                      └── behind: u32

StatusLine (output)
    ├── project_name: String
    ├── git_status: Option<GitRepoStatus>
    ├── model: Model
    ├── five_hour: CycleInfo (colored by threshold)
    ├── seven_day: CycleInfo (colored by threshold)
    └── session_size: Option<SessionSize> (colored by threshold)
```

---

## Validation Summary

| Type | Validation | Invalid State Prevention |
|------|------------|--------------------------|
| UsagePercentage | Clamped 0-100 | Cannot exceed 100% |
| SessionSize | From filesystem | Cannot be negative |
| SizeThreshold | Derived from SessionSize | States map to exact ranges |
| GitStatus | Enum variants | NotRepo or full Repo info |
| Model | Parse with fallback | Unknown preserved, never null |
| AccessToken | Zeroize on drop | Memory cleared after use |
| CycleInfo | Parsed from API | Invalid timestamps fail parse |
