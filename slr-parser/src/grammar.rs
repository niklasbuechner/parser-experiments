mod first;
mod follow;

use super::ast;
use super::Reaction;
use super::Reduction;
use super::Token;
use first::First;
pub(crate) use first::FirstElements;
use follow::Follow;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GrammarSymbol {
    NonTerminal(String),
    Terminal(Token),
}
impl GrammarSymbol {
    pub fn non_terminal(non_terminal: &str) -> Self {
        GrammarSymbol::NonTerminal(non_terminal.to_string())
    }

    pub fn terminal(token: Token) -> Self {
        GrammarSymbol::Terminal(token)
    }
}

pub(crate) struct Grammar {
    productions: HashMap<String, Vec<Vec<GrammarSymbol>>>,
    pub starting_state: usize,
    starting_non_terminal: String,
    end_token: String,
}
impl Grammar {
    pub fn new() -> Self {
        let mut grammar = Grammar::empty();
        grammar.set_end_token("EndSymbol");
        grammar.set_starting_non_terminal("E'");

        grammar.add_production("E'", vec![GrammarSymbol::non_terminal("E")]);

        grammar.add_production(
            "E",
            vec![
                GrammarSymbol::non_terminal("E"),
                GrammarSymbol::terminal(Token::new("Plus")),
                GrammarSymbol::non_terminal("T"),
            ],
        );
        grammar.add_production("E", vec![GrammarSymbol::non_terminal("T")]);

        grammar.add_production(
            "T",
            vec![
                GrammarSymbol::non_terminal("T"),
                GrammarSymbol::terminal(Token::new("Times")),
                GrammarSymbol::non_terminal("F"),
            ],
        );
        grammar.add_production("T", vec![GrammarSymbol::non_terminal("F")]);

        grammar.add_production(
            "F",
            vec![
                GrammarSymbol::terminal(Token::new("OpeningBracket")),
                GrammarSymbol::non_terminal("E"),
                GrammarSymbol::terminal(Token::new("ClosingBracket")),
            ],
        );
        grammar.add_production("F", vec![GrammarSymbol::terminal(Token::new("Id"))]);

        grammar
    }

    pub fn empty() -> Self {
        Grammar {
            end_token: String::new(),
            productions: HashMap::new(),
            starting_state: 0,
            starting_non_terminal: String::new(),
        }
    }

    pub fn add_production(&mut self, non_terminal: &str, production: Vec<GrammarSymbol>) {
        let potential_productions = self.productions.get_mut(non_terminal);
        if let Some(productions) = potential_productions {
            productions.push(production);
        } else {
            self.productions
                .insert(non_terminal.to_string(), vec![production]);
        }
    }

    pub fn set_starting_non_terminal(&mut self, non_terminal: &str) {
        self.starting_non_terminal = non_terminal.to_string();
    }

    pub fn set_end_token(&mut self, end_token: &str) {
        self.end_token = end_token.to_string();
    }

    pub fn get_production(&self, non_terminal: &str) -> &Vec<Vec<GrammarSymbol>> {
        self.productions.get(non_terminal).unwrap()
    }

    pub fn get_first(&self, non_terminal: &str) -> FirstElements {
        First::get_first(&self, non_terminal)
    }

    pub fn get_follow(&self, non_terminal: &str) -> Vec<Token> {
        Follow::get_follow(&self, non_terminal)
    }

    pub fn get_starting_non_terminal(&self) -> &String {
        &self.starting_non_terminal
    }

    pub fn get_end_symbol(&self) -> Token {
        Token::new(&self.end_token)
    }

    pub fn get_non_terminals(&self) -> Vec<&String> {
        self.productions.keys().collect()
    }

