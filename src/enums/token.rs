#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Terminal(String),
    NonTerminal(String),
    Literal(String),
    Number(f64),
    Unknown(String),
}
