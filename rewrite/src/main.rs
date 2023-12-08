mod assets;
mod pages;

use warp::{filters::path::FullPath, reject, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "www=info");
    env_logger::init();

    // TODO: get only.
    let routes = warp::path::full()
        .and_then(serve_asset)
        .with(warp::log("www"));

    println!("listening on http://127.0.0.1:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

async fn serve_asset(requested_path: FullPath) -> Result<impl Reply, Rejection> {
    let requested_path = urlencoding::decode(requested_path.as_str()).map_err(|error| {
        log::error!("URL decoding error: {}", error);
        reject::not_found()
    })?;

    dbg!(&requested_path);

    let path = assets::Path::from_str(&requested_path);
    if path == assets::Path::from_str("/posts/index.html") {
        let page = build_page(&pages::posts());
        let response = warp::reply::Response::new(page.into_string().into());
        let reply = warp::reply::with_header(response, "content-type", "text/html; charset=utf-8");
        return Ok(reply);
    }

    let asset = assets::get(&path).map_err(|error| match error {
        assets::GetAssetError::NotFound(_) => reject::not_found(),
        assets::GetAssetError::Utf8Error(error) => {
            log::error!("UTF-8 error: {}", error);
            reject::not_found()
        }
    })?;

    match asset.content {
        assets::Content::Html(content) => {
            let page = build_page(&content);
            let response = warp::reply::Response::new(page.into_string().into());
            let reply =
                warp::reply::with_header(response, "content-type", "text/html; charset=utf-8");
            Ok(reply)
        }
        assets::Content::Binary { content_type, body } => {
            let response = warp::reply::Response::new(body.into());
            let reply = warp::reply::with_header(response, "content-type", content_type);
            Ok(reply)
        }
    }
}

fn build_page(content: &maud::Markup) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            link rel="stylesheet" href="/index.css";
        }
        main {
            article {
                (content)
            }
        }
    }
}
