use crate::enums::Token;
use crate::enums::Token::*;

use super::{
    validate_string,
    validators::{validate_float, validate_integer},
};

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
            ' ' | '\t' => continue,
            '\n' => {
                line += 1;
                continue;
            }
            '+' => {
                if let Some(&'+') = chars.peek() {
                    chars.next();
                    tokens_and_line.push(TokenLexical {
                        token: Operator("++".to_string()),
                        line,
                    });
                } else {
                    tokens_and_line.push(TokenLexical {
                        token: Operator(c.to_string()),
                        line,
                    });
                }
            }
            '-' => {
                if let Some(&'-') = chars.peek() {
                    chars.next();
                    tokens_and_line.push(TokenLexical {
                        token: Operator("--".to_string()),
                        line,
                    });
                } else {
                    tokens_and_line.push(TokenLexical {
                        token: Operator(c.to_string()),
                        line,
                    });
                }
            }
            '>' => {
                if let Some(&'>') = chars.peek() {
                    chars.next();
                    tokens_and_line.push(TokenLexical {
                        token: Operator(">>".to_string()),
                        line,
                    });
                } else {
                    tokens_and_line.push(TokenLexical {
                        token: Operator(c.to_string()),
                        line,
                    });
                }
            }
            '<' => {
                if let Some(&'<') = chars.peek() {
                    chars.next();
                    tokens_and_line.push(TokenLexical {
                        token: Operator("<<".to_string()),
                        line,
                    });
                } else {
                    tokens_and_line.push(TokenLexical {
                        token: Operator(c.to_string()),
                        line,
                    });
                }
            }
            '/' => {
                if let Some(&'/') = chars.peek() {
                    while let Some(next_char) = chars.next() {
                        if next_char == '\n' {
                            line += 1;
                            break;
                        }
                    }
                } else if let Some(&'*') = chars.peek() {
                    chars.next();
                    while let Some(next_char) = chars.next() {
                        if next_char == '*' {
                            if let Some(&'/') = chars.peek() {
                                chars.next();
                                break;
                            }
                        } else if next_char == '\n' {
                            line += 1;
                        }
                    }
                } else {
                    tokens_and_line.push(TokenLexical {
                        token: Operator(c.to_string()),
                        line,
                    });
                }
            }
            '{' | '}' | ';' | '(' | ')' | ',' | ':' | '+' | '-' | '*' | '>' | '<' | '=' | '!'
            | '\'' | '\"' => {
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
                        '\'' => {
                            let mut lexeme = String::new();
                            while let Some(&peek_char) = chars.peek() {
                                if peek_char == '\'' {
                                    chars.next();
                                    break;
                                }
                                lexeme.push(peek_char);

                                let next_char = chars.next();

                                if next_char == None
                                    || next_char == Some('\n')
                                    || next_char == Some('\r')
                                {
                                    panic!("Erro léxico: caractere não fechado");
                                }
                            }

                            match lexeme.len() {
                                1 => CharValue(lexeme),
                                _ => {
                                    validate_string(lexeme.as_str());
                                    StringValue(lexeme)
                                }
                            }
                        }
                        '\"' => {
                            let mut lexeme = String::new();
                            while let Some(&peek_char) = chars.peek() {
                                if peek_char == '\"' {
                                    chars.next();
                                    break;
                                }
                                lexeme.push(peek_char);

                                let next_char = chars.next();

                                if next_char == None
                                    || next_char == Some('\n')
                                    || next_char == Some('\r')
                                {
                                    panic!("Erro léxico: literal não fechado");
                                }
                            }
                            Literal(lexeme)
                        }
                        _ => Unknown(c.to_string()),
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
                        "void" | "main" | "inicio" | "fim" | "if" | "else" | "while" | "for"
                        | "do" | "cin" | "cout" | "float" | "integer" | "char" | "string" => {
                            Keyword(lexeme)
                        }
                        _ => Identifier(lexeme),
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
                            Ok(lexeme_parsed) => {
                                validate_integer(lexeme_parsed);
                                IntegerValue(lexeme)
                            }
                            Err(_) => match lexeme.parse::<f64>() {
                                Ok(lexeme_parsed) => {
                                    validate_float(lexeme_parsed);
                                    NumericValue(lexeme)
                                }
                                Err(_) => Unknown(lexeme),
                            },
                        },
                        line,
                    });
                }
            }
        }
    }

    tokens_and_line
}
