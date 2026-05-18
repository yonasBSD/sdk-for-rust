use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectPolicyId {
    #[serde(rename = "password-dictionary")]
    #[default]
    PasswordDictionary,
    #[serde(rename = "password-history")]
    PasswordHistory,
    #[serde(rename = "password-personal-data")]
    PasswordPersonalData,
    #[serde(rename = "session-alert")]
    SessionAlert,
    #[serde(rename = "session-duration")]
    SessionDuration,
    #[serde(rename = "session-invalidation")]
    SessionInvalidation,
    #[serde(rename = "session-limit")]
    SessionLimit,
    #[serde(rename = "user-limit")]
    UserLimit,
    #[serde(rename = "membership-privacy")]
    MembershipPrivacy,
}

impl ProjectPolicyId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectPolicyId::PasswordDictionary => "password-dictionary",
            ProjectPolicyId::PasswordHistory => "password-history",
            ProjectPolicyId::PasswordPersonalData => "password-personal-data",
            ProjectPolicyId::SessionAlert => "session-alert",
            ProjectPolicyId::SessionDuration => "session-duration",
            ProjectPolicyId::SessionInvalidation => "session-invalidation",
            ProjectPolicyId::SessionLimit => "session-limit",
            ProjectPolicyId::UserLimit => "user-limit",
            ProjectPolicyId::MembershipPrivacy => "membership-privacy",
        }
    }
}

impl std::fmt::Display for ProjectPolicyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
