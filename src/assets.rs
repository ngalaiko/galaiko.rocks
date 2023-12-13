use crate::path;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn iter() -> impl Iterator<Item = std::path::PathBuf> {
    Assets::iter().map(|asset_path| path::normalize(asset_path.to_string()))
}

#[derive(Debug, Clone)]
pub struct Asset {
    pub path: std::path::PathBuf,
    pub mimetype: String,
    pub data: Vec<u8>,
}

pub fn get(path: &std::path::PathBuf) -> Result<Asset, GetAssetError> {
    let asset_path = Assets::iter()
        .find(|asset_path| path::normalize(asset_path.to_string()).eq(path))
        .ok_or(GetAssetError::NotFound)?;

    let embedded_file =
        Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");

    Ok(Asset {
        path: path.clone(),
        mimetype: embedded_file.metadata.mimetype().to_string(),
        data: embedded_file.data.to_vec(),
    })
}

#[derive(Debug)]
pub enum GetAssetError {
    NotFound,
}

impl std::fmt::Display for GetAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetAssetError::NotFound => {
                write!(f, "Asset not found")
            }
        }
    }
}

impl std::error::Error for GetAssetError {}
