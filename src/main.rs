use warp::{filters::path::FullPath, reject, Filter, Rejection, Reply};

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
    let requested_path = urlencoding::decode(requested_path.as_str()).map_err(|error| {
        log::error!("URL decoding error: {}", error);
        reject::not_found()
    })?;

    let path = path::normalize(&requested_path);
    if path == path::normalize("/posts/index.html") {
        let page = build_page(&pages::posts());
        let response = warp::reply::Response::new(page.into_string().into());
        let reply = warp::reply::with_header(response, "content-type", "text/html; charset=utf-8");
        return Ok(reply);
    }

    let asset = assets::get(&path).map_err(|error| match error {
        assets::GetAssetError::NotFound(_) => reject::not_found(),
        assets::GetAssetError::Utf8Error(error) => {
            log::error!("UTF-8 error: {}", error);
            reject::not_found()
        }
    })?;

    match asset.content {
        assets::Content::Html(content) => {
            let page = build_page(&content);
            let response = warp::reply::Response::new(page.into_string().into());
            let reply =
                warp::reply::with_header(response, "content-type", "text/html; charset=utf-8");
            Ok(reply)
        }
        assets::Content::Binary { content_type, body } => {
            let response = warp::reply::Response::new(body.into());
            let reply = warp::reply::with_header(response, "content-type", content_type);
            Ok(reply)
        }
    }
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

mod path {
    pub fn normalize(s: &str) -> std::path::PathBuf {
        let mut path = std::path::PathBuf::from(s);
        if !path.has_root() {
            path = std::path::PathBuf::from("/").join(path);
        };
        if path.extension().is_none() {
            path.push("index.html");
        };
        match path.extension().and_then(|e| e.to_str()) {
            Some("md") => {
                path.set_extension("html");
            }
            Some(_) => {}
            None => {
                path.push("index.html");
            }
        }
        path
    }

    #[test]
    fn root() {
        let path = normalize("/");
        assert_eq!(path, std::path::PathBuf::from("/index.html"));
    }

    #[test]
    fn empty() {
        let path = normalize("");
        assert_eq!(path, std::path::PathBuf::from("/index.html"));
    }

    #[test]
    fn directory_with_slash() {
        let path = normalize("/posts/");
        assert_eq!(path, std::path::PathBuf::from("/posts/index.html"));
    }

    #[test]
    fn directory_without_slash() {
        let path = normalize("/posts");
        assert_eq!(path, std::path::PathBuf::from("/posts/index.html"));
    }

    #[test]
    fn md_root_file() {
        let path = normalize("/index.md");
        assert_eq!(path, std::path::PathBuf::from("/index.html"));
    }

    #[test]
    fn md_directory_file() {
        let path = normalize("/posts/some title/index.md");
        assert_eq!(
            path,
            std::path::PathBuf::from("/posts/some title/index.html")
        );
    }
}

mod assets {
    use std::path::{Path, PathBuf};

    use super::path;

    #[derive(rust_embed::RustEmbed)]
    #[folder = "assets"]
    struct Assets;

    pub fn iter() -> impl Iterator<Item = PathBuf> {
        Assets::iter().map(|asset_path| path::normalize(&asset_path))
    }

    pub fn get(path: &PathBuf) -> Result<Asset, GetAssetError> {
        let asset_path = Assets::iter()
            .find(|asset_path| path::normalize(asset_path).eq(path))
            .ok_or_else(|| GetAssetError::NotFound(path.clone()))?;

        let embedded_file =
            Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");

        let title = if asset_path.ends_with("index.md") {
            path.parent()
        } else {
            Some(path.as_path())
        }
        .and_then(|p| p.file_stem())
        .and_then(|s| s.to_str())
        .map(ToString::to_string);

        let asset = Asset {
            title,
            path: path.clone(),
            created: embedded_file.metadata.created().map_or(
                chrono::NaiveDate::from_ymd_opt(1970, 1, 1).expect("valid date"),
                |created_ts| {
                    chrono::naive::NaiveDateTime::from_timestamp_opt(
                        created_ts.try_into().unwrap(),
                        0,
                    )
                    .expect("valid timestamp")
                    .date()
                },
            ),
            content: if asset_path.ends_with("md") {
                let html = convert_md(path.parent().expect("expected parent"), &embedded_file.data)
                    .map_err(GetAssetError::Utf8Error)?;
                Content::Html(html)
            } else {
                Content::Binary {
                    content_type: embedded_file.metadata.mimetype().to_string(),
                    body: embedded_file.data.to_vec(),
                }
            },
        };

        Ok(asset)
    }

    #[derive(Debug)]
    pub enum GetAssetError {
        NotFound(PathBuf),
        Utf8Error(std::str::Utf8Error),
    }

    impl std::fmt::Display for GetAssetError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GetAssetError::NotFound(path) => {
                    write!(f, "Asset not found: {}", path.to_str().unwrap())
                }
                GetAssetError::Utf8Error(err) => write!(f, "UTF-8 error: {err}"),
            }
        }
    }

    impl std::error::Error for GetAssetError {}

    #[derive(Debug)]
    pub struct Asset {
        pub title: Option<String>,
        pub path: PathBuf,
        pub content: Content,
        pub created: chrono::NaiveDate,
    }

    #[derive(Debug)]
    pub enum Content {
        Html(maud::Markup),
        Binary { content_type: String, body: Vec<u8> },
    }

    fn fix_link(root: &Path, link: &str) -> String {
        root.join(link).display().to_string()
    }

    fn convert_md(root: &Path, data: &[u8]) -> Result<maud::Markup, std::str::Utf8Error> {
        let md = std::str::from_utf8(data)?;
        let parser = pulldown_cmark::Parser::new(md);
        let parser = parser.map(|event| match event {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(typ, link, title)) => {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(
                    typ,
                    pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                    title,
                ))
            }
            pulldown_cmark::Event::End(pulldown_cmark::Tag::Image(typ, link, title)) => {
                pulldown_cmark::Event::End(pulldown_cmark::Tag::Image(
                    typ,
                    pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                    title,
                ))
            }
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(typ, link, title)) => {
                pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(
                    typ,
                    pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                    title,
                ))
            }
            pulldown_cmark::Event::End(pulldown_cmark::Tag::Link(typ, link, title)) => {
                pulldown_cmark::Event::End(pulldown_cmark::Tag::Link(
                    typ,
                    pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                    title,
                ))
            }
            _ => event,
        });
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        Ok(maud::PreEscaped(html))
    }
}

mod pages {
    use super::assets;

    pub fn posts() -> maud::Markup {
        let mut posts = assets::iter()
            .filter(|asset_path| asset_path.starts_with("/posts"))
            .filter(|asset_path| asset_path.extension().and_then(|e| e.to_str()) == Some("html"))
            .filter_map(|asset_path| assets::get(&asset_path).ok())
            .collect::<Vec<_>>();
        posts.sort_by(|a, b| b.created.cmp(&a.created));
        maud::html! {
            h1 {
                "Posts"
            }
            ul {
                @for post in posts {
                    li {
                        a href=(post.path.display()) {
                            (post.title.unwrap_or_else(|| "Untitled".to_string()))
                        }
                        " "
                        time datetime=(post.created.format("%Y-%m-%d")) {
                            (post.created.format("%Y-%m-%d"))
                        }
                    }
                }
            }
        }
    }
}
