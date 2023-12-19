use crate::{path, routes};

pub async fn serve(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = routes::Routes::read_from_public();

    let mut app = tide::with_state(state);
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(serve_asset);
    app.at("/*path").get(serve_asset);

    app.listen(address).await?;

    Ok(())
}

async fn serve_asset(req: tide::Request<routes::Routes>) -> tide::Result {
    let requested_path = req.url().path();
    let requested_path = urlencoding::decode(requested_path)
        .map_err(|error| tide::Error::new(tide::StatusCode::BadRequest, error))?;

    let normalized_path = path::normalize(requested_path.to_string());

    if let Some(response) = req.state().get(&normalized_path) {
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
