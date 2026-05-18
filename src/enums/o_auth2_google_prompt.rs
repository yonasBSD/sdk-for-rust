use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum OAuth2GooglePrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "consent")]
    Consent,
    #[serde(rename = "select_account")]
    SelectAccount,
}

impl OAuth2GooglePrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            OAuth2GooglePrompt::None => "none",
            OAuth2GooglePrompt::Consent => "consent",
            OAuth2GooglePrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for OAuth2GooglePrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
