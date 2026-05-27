use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SiteAdapter {
    #[serde(rename = "static")]
    #[default]
    Static,
    #[serde(rename = "ssr")]
    Ssr,
}

impl SiteAdapter {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            SiteAdapter::Static => "static",
            SiteAdapter::Ssr => "ssr",
        }
    }
}

impl std::fmt::Display for SiteAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
