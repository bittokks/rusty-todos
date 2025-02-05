use axum::{body::Body, extract::ConnectInfo, http::Request, response::Response};
use tracing::Span;

use std::{net::SocketAddr, time::Duration};

pub fn make_span_with(request: &Request<Body>) -> Span {
    tracing::error_span!("http",
        uri = %request.uri(),
        method = %request.method(),
        source = request.extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|info| {
            tracing::field::display(info.ip().to_string())
            }).unwrap_or_else(|| tracing::field::display("<unkown>".to_string())),
        status = tracing::field::Empty,
        latency = tracing::field::Empty, version = tracing::field::Empty)
}

pub fn on_request(request: &Request<Body>, span: &Span) {
    tracing::trace!("Request");
    span.record("version", tracing::field::debug(request.version()));
}

pub fn on_response(response: &Response<Body>, latency: Duration, span: &Span) {
    span.record(
        "latency",
        tracing::field::display(format!("{}µs", latency.as_micros())),
    );
    span.record("status", tracing::field::display(response.status()));
    tracing::trace!("Response");
}
