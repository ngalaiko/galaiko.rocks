#[tokio::main]
async fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../assets");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map(std::path::PathBuf::from)
        .expect("CARGO_MANIFEST_DIR is not set");
    let input = manifest_dir.join("../assets/");
    let output = manifest_dir.join("public");

    if let Err(err) = convert::convert(input, output).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
