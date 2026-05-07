use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectPolicy {
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

impl ProjectPolicy {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectPolicy::PasswordDictionary => "password-dictionary",
            ProjectPolicy::PasswordHistory => "password-history",
            ProjectPolicy::PasswordPersonalData => "password-personal-data",
            ProjectPolicy::SessionAlert => "session-alert",
            ProjectPolicy::SessionDuration => "session-duration",
            ProjectPolicy::SessionInvalidation => "session-invalidation",
            ProjectPolicy::SessionLimit => "session-limit",
            ProjectPolicy::UserLimit => "user-limit",
            ProjectPolicy::MembershipPrivacy => "membership-privacy",
        }
    }
}

impl std::fmt::Display for ProjectPolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
