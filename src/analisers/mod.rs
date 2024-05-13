mod lexer;
mod semantic;
mod syntatic;
mod tokenizer;
mod validators;

pub use lexer::lexer_analyzer; 
pub use semantic::semantic_analyser;
pub use syntatic::syntatic_analyser;
pub use tokenizer::codify_token;
pub use validators::{validate_string, validate_variables};