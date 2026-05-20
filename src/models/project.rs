//! Project model for Appwrite SDK

use serde::{Deserialize, Serialize};

/// Project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(Default))]
pub struct Project {
    /// Project ID.
    #[serde(rename = "$id")]
    pub id: String,
    /// Project creation date in ISO 8601 format.
    #[serde(rename = "$createdAt")]
    pub created_at: String,
    /// Project update date in ISO 8601 format.
    #[serde(rename = "$updatedAt")]
    pub updated_at: String,
    /// Project name.
    #[serde(rename = "name")]
    pub name: String,
    /// Project team ID.
    #[serde(rename = "teamId")]
    pub team_id: String,
    /// Deprecated since 1.9.5: List of dev keys.
    #[serde(rename = "devKeys")]
    pub dev_keys: Vec<crate::models::DevKey>,
    /// Status for custom SMTP
    #[serde(rename = "smtpEnabled")]
    pub smtp_enabled: bool,
    /// SMTP sender name
    #[serde(rename = "smtpSenderName")]
    pub smtp_sender_name: String,
    /// SMTP sender email
    #[serde(rename = "smtpSenderEmail")]
    pub smtp_sender_email: String,
    /// SMTP reply to name
    #[serde(rename = "smtpReplyToName")]
    pub smtp_reply_to_name: String,
    /// SMTP reply to email
    #[serde(rename = "smtpReplyToEmail")]
    pub smtp_reply_to_email: String,
    /// SMTP server host name
    #[serde(rename = "smtpHost")]
    pub smtp_host: String,
    /// SMTP server port
    #[serde(rename = "smtpPort")]
    pub smtp_port: i64,
    /// SMTP server username
    #[serde(rename = "smtpUsername")]
    pub smtp_username: String,
    /// SMTP server password. This property is write-only and always returned
    /// empty.
    #[serde(rename = "smtpPassword")]
    pub smtp_password: String,
    /// SMTP server secure protocol
    #[serde(rename = "smtpSecure")]
    pub smtp_secure: String,
    /// Number of times the ping was received for this project.
    #[serde(rename = "pingCount")]
    pub ping_count: i64,
    /// Last ping datetime in ISO 8601 format.
    #[serde(rename = "pingedAt")]
    pub pinged_at: String,
    /// Labels for the project.
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    /// Project status
    #[serde(rename = "status")]
    pub status: String,
    /// List of auth methods.
    #[serde(rename = "authMethods")]
    pub auth_methods: Vec<crate::models::ProjectAuthMethod>,
    /// List of services.
    #[serde(rename = "services")]
    pub services: Vec<crate::models::ProjectService>,
    /// List of protocols.
    #[serde(rename = "protocols")]
    pub protocols: Vec<crate::models::ProjectProtocol>,
    /// Project region
    #[serde(rename = "region")]
    pub region: String,
    /// Billing limits reached
    #[serde(rename = "billingLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_limits: Option<crate::models::BillingLimits>,
    /// Project blocks information
    #[serde(rename = "blocks")]
    pub blocks: Vec<crate::models::Block>,
    /// Last time the project was accessed via console. Used with plan's
    /// projectInactivityDays to determine if project is paused.
    #[serde(rename = "consoleAccessedAt")]
    pub console_accessed_at: String,
}

impl Project {
    /// Get id
    pub fn id(&self) -> &String {
        &self.id
    }

    /// Get created_at
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get updated_at
    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    /// Get name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get team_id
    pub fn team_id(&self) -> &String {
        &self.team_id
    }

    /// Get dev_keys
    pub fn dev_keys(&self) -> &Vec<crate::models::DevKey> {
        &self.dev_keys
    }

    /// Get smtp_enabled
    pub fn smtp_enabled(&self) -> &bool {
        &self.smtp_enabled
    }

    /// Get smtp_sender_name
    pub fn smtp_sender_name(&self) -> &String {
        &self.smtp_sender_name
    }

    /// Get smtp_sender_email
    pub fn smtp_sender_email(&self) -> &String {
        &self.smtp_sender_email
    }

    /// Get smtp_reply_to_name
    pub fn smtp_reply_to_name(&self) -> &String {
        &self.smtp_reply_to_name
    }

    /// Get smtp_reply_to_email
    pub fn smtp_reply_to_email(&self) -> &String {
        &self.smtp_reply_to_email
    }

    /// Get smtp_host
    pub fn smtp_host(&self) -> &String {
        &self.smtp_host
    }

    /// Get smtp_port
    pub fn smtp_port(&self) -> &i64 {
        &self.smtp_port
    }

    /// Get smtp_username
    pub fn smtp_username(&self) -> &String {
        &self.smtp_username
    }

    /// Get smtp_password
    pub fn smtp_password(&self) -> &String {
        &self.smtp_password
    }

    /// Get smtp_secure
    pub fn smtp_secure(&self) -> &String {
        &self.smtp_secure
    }

    /// Get ping_count
    pub fn ping_count(&self) -> &i64 {
        &self.ping_count
    }

    /// Get pinged_at
    pub fn pinged_at(&self) -> &String {
        &self.pinged_at
    }

    /// Get labels
    pub fn labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Get status
    pub fn status(&self) -> &String {
        &self.status
    }

    /// Get auth_methods
    pub fn auth_methods(&self) -> &Vec<crate::models::ProjectAuthMethod> {
        &self.auth_methods
    }

    /// Get services
    pub fn services(&self) -> &Vec<crate::models::ProjectService> {
        &self.services
    }

    /// Get protocols
    pub fn protocols(&self) -> &Vec<crate::models::ProjectProtocol> {
        &self.protocols
    }

    /// Get region
    pub fn region(&self) -> &String {
        &self.region
    }

    /// Set billing_limits
    pub fn set_billing_limits(mut self, billing_limits: crate::models::BillingLimits) -> Self {
        self.billing_limits = Some(billing_limits);
        self
    }

    /// Get billing_limits
    pub fn billing_limits(&self) -> Option<&crate::models::BillingLimits> {
        self.billing_limits.as_ref()
    }

    /// Get blocks
    pub fn blocks(&self) -> &Vec<crate::models::Block> {
        &self.blocks
    }

    /// Get console_accessed_at
    pub fn console_accessed_at(&self) -> &String {
        &self.console_accessed_at
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let _model = <Project as Default>::default();
        let _ = _model.id();
        let _ = _model.created_at();
        let _ = _model.updated_at();
        let _ = _model.name();
        let _ = _model.team_id();
        let _ = _model.dev_keys();
        let _ = _model.smtp_enabled();
        let _ = _model.smtp_sender_name();
        let _ = _model.smtp_sender_email();
        let _ = _model.smtp_reply_to_name();
        let _ = _model.smtp_reply_to_email();
        let _ = _model.smtp_host();
        let _ = _model.smtp_port();
        let _ = _model.smtp_username();
        let _ = _model.smtp_password();
        let _ = _model.smtp_secure();
        let _ = _model.ping_count();
        let _ = _model.pinged_at();
        let _ = _model.labels();
        let _ = _model.status();
        let _ = _model.auth_methods();
        let _ = _model.services();
        let _ = _model.protocols();
        let _ = _model.region();
        let _ = _model.blocks();
        let _ = _model.console_accessed_at();
    }

    #[test]
    fn test_project_serialization() {
        let model = <Project as Default>::default();
        let json = serde_json::to_string(&model);
        assert!(json.is_ok());

        let deserialized: Result<Project, _> = serde_json::from_str(&json.unwrap());
        assert!(deserialized.is_ok());
    }
}
