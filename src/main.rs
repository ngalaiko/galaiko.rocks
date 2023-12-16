mod assets;
mod cocktails;
mod cooklang;
mod generated;
mod markdown;
mod movies;
mod pages;
mod path;
mod posts;
mod serve;
mod update;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve {
        #[arg(default_value = "127.0.0.1:8080")]
        address: String,
    },
    Update {
        #[arg(value_enum, default_value_t = update::Resource::All)]
        resource: update::Resource,
    },
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    femme::start();

    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { address } => serve::serve(&address).await,
        Commands::Update { resource } => update::update(&resource).await,
    }
}
