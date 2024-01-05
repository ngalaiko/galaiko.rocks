use crate::types::{cocktails, entries, generated, movies, records, restaurands_and_cafes};

fn new(title: &str, content: &maud::Markup) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";

            link rel="stylesheet" href="/index.css";

            link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
            link rel="icon" sizes="32x32" href="/favicon-32x32.png";
            link rel="icon" sizes="16x16" href="/favicon-16x16.png";
            link rel="manifest" href="/site.webmanifest";

            title { (title) }
        }
        main {
            article {
                header {
                    h1 { (title) }
                }
                (content)
            }
        }
    }
}

#[must_use]
pub fn cocktails(cocktails: &[cocktails::Cocktail]) -> maud::Markup {
    new("cocktails", &generated::cocktails(cocktails))
}

#[must_use]
pub fn posts(posts: &[entries::Entry]) -> maud::Markup {
    new("archive", &generated::posts(posts))
}

#[must_use]
pub fn entry(page: &entries::Entry) -> maud::Markup {
    new(&page.frontmatter.title, &page.body)
}

#[must_use]
pub fn movies(movies: &[movies::Entry]) -> maud::Markup {
    new("movies", &generated::movies(movies))
}

#[must_use]
pub fn records(records: &[records::Record]) -> maud::Markup {
    new("records", &generated::records(records))
}

#[must_use]
pub fn places(places: &[restaurands_and_cafes::Place]) -> maud::Markup {
    new(
        "restaursnts & cafes",
        &generated::restaurants_and_cafes(places),
    )
}

#[must_use]
pub fn cocktail(cocktail: &cocktails::Cocktail) -> maud::Markup {
    new(&cocktail.frontmatter.title, &cocktail.body)
}
