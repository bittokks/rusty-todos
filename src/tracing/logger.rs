use std::fmt::{self, Display, Formatter};

use clap::ValueEnum;

#[derive(Debug, Default, Clone, ValueEnum)]
pub enum Logger {
    #[default]
    Compact,
    Full,
    Json,
    Pretty,
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
