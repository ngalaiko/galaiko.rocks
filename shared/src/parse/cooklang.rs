#[derive(Debug)]
pub struct Recipe {
    title: String,
    inner: cooklang::ScaledRecipe,
    parser: cooklang::CooklangParser,
}

impl Recipe {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn source(&self) -> Option<&cooklang::metadata::NameAndUrl> {
        self.inner.metadata.source()
    }

    pub fn group_ingredients(&self) -> Vec<cooklang::ingredient_list::GroupedIngredient> {
        self.inner.group_ingredients(self.parser.converter())
    }

    pub fn group_cookware(&self) -> Vec<cooklang::ingredient_list::GroupedCookware> {
        self.inner.group_cookware()
    }

    pub fn sections(&self) -> &[cooklang::Section] {
        &self.inner.sections
    }

    pub fn ingredients(&self) -> &[cooklang::Ingredient] {
        &self.inner.ingredients
    }

    pub fn cookware(&self) -> &[cooklang::Cookware] {
        &self.inner.cookware
    }

    pub fn timers(&self) -> &[cooklang::Timer] {
        &self.inner.timers
    }

    pub fn inline_quantities(&self) -> &[cooklang::Quantity] {
        &self.inner.inline_quantities
    }
}

pub fn parse(data: &[u8]) -> Result<Recipe, ParseError> {
    let src = std::str::from_utf8(data).map_err(ParseError::Utf8)?;
    let parser =
        cooklang::CooklangParser::new(cooklang::Extensions::all(), cooklang::Converter::default());
    let (recipe, _) = parser
        .parse(src)
        .into_result()
        .map_err(|e| ParseError::Cooklang(e.to_string()))?;
    let title = recipe
        .metadata
        .map
        .get("title")
        .map(std::string::ToString::to_string)
        .ok_or(ParseError::NoTitle)?;
    Ok(Recipe {
        title,
        inner: recipe.default_scale(),
        parser,
    })
}

#[derive(Debug)]
pub enum ParseError {
    Utf8(std::str::Utf8Error),
    NoTitle,
    Cooklang(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::NoTitle => write!(f, "title must be set"),
            ParseError::Utf8(error) => write!(f, "{error}"),
            ParseError::Cooklang(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ParseError {}
