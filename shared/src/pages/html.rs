use crate::types::{cocktails, entries, movies, places, records};

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

            link rel="stylesheet" href=(format!("/index.css?v={}",chrono::Local::now().timestamp()));

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
pub fn cocktail(cocktail: &cocktails::Cocktail) -> maud::Markup {
    let html = maud::html! {
        aside {
            img src=(format!("./{}.jpeg", cocktail.frontmatter.title)) loading="lazy" alt=(cocktail.frontmatter.title);
        }
        (cocktail.body)
    };

    new(
        &cocktail.frontmatter.title,
        None,
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn post(page: &entries::Entry) -> maud::Markup {
    new(
        &page.frontmatter.title,
        Some(&maud::html! {
            link rel="alternate" type="application/atom+xml" href="/posts/index.atom";
        }),
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
pub fn posts(posts: &[entries::Entry]) -> maud::Markup {
    let mut posts = posts.to_vec();
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    let html = maud::html! {
        ul {
            @for post in posts {
                li {
                    @if let Some(date) = post.frontmatter.date.map(|date| date.format("%Y-%m-%d")) {
                        time datetime=(date) {
                            (date)
                        }
                        " | "
                    }
                    a href=(post.path.display()) {
                        (post.frontmatter.title)
                    }
                }
            }
        }
    };

    new(
        "posts",
        Some(&maud::html! {
            link rel="alternate" type="application/atom+xml" href="/posts/index.atom";
        }),
        &html,
        &footer_with_copy_right(),
    )
}

#[must_use]
pub fn cocktails(cocktails: &[cocktails::Cocktail]) -> maud::Markup {
    let mut cocktails = cocktails.to_vec();
    cocktails.sort_by(|a, b| a.frontmatter.title.cmp(&b.frontmatter.title));

    let html = maud::html! {
        ul {
            @for cocktail in cocktails {
                li {
                    a href=(cocktail.path.display()) {
                        figure {
                            img src=(format!("./{}.jpeg", cocktail.frontmatter.title)) loading="lazy" alt=(cocktail.frontmatter.title);
                            figcaption {
                                center {
                                    (cocktail.frontmatter.title)
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    new(
        "cocktails",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/grid.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn movies(movies: &[movies::Entry]) -> maud::Markup {
    let mut movies = movies.to_vec();
    movies.sort_by(|a, b| b.date.cmp(&a.date));

    let html = maud::html! {
        table {
            thead {
                tr {
                    th { "date" }
                    th { "poster" }
                    th { "film" }
                    th { "liked" }
                    th { "rewatch" }
                }
            }
            @for movie in movies {
                    tr {
                        td { time datetime=(movie.date.format("%Y-%m-%d")) { (movie.date.format("%Y-%m-%d")) } }
                        td { img width="35px" src=(format!("./{}.jpg", movie.title_slug.replace('/', "-"))) loading="lazy" alt=(movie.title); }
                        td { a href=(movie.href) { (movie.title) } }
                        td { @if movie.is_liked   { "♥" } }
                        td { @if movie.is_rewatch { "↻" } }
                }
            }
        }
    };

    new(
        "movies",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/table.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn records(records: &[records::Record]) -> maud::Markup {
    let mut records = records.to_vec();
    records.sort_by(|a, b| {
        let artist_a = &a.basic_information.artists[0]
            .name
            .strip_prefix("The ")
            .unwrap_or(&a.basic_information.artists[0].name);
        let artist_b = &b.basic_information.artists[0]
            .name
            .strip_prefix("The ")
            .unwrap_or(&b.basic_information.artists[0].name);
        artist_a.cmp(artist_b)
    });

    let records = records.iter().filter_map(|record| {
        std::path::Path::new(&record.basic_information.cover_image)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                (
                    record.id,
                    record.basic_information.artists[0].name.clone(),
                    record.basic_information.title.clone(),
                    format!(
                        "./{}.{ext}",
                        record.basic_information.title.replace('/', "-")
                    ),
                )
            })
    });

    let html = maud::html! {
        ul {
            @for (id, artist, title, cover_href) in records {
                li {
                    a href=(format!("https://www.discogs.com/release/{}", id)) {
                        figure {
                            img src=(cover_href) loading="lazy" alt=(title);
                            figcaption {
                                center {
                                    span { (artist) }
                                    br;
                                    span { (title) }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    new(
        "records",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/grid.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn places(places: &[places::Place]) -> maud::Markup {
    let mut places = places.to_vec();
    places.sort_by(|a, b| b.times.cmp(&a.times));

    let html = maud::html! {
        table {
            thead {
                tr {
                    th { "name" }
                    th { "times" }
                    th { "spent" }
                }
            }
            tbody {
                @for place in places {
                    tr {
                        td { (place.name) }
                        td { (place.times) }
                        td { (place.spent) " SEK" }
                    }
                }
            }
        }
    };

    new(
        "places",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/table.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}
