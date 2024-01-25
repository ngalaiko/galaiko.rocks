use crate::{assets, parse, path};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Frontmatter {
    pub title: String,
    /// date of publication
    pub date: Option<chrono::NaiveDate>,
    /// other paths to this entry (if it was moved, etc.)
    #[serde(default)]
    pub aliases: Vec<std::path::PathBuf>,
    /// unique identifier for this entry to use, for example, in atom feed.
    pub id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub path: std::path::PathBuf,
    pub frontmatter: Frontmatter,
    pub body: maud::Markup,
}

impl TryFrom<&assets::Asset> for Entry {
    type Error = FromError;

    fn try_from(asset: &assets::Asset) -> Result<Self, Self::Error> {
        let (frontmatter, body) = parse::markdown(&asset.data).map_err(FromError::Parse)?;
        let frontmatter = frontmatter.ok_or(FromError::FrontmatterNotFound)?;
        Ok(Entry {
            path: path::normalize(&asset.path),
            frontmatter,
            body,
        })
    }
}

#[derive(Debug)]
pub enum FromError {
    FrontmatterNotFound,
    Parse(parse::MarkdownParseError),
}

impl std::fmt::Display for FromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromError::FrontmatterNotFound => write!(f, "frontmatter not found"),
            FromError::Parse(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FromError {}
