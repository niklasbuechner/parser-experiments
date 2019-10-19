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
                GrammarSymbol::NonTerminal("F'".to_string()),
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

        Grammar { productions }
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

            'elements_in_production: for element in production_elements {
                match element {
                    GrammarSymbol::Terminal(token) => {
                        let new_token = token.clone();
                        if !first.contains(&new_token) {
                            first.push(new_token);
                        }

                        break 'elements_in_production;
                    }
                    GrammarSymbol::NonTerminal(name) => {
                        let first_non_terminal = self.get_first(name);

                        'first_elements: for first_element in &first_non_terminal {
                            if first_element == &Token::EmptySet {
                                continue 'first_elements;
                            }

                            if !first.contains(first_element) {
                                first.push(first_element.clone());
                            }
                        }

                        if !first_non_terminal.contains(&Token::EndSymbol) {
                            break 'elements_in_production;
                        }
                    }
                    GrammarSymbol::EndSymbol => {
                        panic!("No production should contain the end symbol")
                    }
                }
            }
        }

        first
    }
}
