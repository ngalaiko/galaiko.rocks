#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    date_added: chrono::DateTime<chrono::Utc>,
    basic_information: BasicInformation,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct BasicInformation {
    title: String,
    artists: Vec<Artist>,
    thumb: String,
    resource_url: String,
    cover_image: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Artist {
    name: String,
    resource_url: String,
}
