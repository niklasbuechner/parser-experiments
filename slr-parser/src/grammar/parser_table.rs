use super::Follow;
use super::Grammar;
use super::GrammarSymbol;
use super::Production;
use super::Reaction;
use super::Reduction;
use super::SlrClosure;
use super::SlrGoto;
use super::Token;
use std::collections::HashMap;

pub(crate) struct ParserTable {
    action_table: Vec<HashMap<Token, Reaction>>,
    goto_table: Vec<HashMap<String, usize>>,
}
impl ParserTable {
    pub fn new(grammar: &Grammar) -> Self {
        let mut collection = StateCollection::from_grammar(grammar);
        let mut table = ParserTable {
            action_table: Vec::new(),
            goto_table: Vec::new(),
        };

        for index in 0..collection.transitions.len() {
            let mut goto = HashMap::new();
            let mut actions = HashMap::new();
            let transitions = &collection.transitions[index];

            for (symbol, state) in transitions {
                match symbol {
                    GrammarSymbol::NonTerminal(name) => {
                        goto.insert(name.clone(), *state);
                    }
                    GrammarSymbol::Terminal(token) => {
                        actions.insert(token.clone(), Reaction::ShiftTo(*state));
                    }
                }
            }

            collection.states[index]
                .iter()
                .filter(|production| production.cursor_is_at_end())
                .for_each(|production| {
                    let follow = Follow::get_follow(grammar, production.get_non_terminal());
                    for element in follow {
                        actions.insert(
                            element,
                            Reaction::Reduce(Reduction::new(
                                production.get_non_terminal(),
                                production.get_element_size(),
                            )),
                        );
                    }

                    if production.get_non_terminal() == grammar.get_starting_non_terminal() {
                        actions.insert(grammar.get_end_symbol(), Reaction::Accept);
                    }
                });

            table.action_table.push(actions);
            table.goto_table.push(goto);
        }

        table
    }

    pub fn get_action_table(self) -> Vec<HashMap<Token, Reaction>> {
        self.action_table
    }

    pub fn get_goto_table(self) -> Vec<HashMap<String, usize>> {
        self.goto_table
    }
}

struct StateCollection {
    states: Vec<Vec<Production>>,
    transitions: Vec<HashMap<GrammarSymbol, usize>>,
}
impl StateCollection {
    pub fn from_grammar(grammar: &Grammar) -> Self {
        let mut collection = StateCollection {
            states: Vec::new(),
            transitions: Vec::new(),
        };

        let initial_non_terminal = grammar.get_starting_non_terminal();
        let initial_productions = grammar
            .get_production(initial_non_terminal)
            .iter()
            .map(|production| Production::from_string(initial_non_terminal, production.clone()))
            .collect();
        let initial_state = SlrClosure::get_closure(grammar, initial_productions);
        collection.add_state(initial_state.clone());

        let mut index = 0;
        while index < collection.states.len() {
            let mut transitions = HashMap::new();

            let goto_symbols = SlrGoto::get_goto_symbols(&collection.states[index]);
            for symbol in &goto_symbols {
                let goto_state = SlrGoto::get_goto(symbol, &collection.states[index]);
                let full_state = SlrClosure::get_closure(grammar, goto_state.clone());

                let new_state_id = collection.add_state(full_state);
                transitions.insert(symbol.clone(), new_state_id);
            }

            collection.transitions.push(transitions);
            index += 1;
        }

        collection
    }

    pub fn add_state(&mut self, state: Vec<Production>) -> usize {
        if let Some(index) = self.get_state_index(&state) {
            return index;
        }

        let index = self.states.len();
        self.states.push(state);

        return index;
    }

    pub fn get_state_index(&self, state: &Vec<Production>) -> Option<usize> {
        self.states
            .iter()
            .position(|current_state| current_state == state)
    }
}
