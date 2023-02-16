extern crate pest;

use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "parser/prompt.pest"]
pub struct PromptPestParser;

#[derive(Debug, Clone)]
pub struct ParseResult<'a> {
    pub tags: Vec<&'a str>,
}

impl ParseResult<'_> {
    pub fn parse(input: &str) -> Result<ParseResult, Error<Rule>> {
        match PromptPestParser::parse(Rule::result, input) {
            Ok(pairs) => Ok(ParseResult { tags: vec![] }),
            Err(e) => Err(e),
        }
    }
}
