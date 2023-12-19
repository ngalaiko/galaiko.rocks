use crate::routes;

pub async fn build<P: AsRef<std::path::Path>>(output: P) -> Result<(), Error> {
    let output = output.as_ref();

    async_std::fs::remove_dir_all(output)
        .await
        .map_err(|error| Error::Io(output.to_path_buf(), error))?;

    let state = routes::Routes::build_from_assets().map_err(Error::Build)?;
    for (path, route) in state.iter() {
        let path = path
            .components()
            .filter(|c| c != &std::path::Component::RootDir)
            .collect::<std::path::PathBuf>();
        let path = output.join(&path);

        if let Some(parent) = path.parent() {
            async_std::fs::create_dir_all(parent)
                .await
                .map_err(|error| Error::Io(parent.to_path_buf(), error))?;
        }

        match route {
            routes::Route::Content { body, .. } => {
                async_std::fs::write(&path, body)
                    .await
                    .map_err(|error| Error::Io(path, error))?;
            }
            routes::Route::Redirect(redirect_to) => {
                let path = path.with_extension("redirect");
                async_std::fs::write(&path, redirect_to.display().to_string())
                    .await
                    .map_err(|error| Error::Io(path, error))?;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Build(routes::BuildError),
    Io(std::path::PathBuf, std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::Build(error) => write!(f, "{error}"),
            Error::Io(path, error) => write!(f, "{}: {error}", path.display()),
        }
    }
}

impl std::error::Error for Error {}
