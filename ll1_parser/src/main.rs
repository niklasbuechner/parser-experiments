mod grammar;

use grammar::Grammar;
use grammar::GrammarSymbol;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
enum Token {
    Id,
    OpenBracket,
    ClosingBracket,
    Plus,
    Times,
    EndSymbol,
}

fn main() {
    let grammar = Grammar::new();

    // First test input string
    // let inputs = vec![
    //     Token::Id,
    //     Token::Plus,
    //     Token::Id,
    //     Token::Times,
    //     Token::Id,
    //     Token::EndSymbol,
    // ];

    // Second test input string
    let inputs = vec![
        Token::OpenBracket,
        Token::Id,
        Token::Times,
        Token::Id,
        Token::ClosingBracket,
        Token::EndSymbol,
    ];

    parse_string(&grammar, &inputs);
}

fn parse_string(grammar: &Grammar, input: &Vec<Token>) {
    let parse_table = grammar.get_parse_table();
    let mut input_pointer = 0;
    let mut reverse_stack = vec![
        GrammarSymbol::EndSymbol,
        GrammarSymbol::NonTerminal("E".to_string()),
    ];

    let mut input_token = &input[input_pointer];
    while reverse_stack.len() > 1 {
        let current_element = &reverse_stack[reverse_stack.len() - 1];
        match current_element {
            GrammarSymbol::Terminal(token) => {
                if token == input_token {
                    println!("Consume token {:?}", token);
                    input_pointer += 1;
                    reverse_stack.pop();
                } else {
                    panic!("Unexpected token {:?}", token);
                }
            }
            GrammarSymbol::NonTerminal(name) => {
                let productions = parse_table.get(name);
                if productions == None {
                    panic!("Productions for non terminal {} do not exist", name);
                }

                let production = productions.unwrap().get(input_token);
                if production == None {
                    panic!(
                        "No production for token {:?} in non terminal {} exists",
                        input_token, name
                    );
                }

                let symbol = reverse_stack.pop();
                let tokens = production.unwrap();
                println!("Replaced symbol {:?} with {:?}", symbol, tokens);

                for reverse_index in 0..tokens.len() {
                    let index = tokens.len() - 1 - reverse_index;

                    reverse_stack.push(tokens[index].clone());
                }
            }
            GrammarSymbol::EndSymbol => {
                if input_token == &Token::EndSymbol {
                    break;
                } else {
                    panic!("Last token is not end symbol.");
                }
            }
        }

        input_token = &input[input_pointer];
    }

    println!("Parsing complete");
}
