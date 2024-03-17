use text_io::read;

use character_sheet_parser::parser;
use character_sheet_parser::tokenizer;
use character_sheet_parser::serializer;

use parser::parse;
use tokenizer::lex;
use tokenizer::validate;

use parser::ast::AST;

use serializer::serialize;

struct State {
    last_ast: Option<AST>,
}

impl State {
    fn new() -> State {
        State { last_ast: None }
    }
}

fn print_tokenize(text: &str) {
    let tokens = lex(text);
    println!("Tokenized: {:#?}", tokens);
    let errors = validate(&tokens[..]);
    match errors {
        Some(errors) => println!("Errors: {:#?}", errors),
        None => (),
    }
}

fn print_parse(state: &mut State, text: &str) {
    let tokens = lex(text);
    let parse_result = parse(&tokens[..]);
    println!("Parse result: {:#?}", &parse_result);

    if let Ok(success) = parse_result {
        state.last_ast = Some(success.ast);
    }
}

fn print_serialized(state: &State) {
    if let Some(ast) = &state.last_ast {
        let serialized = serialize(&ast);
        println!("{}", serialized);
    }
}

fn start_interactive_mode() {
    let mut text: String = "".to_string();
    let mut state: State = State::new();
    loop {
        let line: String = read!("{}\n");

        if line.starts_with(":") {
            let command = line[1..].trim();
            match command {
                "t" | "tokenize" => {
                    print_tokenize(&text);
                    text = "".to_string();
                },
                "p" | "parse" => {
                    print_parse(&mut state, &text);
                    text = "".to_string();
                },
                "s" | "serialize" => {
                    print_serialized(&state);
                },
                "e" | "exit" => break,
                _ => print_help()
            }
        }
        else {
            text.push_str(&line);
            text.push_str("\n");
        }
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  :t, :tokenize - Tokenize the text");
    println!("  :p, :parse - Parse the text");
    println!("  :e, :exit - Exit the program");
}

fn main() {
    start_interactive_mode()
}
