use clap::Parser;

use shared::path;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "127.0.0.1:8080")]
    address: String,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    femme::start();

    let cli = Cli::parse();
    serve(&cli.address).await
}

#[derive(rust_embed::RustEmbed)]
#[folder = "public/"]
struct Public;

async fn serve(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    #[cfg(not(debug_assertions))]
    app.with(HostnameRedirectMiddleware);
    app.with(tide_compress::CompressMiddleware::new());
    app.at("/").get(serve_asset);
    app.at("/*path").get(serve_asset);
    app.listen(address).await?;
    Ok(())
}

async fn serve_asset(req: tide::Request<()>) -> tide::Result {
    let requested_path = req.url().path();
    let requested_path = urlencoding::decode(requested_path)
        .map_err(|error| tide::Error::new(tide::StatusCode::BadRequest, error))?;

    let normalized_path = path::normalize(requested_path.to_string());
    if requested_path.to_string() != normalized_path.display().to_string() {
        let response: tide::Response =
            tide::Redirect::new(normalized_path.display().to_string()).into();
        return Ok(response);
    };

    let asset_path = Public::iter()
        .find(|asset_path| path::normalize(asset_path.to_string()) == normalized_path);

    let response =
        if let Some(embedded_file) = asset_path.and_then(|asset_path| Public::get(&asset_path)) {
            let etag = etag_header(embedded_file.metadata.sha256_hash());
            let is_modified = req
                .header("if-none-match")
                .map(|etags| etags.contains(&etag))
                .unwrap_or_default();
            let content_type = mime_guess::from_path(normalized_path)
                .first_or_octet_stream()
                .to_string();
            if is_modified {
                tide::Response::builder(tide::StatusCode::NotModified)
            } else {
                tide::Response::builder(tide::StatusCode::Ok)
                    .body(tide::Body::from(embedded_file.data.to_vec()))
            }
            .header("content-type", content_type.to_string())
            .header(
                "cache-control",
                match content_type.as_str() {
                    "text/html" => "no-cache, max-age=31536000",
                    _ => "max-age=31536000",
                },
            )
            .header("etag", etag)
            .build()
        } else {
            tide::Response::new(tide::StatusCode::NotFound)
        };

    Ok(response)
}

fn etag_header(hash: [u8; 32]) -> tide::http::headers::HeaderValue {
    tide::http::headers::HeaderValue::from_bytes(
        format!(
            "W\\\"{:x}{:x}{:x}{:x}-{:x}{:x}\"",
            hash[0], hash[1], hash[2], hash[3], hash[30], hash[31],
        )
        .as_bytes()
        .to_vec(),
    )
    .expect("sha256 hash is valid ASCII")
}

struct HostnameRedirectMiddleware;

#[tide::utils::async_trait]
impl tide::Middleware<()> for HostnameRedirectMiddleware {
    async fn handle(&self, request: tide::Request<()>, next: tide::Next<'_, ()>) -> tide::Result {
        if let Some("nikita.galaiko.rocks") = request.host() {
            let response: tide::Response = next.run(request).await;
            Ok(response)
        } else {
            let location = format!("https://nikita.galaiko.rocks{}", request.url().path());
            let response: tide::Response = tide::Redirect::new(location).into();
            Ok(response)
        }
    }
}
