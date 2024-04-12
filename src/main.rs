mod analisers;
mod enums;

use std::fs;
use analisers::lexer_analyzer;
use enums::token_to_string;
use analisers::codify_token;

fn main() {
    match fs::read_to_string(r#"src\main.comp"#) {
        Ok(content) => {
            let tokens = lexer_analyzer(&content);
            for token in &tokens {
                println!("{:?}",codify_token(token));
            }
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}
