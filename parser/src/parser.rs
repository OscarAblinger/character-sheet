mod ast;

use thiserror::Error;

use ast::Feature;
use ast::Model;
use ast::Modifier;
use ast::Reference;

use crate::tokenizer;
use tokenizer::validate;
use tokenizer::Token;
use tokenizer::TokenizationIssue;

#[derive(Debug)]
pub struct ParseSuccess {
    pub model: Model,
    pub warnings: Vec<String>, // todo
    pub infos: Vec<String>,    // todo
}

#[derive(Debug)]
pub struct ParseFailure {
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token at offset {}: {} (expected: {})", .token.get_offset(), .token.get_string(), .expected)]
    UnexpectedToken { token: Token, expected: String },
    #[error("Unexpected text at offset {}: {}", .0, .1)]
    UnexpectedText(i32, String),
    #[error("Unknown error at offset {0}")]
    UnknownError(i32),
}

pub fn parse(tokens: &[Token]) -> Result<ParseSuccess, ParseFailure> {
    match validate(tokens) {
        Some(errors) => Err(ParseFailure {
            errors: errors
                .iter()
                .map(from_tokenization_issue)
                .collect::<Vec<_>>(),
        }),
        None => do_parse(tokens),
    }
}

fn from_tokenization_issue(ti: &TokenizationIssue) -> ParseError {
    match ti {
        TokenizationIssue::UnknownToken(offset, text) => {
            ParseError::UnexpectedText(*offset, text.clone())
        }
    }
}

fn do_parse(tokens: &[Token]) -> Result<ParseSuccess, ParseFailure> {
    let mut parser = Parser {
        tokens,
        position: usize::MAX,
        references: Vec::new(),
    };
    let model = parser.model()?;
    Ok(ParseSuccess {
        model,
        infos: vec![],
        warnings: vec![],
    })
}

struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
    references: Vec<Reference>,
}

