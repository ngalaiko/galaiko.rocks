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
        output: String,
    },
    Letterboxd {
        #[arg(long, default_value = "./assets/movies/")]
        output: String,
    },
    Hledger {
        #[arg(long)]
        file: Option<String>,
        #[arg(long, default_value = "./assets/restaurants_and_cafes/")]
        output: String,
    },
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    femme::start();

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
