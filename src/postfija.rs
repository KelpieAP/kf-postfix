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


pub fn infija_a_postfija(infija: &str) -> Vec<Token> {
    let tokens = lex_program(infija);
    dbg!(&tokens);
    let mut postfija: Vec<Token> = Vec::new(); //resultado final
    let mut pila: Vec<Token> = Vec::new(); //retiene operadores temporalmente
    let mut buffer_termino: Vec<Token> = Vec::new(); //agrupa cosas que no son operadores

    let mut prev_op = false;

    for token in tokens {
        //guarda numeros o variables
        if token.token.name() == "Identifier" || token.token.name() == "IntegerLiteral" || token.token.name() == "FloatLiteral" {
            buffer_termino.push(token.token);
            prev_op = true;
        } else {
            //detecta operador, se vacia
            if !buffer_termino.is_empty() {
                postfija.append(&mut buffer_termino);
            }

            match token.token {
                Token::Minus if !prev_op => {
                    postfija.push(Token::IntegerLiteral(0));
                    pila.push(token.token);
                    prev_op = false;
                }
                Token::LeftParen => {
                    pila.push(token.token);
                    prev_op = false;
                }
                Token::RightParen => {
                    while let Some(top) = pila.pop() {
                        if top == Token::LeftParen { break; }
                        postfija.push(top);
                    }
                    prev_op = true;
                },
                //se acomodan operadores
                Token::Plus | Token::Minus | Token::Mult | Token::By | Token::Expo => {
                    while let Some(top) = pila.last() {
                        if *top == Token::LeftParen { break; }
                        if precedencia(top) >= precedencia(&token.token) {
                            postfija.push(pila.pop().unwrap());
                        } else {
                            break;
                        }
                    }
                    pila.push(token.token);
                    prev_op = false;
                },
                _ => {} 
            }
        }
    }

    if !buffer_termino.is_empty() {
        postfija.append(&mut buffer_termino);
    }

    while let Some(op) = pila.pop() {
        if op != Token::LeftParen {
            postfija.push(op);
        }
    }
    postfija
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

pub fn eval_postfix(vec: &[Token], vars: &HashMap<String, f32>) -> Result<f32, String> {
    let mut stack: Vec<f32> = Vec::new();

    for token in vec {
        // checar valores
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
                        let val2 = stack.pop().unwrap();
                        let val1 = stack.pop().unwrap();
                        val1 + val2
                    }
                }
                Token::Minus => {
                    if stack.len() < 2 {
                        -stack.pop().ok_or("Stack vacío")?
                    } else {
                        let val2 = stack.pop().unwrap();
                        let val1 = stack.pop().unwrap();
                        val1 - val2
                    }
                }
                _ => {
                    let val2 = stack.pop().ok_or("Stack vacio")?;
                    let val1 = stack.pop().ok_or("Stack vacio")?;
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
