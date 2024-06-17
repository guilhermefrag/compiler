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


pub fn type_checker(variable: &str, value: &i32, semantic_analyzer_arr: &Vec<SemanticAnalyser>) -> bool {
    let mut type_: String = String::new();

    
    return true;
}