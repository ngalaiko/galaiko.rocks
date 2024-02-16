use shared::types::records;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Ser(serde_json::Error),
    Io(std::io::Error),
    Download((String, reqwest::StatusCode)),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Reqwest(error) => write!(f, "{error}"),
            Error::Ser(error) => write!(f, "{error}"),
            Error::Io(error) => write!(f, "{error}"),
            Error::Download((url, code)) => write!(f, "{url} returned {code}"),
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
    let output = output.as_ref();

    let mut records = vec![];
    let mut page_url =
        "https://api.discogs.com/users/ngalaiko/collection/folders/0/releases?sort=artist"
            .to_string();
    loop {
        let mut page = get_page(token, &page_url).await?;
        records.append(&mut page.releases);
        if let Some(next_page_url) = page.pagination.urls.next {
            page_url = next_page_url;
        } else {
            break;
        }
    }

    tokio::fs::create_dir_all(&output)
        .await
        .map_err(Error::Io)?;

    let mut all_files = vec![];
    for record in &records {
        let title = record.basic_information.title.replace('/', "-");

        if let (Some(filename), Some(ext)) = (
            std::path::Path::new(&record.basic_information.cover_image)
                .file_stem()
                .and_then(|ext| ext.to_str()),
            std::path::Path::new(&record.basic_information.cover_image)
                .extension()
                .and_then(|ext| ext.to_str()),
        ) {
            if filename == "spacer" {
                continue;
            }
            let mut image_out = output.to_path_buf();
            image_out.push(format!("{title}.{ext}"));

            if !tokio::fs::try_exists(&image_out).await.map_err(Error::Io)? {
                let response = get_image(&record.basic_information.cover_image, token)
                    .await
                    .map_err(Error::Reqwest)?;
                if response.status() != reqwest::StatusCode::OK {
                    return Err(Error::Download((
                        record.basic_information.cover_image.clone(),
                        response.status(),
                    )));
                }
                let image = response.bytes().await.map_err(Error::Reqwest)?;
                tokio::fs::write(&image_out, &image)
                    .await
                    .map_err(Error::Io)?;
            }
            all_files.push(image_out);
        }

        let mut output_json = output.to_path_buf();
        output_json.push(format!("{title}.json"));

        if !tokio::fs::try_exists(&output_json)
            .await
            .map_err(Error::Io)?
        {
            let serialized = serde_json::to_vec_pretty(&record).map_err(Error::Ser)?;
            tokio::fs::write(&output_json, serialized)
                .await
                .map_err(Error::Io)?;
        }
        all_files.push(output_json);
    }

    let mut entries = tokio::fs::read_dir(&output).await.map_err(Error::Io)?;
    while let Some(entry) = entries.next_entry().await.map_err(Error::Io)? {
        if !entry.file_type().await.map_err(Error::Io)?.is_file() {
            continue;
        }
        if !all_files.contains(&entry.path()) {
            tokio::fs::remove_file(entry.path())
                .await
                .map_err(Error::Io)?;
        }
    }

    Ok(())
}

#[tracing::instrument(skip(token))]
async fn get_page(token: &str, page_url: &str) -> Result<Page, Error> {
    let response = reqwest::Client::new()
        .get(page_url)
        .header("User-Agent", "https://nikita.galaiko.rocks")
        .header("Authorization", format!("Discogs token={token}"))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(Error::Reqwest)?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(Error::Download((page_url.to_string(), response.status())));
    }

    let body = response.bytes().await.map_err(Error::Reqwest)?;
    serde_json::from_slice::<Page>(&body).map_err(Error::Ser)
}

#[tracing::instrument(skip(token))]
async fn get_image(url: &str, token: &str) -> Result<reqwest::Response, reqwest::Error> {
    loop {
        let response = reqwest::Client::new()
            .get(url)
            .header("User-Agent", "https://nikita.galaiko.rocks")
            .header("Authorization", format!("Discogs token={token}"))
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("Retry-After")
                    .and_then(|value| value.to_str().ok())
                    .and_then(|value| value.parse::<u64>().ok())
                    .unwrap_or(1);
                tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;
                continue;
            }
            _ => return Ok(response),
        };
    }
}
