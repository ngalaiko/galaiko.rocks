mod assets;
mod cocktails;
mod cooklang;
mod generated;
mod markdown;
mod pages;
mod path;
mod posts;
mod serve;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Serve {
        #[arg(default_value = "127.0.0.1:8080")]
        address: String,
    },
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    femme::start();

    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { address } => serve::serve(&address).await,
    }
}
