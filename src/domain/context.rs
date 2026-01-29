//! Context window usage types

use crate::display::colors::RgbColor;

/// Context window usage information
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContextUsageInfo {
    /// Current tokens used
    used_tokens: u64,
    /// Total context window size
    total_tokens: u64,
}

impl ContextUsageInfo {
    /// Warning threshold: 70% of context window
    const WARNING_PCT: u64 = 70;

    /// Critical threshold: 90% of context window (near auto-compact)
    const CRITICAL_PCT: u64 = 90;

    /// Create from used and total tokens
    pub fn new(used_tokens: u64, total_tokens: u64) -> Self {
        Self {
            used_tokens,
            total_tokens,
        }
    }

    /// Get usage percentage (0-100)
    pub fn percentage(&self) -> u64 {
        if self.total_tokens == 0 {
            return 0;
        }
        (self.used_tokens * 100) / self.total_tokens
    }

    /// Get remaining tokens
    pub fn remaining(&self) -> u64 {
        self.total_tokens.saturating_sub(self.used_tokens)
    }

    /// Get threshold level for color coding
    pub fn threshold(&self) -> ContextThreshold {
        let pct = self.percentage();
        if pct < Self::WARNING_PCT {
            ContextThreshold::Normal
        } else if pct < Self::CRITICAL_PCT {
            ContextThreshold::Warning
        } else {
            ContextThreshold::Critical
        }
    }

    /// Format for display (shows percentage used)
    pub fn format_display(&self) -> String {
        format!("{}%", self.percentage())
    }

    /// Format percentage
    pub fn format_percentage(&self) -> String {
        format!("{}%", self.percentage())
    }
}

/// Context usage threshold levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextThreshold {
    /// < 70%: Normal (green)
    Normal,
    /// 70-90%: Warning (yellow)
    Warning,
    /// > 90%: Critical (red) - near auto-compact
    Critical,
}

impl ContextThreshold {
    /// Get the RGB color for this threshold
    pub fn color(&self) -> RgbColor {
        match self {
            Self::Normal => RgbColor::GREEN,
            Self::Warning => RgbColor::YELLOW,
            Self::Critical => RgbColor::RED,
        }
    }

    /// Get warning indicator emoji
    pub fn indicator(&self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Warning => "\u{26A0}\u{FE0F}", // ⚠️
            Self::Critical => "\u{1F534}",       // 🔴
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_usage_percentage() {
        let usage = ContextUsageInfo::new(50_000, 200_000);
        assert_eq!(usage.percentage(), 25);
    }

    #[test]
    fn test_context_usage_remaining() {
        let usage = ContextUsageInfo::new(50_000, 200_000);
        assert_eq!(usage.remaining(), 150_000);
    }

    #[test]
    fn test_context_threshold_normal() {
        let usage = ContextUsageInfo::new(100_000, 200_000); // 50%
        assert_eq!(usage.threshold(), ContextThreshold::Normal);
    }

    #[test]
    fn test_context_threshold_warning() {
        let usage = ContextUsageInfo::new(150_000, 200_000); // 75%
        assert_eq!(usage.threshold(), ContextThreshold::Warning);
    }

    #[test]
    fn test_context_threshold_critical() {
        let usage = ContextUsageInfo::new(185_000, 200_000); // 92.5%
        assert_eq!(usage.threshold(), ContextThreshold::Critical);
    }

    #[test]
    fn test_context_format_display() {
        let usage = ContextUsageInfo::new(50_000, 200_000);
        assert_eq!(usage.format_display(), "25%");
    }

    #[test]
    fn test_context_zero_total() {
        let usage = ContextUsageInfo::new(0, 0);
        assert_eq!(usage.percentage(), 0);
        assert_eq!(usage.remaining(), 0);
    }
}
