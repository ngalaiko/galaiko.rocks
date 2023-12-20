#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn build() -> Result<(), Box<dyn std::error::Error>> {
    let assets = Assets::iter().map(|asset_path| {
        let embedded_file =
            Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");
        Asset {
            path: path::normalize(asset_path.to_string()),
            mimetype: embedded_file.metadata.mimetype().to_string(),
            data: embedded_file.data.to_vec(),
        }
    });
    //
    //
    //     let (posts, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
    //         asset.path.starts_with("/posts/") && asset.mimetype == "text/markdown"
    //     });
    //     let posts = posts
    //         .iter()
    //         .map(|asset| {
    //             posts::Post::try_from(asset)
    //                 .map_err(|error| BuildError::Post(asset.path.clone(), error))
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //
    //     let (cocktails, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
    //         asset.path.starts_with("/cocktails/") && asset.mimetype == "application/octet-stream"
    //     });
    //     let cocktails = cocktails
    //         .iter()
    //         .map(|asset| {
    //             cocktails::Cocktail::try_from(asset)
    //                 .map_err(|error| BuildError::Cocktail(asset.path.clone(), error))
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //
    //     let (movies, assets): (Vec<_>, Vec<_>) = assets
    //         .into_iter()
    //         .partition(|asset| asset.path.starts_with("/movies/"));
    //     let movies = movies
    //         .iter()
    //         .map(|asset| {
    //             movies::Entry::try_from(asset)
    //                 .map_err(|error| BuildError::Movie(asset.path.clone(), error))
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //
    //     let (records, assets): (Vec<_>, Vec<_>) = assets
    //         .into_iter()
    //         .partition(|asset| asset.path.starts_with("/records/"));
    //     let records = records
    //         .iter()
    //         .map(|asset| {
    //             records::Record::try_from(asset)
    //                 .map_err(|error| BuildError::Record(asset.path.clone(), error))
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //
    //     let (places, assets): (Vec<_>, Vec<_>) = assets
    //         .into_iter()
    //         .partition(|asset| asset.path.starts_with("/restaurants_and_cafes/"));
    //     let places = places
    //         .iter()
    //         .map(|asset| {
    //             restaurands_and_cafes::Place::try_from(asset)
    //                 .map_err(|error| BuildError::BuildPlace(asset.path.clone(), error))
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //
    //     let (pages, assets): (Vec<_>, Vec<_>) = assets
    //         .into_iter()
    //         .partition(|asset| asset.mimetype == "text/markdown");
    //     let pages = pages
    //         .iter()
    //         .map(|asset| {
    //             pages::Page::try_from(asset)
    //                 .map_err(|error| BuildError::Page(asset.path.clone(), error))
    //         })
    //         .collect::<Result<Vec<_>, _>>()?;
    //
    //     let mut routes = HashMap::new();
    //
    //     routes.insert(
    //         std::path::PathBuf::from("/posts/index.html"),
    //         Route::Content {
    //             mimetype: "text/html".to_string(),
    //             body: build_page(&generated::posts(&posts))
    //                 .into_string()
    //                 .as_bytes()
    //                 .to_vec(),
    //         },
    //     );
    //
    //     routes.insert(
    //         std::path::PathBuf::from("/records/index.html"),
    //         Route::Content {
    //             mimetype: "text/html".to_string(),
    //             body: build_page(&generated::records(&records))
    //                 .into_string()
    //                 .as_bytes()
    //                 .to_vec(),
    //         },
    //     );
    //
    //     routes.insert(
    //         std::path::PathBuf::from("/cocktails/index.html"),
    //         Route::Content {
    //             mimetype: "text/html".to_string(),
    //             body: build_page(&generated::cocktails(&cocktails))
    //                 .into_string()
    //                 .as_bytes()
    //                 .to_vec(),
    //         },
    //     );
    //
    //     routes.insert(
    //         std::path::PathBuf::from("/restaurants_and_cafes/index.html"),
    //         Route::Content {
    //             mimetype: "text/html".to_string(),
    //             body: build_page(&generated::restaurants_and_cafes(&places))
    //                 .into_string()
    //                 .as_bytes()
    //                 .to_vec(),
    //         },
    //     );
    //
    //     routes.insert(
    //         std::path::PathBuf::from("/movies/index.html"),
    //         Route::Content {
    //             mimetype: "text/html".to_string(),
    //             body: build_page(&generated::movies(&movies))
    //                 .into_string()
    //                 .as_bytes()
    //                 .to_vec(),
    //         },
    //     );
    //
    //     for post in posts {
    //         for alias in &post.frontmatter.aliases {
    //             routes.insert(alias.clone(), Route::Redirect(post.path.clone()));
    //         }
    //         routes.insert(
    //             post.path.clone(),
    //             Route::Content {
    //                 mimetype: "text/html".to_string(),
    //                 body: build_page(&post.body).into_string().as_bytes().to_vec(),
    //             },
    //         );
    //     }
    //
    //     for cocktail in cocktails {
    //         routes.insert(
    //             cocktail.path.clone(),
    //             Route::Content {
    //                 mimetype: "text/html".to_string(),
    //                 body: build_page(&cocktail.body).into_string().as_bytes().to_vec(),
    //             },
    //         );
    //     }
    //
    //     for page in pages {
    //         routes.insert(
    //             page.path.clone(),
    //             Route::Content {
    //                 mimetype: "text/html".to_string(),
    //                 body: build_page(&page.body).into_string().as_bytes().to_vec(),
    //             },
    //         );
    //     }
    //
    //     for rest in assets {
    //         routes.insert(
    //             rest.path.clone(),
    //             Route::Content {
    //                 mimetype: rest.mimetype.clone(),
    //                 body: rest.data.clone(),
    //             },
    //         );
    //     }
    //
    //     Ok(Self(routes))
    //
    // async_std::fs::remove_dir_all(output)
    //     .await
    //     .map_err(|error| Error::Io(output.to_path_buf(), error))?;
    //
    // let state = routes::Routes::build_from_assets().map_err(Error::Build)?;
    // for (path, route) in state.iter() {
    //     let path = path
    //         .components()
    //         .filter(|c| c != &std::path::Component::RootDir)
    //         .collect::<std::path::PathBuf>();
    //     let path = output.join(&path);
    //
    //     if let Some(parent) = path.parent() {
    //         async_std::fs::create_dir_all(parent)
    //             .await
    //             .map_err(|error| Error::Io(parent.to_path_buf(), error))?;
    //     }
    //
    //     match route {
    //         routes::Route::Content { body, .. } => {
    //             async_std::fs::write(&path, body)
    //                 .await
    //                 .map_err(|error| Error::Io(path, error))?;
    //         }
    //         routes::Route::Redirect(redirect_to) => {
    //             let path = path.with_extension("redirect");
    //             async_std::fs::write(&path, redirect_to.display().to_string())
    //                 .await
    //                 .map_err(|error| Error::Io(path, error))?;
    //         }
    //     }
    // }
    Ok(())
}
