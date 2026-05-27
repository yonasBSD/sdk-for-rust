use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum RedirectStatusCode {
    #[serde(rename = "301")]
    #[default]
    MovedPermanently,
    #[serde(rename = "302")]
    Found,
    #[serde(rename = "307")]
    TemporaryRedirect,
    #[serde(rename = "308")]
    PermanentRedirect,
}

impl RedirectStatusCode {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            RedirectStatusCode::MovedPermanently => "301",
            RedirectStatusCode::Found => "302",
            RedirectStatusCode::TemporaryRedirect => "307",
            RedirectStatusCode::PermanentRedirect => "308",
        }
    }
}

impl std::fmt::Display for RedirectStatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
