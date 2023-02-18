extern crate pest;

use pest::{error::Error, iterators::Pairs, Parser};

#[derive(Parser)]
#[grammar = "parser/prompt.pest"]
pub struct PromptPestParser;

#[derive(Debug, Clone)]
pub struct ParseResult<'a> {
    pub pairs: Pairs<'a, Rule>,
}

impl ParseResult<'_> {
    pub fn parse(input: &str) -> Result<ParseResult, Error<Rule>> {
        match PromptPestParser::parse(Rule::result, input) {
            Ok(pairs) => Ok(ParseResult { pairs }),
            Err(e) => Err(e),
        }
    }
}
