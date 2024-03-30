mod render;

use clap::Parser;
use futures::future::TryFutureExt;

use shared::{
    path,
    types::{cocktails, entries, images, movies, places, records},
};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "./assets")]
    source: std::path::PathBuf,
    #[arg(long, default_value = "./serve/public")]
    destination: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::fmt::fmt()
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();
    if let Err(err) = convert(cli.source, cli.destination).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}

async fn convert<P: AsRef<std::path::Path>>(input: P, output: P) -> Result<(), BuildError> {
    let input = tokio::fs::canonicalize(input.as_ref())
        .await
        .map_err(IOError::Canonicalize)
        .map_err(|error| BuildError::IO(input.as_ref().to_path_buf(), error))?;

    tokio::fs::create_dir_all(&output)
        .await
        .map_err(IOError::CreateDirAll)
        .map_err(|error| BuildError::IO(output.as_ref().to_path_buf(), error))?;
    let output = tokio::fs::canonicalize(output.as_ref())
        .await
        .map_err(IOError::Canonicalize)
        .map_err(|error| BuildError::IO(output.as_ref().to_path_buf(), error))?;

    let asset_paths = traverse(&input)
        .await
        .map_err(|error| BuildError::IO(input.clone(), error))?;

    remove_dir_all(&output)
        .await
        .map_err(|error| BuildError::IO(output.clone(), error))?;

    let (post_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_post_path);

    let (post_image_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_post_image);

    let (cocktail_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_cocktail_path);

    let (cocktail_image_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_cocktail_image);

    let (movie_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_movie_path);

    let (movie_poster_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_movie_poster);

    let (record_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_record_path);

    let (record_cover_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_record_cover);

    let (place_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_place_path);

    let (page_paths, asset_paths): (Vec<_>, Vec<_>) =
        asset_paths.into_iter().partition(is_page_path);

    let asset_paths = asset_paths
        .into_iter()
        .map(|path| {
            path.strip_prefix(&input)
                .expect("input is always a prefix")
                .to_path_buf()
        })
        .collect::<Vec<_>>();

    futures::try_join!(
        build_posts(&input, &output, post_paths.as_slice()),
        build_images(&input, &output, post_image_paths.as_slice(), 800),
        build_cocktails(&input, &output, cocktail_paths.as_slice()),
        build_images(&input, &output, cocktail_image_paths.as_slice(), 800),
        build_images(&input, &output, cocktail_image_paths.as_slice(), 200),
        build_movies(&input, &output, movie_paths.as_slice()),
        build_images(&input, &output, movie_poster_paths.as_slice(), 70),
        build_places(&input, &output, place_paths.as_slice()),
        build_records(&input, &output, record_paths.as_slice()),
        build_images(&input, &output, record_cover_paths.as_slice(), 200),
        build_pages(&input, &output, page_paths.as_slice()),
        build_assets(&input, &output, asset_paths.as_slice()),
    )?;

    Ok(())
}

#[allow(clippy::ptr_arg)]
fn is_post_path(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("md"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("posts")))
}

#[allow(clippy::ptr_arg)]
fn is_cocktail_path(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("cook"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("cocktails")))
}

#[allow(clippy::ptr_arg)]
fn is_movie_path(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("json"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("movies")))
}

#[allow(clippy::ptr_arg)]
fn is_record_path(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("json"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("records")))
}

#[allow(clippy::ptr_arg)]
fn is_place_path(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("json"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("places")))
}

#[allow(clippy::ptr_arg)]
fn is_page_path(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("md"))
        && !path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("posts")))
}

#[allow(clippy::ptr_arg)]
fn is_post_image(path: &std::path::PathBuf) -> bool {
    (path.extension() == Some(std::ffi::OsStr::new("png"))
        || path.extension() == Some(std::ffi::OsStr::new("jpg"))
        || path.extension() == Some(std::ffi::OsStr::new("jpeg")))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("posts")))
}

#[allow(clippy::ptr_arg)]
fn is_cocktail_image(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("jpeg"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("cocktails")))
}

#[allow(clippy::ptr_arg)]
fn is_movie_poster(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("jpg"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("movies")))
}

#[allow(clippy::ptr_arg)]
fn is_record_cover(path: &std::path::PathBuf) -> bool {
    path.extension() == Some(std::ffi::OsStr::new("jpeg"))
        && path
            .components()
            .any(|c| c == std::path::Component::Normal(std::ffi::OsStr::new("records")))
}

