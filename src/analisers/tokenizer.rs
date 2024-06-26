use crate::analisers::validate_variables;
use crate::enums::token_to_string;
use crate::enums::Token;

pub fn codify_token(token: &Token) -> Option<i32> {
    let str_token = token_to_string(token);
    match str_token.as_str() {
        "while" => Some(1),
        "void" => Some(2),
        "string" => Some(3),
        "return" => Some(4),
        "numerointeiro" => Some(5),
        "numerofloat" => Some(6),
        "nomevariavel" => Some(7),
        "nomedochar" => Some(8),
        "nomedavariavel" => Some(9),
        "nomedastring" => Some(10),
        "main" => Some(11),
        "literal" => Some(12),
        "integer" => Some(13),
        "inicio" => Some(14),
        "if" => Some(15),
        "î" => Some(16),
        "for" => Some(17),
        "float" => Some(18),
        "fim" => Some(19),
        "else" => Some(20),
        "do" => Some(21),
        "cout" => Some(22),
        "cin" => Some(23),
        "char" => Some(24),
        "callfuncao" => Some(25),
        ">>" => Some(26),
        ">=" => Some(27),
        ">" => Some(28),
        "==" => Some(29),
        "=" => Some(30),
        "<=" => Some(31),
        "<<" => Some(32),
        "<" => Some(33),
        "++" => Some(34),
        "+" => Some(35),
        "}" => Some(36),
        "{" => Some(37),
        ";" => Some(38),
        ":" => Some(39),
        "/" => Some(40),
        "," => Some(41),
        "*" => Some(42),
        ")" => Some(43),
        "(" => Some(44),
        "$" => Some(45),
        "!=" => Some(46),
        "--" => Some(47),
        "-" => Some(48),
        " " => None,
        "\r" => None,
        "\n" => None,
        "\t" => None,
        _ => {
            if let Token::CharValue(_) = token {
                return Some(8);
            }
            if let Token::StringValue(_) = token {
                return Some(10);
            }
            if let Token::Literal(_) = token {
                return Some(12);
            }
            if let Ok(_int_value) = str_token.parse::<i32>() {
                return Some(5);
            }
            if let Ok(_float_value) = str_token.parse::<f64>() {
                return Some(6);
            }
            {
                validate_variables(&str_token);
                Some(9)
            }
        }
    }
}