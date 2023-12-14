use crate::{assets, markdown};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Frontmatter {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub aliases: Vec<std::path::PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Post {
    pub path: std::path::PathBuf,
    pub frontmatter: Frontmatter,
    pub body: maud::Markup,
}

#[derive(Debug)]
pub enum FromError {
    FrontmatterNotFound,
    Frontmatter(serde_yaml::Error),
    Body(markdown::ToHtmlError),
}

impl std::fmt::Display for FromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromError::FrontmatterNotFound => write!(f, "Frontmatter not found"),
            FromError::Frontmatter(error) => write!(f, "{error}"),
            FromError::Body(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FromError {}

impl TryFrom<&assets::Asset> for Post {
    type Error = FromError;

    fn try_from(asset: &assets::Asset) -> Result<Self, Self::Error> {
        let (frontmatter, md) = markdown::extract_frontmatter(&asset.data);
        let frontmatter = frontmatter.ok_or(FromError::FrontmatterNotFound)?;
        let frontmatter = serde_yaml::from_slice(&frontmatter).map_err(FromError::Frontmatter)?;
        let body = markdown::to_html(&md).map_err(FromError::Body)?;
        Ok(Post {
            path: asset.path.clone(),
            frontmatter,
            body,
        })
    }
}
