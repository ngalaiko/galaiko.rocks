#[derive(Debug, serde::Serialize)]
pub struct Entry {
    pub title: String,
    pub title_slug: String,
    pub date: chrono::NaiveDate,
    pub is_rewatch: bool,
    pub is_liked: bool,
    pub href: String,
    pub poster_large_href: String,
    pub poster_small_href: String,
}
