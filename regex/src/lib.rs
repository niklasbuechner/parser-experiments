mod ast;
mod parsing;
mod state_machine_builder;

use ast::get_ast_for_concatenation_list;
use parsing::calculate_concatenation_list;
use state_machine_builder::StateMachineBuilder;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum RegexAstElements {
    Alternation(Box<RegexAstElements>, Box<RegexAstElements>),
    Concatenation(Box<RegexAstElements>, Box<RegexAstElements>),
    Leaf(MatchingGroup),
    None,
    ZeroOrMore(Box<RegexAstElements>),
    ZeroOrOne(Box<RegexAstElements>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingGroup {
    Character(char),
    Group(Vec<MatchingGroupElements>),
    NegativeGroup(Vec<MatchingGroupElements>),
    AcceptedState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingGroupElements {
    Character(char),
    Range(char, char),
}

struct ConcatenationList {
    list: Vec<usize>,
    consumed_characters: usize,
}
impl ConcatenationList {
    pub fn new(list: Vec<usize>, consumed_characters: usize) -> Self {
        ConcatenationList {
            list,
            consumed_characters,
        }
    }
}

#[derive(Debug, PartialEq)]
struct State {
    pub matching_group: MatchingGroup,
    pub left_next: Option<Vec<usize>>,
    pub right_next: Option<Vec<usize>>,
    pub is_escaped: bool,
}
impl State {
    pub fn new(
        matching_group: MatchingGroup,
        left_next: Option<Vec<usize>>,
        right_next: Option<Vec<usize>>,
    ) -> Self {
        State {
            matching_group,
            left_next,
            right_next,
            is_escaped: false,
        }
    }

    pub fn new_escaped(
        matching_group: MatchingGroup,
        left_next: Option<Vec<usize>>,
        right_next: Option<Vec<usize>>,
    ) -> Self {
        State {
            matching_group,
            left_next,
            right_next,
            is_escaped: true,
        }
    }
}

pub struct RegexEngine {
    matching_groups: Vec<MatchingGroup>,
    transitions: HashMap<usize, (HashMap<usize, usize>, bool)>,
}
impl RegexEngine {
    pub(crate) fn new(ast: &RegexAstElements) -> Self {
        return StateMachineBuilder::create_regex_engine(ast);
    }

    pub(crate) fn new_with_values(
        matching_groups: Vec<MatchingGroup>,
        transitions: HashMap<usize, (HashMap<usize, usize>, bool)>,
    ) -> Self {
        RegexEngine {
            matching_groups,
            transitions,
        }
    }

    pub fn matches(&self, string: &str) -> bool {
        let mut current_state = 0;
        let characters: Vec<char> = string.chars().collect();

        for character in characters {
            let (matching_group_transitions, _) = match self.transitions.get(&current_state) {
                Some(transitions) => transitions,
                None => return false,
            };

            let matching_group_index = match self.get_matching_group_index(character) {
                Some(index) => index,
                None => return false,
            };

            match matching_group_transitions.get(&matching_group_index) {
                Some(new_state) => {
                    current_state = *new_state;
                }
                None => return false,
            }
        }

        match self.transitions.get(&current_state) {
            Some((_, is_accepted)) => return *is_accepted,
            None => return false,
        }
    }

    fn get_matching_group_index(&self, character: char) -> Option<usize> {
        for i in 0..self.matching_groups.len() {
            match &self.matching_groups[i] {
                MatchingGroup::AcceptedState => {}
                MatchingGroup::Character(matching_character) => {
                    if *matching_character == character {
                        return Some(i);
                    }
                }
                MatchingGroup::Group(ref elements) => {
                    for element in elements {
                        match element {
                            MatchingGroupElements::Character(matching_character) => {
                                if *matching_character == character {
                                    return Some(i);
                                }
                            }
                            MatchingGroupElements::Range(start_character, end_character) => {
                                if *start_character <= character && *end_character >= character {
                                    return Some(i);
                                }
                            }
                        }
                    }
                }
                MatchingGroup::NegativeGroup(ref elements) => {
                    for element in elements {
                        match element {
                            MatchingGroupElements::Character(matching_character) => {
                                if *matching_character == character {
                                    return None;
                                }
                            }
                            MatchingGroupElements::Range(start_character, end_character) => {
                                if *start_character <= character && *end_character >= character {
                                    return None;
                                }
                            }
                        }
                    }

                    return Some(i);
                }
            }
        }

        return None;
    }
}

pub fn get_regex_syntax_tree(regex: &str) -> RegexAstElements {
    let mut stack = Vec::with_capacity(regex.len() + 1);
    let concatenation_list = calculate_concatenation_list(&mut stack, regex);

    return get_ast_for_concatenation_list(&stack, &concatenation_list.list);
}

pub fn get_regex_engine(regex: &str) -> RegexEngine {
    let ast = get_regex_syntax_tree(regex);

    return RegexEngine::new(&ast);
}
