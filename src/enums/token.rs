#[derive(Debug)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Operator(String),
    Terminal(String),
    Literal(String),
    Unknown(String),
    Parenthesizer(String),
    Comma(String),
    TypeAssignment(String),
    IntegerValue(String),
    NumericValue(String),
    StringValue(String),
    CharValue(String),
}

pub fn token_to_string(token: &Token) -> String {
    match token {
        Token::Keyword(s) => s.to_string(),
        Token::Identifier(s) => s.to_string(),
        Token::Operator(s) => s.to_string(),
        Token::Terminal(s) => s.to_string(),
        Token::Literal(s) => s.to_string(),
        Token::Unknown(s) => s.to_string(),
        Token::Parenthesizer(s) => s.to_string(),
        Token::Comma(s) => s.to_string(),
        Token::TypeAssignment(s) => s.to_string(),
        Token::IntegerValue(s) => s.to_string(),
        Token::NumericValue(s) => s.to_string(),
        Token::StringValue(s) => s.to_string(),
        Token::CharValue(s) => s.to_string(),
    }
}
