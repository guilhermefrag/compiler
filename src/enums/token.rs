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
 pub fn token_type_to_string(token: &Token) -> String {
    match token {
        Token::Keyword(_) => "Palavra chave".to_string(),
        Token::Identifier(_) => "Identificador".to_string(),
        Token::Operator(_) => "Operador".to_string(),
        Token::Terminal(_) => "Terminal".to_string(),
        Token::NonTerminal(_) => "Não terminal".to_string(),
        Token::Literal(_) => "Literal".to_string(),
        Token::Number(_) => "Número".to_string(),
        Token::Unknown(_) => "Desconhecido".to_string(),
        Token::Variable(_) => "Variável".to_string()
    }
}