#[tracing::instrument(skip_all)]
async fn build_posts(
    input: &std::path::Path,
    output: &std::path::Path,
    post_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let post_assets = post_paths
        .iter()
        .map(|path| read_file(input, path).map_err(|error| BuildError::IO(path.clone(), error)));
    let post_assets = futures::future::try_join_all(post_assets).await?;
    let posts = post_assets
        .iter()
        .map(|(path, data)| {
            entries::Entry::try_from(data.as_slice())
                .map(|entry| (path::normalize(path), entry))
                .map_err(ConvertError::Entry)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let posts_index_path = join(output, "posts/index.html");
    write_file(
        &posts_index_path,
        render::html::posts(posts.as_slice())
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(posts_index_path, error))?;

    let old_posts_atom_path = join(output, "posts.atom");
    write_file(
        &old_posts_atom_path,
        render::html::redirect(path::normalize("posts/index.atom"))
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(old_posts_atom_path, error))?;

    let posts_atom_path = join(output, "posts/index.atom");
    write_file(
        &posts_atom_path,
        render::atom::posts(posts.as_slice()).to_string().as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(posts_atom_path, error))?;

    for (path, post) in posts {
        for alias in &post.frontmatter.aliases {
            let alias_path = join(output, path::normalize(alias));
            write_file(
                &alias_path,
                render::html::redirect(&path).into_string().as_bytes(),
            )
            .await
            .map_err(|error| BuildError::IO(alias_path, error))?;
        }
        let post_path = join(output, &path);
        write_file(
            &post_path,
            render::html::post(&post).into_string().as_bytes(),
        )
        .await
        .map_err(|error| BuildError::IO(post_path, error))?;
    }
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.display()))]
async fn build_image(
    input: &std::path::Path,
    path: &std::path::Path,
    width: u32,
) -> Result<(std::path::PathBuf, Vec<u8>), BuildError> {
    let (send, recv) = tokio::sync::oneshot::channel();

    let (path, data) = read_file(input, path)
        .await
        .map_err(|error| BuildError::IO(path.to_path_buf(), error))?;

    rayon::spawn(move || {
        let image = images::Image::try_from(data.as_slice())
            .map(|image| image.resize(Some(width), None).webp(95.0));
        let _ = send.send(image);
    });

    let image = recv
        .await
        .expect("Panic in rayon::spawn")
        .map_err(ConvertError::Image)
        .map_err(|error| BuildError::Convert(path.clone(), error))?;

    let path = {
        let file_stem = path
            .file_stem()
            .and_then(|file_stem| file_stem.to_str())
            .unwrap_or_default();
        path.with_file_name(format!("{file_stem}.{width}x0@2x.webp"))
    };
    Ok((path::normalize(path), image))
}

