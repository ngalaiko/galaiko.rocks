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

    if let Some(embedded_file) = asset_path.and_then(|asset_path| Public::get(&asset_path)) {
        if let Some("redirect") = normalized_path.extension().and_then(|ext| ext.to_str()) {
            let location = std::str::from_utf8(&embedded_file.data)
                .map_err(|error| tide::Error::new(tide::StatusCode::InternalServerError, error))?;
            Ok(tide::Response::builder(tide::StatusCode::MovedPermanently)
                .header("location", location)
                .build())
        } else {
            Ok(tide::Response::builder(tide::StatusCode::Ok)
                .header("content-type", embedded_file.metadata.mimetype())
                .body(tide::Body::from(embedded_file.data.to_vec()))
                .build())
        }
    } else {
        Ok(tide::Response::new(tide::StatusCode::NotFound))
    }
}
