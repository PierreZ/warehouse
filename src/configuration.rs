use config::{Config, ConfigError, File};
use log::debug;
use serde;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub ssh: SSHConfig,
    pub elasticsearch: ESConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSHConfig {
    pub user: String,
    pub package_manager: String,
    pub key_path: String,
    pub key_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESConfig {
    pub url: String,
    pub index: String,
}

impl Settings {
    pub fn from(path: String) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name(&path).required(false))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
