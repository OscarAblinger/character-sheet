use regex::Regex;

use thiserror::Error;

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub enum Token {
    Identifier(i32, String),
    Dice(i32, String),
    Number(i32, String),
    String(i32, String),
    OpeningBracket(i32),
    ClosingBracket(i32),
    Colon(i32),
    Semicolon(i32),
    Section(i32), // ---
    Operator(i32, String),
    Whitespace(i32, String),
    EndOfInput(i32),
    Unknown(i32, String),
}

impl Token {
    pub fn get_offset(&self) -> i32 {
        match self {
            Token::Identifier(offset, _) => *offset,
            Token::Dice(offset, _) => *offset,
            Token::Number(offset, _) => *offset,
            Token::String(offset, _) => *offset,
            Token::OpeningBracket(offset) => *offset,
            Token::ClosingBracket(offset) => *offset,
            Token::Colon(offset) => *offset,
            Token::Semicolon(offset) => *offset,
            Token::Section(offset) => *offset,
            Token::Operator(offset, _) => *offset,
            Token::Whitespace(offset, _) => *offset,
            Token::EndOfInput(offset) => *offset,
            Token::Unknown(offset, _) => *offset,
        }
    }

    pub fn get_string(&self) -> String {
        match self {
            Token::Identifier(_, text) => text.clone(),
            Token::Dice(_, text) => text.clone(),
            Token::Number(_, text) => text.clone(),
            Token::String(_, text) => text.clone(),
            Token::OpeningBracket(_) => "{".to_string(),
            Token::ClosingBracket(_) => "}".to_string(),
            Token::Colon(_) => ":".to_string(),
            Token::Semicolon(_) => ";".to_string(),
            Token::Section(_) => "---".to_string(),
            Token::Operator(_, text) => text.clone(),
            Token::Whitespace(_, text) => text.clone(),
            Token::EndOfInput(_) => "End of Input".to_string(),
            Token::Unknown(_, text) => text.clone(),
        }
    }

    pub fn eq_type(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Identifier(_, _), Token::Identifier(_, _)) => true,
            (Token::Dice(_, _), Token::Dice(_, _)) => true,
            (Token::Number(_, _), Token::Number(_, _)) => true,
            (Token::String(_, _), Token::String(_, _)) => true,
            (Token::OpeningBracket(_), Token::OpeningBracket(_)) => true,
            (Token::ClosingBracket(_), Token::ClosingBracket(_)) => true,
            (Token::Colon(_), Token::Colon(_)) => true,
            (Token::Semicolon(_), Token::Semicolon(_)) => true,
            (Token::Section(_), Token::Section(_)) => true,
            (Token::Operator(_, _), Token::Operator(_, _)) => true,
            (Token::Whitespace(_, _), Token::Whitespace(_, _)) => true,
            (Token::EndOfInput(_), Token::EndOfInput(_)) => true,
            (Token::Unknown(_, _), Token::Unknown(_, _)) => true,
            _ => false
        }
    }

    pub fn eq_text(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::Identifier(_, a), Token::Identifier(_, b)) if a == b => true,
            (Token::Dice(_, a), Token::Dice(_, b)) if a == b => true,
            (Token::Number(_, a), Token::Number(_, b)) if a == b => true,
            (Token::String(_, a), Token::String(_, b)) if a == b => true,
            (Token::OpeningBracket(_), Token::OpeningBracket(_)) => true,
            (Token::ClosingBracket(_), Token::ClosingBracket(_)) => true,
            (Token::Colon(_), Token::Colon(_)) => true,
            (Token::Semicolon(_), Token::Semicolon(_)) => true,
            (Token::Section(_), Token::Section(_)) => true,
            (Token::Operator(_, a), Token::Operator(_, b)) if a == b => true,
            (Token::Whitespace(_, a), Token::Whitespace(_, b)) if a == b => true,
            (Token::EndOfInput(_), Token::EndOfInput(_)) => true,
            (Token::Unknown(_, a), Token::Unknown(_, b)) if a == b => true,
            _ => false
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
            tokens.push(Token::Identifier(offset, matched.to_string()));
            len = matched.len();
        } else if let Some(captures) = dice_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Dice(offset, matched.to_string()));
            len = matched.len();
        } else if let Some(captures) = number_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Number(offset, matched.to_string()));
            len = matched.len();
        } else if let Some(captures) = string_regex.captures(remaining) {
            let matched = captures.get(1).unwrap().as_str();
            tokens.push(Token::String(offset, matched.to_string()));
            len = captures.get(0).unwrap().len();
        } else if let Some(captures) = opening_bracket_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::OpeningBracket(offset));
            len = matched.len();
        } else if let Some(captures) = closing_bracket_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::ClosingBracket(offset));
            len = matched.len();
        } else if let Some(captures) = colon_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Colon(offset));
            len = matched.len();
        } else if let Some(captures) = semicolon_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Semicolon(offset));
            len = matched.len();
        } else if let Some(captures) = section_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Section(offset));
            len = matched.len();
        } else if let Some(captures) = operator_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Operator(offset, matched.to_string()));
            len = matched.len();
        } else if let Some(captures) = whitespace_regex.captures(remaining) {
            let matched = captures.get(0).unwrap().as_str();
            tokens.push(Token::Whitespace(offset, matched.to_string()));
            len = matched.len();
        } else if let Some(next_char) = remaining.chars().next() {
            tokens.push(Token::Unknown(offset, next_char.to_string()));
            len = 1;
        } else {
            // can never happen
            len = 0;
        }
        offset += len as i32;
        remaining = &remaining[len..];
    }
    
    tokens.push(Token::EndOfInput(offset));
    tokens
}

pub fn validate(tokens: &[Token]) -> Option<Vec<TokenizationIssue>> {
    let unknown_tokens = tokens
        .iter()
        .filter(|t| matches!(t, Token::Unknown(_, _)))
        .map(|t| TokenizationIssue::UnknownToken(t.get_offset(), t.get_string()) )
        .collect::<Vec<_>>();

    if unknown_tokens.is_empty() {
        None
    } else {
        Some(unknown_tokens)
    }
}