impl Parser<'_> {
    fn curr(&self) -> Token {
        self.tokens[self.position].clone()
    }

    fn next_non_ws(&mut self) -> Token {
        if self.position == usize::MAX {
            self.position = 0;
            self.curr()
        } else if matches!(self.curr(), Token::EndOfInput(..)) {
            self.curr()
        } else {
            loop {
                self.position += 1;
                if !matches!(self.curr(), Token::Whitespace(..)) {
                    break;
                }
            }
            self.curr()
        }
    }

    fn peek_non_ws(&mut self) -> Token {
        if self.position == usize::MAX {
            self.tokens[0].clone()
        } else if matches!(self.curr(), Token::EndOfInput(..)) {
            self.curr()
        } else {
            let mut pos = self.position;
            loop {
                pos += 1;
                if !matches!(self.tokens[pos].clone(), Token::Whitespace(..)) {
                    break;
                }
            }
            self.tokens[pos].clone()
        }
    }

    fn expect(&mut self, token: &Token) -> Result<(), ParseFailure> {
        self.expect_explicit(token, token.get_string())
    }

    fn expect_explicit(&mut self, token: &Token, expected: String) -> Result<(), ParseFailure> {
        if !self.next_non_ws().eq_text(token) {
            Err(ParseFailure {
                errors: vec![ParseError::UnexpectedToken {
                    token: self.curr().clone(),
                    expected,
                }],
            })
        } else {
            Ok(())
        }
    }

    fn peek_expect(&mut self, token: &Token) -> Option<()> {
        let peeked = self.peek_non_ws();
        if !peeked.eq_text(token) {
            None
        } else {
            Some(())
        }
    }

    fn accept(&mut self, token: &Token, expected: String) -> Result<String, ParseFailure> {
        let next = self.next_non_ws();
        if !next.eq_type(token) {
            Err(ParseFailure {
                errors: vec![ParseError::UnexpectedToken {
                    token: self.curr().clone(),
                    expected,
                }],
            })
        } else {
            Ok(next.get_string())
        }
    }

    // recursive descent
    fn model(&mut self) -> Result<Model, ParseFailure> {
        let mut features = Vec::new();
        features.push(self.feature()?);

        while let Token::Section(_) = self.curr() {
            self.next_non_ws();
            features.push(self.feature()?);
        }

        Ok(Model { features })
    }

    fn feature(&mut self) -> Result<Feature, ParseFailure> {
        self.expect(&Token::Identifier(0, "Name".to_string()))?;
        self.expect(&Token::Colon(0))?;
        let name = self.accept(&Token::String(0, "".to_string()), "name".to_string())?;
        self.expect(&Token::Semicolon(0))?;

        let description = self
            .peek_expect(&Token::Identifier(0, "Description".to_string()))
            .map(|_| -> Result<String, ParseFailure> {
                let _ = self.next_non_ws(); // skip "Description"
                self.expect(&Token::Colon(0))?;
                let desc = self.accept(&Token::String(0, "".to_string()), "string".to_string())?;
                self.expect(&Token::Semicolon(0))?;

                Ok(desc)
            })
            .unwrap_or(Ok("".to_string()))?;

        let modifiers = self
            .peek_expect(&Token::Identifier(0, "Modifiers".to_string()))
            .map(|_| -> Result<Vec<Modifier>, ParseFailure> {
                let _ = self.next_non_ws(); // skip "Modifiers"
                self.expect(&Token::Colon(0))?;

                let mut modifiers: Vec<Modifier> = vec![];
                loop {
                    match self.peek_non_ws() {
                        Token::Operator(_, o) if o == "+" || o == "-" => {
                            modifiers.push(self.modifier_short()?)
                        }
                        Token::Identifier(_, b) if b == "bonus" => {
                            modifiers.push(self.modifier_long()?)
                        }
                        Token::Identifier(_, s) if s == "set" => {
                            modifiers.push(self.modifier_set()?)
                        }
                        _ => break,
                    }
                }

                Ok(modifiers)
            })
            .unwrap_or_else(|| Ok(vec![]))?;

        Ok(Feature {
            name,
            description,
            modifiers,
        })
    }

    fn modifier_short(&mut self) -> Result<Modifier, ParseFailure> {
        let op = self.accept(&Token::Operator(0, "".to_string()), "+ or -".to_string())?;
        let num: i32 = self
            .accept(&Token::Number(0, "".to_string()), "number".to_string())?
            .parse()
            .unwrap();
        let iden = self.accept(
            &Token::Identifier(0, "".to_string()),
            "identifier".to_string(),
        )?;
        self.expect(&Token::Semicolon(0))?;

        let bonus = if op == "-" { -num } else { num };

        Ok(Modifier {
            referencing: Reference {
                name: iden,
                scope: ast::Scope::Character,
            },
            value: ast::ModifierValue::Bonus(bonus),
        })
    }

    fn modifier_long(&mut self) -> Result<Modifier, ParseFailure> {
        self.expect(&Token::Identifier(0, "bonus".to_string()))?;
        self.expect(&Token::Identifier(0, "to".to_string()))?;
        let iden = self.accept(&Token::Identifier(0, "".to_string()), "identifier".to_string())?;
        self.expect(&Token::Identifier(0, "of".to_string()))?;
        let op: String;
        match self.peek_non_ws() {
            Token::Operator(_, o) if o == "+" || o == "-" => {
                self.next_non_ws();
                op = o;
            },
            Token::Operator(..) => {
                return Err(ParseFailure {
                    errors: vec![ParseError::UnexpectedToken {
                        token: self.curr().clone(),
                        expected: "+ or -".to_string(),
                    }],
                })
            },
            _ => {
                op = "+".to_string();
            },
        }

        let num: i32 = self
            .accept(&Token::Number(0, "".to_string()), "number".to_string())?
            .parse()
            .unwrap();
        self.expect(&Token::Semicolon(0))?;

        let bonus = if op == "-" { -num } else { num };

        Ok(Modifier {
            referencing: Reference {
                name: iden,
                scope: ast::Scope::Character,
            },
            value: ast::ModifierValue::Bonus(bonus),
        })
    }

    fn modifier_set(&mut self) -> Result<Modifier, ParseFailure> {
        self.expect(&Token::Identifier(0, "set".to_string()))?;
        let iden = self.accept(&Token::Identifier(0, "".to_string()), "identifier".to_string())?;
        self.expect(&Token::Identifier(0, "to".to_string()))?;
        let op: String;
        match self.peek_non_ws() {
            Token::Operator(_, o) if o == "+" || o == "-" => {
                self.next_non_ws();
                op = o;
            },
            Token::Operator(..) => {
                return Err(ParseFailure {
                    errors: vec![ParseError::UnexpectedToken {
                        token: self.curr().clone(),
                        expected: "+ or -".to_string(),
                    }],
                })
            },
            _ => {
                op = "+".to_string();
            },
        }

        let num: i32 = self
            .accept(&Token::Number(0, "".to_string()), "number".to_string())?
            .parse()
            .unwrap();
        self.expect(&Token::Semicolon(0))?;

        let bonus = if op == "-" { -num } else { num };

        Ok(Modifier {
            referencing: Reference {
                name: iden,
                scope: ast::Scope::Character,
            },
            value: ast::ModifierValue::Set(bonus),
        })
    }
}
