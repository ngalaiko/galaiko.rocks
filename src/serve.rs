use crate::{path, routes};

use std::sync::{Arc, RwLock};

pub async fn serve(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = routes::Routes::build()?;
    let state = Arc::new(RwLock::new(state));

    #[cfg(debug_assertions)]
    let _watcher = watch_changes(state.clone())?;

    let mut app = tide::with_state(state);
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(serve_asset);
    app.at("/*path").get(serve_asset);

    app.listen(address).await?;

    Ok(())
}

#[cfg(debug_assertions)]
fn watch_changes(state: Arc<RwLock<routes::Routes>>) -> notify::Result<notify::RecommendedWatcher> {
    use notify::Watcher;

    let mut watcher = notify::recommended_watcher(move |_| {
        *state.write().unwrap() = routes::Routes::build().unwrap();
    })?;
    watcher.watch(
        std::path::Path::new("./assets/"),
        notify::RecursiveMode::Recursive,
    )?;
    Ok(watcher)
}

async fn serve_asset(req: tide::Request<Arc<RwLock<routes::Routes>>>) -> tide::Result {
    let requested_path = req.url().path();
    let requested_path = urlencoding::decode(requested_path)
        .map_err(|error| tide::Error::new(tide::StatusCode::BadRequest, error))?;

    let normalized_path = path::normalize(requested_path.to_string());

    let state = req.state().read().expect("Error reading state");
    if let Some(response) = state.get(&normalized_path) {
        tide::Response::try_from(response)
    } else {
        Ok(tide::Response::new(tide::StatusCode::NotFound))
    }
}

impl TryFrom<&routes::Route> for tide::Response {
    type Error = tide::Error;

    fn try_from(value: &routes::Route) -> Result<Self, Self::Error> {
        match value {
            routes::Route::Redirect(path) => {
                let mut response = tide::Response::new(tide::StatusCode::MovedPermanently);
                response.insert_header("location", path.to_str().unwrap());
                Ok(response)
            }
            routes::Route::Content { body, mimetype } => {
                Ok(tide::Response::builder(tide::StatusCode::Ok)
                    .header("content-type", mimetype)
                    .body(tide::Body::from(body.to_owned()))
                    .build())
            }
        }
    }
}
