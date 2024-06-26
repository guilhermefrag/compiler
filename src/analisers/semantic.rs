use crate::dictionaries::{MATCH_TYPES, MATCH_TYPES_TO_STRING, MATCH_TYPES_ASSIGNMENT};

#[derive(Clone, Debug)]
pub struct SemanticAnalyser {
    pub token: String,
    pub category: String,
    pub type_: i32,
    pub level: String,
}

pub fn add_to_semantic_analyzer(semantic_analyzer_arr: &mut Vec<SemanticAnalyser>, token: String, category: String, type_: i32, level: String){
    let semantic_analyzer = SemanticAnalyser {
        token,
        category,
        type_,
        level,
    };
    semantic_analyzer_arr.push(semantic_analyzer);
}


pub fn type_checker(variable_name: &str, variable_value: &i32, semantic_analyzer_arr: &Vec<SemanticAnalyser>, line: i32) -> bool {
    for semantic_analyzer in semantic_analyzer_arr {
        if semantic_analyzer.token == variable_name {
            let expected_type = MATCH_TYPES.get(&semantic_analyzer.type_).unwrap();
            if expected_type != variable_value {
                panic!("Erro de tipo na linha {:?}: A variável '{}' é do tipo {:?}, mas foi atribuído um valor do tipo {:?}", line, variable_name, convert_type(semantic_analyzer.type_), convert_assignment_type(variable_value));
            }
        }
    }
    
    return true;
}

pub fn variable_existence_checker(variable_name: &str, semantic_analyzer_arr: &Vec<SemanticAnalyser>) -> bool {
    for semantic_analyzer in semantic_analyzer_arr {
        if semantic_analyzer.token == variable_name {
            return true;
        }
    }

    return false;
}

fn convert_type(type_: i32) -> String{
    let type_str = MATCH_TYPES_TO_STRING.get(&type_).unwrap();
    return type_str.to_string();
}

fn convert_assignment_type(type_: &i32) -> String{
    let type_str = MATCH_TYPES_ASSIGNMENT.get(&type_).unwrap();
    return type_str.to_string();
}