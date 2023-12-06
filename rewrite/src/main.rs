use warp::{filters::path::FullPath, reject, reply, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    // TODO: get only.
    let routes = warp::path::full().and_then(serve);

    println!("listening on http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

async fn serve(path: FullPath) -> Result<impl Reply, Rejection> {
    let path = normalize_path(path.as_str());
    if let Some(file) = Assets::get(&path) {
        if let Some("md") = std::path::Path::new(&path).extension().and_then(|s| s.to_str()) {
            let res = reply::Response::new(convert_md(&file).expect("TODO").into());
            Ok(res)
        } else {
            let res = reply::Response::new(file.data.into());
            Ok(res)
        }
    } else {
        Err(reject::not_found())
    }
}

fn convert_md(embeded_file: &rust_embed::EmbeddedFile) -> Result<Vec<u8>, std::str::Utf8Error> {
    let md = std::str::from_utf8(&embeded_file.data)?;
    let parser = pulldown_cmark::Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    let doc = maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="/index.css";
        }
        (maud::PreEscaped(html))
    };

    Ok(doc.into_string().as_bytes().to_owned())
}

fn normalize_path(path: &str) -> String {
    let path = path.strip_prefix('/').unwrap_or_default();
    if path.is_empty() {
        return "index.md".to_string();
    }

    let path = std::path::PathBuf::from(path);
    if path.extension().is_none() {
        let path = path.to_str().unwrap_or_default();
        if path.ends_with('/'){
            path.to_string() + "index.md"
        } else {
            path.to_string() + ".md"
        }
    } else {
        path.to_str().unwrap_or_default().to_string()
    }
}

#[test]
fn test_normalize_path() {
    for (from, expected) in [
        ("/", "index.html"),
        ("/index.html", "index.html"),
        ("/favicon.ico", "favicon.ico"),
        ("/posts/index.html", "posts/index.html"),
        ("/posts/", "posts/index.html"),
        ("/now", "now.html"),
    ] {
        assert_eq!(normalize_path(from), expected);
    }
}
