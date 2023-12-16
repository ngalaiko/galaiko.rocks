use crate::movies;

#[derive(clap::ValueEnum, Clone)]
pub enum Resource {
    All,
    Movies,
}

#[derive(Debug)]
pub enum Error {
    Movies(MoviesError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Movies(error) => write!(f, "failed to update movies: {error}"),
        }
    }
}

impl std::error::Error for Error {}

pub async fn update(resource: &Resource) -> Result<(), Box<dyn std::error::Error>> {
    if matches!(resource, Resource::All | Resource::Movies) {
        update_movies().await.map_err(Error::Movies)?;
    }
    Ok(())
}

#[derive(Debug)]
pub enum MoviesError {
    Surf(surf::Error),
    FromEntry(FromEntryError),
}

impl std::fmt::Display for MoviesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surf(error) => write!(f, "{error}"),
            Self::FromEntry(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for MoviesError {}

fn page_url(n: u8) -> String {
    format!("https://letterboxd.com/ngalaiko/films/diary/page/{n}/")
}

#[derive(Debug)]
pub enum FromEntryError {
    Date,
    Title,
    Href,
    Slug,
    Id,
    SmallPoster,
}

impl std::fmt::Display for FromEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromEntryError::Date => write!(f, "watch date not found"),
            FromEntryError::Title => write!(f, "title not found"),
            FromEntryError::Href => write!(f, "link not found"),
            FromEntryError::Slug => write!(f, "slug not found"),
            FromEntryError::Id => write!(f, "id not found"),
            FromEntryError::SmallPoster => write!(f, "small poster not found"),
        }
    }
}

impl std::error::Error for FromEntryError {}

impl TryFrom<select::node::Node<'_>> for movies::Entry {
    type Error = FromEntryError;

    fn try_from(value: select::node::Node) -> Result<Self, Self::Error> {
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

        let href = details
            .and_then(|node| node.attr("data-film-link"))
            .map(|path| format!("https://letterboxd.com{path}"))
            .ok_or(FromEntryError::Href)?;

        let slug = details
            .and_then(|node| node.attr("data-film-slug"))
            .ok_or(FromEntryError::Slug)?;

        let id = details
            .and_then(|node| node.attr("data-film-id"))
            .ok_or(FromEntryError::Id)?;

        let poster_large_href = {
            let mut href = "https://a.ltrbxd.com/resized/film-poster/".to_string();
            for char in id.chars() {
                href += char.to_string().as_str();
                href += "/";
            }
            href += &format!("{id}-{slug}-0-600-0-900-crop.jpg");
            href
        };

        let poster_small_href = value
            .find(Class("td-film-details").descendant(Name("img")))
            .next()
            .and_then(|node| node.attr("src"))
            .map(std::string::ToString::to_string)
            .ok_or(FromEntryError::SmallPoster)?;

        Ok(movies::Entry {
            title,
            date,
            is_rewatch,
            is_liked,
            href,
            poster_large_href,
            poster_small_href,
        })
    }
}

fn parse_page(body: &str) -> Result<(Vec<movies::Entry>, bool), MoviesError> {
    use select::document::Document;
    use select::predicate::{Attr, Class, Name, Predicate};

    let document = Document::from(body);

    let entries = document
        .find(Attr("data-object-name", "entry"))
        .map(movies::Entry::try_from)
        .collect::<Result<Vec<_>, FromEntryError>>()
        .map_err(MoviesError::FromEntry)?;

    let has_next = document.find(Name("a").and(Class("next"))).next().is_some();

    Ok((entries, has_next))
}

#[test]
fn test_parse_page() {
    let input = include_str!("./letterboxd/diary_page.html");

    let (entries, has_next) = parse_page(input).unwrap();
    assert_eq!(entries.len(), 50);
    assert!(has_next);
}

async fn fetch_page(n: u8) -> Result<(Vec<movies::Entry>, bool), MoviesError> {
    let mut response = surf::get(page_url(n)).await.map_err(MoviesError::Surf)?;
    let body = response.body_string().await.map_err(MoviesError::Surf)?;
    parse_page(body.as_str())
}

async fn update_movies() -> Result<(), MoviesError> {
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

    Ok(())
}
