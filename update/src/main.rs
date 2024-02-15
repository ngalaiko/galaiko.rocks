mod discogs;
mod hledger;
mod letterboxd;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Discogs {
        #[arg(long)]
        token: String,
        #[arg(long, default_value = "./assets/records/")]
        output: std::path::PathBuf,
    },
    Letterboxd {
        #[arg(long, default_value = "./assets/movies/")]
        output: std::path::PathBuf,
    },
    Hledger {
        #[arg(long)]
        file: Option<std::path::PathBuf>,
        #[arg(long, default_value = "./assets/places/")]
        output: std::path::PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt::fmt()
        .with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let cli = Cli::parse();
    match cli.command {
        Commands::Letterboxd { output } => letterboxd::update(&output).await.map_err(Into::into),
        Commands::Discogs { token, output } => {
            discogs::update(&token, &output).await.map_err(Into::into)
        }
        Commands::Hledger { file, output } => hledger::update(file.as_deref(), &output)
            .await
            .map_err(Into::into),
    }
}
