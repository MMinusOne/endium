mod errors;
mod interpretter;
mod lexer;
mod tokens;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(file_path) => file_path,
        None => panic!("No file path provided"),
    };

    let file_contents = match std::fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(e) => panic!("{:?}", e),
    };

    // let mut lexer = lexer::Lexer::new(&file_contents);

    // let tokens = lexer.tokenize().unwrap();

    // interpretter::Interpretter::new(tokens).execute();
}
