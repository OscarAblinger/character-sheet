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
    let mut parser = Parser {
        tokens,
        position: 0,
    };
    parser.model();
    parser.to_result()
}

struct Parser<'a> {
    tokens: &'a[Token],
    position: usize,
}

impl Parser<'_> {
    fn to_result(&self) -> ParseResult {
        ParseResult {
            is_ok: true,
            errors: vec![],
        }
    }

    fn curr(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn next_non_ws(&mut self) {
        self.position += 1;
    }

    // recursive descent
    fn model(&mut self) {
        let mut features = Vec::new();
        features.push(self.feature());
        
        while let Token::Section(_) = self.curr() {
            self.next_non_ws();
            features.push(self.feature());
        }
    }

    fn feature(&self) {
    }
}
