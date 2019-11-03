mod first;
mod follow;
mod parser_table;
mod slr_closure;
mod slr_goto;

use super::ast;
use super::Ast;
use super::Reaction;
use super::Reduction;
use super::Token;
use first::First;
pub(crate) use first::FirstElements;
use follow::Follow;
pub(crate) use parser_table::ParserTable;
pub(crate) use slr_closure::Production;
pub(crate) use slr_closure::SlrClosure;
pub(crate) use slr_goto::SlrGoto;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    productions: HashMap<String, Vec<Production>>,
    pub starting_state: usize,
    starting_non_terminal: String,
    end_token: String,
}
impl Grammar {
    pub fn new() -> Self {
        let mut grammar = Grammar::empty();
        grammar.set_end_token("EndSymbol");
        grammar.set_starting_non_terminal("E'");

        grammar.add_production(Production::from_string(
            "E'",
            vec![GrammarSymbol::non_terminal("E")],
        ));

        grammar.add_production(Production::with_function(
            "E",
            vec![
                GrammarSymbol::non_terminal("E"),
                GrammarSymbol::terminal(Token::new("Plus")),
                GrammarSymbol::non_terminal("T"),
            ],
            ast::ast_create_addition,
        ));
        grammar.add_production(Production::from_string(
            "E",
            vec![GrammarSymbol::non_terminal("T")],
        ));

        grammar.add_production(Production::with_function(
            "T",
            vec![
                GrammarSymbol::non_terminal("T"),
                GrammarSymbol::terminal(Token::new("Times")),
                GrammarSymbol::non_terminal("F"),
            ],
            ast::ast_create_multiplication,
        ));
        grammar.add_production(Production::from_string(
            "T",
            vec![GrammarSymbol::non_terminal("F")],
        ));

        grammar.add_production(Production::with_function(
            "F",
            vec![
                GrammarSymbol::terminal(Token::new("OpeningBracket")),
                GrammarSymbol::non_terminal("E"),
                GrammarSymbol::terminal(Token::new("ClosingBracket")),
            ],
            ast::ast_create_calculation,
        ));
        grammar.add_production(Production::with_function(
            "F",
            vec![GrammarSymbol::terminal(Token::new("Id"))],
            ast::ast_create_number,
        ));

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

    pub fn add_production(&mut self, production: Production) {
        let non_terminal = production.get_non_terminal();
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

    pub fn get_production(&self, non_terminal: &str) -> &Vec<Production> {
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

    pub fn get_parser_table(&self) -> ParserTable {
        ParserTable::new(&self)
    }

    pub fn get_slr_table(&self) -> Vec<HashMap<Token, Reaction>> {
        self.get_parser_table().get_action_table()
    }

    pub fn get_goto_table(&self) -> Vec<HashMap<String, usize>> {
        self.get_parser_table().get_goto_table()
    }
}
