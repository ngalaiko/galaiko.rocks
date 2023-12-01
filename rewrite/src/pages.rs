mod posts;

pub fn iter() -> impl Iterator<Item = (std::path::PathBuf, Result<Vec<u8>, PageError>)> {
    posts::iter().map(|(path, post)| {
        (
            std::path::PathBuf::from("posts").join(path),
            post.map_err(PageError::Post)
                .map(|(_, markup)| markup.into_string().into_bytes()),
        )
    })
}

#[derive(Debug)]
pub enum PageError {
    Post(posts::ConvertError),
}

impl std::error::Error for PageError {}

impl std::fmt::Display for PageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PageError::Post(err) => write!(f, "failed to convert post: {err}"),
        }
    }
}
