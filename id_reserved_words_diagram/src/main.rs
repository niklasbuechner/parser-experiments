#[derive(Debug)]
struct Symbol<'a> {
    pub id: &'a str,
    pub is_keyword: bool,
}

fn main() {
    // Reserved keywords: if, then, else
    let code = "if hello then hi else bye - ifHello thenHello elseHello the ele";
    let characters: Vec<char> = code.chars().collect();

    let mut lexem_begin = 0;
    let mut forward = 0;
    let mut state = 0;
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut tokens: Vec<(&str, i32)> = Vec::new();
    loop {
        let input;
        if forward == characters.len() {
            input = '\n'; // This should be eof
        } else if forward < characters.len() {
            input = characters[forward];
        } else {
            break;
        }

        match state {
            0 => match input {
                'i' => state = 12,
                't' => state = 14,
                'e' => state = 18,
                'a'..='z' | 'A'..='Z' => {
                    state = 10;
                },
                _ => {
                    state = 0;
                    // +1 because the character forward is pointing does not lead to a new knot
                    // therefore it is not the start of a lexem.
                    // e.g. " if" - the space is not part of the lexem but forward points to it.
                    lexem_begin = forward + 1;
                    // println!("NEXT - lexem_begin: {}", lexem_begin);
                },
            },
            10 => match input {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {},
                _ => {
                    forward -= 1;
                    // lexem_begin + 1 since the index will be included
                    let lexem = &code[lexem_begin..forward+1];
                    // println!("{} - \"{}\"", forward, lexem);

                    let mut index = None;
                    for i in 0..symbols.len() {
                        let symbol = &symbols[i];
                        if symbol.id == lexem {
                            index = Some(i);
                        }
                    }

                    match index {
                        Some(i) => tokens.push(("ID", i as i32)),
                        None => {
                            tokens.push(("ID", symbols.len() as i32));
                            symbols.push(Symbol{ id: lexem, is_keyword: false });
                        },
                    }

                    lexem_begin = forward + 1;
                    state = 0;
                },
            },
            12 => match input {
                'f' => state = 13,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            13 => match input {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    forward -= 1;
                    state = 10;
                },
                _ => {
                    tokens.push(("IF", -1));

                    lexem_begin = forward + 1;
                    state = 0;
                },
            },
            14 => match input {
                'h' => state = 15,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            15 => match input {
                'e' => state = 16,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            16 => match input {
                'n' => state = 17,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            17 => match input {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    forward -= 1;
                    state = 10;
                },
                _ => {
                    tokens.push(("THEN", -1));

                    lexem_begin = forward + 1;
                    state = 0;
                },
            },
            18 => match input {
                'l' => state = 19,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            19 => match input {
                's' => state = 20,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            20 => match input {
                'e' => state = 21,
                _ => {
                    forward -= 1;
                    state = 10;
                },
            },
            21 => match input {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    forward -= 1;
                    state = 10;
                },
                _ => {
                    tokens.push(("ELSE", -1));

                    lexem_begin = forward + 1;
                    state = 0;
                },
            },
            _ => {
                state = 0;
                lexem_begin = forward + 1;
            },
        };

        forward += 1;
    }

    println!("{}", code);
    for token in tokens {
        if token.0 == "ID" {
            println!("ID - \"{}\"", symbols[token.1 as usize].id);
        } else {
            println!("{}", token.0);
        }
    }
}
