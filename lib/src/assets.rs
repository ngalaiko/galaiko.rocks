#[derive(Debug, Clone)]
pub struct Asset {
    pub path: std::path::PathBuf,
    pub mimetype: String,
    pub data: Vec<u8>,
}
