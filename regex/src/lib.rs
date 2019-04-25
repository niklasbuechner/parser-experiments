mod ast;
mod parsing;

use ast::get_ast_for_concatenation_list;
use parsing::calculate_concatenation_list;

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

pub fn get_regex_syntax_tree(regex: &str) -> RegexAstElements {
    let mut stack = Vec::with_capacity(regex.len() + 1);
    let concatenation_list = calculate_concatenation_list(&mut stack, regex);

    return get_ast_for_concatenation_list(&stack, &concatenation_list.list);
}
