use crate::types::entries;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn posts(posts: &[entries::Entry]) -> atom_syndication::Feed {
    let mut posts = posts
        .iter()
        .filter(|post| post.frontmatter.date.is_some())
        .collect::<Vec<_>>();
    posts.sort_by_key(|post| post.frontmatter.date.expect("date is not null"));
    posts.reverse();

    let updated = posts
        .first()
        .map(|post| post.frontmatter.date.expect("date is not null"))
        .map(|date| date.and_hms_opt(0, 0, 0).expect("time is right").and_utc())
        .expect("posts is not empty");

    let entries = posts
        .iter()
        .map(|post| {
            let id = post
                .frontmatter
                .id
                .clone()
                .unwrap_or_else(|| post.path.display().to_string());
            let updated = post
                .frontmatter
                .date
                .expect("date is not null")
                .and_hms_opt(0, 0, 0)
                .expect("time is right")
                .and_utc();
            atom_syndication::EntryBuilder::default()
                .title(post.frontmatter.title.clone())
                .id(id)
                .links(vec![atom_syndication::Link {
                    href: format!("https://nikita.galaiko.rocks/{}", post.path.display()),
                    rel: "alternate".to_string(),
                    mime_type: Some("text/html".to_string()),
                    ..Default::default()
                }])
                .updated(updated)
                .content(Some(
                    atom_syndication::ContentBuilder::default()
                        .content_type(Some("html".to_string()))
                        .value(Some(post.body.clone().into_string()))
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
