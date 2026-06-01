use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum StatusCode {
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

impl StatusCode {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            StatusCode::MovedPermanently => "301",
            StatusCode::Found => "302",
            StatusCode::TemporaryRedirect => "307",
            StatusCode::PermanentRedirect => "308",
        }
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
