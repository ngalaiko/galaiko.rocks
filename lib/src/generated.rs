use crate::{cocktails, entries, movies, records, restaurands_and_cafes};

#[must_use]
pub fn posts(posts: &[entries::Entry]) -> maud::Markup {
    let mut posts = posts.to_vec();
    posts.sort_by(|a, b| b.frontmatter.date.cmp(&a.frontmatter.date));

    maud::html! {
        ul {
            @for post in posts {
                li {
                    a href=(post.path.display()) {
                        (post.frontmatter.title)
                    }
                    " "
                    @if let Some(date) = post.frontmatter.date.map(|date| date.format("%Y-%m-%d")) {
                        time datetime=(date) {
                            (date)
                        }
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
                    a href=(movie.href) {
                        (movie.title)
                    }
                    " "
                    (movie.date.format("%Y-%m-%d"))
                }
            }
        }
    }
}

#[must_use]
pub fn records(records: &[records::Record]) -> maud::Markup {
    let mut records = records.to_vec();
    records.sort_by(|a, b| b.date_added.cmp(&a.date_added));

    maud::html! {
        ul {
            @for record in records {
                li {
                    a href=(format!("https://www.discogs.com/release/{}", record.id)) {
                       (record.basic_information.artists[0].name) " - " (record.basic_information.title)
                    }
                    " "
                    (record.date_added.format("%Y-%m-%d"))
                }
            }
        }
    }
}

#[must_use]
pub fn restaurants_and_cafes(places: &[restaurands_and_cafes::Place]) -> maud::Markup {
    let mut places = places.to_vec();
    places.sort_by(|a, b| b.times.cmp(&a.times));

    maud::html! {
        ul {
            @for place in places {
                li {
                    (place.name)
                    " "
                    (place.times)
                    " "
                    (place.spent)
                }
            }
        }
    }
}
