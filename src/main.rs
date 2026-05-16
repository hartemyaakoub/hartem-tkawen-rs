//! hartem-tkawen-rs — personal site of Hartem Yaakoub, served from Rust.
//! Axum 0.7 + Maud + rust-embed. AGPL-3.0.

use axum::{
    body::Body,
    extract::Query,
    http::{header, HeaderValue, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::{net::SocketAddr, str::FromStr, time::Instant};
use tower_http::{
    compression::CompressionLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

mod content;
mod i18n;
mod schemas;
mod templates;

use i18n::Lang;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const SERVER_BANNER: &str = concat!("hartem-rs/", env!("CARGO_PKG_VERSION"), " (axum/maud)");

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

#[derive(Debug, Deserialize)]
struct LangQ {
    lang: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info,tower_http=info".into()))
        .init();

    let app = Router::new()
        .route("/", get(home))
        .route("/healthz", get(|| async { "ok" }))
        .route("/robots.txt", get(robots))
        .route("/sitemap.xml", get(sitemap))
        .route("/llms.txt", get(llms))
        .route("/favicon.svg", get(favicon))
        .route("/manifest.webmanifest", get(manifest))
        .route("/img/*path", get(static_file::<&str>))
        .route("/.well-known/*path", get(static_file::<&str>))
        .route("/schemas/*path", get(static_file::<&str>))
        .layer(CompressionLayer::new().gzip(true).br(true))
        .layer(SetResponseHeaderLayer::overriding(
            header::SERVER,
            HeaderValue::from_static(SERVER_BANNER),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-powered-by"),
            HeaderValue::from_static("Rust + Axum + Maud (AGPL-3.0)"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("strict-transport-security"),
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        .layer(TraceLayer::new_for_http());

    // bind to unix socket OR tcp depending on env
    let addr: SocketAddr = std::env::var("HARTEM_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3088".to_string())
        .parse()
        .expect("HARTEM_ADDR must be a valid SocketAddr");

    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    tracing::info!("hartem-tkawen-rs v{} listening on http://{}", VERSION, addr);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    tracing::info!("shutdown signal received");
}

async fn home(Query(q): Query<LangQ>) -> Response {
    let lang = q.lang
        .as_deref()
        .and_then(|s| Lang::from_str(s).ok())
        .unwrap_or(Lang::En);

    let start = Instant::now();
    let body = templates::page(lang, 0).into_string();
    let render_us = start.elapsed().as_micros();

    // re-render once with the actual measured time so the footer reflects truth.
    let final_body = templates::page(lang, render_us).into_string();
    let total_us = start.elapsed().as_micros();

    let timing = format!("render;dur={}.{:03}", total_us / 1000, total_us % 1000);

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .header("server-timing", timing)
        .header("cache-control", "public, max-age=300, stale-while-revalidate=86400")
        .header("vary", "accept-encoding, accept-language")
        .body(Body::from(final_body))
        .unwrap()
}

async fn robots() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        include_str!("../assets/robots.txt"),
    )
}

async fn sitemap() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        include_str!("../assets/sitemap.xml"),
    )
}

async fn llms() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        include_str!("../assets/llms.txt"),
    )
}

async fn favicon() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "image/svg+xml")],
        include_str!("../assets/favicon.svg"),
    )
}

async fn manifest() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/manifest+json")],
        include_str!("../assets/manifest.webmanifest"),
    )
}

async fn static_file<T: AsRef<str>>(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            Response::builder()
                .header(header::CONTENT_TYPE, mime.as_ref())
                .header(header::CACHE_CONTROL, "public, max-age=604800, immutable")
                .body(Body::from(content.data.into_owned()))
                .unwrap()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
