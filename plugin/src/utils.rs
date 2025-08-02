use std::fs;
use serde_json::Result;
use crate::Config;
pub fn read_config(path: &str) -> Result<Config> {
    let data = fs::read_to_string(path)
        .expect("Unable to read file");

    let config: Config = serde_json::from_str(&data)?;
    Ok(config)
}