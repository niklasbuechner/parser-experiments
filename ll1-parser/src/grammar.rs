use super::Token;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub(crate) enum GrammarSymbol {
    Terminal(Token),
    NonTerminal(String),
    EndSymbol,
}

pub(crate) struct Grammar {
    productions: HashMap<String, Vec<Vec<GrammarSymbol>>>,
    start_symbol: String,
}
impl Grammar {
    pub fn new() -> Self {
        let mut productions = HashMap::new();
        productions.insert(
            "E".to_string(),
            vec![vec![
                GrammarSymbol::NonTerminal("T".to_string()),
                GrammarSymbol::NonTerminal("E'".to_string()),
            ]],
        );
        productions.insert(
            "E'".to_string(),
            vec![
                vec![
                    GrammarSymbol::Terminal(Token::Plus),
                    GrammarSymbol::NonTerminal("T".to_string()),
                    GrammarSymbol::NonTerminal("E'".to_string()),
                ],
                Vec::new(),
            ],
        );
        productions.insert(
            "T".to_string(),
            vec![vec![
                GrammarSymbol::NonTerminal("F".to_string()),
                GrammarSymbol::NonTerminal("T'".to_string()),
            ]],
        );
        productions.insert(
            "T'".to_string(),
            vec![
                vec![
                    GrammarSymbol::Terminal(Token::Times),
                    GrammarSymbol::NonTerminal("F".to_string()),
                    GrammarSymbol::NonTerminal("T'".to_string()),
                ],
                Vec::new(),
            ],
        );
        productions.insert(
            "F".to_string(),
            vec![
                vec![
                    GrammarSymbol::Terminal(Token::OpenBracket),
                    GrammarSymbol::NonTerminal("E".to_string()),
                    GrammarSymbol::Terminal(Token::ClosingBracket),
                ],
                vec![GrammarSymbol::Terminal(Token::Id)],
            ],
        );

        Grammar {
            productions,
            start_symbol: "E".to_string(),
        }
    }

    pub fn get_parse_table(&self) -> HashMap<String, HashMap<Token, Vec<GrammarSymbol>>> {
        let mut table = HashMap::new();
        self.productions.keys().for_each(|non_terminal| {
            let mut line = HashMap::new();

            let productions = self.productions.get(non_terminal).unwrap();
            productions.iter().for_each(|production| {
                let first = self.get_first_for_string(production);
                first.iter().for_each(|token| {
                    line.insert(token.clone(), production.clone());
                });
            });

            let all_productions_first = self.get_first(non_terminal);
            if all_productions_first.contains(&Token::EmptySet) {
                let follow = self.get_follow(non_terminal);
                follow.into_iter().for_each(|token| {
                    line.insert(token, Vec::new());
                });
            }

            table.insert(non_terminal.clone(), line);
        });

        table
    }

    pub fn get_first(&self, non_terminal: &str) -> Vec<Token> {
        let productions = self.productions.get(non_terminal).unwrap();
        let mut first = Vec::new();

        'productions: for production_elements in productions {
            if production_elements.len() == 0 {
                first.push(Token::EmptySet);
            }

            let mut first_pieces = self.get_first_for_string(production_elements);
            first.append(&mut first_pieces);
        }

        first
    }

    fn get_first_for_string(&self, string: &Vec<GrammarSymbol>) -> Vec<Token> {
        let mut first = Vec::new();
        let mut one_set_does_not_contain_empty_set = false;
        let mut set_contain_empty_set = false;

        'elements_in_production: for element in string {
            match element {
                GrammarSymbol::Terminal(token) => {
                    let new_token = token.clone();
                    if !first.contains(&new_token) {
                        first.push(new_token);
                    }
                    one_set_does_not_contain_empty_set = true;

                    break 'elements_in_production;
                }
                GrammarSymbol::NonTerminal(name) => {
                    let first_non_terminal = self.get_first(&name);
                    let mut contains_empty_set = false;

                    'first_elements: for first_element in &first_non_terminal {
                        if first_element == &Token::EmptySet {
                            contains_empty_set = true;

                            continue 'first_elements;
                        }

                        if !first.contains(first_element) {
                            first.push(first_element.clone());
                        }
                    }

                    if contains_empty_set {
                        set_contain_empty_set = true;
                    } else {
                        one_set_does_not_contain_empty_set = true;
                    }

                    if !first_non_terminal.contains(&Token::EndSymbol) {
                        break 'elements_in_production;
                    }
                }
                GrammarSymbol::EndSymbol => panic!("No production should contain the end symbol"),
            }
        }

        if !one_set_does_not_contain_empty_set && set_contain_empty_set {
            first.push(Token::EmptySet);
        }

        first
    }

    pub fn get_follow(&self, non_terminal: &str) -> Vec<Token> {
        let mut follow = Vec::new();
        if non_terminal == &self.start_symbol {
            follow.push(Token::EndSymbol);
        }

        'non_terminals: for current_non_terminal in self.productions.keys() {
            let productions = self.productions.get(current_non_terminal).unwrap();

            'productions: for production in productions {
                let follow_elements =
                    self.get_follow_for_production(non_terminal, current_non_terminal, production);
                follow_elements.iter().for_each(|element| {
                    self.add_not_contained_element::<Token>(&mut follow, element);
                });
            }
        }

        follow
    }

    fn get_follow_for_production(
        &self,
        non_terminal: &str,
        current_non_terminal: &str,
        production: &Vec<GrammarSymbol>,
    ) -> Vec<Token> {
        let non_terminal_symbol = GrammarSymbol::NonTerminal(non_terminal.to_string());
        if !production.contains(&non_terminal_symbol) {
            return Vec::new();
        }

        let index = production
            .iter()
            .position(|symbol| symbol == &non_terminal_symbol)
            .unwrap();
        if index == production.len() - 1 {
            if current_non_terminal != non_terminal {
                return self.get_follow(current_non_terminal);
            }

            return Vec::new();
        }

        let mut follow = Vec::new();
        let production_slice = &production[index + 1..].to_vec();
        let first = self.get_first_for_string(production_slice);
        let mut contains_empty_set = false;
        for element in &first {
            if element == &Token::EmptySet {
                contains_empty_set = true;

                continue;
            }

            self.add_not_contained_element(&mut follow, element);
        }

        if contains_empty_set && current_non_terminal != non_terminal {
            let follow_of_parent_production = self.get_follow(current_non_terminal);
            for element in &follow_of_parent_production {
                self.add_not_contained_element(&mut follow, element);
            }
        }

        follow
    }

    fn add_not_contained_element<T: Clone + PartialEq>(
        &self,
        elements: &mut Vec<T>,
        new_element: &T,
    ) {
        if !elements.contains(new_element) {
            elements.push(new_element.clone())
        }
    }
}
