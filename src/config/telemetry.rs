use serde::Deserialize;

use crate::{
    error::{self, Result},
    tracing::logger::{Level, Logger},
};

use std::{env::VarError, error::Error, io::IsTerminal};

use color_eyre::eyre::WrapErr;
use tracing::Subscriber;
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    filter::Directive,
    layer::{Layer, SubscriberExt},
    registry::LookupSpan,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Debug, Clone, Deserialize)]
pub struct TelemetryConfig {
    pub directives: Vec<String>,
    pub logger: Logger,
    pub level: Level,
}

impl TelemetryConfig {
    pub fn setup(&self) -> Result<()> {
        let filter_layer = self.filter_layer()?;

        let registry = tracing_subscriber::registry()
            .with(filter_layer)
            .with(ErrorLayer::default());

        match self.logger {
            Logger::Compact => registry.with(self.fmt_layer_compact()).try_init()?,
            Logger::Full => registry.with(self.fmt_layer_full()).try_init()?,
            Logger::Json => registry.with(self.fmt_layer_json()).try_init()?,
            Logger::Pretty => registry.with(self.fmt_layer_pretty()).try_init()?,
        }

        Ok(())
    }

    pub fn filter_layer(&self) -> Result<EnvFilter> {
        let mut filter_layer = match EnvFilter::try_from_default_env() {
            Ok(filter) => filter,
            Err(e) => {
                // Catch a parsing error and ignore missing env error
                if let Some(source) = e.source() {
                    match source.downcast_ref::<VarError>() {
                        Some(VarError::NotPresent) => (),
                        _ => return Err(error::Error::InternalServerError.into()),
                    }
                }
                // if --directive is specified, don't set a default
                if self.directives.is_empty() {
                    EnvFilter::try_new(&format!(
                        "{}={}",
                        env!("CARGO_PKG_NAME").replace('-', "_"),
                        self.log_level()
                    ))?
                } else {
                    EnvFilter::try_new("")?
                }
            }
        };

        let directives = self.directives()?;

        for directive in directives {
            let cloned = directive.clone();
            filter_layer = filter_layer.add_directive(cloned);
        }

        Ok(filter_layer)
    }
    pub fn fmt_layer_compact<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .compact()
            .without_time()
            .with_target(false)
            .with_line_number(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_file(false)
    }

    pub fn fmt_layer_full<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
    }

    pub fn fmt_layer_json<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .json()
    }

    pub fn fmt_layer_pretty<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::Layer::new()
            .with_ansi(std::io::stderr().is_terminal())
            .with_writer(std::io::stderr)
            .pretty()
    }

    pub fn log_level(&self) -> &str {
        self.level.as_str()
    }

    pub fn directives(&self) -> Result<Vec<Directive>> {
        let mut directives = Vec::new();

        for module in &self.directives {
            let directive = format!("{}={}", module, self.log_level());

            match directive.parse::<Directive>() {
                Ok(directive) => directives.push(directive),
                Err(e) => {
                    return Err(error::Error::InternalServerError.into());
                }
            }
        }

        Ok(directives)
    }
}
