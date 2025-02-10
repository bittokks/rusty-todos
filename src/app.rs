use std::{
    io::IsTerminal,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::Arc,
};

use axum::{http::StatusCode, routing::get, Router};
use clap::Parser;
use color_eyre::config::{HookBuilder, Theme};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::{
    config::{
        app::{AppConfig, AppEnvironment},
        state::AppContext,
    },
    controllers::auth,
    error::Result as AppResult,
    tracing::http,
};

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

    /// The file which the configuration details are contained in. Must be a yaml file, in a config parallel
    /// to the source directory
    #[clap(long, default_value_t = AppEnvironment::Development)]
    pub env: AppEnvironment<'static>,
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

        let config = AppConfig::build(&cli.env)?;

        config.telemetry.setup()?;

        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(http::make_span_with)
            .on_request(http::on_request)
            .on_response(http::on_response);

        let ctx = AppContext::new(&config).await?;

        let app = Router::new()
            .route("/", get(hello))
            .route("/health", get(health))
            .fallback(page_404)
            .nest("/auth", auth::routes())
            .layer(trace_layer)
            .with_state(Arc::new(ctx));

        let listener = match TcpListener::bind(config.server.address()).await {
            Ok(listener) => listener,
            Err(e) => match e.kind() {
                tokio::io::ErrorKind::AddrInUse => TcpListener::bind(&cli.address).await?,
                _ => {
                    return Err(crate::error::Error::InternalServerError.into());
                }
            },
        };

        tracing::info!("Listening on {:?}", listener.local_addr().unwrap());

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
