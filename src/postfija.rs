fn precedencia(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

pub fn infija_a_postfija(infija: &str) -> String {
    let mut postfija = Vec::new();
    let mut pila: Vec<char> = Vec::new();
    let mut buffer_termino = String::new();

    for c in infija.chars() {
        if c.is_whitespace() { continue; }

        if c.is_alphanumeric() {
            buffer_termino.push(c);
        } else {
            if !buffer_termino.is_empty() {
                postfija.push(buffer_termino.clone());
                buffer_termino.clear();
            }

            match c {
                '(' => pila.push(c),
                ')' => {
                    while let Some(top) = pila.pop() {
                        if top == '(' { break; }
                        postfija.push(top.to_string());
                    }
                },
                '+' | '-' | '*' | '/' | '^' => {
                    while let Some(&top) = pila.last() {
                        if top == '(' { break; }
                        if precedencia(top) >= precedencia(c) {
                            postfija.push(pila.pop().unwrap().to_string());
                        } else {
                            break;
                        }
                    }
                    pila.push(c);
                },
                _ => {} 
            }
        }
    }

    if !buffer_termino.is_empty() {
        postfija.push(buffer_termino);
    }

    while let Some(op) = pila.pop() {
        if op != '(' {
            postfija.push(op.to_string());
        }
    }
    postfija.join(" ")
}