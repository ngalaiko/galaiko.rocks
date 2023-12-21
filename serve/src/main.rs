use clap::Parser;

use lib::path;

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

pub async fn serve(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
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
    let asset_path = Public::iter()
        .find(|asset_path| path::normalize(asset_path.to_string()) == normalized_path);

    let response =
        if let Some(embedded_file) = asset_path.and_then(|asset_path| Public::get(&asset_path)) {
            let data = embedded_file.data.to_vec();
            if let Some(location) = data.strip_prefix(b"redirect: ") {
                let location = std::str::from_utf8(location)?;
                tide::Redirect::new(location).into()
            } else {
                let sha256_hash = embedded_file.metadata.sha256_hash();
                tide::Response::builder(tide::StatusCode::Ok)
                    .header("content-type", embedded_file.metadata.mimetype())
                    .header(
                        "etag",
                        format!("\"{:x}-{:x}\"", sha256_hash[0], sha256_hash[31]),
                    )
                    .header("cache-control", "public, max-age=31536000, immutable")
                    .header(
                        "last-modified",
                        embedded_file
                            .metadata
                            .last_modified()
                            .unwrap_or(0)
                            .to_string(),
                    )
                    .body(tide::Body::from(embedded_file.data.to_vec()))
                    .build()
            }
        } else {
            tide::Response::new(tide::StatusCode::NotFound)
        };

    Ok(response)
}
