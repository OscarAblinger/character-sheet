pub mod ast;

use thiserror::Error;

/*
use ast::Feature;
use ast::AST;
use ast::Model;
use ast::Modifier;
use ast::Reference;
*/

use std::rc::Rc;
use std::cell::RefCell;

use crate::cast::*;

use crate::tokenizer;
use tokenizer::validate;
use tokenizer::Token;
use tokenizer::TokenType;
use tokenizer::TokenizationIssue;

#[derive(Debug)]
pub struct ParseSuccess {
    pub cast: CAST,
    pub warnings: Vec<String>, // todo
    pub infos: Vec<String>,    // todo
}

#[derive(Debug)]
pub struct ParseFailure {
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token at offset {}: {} (expected: {})", .token.offset, .token.token_type.get_string(), .expected)]
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
        None => do_parse(to_cast_tokens(tokens)),
    }
}

fn from_tokenization_issue(ti: &TokenizationIssue) -> ParseError {
    match ti {
        TokenizationIssue::UnknownToken(offset, text) => {
            ParseError::UnexpectedText(*offset, text.clone())
        }
    }
}

fn to_cast_tokens(tokens: &[Token]) -> Vec<Rc<RefCell<CASTNode>>> {
    let mut cast_tokens: Vec<Rc<RefCell<CASTNode>>> = Vec::new();

    for token in tokens {
        cast_tokens.push(Rc::new(RefCell::new(CASTNode {
            start_offset: token.offset,
            end_offset: token.offset + token.token_type.get_string().len(),
            fmt_type: match token.token_type {
                TokenType::Whitespace(s) => {
                    let newlines = s.chars().filter(|&c| c == '\n').count();
                    CASTNodeFmtType::Whitespace(WhitespaceOptions {
                        // count of spaces in s
                        spaces: s.chars().filter(|&c| c == ' ').count(),

                        newlines,
                        min_newlines: newlines,
                        max_newlines: newlines,

                        indent_incr: 0,
                        indent_decr: 0,
                    })
                },
                t => {
                    CASTNodeFmtType::Text(t.get_string())
                },
            }
        })));
    }

    cast_tokens
}

fn do_parse(nodes: Vec<Rc<RefCell<CASTNode>>>) -> Result<ParseSuccess, ParseFailure> {
    let mut parser = Parser {
        nodes,
        position: usize::MAX,
        references: Vec::new(),
    };
    let model = parser.model()?;
    Ok(ParseSuccess {
        cast: CAST { nodes, model, references: parser.references },
        infos: vec![],
        warnings: vec![],
    })
}

struct Parser {
    nodes: Vec<Rc<RefCell<CASTNode>>>,
    position: usize,
    references: Vec<Rc<RefCell<CASTReference>>>,
}

impl Parser {
    fn curr(&self) -> Rc<RefCell<CASTNode>> {
        self.nodes[self.position].clone()
    }

    fn curr_t(&self) -> TokenType {
        self.curr().borrow().fmt_type
    }

    fn next_non_ws(&mut self) -> Token {
        if self.position == usize::MAX {
            self.position = 0;
            self.curr()
        } else if matches!(self.curr_t(), TokenType::EndOfInput) {
            self.curr()
        } else {
            loop {
                self.position += 1;
                if !matches!(self.curr_t(), TokenType::Whitespace(_)) {
                    break;
                }
            }
            self.curr()
        }
    }

    fn peek_non_ws(&mut self) -> Token {
        if self.position == usize::MAX {
            self.tokens[0].clone()
        } else if self.curr_t() == TokenType::EndOfInput {
            self.curr()
        } else {
            let mut pos = self.position;
            loop {
                pos += 1;
                if !matches!(self.tokens[pos].token_type, TokenType::Whitespace(_)) {
                    break;
                }
            }
            self.tokens[pos].clone()
        }
    }

    fn expect(&mut self, token_type: &TokenType) -> Result<(), ParseFailure> {
        self.expect_explicit(token_type, token_type.get_string())
    }

    fn expect_explicit(&mut self, token_type: &TokenType, expected: String) -> Result<(), ParseFailure> {
        if !(&self.next_non_ws().token_type == token_type) {
            Err(self.fail(expected))
        } else {
            Ok(())
        }
    }

    fn peek_expect(&mut self, token: &TokenType) -> Option<()> {
        let peeked = self.peek_non_ws();
        if !(&peeked.token_type == token) {
            None
        } else {
            Some(())
        }
    }

