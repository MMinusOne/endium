mod apis;
mod engine;
mod errors;
mod utils;
use apis::*;
use engine::*;
use utils::*;

use std::env;

use crate::{engine::heap::Heap, errors::Error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.get(1) {
        Some(file_path) => file_path,
        None => panic!("No file path provided"),
    };

    if let Ok(file_contents) = std::fs::read_to_string(&file_path) {
        let mut lexer = lexer::Lexer::new(&file_contents);

        let tokens = lexer.tokenize().unwrap();

        println!("{:#?}", tokens);

        let _ = interpretter::Interpretter::new(Some(tokens), None).execute();
        let heap_data = Heap::instance().lock().unwrap();
    } else {
        Error::FileNotFound(file_path.to_string());
    }
}
