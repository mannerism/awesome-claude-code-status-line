//! Session size types for transcript file monitoring

use std::path::Path;

use crate::display::colors::RgbColor;

/// Session file size in bytes
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionSize(u64);

impl SessionSize {
    /// Warning threshold: 5 MB
    const WARNING_THRESHOLD: u64 = 5 * 1024 * 1024;

    /// Critical threshold: 15 MB
    const CRITICAL_THRESHOLD: u64 = 15 * 1024 * 1024;

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
        if self.0 < Self::WARNING_THRESHOLD {
            SizeThreshold::Normal
        } else if self.0 < Self::CRITICAL_THRESHOLD {
            SizeThreshold::Warning
        } else {
            SizeThreshold::Critical
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
    fn test_session_size_new() {
        let size = SessionSize::new(1024);
        assert_eq!(size.bytes(), 1024);
    }

    #[test]
    fn test_session_size_threshold_normal() {
        // Under 5MB
        let size = SessionSize::new(1 * 1024 * 1024); // 1MB
        assert_eq!(size.threshold(), SizeThreshold::Normal);

        let size = SessionSize::new(4 * 1024 * 1024); // 4MB
        assert_eq!(size.threshold(), SizeThreshold::Normal);
    }

    #[test]
    fn test_session_size_threshold_warning() {
        // 5MB to 15MB
        let size = SessionSize::new(5 * 1024 * 1024); // 5MB exactly
        assert_eq!(size.threshold(), SizeThreshold::Warning);

        let size = SessionSize::new(10 * 1024 * 1024); // 10MB
        assert_eq!(size.threshold(), SizeThreshold::Warning);

        let size = SessionSize::new(14 * 1024 * 1024); // 14MB
        assert_eq!(size.threshold(), SizeThreshold::Warning);
    }

    #[test]
    fn test_session_size_threshold_critical() {
        // Over 15MB
        let size = SessionSize::new(15 * 1024 * 1024); // 15MB exactly
        assert_eq!(size.threshold(), SizeThreshold::Critical);

        let size = SessionSize::new(20 * 1024 * 1024); // 20MB
        assert_eq!(size.threshold(), SizeThreshold::Critical);
    }

    #[test]
    fn test_session_size_format_display_kb() {
        let size = SessionSize::new(512 * 1024); // 512KB
        assert_eq!(size.format_display(), "512KB");
    }

    #[test]
    fn test_session_size_format_display_mb() {
        let size = SessionSize::new(2 * 1024 * 1024 + 512 * 1024); // 2.5MB
        assert_eq!(size.format_display(), "2.5MB");
    }

    #[test]
    fn test_size_threshold_color() {
        assert_eq!(SizeThreshold::Normal.color(), RgbColor::GREEN);
        assert_eq!(SizeThreshold::Warning.color(), RgbColor::YELLOW);
        assert_eq!(SizeThreshold::Critical.color(), RgbColor::RED);
    }

    #[test]
    fn test_size_threshold_indicator() {
        assert_eq!(SizeThreshold::Normal.indicator(), "");
        assert_eq!(SizeThreshold::Warning.indicator(), "\u{26A0}\u{FE0F}");
        assert_eq!(SizeThreshold::Critical.indicator(), "\u{1F534}");
    }

    #[test]
    fn test_session_size_from_file() {
        use std::io::Write;
        let mut temp = tempfile::NamedTempFile::new().unwrap();
        let data = vec![0u8; 1024]; // 1KB of data
        temp.write_all(&data).unwrap();
        temp.flush().unwrap();

        let size = SessionSize::from_file(temp.path()).unwrap();
        assert_eq!(size.bytes(), 1024);
    }

    #[test]
    fn test_session_size_from_file_not_found() {
        let result = SessionSize::from_file(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }
}
