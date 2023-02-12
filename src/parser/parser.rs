#[derive(Debug, Clone)]
pub struct ParseResult {
    pub tags: Vec<&str>,
}

impl ParseResult {
    pub fn parse(input: &str) -> ParseResult {
        ParseResult { tags: Vec::new() }
    }
}