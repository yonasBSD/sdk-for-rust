//! UsageGaugeList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Usage gauges list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageGaugeList {
    /// Total number of gauges that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of gauges.
    #[serde(rename = "gauges")]
    pub gauges: Vec<crate::models::UsageGauge>,
}

impl UsageGaugeList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get gauges
    pub fn gauges(&self) -> &Vec<crate::models::UsageGauge> {
        &self.gauges
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_gauge_list_creation() {
        let _model = <UsageGaugeList as Default>::default();
        let _ = _model.total();
        let _ = _model.gauges();
    }

    #[test]
    fn test_usage_gauge_list_serialization() {
        let model = <UsageGaugeList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageGaugeList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
