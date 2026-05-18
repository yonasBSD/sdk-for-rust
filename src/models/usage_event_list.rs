//! UsageEventList model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Usage events list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct UsageEventList {
    /// Total number of events that matched your query.
    #[serde(rename = "total")]
    pub total: i64,
    /// List of events.
    #[serde(rename = "events")]
    pub events: Vec<crate::models::UsageEvent>,
}

impl UsageEventList {
    /// Get total
    pub fn total(&self) -> &i64 {
        &self.total
    }

    /// Get events
    pub fn events(&self) -> &Vec<crate::models::UsageEvent> {
        &self.events
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_event_list_creation() {
        let _model = <UsageEventList as Default>::default();
        let _ = _model.total();
        let _ = _model.events();
    }

    #[test]
    fn test_usage_event_list_serialization() {
        let model = <UsageEventList as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<UsageEventList, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
