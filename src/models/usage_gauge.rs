//! UsageGauge model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageGauge
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageGauge {
    /// The metric key.
    #[serde(rename = "metric")]
    pub metric: String,
    /// The current snapshot value.
    #[serde(rename = "value")]
    pub value: i64,
    /// The snapshot timestamp.
    #[serde(rename = "time")]
    pub time: String,
    /// The resource type.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// The resource ID.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
}

impl UsageGauge {
    /// Get metric
    pub fn metric(&self) -> &String {
        &self.metric
    }

    /// Get value
    pub fn value(&self) -> &i64 {
        &self.value
    }

    /// Get time
    pub fn time(&self) -> &String {
        &self.time
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_gauge_creation() {
        let _model = <UsageGauge as Default>::default();
        let _ = _model.metric();
        let _ = _model.value();
        let _ = _model.time();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
    }

    #[test]
    fn test_usage_gauge_serialization() {
        let model = <UsageGauge as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageGauge, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
