use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let state = State::build()?;
    let state = Arc::new(RwLock::new(state));

    #[cfg(debug_assertions)]
    let _watcher = watch_changes(state.clone())?;

    let mut app = tide::with_state(state);
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(serve_asset);
    app.at("/*path").get(serve_asset);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

#[cfg(debug_assertions)]
fn watch_changes(state: Arc<RwLock<State>>) -> notify::Result<notify::RecommendedWatcher> {
    use notify::Watcher;

    let mut watcher = notify::recommended_watcher(move |_| {
        *state.write().unwrap() = State::build().unwrap();
    })?;
    watcher.watch(
        std::path::Path::new("./assets"),
        notify::RecursiveMode::Recursive,
    )?;
    Ok(watcher)
}

async fn serve_asset(req: tide::Request<Arc<RwLock<State>>>) -> tide::Result {
    let requested_path = req.url().path();
    let requested_path = urlencoding::decode(requested_path)
        .map_err(|error| tide::Error::new(tide::StatusCode::BadRequest, error))?;

    let normalized_path = path::normalize(requested_path.to_string());

    let state = req.state().read().expect("Error reading state");
    if let Some(response) = state.0.get(&normalized_path) {
        tide::Response::try_from(response)
    } else {
        Ok(tide::Response::new(tide::StatusCode::NotFound))
    }
}

#[derive(Debug, Clone)]
enum Response {
    Redirect(std::path::PathBuf),
    Asset(assets::Asset),
}

impl TryFrom<&Response> for tide::Response {
    type Error = tide::Error;

    fn try_from(value: &Response) -> Result<Self, Self::Error> {
        match value {
            Response::Redirect(path) => {
                let mut response = tide::Response::new(tide::StatusCode::MovedPermanently);
                response.insert_header("Location", path.to_str().unwrap());
                Ok(response)
            }
            Response::Asset(
                assets::Asset::Post { html, .. } | assets::Asset::Page { html, .. },
            ) => {
                let html = build_page(html);
                let body = tide::Body::from(html.into_string());
                Ok(tide::Response::builder(tide::StatusCode::Ok)
                    .header("content-type", "text/html")
                    .body(body)
                    .build())
            }
            Response::Asset(assets::Asset::Other { mimetype, data }) => {
                let body = tide::Body::from(data.to_owned());
                Ok(tide::Response::builder(tide::StatusCode::Ok)
                    .header("content-type", mimetype)
                    .body(body)
                    .build())
            }
        }
    }
}

#[derive(Clone)]
struct State(Arc<HashMap<std::path::PathBuf, Response>>);

