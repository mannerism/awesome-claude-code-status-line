//! Anthropic API client for usage data

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::domain::usage::{CycleInfo, UsagePercentage};
use crate::error::StatusLineError;

use super::keychain::AccessToken;

/// API endpoint for usage data
const USAGE_API_URL: &str = "https://api.anthropic.com/api/oauth/usage";

/// API beta version header
const API_BETA_VERSION: &str = "oauth-2025-04-20";

/// User agent for API requests
const USER_AGENT: &str = "claude-code/2.0.31";

/// Response from /api/oauth/usage endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct UsageResponse {
    pub five_hour: ApiCycleInfo,
    pub seven_day: ApiCycleInfo,
}

/// Cycle information from API
#[derive(Debug, Clone, Deserialize)]
pub struct ApiCycleInfo {
    /// Utilization percentage (0-100)
    pub utilization: f64,
    /// ISO8601 reset timestamp
    pub resets_at: String,
}

impl UsageResponse {
    /// Convert to domain types
    pub fn to_domain(&self) -> Result<(CycleInfo, CycleInfo), StatusLineError> {
        let five_hour = CycleInfo::new(
            UsagePercentage::from_float(self.five_hour.utilization),
            DateTime::parse_from_rfc3339(&self.five_hour.resets_at)?.with_timezone(&Utc),
        );

        let seven_day = CycleInfo::new(
            UsagePercentage::from_float(self.seven_day.utilization),
            DateTime::parse_from_rfc3339(&self.seven_day.resets_at)?.with_timezone(&Utc),
        );

        Ok((five_hour, seven_day))
    }
}

/// Fetch usage data from Anthropic API
pub fn fetch_usage(token: &AccessToken) -> Result<UsageResponse, StatusLineError> {
    let response = ureq::get(USAGE_API_URL)
        .set("Authorization", &format!("Bearer {}", token.as_str()))
        .set("anthropic-beta", API_BETA_VERSION)
        .set("User-Agent", USER_AGENT)
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(5))
        .call()
        .map_err(|e| StatusLineError::ApiRequest(e.to_string()))?;

    let usage: UsageResponse = response
        .into_json()
        .map_err(|e| StatusLineError::ApiResponse(e.to_string()))?;

    Ok(usage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_response_deserialize() {
        let json = r#"{
            "five_hour": {
                "utilization": 35.5,
                "resets_at": "2026-01-29T15:30:00Z"
            },
            "seven_day": {
                "utilization": 68.2,
                "resets_at": "2026-02-03T00:00:00Z"
            }
        }"#;

        let response: UsageResponse = serde_json::from_str(json).unwrap();
        assert!((response.five_hour.utilization - 35.5).abs() < 0.01);
        assert!((response.seven_day.utilization - 68.2).abs() < 0.01);
    }

    #[test]
    fn test_usage_response_to_domain() {
        let json = r#"{
            "five_hour": {
                "utilization": 35.5,
                "resets_at": "2026-01-29T15:30:00Z"
            },
            "seven_day": {
                "utilization": 68.2,
                "resets_at": "2026-02-03T00:00:00Z"
            }
        }"#;

        let response: UsageResponse = serde_json::from_str(json).unwrap();
        let (five_hour, seven_day) = response.to_domain().unwrap();

        assert_eq!(five_hour.utilization.value(), 36); // rounded
        assert_eq!(seven_day.utilization.value(), 68); // rounded
    }
}
