use crate::tokenizer;

use tokenizer::Token;
use tokenizer::TokenizationIssue;
use tokenizer::validate;

#[derive(Debug)]
pub struct ParseResult {
    pub is_ok: bool,
    pub errors: Vec<ParseError>,
}

#[derive(Debug)]
pub struct ParseError {
    pub offset: i32,
    pub message: String,
}

pub fn parse(tokens: &[Token]) -> ParseResult {
    match validate(tokens) {
        Some(errors) => {
            ParseResult {
                is_ok: false,
                errors: errors.iter()
                    .map(from_tokenization_issue)
                    .collect::<Vec<_>>()
            }
        }
        None => {
            do_parse(tokens)
        }
    }
}

fn from_tokenization_issue(ti: &TokenizationIssue) -> ParseError {
    ParseError {
        offset: ti.token.get_offset(),
        message: ti.message.clone(),
    }
}

fn do_parse(tokens: &[Token]) -> ParseResult {
    ParseResult {
        is_ok: true,
        errors: vec![],
    }
}
