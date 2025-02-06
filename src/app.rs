use std::{
    io::IsTerminal,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::Arc,
};

use axum::{http::StatusCode, routing::get, Router};
use clap::{Parser, Subcommand};
use color_eyre::config::{HookBuilder, Theme};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    config::{db::DatabaseCommands, state::AppContext},
    controllers::auth,
    error::Result as AppResult,
    tracing::{http, instrumentation::InstrumentationCommands},
};

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Configure how to connect to a database.
    Database(DatabaseCommands),

    /// Configure telemetry and instrumentation of the app.
    Instrumentation(InstrumentationCommands),
}

/// Configuration details of our web server.
#[derive(Parser, Clone)]
#[command(
    name = "todos",
    version = "0.1.0",
    about = "A web backend for todos application",
    author = "Simon Bittok <bittokkibet@gmail.com>"
)]
pub struct App {
    /// The socket address which the TCP will bind to.
    /// Uses IP V6
    #[clap(long, default_value_t = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 8000))]
    pub address: SocketAddr,
    #[command(subcommand)]
    pub commands: Commands,
}

impl App {
    pub async fn run() -> AppResult<()> {
        HookBuilder::default()
            .theme(if std::io::stderr().is_terminal() {
                Theme::dark()
            } else {
                Theme::new()
            })
            .install()?;

        let cli = Self::parse();

        match &cli.commands {
            Commands::Instrumentation(telemetry) => {
                let _setup = telemetry.setup()?;
            }
            _ => {}
        }

        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(http::make_span_with)
            .on_request(http::on_request)
            .on_response(http::on_response);

        let ctx = AppContext::new(&cli).await?;

        let app = Router::new()
            .route("/", get(hello))
            .route("/health", get(health))
            .fallback(page_404)
            .nest("/auth", auth::routes())
            .layer(trace_layer)
            .with_state(Arc::new(ctx));

        tracing::info!("Listening on {}", &cli.address);

        let listener = TcpListener::bind(&cli.address).await?;

        axum::serve(listener, app).await?;

        Ok(())
    }
}

#[tracing::instrument]
async fn hello() -> impl axum::response::IntoResponse {
    let message = "Welcome to my Home Page!";

    (StatusCode::OK, message.to_string())
}

#[tracing::instrument]
async fn health() -> impl axum::response::IntoResponse {
    let message = "Server is UP and Running";

    (StatusCode::OK, message.to_string())
}

#[tracing::instrument]
async fn page_404() -> impl axum::response::IntoResponse {
    let message = "The requested page was not found";

    (StatusCode::NOT_FOUND, message.to_string())
}
