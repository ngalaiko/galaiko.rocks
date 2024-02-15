use shared::types::movies;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    FromEntry(FromEntryError),
    Ser(serde_json::Error),
    Io(std::io::Error),
    Download((String, reqwest::StatusCode)),
    Poster(PosterError),
    UTF8(std::string::FromUtf8Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(error) => write!(f, "{error}"),
            Self::FromEntry(error) => write!(f, "{error}"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Ser(error) => write!(f, "{error}"),
            Self::Download((url, code)) => write!(f, "{url} returned {code}"),
            Self::Poster(error) => write!(f, "{error}"),
            Self::UTF8(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {}

fn page_url(n: u8) -> String {
    format!("https://letterboxd.com/ngalaiko/films/diary/page/{n}/")
}

#[derive(Debug)]
pub enum FromEntryError {
    Date,
    Title,
    Href,
    Slug,
}

impl std::fmt::Display for FromEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromEntryError::Date => write!(f, "watch date not found"),
            FromEntryError::Title => write!(f, "title not found"),
            FromEntryError::Href => write!(f, "link not found"),
            FromEntryError::Slug => write!(f, "slug not found"),
        }
    }
}

impl std::error::Error for FromEntryError {}

fn parse_entry(value: select::node::Node) -> Result<movies::Entry, FromEntryError> {
    use select::predicate::{Class, Name, Predicate};
    let date = value
        .find(Class("td-day").descendant(Name("a")))
        .next()
        .and_then(|node| node.attr("href"))
        .and_then(|href| {
            chrono::NaiveDate::parse_from_str(href, "/ngalaiko/films/diary/for/%Y/%m/%d/").ok()
        })
        .ok_or(FromEntryError::Date)?;

    let title = value
        .find(
            Class("td-film-details")
                .descendant(Name("h3"))
                .descendant(Name("a")),
        )
        .next()
        .and_then(|node| node.first_child())
        .and_then(|node| node.as_text())
        .map(std::string::ToString::to_string)
        .ok_or(FromEntryError::Title)?;

    let is_liked = value
        .find(Class("td-like").descendant(Class("icon-liked")))
        .next()
        .is_some();

    let is_rewatch = value
        .find(Class("td-rewatch").and(Class("icon-status-off").not()))
        .next()
        .is_some();

    let details = value.find(Class("td-actions")).next();

    let href = value
        .find(
            Class("td-film-details")
                .descendant(Name("h3"))
                .descendant(Name("a")),
        )
        .next()
        .and_then(|node| node.attr("href"))
        .map(std::string::ToString::to_string)
        .map(|href| format!("https://letterboxd.com{href}"))
        .ok_or(FromEntryError::Href)?;

    let title_slug = details
        .and_then(|node| node.attr("data-film-slug"))
        .map(std::string::ToString::to_string)
        .ok_or(FromEntryError::Slug)?;

    Ok(movies::Entry {
        title,
        title_slug,
        date,
        is_rewatch,
        is_liked,
        href,
    })
}

fn parse_page(body: &str) -> Result<(Vec<movies::Entry>, bool), Error> {
    use select::document::Document;
    use select::predicate::{Attr, Class, Name, Predicate};

    let document = Document::from(body);

    let entries = document
        .find(Attr("data-object-name", "entry"))
        .map(parse_entry)
        .collect::<Result<Vec<_>, FromEntryError>>()
        .map_err(Error::FromEntry)?;

    let has_next = document.find(Name("a").and(Class("next"))).next().is_some();

    Ok((entries, has_next))
}

#[tracing::instrument]
async fn fetch_page(n: u8) -> Result<(Vec<movies::Entry>, bool), Error> {
    let response = reqwest::get(page_url(n)).await.map_err(Error::Reqwest)?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(Error::Download((page_url(n), response.status())));
    }
    let body = response.bytes().await.map_err(Error::Reqwest)?;
    let body = String::from_utf8(body.to_vec()).map_err(Error::UTF8)?;
    parse_page(body.as_str())
}

