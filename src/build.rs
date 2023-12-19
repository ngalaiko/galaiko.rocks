use crate::routes;

pub async fn build<P: AsRef<std::path::Path>>(output: P) -> Result<(), routes::BuildError> {
    let state = routes::Routes::build()?;
    Ok(())
}
