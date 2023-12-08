#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
struct Assets;

pub fn iter() -> impl Iterator<Item = Path> {
    Assets::iter().map(|asset_path| Path::from_str(&asset_path))
}

pub fn get(path: &Path) -> Result<Asset, GetAssetError> {
    let asset_path = Assets::iter()
        .find(|asset_path| Path::from_str(asset_path).eq(path))
        .ok_or_else(|| GetAssetError::NotFound(path.clone()))?;

    let embedded_file =
        Assets::get(&asset_path).expect("Assets::iter() returned a non-existent path");
    let asset = Asset {
        path: path.clone(),
        created: embedded_file.metadata.created().map_or(
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1).expect("valid date"),
            |created_ts| {
                chrono::naive::NaiveDateTime::from_timestamp_opt(created_ts.try_into().unwrap(), 0)
                    .expect("valid timestamp")
                    .date()
            },
        ),
        content: if asset_path.ends_with("index.md") {
            let html = convert_md(&path.with_extension(""), &embedded_file.data)
                .map_err(GetAssetError::Utf8Error)?;
            Content::Html(html)
        } else if asset_path.ends_with("md") {
            let html = convert_md(
                &path.parent().expect("expected parent"),
                &embedded_file.data,
            )
            .map_err(GetAssetError::Utf8Error)?;
            Content::Html(html)
        } else {
            Content::Binary {
                content_type: embedded_file.metadata.mimetype().to_string(),
                body: embedded_file.data.to_vec(),
            }
        },
    };

    Ok(asset)
}

#[derive(Debug)]
pub enum GetAssetError {
    NotFound(Path),
    Utf8Error(std::str::Utf8Error),
}

impl std::fmt::Display for GetAssetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetAssetError::NotFound(path) => {
                write!(f, "Asset not found: {}", path.to_str().unwrap())
            }
            GetAssetError::Utf8Error(err) => write!(f, "UTF-8 error: {err}"),
        }
    }
}

impl std::error::Error for GetAssetError {}

#[derive(Debug)]
pub struct Asset {
    pub path: Path,
    pub content: Content,
    pub created: chrono::NaiveDate,
}

#[derive(Debug)]
pub enum Content {
    Html(maud::Markup),
    Binary { content_type: String, body: Vec<u8> },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path(std::path::PathBuf);

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_str().unwrap().fmt(f)
    }
}

impl Path {
    pub fn starts_with<P: AsRef<std::path::Path>>(&self, other: P) -> bool {
        self.0.starts_with(other.as_ref())
    }

    pub fn extension(&self) -> Option<&std::ffi::OsStr> {
        self.0.extension()
    }

    fn with_extension<P: AsRef<std::path::Path>>(&self, extension: P) -> Path {
        Path(self.0.with_extension(extension.as_ref()))
    }

    fn parent(&self) -> Option<Path> {
        self.0.parent().map(|p| Path(p.to_path_buf()))
    }

    fn join<P: AsRef<std::path::Path>>(&self, other: P) -> Path {
        Path(self.0.join(other.as_ref()))
    }

    pub fn file_stem(&self) -> Option<&std::ffi::OsStr> {
        self.0.file_stem()
    }

    pub fn to_str(&self) -> Option<&str> {
        self.0.to_str()
    }
}

impl Path {
    pub fn from_str(s: &str) -> Self {
        if s.is_empty() || s == "/" || s == "/index.md" || s == "index.md" {
            return Path(std::path::PathBuf::from("/index.html"));
        }

        let path = std::path::PathBuf::from(s);

        let extension = path.extension();
        let file_stem = path.file_stem();
        let parent = path.parent();

        let mut new_path = std::path::PathBuf::new();
        new_path.push("/");
        if let Some(parent) = parent {
            for component in parent.components() {
                new_path.push(component);
            }
        }

        match extension.and_then(|e| e.to_str()) {
            Some("md") => match file_stem.and_then(|s| s.to_str()) {
                Some("index") => {
                    new_path.set_extension("html");
                }
                Some(file_stem) => {
                    new_path.push(file_stem);
                    new_path.set_extension("html");
                }
                None => unreachable!(),
            },
            Some(ext) => {
                if let Some(file_stem) = file_stem.and_then(|s| s.to_str()) {
                    new_path.push(file_stem);
                    new_path.set_extension(ext);
                } else {
                    unreachable!();
                }
            }
            None => {
                if let Some(file_stem) = file_stem.and_then(|s| s.to_str()) {
                    new_path.push(file_stem);
                    new_path.push("index.html");
                } else {
                    new_path.push("index.html");
                }
            }
        }

        Path(new_path)
    }
}

fn fix_link(root: &Path, link: &str) -> String {
    root.join(link).to_string()
}

fn convert_md(root: &Path, data: &[u8]) -> Result<maud::Markup, std::str::Utf8Error> {
    let md = std::str::from_utf8(data)?;
    let parser = pulldown_cmark::Parser::new(md);
    let parser = parser.map(|event| match event {
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(typ, link, title)) => {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                title,
            ))
        }
        pulldown_cmark::Event::End(pulldown_cmark::Tag::Image(typ, link, title)) => {
            pulldown_cmark::Event::End(pulldown_cmark::Tag::Image(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                title,
            ))
        }
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(typ, link, title)) => {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                title,
            ))
        }
        pulldown_cmark::Event::End(pulldown_cmark::Tag::Link(typ, link, title)) => {
            pulldown_cmark::Event::End(pulldown_cmark::Tag::Link(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(root, &link).into_boxed_str()),
                title,
            ))
        }
        _ => event,
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    Ok(maud::PreEscaped(html))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root() {
        let path = Path::from_str("/");
        assert_eq!(path.0, std::path::PathBuf::from("/index.html"));
    }

    #[test]
    fn empty() {
        let path = Path::from_str("");
        assert_eq!(path.0, std::path::PathBuf::from("/index.html"));
    }
    #[test]
    fn directory() {
        let path = Path::from_str("/posts/");
        assert_eq!(path.0, std::path::PathBuf::from("/posts/index.html"));
    }

    mod markdown {
        use super::*;

        #[test]
        fn root() {
            let path = Path::from_str("/index.md");
            assert_eq!(path.0, std::path::PathBuf::from("/index.html"));
        }

        #[test]
        fn directory() {
            let path = Path::from_str("/posts/some title/index.md");
            assert_eq!(path.0, std::path::PathBuf::from("/posts/some-title.html"));
        }
    }
}
