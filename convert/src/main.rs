mod render;

use clap::Parser;

use types::{
    path, {cocktails, entries, images, movies, places, records},
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
    let reader = PrefixReader::new(input).await?;
    let writer = PrefixWriter::new(output).await?;
    writer.clean().await?;

    let asset_paths = reader.traverse().await?;

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

    let asset_paths = asset_paths.into_iter().collect::<Vec<_>>();

    futures::try_join!(
        build_posts(&reader, &writer, post_paths.as_slice()),
        build_images(&reader, &writer, post_image_paths.as_slice(), 800),
        build_cocktails(&reader, &writer, cocktail_paths.as_slice()),
        build_images(&reader, &writer, cocktail_image_paths.as_slice(), 800),
        build_images(&reader, &writer, cocktail_image_paths.as_slice(), 200),
        build_movies(&reader, &writer, movie_paths.as_slice()),
        build_images(&reader, &writer, movie_poster_paths.as_slice(), 70),
        build_places(&reader, &writer, place_paths.as_slice()),
        build_records(&reader, &writer, record_paths.as_slice()),
        build_images(&reader, &writer, record_cover_paths.as_slice(), 200),
        build_pages(&reader, &writer, page_paths.as_slice()),
        build_assets(&reader, &writer, asset_paths.as_slice()),
    )?;

    Ok(())
}

#[allow(clippy::ptr_arg)]
fn is_post_path(path: &std::path::PathBuf) -> bool {
    path.starts_with("posts") && path.extension() == Some(std::ffi::OsStr::new("md"))
}

#[allow(clippy::ptr_arg)]
fn is_cocktail_path(path: &std::path::PathBuf) -> bool {
    path.starts_with("cocktails") && path.extension() == Some(std::ffi::OsStr::new("cook"))
}

