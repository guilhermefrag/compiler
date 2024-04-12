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
    Variable(String)
}

pub fn token_to_string(token: &Token) -> String {
    match token {
        Token::Keyword(s) => s.to_string(),
        Token::Identifier(s) => s.to_string(),
        Token::Operator(s) => s.to_string(),
        Token::Terminal(s) => s.to_string(),
        Token::NonTerminal(s) => s.to_string(),
        Token::Literal(s) => s.to_string(),
        Token::Number(n) => n.to_string(),
        Token::Unknown(s) => s.to_string(),
        Token::Variable(s ) => s.to_string()
    }
}
