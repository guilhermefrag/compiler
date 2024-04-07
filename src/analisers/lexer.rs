use crate::enums::Token;

pub fn lexer_analyzer(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => continue, // Skip whitespace
            '{' => tokens.push(Token::Terminal(c.to_string())),
            '}' => tokens.push(Token::Terminal(c.to_string())),
            ';' => tokens.push(Token::Terminal(c.to_string())),
            '(' => tokens.push(Token::Terminal(c.to_string())),
            ')' => tokens.push(Token::Terminal(c.to_string())),
            ',' => tokens.push(Token::Terminal(c.to_string())),
            ':' => tokens.push(Token::Terminal(c.to_string())),
            '+' => tokens.push(Token::Operator(c.to_string())),
            '-' => tokens.push(Token::Operator(c.to_string())),
            '*' => tokens.push(Token::Operator(c.to_string())),
            '/' => tokens.push(Token::Operator(c.to_string())),
            '>' => {
                if let Some('&') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("&&".to_string()));
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                }
            }
            '<' => {
                if let Some('&') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("&&".to_string()));
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("==".to_string()));
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                }
            }
            '!' => {
                if let Some('=') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("!=".to_string()));
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                }
            }
            '"' => {
                let mut lexeme = String::new();
                while let Some(&next_char) = chars.peek() {
                    if next_char == '"' {
                        chars.next(); // Consume the closing quote
                        break;
                    }
                    lexeme.push(next_char);
                    chars.next();
                }
                tokens.push(Token::Literal(lexeme));
            }
            _ => {
                if c.is_alphabetic() {
                    let mut lexeme = c.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            lexeme.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    match lexeme.as_str() {
                        "void" => tokens.push(Token::Keyword(lexeme)),
                        "main" => tokens.push(Token::Keyword(lexeme)),
                        "inicio" => tokens.push(Token::Keyword(lexeme)),
                        "fim" => tokens.push(Token::Keyword(lexeme)),
                        "if" => tokens.push(Token::Keyword(lexeme)),
                        "else" => tokens.push(Token::Keyword(lexeme)),
                        "while" => tokens.push(Token::Keyword(lexeme)),
                        "for" => tokens.push(Token::Keyword(lexeme)),
                        "do" => tokens.push(Token::Keyword(lexeme)),
                        "cin" => tokens.push(Token::Keyword(lexeme)),
                        "cout" => tokens.push(Token::Keyword(lexeme)),
                        _ => tokens.push(Token::Identifier(lexeme)),
                    }
                } else if c.is_numeric() {
                    let mut lexeme = c.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_numeric() || next_char == '.' {
                            lexeme.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(lexeme.parse().unwrap()));
                } else {
                    tokens.push(Token::Unknown(c.to_string()));
                }
            }
        }
    }
    
    tokens
}