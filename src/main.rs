mod assets;
mod cocktails;
mod cooklang;
mod generated;
mod markdown;
mod movies;
mod pages;
mod path;
mod posts;
mod records;
mod restaurands_and_cafes;
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
    #[command(subcommand)]
    Update(UpdateSubcommand),
}

#[derive(Subcommand)]
enum UpdateSubcommand {
    Records {
        #[arg(long)]
        token: String,
        #[arg(long, default_value = "./assets/records/index.json")]
        output: String,
    },
    Movies {
        #[arg(long, default_value = "./assets/movies/index.json")]
        output: String,
    },
    RestaurantsAndCafes {
        #[arg(long)]
        file: Option<String>,
        #[arg(long, default_value = "./assets/restaurants_and_cafes/index.json")]
        output: String,
    },
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    femme::start();

    let cli = Cli::parse();
    match cli.command {
        Commands::Serve { address } => serve::serve(&address).await,
        Commands::Update(UpdateSubcommand::Movies { output }) => {
            update::movies(&output).await.map_err(Into::into)
        }
        Commands::Update(UpdateSubcommand::Records { token, output }) => {
            update::records(&token, &output).await.map_err(Into::into)
        }
        Commands::Update(UpdateSubcommand::RestaurantsAndCafes { file, output }) => {
            update::restaurants_and_cafes(file.as_deref(), &output)
                .await
                .map_err(Into::into)
        }
    }
}
