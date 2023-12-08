use warp::{filters::path::FullPath, reject, reply, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "www=info");
    env_logger::init();

    // TODO: get only.
    let routes = warp::path::full()
        .and_then(serve_asset)
        .with(warp::log("www"));

    println!("listening on http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn serve_asset(requested_path: FullPath) -> Result<impl Reply, Rejection> {
    find_asset(requested_path.as_str())
        .map(|(content_type, body)| reply::with_header(body, "content-type", content_type))
        .ok_or_else(reject::not_found)
}

fn posts_index_page() -> maud::Markup {
    let mut posts = Assets::iter()
        .filter(|asset_path| asset_path.starts_with("posts/"))
        .filter(|asset_path| asset_path.ends_with(".md"))
        .map(|asset_path| {
            let path = std::path::Path::new(asset_path.as_ref());
            (
                if let Some("index.md") = path.file_name().and_then(|s| s.to_str()) {
                    path.parent().unwrap().file_stem().unwrap().to_str().unwrap().to_string()
                } else {
                    path.file_stem().unwrap().to_str().unwrap().to_string()
                },
                format!("/{}", cleanup_asset_path(&asset_path)),
                Assets::get(&asset_path)
                    .expect("always found")
                    .metadata
                    .created()
                    .map_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).expect("valid date"), |created_ts| {
                        chrono::naive::NaiveDateTime::from_timestamp_opt(
                            created_ts.try_into().unwrap(),
                            0,
                        )
                        .expect("valid timestamp")
                        .date()
                    }),
            )
        }).collect::<Vec<_>>();
    posts.sort_by(|(_,_, a), (_,_, b)| b.cmp(a));
    maud::html! {
        h1 {
            "Posts"
        }
        ul {
            @for (title, path, created) in posts {
                li {
                    a href=(path) {
                        (title)
                    }
                    " "
                    span class="created" {
                        (created.format("%Y-%m-%d"))
                    }
                }
            }
        }
    }
}

fn find_asset(request_path: &str) -> Option<(String, Vec<u8>)> {
    let clean_request_path = cleanup_requested_path(request_path);
    match clean_request_path.as_str() {
        "posts/index.html" => Some((
            "text/html; charset=utf-8".to_string(),
            build_page(&posts_index_page()).into_string().into_bytes(),
        )),
        _ => Assets::iter()
            .find(|asset_path| {
                let clean_asset_path = cleanup_asset_path(asset_path);
                clean_asset_path == clean_request_path
            })
            .and_then(|asset_path| Asset::load(asset_path.as_ref()))
            .and_then(|asset| match asset {
                Ok(Asset::Html(content)) => Some((
                    "text/html; charset=utf-8".to_string(),
                    build_page(&content).into_string().into_bytes(),
                )),
                Ok(Asset::Binary { content_type, body }) => Some((content_type, body)),
                Err(_) => None,
            }),
    }
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

impl Asset {
    fn load(path: &str) -> Option<Result<Asset, std::str::Utf8Error>> {
        Assets::get(path).map(|asset| {
            match std::path::Path::new(path)
                .extension()
                .and_then(|s| s.to_str())
            {
                Some("md") => convert_md(&asset.data).map(Asset::Html),
                _ => Ok(Asset::Binary {
                    body: asset.data.to_vec(),
                    content_type: asset.metadata.mimetype().to_string(),
                }),
            }
        })
    }
}

enum Asset {
    Html(maud::Markup),
    Binary { content_type: String, body: Vec<u8> },
}

fn convert_md(data: &[u8]) -> Result<maud::Markup, std::str::Utf8Error> {
    let md = std::str::from_utf8(data)?;
    let parser = pulldown_cmark::Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    Ok(maud::PreEscaped(html))
}

fn build_page(content: &maud::Markup) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="/index.css";
        }
        main {
            article {
                (content)
            }
        }
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
