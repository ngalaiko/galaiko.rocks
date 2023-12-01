use pulldown_cmark::{Event, Tag};
use yaml_rust::YamlLoader;

pub fn iter() -> impl Iterator<
    Item = (
        std::path::PathBuf,
        Result<(Frontmatter, maud::Markup), ConvertError>,
    ),
> {
    Posts::iter().map(|asset| {
        let path = std::path::PathBuf::from(asset.as_ref());
        let path = if let Some("index.md") = path.file_name().and_then(|name| name.to_str()) {
            // If the file is named `index.md`, then we want to use the parent directory as the
            // path.
            path.parent().unwrap().to_path_buf()
        } else {
            // Otherwise, we want to use the file name as the path.
            path
        }
        .with_extension("html");

        (
            path,
            convert_post(&Posts::get(asset.as_ref()).expect("always found")),
        )
    })
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets/pages/posts"]
#[include = "**/*.md"]
struct Posts;

fn convert_post(
    asset: &rust_embed::EmbeddedFile,
) -> Result<(Frontmatter, maud::Markup), ConvertError> {
    let md = std::str::from_utf8(asset.data.as_ref()).map_err(ConvertError::Utf8)?;

    let parser = pulldown_cmark::Parser::new(md);
    let (frontmatter, body) = extract_frontmatter(parser).map_err(ConvertError::Frontmatter)?;

    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, body.into_iter());

    Ok((frontmatter, maud::PreEscaped(html)))
}

pub struct Frontmatter {
    /// Post title.
    pub title: String,
    /// Publication date.
    pub date: chrono::DateTime<chrono::FixedOffset>,
    /// Whether the post is a draft.
    pub is_draft: bool,
    /// Alternative paths to the post.
    pub aliases: Vec<std::path::PathBuf>,
}

fn extract_frontmatter<'a>(
    iter: impl Iterator<Item = Event<'a>>,
) -> Result<(Frontmatter, Vec<Event<'a>>), FrontmatterError> {
    let is_frontmatter_start = |event: &Event| -> bool {
        matches!(
            event,
            Event::Start(Tag::Heading(
                pulldown_cmark::HeadingLevel::H2,
                None,
                classes
            )) if classes.is_empty()
        )
    };
    let is_frontmatter_end = |event: &Event| -> bool {
        matches!(
            event,
            Event::End(Tag::Heading(
                pulldown_cmark::HeadingLevel::H2,
                None,
                classes
            )) if classes.is_empty()
        )
    };

    let mut is_frontmatter = false;
    let mut frontmatter_found = false;
    let mut raw_frontmatter = String::new();
    let parser = iter
        .skip_while(|event| !is_frontmatter_start(event))
        .filter_map(|event| {
            if frontmatter_found {
                Some(event)
            } else if is_frontmatter_start(&event) {
                is_frontmatter = true;
                None
            } else if is_frontmatter_end(&event) {
                is_frontmatter = false;
                frontmatter_found = true;
                None
            } else if is_frontmatter {
                match event {
                    Event::Text(text) => {
                        raw_frontmatter.push_str(&text);
                        None
                    }
                    Event::SoftBreak => {
                        raw_frontmatter.push('\n');
                        None
                    }
                    _ => panic!("unexpected event in frontmatter"),
                }
            } else {
                Some(event)
            }
        })
        .collect::<Vec<_>>();

    let parsed_frontmatter =
        YamlLoader::load_from_str(&raw_frontmatter).map_err(FrontmatterError::ParseYaml)?;
    let title = parsed_frontmatter[0]["title"]
        .as_str()
        .ok_or(FrontmatterError::TitleMissing)?
        .to_owned();
    let date = parsed_frontmatter[0]["date"]
        .as_str()
        .ok_or(FrontmatterError::TitleMissing)
        .and_then(|date| {
            chrono::DateTime::parse_from_rfc3339(date).map_err(FrontmatterError::InvalidDate)
        })?;
    let is_draft = parsed_frontmatter[0]["draft"].as_bool().unwrap_or(false);
    let aliases = parsed_frontmatter[0]["aliases"]
        .as_vec()
        .map(|aliases| {
            aliases
                .iter()
                .filter_map(|alias| alias.as_str().map(std::path::PathBuf::from))
                .collect()
        })
        .unwrap_or_default();
    Ok((
        Frontmatter {
            title,
            date,
            is_draft,
            aliases,
        },
        parser,
    ))
}

#[derive(Debug)]
pub enum FrontmatterError {
    ParseYaml(yaml_rust::ScanError),
    TitleMissing,
    InvalidDate(chrono::ParseError),
}

impl std::error::Error for FrontmatterError {}

impl std::fmt::Display for FrontmatterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FrontmatterError::ParseYaml(err) => write!(f, "{err}"),
            FrontmatterError::TitleMissing => write!(f, "title missing"),
            FrontmatterError::InvalidDate(err) => write!(f, "failed to parse date: {err}"),
        }
    }
}

#[derive(Debug)]
pub enum ConvertError {
    Frontmatter(FrontmatterError),
    Utf8(std::str::Utf8Error),
}

impl std::error::Error for ConvertError {}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::Frontmatter(err) => write!(f, "{err}"),
            ConvertError::Utf8(err) => write!(f, "{err}"),
        }
    }
}
