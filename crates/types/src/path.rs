pub fn normalize<P: AsRef<std::path::Path>>(path: P) -> std::path::PathBuf {
    let path = add_root(path);
    let path = change_extension(path);
    remove_cur_dir(path)
}

fn add_root<P: AsRef<std::path::Path>>(path: P) -> std::path::PathBuf {
    let path = path.as_ref();
    if path.has_root() {
        path.to_path_buf()
    } else {
        std::path::PathBuf::from("/").join(path)
    }
}

fn change_extension<P: AsRef<std::path::Path>>(path: P) -> std::path::PathBuf {
    let path = path.as_ref();
    match path.extension().and_then(|e| e.to_str()) {
        Some("md" | "cook") => path.with_extension("html"),
        Some(_) => path.to_path_buf(),
        None => path.join("index.html"),
    }
}

fn remove_cur_dir<P: AsRef<std::path::Path>>(path: P) -> std::path::PathBuf {
    path.as_ref()
        .components()
        .filter(|component| !matches!(component, std::path::Component::CurDir))
        .collect()
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
