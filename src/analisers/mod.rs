mod lexer;
mod semantic;
mod syntatic;

pub use lexer::lexer_analyzer; 
pub use semantic::semantic_analyser;
pub use syntatic::syntatic_analyser;
