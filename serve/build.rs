use shared::{
    assets, build, path,
    types::{cocktails, entries, movies, places, records},
};

#[derive(rust_embed::RustEmbed)]
#[folder = "../assets/"]
struct Assets;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../assets");

    if let Err(err) = build() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

#[derive(Debug)]
enum BuildError {
    Io(std::io::Error),
    Var(std::env::VarError),
    Entry(std::path::PathBuf, entries::FromError),
    Cocktail(std::path::PathBuf, cocktails::FromError),
    Movie(std::path::PathBuf, movies::FromError),
    Record(std::path::PathBuf, records::FromError),
    Place(std::path::PathBuf, places::FromError),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Io(error) => write!(f, "{error}"),
            BuildError::Var(error) => write!(f, "{error}"),
            BuildError::Entry(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Cocktail(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Movie(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Record(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Place(path, error) => write!(f, "{}: {error}", path.display()),
        }
    }
}

impl std::error::Error for BuildError {}

#[allow(clippy::too_many_lines)]
fn build() -> Result<(), BuildError> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").map_err(BuildError::Var)?;
    let output = std::path::PathBuf::from(manifest_dir).join("public");

    remove_dir_all(&output).map_err(BuildError::Io)?;

    let assets = Assets::iter()
        .filter_map(|asset_path| {
            Assets::get(&asset_path)
                .map(|asset| (std::path::PathBuf::from(asset_path.to_string()), asset))
        })
        .map(|(path, asset)| assets::Asset {
            path,
            data: asset.data.to_vec(),
        });

    let (posts, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("posts/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("md"))
    });
    let posts = posts
        .iter()
        .map(|asset| {
            entries::Entry::try_from(asset)
                .map_err(|err| BuildError::Entry(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (cocktails, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("cocktails/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("cook"))
    });
    let cocktails = cocktails
        .iter()
        .map(|asset| {
            cocktails::Cocktail::try_from(asset)
                .map_err(|err| BuildError::Cocktail(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (movies, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("movies/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("json"))
    });
    let movies = movies
        .iter()
        .map(|asset| {
            movies::Entry::try_from(asset).map_err(|err| BuildError::Movie(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (records, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("records/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("json"))
    });
    let records = records
        .iter()
        .map(|asset| {
            records::Record::try_from(asset)
                .map_err(|err| BuildError::Record(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (places, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("places/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("json"))
    });
    let places = places
        .iter()
        .map(|asset| {
            places::Place::try_from(asset).map_err(|err| BuildError::Place(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (pages, assets): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .partition(|asset| asset.path.extension() == Some(std::ffi::OsStr::new("md")));
    let pages = pages
        .iter()
        .map(|asset| {
            entries::Entry::try_from(asset)
                .map_err(|err| BuildError::Entry(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?;

    write(
        join(&output, "posts/index.html"),
        build::html::posts(posts.as_slice())
            .into_string()
            .as_bytes(),
    )
    .map_err(BuildError::Io)?;

    write(
        join(&output, "posts.atom"),
        "redirect: /posts/index.atom".as_bytes(),
    )
    .map_err(BuildError::Io)?;
    write(
        join(&output, "posts/index.atom"),
        build::atom::posts(posts.as_slice()).to_string().as_bytes(),
    )
    .map_err(BuildError::Io)?;

    write(
        join(&output, "records/index.html"),
        build::html::records(records.as_slice())
            .into_string()
            .as_bytes(),
    )
    .map_err(BuildError::Io)?;

    write(
        join(&output, "cocktails/index.html"),
        build::html::cocktails(cocktails.as_slice())
            .into_string()
            .as_bytes(),
    )
    .map_err(BuildError::Io)?;

    write(
        join(&output, "restaurants_and_cafes/index.html"),
        "redirect: /places/index.html".as_bytes(),
    )
    .map_err(BuildError::Io)?;
    write(
        join(&output, "places/index.html"),
        build::html::places(places.as_slice())
            .into_string()
            .as_bytes(),
    )
    .map_err(BuildError::Io)?;

    write(
        join(&output, "movies/index.html"),
        build::html::movies(movies.as_slice())
            .into_string()
            .as_bytes(),
    )
    .map_err(BuildError::Io)?;

    for post in posts {
        for alias in &post.frontmatter.aliases {
            write(
                join(&output, path::normalize(alias)),
                build::html::redirect(&post.path).into_string().as_bytes(),
            )
            .map_err(BuildError::Io)?;
        }
        write(
            join(&output, &post.path),
            build::html::post(&post).into_string().as_bytes(),
        )
        .map_err(BuildError::Io)?;
    }

    for cocktail in cocktails {
        write(
            join(&output, &cocktail.path),
            build::html::cocktail(&cocktail).into_string().as_bytes(),
        )
        .map_err(BuildError::Io)?;
    }

    for page in pages {
        write(
            join(&output, &page.path),
            build::html::entry(&page).into_string().as_bytes(),
        )
        .map_err(BuildError::Io)?;
    }

    for rest in assets {
        write(join(&output, &rest.path), &rest.data).map_err(BuildError::Io)?;
    }

    Ok(())
}

fn join<P, O>(path: P, other: O) -> std::path::PathBuf
where
    P: AsRef<std::path::Path>,
    O: AsRef<std::path::Path>,
{
    let other = other
        .as_ref()
        .components()
        .filter(|c| *c != std::path::Component::RootDir)
        .collect::<std::path::PathBuf>();
    path.as_ref().join(other)
}

fn write<P: AsRef<std::path::Path>>(path: P, data: &[u8]) -> Result<(), std::io::Error> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::write(path, data)?;
    Ok(())
}

fn remove_dir_all<P: AsRef<std::path::Path>>(path: P) -> Result<(), std::io::Error> {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}
