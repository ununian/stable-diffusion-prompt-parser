use pest::iterators::Pairs;
use serde::Serialize;

use super::parser::Rule;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Range(pub usize, pub usize);

#[derive(Debug, Clone, Copy, Serialize)]
pub enum BracketType {
    S,
    M,
    L,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct BracketLevel(pub usize);

#[derive(Debug, Clone, Copy, Serialize)]
pub enum PromptCSTKind {
    TagStatement,
    SingleTag,
    Word,
    Bracket(BracketType, BracketLevel),
    Separator,
}

#[derive(Debug, Clone, Serialize)]
pub struct PromptCST {
    pub kind: PromptCSTKind,
    pub text: String,
    pub range: Range,
    pub inner: Vec<PromptCST>,
    pub bracket_modifier: String,
}

impl PromptCST {
    pub fn new(
        kind: PromptCSTKind,
        text: String,
        range: Range,
        inner: Vec<PromptCST>,
        bracket_level: &Vec<BracketType>,
    ) -> Self {
        Self {
            kind,
            text,
            range,
            inner,
            bracket_modifier: String::from(
                bracket_level
                    .clone()
                    .iter()
                    .map(|b| match b {
                        BracketType::S => 's',
                        BracketType::M => 'm',
                        BracketType::L => 'l',
                    })
                    .collect::<String>(),
            ),
        }
    }

    pub fn parse(pairs: Pairs<Rule>) -> Vec<PromptCST> {
        PromptCST::parse_inner(pairs, &mut vec![])
    }

    // convert the pest pairs into a CST
    pub fn parse_inner(pairs: Pairs<Rule>, bracket_level: &mut Vec<BracketType>) -> Vec<PromptCST> {
        let mut cst = vec![];

        for pair in pairs {
            let span = pair.as_span();
            let range = Range(span.start(), span.end());

            match pair.as_rule() {
                Rule::tag_statement => cst.push(PromptCST::new(
                    PromptCSTKind::TagStatement,
                    String::from(span.as_str()),
                    range,
                    PromptCST::parse_inner(pair.into_inner(), bracket_level),
                    bracket_level,
                )),

                Rule::bracket_s | Rule::bracket_m | Rule::bracket_l => {
                    let bracket_type = match pair.as_rule() {
                        Rule::bracket_s => BracketType::S,
                        Rule::bracket_m => BracketType::M,
                        Rule::bracket_l => BracketType::L,
                        _ => unreachable!(),
                    };

                    bracket_level.push(bracket_type);

                    let inner = PromptCST::parse_inner(pair.into_inner(), bracket_level);

                    bracket_level.pop();

                    cst.push(PromptCST::new(
                        PromptCSTKind::Bracket(bracket_type, BracketLevel(bracket_level.len())),
                        String::from(span.as_str()),
                        range,
                        inner,
                        bracket_level,
                    ));
                }

                Rule::single_tag => cst.push(PromptCST::new(
                    PromptCSTKind::SingleTag,
                    String::from(span.as_str()),
                    range,
                    PromptCST::parse_inner(pair.into_inner(), bracket_level),
                    bracket_level,
                )),

                Rule::word => cst.push(PromptCST::new(
                    PromptCSTKind::Word,
                    String::from(span.as_str()),
                    range,
                    PromptCST::parse_inner(pair.into_inner(), bracket_level),
                    bracket_level,
                )),
                _ => {}
            }
        }

        cst
    }
}
