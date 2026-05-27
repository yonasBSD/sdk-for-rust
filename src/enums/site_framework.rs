use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SiteFramework {
    #[serde(rename = "analog")]
    #[default]
    Analog,
    #[serde(rename = "angular")]
    Angular,
    #[serde(rename = "nextjs")]
    Nextjs,
    #[serde(rename = "react")]
    React,
    #[serde(rename = "nuxt")]
    Nuxt,
    #[serde(rename = "vue")]
    Vue,
    #[serde(rename = "sveltekit")]
    Sveltekit,
    #[serde(rename = "astro")]
    Astro,
    #[serde(rename = "tanstack-start")]
    TanstackStart,
    #[serde(rename = "remix")]
    Remix,
    #[serde(rename = "lynx")]
    Lynx,
    #[serde(rename = "flutter")]
    Flutter,
    #[serde(rename = "react-native")]
    ReactNative,
    #[serde(rename = "vite")]
    Vite,
    #[serde(rename = "other")]
    Other,
}

impl SiteFramework {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            SiteFramework::Analog => "analog",
            SiteFramework::Angular => "angular",
            SiteFramework::Nextjs => "nextjs",
            SiteFramework::React => "react",
            SiteFramework::Nuxt => "nuxt",
            SiteFramework::Vue => "vue",
            SiteFramework::Sveltekit => "sveltekit",
            SiteFramework::Astro => "astro",
            SiteFramework::TanstackStart => "tanstack-start",
            SiteFramework::Remix => "remix",
            SiteFramework::Lynx => "lynx",
            SiteFramework::Flutter => "flutter",
            SiteFramework::ReactNative => "react-native",
            SiteFramework::Vite => "vite",
            SiteFramework::Other => "other",
        }
    }
}

impl std::fmt::Display for SiteFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
