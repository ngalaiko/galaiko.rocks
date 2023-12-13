use crate::posts::Post;

pub fn posts(posts: &[Post]) -> maud::Markup {
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
