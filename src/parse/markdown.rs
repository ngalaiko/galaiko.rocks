pub fn parse<T: serde::de::DeserializeOwned>(
    md: &[u8],
) -> Result<(Option<T>, maud::Markup), ParseError> {
    let (frontmatter, md) = extract_frontmatter(md);
    let frontmatter = frontmatter
        .map(|frontmatter| serde_yaml::from_slice::<T>(&frontmatter))
        .transpose()
        .map_err(ParseError::De)?;

    let md = std::str::from_utf8(&md).map_err(ParseError::Utf8)?;
    let parser = pulldown_cmark::Parser::new(md);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);

    Ok((frontmatter, maud::PreEscaped(html)))
}

fn extract_frontmatter(markdown: &[u8]) -> (Option<Vec<u8>>, Vec<u8>) {
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
pub enum ParseError {
    De(serde_yaml::Error),
    Utf8(std::str::Utf8Error),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::De(error) => write!(f, "{error}"),
            ParseError::Utf8(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for ParseError {}
