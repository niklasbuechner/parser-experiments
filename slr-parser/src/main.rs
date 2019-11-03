mod ast;
mod grammar;
mod test;

use ast::Ast;
use grammar::FirstElements;
use grammar::Grammar;
use grammar::GrammarSymbol;
use grammar::Production;
use grammar::SlrClosure;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
// enum Token {
//     Id,
//     Plus,
//     Times,
//     OpeningBracket,
//     ClosingBracket,
//     EndSymbol,
// }
struct Token {
    value: String,
    is_end_symbol: bool,
}
impl Token {
    pub fn new(value: &str) -> Self {
        Token {
            value: value.to_string(),
            is_end_symbol: false,
        }
    }

    pub fn end_token(value: &str) -> Self {
        Token {
            value: value.to_string(),
            is_end_symbol: false,
        }
    }
}

#[derive(Clone, PartialEq)]
enum Reaction {
    ShiftTo(usize),
    Reduce(Reduction),
    Accept,
}
#[derive(Clone)]
struct Reduction {
    non_terminal: String,
    amount_of_elements_on_stack: usize,
    ast_function: fn(&mut Vec<Ast>),
}
impl Reduction {
    pub fn new(non_terminal: &str, amount_of_elements_on_stack: usize) -> Self {
        Self::with_function(non_terminal, amount_of_elements_on_stack, |_| {})
    }

    pub fn with_function(
        non_terminal: &str,
        amount_of_elements_on_stack: usize,
        ast_function: fn(&mut Vec<Ast>),
    ) -> Self {
        Reduction {
            non_terminal: non_terminal.to_string(),
            amount_of_elements_on_stack,
            ast_function,
        }
    }
}
impl PartialEq for Reduction {
    fn eq(&self, other: &Self) -> bool {
        return self.non_terminal == other.non_terminal
            && self.amount_of_elements_on_stack == other.amount_of_elements_on_stack;
    }
}

fn main() {
    let token_stream = vec![
        Token::new("Id"),
        Token::new("Times"),
        Token::new("Id"),
        Token::end_token("EndSymbol"),
    ];
    let grammar = Grammar::new();

    let (output, _) = parse_string(&token_stream, grammar);

    println!("{}", output);
}

fn parse_string(tokens: &Vec<Token>, grammar: Grammar) -> (String, Vec<Ast>) {
    let goto_table = grammar.get_goto_table();
    let parser_table = grammar.get_slr_table();
    let mut index = 0;
    let mut ast_stack = Vec::with_capacity(10);
    let mut state_stack = Vec::with_capacity(100);
    state_stack.push(grammar.starting_state);

    let mut output = String::new();

    loop {
        let symbol = &tokens[index];
        let current_state = state_stack[state_stack.len() - 1];
        let reaction = parser_table[current_state].get(symbol);

        if reaction == None {
            println!("{}", output);
            println!(
                "Symbol {:#?} - Index: {} - Current state: {}",
                symbol, index, current_state
            );
            panic!("Syntax error");
        }

        match reaction.unwrap() {
            Reaction::ShiftTo(next_state) => {
                output.push_str(&format!("Shift to {}\n", next_state));
                state_stack.push(*next_state);
                index += 1;
            }
            Reaction::Reduce(reduction) => {
                output.push_str(&format!(
                    "Reduce to {} and pop stack by {}\n",
                    reduction.non_terminal.to_string(),
                    reduction.amount_of_elements_on_stack
                ));

                (reduction.ast_function)(&mut ast_stack);

                for _ in 0..reduction.amount_of_elements_on_stack {
                    state_stack.pop();
                }

                let actual_state = state_stack[state_stack.len() - 1];
                let potential_next_state = goto_table[actual_state].get(&reduction.non_terminal);
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

    (output, ast_stack)
}
