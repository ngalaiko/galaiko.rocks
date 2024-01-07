use shared::types::records;

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

pub async fn update<P: AsRef<async_std::path::Path>>(token: &str, output: P) -> Result<(), Error> {
    use async_std::prelude::*;

    let output = output.as_ref();

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

    async_std::fs::create_dir_all(&output)
        .await
        .map_err(Error::Io)?;

    let mut all_files = vec![];
    for record in &records {
        let title = record.basic_information.title.replace('/', "-");
        let mut output_json = output.to_path_buf();
        output_json.push(format!("{title}.json"));

        if !output_json.exists().await {
            let serialized = serde_json::to_vec_pretty(&record).map_err(Error::Ser)?;
            async_std::fs::write(&output_json, serialized)
                .await
                .map_err(Error::Io)?;
        }
        all_files.push(output_json);

        if let Some(ext) =
            std::path::Path::new(&record.basic_information.cover_image).extension()
            .and_then(|ext| ext.to_str())
        {
            let mut image_out = output.to_path_buf();
            image_out.push(format!("{title}.{ext}"));

            if !image_out.exists().await {
                let mut response = get_image(&record.basic_information.cover_image, token)
                    .await
                    .map_err(Error::Surf)?;
                let image = response.body_bytes().await.map_err(Error::Surf)?;
                async_std::fs::write(&image_out, &image)
                    .await
                    .map_err(Error::Io)?;
            }
            all_files.push(image_out);
        }
    }

    let mut entries = async_std::fs::read_dir(&output).await.map_err(Error::Io)?;
    while let Some(res) = entries.next().await {
        let entry = res.map_err(Error::Io)?;
        if !entry.file_type().await.map_err(Error::Io)?.is_file() {
            continue;
        }
        if !all_files.contains(&entry.path()) {
            async_std::fs::remove_file(entry.path())
                .await
                .map_err(Error::Io)?;
        }
    }

    Ok(())
}

async fn get_image(url: &str, token: &str) -> Result<surf::Response, surf::Error> {
    loop {
        let response = surf::get(url)
            .header("Authorization", format!("Discogs token={token}"))
            .await
            .map_err(Error::Surf)?;

        match response.status() {
            surf::StatusCode::TooManyRequests => {
                let retry_after = response
                    .header("Retry-After")
                    .and_then(|value| value.get(0))
                    .and_then(|value| value.as_str().parse::<u64>().ok())
                    .unwrap_or(1);
                async_std::task::sleep(std::time::Duration::from_secs(retry_after)).await;
                continue;
            }
            _ => return Ok(response),
        };
    }
}
