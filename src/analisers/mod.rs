mod lexer;
mod semantic;
mod syntatic;
mod tokenizer;

pub use lexer::lexer_analyzer; 
pub use semantic::semantic_analyser;
pub use syntatic::syntatic_analyser;
pub use tokenizer::codify_token;
