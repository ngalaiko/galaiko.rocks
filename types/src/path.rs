pub fn normalize<P: AsRef<std::path::Path>>(s: P) -> std::path::PathBuf {
    let mut path = s.as_ref().to_path_buf();
    if !path.has_root() {
        path = std::path::PathBuf::from("/").join(path);
    };
    match path.extension().and_then(|e| e.to_str()) {
        Some("md" | "cook") => {
            path.set_extension("html");
        }
        Some(_) => {}
        None => {
            path.push("index.html");
        }
    }
    path
}

#[test]
fn root() {
    let path = normalize("/");
    assert_eq!(path, std::path::PathBuf::from("/index.html"));
}

#[test]
fn empty() {
    let path = normalize("");
    assert_eq!(path, std::path::PathBuf::from("/index.html"));
}

#[test]
fn directory_with_slash() {
    let path = normalize("/posts/");
    assert_eq!(path, std::path::PathBuf::from("/posts/index.html"));
}

#[test]
fn directory_without_slash() {
    let path = normalize("/posts");
    assert_eq!(path, std::path::PathBuf::from("/posts/index.html"));
}

#[test]
fn md_root_file() {
    let path = normalize("/index.md");
    assert_eq!(path, std::path::PathBuf::from("/index.html"));
}

#[test]
fn cook_file() {
    let path = normalize("/posts/some title.cook");
    assert_eq!(path, std::path::PathBuf::from("/posts/some title.html"));
}
