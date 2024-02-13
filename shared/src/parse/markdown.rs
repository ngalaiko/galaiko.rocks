use crate::path;

static OPTIONS: once_cell::sync::Lazy<pulldown_cmark::Options> =
    once_cell::sync::Lazy::new(|| pulldown_cmark::Options::ENABLE_TABLES);

pub fn parse(md: &[u8]) -> Result<Vec<pulldown_cmark::Event>, ParseError> {
    let md = std::str::from_utf8(md).map_err(ParseError::Utf8)?;
    let parser = pulldown_cmark::Parser::new_ext(md, *OPTIONS);
    let parser = parser.map(process_event).collect();
    Ok(parser)
}

fn process_event(event: pulldown_cmark::Event) -> pulldown_cmark::Event {
    match event {
        pulldown_cmark::Event::Start(tag) => pulldown_cmark::Event::Start(process_tag(tag)),
        pulldown_cmark::Event::End(tag) => pulldown_cmark::Event::End(process_tag(tag)),
        event => event,
    }
}

fn process_tag(tag: pulldown_cmark::Tag) -> pulldown_cmark::Tag {
    match tag {
        pulldown_cmark::Tag::Link(typ, link, title) if is_local_link(&link) => {
            pulldown_cmark::Tag::Link(
                typ,
                path::normalize(link.to_string())
                    .display()
                    .to_string()
                    .into(),
                title,
            )
        }
        pulldown_cmark::Tag::Image(typ, link, title) if is_local_image_link(&link) => {
            pulldown_cmark::Tag::Image(typ, replace_with_resized_image(link), title)
        }
        tag => tag,
    }
}

fn is_local_image_link(link: &str) -> bool {
    is_local_link(link) && is_image_link(link)
}

fn is_local_link(link: &str) -> bool {
    link.starts_with('/') || link.starts_with('.')
}

fn is_image_link(link: &str) -> bool {
    std::path::Path::new(link).extension().map_or(false, |ext| {
        ext.eq_ignore_ascii_case("jpeg")
            || ext.eq_ignore_ascii_case("jpg")
            || ext.eq_ignore_ascii_case("png")
    })
}

#[allow(clippy::needless_pass_by_value)]
fn replace_with_resized_image(link: pulldown_cmark::CowStr<'_>) -> pulldown_cmark::CowStr<'_> {
    let parts = link.split('.').collect::<Vec<_>>();
    let (name, _) = parts.split_at(parts.len() - 1);
    let link = format!("{}.800x0@2x.webp", name.join("."));
    pulldown_cmark::CowStr::Boxed(link.into())
}

#[derive(Debug)]
pub enum ParseError {
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Utf8(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ParseError {}
