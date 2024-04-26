use crate::render;

use types;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn posts(posts: &[(std::path::PathBuf, types::entries::Entry)]) -> atom_syndication::Feed {
    let mut posts = posts
        .iter()
        .filter_map(|(path, post)| {
            post.frontmatter.date.map(|date| {
                let path = path.display().to_string();
                (
                    post.frontmatter.id.clone().unwrap_or_else(|| path.clone()),
                    date.and_hms_opt(0, 0, 0).expect("time is right").and_utc(),
                    post.frontmatter.title.clone(),
                    path,
                    render::html::markdown(&post.body).into_string(),
                )
            })
        })
        .collect::<Vec<_>>();
    posts.sort_by(|a, b| b.1.cmp(&a.1));
    posts.reverse();

    let updated = posts
        .iter()
        .map(|(_, updated, _, _, _)| *updated)
        .next()
        .expect("at least one post");

    let entries = posts
        .into_iter()
        .map(|(id, updated, title, path, body)| {
            atom_syndication::EntryBuilder::default()
                .id(id)
                .title(title)
                .links(vec![atom_syndication::Link {
                    href: format!("https://nikita.galaiko.rocks/{path}"),
                    rel: "alternate".to_string(),
                    mime_type: Some("text/html".to_string()),
                    ..Default::default()
                }])
                .updated(updated)
                .content(Some(
                    atom_syndication::ContentBuilder::default()
                        .content_type(Some("html".to_string()))
                        .value(Some(body))
                        .build(),
                ))
                .build()
        })
        .collect::<Vec<_>>();

    atom_syndication::FeedBuilder::default()
        .title("posts by nikita galaiko")
        .id("https://galaiko.rocks/posts".to_string())
        .links(vec![
            atom_syndication::Link {
                href: "https://nikita.glaiko.rocks/posts/index.atom".to_string(),
                rel: "self".to_string(),
                mime_type: Some("application/atom+xml".to_string()),
                ..Default::default()
            },
            atom_syndication::Link {
                href: "https://nikita.galaiko.rocks/posts/".to_string(),
                rel: "alternate".to_string(),
                mime_type: Some("text/html".to_string()),
                ..Default::default()
            },
        ])
        .updated(updated)
        .icon(Some("https://nikita.galaiko.rocks/favicon.ico".to_string()))
        .authors(vec![atom_syndication::Person {
            name: "nikita galaiko".to_string(),
            email: Some("nikita@galaiko.rocks".to_string()),
            uri: Some("https://nikita.galaiko.rocks/index.html".to_string()),
        }])
        .entries(entries)
        .build()
}
