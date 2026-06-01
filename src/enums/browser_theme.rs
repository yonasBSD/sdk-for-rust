use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BrowserTheme {
    #[serde(rename = "light")]
    #[default]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

impl BrowserTheme {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            BrowserTheme::Light => "light",
            BrowserTheme::Dark => "dark",
        }
    }
}

impl std::fmt::Display for BrowserTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
