use super::Ast;
use super::Grammar;
use super::GrammarSymbol;
use super::Token;

#[derive(Clone)]
pub(crate) struct Production {
    non_terminal: String,
    elements: Vec<GrammarSymbol>,
    // Determines the index of the cursor. If the cursor is at the begining, the cursor is at 0.
    // If the cursor is at the end the cursor is at elements.len()
    cursor: usize,
    ast_function: fn(&mut Vec<Ast>),
}
impl Production {
    pub fn new(
        non_terminal: &str,
        elements: Vec<GrammarSymbol>,
        cursor: usize,
        ast_function: fn(&mut Vec<Ast>),
    ) -> Self {
        Production {
            non_terminal: non_terminal.to_string(),
            elements,
            cursor,
            ast_function,
        }
    }

    pub fn from_string(non_terminal: &str, elements: Vec<GrammarSymbol>) -> Self {
        Production::new(non_terminal, elements, 0, |_| {})
    }

    pub fn with_function(
        non_terminal: &str,
        elements: Vec<GrammarSymbol>,
        ast_function: fn(&mut Vec<Ast>),
    ) -> Self {
        Production::new(non_terminal, elements, 0, ast_function)
    }

    pub fn get_non_terminal(&self) -> &str {
        &self.non_terminal
    }

    pub fn get_next_grammar_symbol(&self) -> Option<&GrammarSymbol> {
        if self.cursor == self.elements.len() {
            return None;
        }

        return Some(&self.elements[self.cursor]);
    }

    pub fn move_cursor_ahead(&mut self) {
        self.cursor += 1;
    }

    pub fn cursor_is_at_end(&self) -> bool {
        self.cursor == self.elements.len()
    }

    pub fn get_element_size(&self) -> usize {
        self.elements.len()
    }

    pub fn get_elements(&self) -> &Vec<GrammarSymbol> {
        &self.elements
    }

    pub fn get_function(&self) -> &fn(&mut Vec<Ast>) {
        &self.ast_function
    }
}
impl std::fmt::Debug for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Reduction {{ non_terminal: {}, elements: {:#?}, cursor: {} }}",
            self.non_terminal, self.elements, self.cursor,
        )
    }
}
impl PartialEq for Production {
    fn eq(&self, other: &Self) -> bool {
        return self.non_terminal == other.non_terminal
            && self.elements == other.elements
            && self.cursor == other.cursor;
    }
}

pub(crate) struct SlrClosure {}
impl SlrClosure {
    pub fn get_closure(grammar: &Grammar, mut productions: Vec<Production>) -> Vec<Production> {
        loop {
            let cloned_productions = productions.clone();

            'productions: for production in &cloned_productions {
                let next_symbol = production.get_next_grammar_symbol();
                if next_symbol == None {
                    continue 'productions;
                }

                match next_symbol.unwrap() {
                    GrammarSymbol::NonTerminal(name) => {
                        if name == production.get_non_terminal() {
                            continue 'productions;
                        }

                        grammar
                            .get_production(name)
                            .iter()
                            .for_each(|new_production| {
                                if !productions.contains(new_production) {
                                    productions.push(new_production.clone());
                                }
                            });
                    }
                    _ => {}
                }
            }

            if cloned_productions == productions {
                if productions.contains(&Production::new(
                    "F",
                    vec![
                        GrammarSymbol::terminal(Token::new("OpeningBracket")),
                        GrammarSymbol::non_terminal("E"),
                        GrammarSymbol::terminal(Token::new("ClosingBracket")),
                    ],
                    1,
                    |_| {},
                )) {
                    println!("{:#?}", productions);
                }

                return productions;
            }
        }
    }
}
