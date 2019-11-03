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

    pub fn get_first_for_string(grammar: &Grammar, string: &Vec<GrammarSymbol>) -> FirstElements {
        let mut first = First {
            firsts_of_non_terminals: HashMap::new(),
            is_non_terminal_nullable: HashMap::new(),
        };
        let tokens = first.calculate_first_for_string(grammar, string);

        FirstElements {
            tokens,
            empty_set: first.calculate_nullability_for_string(grammar, string),
        }
    }

    pub fn add_token(&mut self, non_terminal: &str, token: Token) {
        let tokens = self.firsts_of_non_terminals.get_mut(non_terminal).unwrap();

        tokens.push(token);
    }

    pub fn calculate_first(&mut self, grammar: &Grammar, non_terminal: &str) {
        // Prevent endless loops where a production A -> ... depends on itself.
        self.firsts_of_non_terminals
            .insert(non_terminal.to_string(), Vec::new());

        self.calculate_if_non_terminal_is_nullable(grammar, non_terminal);

        let productions = grammar.get_production(non_terminal);
        'productions: for production in productions {
            self.calculate_first_for_string(grammar, production.get_elements())
                .into_iter()
                .for_each(|token| self.add_token(non_terminal, token));
        }
    }

    pub fn calculate_first_for_string(
        &mut self,
        grammar: &Grammar,
        string: &Vec<GrammarSymbol>,
    ) -> Vec<Token> {
        let mut tokens = Vec::new();
        for element in string {
            match element {
                GrammarSymbol::Terminal(token) => {
                    tokens.push(token.clone());

                    return tokens;
                }
                GrammarSymbol::NonTerminal(name) => {
                    let potential_terminals = self.firsts_of_non_terminals.get(name);
                    if potential_terminals == None {
                        self.calculate_first(grammar, name);
                    }

                    let terminals = self.firsts_of_non_terminals.get(name).unwrap();
                    for token in &terminals.clone() {
                        tokens.push(token.clone());
                    }

                    if *self.is_non_terminal_nullable.get(name).unwrap() == false {
                        return tokens;
                    }
                }
            }
        }

        return tokens;
    }

    pub fn calculate_if_non_terminal_is_nullable(&mut self, grammar: &Grammar, non_terminal: &str) {
        // Prevent endless loops where a production A -> ... depends on itself.
        self.is_non_terminal_nullable
            .insert(non_terminal.to_string(), false);

        let productions = grammar.get_production(non_terminal);
        'productions: for production in productions {
            let is_production_nullable =
                self.calculate_nullability_for_string(grammar, production.get_elements());
            if is_production_nullable {
                self.is_non_terminal_nullable
                    .insert(non_terminal.to_string(), true);

                return;
            } else {
                continue 'productions;
            }
        }

        self.is_non_terminal_nullable
            .insert(non_terminal.to_string(), false);
    }

    pub fn calculate_nullability_for_string(
        &mut self,
        grammar: &Grammar,
        string: &Vec<GrammarSymbol>,
    ) -> bool {
        for element in string {
            match element {
                GrammarSymbol::Terminal(_) => return false,
                GrammarSymbol::NonTerminal(name) => {
                    if self.is_non_terminal_nullable.get(name) == None {
                        self.calculate_if_non_terminal_is_nullable(grammar, name);
                    }

                    if *self.is_non_terminal_nullable.get(name).unwrap() == false {
                        return false;
                    }
                }
            }
        }

        // This production does not contain a single element which was not nullable.
        // (This might be because is does not contain any element). Therefore, the whole
        // production is nullable.
        return true;
    }
}
