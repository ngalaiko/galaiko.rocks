use crate::assets;

mod markdown;

pub fn iter() -> impl Iterator<Item = (std::path::PathBuf, Result<Vec<u8>, PageError>)> {
    assets::iter().map(
        |(path, embeded_file)| match path.extension().and_then(|ext| ext.to_str()) {
            Some("md") => (
                path.with_extension("html"),
                markdown::Markdown::try_from(&embeded_file)
                    .map_err(PageError::Markdown)
                    .map(|md| md.body.into_string().as_bytes().to_owned()),
            ),
            _ => (path, Ok(embeded_file.data.to_vec())),
        },
    )
}

#[derive(Debug)]
pub enum PageError {
    Markdown(markdown::ConvertError),
}

impl std::error::Error for PageError {}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageError::Markdown(err) => write!(f, "failed to convert markdown: {err}"),
        }
    }
}
