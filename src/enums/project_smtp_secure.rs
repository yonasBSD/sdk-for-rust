use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectSMTPSecure {
    #[serde(rename = "tls")]
    #[default]
    Tls,
    #[serde(rename = "ssl")]
    Ssl,
}

impl ProjectSMTPSecure {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectSMTPSecure::Tls => "tls",
            ProjectSMTPSecure::Ssl => "ssl",
        }
    }
}

impl std::fmt::Display for ProjectSMTPSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
