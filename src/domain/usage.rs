//! Usage tracking types for Claude Code status line

use chrono::{DateTime, Duration, Local, Utc};

use crate::display::colors::RgbColor;

/// Usage percentage, guaranteed to be in range 0-100
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
    /// Get the RGB color for this threshold
    pub fn color(&self) -> RgbColor {
        match self {
            Self::Normal => RgbColor::GREEN,
            Self::Warning => RgbColor::YELLOW,
            Self::Critical => RgbColor::RED,
        }
    }
}

/// Usage cycle information from API
#[derive(Debug, Clone)]
pub struct CycleInfo {
    /// Current utilization percentage
    pub utilization: UsagePercentage,
    /// When this cycle resets
    pub resets_at: DateTime<Utc>,
}

impl CycleInfo {
    /// Create a new cycle info
    pub fn new(utilization: UsagePercentage, resets_at: DateTime<Utc>) -> Self {
        Self {
            utilization,
            resets_at,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_usage_percentage_new() {
        assert_eq!(UsagePercentage::new(50).value(), 50);
        assert_eq!(UsagePercentage::new(0).value(), 0);
        assert_eq!(UsagePercentage::new(100).value(), 100);
    }

    #[test]
    fn test_usage_percentage_clamps() {
        assert_eq!(UsagePercentage::new(150).value(), 100);
        assert_eq!(UsagePercentage::new(255).value(), 100);
    }

    #[test]
    fn test_usage_percentage_from_float() {
        assert_eq!(UsagePercentage::from_float(35.5).value(), 36);
        assert_eq!(UsagePercentage::from_float(35.4).value(), 35);
        assert_eq!(UsagePercentage::from_float(-10.0).value(), 0);
        assert_eq!(UsagePercentage::from_float(150.0).value(), 100);
    }

    #[test]
    fn test_usage_percentage_threshold_normal() {
        assert_eq!(UsagePercentage::new(0).threshold(), UsageThreshold::Normal);
        assert_eq!(UsagePercentage::new(25).threshold(), UsageThreshold::Normal);
        assert_eq!(UsagePercentage::new(49).threshold(), UsageThreshold::Normal);
    }

    #[test]
    fn test_usage_percentage_threshold_warning() {
        assert_eq!(
            UsagePercentage::new(50).threshold(),
            UsageThreshold::Warning
        );
        assert_eq!(
            UsagePercentage::new(60).threshold(),
            UsageThreshold::Warning
        );
        assert_eq!(
            UsagePercentage::new(74).threshold(),
            UsageThreshold::Warning
        );
    }

    #[test]
    fn test_usage_percentage_threshold_critical() {
        assert_eq!(
            UsagePercentage::new(75).threshold(),
            UsageThreshold::Critical
        );
        assert_eq!(
            UsagePercentage::new(90).threshold(),
            UsageThreshold::Critical
        );
        assert_eq!(
            UsagePercentage::new(100).threshold(),
            UsageThreshold::Critical
        );
    }

    #[test]
    fn test_usage_threshold_color() {
        assert_eq!(UsageThreshold::Normal.color(), RgbColor::GREEN);
        assert_eq!(UsageThreshold::Warning.color(), RgbColor::YELLOW);
        assert_eq!(UsageThreshold::Critical.color(), RgbColor::RED);
    }

    #[test]
    fn test_cycle_info_format_time_remaining() {
        let now = Utc::now();
        let resets_at = now + Duration::hours(2) + Duration::minutes(30);
        let cycle = CycleInfo::new(UsagePercentage::new(50), resets_at);

        let formatted = cycle.format_time_remaining();
        // Should be approximately "2h30m" (may vary slightly due to test timing)
        assert!(formatted.contains('h'));
        assert!(formatted.contains('m'));
    }

    #[test]
    fn test_cycle_info_time_until_reset_past() {
        let past = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
        let cycle = CycleInfo::new(UsagePercentage::new(50), past);

        let remaining = cycle.time_until_reset();
        assert_eq!(remaining, Duration::zero());
    }
}
