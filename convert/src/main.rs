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
    let subscriber = tracing_subscriber::fmt::fmt()
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();
    if let Err(err) = convert::convert(cli.source, cli.destination).await {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
