mod analisers;
mod enums;

use analisers::codify_token;
use analisers::lexer_analyzer;
use enums::token_to_string;
use std::fs;

fn main() {
    match fs::read_to_string(r#"src\loop.comp"#) {
        Ok(content) => {
            let tokens = lexer_analyzer(&content);
            for token in &tokens {
                let codified_token = codify_token(&token.token);

                let token_str: String = if codified_token.is_none() {
                    "Não é um token válido".to_string()
                } else {
                    codified_token.unwrap().to_string()
                };
                print!(
                    "Linha {:?} token {:?} >> {:?}\n ",
                    token.line,
                    token_str,
                    token_to_string(&token.token),
                );
            }
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}
