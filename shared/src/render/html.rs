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

fn page(
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
#[allow(clippy::too_many_lines)]
pub fn cocktail(cocktail: &cocktails::Cocktail) -> maud::Markup {
    let ingredient_list = cocktail.recipe.group_ingredients();
    let cookware_list = cocktail.recipe.group_cookware();
    let source_url = cocktail
        .recipe
        .source()
        .and_then(cooklang::metadata::NameAndUrl::url);
    let source_name = cocktail
        .recipe
        .source()
        .and_then(cooklang::metadata::NameAndUrl::name)
        .or_else(|| source_url.as_ref().and_then(|u| u.host_str()));

    let html = maud::html! {
        aside {
            img src=(format!("./{}.800x0@2x.webp", cocktail.recipe.title())) loading="lazy" alt=(cocktail.recipe.title());
        }

        @if !ingredient_list.is_empty() {
            h2 { "ingredients:" }
            ul {
                @for entry in &ingredient_list {
                    li {
                        b { (entry.ingredient.display_name()) }
                        @if !entry.quantity.is_empty() {": " (entry.quantity) }
                        @if let Some(n) = &entry.ingredient.note { " (" (n) ")" }
                    }
                }
            }
        }

        @if !cookware_list.is_empty() {
            h2 { "cookware:" }
            ul {
                @for item in &cookware_list {
                    li {
                        b { (item.cookware.display_name()) }
                        @if !item.amount.is_empty() { ": " (item.amount) }
                    }
                }
            }
        }

        @if !cookware_list.is_empty() || !ingredient_list.is_empty() {
            hr {}
        }

        @for (s_index, section) in cocktail.recipe.sections().iter().enumerate() {
            @let s_num = s_index + 1;
            @if let Some(name) = &section.name {
                h3 { "(" (s_num) ") " (name) }
            } @else if cocktail.recipe.sections().len() > 1 {
                h3 { "section " (s_num) }
            }

            @for content in &section.content {
                @match content {
                    cooklang::Content::Text(t) => p { (t) },
                    cooklang::Content::Step(s) => p {
                        b { (s.number) ". " }
                        @for item in &s.items {
                            @match item {
                                cooklang::Item::Ingredient { index } => {
                                    @let igr = &cocktail.recipe.ingredients()[*index];
                                    span.ingredient {
                                        (igr.display_name())
                                        @if let Some(q) = &igr.quantity {
                                            i { "(" (q) ")" }
                                        }
                                        @if let Some((index, target)) = &igr.relation.references_to() {
                                            @match target {
                                                cooklang::IngredientReferenceTarget::Step => {
                                                    i { "(from step " (section.content[*index].unwrap_step().number) ")" }
                                                }
                                                cooklang::IngredientReferenceTarget::Section => {
                                                    @let sect = *index + 1;
                                                    i { "(from section " (sect) ")" }
                                                }
                                                cooklang::IngredientReferenceTarget::Ingredient => {}
                                            }
                                        }
                                    }
                                }
                                cooklang::Item::Cookware { index } => {
                                    @let cw = &cocktail.recipe.cookware()[*index];
                                    span.cookware {
                                        (cw.display_name())
                                        @if let Some(q) = &cw.quantity {
                                            i { "(" (q) ")" }
                                        }
                                    }
                                }
                                cooklang::Item::Timer { index } => {
                                    @let tm = &cocktail.recipe.timers()[*index];
                                    span.timer {
                                        @if let Some(name) = &tm.name {
                                            "(" (name) ")"
                                        }
                                        @if let Some(q) = &tm.quantity {
                                            i { (q) }
                                        }
                                    }
                                }
                                cooklang::Item::InlineQuantity { index } => {
                                    @let q = &cocktail.recipe.inline_quantities()[*index];
                                    i.temp { (q) }
                                }
                                cooklang::Item::Text { value } => {
                                    (value)
                                }
                            }
                        }
                    }
                }
            }
        }

        @match (source_url, source_name) {
            (Some(url), Some(source_name)) => {
                hr {}
                p {
                    "source: "
                    a href=(url) { (source_name) };
                }
            },
            (Some(url), None) => {
                hr {}
                a href=(url) { "source" };
            }
            (None, Some(name)) => {
                hr {}
                p {
                    "source: "
                    (name)
                }
            }
            (None, None) => {}
        }
    };

    page(
        cocktail.recipe.title(),
        None,
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn post(entry: &entries::Entry) -> maud::Markup {
    page(
        &entry.frontmatter.title,
        Some(&maud::html! {
            link rel="alternate" type="application/atom+xml" href="/posts/index.atom";
        }),
        &markdown(&entry.body),
        &footer_with_copy_right(),
    )
}

#[must_use]
pub fn entry(entry: &entries::Entry) -> maud::Markup {
    page(
        &entry.frontmatter.title,
        None,
        &markdown(&entry.body),
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn posts(posts: &[(std::path::PathBuf, entries::Entry)]) -> maud::Markup {
    let mut posts = posts
        .iter()
        .filter_map(|(path, post)| {
            post.frontmatter.date.map(|date| {
                (
                    path.display().to_string(),
                    date.format("%Y-%m-%d").to_string(),
                    post.frontmatter.title.clone(),
                )
            })
        })
        .collect::<Vec<_>>();
    posts.sort_by(|a, b| b.1.cmp(&a.1));

    let html = maud::html! {
        ul {
            @for (href, date, title) in posts {
                li {
                    time datetime=(date) {
                        (date)
                    }
                    " | "
                    a href=(href) { (title) }
                }
            }
        }
    };

    page(
        "posts",
        Some(&maud::html! {
            link rel="alternate" type="application/atom+xml" href="/posts/index.atom";
        }),
        &html,
        &footer_with_copy_right(),
    )
}

#[must_use]
pub fn cocktails(cocktails: &[(std::path::PathBuf, cocktails::Cocktail)]) -> maud::Markup {
    let mut cocktails = cocktails
        .iter()
        .map(|(path, cocktail)| (path.display().to_string(), cocktail.recipe.title()))
        .collect::<Vec<_>>();
    cocktails.sort_by(|a, b| a.1.cmp(b.1));

    let html = maud::html! {
        ul {
            @for (href, title) in cocktails {
                li {
                    a href=(href) {
                        figure {
                            img width="200px"
                            src=(format!("./{title}.200x0@2x.webp")) loading="lazy" alt=(title);
                            figcaption { center { (title) } }
                        }
                    }
                }
            }
        }
    };

    page(
        "cocktails",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/grid.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn movies(movies: &[(std::path::PathBuf, movies::Entry)]) -> maud::Markup {
    let mut movies = movies
        .iter()
        .map(|(_, movie)| {
            (
                movie.date.format("%Y-%m-%d").to_string(),
                format!("./{}.70x0@2x.webp", movie.title_slug.replace('/', "-")),
                movie.href.clone(),
                movie.title.clone(),
                movie.is_liked,
            )
        })
        .collect::<Vec<_>>();
    movies.sort_by(|a, b| b.0.cmp(&a.0));

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

    page(
        "movies",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/table.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn records(records: &[(std::path::PathBuf, records::Record)]) -> maud::Markup {
    let mut records = records
        .iter()
        .map(|(_, record)| {
            (
                record.id,
                record.basic_information.artists[0].name.clone(),
                record.basic_information.title.clone(),
                format!(
                    "./{}.200x0@2x.webp",
                    record.basic_information.title.replace('/', "-")
                ),
            )
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
                            img width="200" src=(cover_href) loading="lazy" alt=(title);
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

    page(
        "records",
        Some(&maud::html! {
            link rel="stylesheet" href=(format!("/styles/grid.css?v={}", chrono::Local::now().timestamp()));
        }),
        &html,
        &footer_without_copy_right(),
    )
}

#[must_use]
pub fn places(places: &[(std::path::PathBuf, places::Place)]) -> maud::Markup {
    let mut places = places
        .iter()
        .map(|(_path, place)| (place.name.clone(), place.times, place.spent))
        .collect::<Vec<_>>();
    places.sort_by(|a, b| b.1.cmp(&a.1));

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

    page(
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
    page(
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

#[must_use]
pub fn markdown(events: &[pulldown_cmark::Event]) -> maud::Markup {
    let events = events.to_vec();
    let mut body = String::new();
    pulldown_cmark::html::push_html(&mut body, events.into_iter());
    maud::PreEscaped(body)
}
