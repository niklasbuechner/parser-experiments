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
        let e_opening_bracket_and_id_production = vec![
            GrammarSymbol::NonTerminal("T".to_string()),
            GrammarSymbol::NonTerminal("E'".to_string()),
        ];

        let e_stroke_plus_production = vec![
            GrammarSymbol::Terminal(Token::Plus),
            GrammarSymbol::NonTerminal("T".to_string()),
            GrammarSymbol::NonTerminal("E'".to_string()),
        ];

        let t_opening_bracket_and_id_production = vec![
            GrammarSymbol::NonTerminal("F".to_string()),
            GrammarSymbol::NonTerminal("T'".to_string()),
        ];

        let t_stroke_times_production = vec![
            GrammarSymbol::Terminal(Token::Times),
            GrammarSymbol::NonTerminal("F".to_string()),
            GrammarSymbol::NonTerminal("T'".to_string()),
        ];

        let f_id_production = vec![GrammarSymbol::Terminal(Token::Id)];
        let f_opening_bracket_production = vec![
            GrammarSymbol::Terminal(Token::OpenBracket),
            GrammarSymbol::NonTerminal("E".to_string()),
            GrammarSymbol::Terminal(Token::ClosingBracket),
        ];

        let mut e_line = HashMap::new();
        e_line.insert(
            Token::OpenBracket,
            e_opening_bracket_and_id_production.clone(),
        );
        e_line.insert(Token::Id, e_opening_bracket_and_id_production);

        let mut e_stroke_line = HashMap::new();
        e_stroke_line.insert(Token::ClosingBracket, Vec::new());
        e_stroke_line.insert(Token::Plus, e_stroke_plus_production);
        e_stroke_line.insert(Token::EndSymbol, Vec::new());

        let mut t_line = HashMap::new();
        t_line.insert(
            Token::OpenBracket,
            t_opening_bracket_and_id_production.clone(),
        );
        t_line.insert(Token::Id, t_opening_bracket_and_id_production);

        let mut t_stroke_line = HashMap::new();
        t_stroke_line.insert(Token::ClosingBracket, Vec::new());
        t_stroke_line.insert(Token::Plus, Vec::new());
        t_stroke_line.insert(Token::Times, t_stroke_times_production);
        t_stroke_line.insert(Token::EndSymbol, Vec::new());

        let mut f_line = HashMap::new();
        f_line.insert(Token::Id, f_id_production);
        f_line.insert(Token::OpenBracket, f_opening_bracket_production);

        let mut table = HashMap::new();
        table.insert("E".to_string(), e_line);
        table.insert("E'".to_string(), e_stroke_line);
        table.insert("T".to_string(), t_line);
        table.insert("T'".to_string(), t_stroke_line);
        table.insert("F".to_string(), f_line);

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

        let non_terminal_symbol = GrammarSymbol::NonTerminal(non_terminal.to_string());
        'non_terminals: for current_non_terminal in self.productions.keys() {
            let productions = self.productions.get(current_non_terminal).unwrap();

            'productions: for production in productions {
                if !production.contains(&non_terminal_symbol) {
                    continue 'productions;
                }

                let index = production
                    .iter()
                    .position(|symbol| symbol == &non_terminal_symbol)
                    .unwrap();
                if index == production.len() - 1 {
                    if current_non_terminal != non_terminal {
                        let follow_of_parent_production = self.get_follow(current_non_terminal);
                        'follow_of_parent: for element in &follow_of_parent_production {
                            if follow.contains(element) {
                                continue 'follow_of_parent;
                            }

                            follow.push(element.clone());
                        }
                    }

                    continue 'productions;
                }

                let production_slice = &production[index + 1..].to_vec();
                let first = self.get_first_for_string(production_slice);
                'first: for element in &first {
                    if element == &Token::EmptySet {
                        if current_non_terminal != non_terminal {
                            let follow_of_parent_production = self.get_follow(current_non_terminal);
                            'empty_follow_of_parent: for element in &follow_of_parent_production {
                                if follow.contains(element) {
                                    continue 'empty_follow_of_parent;
                                }

                                follow.push(element.clone());
                            }
                        }

                        continue 'first;
                    }

                    if follow.contains(element) {
                        continue 'first;
                    }

                    follow.push(element.clone());
                }
            }
        }

        follow
    }
}
