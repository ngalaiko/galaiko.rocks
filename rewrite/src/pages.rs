use crate::assets;

pub fn posts() -> maud::Markup {
    let mut posts = assets::iter()
        .filter(|asset_path| asset_path.starts_with("/posts"))
        .filter(|asset_path| asset_path.extension().and_then(|e| e.to_str()) == Some("html"))
        .filter_map(|asset_path| assets::get(&asset_path).ok())
        .collect::<Vec<_>>();
    posts.sort_by(|a, b| b.created.cmp(&a.created));
    maud::html! {
        h1 {
            "Posts"
        }
        ul {
            @for post in posts {
                li {
                    a href=(post.path) {
                        (post.path.file_stem().unwrap().to_str().unwrap())
                    }
                    " "
                    time datetime=(post.created.format("%Y-%m-%d")) {
                        (post.created.format("%Y-%m-%d"))
                    }
                }
            }
        }
    }
}
