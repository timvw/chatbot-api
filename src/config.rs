use serde::Deserialize;
use std::string::ToString;

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Logging {
    pub log_level: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
    pub max_tokens: u32,
    pub model: String,
}

impl Default for ConfigInfo {
    fn default() -> ConfigInfo {
        ConfigInfo {
            location: None,
            env_prefix: None,
            model: "gpt-4o".to_string(),
            max_tokens: 4096,
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Settings {
    #[serde(default)]
    pub config: ConfigInfo,
    #[serde(default)]
    pub logging: Logging,
}
