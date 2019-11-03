use super::First;
use super::Grammar;
use super::GrammarSymbol;
use super::Token;
use std::collections::HashMap;

pub(crate) struct Follow {
    follow_of_non_terminal: HashMap<String, Vec<Token>>,
}
impl Follow {
    pub fn get_follow(grammar: &Grammar, non_terminal: &str) -> Vec<Token> {
        let mut follow = Follow {
            follow_of_non_terminal: HashMap::new(),
        };

        follow.calculate_follow(grammar, non_terminal);

        follow
            .follow_of_non_terminal
            .get(non_terminal)
            .unwrap()
            .to_vec()
    }

    fn calculate_follow(&mut self, grammar: &Grammar, non_terminal: &str) {
        let mut follow = Vec::new();
        self.follow_of_non_terminal
            .insert(non_terminal.to_string(), follow.clone());

        if non_terminal == grammar.get_starting_non_terminal() {
            follow.push(grammar.get_end_symbol());
        }

        let symbol = GrammarSymbol::non_terminal(non_terminal);
        let productions = self.get_productions_with_symbol(grammar, &symbol);
        for (current_non_terminal, production) in productions {
            let substring = self.get_string_after_symbol(production, &symbol);

            let first = First::get_first_for_string(grammar, &substring);
            first.tokens.iter().for_each(|token| {
                if !follow.contains(token) {
                    follow.push(token.clone())
                }
            });

            if first.empty_set == true {
                if None == self.follow_of_non_terminal.get(&current_non_terminal) {
                    self.calculate_follow(grammar, &current_non_terminal);
                }

                let follow_of_parent = self
                    .follow_of_non_terminal
                    .get(&current_non_terminal)
                    .unwrap();
                follow_of_parent.iter().for_each(|token| {
                    if !follow.contains(token) {
                        follow.push(token.clone())
                    }
                });
            }
        }

        self.follow_of_non_terminal
            .insert(non_terminal.to_string(), follow);
    }

    fn get_productions_with_symbol<'a>(
        &self,
        grammar: &'a Grammar,
        symbol: &GrammarSymbol,
    ) -> Vec<(String, &'a Vec<GrammarSymbol>)> {
        let mut productions_with_symbol = Vec::new();

        let non_terminals = grammar.get_non_terminals();
        for single_non_terminal in non_terminals {
            grammar
                .get_production(&single_non_terminal)
                .into_iter()
                .filter(|production| production.contains(&symbol))
                .for_each(|production| {
                    productions_with_symbol.push((single_non_terminal.clone(), production))
                });
        }

        productions_with_symbol
    }

    fn get_string_after_symbol(
        &self,
        production: &Vec<GrammarSymbol>,
        symbol: &GrammarSymbol,
    ) -> Vec<GrammarSymbol> {
        let position = production.iter().position(|element| element == symbol);
        if position == None {
            return Vec::new();
        }

        production[position.unwrap() + 1..].to_vec()
    }
}
