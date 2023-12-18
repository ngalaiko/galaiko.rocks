use crate::{cocktails, movies, posts};

pub fn posts(posts: &[posts::Post]) -> maud::Markup {
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
                    time datetime=(post.frontmatter.date.format("%Y-%m-%d")) {
                        (post.frontmatter.date.format("%Y-%m-%d"))
                    }
                }
            }
        }
    }
}

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
