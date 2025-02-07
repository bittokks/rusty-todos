use std::fmt::{self, Display, Formatter};

use clap::ValueEnum;
use serde::Deserialize;

#[derive(Debug, Default, Clone, ValueEnum, PartialEq, Deserialize)]
pub enum Logger {
    #[default]
    Compact,
    Full,
    Json,
    Pretty,
}

impl From<&str> for Logger {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "full" => Self::Full,
            "json" => Self::Json,
            "pretty" => Self::Pretty,
            _ => Self::Compact,
        }
    }
}

impl Display for Logger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let logger = match self {
            Self::Compact => "compact",
            Self::Full => "full",
            Self::Json => "json",
            Self::Pretty => "pretty",
        };

        write!(f, "{}", logger)
    }
}

#[derive(Debug, Clone, PartialEq, Default, Deserialize)]
pub enum Level {
    Off,
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Off => "off",
            Self::Trace => "trace",
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for Level {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "trace" => Self::Trace,
            "debug" => Self::Debug,
            "info" => Self::Info,
            "warn" => Self::Warn,
            "error" => Self::Error,
            _ => Self::Off,
        }
    }
}
