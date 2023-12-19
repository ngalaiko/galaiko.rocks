use crate::path;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

#[derive(Debug, Clone)]
pub struct Asset {
    pub path: std::path::PathBuf,
    pub mimetype: String,
    pub data: Vec<u8>,
}

pub fn iter() -> impl Iterator<Item = Asset> {
    Assets::iter().map(|asset_path| {
        let embedded_file =
            Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");
        Asset {
            path: path::normalize(asset_path.to_string()),
            mimetype: embedded_file.metadata.mimetype().to_string(),
            data: embedded_file.data.to_vec(),
        }
    })
}
