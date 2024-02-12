use crate::{assets, parse, path};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Frontmatter {
    pub title: String,
}

#[derive(Debug)]
pub struct Cocktail {
    pub path: std::path::PathBuf,
    pub frontmatter: Frontmatter,
    pub recipe: parse::cooklang::Recipe,
}

impl TryFrom<&assets::Asset> for Cocktail {
    type Error = FromError;

    fn try_from(asset: &assets::Asset) -> Result<Self, Self::Error> {
        let title = asset
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(std::borrow::ToOwned::to_owned)
            .ok_or(FromError::NoTitle)?;
        let recipe = parse::cooklang::parse(&asset.data).map_err(FromError::Cooklang)?;
        Ok(Cocktail {
            path: path::normalize(&asset.path),
            recipe,
            frontmatter: Frontmatter { title },
        })
    }
}

#[derive(Debug)]
pub enum FromError {
    NoTitle,
    Cooklang(parse::cooklang::ParseError),
}

impl std::fmt::Display for FromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromError::NoTitle => write!(f, "No title"),
            FromError::Cooklang(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FromError {}
