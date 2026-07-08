use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE: &str = "commit-linter.toml";

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Config {
    pub max_header_length: usize,
    pub scope_required: bool,
    pub scope_no_spaces: bool,
    pub scope_lowercase: bool,
    pub types: Vec<String>,
    #[serde(default)]
    pub rules: RulesConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RulesConfig {
    #[serde(default = "enabled")]
    pub header_format: bool,
    #[serde(default = "enabled")]
    pub header_max_length: bool,
    #[serde(default = "enabled")]
    pub type_required: bool,
    #[serde(default = "enabled")]
    pub type_lowercase: bool,
    #[serde(default = "enabled")]
    pub type_allowed: bool,
    #[serde(default = "enabled")]
    pub subject_required: bool,
    #[serde(default = "enabled")]
    pub scope_not_empty: bool,
    #[serde(default = "enabled")]
    pub scope_no_spaces: bool,
    #[serde(default = "enabled")]
    pub scope_lowercase: bool,
    #[serde(default = "enabled")]
    pub scope_required: bool,
}

fn enabled() -> bool {
    true
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            header_format: true,
            header_max_length: true,
            type_required: true,
            type_lowercase: true,
            type_allowed: true,
            subject_required: true,
            scope_not_empty: true,
            scope_no_spaces: true,
            scope_lowercase: true,
            scope_required: true,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_header_length: 100,
            scope_required: false,
            scope_no_spaces: true,
            scope_lowercase: true,
            types: vec![
                "feat".into(),
                "fix".into(),
                "docs".into(),
                "style".into(),
                "refactor".into(),
                "perf".into(),
                "test".into(),
                "build".into(),
                "ci".into(),
                "chore".into(),
                "revert".into(),
            ],
            rules: RulesConfig::default(),
        }
    }
}

pub fn load_config() -> Result<Config> {
    let path = Path::new(CONFIG_FILE);
    if !path.exists() {
        return Ok(Config::default());
    }

    let contents = fs::read_to_string(path).context("failed to read commit-linter.toml")?;
    toml::from_str(&contents).context("failed to parse commit-linter.toml")
}

pub fn init_config() -> Result<bool> {
    let path = Path::new(CONFIG_FILE);
    if path.exists() {
        return Ok(false);
    }

    fs::write(path, default_config_toml()).context("failed to write commit-linter.toml")?;
    Ok(true)
}

pub fn default_config_toml() -> String {
    toml::to_string_pretty(&Config::default()).expect("default config should serialize")
}
