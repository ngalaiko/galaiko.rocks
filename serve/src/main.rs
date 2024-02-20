use axum::extract::Host;
use axum::http::uri::Authority;
use axum::http::{header, StatusCode, Uri};
use axum::response::{IntoResponse, Redirect};
use axum::{routing::get, Router};
use clap::Parser;
use tower_http::compression::CompressionLayer;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use shared::path;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:8080")]
    address: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();
    let serve = Cli::parse();
    let comression_layer: CompressionLayer = CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true);
    let app = Router::new()
        .route("/", get(handler))
        .route("/*path", get(handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(comression_layer);
    let listener = tokio::net::TcpListener::bind(serve.address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(rust_embed::RustEmbed)]
#[folder = "public/"]
struct Public;

async fn handler(
    headers: axum::http::HeaderMap,
    uri: Uri,
    Host(hostname): Host,
) -> impl IntoResponse {
    if !cfg!(debug_assertions) && hostname != "nikita.galaiko.rocks" {
        let mut parts = uri.into_parts();
        parts.scheme = Some("https".parse().expect("always valid"));
        parts.authority = Some(Authority::from_static("nikita.galaiko.rocks"));
        let uri = Uri::from_parts(parts).expect("always valid");
        return Redirect::to(&uri.to_string()).into_response();
    }

    let Ok(requested_path) = urlencoding::decode(uri.path()) else {
        return (StatusCode::BAD_REQUEST, "400 Bad Request").into_response();
    };

    let normalized_path = path::normalize(requested_path.to_string());
    if normalized_path.display().to_string() != requested_path {
        return Redirect::to(normalized_path.display().to_string().as_str()).into_response();
    }

    let Some(embedded_file) = Public::iter()
        .find(|asset_path| path::normalize(asset_path.to_string()) == normalized_path)
        .and_then(|asset_path| Public::get(&asset_path))
    else {
        return (StatusCode::NOT_FOUND, "404 Not Found").into_response()
    };

    let etag = etag_header(embedded_file.metadata.sha256_hash());
    let is_modified = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        .map_or(false, |value| value.contains(&etag));
    let content_type = mime_guess::from_path(normalized_path)
        .first_or_octet_stream()
        .to_string();

    let headers = [
        (header::ETAG, etag),
        (
            header::CACHE_CONTROL,
            match content_type.as_str() {
                "text/html" => "no-cache, max-age=31536000".to_string(),
                _ => "max-age=31536000".to_string(),
            },
        ),
        (header::CONTENT_TYPE, content_type),
    ];

    if is_modified {
        (StatusCode::NOT_MODIFIED, headers).into_response()
    } else {
        (StatusCode::OK, headers, embedded_file.data).into_response()
    }
}

fn etag_header(hash: [u8; 32]) -> String {
    format!(
        "W\\\"{:x}{:x}{:x}{:x}-{:x}{:x}\"",
        hash[0], hash[1], hash[2], hash[3], hash[30], hash[31],
    )
}
