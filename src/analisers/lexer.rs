use crate::enums::Token;

pub struct TokenLexical {
    pub token: Token,
    pub line: i32,
}

pub fn lexer_analyzer(code: &str) -> Vec<TokenLexical> {
    let mut tokens = Vec::new();
    let mut chars = code.chars().peekable();
    let mut line = 1;
    let mut tokens_and_line = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => continue, // Skip whitespace
            '\n' => {
                line += 1;
                continue;
            }
            '{' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            '}' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            ';' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            '(' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            ')' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            ',' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            ':' => tokens_and_line.push(TokenLexical {
                token: Token::Terminal(c.to_string()),
                line,
            }),
            '+' => tokens_and_line.push(TokenLexical {
                token: Token::Operator(c.to_string()),
                line,
            }),
            '-' => tokens_and_line.push(TokenLexical {
                token: Token::Operator(c.to_string()),
                line,
            }),
            '*' => tokens_and_line.push(TokenLexical {
                token: Token::Operator(c.to_string()),
                line,
            }),
            '/' => tokens_and_line.push(TokenLexical {
                token: Token::Operator(c.to_string()),
                line,
            }),
            '>' => {
                if let Some('>') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator(">>".to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator(">>".to_string()),
                        line,
                    });
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator(c.to_string()),
                        line,
                    });
                }
            }
            '<' => {
                if let Some('<') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("<<".to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator("<<".to_string()),
                        line,
                    });
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator(c.to_string()),
                        line,
                    });
                }
            }
            '=' => {
                if let Some('=') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("==".to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator("==".to_string()),
                        line,
                    });
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator(c.to_string()),
                        line,
                    });
                }
            }
            '!' => {
                if let Some('=') = chars.peek() {
                    chars.next(); // Consume the next character
                    tokens.push(Token::Operator("!=".to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator("!=".to_string()),
                        line,
                    });
                } else {
                    tokens.push(Token::Operator(c.to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Operator(c.to_string()),
                        line,
                    });
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
                tokens_and_line.push(TokenLexical {
                    token: Token::Literal(lexeme),
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
                    match lexeme.as_str() {
                        "void" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "main" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "inicio" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "fim" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "if" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "else" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "while" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "for" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "do" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "cin" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        "cout" => tokens_and_line.push(TokenLexical {
                            token: Token::Keyword(lexeme),
                            line,
                        }),
                        _ => tokens_and_line.push(TokenLexical {
                            token: Token::Identifier(lexeme),
                            line,
                        }),
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
                    tokens_and_line.push(TokenLexical {
                        token: Token::Number(lexeme.parse().unwrap()),
                        line,
                    });
                } else {
                    tokens.push(Token::Variable(c.to_string()));
                    tokens_and_line.push(TokenLexical {
                        token: Token::Variable(c.to_string()),
                        line,
                    });
                }
            }
        }
    }

    tokens_and_line
}
