use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Region {
    #[serde(rename = "fra")]
    #[default]
    Fra,
    #[serde(rename = "nyc")]
    Nyc,
    #[serde(rename = "syd")]
    Syd,
    #[serde(rename = "sfo")]
    Sfo,
    #[serde(rename = "sgp")]
    Sgp,
    #[serde(rename = "tor")]
    Tor,
}

impl Region {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Region::Fra => "fra",
            Region::Nyc => "nyc",
            Region::Syd => "syd",
            Region::Sfo => "sfo",
            Region::Sgp => "sgp",
            Region::Tor => "tor",
        }
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
