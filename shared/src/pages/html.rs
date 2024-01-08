use crate::types::{cocktails, entries, generated, movies, places, records};

fn footer_without_copy_right() -> maud::Markup {
    maud::html! {
        a href="/privacy.html" { "privacy policy" }
        span { "·" }
        a href="https://github.com/ngalaiko/galaiko.rocks" { "source" }
    }
}

fn footer_with_copy_right() -> maud::Markup {
    let year = chrono::Local::now().format("%Y").to_string();
    maud::html! {
        a href="/index.html" { "nikita galaiko" }
        span { "·" }
        a href="https://creativecommons.org/licenses/by-nc/4.0/" rel="license" { "CC BY-NC 4.0" }
        span { "·" }
        span {
            "2018.."(year)
        }
        span { "·" }
        (footer_without_copy_right())
    }
}

fn new(
    title: &str,
    header: Option<&maud::Markup>,
    content: &maud::Markup,
    footer: &maud::Markup,
) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";

            link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
            link rel="icon" sizes="32x32" href="/favicon-32x32.png";
            link rel="icon" sizes="16x16" href="/favicon-16x16.png";
            link rel="manifest" href="/site.webmanifest";

            link rel="stylesheet" href="/index.css";

            @if let Some(header) = header {
                (header)
            }

            title { (title) }
        }
        body {
            main {
                article {
                    header {
                        h1 { (title) }
                    }
                    (content)
                }
            }
            footer {
                (footer)
            }
        }
    }
}

#[must_use]
pub fn cocktails(cocktails: &[cocktails::Cocktail]) -> maud::Markup {
    new(
        "cocktails",
        Some(&maud::html! {
            link rel="stylesheet" href="/styles/grid.css";
        }),
        &generated::cocktails(cocktails),
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn cocktail(cocktail: &cocktails::Cocktail) -> maud::Markup {
    new(
        &cocktail.frontmatter.title,
        None,
        &cocktail.body,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn posts(posts: &[entries::Entry]) -> maud::Markup {
    new(
        "posts",
        Some(&maud::html! {
            link rel="alternate" type="application/atom+xml" href="/posts/index.atom";
        }),
        &generated::posts(posts),
        &footer_with_copy_right(),
    )
}

#[must_use]
pub fn post(page: &entries::Entry) -> maud::Markup {
    new(
        &page.frontmatter.title,
        None,
        &page.body,
        &footer_with_copy_right(),
    )
}

#[must_use]
pub fn entry(page: &entries::Entry) -> maud::Markup {
    new(
        &page.frontmatter.title,
        None,
        &page.body,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn movies(movies: &[movies::Entry]) -> maud::Markup {
    new(
        "movies",
        None,
        &generated::movies(movies),
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn records(records: &[records::Record]) -> maud::Markup {
    new(
        "records",
        Some(&maud::html! {
            link rel="stylesheet" href="/styles/grid.css";
        }),
        &generated::records(records),
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn places(places: &[places::Place]) -> maud::Markup {
    new(
        "places",
        Some(&maud::html! {
            link rel="stylesheet" href="/styles/table.css";
        }),
        &generated::places(places),
        &footer_without_copy_right(),
    )
}
