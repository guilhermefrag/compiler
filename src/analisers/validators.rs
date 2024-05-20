use regex::Regex;

pub fn validate_integer(value: i32) {
    if value > 2000 || value < 0 {
        panic!("Erro léxico: Valores inteiros devem ser entre 0 e 2000");
    }
}

pub fn validate_float(value: f64) {
    if value > 2000.0 || value < 0.0 {
        panic!("Erro léxico: Valores float devem ser entre 0 e 2000");
    }
}

pub fn validate_string(value: &str) {
    if value.len() > 20 {
        panic!("Erro léxico: Strings devem ter no máximo 20 caracteres");
    }
}

pub fn validate_variables(value: &str) {
    if value.len() > 30 {
        panic!("Erro léxico: Variáveis devem ter no máximo 30 caracteres");
    }

    if Regex::new(r"^[a-zA-Z_]*$").unwrap().is_match(value) == false {
        panic!("Erro léxico: Variáveis devem possuir apenas caracteres, e não especiais");
    }
}
