//! Project service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Project service allows you to manage all the projects in your Appwrite
/// server.
#[derive(Debug, Clone)]
pub struct Project {
    client: Client,
}

impl Project {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all API keys from the current project.
    pub async fn list_keys(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::KeyList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/keys".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new API key. It's recommended to have multiple API keys with
    /// strict scopes for separate functions within your project.
    pub async fn create_key(
        &self,
        key_id: impl Into<String>,
        name: impl Into<String>,
        scopes: Vec<crate::enums::Scopes>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::Key> {
        let mut params = HashMap::new();
        params.insert("keyId".to_string(), json!(key_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("scopes".to_string(), json!(scopes));
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a key by its unique ID.
    pub async fn get_key(
        &self,
        key_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Key> {
        let params = HashMap::new();

        let path = "/project/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update a key by its unique ID. Use this endpoint to update the name,
    /// scopes, or expiration time of an API key.
    pub async fn update_key(
        &self,
        key_id: impl Into<String>,
        name: impl Into<String>,
        scopes: Vec<crate::enums::Scopes>,
        expire: Option<&str>,
    ) -> crate::error::Result<crate::models::Key> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("scopes".to_string(), json!(scopes));
        if let Some(value) = expire {
            params.insert("expire".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a key by its unique ID. Once deleted, the key can no longer be used
    /// to authenticate API calls.
    pub async fn delete_key(
        &self,
        key_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/keys/{keyId}".to_string().replace("{keyId}", &key_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Update the project labels. Labels can be used to easily filter projects in
    /// an organization.
    pub async fn update_labels(
        &self,
        labels: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("labels".to_string(), json!(labels.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/labels".to_string();

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Get a list of all platforms in the project. This endpoint returns an array
    /// of all platforms and their configurations.
    pub async fn list_platforms(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::PlatformList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/platforms".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new Android platform for your project. Use this endpoint to
    /// register a new Android platform where your users will run your application
    /// which will interact with the Appwrite API.
    pub async fn create_android_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        application_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformAndroid> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("applicationId".to_string(), json!(application_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/android".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an Android platform by its unique ID. Use this endpoint to update
    /// the platform's name or application ID.
    pub async fn update_android_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        application_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformAndroid> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("applicationId".to_string(), json!(application_id.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/android/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Apple platform for your project. Use this endpoint to register
    /// a new Apple platform where your users will run your application which will
    /// interact with the Appwrite API.
    pub async fn create_apple_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        bundle_identifier: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformApple> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("bundleIdentifier".to_string(), json!(bundle_identifier.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/apple".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update an Apple platform by its unique ID. Use this endpoint to update the
    /// platform's name or bundle identifier.
    pub async fn update_apple_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        bundle_identifier: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformApple> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("bundleIdentifier".to_string(), json!(bundle_identifier.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/apple/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Linux platform for your project. Use this endpoint to register
    /// a new Linux platform where your users will run your application which will
    /// interact with the Appwrite API.
    pub async fn create_linux_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformLinux> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageName".to_string(), json!(package_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/linux".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Linux platform by its unique ID. Use this endpoint to update the
    /// platform's name or package name.
    pub async fn update_linux_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformLinux> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageName".to_string(), json!(package_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/linux/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new web platform for your project. Use this endpoint to register a
    /// new platform where your users will run your application which will interact
    /// with the Appwrite API.
    pub async fn create_web_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        hostname: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWeb> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("hostname".to_string(), json!(hostname.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/web".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a web platform by its unique ID. Use this endpoint to update the
    /// platform's name or hostname.
    pub async fn update_web_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        hostname: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWeb> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("hostname".to_string(), json!(hostname.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/web/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Create a new Windows platform for your project. Use this endpoint to
    /// register a new Windows platform where your users will run your application
    /// which will interact with the Appwrite API.
    pub async fn create_windows_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_identifier_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWindows> {
        let mut params = HashMap::new();
        params.insert("platformId".to_string(), json!(platform_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageIdentifierName".to_string(), json!(package_identifier_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/windows".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a Windows platform by its unique ID. Use this endpoint to update the
    /// platform's name or package identifier name.
    pub async fn update_windows_platform(
        &self,
        platform_id: impl Into<String>,
        name: impl Into<String>,
        package_identifier_name: impl Into<String>,
    ) -> crate::error::Result<crate::models::PlatformWindows> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        params.insert("packageIdentifierName".to_string(), json!(package_identifier_name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/windows/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Get a platform by its unique ID. This endpoint returns the platform's
    /// details, including its name, type, and key configurations.
    pub async fn get_platform(
        &self,
        platform_id: impl Into<String>,
    ) -> crate::error::Result<serde_json::Value> {
        let params = HashMap::new();

        let path = "/project/platforms/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Delete a platform by its unique ID. This endpoint removes the platform and
    /// all its configurations from the project.
    pub async fn delete_platform(
        &self,
        platform_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/platforms/{platformId}".to_string().replace("{platformId}", &platform_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if team members can see other
    /// members information. When enabled, all team members can see ID, name,
    /// email, phone number, and MFA status of other members..
    pub async fn update_membership_privacy_policy(
        &self,
        user_id: Option<bool>,
        user_email: Option<bool>,
        user_phone: Option<bool>,
        user_name: Option<bool>,
        user_mfa: Option<bool>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = user_id {
            params.insert("userId".to_string(), json!(value));
        }
        if let Some(value) = user_email {
            params.insert("userEmail".to_string(), json!(value));
        }
        if let Some(value) = user_phone {
            params.insert("userPhone".to_string(), json!(value));
        }
        if let Some(value) = user_name {
            params.insert("userName".to_string(), json!(value));
        }
        if let Some(value) = user_mfa {
            params.insert("userMFA".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/membership-privacy".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if new passwords are checked
    /// against most common passwords dictionary. When enabled, and user changes
    /// their password, password must not be contained in the dictionary.
    pub async fn update_password_dictionary_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/password-dictionary".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updates one of password strength policies. Based on total length
    /// configured, previous password hashes are stored, and users cannot choose a
    /// new password that is already stored in the passwird history list, when
    /// updating an user password, or setting new one through password recovery.
    /// 
    /// Keep in mind, while password history policy is disabled, the history is not
    /// being stored. Enabling the policy will not have any history on existing
    /// users, and it will only start to collect and enforce the policy on password
    /// changes since the policy is enabled.
    pub async fn update_password_history_policy(
        &self,
        total: Option<i64>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/password-history".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if password strength is checked
    /// against personal data. When enabled, and user sets or changes their
    /// password, the password must not contain user ID, name, email or phone
    /// number.
    pub async fn update_password_personal_data_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/password-personal-data".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if email alert is sent upon
    /// session creation. When enabled, and user signs into their account, they
    /// will be sent an email notification. There is an exception, the first
    /// session after a new sign up does not trigger an alert, even if the policy
    /// is enabled.
    pub async fn update_session_alert_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-alert".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update maximum duration how long sessions created within a project should
    /// stay active for.
    pub async fn update_session_duration_policy(
        &self,
        duration: i64,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("duration".to_string(), json!(duration));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-duration".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Updating this policy allows you to control if existing sessions should be
    /// invalidated when a password of a user is changed. When enabled, and user
    /// changes their password, they will be logged out of all their devices.
    pub async fn update_session_invalidation_policy(
        &self,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-invalidation".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the maximum number of sessions allowed per user. When the limit is
    /// hit, the oldest session will be deleted to make room for new one.
    pub async fn update_session_limit_policy(
        &self,
        total: Option<i64>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/session-limit".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the maximum number of users in the project. When the limit is hit or
    /// amount of existing users already exceeded the limit, all users remain
    /// active, but new user sign up will be prohibited.
    pub async fn update_user_limit_policy(
        &self,
        total: Option<i64>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/policies/user-limit".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update properties of a specific protocol. Use this endpoint to enable or
    /// disable a protocol in your project.
    pub async fn update_protocol(
        &self,
        protocol_id: crate::enums::ProtocolId,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/protocols/{protocolId}".to_string().replace("{protocolId}", &protocol_id.to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update properties of a specific service. Use this endpoint to enable or
    /// disable a service in your project.
    pub async fn update_service(
        &self,
        service_id: crate::enums::ServiceId,
        enabled: bool,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        params.insert("enabled".to_string(), json!(enabled));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/services/{serviceId}".to_string().replace("{serviceId}", &service_id.to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Update the SMTP configuration for your project. Use this endpoint to
    /// configure your project's SMTP provider with your custom settings for
    /// sending transactional emails.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_smtp(
        &self,
        host: Option<&str>,
        port: Option<i64>,
        username: Option<&str>,
        password: Option<&str>,
        sender_email: Option<&str>,
        sender_name: Option<&str>,
        reply_to_email: Option<&str>,
        reply_to_name: Option<&str>,
        secure: Option<crate::enums::Secure>,
        enabled: Option<bool>,
    ) -> crate::error::Result<crate::models::Project> {
        let mut params = HashMap::new();
        if let Some(value) = host {
            params.insert("host".to_string(), json!(value));
        }
        if let Some(value) = port {
            params.insert("port".to_string(), json!(value));
        }
        if let Some(value) = username {
            params.insert("username".to_string(), json!(value));
        }
        if let Some(value) = password {
            params.insert("password".to_string(), json!(value));
        }
        if let Some(value) = sender_email {
            params.insert("senderEmail".to_string(), json!(value));
        }
        if let Some(value) = sender_name {
            params.insert("senderName".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        if let Some(value) = secure {
            params.insert("secure".to_string(), json!(value));
        }
        if let Some(value) = enabled {
            params.insert("enabled".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/smtp".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Send a test email to verify SMTP configuration.
    pub async fn create_smtp_test(
        &self,
        emails: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<()> {
        let mut params = HashMap::new();
        params.insert("emails".to_string(), json!(emails.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/smtp/tests".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Update a custom email template for the specified locale and type. Use this
    /// endpoint to modify the content of your email templates.
    #[allow(clippy::too_many_arguments)]
    pub async fn update_email_template(
        &self,
        template_id: crate::enums::EmailTemplateType,
        locale: Option<crate::enums::EmailTemplateLocale>,
        subject: Option<&str>,
        message: Option<&str>,
        sender_name: Option<&str>,
        sender_email: Option<&str>,
        reply_to_email: Option<&str>,
        reply_to_name: Option<&str>,
    ) -> crate::error::Result<crate::models::EmailTemplate> {
        let mut params = HashMap::new();
        params.insert("templateId".to_string(), json!(template_id));
        if let Some(value) = locale {
            params.insert("locale".to_string(), json!(value));
        }
        if let Some(value) = subject {
            params.insert("subject".to_string(), json!(value));
        }
        if let Some(value) = message {
            params.insert("message".to_string(), json!(value));
        }
        if let Some(value) = sender_name {
            params.insert("senderName".to_string(), json!(value));
        }
        if let Some(value) = sender_email {
            params.insert("senderEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_email {
            params.insert("replyToEmail".to_string(), json!(value));
        }
        if let Some(value) = reply_to_name {
            params.insert("replyToName".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/templates/email".to_string();

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get a custom email template for the specified locale and type. This
    /// endpoint returns the template content, subject, and other configuration
    /// details.
    pub async fn get_email_template(
        &self,
        template_id: crate::enums::EmailTemplateType,
        locale: Option<crate::enums::EmailTemplateLocale>,
    ) -> crate::error::Result<crate::models::EmailTemplate> {
        let mut params = HashMap::new();
        if let Some(value) = locale {
            params.insert("locale".to_string(), json!(value));
        }

        let path = "/project/templates/email/{templateId}".to_string().replace("{templateId}", &template_id.to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Get a list of all project environment variables.
    pub async fn list_variables(
        &self,
        queries: Option<Vec<String>>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::VariableList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/project/variables".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new project environment variable. These variables can be accessed
    /// by all functions and sites in the project.
    pub async fn create_variable(
        &self,
        variable_id: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
        secret: Option<bool>,
    ) -> crate::error::Result<crate::models::Variable> {
        let mut params = HashMap::new();
        params.insert("variableId".to_string(), json!(variable_id.into()));
        params.insert("key".to_string(), json!(key.into()));
        params.insert("value".to_string(), json!(value.into()));
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/variables".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a variable by its unique ID.
    pub async fn get_variable(
        &self,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Variable> {
        let params = HashMap::new();

        let path = "/project/variables/{variableId}".to_string().replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update variable by its unique ID.
    pub async fn update_variable(
        &self,
        variable_id: impl Into<String>,
        key: Option<&str>,
        value: Option<&str>,
        secret: Option<bool>,
    ) -> crate::error::Result<crate::models::Variable> {
        let mut params = HashMap::new();
        if let Some(value) = key {
            params.insert("key".to_string(), json!(value));
        }
        if let Some(value) = value {
            params.insert("value".to_string(), json!(value));
        }
        if let Some(value) = secret {
            params.insert("secret".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/variables/{variableId}".to_string().replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a variable by its unique ID.
    pub async fn delete_variable(
        &self,
        variable_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/project/variables/{variableId}".to_string().replace("{variableId}", &variable_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Project {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let client = Client::new();
        let service = Project::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
