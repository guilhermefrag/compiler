#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Terminal(String),
    Literal(String),
    Number(f64),
    Unknown(String),
    Variable(String),
    Parenthesizer(String),
    Comma(String),
    TypeAssignment(String)
}

pub fn token_to_string(token: &Token) -> String {
    match token {
        Token::Keyword(s) => s.to_string(),
        Token::Identifier(s) => s.to_string(),
        Token::Operator(s) => s.to_string(),
        Token::Terminal(s) => s.to_string(),
        Token::Literal(s) => s.to_string(),
        Token::Number(n) => n.to_string(),
        Token::Unknown(s) => s.to_string(),
        Token::Variable(s) => s.to_string(),
        Token::Parenthesizer(s) => s.to_string(),
        Token::Comma(s) => s.to_string(),
        Token::TypeAssignment(s) => s.to_string()
    }
}
pub fn token_type_to_string(token: &Token) -> String {
    match token {
        Token::Keyword(_) => "Palavra chave".to_string(),
        Token::Identifier(_) => "Identificador".to_string(),
        Token::Operator(_) => "Operador".to_string(),
        Token::Terminal(_) => "Terminal".to_string(),
        Token::Literal(_) => "Literal".to_string(),
        Token::Number(_) => "Número".to_string(),
        Token::Unknown(_) => "Desconhecido".to_string(),
        Token::Variable(_) => "Variável".to_string(),
        Token::Parenthesizer(_) => "Parentesizador".to_string(),
        Token::Comma(_) => "Vírgula".to_string(),
        Token::TypeAssignment(_) => "Atribuição de tipo".to_string()
    }
}
