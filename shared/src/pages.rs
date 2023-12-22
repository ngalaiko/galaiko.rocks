use crate::types::{cocktails, entries, generated, movies, records, restaurands_and_cafes};

pub struct Page(maud::Markup);

impl Page {
    #[must_use]
    pub fn into_string(self) -> String {
        self.0.into()
    }
}

impl From<&maud::Markup> for Page {
    fn from(value: &maud::Markup) -> Self {
        Self(maud::html! {
            (maud::DOCTYPE)
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                link rel="stylesheet" href="/index.css";
            }
            main {
                article {
                    (value)
                }
            }
        })
    }
}

impl From<&cocktails::Cocktail> for Page {
    fn from(value: &cocktails::Cocktail) -> Self {
        Self::from(&value.body)
    }
}

impl From<&entries::Entry> for Page {
    fn from(value: &entries::Entry) -> Self {
        Self::from(&value.body)
    }
}

impl From<&[entries::Entry]> for Page {
    fn from(value: &[entries::Entry]) -> Self {
        Self::from(&generated::posts(value))
    }
}

impl From<&[restaurands_and_cafes::Place]> for Page {
    fn from(value: &[restaurands_and_cafes::Place]) -> Self {
        Self::from(&generated::restaurants_and_cafes(value))
    }
}

impl From<&[cocktails::Cocktail]> for Page {
    fn from(value: &[cocktails::Cocktail]) -> Self {
        Self::from(&generated::cocktails(value))
    }
}

impl From<&[movies::Entry]> for Page {
    fn from(value: &[movies::Entry]) -> Self {
        Self::from(&generated::movies(value))
    }
}

impl From<&[records::Record]> for Page {
    fn from(value: &[records::Record]) -> Self {
        Self::from(&generated::records(value))
    }
}
