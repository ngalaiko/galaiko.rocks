use crate::path;

/// Extracts the frontmatter from a markdown file.
/// Returns parsed frontmatter and the remaining markdown.
pub fn extract_frontmatter(markdown: &[u8]) -> (Option<Vec<u8>>, Vec<u8>) {
    let lines = markdown.split(|b| *b == b'\n');
    let mut frontmatter = Vec::new();
    let mut markdown = Vec::new();
    let mut in_frontmatter = false;
    for line in lines {
        if line == b"---" {
            in_frontmatter = !in_frontmatter;
        } else if in_frontmatter {
            frontmatter.extend_from_slice(line);
            frontmatter.push(b'\n');
        } else {
            markdown.extend_from_slice(line);
            markdown.push(b'\n');
        }
    }
    let frontmatter = if frontmatter.is_empty() {
        None
    } else {
        Some(frontmatter)
    };
    (frontmatter, markdown)
}

#[derive(Debug)]
pub enum ToHtmlError {
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for ToHtmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToHtmlError::Utf8(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ToHtmlError {}

pub fn to_html(path: &std::path::Path, data: &[u8]) -> Result<maud::Markup, ToHtmlError> {
    let md = std::str::from_utf8(data).map_err(ToHtmlError::Utf8)?;
    let parser = pulldown_cmark::Parser::new(md);
    let parser = parser.map(|event| match event {
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(typ, link, title)) => {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Image(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(path, &link).into_boxed_str()),
                title,
            ))
        }
        pulldown_cmark::Event::End(pulldown_cmark::Tag::Image(typ, link, title)) => {
            pulldown_cmark::Event::End(pulldown_cmark::Tag::Image(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(path, &link).into_boxed_str()),
                title,
            ))
        }
        pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(typ, link, title)) => {
            pulldown_cmark::Event::Start(pulldown_cmark::Tag::Link(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(path, &link).into_boxed_str()),
                title,
            ))
        }
        pulldown_cmark::Event::End(pulldown_cmark::Tag::Link(typ, link, title)) => {
            pulldown_cmark::Event::End(pulldown_cmark::Tag::Link(
                typ,
                pulldown_cmark::CowStr::Boxed(fix_link(path, &link).into_boxed_str()),
                title,
            ))
        }
        _ => event,
    });
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    Ok(maud::PreEscaped(html))
}

fn fix_link(path: &std::path::Path, link: &str) -> String {
    let link = path::normalize(link);
    path.parent()
        .unwrap()
        .join(link)
        .to_string_lossy()
        .to_string()
}
