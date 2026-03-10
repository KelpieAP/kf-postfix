use std::collections::HashMap;

use kf_compiler::{
    lex_program,
    Token
};

fn precedencia(op: &Token) -> u8 {
    match op {
        Token::Plus | Token::Minus => 1,
        Token::Mult | Token::By => 2,
        Token::Expo => 3,
        _ => 0,
    }
}

pub fn infija_a_prefija(infija: &str) -> Vec<Token> {
    let mut tokens = lex_program(infija);
    
    // PASO 1: Invertir tokens y cambiar de lado los paréntesis
    tokens.reverse();
    for t in &mut tokens {
        match t.token {
            Token::LeftParen => t.token = Token::RightParen,
            Token::RightParen => t.token = Token::LeftParen,
            _ => {}
        }
    }

    let mut prefija: Vec<Token> = Vec::new(); 
    let mut pila: Vec<Token> = Vec::new(); 
    let mut buffer_termino: Vec<Token> = Vec::new(); 

    for token in tokens {
        if token.token.name() == "Identifier" || token.token.name() == "IntegerLiteral" || token.token.name() == "FloatLiteral" {
            buffer_termino.push(token.token);
        } else {
            if !buffer_termino.is_empty() {
                prefija.append(&mut buffer_termino);
            }

            match token.token {
                Token::LeftParen => {
                    pila.push(token.token);
                }
                Token::RightParen => {
                    while let Some(top) = pila.pop() {
                        if top == Token::LeftParen { break; }
                        prefija.push(top);
                    }
                },
                Token::Plus | Token::Minus | Token::Mult | Token::By | Token::Expo => {
                    while let Some(top) = pila.last() {
                        if *top == Token::LeftParen { break; }
                        // NOTA CLAVE: En prefija se compara con MAYOR estricto (>)
                        if precedencia(top) > precedencia(&token.token) {
                            prefija.push(pila.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    pila.push(token.token);
                },
                _ => {} 
            }
        }
    }

    if !buffer_termino.is_empty() {
        prefija.append(&mut buffer_termino);
    }

    while let Some(op) = pila.pop() {
        if op != Token::LeftParen {
            prefija.push(op);
        }
    }

    // PASO 2: Invertir el resultado final
    prefija.reverse();
    prefija
}

pub fn token_vec_to_string(vec: &Vec<Token>) -> String {
    let mut res: String = String::new();
    for token in vec {
        res += &(token.value() + " ");
    }
    res
}

pub fn get_identifiers(vec: &[Token]) -> Vec<String> {
    let mut ids = Vec::new();
    for token in vec {
        if token.name() == "Identifier" {
            let name = token.value();
            if !ids.contains(&name) {
                ids.push(name);
            }
        }
    }
    ids
}

pub fn eval_prefix(vec: &[Token], vars: &HashMap<String, f32>) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::new();

    // NOTA CLAVE: La prefija se evalúa leyendo de DERECHA a IZQUIERDA (.rev())
    for token in vec.iter().rev() {
        
        if token.name() == "Identifier" {
            match vars.get(&token.value()) {
                Some(&val) => stack.push(val),
                None => return Err(format!("Variable '{}' sin valor", token.value())),
            }
        } else if token.name() == "IntegerLiteral" || token.name() == "FloatLiteral" {
            match token.value().parse::<f32>() {
                Ok(val) => stack.push(val),
                Err(_)=> return Err(format!("Error con {}", token.value())),
            }
        } else {

            let result = match token {
                Token::Plus => {
                    if stack.len() < 2 {
                        stack.pop().ok_or("Stack vacío")?
                    } else {
                        // En prefija, al leer al revés, el val1 sale primero de la pila
                        let val1 = stack.pop().unwrap();
                        let val2 = stack.pop().unwrap();
                        val1 + val2
                    }
                }
                Token::Minus => {
                    if stack.len() < 2 {
                        -stack.pop().ok_or("Stack vacío")?
                    } else {
                        let val1 = stack.pop().unwrap();
                        let val2 = stack.pop().unwrap();
                        val1 - val2
                    }
                }
                _ => {
                    let val1 = stack.pop().ok_or("Stack vacio")?;
                    let val2 = stack.pop().ok_or("Stack vacio")?;
                    match token {
                        Token::Mult => {
                            val1 * val2
                        }
                        Token::By => {
                            val1 / val2
                        }
                        Token::Expo => {
                            val1.powf(val2)
                        },
                        _ => return Err("Operador desconocido".to_string()),
                    }
                }
            };
            stack.push(result);
        }
    }
    stack.pop().ok_or("Expresión inválida".to_string())
}