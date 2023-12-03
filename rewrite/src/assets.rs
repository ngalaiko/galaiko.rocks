pub fn iter() -> impl Iterator<Item = (std::path::PathBuf, rust_embed::EmbeddedFile)> {
    Static::iter().map(|file| {
        (
            std::path::PathBuf::from(file.as_ref()),
            Static::get(file.as_ref()).unwrap(),
        )
    })
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Static;
