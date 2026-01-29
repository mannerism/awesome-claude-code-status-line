//! RGB color types for terminal output

/// RGB color for terminal output using ANSI 24-bit color codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    /// Green color for normal usage (< 50%)
    pub const GREEN: Self = Self { r: 0, g: 255, b: 0 };

    /// Yellow color for warning usage (50-74%)
    pub const YELLOW: Self = Self {
        r: 255,
        g: 255,
        b: 0,
    };

    /// Red color for critical usage (>= 75%)
    pub const RED: Self = Self {
        r: 255,
        g: 100,
        b: 100,
    };

    /// ANSI reset sequence
    pub const RESET: &'static str = "\x1b[0m";

    /// Create a new RGB color
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Format as ANSI escape sequence for foreground color
    pub fn to_ansi(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// Wrap text with this color and reset
    pub fn colorize(&self, text: &str) -> String {
        format!("{}{}{}", self.to_ansi(), text, Self::RESET)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_color() {
        let color = RgbColor::new(128, 64, 32);
        assert_eq!(color.r, 128);
        assert_eq!(color.g, 64);
        assert_eq!(color.b, 32);
    }

    #[test]
    fn test_color_constants() {
        assert_eq!(RgbColor::GREEN, RgbColor::new(0, 255, 0));
        assert_eq!(RgbColor::YELLOW, RgbColor::new(255, 255, 0));
        assert_eq!(RgbColor::RED, RgbColor::new(255, 100, 100));
    }

    #[test]
    fn test_to_ansi() {
        let color = RgbColor::new(255, 128, 64);
        assert_eq!(color.to_ansi(), "\x1b[38;2;255;128;64m");
    }

    #[test]
    fn test_to_ansi_green() {
        assert_eq!(RgbColor::GREEN.to_ansi(), "\x1b[38;2;0;255;0m");
    }

    #[test]
    fn test_colorize() {
        let color = RgbColor::GREEN;
        let result = color.colorize("50%");
        assert!(result.starts_with("\x1b[38;2;0;255;0m"));
        assert!(result.contains("50%"));
        assert!(result.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_reset_constant() {
        assert_eq!(RgbColor::RESET, "\x1b[0m");
    }
}
