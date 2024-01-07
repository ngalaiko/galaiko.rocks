use crate::types::{cocktails, entries, movies, places, records};

#[must_use]
pub fn posts(posts: &[entries::Entry]) -> maud::Markup {
    let mut posts = posts.to_vec();
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    maud::html! {
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
    }
}

#[must_use]
pub fn cocktails(cocktails: &[cocktails::Cocktail]) -> maud::Markup {
    let mut cocktails = cocktails.to_vec();
    cocktails.sort_by(|a, b| a.frontmatter.title.cmp(&b.frontmatter.title));

    maud::html! {
        ul {
            @for cocktail in cocktails {
                li {
                    a href=(cocktail.path.display()) {
                        (cocktail.frontmatter.title)
                    }
                }
            }
        }
    }
}

#[must_use]
pub fn movies(movies: &[movies::Entry]) -> maud::Markup {
    let mut movies = movies.to_vec();
    movies.sort_by(|a, b| b.date.cmp(&a.date));

    maud::html! {
        ul {
            @for movie in movies {
                li {
                    (movie.date.format("%Y-%m-%d"))
                    " | "
                    a href=(movie.href) {
                        (movie.title)
                    }
                    @if movie.is_liked {
                        " ♥"
                    }
                    @if movie.is_rewatch {
                        " ↻"
                    }
                }
            }
        }
    }
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

    maud::html! {
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
    }
}

#[must_use]
pub fn places(places: &[places::Place]) -> maud::Markup {
    let mut places = places.to_vec();
    places.sort_by(|a, b| b.times.cmp(&a.times));

    maud::html! {
        table {
            thead {
                tr {
                    th {"name"}
                    th {"times"}
                    th {"spent"}
                }
            }
            tbody {
                @for place in places {
                    tr {
                        td {
                            (place.name)
                        }
                        td {
                            (place.times)
                        }
                        td {
                            (place.spent) " SEK"
                        }
                    }
                }
            }
        }
    }
}
