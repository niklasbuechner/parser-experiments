#[derive(Debug)]
struct Symbol<'a> {
    pub id: &'a str,
    pub is_keyword: bool,
}

fn main() {
    // Reserved keywords: if, then, else
    let code = "if hello then hi else bye";
    let characters: Vec<char> = code.chars().collect();

    let mut lexem_begin = 0;
    let mut forward = 0;
    let mut state = 0;
    let mut symbols = vec![
        Symbol { id: "if", is_keyword: true },
        Symbol { id: "then", is_keyword: true },
        Symbol { id: "else", is_keyword: true },
    ];

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
                        Some(i) => match symbols[i].id {
                            "if" => tokens.push(("IF", -1)),
                            "then" => tokens.push(("THEN", -1)),
                            "else" => tokens.push(("ELSE", -1)),
                            _ => {},
                        },
                        None => {
                            tokens.push(("ID", symbols.len() as i32));
                            symbols.push(Symbol{ id: lexem, is_keyword: false });
                        },
                    }

                    lexem_begin = forward;
                    state = 0;
                },
            },
            _ => {
                state = 0;
                lexem_begin = forward;
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
