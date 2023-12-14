static PARSER: once_cell::sync::Lazy<cooklang::CooklangParser> = once_cell::sync::Lazy::new(|| {
    cooklang::CooklangParser::new(cooklang::Extensions::all(), cooklang::Converter::default())
});

pub fn parse(data: &[u8]) -> Result<cooklang::ScaledRecipe, ParseError> {
    let src = std::str::from_utf8(data).map_err(ParseError::Utf8)?;
    let (recipe, _) = PARSER
        .parse(src)
        .into_result()
        .map_err(|e| ParseError::Cooklang(e.to_string()))?;

    Ok(recipe.default_scale())
}

#[allow(clippy::too_many_lines)]
pub fn to_html(recipe: &cooklang::ScaledRecipe) -> maud::Markup {
    let ingredient_list = recipe.group_ingredients(PARSER.converter());
    maud::html! {
        @if !recipe.metadata.map.is_empty() {
            ul {
                @for (key, value) in &recipe.metadata.map {
                    li.metadata {
                        span.key { (key) } ":" (value)
                    }
                }
            }

            hr {}
        }

        @if !ingredient_list.is_empty() {
            h2 { "Ingredients:" }
            ul {
                @for entry in &ingredient_list {
                    li {
                        b { (entry.ingredient.display_name()) }
                        @if !entry.quantity.is_empty() {": " (entry.quantity) }
                        @if let Some(n) = &entry.ingredient.note { " (" (n) ")" }
                    }
                }
            }
        }

        @if !recipe.cookware.is_empty() {
            h2 { "Cookware:" }
            ul {
                @for item in recipe.cookware.iter().filter(|c| c.modifiers().should_be_listed()) {
                    @let amount = item.group_amounts(&recipe.cookware).iter()
                                        .map(std::string::ToString::to_string)
                                        .reduce(|s, q| format!("{s}, {q}"))
                                        .unwrap_or(String::new());
                    li {
                        b { (item.display_name()) }
                        @if !amount.is_empty() { ": " (amount) }
                        @if let Some(n) = &item.note { " (" (n) ")" }
                    }
                }
            }
        }
        @if !recipe.cookware.is_empty() || !ingredient_list.is_empty() {
            hr {}
        }
        @for (s_index, section) in recipe.sections.iter().enumerate() {
            @let s_num = s_index + 1;
            @if let Some(name) = &section.name {
                h3 { "(" (s_num) ") " (name) }
            } @else if recipe.sections.len() > 1 {
                h3 { "Section " (s_num) }
            }

            @for content in &section.content {
                @match content {
                    cooklang::Content::Text(t) => p { (t) },
                    cooklang::Content::Step(s) => p {
                        b { (s.number) ". " }
                        @for item in &s.items {
                            @match item {
                                cooklang::Item::Ingredient { index } => {
                                    @let igr = &recipe.ingredients[*index];
                                    span.ingredient {
                                        (igr.display_name())
                                        @if let Some(q) = &igr.quantity {
                                            i { "(" (q) ")" }
                                        }
                                        @if let Some((index, target)) = &igr.relation.references_to() {
                                            @match target {
                                                cooklang::IngredientReferenceTarget::Step => {
                                                    i { "(from step " (section.content[*index].unwrap_step().number) ")" }
                                                }
                                                cooklang::IngredientReferenceTarget::Section => {
                                                    @let sect = *index + 1;
                                                    i { "(from section " (sect) ")" }
                                                }
                                                cooklang::IngredientReferenceTarget::Ingredient => {}
                                            }
                                        }
                                    }
                                }
                                cooklang::Item::Cookware { index } => {
                                    @let cw = &recipe.cookware[*index];
                                    span.cookware {
                                        (cw.display_name())
                                        @if let Some(q) = &cw.quantity {
                                            i { "(" (q) ")" }
                                        }
                                    }
                                }
                                cooklang::Item::Timer { index } => {
                                    @let tm = &recipe.timers[*index];
                                    span.timer {
                                        @if let Some(name) = &tm.name {
                                            "(" (name) ")"
                                        }
                                        @if let Some(q) = &tm.quantity {
                                            i { (q) }
                                        }
                                    }
                                }
                                cooklang::Item::InlineQuantity { index } => {
                                    @let q = &recipe.inline_quantities[*index];
                                    i.temp { (q) }
                                }
                                cooklang::Item::Text { value } => {
                                    (value)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Utf8(std::str::Utf8Error),
    NoName,
    Cooklang(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Utf8(error) => write!(f, "{error}"),
            ParseError::NoName => write!(f, "No name"),
            ParseError::Cooklang(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ParseError {}
