pub fn iter() -> impl Iterator<Item = (std::path::PathBuf, Vec<u8>)> {
    Static::iter().map(|file| {
        (
            std::path::PathBuf::from(file.as_ref()),
            Static::get(file.as_ref()).unwrap().data.to_vec(),
        )
    })
}

#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Static;
