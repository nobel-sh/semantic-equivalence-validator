use serde_derive::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    FileRead(#[from] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("Invalid configuration: {0}")]
    Validation(String),
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub rustc: RustcConfig,
    pub gccrs: GccrsConfig,
}

#[derive(Deserialize, Debug)]
pub struct RustcConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct GccrsConfig {
    pub path: PathBuf,
    pub args: Vec<String>,
}

impl AppConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if !self.rustc.path.exists() {
            return Err(ConfigError::Validation(format!(
                "Rustc path does not exist: {}",
                self.rustc.path.display()
            )));
        }
        if !self.gccrs.path.exists() {
            return Err(ConfigError::Validation(format!(
                "GCCRS path does not exist: {}",
                self.gccrs.path.display()
            )));
        }
        Ok(())
    }
}

