//! UsageEvent model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// usageEvent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageEvent {
    /// The metric key.
    #[serde(rename = "metric")]
    pub metric: String,
    /// The metric value.
    #[serde(rename = "value")]
    pub value: i64,
    /// The event timestamp.
    #[serde(rename = "time")]
    pub time: String,
    /// The API endpoint path.
    #[serde(rename = "path")]
    pub path: String,
    /// The HTTP method.
    #[serde(rename = "method")]
    pub method: String,
    /// HTTP status code. Stored as string to preserve unset state (empty string =
    /// not available).
    #[serde(rename = "status")]
    pub status: String,
    /// The resource type.
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    /// The resource ID.
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// Country code in [ISO 3166-1](http://en.wikipedia.org/wiki/ISO_3166-1)
    /// two-character format.
    #[serde(rename = "countryCode")]
    pub country_code: String,
    /// The user agent string.
    #[serde(rename = "userAgent")]
    pub user_agent: String,
}

impl UsageEvent {
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

    /// Get path
    pub fn path(&self) -> &String {
        &self.path
    }

    /// Get method
    pub fn method(&self) -> &String {
        &self.method
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get resource_type
    pub fn resource_type(&self) -> &String {
        &self.resource_type
    }

    /// Get resource_id
    pub fn resource_id(&self) -> &String {
        &self.resource_id
    }

    /// Get country_code
    pub fn country_code(&self) -> &String {
        &self.country_code
    }

    /// Get user_agent
    pub fn user_agent(&self) -> &String {
        &self.user_agent
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_event_creation() {
        let _model = <UsageEvent as Default>::default();
        let _ = _model.metric();
        let _ = _model.value();
        let _ = _model.time();
        let _ = _model.path();
        let _ = _model.method();
        let _ = _model.status();
        let _ = _model.resource_type();
        let _ = _model.resource_id();
        let _ = _model.country_code();
        let _ = _model.user_agent();
    }

    #[test]
    fn test_usage_event_serialization() {
        let model = <UsageEvent as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageEvent, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
