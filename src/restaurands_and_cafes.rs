#[derive(Debug, serde::Serialize)]
pub struct Place {
    pub name: String,
    pub spent: f64,
    pub times: u8,
}
