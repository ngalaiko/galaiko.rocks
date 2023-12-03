use crate::assets;

mod markdown;
mod posts;

pub fn iter() -> impl Iterator<Item = (std::path::PathBuf, Result<Vec<u8>, PageError>)> {
    assets::iter().map(|(path, data)| match path {
        path if is_post_page(&path) => (
            path.with_extension("html"),
            posts::Post::try_from(data.as_ref())
                .map_err(PageError::Post)
                .map(|post| post.body.into_string().as_bytes().to_owned()),
        ),
        path if is_markdown_page(&path) => (
            path.with_extension("html"),
            markdown::Markdown::try_from(data.as_ref())
                .map_err(PageError::Markdown)
                .map(|md| md.body.into_string().as_bytes().to_owned()),
        ),
        path => (path, Ok(data)),
    })
}

fn is_markdown_page(path: &std::path::Path) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("md"))
}

fn is_post_page(path: &std::path::Path) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("md")) && path.starts_with("posts/")
}

#[derive(Debug)]
pub enum PageError {
    Post(posts::ConvertError),
    Markdown(markdown::ConvertError),
}

impl std::error::Error for PageError {}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageError::Post(err) => write!(f, "failed to convert post: {err}"),
            PageError::Markdown(err) => write!(f, "failed to convert markdown: {err}"),
        }
    }
}
