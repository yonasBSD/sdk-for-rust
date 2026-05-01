use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum EmailTemplateType {
    #[serde(rename = "verification")]
    #[default]
    Verification,
    #[serde(rename = "magicSession")]
    MagicSession,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "invitation")]
    Invitation,
    #[serde(rename = "mfaChallenge")]
    MfaChallenge,
    #[serde(rename = "sessionAlert")]
    SessionAlert,
    #[serde(rename = "otpSession")]
    OtpSession,
}

impl EmailTemplateType {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            EmailTemplateType::Verification => "verification",
            EmailTemplateType::MagicSession => "magicSession",
            EmailTemplateType::Recovery => "recovery",
            EmailTemplateType::Invitation => "invitation",
            EmailTemplateType::MfaChallenge => "mfaChallenge",
            EmailTemplateType::SessionAlert => "sessionAlert",
            EmailTemplateType::OtpSession => "otpSession",
        }
    }
}

impl std::fmt::Display for EmailTemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
