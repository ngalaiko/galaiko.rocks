use crate::{assets, cooklang};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Frontmatter {
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct Cocktail {
    pub path: std::path::PathBuf,
    pub frontmatter: Frontmatter,
    pub body: maud::Markup,
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
        cooklang::parse(&asset.data)
            .map(|recipe| Cocktail {
                body: cooklang::to_html(&recipe),
                frontmatter: Frontmatter { title },
                path: asset.path.clone(),
            })
            .map_err(FromError::Cooklang)
    }
}

#[derive(Debug)]
pub enum FromError {
    NoTitle,
    Cooklang(cooklang::ParseError),
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
