use std::{
    io::IsTerminal,
    net::{IpAddr, Ipv6Addr, SocketAddr},
};

use axum::{http::StatusCode, routing::get, Router};
use clap::Parser;
use color_eyre::config::{HookBuilder, Theme};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::tracing::{http, instrumentation::Instrumentation};

#[derive(Parser)]
pub struct App {
    #[clap(long, default_value_t = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 8000))]
    address: SocketAddr,
    #[clap(flatten)]
    instrumentation: Instrumentation,
}

impl App {
    pub async fn run() -> color_eyre::Result<()> {
        HookBuilder::default()
            .theme(if std::io::stderr().is_terminal() {
                Theme::dark()
            } else {
                Theme::new()
            })
            .install()?;

        let cli = Self::parse();
        cli.instrumentation.setup()?;

        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(http::make_span_with)
            .on_request(http::on_request)
            .on_response(http::on_response);

        let app = Router::new()
            .route("/", get(hello))
            .route("/health", get(health))
            .fallback(page_404)
            .layer(trace_layer);
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
