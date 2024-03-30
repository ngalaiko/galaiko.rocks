use crate::parse;

#[derive(Debug, serde::Deserialize)]
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

#[derive(Debug)]
pub struct Entry<'a> {
    pub frontmatter: Frontmatter,
    pub body: Vec<pulldown_cmark::Event<'a>>,
}

impl<'a> TryFrom<&'a [u8]> for Entry<'a> {
    type Error = FromError;

    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        let (frontmatter, md) = parse::frontmatter::parse(data).map_err(FromError::Frontmatter)?;
        let body = parse::markdown::parse(md).map_err(FromError::Body)?;
        Ok(Entry { frontmatter, body })
    }
}

#[derive(Debug)]
pub enum FromError {
    Frontmatter(parse::frontmatter::ParseError),
    Body(parse::markdown::ParseError),
}

impl std::fmt::Display for FromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromError::Frontmatter(error) => write!(f, "{error}"),
            FromError::Body(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FromError {}
