mod analisers;
mod enums;

use std::fs;
use analisers::lexer_analyzer;

fn main() {
    match fs::read_to_string(r#"src\main.comp"#) {
        Ok(content) => {
            let tokens = lexer_analyzer(&content);
            print!("{:?}", tokens)
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}
