use regex::Regex;

use thiserror::Error;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub offset: i32,
}

impl Token {
    pub fn new(token_type: TokenType, offset: i32) -> Self {
        Self {
            token_type,
            offset
        }
    }
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum TokenType {
    Identifier(String),
    Dice(String),
    Number(i32),
    String(String),
    OpeningBracket,
    ClosingBracket,
    Colon,
    Semicolon,
    Section, // ---
    Operator(String),
    Whitespace(String),
    EndOfInput,
    Unknown(String),
}

impl TokenType {
    pub fn get_string(&self) -> String {
        match self {
            TokenType::Identifier(text) => text.clone(),
            TokenType::Dice(text) => text.to_string(),
            TokenType::Number(number) => number.to_string(),
            TokenType::String(text) => text.clone(),
            TokenType::OpeningBracket => "{".to_string(),
            TokenType::ClosingBracket => "}".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Section => "---".to_string(),
            TokenType::Operator(text) => text.clone(),
            TokenType::Whitespace(text) => text.clone(),
            TokenType::EndOfInput => "End of Input".to_string(),
            TokenType::Unknown(text) => text.clone(),
        }
    }

    pub fn eq_type(&self, other: &TokenType) -> bool {
        match (self, other) {
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            (TokenType::Dice(_), TokenType::Dice(_)) => true,
            (TokenType::Number(_), TokenType::Number(_)) => true,
            (TokenType::String(_), TokenType::String(_)) => true,
            (TokenType::Operator(_), TokenType::Operator(_)) => true,
            (TokenType::Whitespace(_), TokenType::Whitespace(_)) => true,
            (TokenType::Unknown(_), TokenType::Unknown(_)) => true,
            (a, b) if a == b => true,
            (_, _) => false,
        }
    }
}

#[derive(Debug,Error)]
pub enum TokenizationIssue {
    #[error("Unknown token found: '{1}'")]
    UnknownToken(i32, String),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = input;
    let mut offset = 0;

    let identifier_regex: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z_-]*").unwrap();
    let dice_regex: Regex = Regex::new(r"^\d+d\d+([+-]\d+d\d+)*([+-]\d+)?").unwrap();
    let number_regex: Regex = Regex::new(r"^\d+([,.]\d+)?").unwrap();
    let string_regex: Regex = Regex::new(r#"^"(([^"]|\\")*)""#).unwrap();
    let opening_bracket_regex: Regex = Regex::new(r"^\{").unwrap();
    let closing_bracket_regex: Regex = Regex::new(r"^}").unwrap();
    let colon_regex: Regex = Regex::new(r"^:").unwrap();
    let semicolon_regex: Regex = Regex::new(r"^;").unwrap();
    let section_regex: Regex = Regex::new(r"^---").unwrap();
    let operator_regex: Regex = Regex::new(r"^[+\-*/]").unwrap();
    let whitespace_regex: Regex = Regex::new(r"^\s+").unwrap();

    while !remaining.is_empty() {
        let len: usize;
        if let Some(captures) = identifier_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Identifier(matched.to_string()), offset));
            len = matched.len();
        } else if let Some(captures) = dice_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Dice(matched.to_string()), offset));
            len = matched.len();
        } else if let Some(captures) = number_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Number(matched.parse().unwrap()), offset));
            len = matched.len();
        } else if let Some(captures) = string_regex.captures(remaining) {
            let matched = captures.get(1).unwrap().as_str();
            tokens.push(Token::new(TokenType::String(matched.to_string()), offset));
            len = captures.get(0).unwrap().len();
        } else if let Some(captures) = opening_bracket_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::OpeningBracket,offset));
            len = matched.len();
        } else if let Some(captures) = closing_bracket_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::ClosingBracket,offset));
            len = matched.len();
        } else if let Some(captures) = colon_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Colon, offset));
            len = matched.len();
        } else if let Some(captures) = semicolon_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Semicolon, offset));
            len = matched.len();
        } else if let Some(captures) = section_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Section, offset));
            len = matched.len();
        } else if let Some(captures) = operator_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Operator(matched.to_string()), offset));
            len = matched.len();
        } else if let Some(captures) = whitespace_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::new(TokenType::Whitespace(matched.to_string()), offset));
            len = matched.len();
        } else if let Some(next_char) = remaining.chars().next() {
            tokens.push(Token::new(TokenType::Unknown(next_char.to_string()), offset));
            len = 1;
        } else {
            // can never happen
            len = 0;
        }
        offset += len as i32;
        remaining = &remaining[len..];
    }
    
    tokens.push(Token::new(TokenType::EndOfInput, offset));
    tokens
}

pub fn validate(tokens: &[Token]) -> Option<Vec<TokenizationIssue>> {
    let unknown_tokens = tokens
        .iter()
        .filter(|t| matches!(t.token_type, TokenType::Unknown(_)))
        .map(|t| TokenizationIssue::UnknownToken(t.offset, t.token_type.get_string()) )
        .collect::<Vec<_>>();

    if unknown_tokens.is_empty() {
        None
    } else {
        Some(unknown_tokens)
    }
}
