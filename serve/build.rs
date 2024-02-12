use shared::{
    assets, path, render,
    types::{self, cocktails, entries, images, movies, places, records},
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../assets");

    if let Err(err) = build() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

const WEBP_QUALITY: f32 = 95.0;

#[allow(clippy::too_many_lines)]
fn build() -> Result<(), BuildError<'static>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .map_err(|error| BuildError::Var("CARGO_MANIFEST_DIR", error))?;
    let input = manifest_dir.join("../assets/");
    let output = manifest_dir.join("public");

    let asset_paths = walkdir(&input).map_err(|error| BuildError::WalkDir(input.clone(), error))?;

    let assets = asset_paths
        .into_iter()
        .map(|path| {
            std::fs::read(&path)
                .map(|data| (path.clone(), data))
                .map_err(|error| BuildError::Read(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|(path, data)| assets::Asset {
            path: path
                .strip_prefix(&input)
                .expect("always inside root")
                .to_path_buf(),
            data,
        })
        .collect::<Vec<_>>();

    remove_dir_all(&output).map_err(|error| BuildError::Remove(output.clone(), error))?;

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

    let (post_images, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("posts/")
            && matches!(
                asset.path.extension(),
                Some(ext) if ext == "png" || ext == "jpg" || ext == "jpeg"
            )
    });
    let post_images = post_images
        .iter()
        .map(|asset| {
            types::images::Image::try_from(asset)
                .map(|image| image.resize(Some(800), None))
                .map_err(|err| BuildError::Image(asset.path.clone(), err))
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

    let (cocktail_images, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("cocktails/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("jpeg"))
    });
    let cocktail_images = cocktail_images
        .iter()
        .map(|asset| {
            types::images::Image::try_from(asset)
                .map(|image| vec![image.resize(Some(200), None), image.resize(Some(800), None)])
                .map_err(|err| BuildError::Image(asset.path.clone(), err))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

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

    let (movie_posters, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("movies/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("jpg"))
    });
    let movie_posters = movie_posters
        .iter()
        .map(|asset| {
            types::images::Image::try_from(asset)
                .map(|image| image.resize(Some(70), None))
                .map_err(|err| BuildError::Image(asset.path.clone(), err))
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

    let (record_covers, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("records/")
            && asset.path.extension() == Some(std::ffi::OsStr::new("jpeg"))
    });
    let record_covers = record_covers
        .iter()
        .map(|asset| {
            types::images::Image::try_from(asset)
                .map(|image| image.resize(Some(200), None))
                .map_err(|err| BuildError::Image(asset.path.clone(), err))
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
        render::html::posts(posts.as_slice())
            .into_string()
            .as_bytes(),
    )?;

    write(
        join(&output, "posts.atom"),
        "redirect: /posts/index.atom".as_bytes(),
    )?;
    write(
        join(&output, "posts/index.atom"),
        render::atom::posts(posts.as_slice()).to_string().as_bytes(),
    )?;

    write(
        join(&output, "records/index.html"),
        render::html::records(records.as_slice())
            .into_string()
            .as_bytes(),
    )?;

    for cover in record_covers {
        write(join(&output, &cover.path), &cover.webp(WEBP_QUALITY))?;
    }

    write(
        join(&output, "cocktails/index.html"),
        render::html::cocktails(cocktails.as_slice())
            .into_string()
            .as_bytes(),
    )?;
    for image in cocktail_images {
        write(join(&output, &image.path), &image.webp(WEBP_QUALITY))?;
    }

    write(
        join(&output, "restaurants_and_cafes/index.html"),
        "redirect: /places/index.html".as_bytes(),
    )?;
    write(
        join(&output, "places/index.html"),
        render::html::places(places.as_slice())
            .into_string()
            .as_bytes(),
    )?;

    write(
        join(&output, "movies/index.html"),
        render::html::movies(movies.as_slice())
            .into_string()
            .as_bytes(),
    )?;
    for poster in movie_posters {
        write(join(&output, &poster.path), &poster.webp(WEBP_QUALITY))?;
    }

    for post in posts {
        for alias in &post.frontmatter.aliases {
            write(
                join(&output, path::normalize(alias)),
                render::html::redirect(&post.path).into_string().as_bytes(),
            )?;
        }
        write(
            join(&output, &post.path),
            render::html::post(&post).into_string().as_bytes(),
        )?;
    }

    for image in post_images {
        write(join(&output, &image.path), &image.webp(WEBP_QUALITY))?;
    }

    for cocktail in cocktails {
        write(
            join(&output, &cocktail.path),
            render::html::cocktail(&cocktail).into_string().as_bytes(),
        )?;
    }

    for page in pages {
        write(
            join(&output, &page.path),
            render::html::entry(&page).into_string().as_bytes(),
        )?;
    }

    for rest in assets {
        write(join(&output, &rest.path), &rest.data)?;
    }

    Ok(())
}

fn walkdir<P: AsRef<std::path::Path>>(root: P) -> Result<Vec<std::path::PathBuf>, std::io::Error> {
    let root = root.as_ref();
    let mut stack = vec![root.to_path_buf()];
    let mut files = vec![];
    while let Some(item) = stack.pop() {
        if item.is_file() {
            files.push(item);
        } else if item.is_symlink() {
            stack.push(item.read_link()?);
        } else if item.is_dir() {
            for item in std::fs::read_dir(item)? {
                let item = item?;
                stack.push(item.path());
            }
        }
    }
    Ok(files)
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

fn write<'a, P: AsRef<std::path::Path>>(path: P, data: &[u8]) -> Result<(), BuildError<'a>> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|error| BuildError::CreateDir(path.to_path_buf(), error))?;
        }
    }
    std::fs::write(path, data).map_err(|error| BuildError::Write(path.to_path_buf(), error))?;
    Ok(())
}

fn remove_dir_all<P: AsRef<std::path::Path>>(path: P) -> Result<(), std::io::Error> {
    let path = path.as_ref();
    if path.exists() {
        std::fs::remove_dir_all(path)?;
    }
    Ok(())
}

#[derive(Debug)]
enum BuildError<'a> {
    Var(&'a str, std::env::VarError),
    Remove(std::path::PathBuf, std::io::Error),
    WalkDir(std::path::PathBuf, std::io::Error),
    CreateDir(std::path::PathBuf, std::io::Error),
    Read(std::path::PathBuf, std::io::Error),
    Write(std::path::PathBuf, std::io::Error),
    Entry(std::path::PathBuf, entries::FromError),
    Cocktail(std::path::PathBuf, cocktails::FromError),
    Movie(std::path::PathBuf, movies::FromError),
    Record(std::path::PathBuf, records::FromError),
    Place(std::path::PathBuf, places::FromError),
    Image(std::path::PathBuf, images::ImageError),
}

impl std::fmt::Display for BuildError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Var(name, error) => write!(f, "env variale {name}: {error}"),
            BuildError::WalkDir(path, error) => {
                write!(f, "walking {}: {error}", path.display())
            }
            BuildError::CreateDir(path, error) => {
                write!(f, "creating {}: {error}", path.display())
            }
            BuildError::Remove(path, error) => {
                write!(f, "removing {}: {error}", path.display())
            }
            BuildError::Write(path, error) => {
                write!(f, "writing {}: {error}", path.display())
            }
            BuildError::Read(path, error) => {
                write!(f, "reading {}: {error}", path.display())
            }
            BuildError::Entry(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Cocktail(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Movie(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Record(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Place(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Image(path, error) => write!(f, "{}: {error}", path.display()),
        }
    }
}

impl std::error::Error for BuildError<'_> {}
