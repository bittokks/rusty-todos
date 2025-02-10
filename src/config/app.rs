use std::{borrow::Cow, path::PathBuf};

use crate::error::{Error, Result};

use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

use super::{db::DatabaseConfig, telemetry::TelemetryConfig};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum AppEnvironment<'a> {
    #[default]
    Development,
    Production,
    Testing,
    Other(Cow<'a, str>),
}

impl<'a> AppEnvironment<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Development => "development",
            Self::Production => "production",
            Self::Testing => "testing",
            Self::Other(env) => env,
        }
    }
}

impl<'a> From<&str> for AppEnvironment<'a> {
    fn from(s: &str) -> Self {
        match s.to_string().to_lowercase().as_str() {
            "development" | "dev" => Self::Development,
            "production" | "prod" => Self::Production,
            "testing" | "test" => Self::Testing,
            other => Self::Other(Cow::Owned(other.into())),
        }
    }
}

impl std::fmt::Display for AppEnvironment<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub telemetry: TelemetryConfig,
}

impl AppConfig {
    pub fn dir(env: &AppEnvironment) -> Result<PathBuf> {
        let base_dir = std::env::current_dir()?;
        let config_dir = base_dir.join("config");

        let file = config_dir.join(format!("{}.yaml", env.as_str()));

        if file.exists() && file.is_file() {
            Ok(file)
        } else {
            Err(Error::ConfigFile("Config File has an error".into()).into())
        }
    }

    pub fn build(env: &AppEnvironment) -> Result<Self> {
        let file = Self::dir(env)?;

        let settings = Config::builder()
            .add_source(File::from(file))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        settings.try_deserialize::<Self>().map_err(Into::into)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", &self.host, self.port)
    }

    pub fn url(&self) -> String {
        format!("{}://{}:{}", &self.protocol, &self.host, self.port)
    }
}
