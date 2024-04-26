pub fn parse<T: serde::de::DeserializeOwned>(md: &[u8]) -> Result<(T, &[u8]), ParseError> {
    let lines = md.split(|b| *b == b'\n');
    let mut frontmatter = Vec::new();
    let mut in_frontmatter = false;
    let mut offset = 0;
    for line in lines {
        if line == b"---" {
            offset += 4;
            if in_frontmatter {
                return serde_yaml::from_slice(&frontmatter)
                    .map_err(ParseError::De)
                    .map(|t| (t, &md[offset..]));
            }
            in_frontmatter = !in_frontmatter;
        } else if in_frontmatter {
            frontmatter.extend_from_slice(line);
            frontmatter.push(b'\n');
            offset += line.len() + 1;
        } else {
            return Err(ParseError::NotFound);
        }
    }
    Err(ParseError::NotFound)
}

#[derive(Debug)]
pub enum ParseError {
    De(serde_yaml::Error),
    NotFound,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::De(error) => write!(f, "{error}"),
            ParseError::NotFound => write!(f, "not found"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, serde::Deserialize, PartialEq)]
    struct TestFrontmatter {
        number: i32,
        string: String,
    }

    #[test]
    fn test_parse() {
        let input = b"---
number: 1
string: hello world
---
# Hello, world!
";
        let (frontmatter, body) = parse::<TestFrontmatter>(input).unwrap();
        assert_eq!(frontmatter.number, 1);
        assert_eq!(frontmatter.string, "hello world");
        assert_eq!(body, b"# Hello, world!\n");
    }
}
