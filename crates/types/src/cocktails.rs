use crate::parse;

#[derive(Debug)]
pub struct Cocktail {
    pub recipe: parse::cooklang::Recipe,
}

impl TryFrom<&[u8]> for Cocktail {
    type Error = FromError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let recipe = parse::cooklang::parse(data).map_err(FromError::Parse)?;
        Ok(Cocktail { recipe })
    }
}

#[derive(Debug)]
pub enum FromError {
    Parse(parse::cooklang::ParseError),
}

impl std::fmt::Display for FromError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FromError::Parse(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for FromError {}
