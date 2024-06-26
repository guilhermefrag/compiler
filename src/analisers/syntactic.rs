use std::vec;

use crate::productions::{get_productions, GET_CAN_BE_EMPTY};
use crate::productions::{find, get_parsing_table};
use crate::analisers::semantic::{add_to_semantic_analyzer, type_checker, SemanticAnalyser, variable_existence_checker };
use crate::dictionaries::TYPES;

#[derive(Clone, Debug)]
pub struct TokenSyntactic {
    pub line: i32,
    pub token: i32,
    pub token_str: String,
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

    let mut semantic_analyzer: Vec<SemanticAnalyser> = Vec::new(); 

    let mut expansions_arr: Vec<String> = Vec::new();
    let mut input_arr: Vec<String> = Vec::new();
    let mut lines_arr: Vec<i32> = Vec::new();
    let mut str_tokens_arr: Vec<String> = Vec::new();
    let mut element_line: i32 = 0;

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
        println!(
            "Entrada token analisada: {:?} | Token: {:?}/{:?} | Linha: {:?}",
            top_expansion_arr,
            get_token_str_by_token(
                top_input_arr.parse::<i32>().unwrap(),
                tokens_syntactic.clone()
            ),
            top_input_arr,
            lines_arr[element_line as usize]
        );
        println!("Pilha: {:?}", expansions_arr);
        if top_expansion_arr == "16" {
            expansions_arr.remove(0);
            top_expansion_arr = expansions_arr[0].clone();
        } else {
            if top_expansion_arr.parse::<i32>().is_ok() {
                let parsed_top_expansion = top_expansion_arr.parse::<i32>().unwrap();

                if parsed_top_expansion <= 48 && parsed_top_expansion >= 1 {
                    
                    if parsed_top_expansion == 9 {
                        let mut i = 0;

                        for token in input_arr.clone() {
                            if token == "39" {
                                let variable_name = str_tokens_arr[element_line as usize].clone();
                                let variable_type = input_arr[i + 1].clone().parse::<i32>().unwrap();

                                let is_variable_existent = variable_existence_checker(
                                    &variable_name,
                                    &semantic_analyzer
                                );
                                
                                if is_variable_existent {
                                    panic!("Erro de variável na linha: {:?} A variável {:?} já foi declarada",   lines_arr[element_line as usize], variable_name);
                                }

                                if TYPES.contains(&variable_type) {
                                    add_to_semantic_analyzer(
                                        &mut semantic_analyzer, 
                                        str_tokens_arr[element_line as usize].clone(),
                                        "variable".to_string(),
                                        variable_type,
                                        "global".to_string()
                                    );
                                    break;
                                } else {
                                    panic!("Erro de tipo na linha: {:?} O tipo {:?} não é um tipo válido",  lines_arr[element_line as usize], variable_type);
                                }
                            } else if token == "30" {
                                let variable_name = str_tokens_arr[element_line as usize].clone();
                                let variable_value = input_arr[i + 1].clone().parse::<i32>().unwrap();
                                
                                let is_variable_existent = variable_existence_checker(
                                    &variable_name,
                                    &semantic_analyzer
                                );

                                if !is_variable_existent {
                                    panic!("Erro de variável na linha {:?}: A variável {:?} não foi declarada",  lines_arr[element_line as usize], variable_name);
                                }
                                
                                type_checker(&variable_name, &variable_value, &semantic_analyzer, lines_arr[element_line as usize]);
                                break;
                            }
                            i += 1;
                        }
                    } 

                    if top_expansion_arr == top_input_arr {
                        expansions_arr.remove(0);
                        input_arr.remove(0);
                        top_expansion_arr = expansions_arr[0].clone();
                        element_line = element_line + 1;
                        top_input_arr = input_arr[0].clone();
                        continue;
                    } else {
                        panic!(
                            "Erro no token {:?} na linha {:?} com o token {:?}",
                            parsed_top_expansion,
                            lines_arr[element_line as usize],
                            get_token_str_by_token(
                                top_input_arr.parse::<i32>().unwrap(),
                                tokens_syntactic.clone()
                            )
                        );
                    }
                } else if parsed_top_expansion <= 80 && parsed_top_expansion >= 49 {
                    let node = find(
                        parsing_table.clone(),
                        parsed_top_expansion,
                        top_input_arr.parse::<i32>().unwrap(),
                    );
                    if !node.is_none() {
                        expansions_arr.remove(0);

                        let production_list =
                            productions[(node.unwrap().production - 1) as usize].clone();

                        for prod in production_list.iter().rev() {
                            expansions_arr.insert(0, prod.to_string());
                        }

                        top_expansion_arr = expansions_arr[0].clone();

                        continue;

                    } else if GET_CAN_BE_EMPTY(parsed_top_expansion) {
                        expansions_arr.remove(0);
                        top_expansion_arr = expansions_arr[0].clone();
                        continue;
                        
                    } else {
                        panic!(
                            "Erro na entrada {:?} na linha {:?} com o token {:?} - {:?}",
                            parsed_top_expansion,
                            lines_arr[element_line as usize],
                            get_token_str_by_token(
                                top_input_arr.parse::<i32>().unwrap(),
                                tokens_syntactic.clone()
                            ),
                            top_input_arr
                        );
                    }
                }
            }
        }
    }
}
