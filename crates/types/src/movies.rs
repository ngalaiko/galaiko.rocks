#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Entry {
    pub title: String,
    pub title_slug: String,
    pub date: chrono::NaiveDate,
    pub is_rewatch: bool,
    pub is_liked: bool,
    pub href: String,
}

impl TryFrom<&[u8]> for Entry {
    type Error = FromError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(value).map_err(FromError::De)
    }
}

#[derive(Debug)]
pub enum FromError {
    De(serde_json::Error),
}

impl std::fmt::Display for FromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromError::De(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FromError {}
