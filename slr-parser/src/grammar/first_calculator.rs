use super::Grammar;
use super::GrammarSymbol;
use super::Token;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub(crate) struct FirstElements {
    pub empty_set: bool,
    pub tokens: Vec<Token>,
}

#[derive(PartialEq)]
pub(crate) struct First {
    firsts_of_non_terminals: HashMap<String, Vec<Token>>,
    is_non_terminal_nullable: HashMap<String, bool>,
}
impl First {
    pub fn get_first(grammar: &Grammar, non_terminal: &str) -> FirstElements {
        let mut first = First {
            firsts_of_non_terminals: HashMap::new(),
            is_non_terminal_nullable: HashMap::new(),
        };
        first.calculate_first(grammar, non_terminal);

        FirstElements {
            tokens: first
                .firsts_of_non_terminals
                .get(non_terminal)
                .unwrap()
                .clone(),
            empty_set: *first.is_non_terminal_nullable.get(non_terminal).unwrap(),
        }
    }

    pub fn add_token(&mut self, non_terminal: &str, token: &Token) {
        let tokens = self.firsts_of_non_terminals.get_mut(non_terminal).unwrap();

        tokens.push(token.clone());
    }

    pub fn calculate_first(&mut self, grammar: &Grammar, non_terminal: &str) {
        // Prevent endless loops where a production A -> ... depends on itself.
        self.firsts_of_non_terminals
            .insert(non_terminal.to_string(), Vec::new());

        self.calculate_if_non_terminal_is_nullable(grammar, non_terminal);

        let productions = grammar.get_production(non_terminal);
        'productions: for production in productions {
            for element in production {
                match element {
                    GrammarSymbol::Terminal(token) => {
                        self.add_token(non_terminal, token);

                        continue 'productions;
                    }
                    GrammarSymbol::NonTerminal(name) => {
                        let potential_terminals = self.firsts_of_non_terminals.get(name);
                        if potential_terminals == None {
                            self.calculate_first(grammar, name);
                        }

                        let terminals = self.firsts_of_non_terminals.get(name).unwrap();
                        for token in &terminals.clone() {
                            self.add_token(non_terminal, token);
                        }

                        if *self.is_non_terminal_nullable.get(name).unwrap() == false {
                            continue 'productions;
                        }
                    }
                }
            }
        }
    }

    pub fn calculate_if_non_terminal_is_nullable(&mut self, grammar: &Grammar, non_terminal: &str) {
        // Prevent endless loops where a production A -> ... depends on itself.
        self.is_non_terminal_nullable
            .insert(non_terminal.to_string(), false);

        let productions = grammar.get_production(non_terminal);
        'productions: for production in productions {
            for element in production {
                match element {
                    GrammarSymbol::Terminal(_) => continue 'productions,
                    GrammarSymbol::NonTerminal(name) => {
                        if self.is_non_terminal_nullable.get(name) == None {
                            self.calculate_if_non_terminal_is_nullable(grammar, name);
                        }

                        if *self.is_non_terminal_nullable.get(name).unwrap() == false {
                            continue 'productions;
                        }
                    }
                }
            }

            // This production does not contain a single element which was not nullable.
            // (This might be because is does not contain any element). Therefore, the whole
            // production is nullable.
            self.is_non_terminal_nullable
                .insert(non_terminal.to_string(), true);

            return;
        }

        self.is_non_terminal_nullable
            .insert(non_terminal.to_string(), false);
    }
}
