use crate::assets;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Place {
    pub name: String,
    pub spent: f64,
    pub times: u8,
}

impl TryFrom<&assets::Asset> for Place {
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
