mod grammar;
mod test;

use grammar::Grammar;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Token {
    Id,
    Plus,
    Times,
    OpeningBracket,
    ClosingBracket,
    EndSymbol,
}

#[derive(Eq, Hash, PartialEq)]
enum NonTerminal {
    E,
    T,
    F,
}
impl NonTerminal {
    pub fn to_string(&self) -> String {
        match self {
            NonTerminal::E => "E".to_string(),
            NonTerminal::T => "T".to_string(),
            NonTerminal::F => "F".to_string(),
        }
    }
}

#[derive(PartialEq)]
enum Reaction {
    ShiftTo(usize),
    ReduceToAndPopStackBy(NonTerminal, usize),
    Accept,
}

fn main() {
    let token_stream = vec![Token::Id, Token::Times, Token::Id, Token::EndSymbol];
    let grammar = Grammar::new();

    let output = parse_string(&token_stream, grammar);

    println!("{}", output);
}

fn parse_string(tokens: &Vec<Token>, grammar: Grammar) -> String {
    let goto_table = grammar.get_goto_table();
    let parser_table = grammar.get_slr_table();
    let mut index = 0;
    let mut state_stack = Vec::with_capacity(100);
    state_stack.push(grammar.starting_state);

    let mut output = String::new();

    loop {
        let symbol = &tokens[index];
        let current_state = state_stack[state_stack.len() - 1];
        let reaction = parser_table[current_state].get(symbol);

        if reaction == None {
            println!("{}", output);
            println!("Symbol {:#?} - Index: {} - Current state: {}", symbol, index, current_state);
            panic!("Syntax error");
        }

        match reaction.unwrap() {
            Reaction::ShiftTo(next_state) => {
                output.push_str(&format!("Shift to {}\n", next_state));
                state_stack.push(*next_state);
                index += 1;
            }
            Reaction::ReduceToAndPopStackBy(terminal, amount_of_elements_on_stack) => {
                output.push_str(&format!(
                    "Reduce to {} and pop stack by {}\n",
                    terminal.to_string(),
                    amount_of_elements_on_stack
                ));
                for _ in 0..*amount_of_elements_on_stack {
                    state_stack.pop();
                }

                let actual_state = state_stack[state_stack.len() - 1];
                let potential_next_state = goto_table[actual_state].get(terminal);
                if let Some(next_state) = potential_next_state {
                    output.push_str(&format!("Go to {}\n", next_state));
                    state_stack.push(*next_state);
                } else {
                    output.push_str(&format!("No move found. State: {}\n", actual_state));
                }
            }
            Reaction::Accept => {
                output.push_str("Accept\n");
                break;
            }
        }
    }

    output
}
