use warp::{filters::path::FullPath, reject, reply, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "www=info");
    env_logger::init();

    // TODO: get only.
    let routes = warp::path::full().and_then(serve_asset)
        .with(warp::log("www"));

    println!("listening on http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn serve_asset(requested_path: FullPath) -> Result<impl Reply, Rejection> {
    find_asset(requested_path.as_str())
        .and_then(|asset| {
            asset
                .content
                .to_vec()
                .ok()
                .map(|content| (asset.content_type, content))
        })
        .map(|(content_type, content)| reply::with_header(content, "content-type", content_type))
        .ok_or_else(reject::not_found)
}

fn find_asset(request_path: &str) -> Option<Asset> {
    let clean_request_path = cleanup_requested_path(request_path);
    Assets::iter()
        .find(|asset_path| {
            let clean_asset_path = cleanup_asset_path(asset_path);
            clean_asset_path == clean_request_path
        })
        .and_then(|asset_path| Asset::load(asset_path.as_ref()))
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

struct Asset {
    content: AssetContent,
    content_type: String,
}

impl Asset {
    fn load(path: &str) -> Option<Asset> {
        Assets::get(path).map(|asset| {
            match std::path::Path::new(path)
                .extension()
                .and_then(|s| s.to_str())
            {
                Some("md") => Asset {
                    content_type: "text/html".to_string(),
                    content: AssetContent::Markdown(asset.data.to_vec()),
                },
                _ => Asset {
                    content: AssetContent::Binary(asset.data.to_vec()),
                    content_type: asset.metadata.mimetype().to_string(),
                },
            }
        })
    }
}

enum AssetContent {
    Markdown(Vec<u8>),
    Binary(Vec<u8>),
}

impl AssetContent {
    fn to_vec(&self) -> Result<Vec<u8>, std::str::Utf8Error> {
        match self {
            AssetContent::Markdown(data) => convert_md(data),
            AssetContent::Binary(data) => Ok(data.to_owned()),
        }
    }
}

fn convert_md(data: &[u8]) -> Result<Vec<u8>, std::str::Utf8Error> {
    let md = std::str::from_utf8(data)?;
    let parser = pulldown_cmark::Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let doc = build_page(&maud::PreEscaped(html));

    Ok(doc.into_string().as_bytes().to_owned())
}

fn build_page(content: &maud::Markup) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="/index.css";
        }
        (content)
    }
}

fn cleanup_asset_path(path: &str) -> String {
    if path.is_empty() {
        return "index.html".to_string();
    }

    let path = std::path::PathBuf::from(path);
    let path = match (path.parent(), path.file_name().and_then(|s| s.to_str())) {
        (Some(parent), Some("index.md")) if !parent.as_os_str().is_empty() => {
            parent.with_extension("md")
        }
        _ => path,
    };

    let extension = path.extension();

    match extension.and_then(|s| s.to_str()) {
        Some("md") => path
            .with_extension("")
            .components()
            .map(|c| slug::slugify(c.as_os_str().to_str().unwrap_or_default()))
            .map(std::path::PathBuf::from)
            .fold(std::path::PathBuf::new(), |acc, p| acc.join(p))
            .with_extension("html"),
        Some(ext) => path.with_extension(ext),
        _ => path,
    }
    .to_str()
    .unwrap_or_default()
    .to_string()
}

fn cleanup_requested_path(path: &str) -> String {
    let path = path.strip_prefix('/').unwrap_or(path);
    if path.is_empty() {
        return "index.html".to_string();
    }

    let path = std::path::PathBuf::from(path);
    if path.extension().is_none() {
        let path = path.to_str().unwrap_or_default();
        if path.ends_with('/') {
            path.to_string() + "index.html"
        } else {
            path.to_string() + ".html"
        }
    } else {
        path.to_str().unwrap_or_default().to_string()
    }
}

#[test]
fn test_cleanup_requested_path() {
    for (from, expected) in [
        ("/", "index.html"),
        ("/index.html", "index.html"),
        ("android-chrome-192x192.png", "android-chrome-192x192.png"),
        ("/favicon.ico", "favicon.ico"),
        ("/posts/index.html", "posts/index.html"),
        ("/posts/", "posts/index.html"),
        ("/now", "now.html"),
    ] {
        assert_eq!(cleanup_requested_path(from), expected);
    }
}

#[test]
fn test_cleanup_asset_path() {
    for (from, expected) in [
        ("", "index.html"),
        ("index.html", "index.html"),
        ("Clean up.md", "clean-up.html"),
        ("Font.woff", "Font.woff"),
        ("index.md", "index.html"),
        ("android-chrome-192x192.png", "android-chrome-192x192.png"),
        ("favicon.ico", "favicon.ico"),
        ("posts/index.html", "posts/index.html"),
    ] {
        assert_eq!(cleanup_asset_path(from), expected);
    }
}
