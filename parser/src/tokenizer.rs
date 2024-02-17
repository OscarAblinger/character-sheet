use regex::Regex;

#[derive(Debug,Clone)]
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
            Token::Unknown(_, text) => text.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TokenizationIssue {
    pub token: Token,
    pub message: String,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = input;
    let mut offset = 0;

    let identifier_regex: Regex = Regex::new(r"^[a-zA-Z_-]+").unwrap();
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
            len = matched.len();
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

    tokens
}

pub fn validate(tokens: &[Token]) -> Option<Vec<TokenizationIssue>> {
    let unknown_tokens = tokens
        .iter()
        .filter(|t| matches!(t, Token::Unknown(_, _)))
        .map(|t| TokenizationIssue { token: (*t).clone(),  message: ("Unknown token '".to_owned() + &t.get_string() + "' encountered.") })
        .collect::<Vec<_>>();

    if unknown_tokens.is_empty() {
        None
    } else {
        Some(unknown_tokens)
    }
}