pub async fn update<P: AsRef<std::path::Path>>(output: P) -> Result<(), Error> {
    let output = output.as_ref();

    let mut n = 1;
    let mut entries = vec![];
    loop {
        let (mut page_entries, has_next) = fetch_page(n).await?;
        entries.append(&mut page_entries);
        if has_next {
            n += 1;
        } else {
            break;
        }
    }

    let mut all_files = vec![];
    for entry in &entries {
        let path = entry
            .href
            .strip_prefix("https://letterboxd.com/ngalaiko/film/")
            .expect("invalid href");
        let components = path.split('/').collect::<Vec<_>>();
        let watch_number = if components.len() == 3 {
            components[1].parse::<u64>().expect("invalid watch number")
        } else {
            0
        };
        let data_output = output
            .join(&entry.title_slug)
            .join(watch_number.to_string())
            .with_extension("json")
            .clone();

        let dir = data_output.parent().expect("invalid output path");
        if !tokio::fs::try_exists(&data_output)
            .await
            .map_err(Error::Io)?
        {
            tokio::fs::create_dir_all(&dir).await.map_err(Error::Io)?;

            tokio::fs::write(
                &data_output,
                serde_json::to_vec_pretty(&entry).map_err(Error::Ser)?,
            )
            .await
            .map_err(Error::Io)?;
        }

        let poster_output = dir.with_extension("jpg");
        if !tokio::fs::try_exists(&poster_output)
            .await
            .map_err(Error::Io)?
        {
            let image = get_poster(entry).await.map_err(Error::Poster)?;
            tokio::fs::write(&poster_output, &image)
                .await
                .map_err(Error::Io)?;
        }

        all_files.push(data_output);
        all_files.push(poster_output);
    }

    let mut entries = tokio::fs::read_dir(&output).await.map_err(Error::Io)?;
    while let Some(entry) = entries.next_entry().await.map_err(Error::Io)? {
        let metadata = tokio::fs::metadata(entry.path()).await.map_err(Error::Io)?;
        if !metadata.is_file() {
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

#[derive(Debug)]
pub enum PosterError {
    Reqwest(reqwest::Error),
    Download((String, reqwest::StatusCode)),
    NoLdJson,
    LdDe(serde_json::Error),
    UTF8(std::string::FromUtf8Error),
}

impl std::fmt::Display for PosterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PosterError::Reqwest(error) => write!(f, "{error}"),
            PosterError::Download((url, code)) => write!(f, "{url} returned {code}"),
            PosterError::NoLdJson => write!(f, "no ld+json found"),
            PosterError::LdDe(error) => write!(f, "{error}"),
            PosterError::UTF8(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PosterError {}

#[tracing::instrument(skip(entry), fields(title = entry.title.as_str()))]
async fn get_poster(entry: &movies::Entry) -> Result<Vec<u8>, PosterError> {
    use select::document::Document;
    use select::predicate::Attr;

    #[derive(Debug, serde::Deserialize)]
    struct LdData<'a> {
        image: &'a str,
    }

    let film_page_url = format!("https://letterboxd.com/film/{}/", entry.title_slug);
    let response = reqwest::get(&film_page_url)
        .await
        .map_err(PosterError::Reqwest)?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(PosterError::Download((film_page_url, response.status())));
    }
    let body = response.bytes().await.map_err(PosterError::Reqwest)?;
    let body = String::from_utf8(body.to_vec()).map_err(PosterError::UTF8)?;
    let movie_page = Document::from(body.as_str());

    let ld_data = movie_page
        .find(Attr("type", "application/ld+json"))
        .next()
        .map(|node| node.text())
        .ok_or(PosterError::NoLdJson)?;
    let ld_data = ld_data.trim_start_matches("\n/* <![CDATA[ */\n");
    let ld_data = ld_data.trim_end_matches("\n/* ]]> */\n");
    let ld_data = serde_json::from_str::<LdData>(ld_data).map_err(PosterError::LdDe)?;

    let image_url = ld_data.image.replace("0-230-0-345", "0-600-0-900");

    let response = reqwest::get(image_url)
        .await
        .map_err(PosterError::Reqwest)?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(PosterError::Download((
            ld_data.image.to_string(),
            response.status(),
        )));
    }
    let image = response.bytes().await.map_err(PosterError::Reqwest)?;
    Ok(image.to_vec())
}
