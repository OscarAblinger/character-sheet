use text_io::read;

mod tokenizer;
mod parser;

use tokenizer::lex;
use tokenizer::validate;
use parser::parse;

fn print_tokenize(text: &str) {
    let tokens = lex(text);
    println!("Tokenized: {:#?}", tokens);
    let errors = validate(&tokens[..]);
    match errors {
        Some(errors) => println!("Errors: {:#?}", errors),
        None => (),
    }
}

fn print_parse(text: &str) {
    let tokens = lex(text);
    let parse_result = parse(&tokens[..]);
    println!("Parse result: {:#?}", parse_result);
}

fn start_interactive_mode() {
    let mut text: String = "".to_string();
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
                    print_parse(&text);
                    text = "".to_string();
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
