use lib::{assets, cocktails, entries, generated, movies, path, records, restaurands_and_cafes};

#[derive(rust_embed::RustEmbed)]
#[folder = "../assets/"]
struct Assets;

#[allow(clippy::too_many_lines)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../assets");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let output = std::path::PathBuf::from(manifest_dir).join("public");

    remove_dir_all(&output)?;

    let assets = Assets::iter()
        .filter_map(|asset_path| {
            Assets::get(&asset_path).map(|asset| (path::normalize(asset_path.to_string()), asset))
        })
        .map(|(path, asset)| assets::Asset {
            path,
            mimetype: asset.metadata.mimetype().to_string(),
            data: asset.data.to_vec(),
        });

    let (posts, assets): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .partition(|asset| asset.path.starts_with("/posts/") && asset.mimetype == "text/markdown");
    let posts = posts
        .iter()
        .map(entries::Entry::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let (cocktails, assets): (Vec<_>, Vec<_>) = assets.into_iter().partition(|asset| {
        asset.path.starts_with("/cocktails/") && asset.mimetype == "application/octet-stream"
    });
    let cocktails = cocktails
        .iter()
        .map(cocktails::Cocktail::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let (movies, assets): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .partition(|asset| asset.path.starts_with("/movies/"));
    let movies = movies
        .iter()
        .map(movies::Entry::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let (records, assets): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .partition(|asset| asset.path.starts_with("/records/"));
    let records = records
        .iter()
        .map(records::Record::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let (places, assets): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .partition(|asset| asset.path.starts_with("/restaurants_and_cafes/"));
    let places = places
        .iter()
        .map(restaurands_and_cafes::Place::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let (pages, assets): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .partition(|asset| asset.mimetype == "text/markdown");
    let pages = pages
        .iter()
        .map(entries::Entry::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    write(
        join(&output, "posts/index.html"),
        build_page(&generated::posts(&posts))
            .into_string()
            .as_bytes(),
    )?;

    write(
        join(&output, "records/index.html"),
        build_page(&generated::records(&records))
            .into_string()
            .as_bytes(),
    )?;

    write(
        join(&output, "cocktails/index.html"),
        build_page(&generated::cocktails(&cocktails))
            .into_string()
            .as_bytes(),
    )?;

    write(
        join(&output, "restaurants_and_cafes/index.html"),
        build_page(&generated::restaurants_and_cafes(&places))
            .into_string()
            .as_bytes(),
    )?;

    write(
        join(&output, "movies/index.html"),
        build_page(&generated::movies(&movies))
            .into_string()
            .as_bytes(),
    )?;

    for post in posts {
        for alias in &post.frontmatter.aliases {
            write(
                join(&output, path::normalize(alias)),
                format!("redirect: {}", post.path.display()).as_bytes(),
            )?;
        }
        write(
            join(&output, &post.path),
            build_page(&post.body).into_string().as_bytes(),
        )?;
    }

    for cocktail in cocktails {
        write(
            join(&output, &cocktail.path),
            build_page(&cocktail.body).into_string().as_bytes(),
        )?;
    }

    for page in pages {
        write(
            join(&output, &page.path),
            build_page(&page.body).into_string().as_bytes(),
        )?;
    }

    for rest in assets {
        write(join(&output, &rest.path), &rest.data)?;
    }

    Ok(())
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
