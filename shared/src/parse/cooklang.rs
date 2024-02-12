#[derive(Debug)]
pub struct Recipe {
    recipe: cooklang::ScaledRecipe,
    parser: cooklang::CooklangParser,
}

impl Recipe {
    pub fn source(&self) -> Option<&cooklang::metadata::NameAndUrl> {
        self.recipe.metadata.source()
    }

    pub fn group_ingredients(&self) -> Vec<cooklang::ingredient_list::GroupedIngredient> {
        self.recipe.group_ingredients(self.parser.converter())
    }

    pub fn group_cookware(&self) -> Vec<cooklang::ingredient_list::GroupedCookware> {
        self.recipe.group_cookware()
    }

    pub fn sections(&self) -> &[cooklang::Section] {
        &self.recipe.sections
    }

    pub fn ingredients(&self) -> &[cooklang::Ingredient] {
        &self.recipe.ingredients
    }

    pub fn cookware(&self) -> &[cooklang::Cookware] {
        &self.recipe.cookware
    }

    pub fn timers(&self) -> &[cooklang::Timer] {
        &self.recipe.timers
    }

    pub fn inline_quantities(&self) -> &[cooklang::Quantity] {
        &self.recipe.inline_quantities
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
    Ok(Recipe {
        recipe: recipe.default_scale(),
        parser,
    })
}

#[derive(Debug)]
pub enum ParseError {
    Utf8(std::str::Utf8Error),
    Cooklang(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Utf8(error) => write!(f, "{error}"),
            ParseError::Cooklang(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ParseError {}
