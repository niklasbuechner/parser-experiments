use super::Grammar;
use super::GrammarSymbol;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Production {
    non_terminal: String,
    elements: Vec<GrammarSymbol>,
    // Determines the index of the cursor. If the cursor is at the begining, the cursor is at 0.
    // If the cursor is at the end the cursor is at elements.len()
    cursor: usize,
}
impl Production {
    pub fn new(non_terminal: &str, elements: Vec<GrammarSymbol>, cursor: usize) -> Self {
        Production {
            non_terminal: non_terminal.to_string(),
            elements,
            cursor,
        }
    }

    pub fn from_string(non_terminal: &str, elements: Vec<GrammarSymbol>) -> Self {
        Production::new(non_terminal, elements, 0)
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
                            .for_each(|single_production| {
                                let new_production =
                                    Production::from_string(name, single_production.to_vec());
                                if !productions.contains(&new_production) {
                                    productions.push(new_production);
                                }
                            });
                    }
                    _ => {}
                }
            }

            if cloned_productions == productions {
                return productions;
            }
        }
    }
}
