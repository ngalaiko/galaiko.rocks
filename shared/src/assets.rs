#[derive(Debug, Clone)]
pub struct Asset {
    pub path: std::path::PathBuf,
    pub data: Vec<u8>,
}
