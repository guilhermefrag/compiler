mod analisers;
mod enums;
mod productions;
mod dictionaries;

use analisers::{codify_token, lexer_analyzer, syntactic_analyser, TokenSyntactic};

use enums::token_to_string;
use std::fs;

fn main() {
    match fs::read_to_string(r#"src/main.comp"#) {
        Ok(content) => {
            let tokens = lexer_analyzer(&content);

            let mut token_syntactic: Vec<TokenSyntactic> = Vec::new();

            for token in &tokens {
                let codified_token = codify_token(&token.token);

                let token_str: String = if codified_token.is_none() {
                    "Não é um token válido".to_string()
                } else {
                    codified_token.unwrap().to_string()
                };

                token_syntactic.push(TokenSyntactic {
                    line: token.line,
                    token: codified_token.unwrap(),
                    token_str: token_to_string(&token.token),
                });

                // print!(
                //     "Linha {:?} token {:?} >> {:?}\n ",
                //     token.line,
                //     token_str,
                //     token_to_string(&token.token),
                // );
            }

            syntactic_analyser(token_syntactic.clone());
        }
        Err(err) => {
            eprintln!("Error reading the file: {}", err);
        }
    }
}
