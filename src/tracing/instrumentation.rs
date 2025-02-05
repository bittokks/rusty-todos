use std::{env::VarError, error::Error, io::IsTerminal};

use clap::{ArgAction, Args};
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

use super::logger::Logger;

#[derive(Debug, Default, Args)]
pub struct Instrumentation {
    /// Enable debug logs -vv for trace
    #[clap(short = 'v',  long, action = ArgAction::Count, global = true)]
    verbose: u8,
    /// Which logger to use i.e JSON, FULL
    #[clap(long, default_value_t = Default::default(), global = true)]
    logger: Logger,
    /// Tracing direcctives
    #[clap(long = "directive", global = true, value_delimiter = ',', num_args = 0.. )]
    directives: Vec<Directive>,
}

impl Instrumentation {
    pub fn log_level(&self) -> String {
        match self.verbose {
            0 => "error",
            1 => "warn",
            2 => "info",
            3 => "debug",
            _ => "trace",
        }
        .to_string()
    }

    pub fn setup(&self) -> color_eyre::Result<()> {
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

    pub fn filter_layer(&self) -> color_eyre::Result<EnvFilter> {
        let mut filter_layer = match EnvFilter::try_from_default_env() {
            Ok(filter) => filter,
            Err(e) => {
                // Catch a parsing error and ignore missing env error
                if let Some(source) = e.source() {
                    match source.downcast_ref::<VarError>() {
                        Some(VarError::NotPresent) => (),
                        _ => return Err(e).wrap_err_with(|| "parsing RUST_LOG directives"),
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

        for directive in &self.directives {
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
}
