use crate::{assets, parse};

#[derive(serde::Deserialize)]
pub struct Frontmatter {
    pub title: String,
}

pub struct Page {
    pub path: std::path::PathBuf,
    pub frontmatter: Frontmatter,
    pub body: maud::Markup,
}

impl TryFrom<&assets::Asset> for Page {
    type Error = FromError;

    fn try_from(asset: &assets::Asset) -> Result<Self, Self::Error> {
        let (frontmatter, body) = parse::markdown(&asset.data).map_err(FromError::Parse)?;
        let frontmatter = frontmatter.ok_or(FromError::FrontmatterNotFound)?;
        Ok(Page {
            path: asset.path.clone(),
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
