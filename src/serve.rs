use crate::{
    assets, cocktails, generated, movies, pages, path, posts, records, restaurands_and_cafes,
};

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub async fn serve(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let state = State::build()?;
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
fn watch_changes(state: Arc<RwLock<State>>) -> notify::Result<notify::RecommendedWatcher> {
    use notify::Watcher;

    let mut watcher = notify::recommended_watcher(move |_| {
        *state.write().unwrap() = State::build().unwrap();
    })?;
    watcher.watch(
        std::path::Path::new("./assets"),
        notify::RecursiveMode::Recursive,
    )?;
    Ok(watcher)
}

async fn serve_asset(req: tide::Request<Arc<RwLock<State>>>) -> tide::Result {
    let requested_path = req.url().path();
    let requested_path = urlencoding::decode(requested_path)
        .map_err(|error| tide::Error::new(tide::StatusCode::BadRequest, error))?;

    let normalized_path = path::normalize(requested_path.to_string());

    let state = req.state().read().expect("Error reading state");
    if let Some(response) = state.0.get(&normalized_path) {
        tide::Response::try_from(response)
    } else {
        Ok(tide::Response::new(tide::StatusCode::NotFound))
    }
}

#[derive(Debug, Clone)]
enum Response {
    Redirect(std::path::PathBuf),
    Content { mimetype: String, body: Vec<u8> },
}

impl TryFrom<&Response> for tide::Response {
    type Error = tide::Error;

    fn try_from(value: &Response) -> Result<Self, Self::Error> {
        match value {
            Response::Redirect(path) => {
                let mut response = tide::Response::new(tide::StatusCode::MovedPermanently);
                response.insert_header("Location", path.to_str().unwrap());
                Ok(response)
            }
            Response::Content { body, mimetype } => {
                Ok(tide::Response::builder(tide::StatusCode::Ok)
                    .header("content-type", mimetype)
                    .body(tide::Body::from(body.to_owned()))
                    .build())
            }
        }
    }
}

#[derive(Clone)]
struct State(Arc<HashMap<std::path::PathBuf, Response>>);

#[derive(Debug)]
pub enum BuildError {
    GetAsset(std::path::PathBuf, assets::GetAssetError),
    BuildPost(std::path::PathBuf, posts::FromError),
    BuildPage(std::path::PathBuf, pages::FromError),
    BuildCocktail(std::path::PathBuf, cocktails::FromError),
    BuildMovie(std::path::PathBuf, movies::FromError),
    BuildRecord(std::path::PathBuf, records::FromError),
    BuildPlace(std::path::PathBuf, restaurands_and_cafes::FromError),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::GetAsset(path, error) => {
                write!(f, "Error getting asset {}: {}", path.display(), error)
            }
            BuildError::BuildPost(path, error) => {
                write!(f, "Error building post {}: {}", path.display(), error)
            }
            BuildError::BuildPage(path, error) => {
                write!(f, "Error building page {}: {}", path.display(), error)
            }
            BuildError::BuildCocktail(path, error) => {
                write!(f, "Error building cocktail {}: {}", path.display(), error)
            }
            BuildError::BuildMovie(path, error) => {
                write!(f, "Error building movie {}: {}", path.display(), error)
            }
            BuildError::BuildRecord(path, error) => {
                write!(f, "Error building record {}: {}", path.display(), error)
            }
            BuildError::BuildPlace(path, error) => {
                write!(f, "Error building place {}: {}", path.display(), error)
            }
        }
    }
}

impl std::error::Error for BuildError {}