#[allow(clippy::ptr_arg)]
fn is_movie_path(path: &std::path::PathBuf) -> bool {
    path.starts_with("movies") && path.extension() == Some(std::ffi::OsStr::new("json"))
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
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let post_assets = paths.iter().map(|path| reader.read(path));
    let post_assets = futures::future::try_join_all(post_assets).await?;
    let posts = post_assets
        .iter()
        .zip(paths)
        .map(|(data, path)| {
            entries::Entry::try_from(data.as_slice())
                .map_err(ConvertError::Entry)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let paths = paths.iter().map(path::normalize).collect::<Vec<_>>();
    let posts = posts.into_iter().zip(paths).collect::<Vec<_>>();

    writer
        .write(
            "posts/index.html",
            render::html::posts(posts.as_slice()).into_string(),
        )
        .await?;

    writer
        .write(
            "posts.atom",
            render::html::redirect(path::normalize("posts/index.atom")).into_string(),
        )
        .await?;

    writer
        .write(
            "posts/index.atom",
            render::atom::posts(posts.as_slice()).to_string(),
        )
        .await?;

    for (post, path) in posts {
        for alias in &post.frontmatter.aliases {
            writer
                .write(
                    path::normalize(alias),
                    render::html::redirect(&path).into_string(),
                )
                .await?;
        }
        writer
            .write(&path, render::html::post(&post).into_string())
            .await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all, fields(path = %path.display()))]
async fn build_image(
    reader: &PrefixReader,
    path: &std::path::Path,
    width: u32,
) -> Result<Vec<u8>, BuildError> {
    let (send, recv) = tokio::sync::oneshot::channel();

    let data = reader.read(&path).await?;

    rayon::spawn(move || {
        let image = images::Image::try_from(data.as_slice())
            .map(|image| image.resize(Some(width), None).webp(95.0));
        let _ = send.send(image);
    });

    let image = recv
        .await
        .expect("Panic in rayon::spawn")
        .map_err(ConvertError::Image)
        .map_err(|error| BuildError::Convert(path.to_path_buf(), error))?;

    Ok(image)
}

#[tracing::instrument(skip_all)]
async fn build_images(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
    width: u32,
) -> Result<(), BuildError> {
    for path in paths {
        let image = build_image(reader, path, width).await?;
        let path = {
            let file_stem = path
                .file_stem()
                .and_then(|file_stem| file_stem.to_str())
                .unwrap_or_default();
            path.with_file_name(format!("{file_stem}.{width}x0@2x.webp"))
        };
        writer.write(path::normalize(path), &image).await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_cocktails(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let assets = paths.iter().map(|path| reader.read(path));
    let assets = futures::future::try_join_all(assets).await?;
    let cocktails = assets
        .iter()
        .zip(paths)
        .map(|(data, path)| {
            cocktails::Cocktail::try_from(data.as_slice())
                .map_err(ConvertError::Cocktail)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let cocktails = cocktails.into_iter().zip(paths).collect::<Vec<_>>();

    writer
        .write(
            "cocktails/index.html",
            render::html::cocktails(cocktails.as_slice()).into_string(),
        )
        .await?;

    for (cocktail, path) in cocktails {
        writer
            .write(
                &path::normalize(path),
                render::html::cocktail(&cocktail).into_string().as_bytes(),
            )
            .await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_movies(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let assets = paths.iter().map(|path| reader.read(path));
    let assets = futures::future::try_join_all(assets).await?;
    let movies = assets
        .iter()
        .zip(paths)
        .map(|(data, path)| {
            movies::Entry::try_from(data.as_slice())
                .map_err(ConvertError::Movie)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    writer
        .write(
            "movies/index.html",
            render::html::movies(movies.as_slice()).into_string(),
        )
        .await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_records(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let assets = paths.iter().map(|path| reader.read(path));
    let assets = futures::future::try_join_all(assets).await?;
    let records = assets
        .iter()
        .zip(paths)
        .map(|(data, path)| {
            records::Record::try_from(data.as_slice())
                .map_err(ConvertError::Record)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    writer
        .write(
            "records/index.html",
            render::html::records(records.as_slice()).into_string(),
        )
        .await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_places(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let assets = paths.iter().map(|path| reader.read(path));
    let assets = futures::future::try_join_all(assets).await?;
    let places = assets
        .iter()
        .zip(paths)
        .map(|(data, path)| {
            places::Place::try_from(data.as_slice())
                .map_err(ConvertError::Place)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    writer
        .write(
            "restaurants_and_cafes/index.html",
            render::html::redirect(path::normalize("places/index.html")).into_string(),
        )
        .await?;

    writer
        .write(
            "places/index.html",
            render::html::places(places.as_slice()).into_string(),
        )
        .await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_pages(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    let assets = paths.iter().map(|path| reader.read(path));
    let assets = futures::future::try_join_all(assets).await?;
    let pages = assets
        .iter()
        .zip(paths)
        .map(|(data, path)| {
            entries::Entry::try_from(data.as_slice())
                .map_err(ConvertError::Entry)
                .map_err(|error| BuildError::Convert(path.clone(), error))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let pages = pages.into_iter().zip(paths).collect::<Vec<_>>();

    for (page, path) in pages {
        writer
            .write(
                path::normalize(path),
                render::html::entry(&page).into_string(),
            )
            .await?;
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn build_assets(
    reader: &PrefixReader,
    writer: &PrefixWriter,
    paths: &[std::path::PathBuf],
) -> Result<(), BuildError> {
    for path in paths {
        let data = reader.read(&path).await?;
        writer.write(path::normalize(path), data).await?;
    }
    Ok(())
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

struct PrefixReader(std::path::PathBuf);

impl PrefixReader {
    pub async fn new<P: AsRef<std::path::Path>>(prefix: P) -> Result<Self, BuildError> {
        tokio::fs::canonicalize(prefix.as_ref())
            .await
            .map_err(IOError::Canonicalize)
            .map_err(|error| BuildError::IO(prefix.as_ref().to_path_buf(), error))
            .map(Self)
    }

    #[tracing::instrument(skip(self), fields(path = %path.as_ref().display()))]
    pub async fn read<P: AsRef<std::path::Path>>(&self, path: P) -> Result<Vec<u8>, BuildError> {
        let path = self.0.join(path);

        tokio::fs::read(&path)
            .await
            .map_err(IOError::Read)
            .map_err(|error| BuildError::IO(path.clone(), error))
    }

    #[tracing::instrument(skip(self), fields(path = %self.0.display()))]
    pub async fn traverse(&self) -> Result<Vec<std::path::PathBuf>, BuildError> {
        let mut stack = vec![self.0.clone()];
        let mut files = vec![];
        while let Some(item) = stack.pop() {
            let metadata = tokio::fs::metadata(&item)
                .await
                .map_err(IOError::Metadata)
                .map_err(|error| BuildError::IO(item.clone(), error))?;
            if metadata.is_file() {
                let item = item
                    .strip_prefix(&self.0)
                    .expect("input is always a prefix");
                files.push(item.to_path_buf());
            } else if metadata.is_symlink() {
                stack.push(
                    tokio::fs::read_link(&item)
                        .await
                        .map_err(IOError::ReadLink)
                        .map_err(|error| BuildError::IO(item.clone(), error))?,
                );
            } else if metadata.is_dir() {
                let mut entries = tokio::fs::read_dir(&item)
                    .await
                    .map_err(IOError::ReadDir)
                    .map_err(|error| BuildError::IO(item.clone(), error))?;

                while let Some(entry) = entries
                    .next_entry()
                    .await
                    .map_err(IOError::ReadDir)
                    .map_err(|error| BuildError::IO(item.clone(), error))?
                {
                    stack.push(entry.path());
                }
            }
        }
        Ok(files)
    }
}

struct PrefixWriter(std::path::PathBuf);

impl PrefixWriter {
    pub async fn new<P: AsRef<std::path::Path>>(prefix: P) -> Result<Self, BuildError> {
        tokio::fs::create_dir_all(&prefix)
            .await
            .map_err(IOError::CreateDirAll)
            .map_err(|error| BuildError::IO(prefix.as_ref().to_path_buf(), error))?;

        tokio::fs::canonicalize(prefix.as_ref())
            .await
            .map_err(IOError::Canonicalize)
            .map_err(|error| BuildError::IO(prefix.as_ref().to_path_buf(), error))
            .map(Self)
    }

    pub async fn clean(&self) -> Result<(), BuildError> {
        match tokio::fs::remove_dir_all(&self.0).await {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(IOError::Remove(error)),
        }
        .map_err(|error| BuildError::IO(self.0.clone(), error))
    }

    #[tracing::instrument(skip(self, contents), fields(path = %path.as_ref().display()))]
    pub async fn write(
        &self,
        path: impl AsRef<std::path::Path>,
        contents: impl AsRef<[u8]>,
    ) -> Result<(), BuildError> {
        let path = path.as_ref();
        let path = self.0.join(
            path.components()
                .filter(|c| *c != std::path::Component::RootDir)
                .collect::<std::path::PathBuf>(),
        );

        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(IOError::CreateDirAll)
                .map_err(|error| BuildError::IO(parent.to_path_buf(), error))?;
        }
        tokio::fs::write(&path, contents)
            .await
            .map_err(IOError::Write)
            .map_err(|error| BuildError::IO(path.clone(), error))
    }
}
