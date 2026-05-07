use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AuthMethod {
    #[serde(rename = "email-password")]
    #[default]
    EmailPassword,
    #[serde(rename = "magic-url")]
    MagicUrl,
    #[serde(rename = "email-otp")]
    EmailOtp,
    #[serde(rename = "anonymous")]
    Anonymous,
    #[serde(rename = "invites")]
    Invites,
    #[serde(rename = "jwt")]
    Jwt,
    #[serde(rename = "phone")]
    Phone,
}

impl AuthMethod {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            AuthMethod::EmailPassword => "email-password",
            AuthMethod::MagicUrl => "magic-url",
            AuthMethod::EmailOtp => "email-otp",
            AuthMethod::Anonymous => "anonymous",
            AuthMethod::Invites => "invites",
            AuthMethod::Jwt => "jwt",
            AuthMethod::Phone => "phone",
        }
    }
}

impl std::fmt::Display for AuthMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
