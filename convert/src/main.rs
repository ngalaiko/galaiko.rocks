use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "./assets")]
    source: std::path::PathBuf,
    #[arg(long, default_value = "./serve/public")]
    destination: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if let Err(err) = convert::convert(cli.source, cli.destination).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
