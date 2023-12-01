mod pages;

#[derive(clap::Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Build,
}

fn main() {
    use clap::Parser;
    let cli = Cli::parse();
    if let Err(error) = match cli.command {
        Commands::Build => build(),
    } {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn build() -> Result<(), BuildError> {
    pages::iter().try_for_each(|(path, bytes)| {
        let bytes = bytes.map_err(|error| BuildError::Assets {
            error,
            path: path.clone(),
        })?;

        let output = std::path::PathBuf::from("public").join(path);
        std::fs::create_dir_all(output.parent().expect("always exists")).map_err(|error| {
            BuildError::Io {
                error,
                path: output.clone(),
            }
        })?;
        std::fs::write(&output, bytes).map_err(|error| BuildError::Io {
            error,
            path: output.clone(),
        })?;
        println!("{} written", output.display());
        Ok(())
    })
}

#[derive(Debug)]
enum BuildError {
    Assets {
        path: std::path::PathBuf,
        error: pages::PageError,
    },
    Io {
        path: std::path::PathBuf,
        error: std::io::Error,
    },
}

impl std::error::Error for BuildError {}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::Assets { path, error } => {
                write!(f, "{}: {error}", path.display())
            }
            BuildError::Io { path, error } => {
                write!(f, "{}: {error}", path.display())
            }
        }
    }
}