#[derive(Debug)]
pub enum BuildError {
    GetAssetError(std::path::PathBuf, assets::GetAssetError),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::GetAssetError(path, error) => {
                write!(f, "Error getting asset {}: {}", path.display(), error)
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl State {
    fn build() -> Result<Self, BuildError> {
        let assets = assets::iter()
            .map(|asset_path| {
                assets::get(&asset_path)
                    .map_err(|error| BuildError::GetAssetError(asset_path.clone(), error))
                    .map(|asset| (asset_path, asset))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut routes = HashMap::new();

        for (asset_path, asset) in &assets {
            if let assets::Asset::Post { frontmatter, .. } = &asset {
                for alias in &frontmatter.aliases {
                    routes.insert(alias.clone(), Response::Redirect(asset_path.clone()));
                }
            }
            routes.insert(asset_path.clone(), Response::Asset(asset.clone()));
        }

        routes.insert(
            std::path::PathBuf::from("/posts/index.html"),
            Response::Asset(pages::posts(
                assets
                    .into_iter()
                    .filter(|(path, _)| path.starts_with("/posts/"))
                    .filter_map(|(path, asset)| {
                        if let assets::Asset::Post { frontmatter, .. } = &asset {
                            Some((path.clone(), frontmatter.clone()))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .as_mut_slice(),
            )),
        );

        Ok(Self(Arc::new(routes)))
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
    pub fn normalize<P: AsRef<std::path::Path>>(s: P) -> std::path::PathBuf {
        let mut path = s.as_ref().to_path_buf();
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
    use std::path::PathBuf;

    use crate::markdown;

    use super::path;

    #[derive(rust_embed::RustEmbed)]
    #[folder = "assets"]
    struct Assets;

    pub fn iter() -> impl Iterator<Item = PathBuf> {
        Assets::iter().map(|asset_path| path::normalize(asset_path.to_string()))
    }

    #[derive(Debug, Clone)]
    pub enum Asset {
        Post {
            frontmatter: markdown::PostFrontmatter,
            html: maud::Markup,
        },
        Page {
            frontmatter: markdown::PageFrontmatter,
            html: maud::Markup,
        },
        Other {
            mimetype: String,
            data: Vec<u8>,
        },
    }

    pub fn get(path: &PathBuf) -> Result<Asset, GetAssetError> {
        let asset_path = Assets::iter()
            .find(|asset_path| path::normalize(asset_path.to_string()).eq(path))
            .ok_or(GetAssetError::NotFound)?;

        let embedded_file =
            Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");

        match embedded_file.metadata.mimetype() {
            "text/markdown" if path.starts_with("/posts/") => {
                let (frontmatter, md) = markdown::extract_frontmatter(&embedded_file.data);
                let frontmatter = frontmatter
                    .map_or(Err(FrontmatterError::NotFound), |frontmatter| {
                        serde_yaml::from_slice(&frontmatter).map_err(FrontmatterError::SerdeYaml)
                    })
                    .map_err(GetAssetError::Frontmatter)?;

                let html = markdown::to_html(path, &md).expect("Error parsing markdown");
                Ok(Asset::Post { frontmatter, html })
            }
            "text/markdown" => {
                let (frontmatter, md) = markdown::extract_frontmatter(&embedded_file.data);
                let frontmatter = frontmatter
                    .map_or(Err(FrontmatterError::NotFound), |frontmatter| {
                        serde_yaml::from_slice(&frontmatter).map_err(FrontmatterError::SerdeYaml)
                    })
                    .map_err(GetAssetError::Frontmatter)?;

                let html = markdown::to_html(path, &md).expect("Error parsing markdown");
                Ok(Asset::Page { frontmatter, html })
            }
            mimetype => Ok(Asset::Other {
                mimetype: mimetype.to_string(),
                data: embedded_file.data.to_vec(),
            }),
        }
    }

    #[derive(Debug)]
    pub enum FrontmatterError {
        NotFound,
        SerdeYaml(serde_yaml::Error),
    }

    impl std::fmt::Display for FrontmatterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FrontmatterError::NotFound => write!(f, "Frontmatter not found"),
                FrontmatterError::SerdeYaml(error) => write!(f, "{error}"),
            }
        }
    }

    impl std::error::Error for FrontmatterError {}

    #[derive(Debug)]
    pub enum GetAssetError {
        NotFound,
        ToHtml(markdown::ToHtmlError),
        Frontmatter(FrontmatterError),
    }

    impl std::fmt::Display for GetAssetError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GetAssetError::NotFound => {
                    write!(f, "Asset not found")
                }
                GetAssetError::ToHtml(error) => {
                    write!(f, "Error parsing markdown: {error}")
                }
                GetAssetError::Frontmatter(error) => {
                    write!(f, "Error parsing frontmatter: {error}")
                }
            }
        }
    }

    impl std::error::Error for GetAssetError {}
}

mod markdown {
    use crate::path;

    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct PostFrontmatter {
        pub title: String,
        pub date: chrono::NaiveDate,
        pub aliases: Vec<std::path::PathBuf>,
    }

    #[derive(Debug, Clone, serde::Deserialize)]
    pub struct PageFrontmatter {
        pub title: String,
    }

    /// Extracts the frontmatter from a markdown file.
    /// Returns parsed frontmatter and the remaining markdown.
    pub fn extract_frontmatter(markdown: &[u8]) -> (Option<Vec<u8>>, Vec<u8>) {
        let lines = markdown.split(|b| *b == b'\n');
        let mut frontmatter = Vec::new();
        let mut markdown = Vec::new();
        let mut in_frontmatter = false;
        for line in lines {
            if line == b"---" {
                in_frontmatter = !in_frontmatter;
            } else if in_frontmatter {
                frontmatter.extend_from_slice(line);
                frontmatter.push(b'\n');
            } else {
                markdown.extend_from_slice(line);
                markdown.push(b'\n');
            }
        }
        let frontmatter = if frontmatter.is_empty() {
            None
        } else {
            Some(frontmatter)
        };
        (frontmatter, markdown)
    }

    #[derive(Debug)]
    pub enum ToHtmlError {
        Utf8(std::str::Utf8Error),
    }

    impl std::fmt::Display for ToHtmlError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ToHtmlError::Utf8(error) => write!(f, "{error}"),
            }
        }
    }

    impl std::error::Error for ToHtmlError {}

    pub fn to_html(root: &std::path::Path, data: &[u8]) -> Result<maud::Markup, ToHtmlError> {
        let md = std::str::from_utf8(data).map_err(ToHtmlError::Utf8)?;
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

    fn fix_link(root: &std::path::Path, link: &str) -> String {
        let link = path::normalize(link);
        root.parent()
            .unwrap()
            .join(link)
            .to_string_lossy()
            .to_string()
    }
}

mod pages {
    use super::assets;
    use super::markdown;

    pub fn posts(posts: &mut [(std::path::PathBuf, markdown::PostFrontmatter)]) -> assets::Asset {
        posts.sort_by(|(_, a), (_, b)| b.date.cmp(&a.date));

        assets::Asset::Page {
            frontmatter: markdown::PageFrontmatter {
                title: "Posts".to_string(),
            },
            html: maud::html! {
                ul {
                    @for (path, post) in posts {
                        li {
                            a href=(path.display()) {
                                (post.title)
                            }
                            " "
                            time datetime=(post.date.format("%Y-%m-%d")) {
                                (post.date.format("%Y-%m-%d"))
                            }
                        }
                    }
                }
            },
        }
    }
}
