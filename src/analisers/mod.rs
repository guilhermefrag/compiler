mod lexer;
mod semantic;
mod syntactic;
mod tokenizer;
mod validators;

pub use lexer::lexer_analyzer; 
pub use semantic::add_to_semantic_analyzer;
pub use semantic::SemanticAnalyser;
pub use semantic::type_checker;
pub use syntactic::{syntactic_analyser, TokenSyntactic};
pub use tokenizer::codify_token;
pub use validators::{validate_string, validate_variables};