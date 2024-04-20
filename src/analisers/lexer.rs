use crate::enums::Token;
use crate::enums::Token::*;

pub struct TokenLexical {
    pub token: Token,
    pub line: i32,
}

pub fn lexer_analyzer(code: &str) -> Vec<TokenLexical> {
    let mut tokens_and_line = Vec::new();
    let mut chars = code.chars().peekable();
    let mut line = 1;

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => continue, // Skip whitespace
            '\n' => {
                line += 1;
                continue;
            }
            '{' | '}' | ';' | '(' | ')' | ',' | ':' | '+' | '-' | '*' | '/' | '>' | '<' | '=' | '!' | '"' => {
                tokens_and_line.push(TokenLexical {
                    token: match c {
                        '{' | '}' | '(' | ')' => Parenthesizer(c.to_string()),
                        ';' => Terminal(c.to_string()),
                        ',' => Comma(c.to_string()),
                        ':' => TypeAssignment(c.to_string()),
                        '+' | '-' | '*' | '/' => Operator(c.to_string()),
                        '>' | '<' | '=' | '!' => {
                            if let Some('=') = chars.peek() {
                                chars.next(); // Consume the next character
                                match (c, '=') {
                                    ('>', '=') => Operator(">=".to_string()),
                                    ('<', '=') => Operator("<=".to_string()),
                                    ('=', '=') => Operator("==".to_string()),
                                    ('!', '=') => Operator("!=".to_string()),
                                    _ => Unknown(c.to_string()),
                                }
                            } else {
                                Operator(c.to_string())
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
                            Literal(lexeme)
                        }
                        _ => Unknown(c.to_string()), // Default case for unknown characters
                    },
                    line,
                });
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
                    let token = match lexeme.as_str() {
                        "void" | "main" | "inicio" | "fim" | "if" | "else" | "while" | "for" | "do" | "cin" | "cout" | "float" | "integer" | "char" | "string" =>
                            Keyword(lexeme),
                        _ => Variable(lexeme),
                    };
                    tokens_and_line.push(TokenLexical { token, line });
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
                    tokens_and_line.push(TokenLexical {
                        token: match lexeme.parse::<i32>() {
                            Ok(_) => IntegerValue(lexeme),
                            Err(_) => match lexeme.parse::<f64>() {
                                Ok(_) => NumericValue(lexeme.parse().unwrap()),
                                Err(_) => Unknown(lexeme),
                            }
                        },
                        line,
                    });
                }
            }
        }
    }

    tokens_and_line
}