    pub fn get_slr_table(&self) -> Vec<HashMap<Token, Reaction>> {
        let mut state_0_tokens = HashMap::new();
        state_0_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
        state_0_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

        let mut state_1_tokens = HashMap::new();
        state_1_tokens.insert(Token::new("Plus"), Reaction::ShiftTo(6));
        state_1_tokens.insert(Token::end_token("EndSymbol"), Reaction::Accept);

        let mut state_2_tokens = HashMap::new();
        state_2_tokens.insert(Token::new("Plus"), Reaction::Reduce(Reduction::new("E", 1)));
        state_2_tokens.insert(Token::new("Times"), Reaction::ShiftTo(7));
        state_2_tokens.insert(
            Token::new("ClosingBracket"),
            Reaction::Reduce(Reduction::new("E", 1)),
        );
        state_2_tokens.insert(
            Token::end_token("EndSymbol"),
            Reaction::Reduce(Reduction::new("E", 1)),
        );

        let mut state_3_tokens = HashMap::new();
        state_3_tokens.insert(Token::new("Plus"), Reaction::Reduce(Reduction::new("T", 1)));
        state_3_tokens.insert(
            Token::new("Times"),
            Reaction::Reduce(Reduction::new("T", 1)),
        );
        state_3_tokens.insert(
            Token::new("ClosingBracket"),
            Reaction::Reduce(Reduction::new("T", 1)),
        );
        state_3_tokens.insert(
            Token::end_token("EndSymbol"),
            Reaction::Reduce(Reduction::new("T", 1)),
        );

        let mut state_4_tokens = HashMap::new();
        state_4_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
        state_4_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

        let mut state_5_tokens = HashMap::new();
        let reduce_to_number =
            Reaction::Reduce(Reduction::with_function("F", 1, ast::ast_create_number));
        state_5_tokens.insert(Token::new("Plus"), reduce_to_number.clone());
        state_5_tokens.insert(Token::new("Times"), reduce_to_number.clone());
        state_5_tokens.insert(Token::new("ClosingBracket"), reduce_to_number.clone());
        state_5_tokens.insert(Token::end_token("EndSymbol"), reduce_to_number.clone());

        let mut state_6_tokens = HashMap::new();
        state_6_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
        state_6_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

        let mut state_7_tokens = HashMap::new();
        state_7_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
        state_7_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

        let mut state_8_tokens = HashMap::new();
        state_8_tokens.insert(Token::new("Plus"), Reaction::ShiftTo(6));
        state_8_tokens.insert(Token::new("ClosingBracket"), Reaction::ShiftTo(11));

        let mut state_9_tokens = HashMap::new();
        let reduce_to_addition =
            Reaction::Reduce(Reduction::with_function("E", 3, ast::ast_create_addition));
        state_9_tokens.insert(Token::new("Plus"), reduce_to_addition.clone());
        state_9_tokens.insert(Token::new("Times"), Reaction::ShiftTo(7));
        state_9_tokens.insert(Token::new("ClosingBracket"), reduce_to_addition.clone());
        state_9_tokens.insert(Token::end_token("EndSymbol"), reduce_to_addition.clone());

        let mut state_10_tokens = HashMap::new();
        let reduce_to_multiplication = Reaction::Reduce(Reduction::with_function(
            "T",
            3,
            ast::ast_create_multiplication,
        ));
        state_10_tokens.insert(Token::new("Plus"), reduce_to_multiplication.clone());
        state_10_tokens.insert(Token::new("Times"), reduce_to_multiplication.clone());
        state_10_tokens.insert(
            Token::new("ClosingBracket"),
            reduce_to_multiplication.clone(),
        );
        state_10_tokens.insert(
            Token::end_token("EndSymbol"),
            reduce_to_multiplication.clone(),
        );

        let mut state_11_tokens = HashMap::new();
        let reduce_to_calculation = Reaction::Reduce(Reduction::with_function(
            "F",
            3,
            ast::ast_create_calculation,
        ));
        state_11_tokens.insert(Token::new("Plus"), reduce_to_calculation.clone());
        state_11_tokens.insert(Token::new("Times"), reduce_to_calculation.clone());
        state_11_tokens.insert(Token::new("ClosingBracket"), reduce_to_calculation.clone());
        state_11_tokens.insert(Token::end_token("EndSymbol"), reduce_to_calculation.clone());

        let mut table: Vec<HashMap<Token, Reaction>> = Vec::new();
        table.push(state_0_tokens);
        table.push(state_1_tokens);
        table.push(state_2_tokens);
        table.push(state_3_tokens);
        table.push(state_4_tokens);
        table.push(state_5_tokens);
        table.push(state_6_tokens);
        table.push(state_7_tokens);
        table.push(state_8_tokens);
        table.push(state_9_tokens);
        table.push(state_10_tokens);
        table.push(state_11_tokens);

        table
    }

    pub fn get_goto_table(&self) -> Vec<HashMap<String, usize>> {
        let mut state_0_goto = HashMap::new();
        state_0_goto.insert("E".to_string(), 1);
        state_0_goto.insert("T".to_string(), 2);
        state_0_goto.insert("F".to_string(), 3);

        let mut state_4_goto = HashMap::new();
        state_4_goto.insert("E".to_string(), 8);
        state_4_goto.insert("T".to_string(), 2);
        state_4_goto.insert("F".to_string(), 3);

        let mut state_6_goto = HashMap::new();
        state_6_goto.insert("T".to_string(), 9);
        state_6_goto.insert("F".to_string(), 3);

        let mut state_7_goto = HashMap::new();
        state_7_goto.insert("F".to_string(), 10);

        let mut table: Vec<HashMap<String, usize>> = Vec::new();
        table.push(state_0_goto);
        table.push(HashMap::new());
        table.push(HashMap::new());
        table.push(HashMap::new());
        table.push(state_4_goto);
        table.push(HashMap::new());
        table.push(state_6_goto);
        table.push(state_7_goto);
        table.push(HashMap::new());
        table.push(HashMap::new());
        table.push(HashMap::new());
        table.push(HashMap::new());

        table
    }
}
