// use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Database {
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Logging {
    pub log_level: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct ConfigInfo {
    pub location: Option<String>,
    pub env_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    // #[serde(default)]
    // pub config: ConfigInfo,
    // #[serde(default)]
    // pub database: Database,
    // #[serde(default)]
    // pub logging: Logging,
}
