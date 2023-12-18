use crate::assets;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Record {
    pub id: u64,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub basic_information: BasicInformation,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BasicInformation {
    pub title: String,
    pub artists: Vec<Artist>,
    pub thumb: String,
    pub cover_image: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Artist {
    pub name: String,
}

impl TryFrom<&assets::Asset> for Record {
    type Error = FromError;

    fn try_from(value: &assets::Asset) -> Result<Self, Self::Error> {
        serde_json::from_slice(&value.data).map_err(FromError::De)
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
