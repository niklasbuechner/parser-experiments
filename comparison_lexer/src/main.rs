fn main() {
    let code = "<= <> >= = <= >= < >";
    let characters: Vec<char> = code.chars().collect();

    // Not needed because we never need the lexem.
    // let mut lexem_begin = 0;
    // let mut forward = 0;
    let mut state = 0;

    let mut tokens = Vec::new();
    for i in 0..characters.len() + 1 {
        let input;
        if i == characters.len() {
            input = '\n'; // This should be eof
        } else {
            input = characters[i];
        }
        // forward += 1;

        match state {
            0 => match input {
                '<' => state = 1,
                '=' => {
                    tokens.push("EQUALS");
                    state = 0;
                },
                '>' => state = 6,
                _ => state = 0,
            },
            1 => match input {
                '=' => {
                    tokens.push("LESS_EQUALS");
                    state = 0;
                },
                '>' => {
                    tokens.push("NOT_EQUALS");
                    state = 0;
                },
                _ => {
                    tokens.push("LESS_THAN");
                    state = 0;
                },
            }, 
            6 => match input {
                '=' => {
                    tokens.push("GREATER_EQUALS");
                    state = 0;
                },
                _ => {
                    tokens.push("GREATER_THAN");
                    state = 0;
                },
            },
            _ => state = 0,
        };
    }

    println!("{}", code);
    println!("{:#?}", tokens);
}
