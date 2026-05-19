//! Usage service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Usage {
    client: Client,
}

impl Usage {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Query usage event metrics from the usage database. Returns individual event
    /// rows with full metadata. Pass Query objects as JSON strings to filter,
    /// paginate, and order results. Supported query methods: equal,
    /// greaterThanEqual, lessThanEqual, orderAsc, orderDesc, limit, offset.
    /// Supported filter attributes: metric, path, method, status, resource,
    /// resourceId, country, userAgent, time (these match the underlying column
    /// names — note that the response surfaces `resource` as `resourceType` and
    /// `country` as `countryCode`). When no time filter is supplied the endpoint
    /// defaults to the last 7 days. Default `limit(100)` is applied if none is
    /// given; user-supplied limits are capped at 500. The `total` field is capped
    /// at 5000 to keep counts predictable — pass `total=false` to skip the count
    /// entirely.
    pub async fn list_events(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::UsageEventList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/usage/events".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Query usage gauge metrics (point-in-time resource snapshots) from the usage
    /// database. Returns individual gauge snapshots with metric, value, and
    /// timestamp. Pass Query objects as JSON strings to filter, paginate, and
    /// order results. Supported query methods: equal, greaterThanEqual,
    /// lessThanEqual, orderAsc, orderDesc, limit, offset. Supported filter
    /// attributes: metric, time. Use `orderDesc("time"), limit(1)` to fetch the
    /// most recent snapshot. When no time filter is supplied the endpoint defaults
    /// to the last 7 days. Default `limit(100)` is applied if none is given;
    /// user-supplied limits are capped at 500. The `total` field is capped at 5000
    /// to keep counts predictable — pass `total=false` to skip the count
    /// entirely.
    pub async fn list_gauges(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::UsageGaugeList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/usage/gauges".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

}

impl crate::services::Service for Usage {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_creation() {
        let client = Client::new();
        let service = Usage::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
