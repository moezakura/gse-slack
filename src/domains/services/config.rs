use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

pub struct ConfigService {
    data: ConfigData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigData {
    pub slack_token: String,
    pub gse_token: String,
}

impl ConfigService {
    pub fn new(config_path: String) -> Result<ConfigService, Box<dyn Error>> {
        let content = fs::read_to_string(config_path)?;
        let data: ConfigData = serde_yaml::from_str(&content)?;

        Ok(ConfigService { data })
    }

    pub fn get_data(&self) -> ConfigData {
        self.data.clone()
    }
}
