mod tokens;
mod lexer;
mod errors; 

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = match args.get(1) {
        Some(file_path) => file_path,
        None => panic!("No file path provided"),
    };

    

    println!("{file}")
}
