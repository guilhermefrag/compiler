use std::any;

use crate::enums::token_to_string;
use crate::productions::get_productions;
use crate::productions::{get_parsing_table, Productions};

#[derive(Clone)]
pub struct TokenSyntactic {
    pub line: i32,
    pub token: i32,
    pub token_str: String,
}

// Algoritmo de parser

// Início
// X recebe o topo da pilha
// “a” recebe o símbolo da entrada
// Repita
//     Se X=î então
//         Retire o elemento do topo da pilha
//         X recebe o topo da pilha
//     Senão
//         Se X é terminal então
//             Se X=a então
//                 Retire o elemento do topo da pilha
//                 Sai do Repita
//         Senão
//             Erro
//             Encerra o programa
//     Fim Se
//     Senão (* X é não-terminal*)
//         Se M(X,a) <> ∅ então (existe uma regra)
//             Retire o elemento do topo da pilha
//             Coloque o conteúdo da regra na pilha
//             X recebe o topo da pilha
//         Senão
//             Erro
//             Encerra o programa
//         Fim Se
//     Fim Se
// Até X=$ (*pilha vazia, análise concluída*)
// Fim

fn get_line_by_token(token: i32, tokens_syntactic: Vec<TokenSyntactic>) -> i32 {
    for token_syntactic in tokens_syntactic {
        if token_syntactic.token == token {
            return token_syntactic.line;
        }
    }

    return 0;
}

fn get_token_str_by_token(token: i32, tokens_syntactic: Vec<TokenSyntactic>) -> String {
    for token_syntactic in tokens_syntactic {
        if token_syntactic.token == token {
            return token_syntactic.token_str;
        }
    }

    return "".to_string();
}

pub fn syntactic_analyser(tokens_syntactic: Vec<TokenSyntactic>) {
    let productions = get_productions();
    let parsing_table = get_parsing_table();

    let mut expansions_arr: Vec<String> = Vec::new();
    let mut input_arr: Vec<String> = Vec::new();
    let mut lines_arr: Vec<i32> = Vec::new();
    let mut str_tokens_arr: Vec<String> = Vec::new();

    for token in tokens_syntactic.clone() {
        input_arr.push(token.token.to_string());
        lines_arr.push(token.line);
        str_tokens_arr.push(token.token_str.clone());
    }

    input_arr.push("$".to_string());

    for prod in productions[0].clone() {
        expansions_arr.push(prod.to_string());
    }

    expansions_arr.push("$".to_string());

    let mut top_expansion_arr = expansions_arr[0].clone();
    let mut top_input_arr = input_arr[0].clone();

    while top_expansion_arr != "$" {
        if top_expansion_arr == "16" {
            expansions_arr.remove(0);
            top_expansion_arr = expansions_arr[0].clone();
        } else {
            if top_expansion_arr.parse::<i32>().is_ok() {
                let parsed_top_expansion = top_expansion_arr.parse::<i32>().unwrap();

                if parsed_top_expansion <= 48 && parsed_top_expansion >= 1 {
                    if top_expansion_arr == top_input_arr {
                        expansions_arr.remove(0);
                        input_arr.remove(0);
                        top_expansion_arr = expansions_arr[0].clone();
                        top_input_arr = input_arr[0].clone();
                        continue;
                    } else {
                        panic!(
                            "Erro no token {:?} na linha {:?} com o token {:?}",
                            parsed_top_expansion,
                            get_line_by_token(
                                top_input_arr.parse::<i32>().unwrap(),
                                tokens_syntactic.clone()
                            ),
                            get_token_str_by_token(
                                top_input_arr.parse::<i32>().unwrap(),
                                tokens_syntactic.clone()
                            )
                        );
                    }
                } else if parsed_top_expansion <= 80 && parsed_top_expansion >= 49 {
                    if parsing_table[parsed_top_expansion as usize]
                        [top_input_arr.parse::<i32>().unwrap()  as usize]
                        .is_some()
                    {
                        expansions_arr.remove(0);

                        let production_list =
                            productions[parsing_table[parsed_top_expansion as usize]
                                [top_input_arr.parse::<i32>().unwrap() as usize]
                                .unwrap() as usize]
                                .clone();

                        for prod in production_list {
                            expansions_arr.insert(0, prod.to_string());
                        }

                        top_expansion_arr = expansions_arr[0].clone();
                        continue;
                    } else {
                        panic!(
                            "Erro no token {:?} na linha {:?} com o token {:?}",
                            parsed_top_expansion,
                            get_line_by_token(
                                top_input_arr.parse::<i32>().unwrap(),
                                tokens_syntactic.clone()
                            ),
                            get_token_str_by_token(
                                top_input_arr.parse::<i32>().unwrap(),
                                tokens_syntactic.clone()
                            )
                        );
                    }
                }
            }
        }
    }
}