    fn accept(&mut self, token: &TokenType, expected: String) -> Result<String, ParseFailure> {
        let next = self.next_non_ws();
        if !next.token_type.eq_type(token) {
            Err(self.fail(expected))
        } else {
            Ok(next.token_type.get_string())
        }
    }

    fn fail(&self, expected: String) -> ParseFailure {
        ParseFailure {
            errors: vec![ParseError::UnexpectedToken {
                token: self.curr().clone(),
                expected,
            }],
        }
    }

    // recursive descent
    fn model(&mut self) -> Result<Model, ParseFailure> {
        let mut features = Vec::new();
        features.push(self.feature()?);

        while let TokenType::Section = self.curr().token_type {
            self.next_non_ws();
            features.push(self.feature()?);
        }

        Ok(Model { features })
    }

    fn feature(&mut self) -> Result<Feature, ParseFailure> {
        self.expect(&TokenType::Identifier("Name".to_string()))?;
        self.expect(&TokenType::Colon)?;
        let name = self.accept(&TokenType::String("".to_string()), "name".to_string())?;
        self.expect(&TokenType::Semicolon)?;

        let description = self
            .peek_expect(&TokenType::Identifier("Description".to_string()))
            .map(|_| -> Result<String, ParseFailure> {
                let _ = self.next_non_ws(); // skip "Description"
                self.expect(&TokenType::Colon)?;
                let desc = self.accept(&TokenType::String("".to_string()), "string".to_string())?;
                self.expect(&TokenType::Semicolon)?;

                Ok(desc)
            })
            .unwrap_or(Ok("".to_string()))?;

        let modifiers = self
            .peek_expect(&TokenType::Identifier("Modifiers".to_string()))
            .map(|_| -> Result<Vec<Modifier>, ParseFailure> {
                let _ = self.next_non_ws(); // skip "Modifiers"
                self.expect(&TokenType::Colon)?;

                let mut modifiers: Vec<Modifier> = vec![];
                loop {
                    match self.peek_non_ws().token_type {
                        TokenType::Operator(o) if o == "+" || o == "-" => {
                            modifiers.push(self.modifier_short()?)
                        }
                        TokenType::Identifier(b) if b == "bonus" => {
                            modifiers.push(self.modifier_long()?)
                        }
                        TokenType::Identifier(s) if s == "set" => {
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
        let op = self.accept(&TokenType::Operator("".to_string()), "+ or -".to_string())?;
        let num: i32 = self
            .accept(&TokenType::Number(0), "number".to_string())?
            .parse()
            .unwrap();
        let iden = self.accept(
            &TokenType::Identifier("".to_string()),
            "identifier".to_string(),
        )?;
        self.expect(&TokenType::Semicolon)?;

        let bonus = if op == "-" { -num } else { num };

        Ok(Modifier {
            referencing: Reference {
                name: iden,
                scope: ast::Scope::Character,
            },
            value: ast::ModifierValue::SimpleBonus(bonus),
        })
    }

    fn modifier_long(&mut self) -> Result<Modifier, ParseFailure> {
        self.expect(&TokenType::Identifier("bonus".to_string()))?;
        self.expect(&TokenType::Identifier("to".to_string()))?;
        let iden = self.accept(&TokenType::Identifier("".to_string()), "identifier".to_string())?;
        self.expect(&TokenType::Identifier("of".to_string()))?;
        let op: String;
        match self.peek_non_ws().token_type {
            TokenType::Operator(o) if o == "+" || o == "-" => {
                self.next_non_ws();
                op = o;
            },
            TokenType::Operator(..) => {
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
            .accept(&TokenType::Number(0), "number".to_string())?
            .parse()
            .unwrap();
        self.expect(&TokenType::Semicolon)?;

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
        self.expect(&TokenType::Identifier("set".to_string()))?;
        let iden = self.accept(&TokenType::Identifier("".to_string()), "identifier".to_string())?;
        self.expect(&TokenType::Identifier("to".to_string()))?;
        let op: String;
        match self.peek_non_ws().token_type {
            TokenType::Operator(o) if o == "+" || o == "-" => {
                self.next_non_ws();
                op = o;
            },
            TokenType::Operator(..) => {
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
            .accept(&TokenType::Number(0), "number".to_string())?
            .parse()
            .unwrap();
        self.expect(&TokenType::Semicolon)?;

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
