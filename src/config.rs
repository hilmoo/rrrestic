use anyhow::{Context, Result};
use serde::Deserialize;
use std::{collections::HashMap, env, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub backup: Vec<BackupJob>,
}

#[derive(Debug, Deserialize)]
pub struct BackupJob {
    pub name: String,
    pub repository: String,
    pub source: String,
    pub env: Option<HashMap<String, String>>,
    pub before: Option<Vec<String>>,
    pub after: Option<Vec<String>>,
    pub failure: Option<Vec<String>>,
    pub success: Option<Vec<String>>,
    pub extra_args: Option<Vec<String>>,
}

impl Config {
    pub fn get_config_path() -> PathBuf {
        if let Ok(path) = env::var("RRRESTIC_CONFIG") {
            return PathBuf::from(path);
        }

        PathBuf::from("rrrestic.toml")
    }

    pub fn load(path: &str) -> Result<Self> {
        let file_contents = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {}", path))?;

        let expanded = shellexpand::env(&file_contents).context("Failed to expand env vars")?;

        let config: Config = toml::from_str(&expanded)
            .with_context(|| format!("Failed to parse TOML from {}", path))?;

        Ok(config)
    }
}
