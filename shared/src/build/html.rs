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

            meta http-equiv="Content-Security-Policy" content="default-src 'self';";

            link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png";
            link rel="icon" sizes="32x32" href="/favicon-32x32.png";
            link rel="icon" sizes="16x16" href="/favicon-16x16.png";
            link rel="manifest" href="/site.webmanifest";

            link rel="stylesheet" href=(format!("/fonts/index.css?v={}", chrono::Local::now().timestamp()));
            link rel="stylesheet" href=(format!("/index.css?v={}", chrono::Local::now().timestamp()));

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
    let mut posts = posts
        .iter()
        .filter(|post| post.frontmatter.date.is_some())
        .collect::<Vec<_>>();
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
    let mut cocktails = cocktails
        .iter()
        .map(|cocktail| {
            (
                cocktail.path.display().to_string(),
                format!("./{}.jpeg", cocktail.frontmatter.title),
                cocktail.frontmatter.title.clone(),
            )
        })
        .collect::<Vec<_>>();
    cocktails.sort_by(|a, b| a.2.cmp(&b.2));

    let html = maud::html! {
        ul {
            @for (href, image_href, title) in cocktails {
                li {
                    a href=(href) {
                        figure {
                            img src=(image_href) loading="lazy" alt=(title);
                            figcaption { center { (title) } }
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
    let mut movies = movies
        .iter()
        .map(|movie| {
            (
                movie.date.format("%Y-%m-%d").to_string(),
                format!("./{}.jpg", movie.title_slug.replace('/', "-")),
                movie.href.clone(),
                movie.title.clone(),
                movie.is_liked,
            )
        })
        .collect::<Vec<_>>();
    movies.sort_by(|a, b| a.0.cmp(&b.0));

    let html = maud::html! {
        table {
            thead {
                tr {
                    th { "date" }
                    th { "poster" }
                    th { "film" }
                }
            }
            @for (date, poster_href, href, title, is_liked) in movies {
                tr {
                    td { time datetime=(date) { (date) } }
                    td { img width="70px" src=(poster_href) loading="lazy" alt=(title); }
                    td {
                        a href=(href) { (title) }
                        @if is_liked { " | " "♥" }
                    }
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
    let mut records = records
        .into_iter()
        .filter_map(|record| {
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
        })
        .collect::<Vec<_>>();
    records.sort_by(|a, b| a.2.cmp(&b.2));
    records.sort_by(|a, b| a.1.cmp(&b.1));

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
    let mut places = places
        .iter()
        .map(|place| (place.name.clone(), place.times, place.spent))
        .collect::<Vec<_>>();
    places.sort_by_key(|p| p.1);

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
                @for (name, times, spent) in places {
                    tr {
                        td { (name) }
                        td { (times) }
                        td { (spent) " SEK" }
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

#[must_use]
pub fn redirect<P: AsRef<std::path::Path>>(to: P) -> maud::Markup {
    let to = to.as_ref().display().to_string();
    new(
        "redirect",
        Some(&maud::html! {
            meta http-equiv="refresh" content=(format!("0; url={}", to));
            link rel="canonical" href=(to);
        }),
        &maud::html! {
            "If you are not redirected automatically, follow this" a href=(to) { "link" }
        },
        &footer_without_copy_right(),
    )
}