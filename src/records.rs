#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    pub id: u64,
    pub date_added: chrono::DateTime<chrono::Utc>,
    pub basic_information: BasicInformation,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BasicInformation {
    pub title: String,
    pub artists: Vec<Artist>,
    pub thumb: String,
    pub cover_image: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Artist {
    pub name: String,
}
