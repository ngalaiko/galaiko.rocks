use crate::records;

#[derive(Debug)]
pub enum Error {
    Surf(surf::Error),
    Ser(serde_json::Error),
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Surf(error) => write!(f, "{error}"),
            Error::Ser(error) => write!(f, "{error}"),
            Error::Io(error) => write!(f, "{error}"),
        }
    }
}

#[derive(serde::Deserialize)]
struct Page {
    pagination: Pagination,
    releases: Vec<records::Record>,
}

#[derive(serde::Deserialize)]
struct Pagination {
    urls: Urls,
}

#[derive(serde::Deserialize)]
struct Urls {
    next: Option<String>,
}

impl std::error::Error for Error {}

pub async fn update<P: AsRef<std::path::Path>>(token: &str, output: P) -> Result<(), Error> {
    use async_std::prelude::*;

    let mut records = vec![];
    let mut page_url =
        "https://api.discogs.com/users/ngalaiko/collection/folders/0/releases?sort=artist"
            .to_string();
    loop {
        let mut response = surf::get(&page_url)
            .header("Authorization", format!("Discogs token={token}"))
            .header("Accept", "application/json")
            .await
            .map_err(Error::Surf)?;

        let mut page = response.body_json::<Page>().await.map_err(Error::Surf)?;
        records.append(&mut page.releases);
        if let Some(next_page_url) = page.pagination.urls.next {
            page_url = next_page_url;
        } else {
            break;
        }
    }

    let mut outputs = vec![];
    for record in &records {
        let title = record.basic_information.title.replace('/', "-");
        let output = output.as_ref().join(format!("{title}.json"));
        let serialized = serde_json::to_vec_pretty(&record).map_err(Error::Ser)?;
        async_std::fs::write(&output, serialized)
            .await
            .map_err(Error::Io)?;
        outputs.push(output);
    }

    let mut entries = async_std::fs::read_dir(output.as_ref())
        .await
        .map_err(Error::Io)?;

    while let Some(res) = entries.next().await {
        let entry = res.map_err(Error::Io)?;
        let path = std::path::PathBuf::from(entry.path().display().to_string());
        if !outputs.contains(&path) {
            async_std::fs::remove_file(entry.path())
                .await
                .map_err(Error::Io)?;
        }
    }

    Ok(())
}
