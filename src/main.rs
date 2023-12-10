use std::{collections::HashMap, sync::Arc};

use self::path::normalize;

#[async_std::main]
async fn main() -> tide::Result<()> {
    femme::start();

    let mut app = tide::with_state(State::build()?);
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(serve_asset);
    app.at("/*path").get(serve_asset);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn serve_asset(req: tide::Request<State>) -> tide::Result {
    let requested_path = req.url().path();
    let requested_path = urlencoding::decode(requested_path)
        .map_err(|error| tide::Error::new(tide::StatusCode::BadRequest, error))?;

    let normalized_path = normalize(requested_path.to_string());

    if let Some(response) = req.state().0.get(&normalized_path) {
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
                assets::Asset::Markdown { html } | assets::Asset::Generated { html },
            ) => {
                let html = build_page(html);
                let body = tide::Body::from(html.clone().into_string());
                Ok(tide::Response::builder(tide::StatusCode::Ok)
                    .header("content-type", "text/html")
                    .body(body)
                    .build())
            }
            Response::Asset(assets::Asset::Post(post)) => {
                let html = build_page(&post.html);
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

        for (asset_path, asset) in assets {
            if let assets::Asset::Post(post) = &asset {
                for alias in &post.frontmatter.aliases {
                    routes.insert(
                        path::normalize(alias),
                        Response::Redirect(asset_path.clone()),
                    );
                }
            }
            routes.insert(asset_path, Response::Asset(asset));
        }

        routes.insert(
            std::path::PathBuf::from("/posts/index.html"),
            Response::Asset(pages::posts(
                &mut routes
                    .iter()
                    .filter_map(|(path, asset)| match asset {
                        Response::Asset(assets::Asset::Post(post)) => {
                            Some((path.clone(), post.clone()))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
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
    pub struct Post {
        pub frontmatter: markdown::Frontmatter,
        pub html: maud::Markup,
    }

    #[derive(Debug, Clone)]
    pub enum Asset {
        Post(Post),
        Markdown { html: maud::Markup },
        Generated { html: maud::Markup },
        Other { mimetype: String, data: Vec<u8> },
    }

    pub fn get(path: &PathBuf) -> Result<Asset, GetAssetError> {
        let asset_path = Assets::iter()
            .find(|asset_path| path::normalize(asset_path.to_string()).eq(path))
            .ok_or(GetAssetError::NotFound)?;

        let embedded_file =
            Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");

        match embedded_file.metadata.mimetype() {
            "text/markdown" if path.starts_with("/posts") => {
                let (frontmatter, md) = markdown::extract_frontmatter(&embedded_file.data)
                    .map_err(GetAssetError::Frontmatter)?;
                let html = markdown::to_html(path, &md).expect("Error parsing markdown");
                Ok(Asset::Post(Post { frontmatter, html }))
            }
            "text/markdown" => {
                let md = embedded_file.data.to_vec();
                let html = markdown::to_html(path, &md).map_err(GetAssetError::ToHtml)?;
                Ok(Asset::Markdown { html })
            }
            mimetype => Ok(Asset::Other {
                mimetype: mimetype.to_string(),
                data: embedded_file.data.to_vec(),
            }),
        }
    }

    #[derive(Debug)]
    pub enum GetAssetError {
        NotFound,
        ToHtml(markdown::ToHtmlError),
        Frontmatter(markdown::FrontmatterError),
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
    pub struct Frontmatter {
        pub title: String,
        pub date: chrono::NaiveDate,
        pub aliases: Vec<std::path::PathBuf>,
    }

    #[derive(Debug)]
    pub enum FrontmatterError {
        SerdeYaml(serde_yaml::Error),
    }

    impl std::fmt::Display for FrontmatterError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FrontmatterError::SerdeYaml(error) => write!(f, "{error}"),
            }
        }
    }

    impl std::error::Error for FrontmatterError {}

    /// Extracts the frontmatter from a markdown file.
    /// Returns parsed frontmatter and the remaining markdown.
    pub fn extract_frontmatter(
        markdown: &[u8],
    ) -> Result<(Frontmatter, Vec<u8>), FrontmatterError> {
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
        let frontmatter =
            serde_yaml::from_slice(&frontmatter).map_err(FrontmatterError::SerdeYaml)?;
        Ok((frontmatter, markdown))
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

    pub fn posts(posts: &mut [(std::path::PathBuf, assets::Post)]) -> assets::Asset {
        posts.sort_by(|(_, a), (_, b)| b.frontmatter.date.cmp(&a.frontmatter.date));

        assets::Asset::Generated {
            html: maud::html! {
                h1 {
                    "Posts"
                }
                ul {
                    @for (path, post) in posts {
                        li {
                            a href=(path.display()) {
                                (post.frontmatter.title)
                            }
                            " "
                            time datetime=(post.frontmatter.date.format("%Y-%m-%d")) {
                                (post.frontmatter.date.format("%Y-%m-%d"))
                            }
                        }
                    }
                }
            },
        }
    }
}