#[tracing::instrument(skip_all)]
async fn build_images(
    input: &std::path::Path,
    output: &std::path::Path,
    image_paths: &[std::path::PathBuf],
    width: u32,
) -> Result<(), BuildError> {
    let images = image_paths
        .iter()
        .map(|path| build_image(input, path, width));
    let images = futures::future::try_join_all(images).await?;

    for (path, image) in images {
        let image_path = join(output, &path);
        write_file(&image_path, &image)
            .await
            .map_err(|error| BuildError::IO(image_path, error))?;
    }
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.display()))]
async fn build_cocktail(
    input: &std::path::Path,
    path: &std::path::Path,
) -> Result<(std::path::PathBuf, cocktails::Cocktail), BuildError> {
    read_file(input, path)
        .map_err(|error| BuildError::IO(path.to_path_buf(), error))
        .await
        .and_then(|(path, data)| {
            cocktails::Cocktail::try_from(data.as_slice())
                .map(|cocktail| (path::normalize(&path), cocktail))
                .map_err(ConvertError::Cocktail)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
}

#[tracing::instrument(skip_all)]
async fn build_cocktails(
    input: &std::path::Path,
    output: &std::path::Path,
    cocktail_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let cocktails = cocktail_paths
        .iter()
        .map(|path| build_cocktail(input, path));
    let cocktails = futures::future::try_join_all(cocktails).await?;
    let cocktails_index_path = join(output, "cocktails/index.html");
    write_file(
        &cocktails_index_path,
        render::html::cocktails(cocktails.as_slice())
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(cocktails_index_path, error))?;
    for (path, cocktail) in cocktails {
        let cocktail_path = join(output, &path);
        write_file(
            &cocktail_path,
            render::html::cocktail(&cocktail).into_string().as_bytes(),
        )
        .await
        .map_err(|error| BuildError::IO(cocktail_path, error))?;
    }
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.display()))]
async fn build_movie(
    input: &std::path::Path,
    path: &std::path::Path,
) -> Result<(std::path::PathBuf, movies::Entry), BuildError> {
    read_file(input, path)
        .map_err(|error| BuildError::IO(path.to_path_buf(), error))
        .await
        .and_then(|(path, data)| {
            movies::Entry::try_from(data.as_slice())
                .map(|movie| (path::normalize(&path), movie))
                .map_err(ConvertError::Movie)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
}

#[tracing::instrument(skip_all)]
async fn build_movies(
    input: &std::path::Path,
    output: &std::path::Path,
    movie_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let movies = movie_paths.iter().map(|path| build_movie(input, path));
    let movies = futures::future::try_join_all(movies).await?;
    let movies_index_path = join(output, "movies/index.html");
    write_file(
        &movies_index_path,
        render::html::movies(movies.as_slice())
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(movies_index_path, error))?;
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.display()))]
async fn build_record(
    input: &std::path::Path,
    path: &std::path::Path,
) -> Result<(std::path::PathBuf, records::Record), BuildError> {
    read_file(input, path)
        .map_err(|error| BuildError::IO(path.to_path_buf(), error))
        .await
        .and_then(|(path, data)| {
            records::Record::try_from(data.as_slice())
                .map(|record| (path::normalize(&path), record))
                .map_err(ConvertError::Record)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
}

#[tracing::instrument(skip_all)]
async fn build_records(
    input: &std::path::Path,
    output: &std::path::Path,
    record_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let records = record_paths.iter().map(|path| build_record(input, path));
    let records = futures::future::try_join_all(records).await?;
    let records_index_path = join(output, "records/index.html");
    write_file(
        &records_index_path,
        render::html::records(records.as_slice())
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(records_index_path.clone(), error))?;
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.display()))]
async fn build_place(
    input: &std::path::Path,
    path: &std::path::Path,
) -> Result<(std::path::PathBuf, places::Place), BuildError> {
    read_file(input, path)
        .map_err(|error| BuildError::IO(path.to_path_buf(), error))
        .await
        .and_then(|(path, data)| {
            places::Place::try_from(data.as_slice())
                .map(|place| (path::normalize(&path), place))
                .map_err(ConvertError::Place)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
}

#[tracing::instrument(skip_all)]
async fn build_places(
    input: &std::path::Path,
    output: &std::path::Path,
    place_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let places = place_paths.iter().map(|path| build_place(input, path));
    let places = futures::future::try_join_all(places).await?;
    let old_places_index_path = join(output, "restaurants_and_cafes/index.html");
    write_file(
        &old_places_index_path,
        render::html::redirect(path::normalize("restaurants_and_cafes/index.html"))
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(old_places_index_path.clone(), error))?;
    let places_index_path = join(output, "places/index.html");
    write_file(
        &places_index_path,
        render::html::places(places.as_slice())
            .into_string()
            .as_bytes(),
    )
    .await
    .map_err(|error| BuildError::IO(places_index_path.clone(), error))?;
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_pages(
    input: &std::path::Path,
    output: &std::path::Path,
    page_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let page_assets = page_paths
        .iter()
        .map(|path| read_file(input, path).map_err(|error| BuildError::IO(path.clone(), error)));
    let page_assets = futures::future::try_join_all(page_assets).await?;
    let pages = page_assets
        .iter()
        .map(|(path, data)| {
            entries::Entry::try_from(data.as_slice())
                .map(|entry| (path::normalize(path), entry))
                .map_err(ConvertError::Entry)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    for (path, page) in pages {
        let path = join(output, &path);
        write_file(&path, render::html::entry(&page).into_string().as_bytes())
            .await
            .map_err(|error| BuildError::IO(path.clone(), error))?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_assets(
    input: &std::path::Path,
    output: &std::path::Path,
    asset_paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    futures::future::try_join_all(
        asset_paths
            .iter()
            .map(|path| copy_file(input.join(path), path::normalize(output.join(path)))),
    )
    .await?;
    Ok(())
}

#[tracing::instrument(skip_all, fields(source = %source.as_ref().display(), destination = %destination.as_ref().display()))]
async fn copy_file<P: AsRef<std::path::Path>>(source: P, destination: P) -> Result<(), BuildError> {
    if let Some(parent) = destination.as_ref().parent() {
        if !parent.exists() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(IOError::CreateDirAll)
                .map_err(|error| BuildError::IO(parent.to_path_buf(), error))?;
        }
    }
    tokio::fs::copy(&source, &destination)
        .await
        .map_err(IOError::Copy)
        .map_err(|error| BuildError::IO(destination.as_ref().to_path_buf(), error))?;
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.as_ref().display()))]
async fn remove_dir_all<P: AsRef<std::path::Path>>(path: P) -> Result<(), IOError> {
    match tokio::fs::remove_dir_all(path).await {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(IOError::Remove(error)),
    }
}

#[tracing::instrument(skip_all, fields(path = %path.as_ref().display()))]
async fn read_file<P: AsRef<std::path::Path>>(
    root: P,
    path: P,
) -> Result<(std::path::PathBuf, Vec<u8>), IOError> {
    tokio::fs::read(&path)
        .await
        .map(|data| {
            (
                path.as_ref()
                    .strip_prefix(root)
                    .expect("always inside root")
                    .to_path_buf(),
                data,
            )
        })
        .map_err(IOError::Read)
}

#[tracing::instrument(skip_all, fields(path = %path.as_ref().display()))]
async fn write_file<P: AsRef<std::path::Path>>(path: P, data: &[u8]) -> Result<(), IOError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(IOError::CreateDirAll)?;
    }
    tokio::fs::write(path, data).await.map_err(IOError::Write)
}

#[tracing::instrument(fields(path = %path.as_ref().display()))]
async fn traverse<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<std::path::PathBuf>, IOError> {
    let root = path.as_ref();
    let mut stack = vec![root.to_path_buf()];
    let mut files = vec![];
    while let Some(item) = stack.pop() {
        let metadata = tokio::fs::metadata(&item)
            .await
            .map_err(IOError::Metadata)?;
        if metadata.is_file() {
            files.push(item);
        } else if metadata.is_symlink() {
            stack.push(
                tokio::fs::read_link(item)
                    .await
                    .map_err(IOError::ReadLink)?,
            );
        } else if metadata.is_dir() {
            let mut entries = tokio::fs::read_dir(item).await.map_err(IOError::ReadDir)?;

            while let Some(entry) = entries.next_entry().await.map_err(IOError::ReadDir)? {
                stack.push(entry.path());
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

#[derive(Debug)]
pub enum IOError {
    ReadDir(std::io::Error),
    ReadLink(std::io::Error),
    Metadata(std::io::Error),
    Read(std::io::Error),
    Write(std::io::Error),
    CreateDirAll(std::io::Error),
    Remove(std::io::Error),
    Copy(std::io::Error),
    Canonicalize(std::io::Error),
}

impl std::fmt::Display for IOError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            IOError::ReadDir(error) => write!(f, "read dir: {error}"),
            IOError::ReadLink(error) => write!(f, "read link: {error}"),
            IOError::Metadata(error) => write!(f, "metadata: {error}"),
            IOError::Read(error) => write!(f, "read: {error}"),
            IOError::Write(error) => write!(f, "write: {error}"),
            IOError::CreateDirAll(error) => write!(f, "create dir: {error}"),
            IOError::Remove(error) => write!(f, "remove: {error}"),
            IOError::Copy(error) => write!(f, "copy: {error}"),
            IOError::Canonicalize(error) => write!(f, "canonicalize: {error}"),
        }
    }
}

impl std::error::Error for IOError {}

#[derive(Debug)]
pub enum ConvertError {
    Entry(entries::FromError),
    Cocktail(cocktails::FromError),
    Movie(movies::FromError),
    Record(records::FromError),
    Place(places::FromError),
    Image(images::ImageError),
}

impl std::fmt::Display for ConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConvertError::Entry(error) => write!(f, "entry: {error}"),
            ConvertError::Cocktail(error) => write!(f, "cocktail: {error}"),
            ConvertError::Movie(error) => write!(f, "movie: {error}"),
            ConvertError::Record(error) => write!(f, "record: {error}"),
            ConvertError::Place(error) => write!(f, "place: {error}"),
            ConvertError::Image(error) => write!(f, "image: {error}"),
        }
    }
}

impl std::error::Error for ConvertError {}

#[derive(Debug)]
pub enum BuildError {
    IO(std::path::PathBuf, IOError),
    Convert(std::path::PathBuf, ConvertError),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::IO(path, error) => write!(f, "{}: {error}", path.display()),
            BuildError::Convert(path, error) => write!(f, "{}: {error}", path.display()),
        }
    }
}

impl std::error::Error for BuildError {}
