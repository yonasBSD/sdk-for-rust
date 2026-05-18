use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectOAuth2GooglePrompt {
    #[serde(rename = "none")]
    #[default]
    None,
    #[serde(rename = "consent")]
    Consent,
    #[serde(rename = "select_account")]
    SelectAccount,
}

impl ProjectOAuth2GooglePrompt {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectOAuth2GooglePrompt::None => "none",
            ProjectOAuth2GooglePrompt::Consent => "consent",
            ProjectOAuth2GooglePrompt::SelectAccount => "select_account",
        }
    }
}

impl std::fmt::Display for ProjectOAuth2GooglePrompt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
