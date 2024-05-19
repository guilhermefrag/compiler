mod lexer;
mod semantic;
mod syntactic;
mod tokenizer;
mod validators;

pub use lexer::lexer_analyzer; 
pub use semantic::semantic_analyser;
pub use syntactic::{syntactic_analyser, TokenSyntactic};
pub use tokenizer::codify_token;
pub use validators::{validate_string, validate_variables};