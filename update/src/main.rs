mod hledger;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Hledger {
        #[arg(long)]
        file: Option<std::path::PathBuf>,
        #[arg(long, default_value = "./assets/places/")]
        output: std::path::PathBuf,
    },
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
    match cli.command {
        Commands::Hledger { file, output } => {
            if let Err(error) = hledger::update(file.as_deref(), &output).await {
                eprintln!("{error}");
                std::process::exit(1);
            }
        }
    }
}
