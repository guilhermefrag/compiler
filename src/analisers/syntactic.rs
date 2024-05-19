use crate::productions::get_parsing_table;
use crate::productions::get_productions;

#[derive(Clone)]
pub struct TokenSyntactic {
    pub line: i32,
    pub token: i32,
}

pub fn syntactic_analyser(tokens_syntactic: Vec<TokenSyntactic>) {
    let productions = get_productions();
    let parsing_table = get_parsing_table();


    
}