impl State {
    #[allow(clippy::too_many_lines)]
    fn build() -> Result<Self, BuildError> {
        let assets = assets::iter()
            .map(|asset_path| {
                assets::get(&asset_path)
                    .map_err(|error| BuildError::GetAsset(asset_path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (posts, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
            asset.path.starts_with("/posts/") && asset.mimetype == "text/markdown"
        });
        let posts = posts
            .iter()
            .map(|asset| {
                posts::Post::try_from(asset)
                    .map_err(|error| BuildError::BuildPost(asset.path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (cocktails, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
            asset.path.starts_with("/cocktails/") && asset.mimetype == "application/octet-stream"
        });
        let cocktails = cocktails
            .iter()
            .map(|asset| {
                cocktails::Cocktail::try_from(asset)
                    .map_err(|error| BuildError::BuildCocktail(asset.path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (movies, assets): (Vec<_>, Vec<_>) = assets
            .into_iter()
            .partition(|asset| asset.path.starts_with("/movies/"));
        let movies = movies
            .iter()
            .map(|asset| {
                movies::Entry::try_from(asset)
                    .map_err(|error| BuildError::BuildMovie(asset.path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (records, assets): (Vec<_>, Vec<_>) = assets
            .into_iter()
            .partition(|asset| asset.path.starts_with("/records/"));
        let records = records
            .iter()
            .map(|asset| {
                records::Record::try_from(asset)
                    .map_err(|error| BuildError::BuildRecord(asset.path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (places, assets): (Vec<_>, Vec<_>) = assets
            .into_iter()
            .partition(|asset| asset.path.starts_with("/restaurants_and_cafes/"));
        let places = places
            .iter()
            .map(|asset| {
                restaurands_and_cafes::Place::try_from(asset)
                    .map_err(|error| BuildError::BuildPlace(asset.path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let (pages, assets): (Vec<_>, Vec<_>) = assets
            .into_iter()
            .partition(|asset| asset.mimetype == "text/markdown");
        let pages = pages
            .iter()
            .map(|asset| {
                pages::Page::try_from(asset)
                    .map_err(|error| BuildError::BuildPage(asset.path.clone(), error))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let mut routes = HashMap::new();

        routes.insert(
            std::path::PathBuf::from("/posts/index.html"),
            Response::Content {
                mimetype: "text/html".to_string(),
                body: build_page(&generated::posts(&posts))
                    .into_string()
                    .as_bytes()
                    .to_vec(),
            },
        );

        routes.insert(
            std::path::PathBuf::from("/records/index.html"),
            Response::Content {
                mimetype: "text/html".to_string(),
                body: build_page(&generated::records(&records))
                    .into_string()
                    .as_bytes()
                    .to_vec(),
            },
        );

        routes.insert(
            std::path::PathBuf::from("/cocktails/index.html"),
            Response::Content {
                mimetype: "text/html".to_string(),
                body: build_page(&generated::cocktails(&cocktails))
                    .into_string()
                    .as_bytes()
                    .to_vec(),
            },
        );

        routes.insert(
            std::path::PathBuf::from("/restaurants_and_cafes/index.html"),
            Response::Content {
                mimetype: "text/html".to_string(),
                body: build_page(&generated::restaurants_and_cafes(&places))
                    .into_string()
                    .as_bytes()
                    .to_vec(),
            },
        );

        routes.insert(
            std::path::PathBuf::from("/movies/index.html"),
            Response::Content {
                mimetype: "text/html".to_string(),
                body: build_page(&generated::movies(&movies))
                    .into_string()
                    .as_bytes()
                    .to_vec(),
            },
        );

        for post in posts {
            for alias in &post.frontmatter.aliases {
                routes.insert(alias.clone(), Response::Redirect(post.path.clone()));
            }
            routes.insert(
                post.path.clone(),
                Response::Content {
                    mimetype: "text/html".to_string(),
                    body: build_page(&post.body).into_string().as_bytes().to_vec(),
                },
            );
        }

        for cocktail in cocktails {
            routes.insert(
                cocktail.path.clone(),
                Response::Content {
                    mimetype: "text/html".to_string(),
                    body: build_page(&cocktail.body).into_string().as_bytes().to_vec(),
                },
            );
        }

        for page in pages {
            routes.insert(
                page.path.clone(),
                Response::Content {
                    mimetype: "text/html".to_string(),
                    body: build_page(&page.body).into_string().as_bytes().to_vec(),
                },
            );
        }

        for rest in assets {
            routes.insert(
                rest.path.clone(),
                Response::Content {
                    mimetype: rest.mimetype.clone(),
                    body: rest.data.clone(),
                },
            );
        }

        Ok(Self(Arc::new(routes)))
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
