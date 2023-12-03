pub struct Markdown {
    pub body: maud::Markup,
}

impl TryFrom<&rust_embed::EmbeddedFile> for Markdown {
    type Error = ConvertError;

    fn try_from(raw: &rust_embed::EmbeddedFile) -> Result<Self, Self::Error> {
        let md = std::str::from_utf8(&raw.data).map_err(ConvertError::Utf8)?;

        let parser = pulldown_cmark::Parser::new(md);

        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        Ok(Self {
            body: maud::PreEscaped(html),
        })
    }
}

#[derive(Debug)]
pub enum ConvertError {
    Utf8(std::str::Utf8Error),
}

impl std::error::Error for ConvertError {}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::Utf8(err) => write!(f, "{err}"),
        }
    }
}
