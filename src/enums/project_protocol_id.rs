use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ProjectProtocolId {
    #[serde(rename = "rest")]
    #[default]
    Rest,
    #[serde(rename = "graphql")]
    Graphql,
    #[serde(rename = "websocket")]
    Websocket,
}

impl ProjectProtocolId {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            ProjectProtocolId::Rest => "rest",
            ProjectProtocolId::Graphql => "graphql",
            ProjectProtocolId::Websocket => "websocket",
        }
    }
}

impl std::fmt::Display for ProjectProtocolId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
